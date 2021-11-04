#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
pub mod Dialogs;
#[cfg(feature = "Win32_UI_Controls_RichEdit")]
pub mod RichEdit;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ACM_ISPLAYING: u32 = 1128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ACM_OPEN: u32 = 1127u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ACM_OPENA: u32 = 1124u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ACM_OPENW: u32 = 1127u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ACM_PLAY: u32 = 1125u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ACM_STOP: u32 = 1126u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ACN_START: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ACN_STOP: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ACS_AUTOPLAY: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ACS_CENTER: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ACS_TIMER: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ACS_TRANSPARENT: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BCM_FIRST: u32 = 5632u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BCM_GETIDEALSIZE: u32 = 5633u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BCM_GETIMAGELIST: u32 = 5635u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BCM_GETNOTE: u32 = 5642u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BCM_GETNOTELENGTH: u32 = 5643u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BCM_GETSPLITINFO: u32 = 5640u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BCM_GETTEXTMARGIN: u32 = 5637u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BCM_SETDROPDOWNSTATE: u32 = 5638u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BCM_SETIMAGELIST: u32 = 5634u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BCM_SETNOTE: u32 = 5641u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BCM_SETSHIELD: u32 = 5644u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BCM_SETSPLITINFO: u32 = 5639u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BCM_SETTEXTMARGIN: u32 = 5636u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BCN_DROPDOWN: u32 = 4294966048u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BCN_FIRST: u32 = 4294966046u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BCN_HOTITEMCHANGE: u32 = 4294966047u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BCSIF_GLYPH: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BCSIF_IMAGE: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BCSIF_SIZE: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BCSIF_STYLE: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BCSS_ALIGNLEFT: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BCSS_IMAGE: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BCSS_NOSPLIT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BCSS_STRETCH: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BGTYPE(pub i32);
pub const BT_IMAGEFILE: BGTYPE = BGTYPE(0i32);
pub const BT_BORDERFILL: BGTYPE = BGTYPE(1i32);
pub const BT_NONE: BGTYPE = BGTYPE(2i32);
impl ::std::convert::From<i32> for BGTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BGTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BORDERTYPE(pub i32);
pub const BT_RECT: BORDERTYPE = BORDERTYPE(0i32);
pub const BT_ROUNDRECT: BORDERTYPE = BORDERTYPE(1i32);
pub const BT_ELLIPSE: BORDERTYPE = BORDERTYPE(2i32);
impl ::std::convert::From<i32> for BORDERTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BORDERTYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct BP_ANIMATIONPARAMS {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub style: BP_ANIMATIONSTYLE,
    pub dwDuration: u32,
}
impl BP_ANIMATIONPARAMS {}
impl ::std::default::Default for BP_ANIMATIONPARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BP_ANIMATIONPARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BP_ANIMATIONPARAMS").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("style", &self.style).field("dwDuration", &self.dwDuration).finish()
    }
}
impl ::std::cmp::PartialEq for BP_ANIMATIONPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.style == other.style && self.dwDuration == other.dwDuration
    }
}
impl ::std::cmp::Eq for BP_ANIMATIONPARAMS {}
unsafe impl ::windows::runtime::Abi for BP_ANIMATIONPARAMS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BP_ANIMATIONSTYLE(pub i32);
pub const BPAS_NONE: BP_ANIMATIONSTYLE = BP_ANIMATIONSTYLE(0i32);
pub const BPAS_LINEAR: BP_ANIMATIONSTYLE = BP_ANIMATIONSTYLE(1i32);
pub const BPAS_CUBIC: BP_ANIMATIONSTYLE = BP_ANIMATIONSTYLE(2i32);
pub const BPAS_SINE: BP_ANIMATIONSTYLE = BP_ANIMATIONSTYLE(3i32);
impl ::std::convert::From<i32> for BP_ANIMATIONSTYLE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BP_ANIMATIONSTYLE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BP_BUFFERFORMAT(pub i32);
pub const BPBF_COMPATIBLEBITMAP: BP_BUFFERFORMAT = BP_BUFFERFORMAT(0i32);
pub const BPBF_DIB: BP_BUFFERFORMAT = BP_BUFFERFORMAT(1i32);
pub const BPBF_TOPDOWNDIB: BP_BUFFERFORMAT = BP_BUFFERFORMAT(2i32);
pub const BPBF_TOPDOWNMONODIB: BP_BUFFERFORMAT = BP_BUFFERFORMAT(3i32);
impl ::std::convert::From<i32> for BP_BUFFERFORMAT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BP_BUFFERFORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct BP_PAINTPARAMS {
    pub cbSize: u32,
    pub dwFlags: BP_PAINTPARAMS_FLAGS,
    pub prcExclude: *mut super::super::Foundation::RECT,
    pub pBlendFunction: *mut super::super::Graphics::Gdi::BLENDFUNCTION,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl BP_PAINTPARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for BP_PAINTPARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for BP_PAINTPARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BP_PAINTPARAMS").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("prcExclude", &self.prcExclude).field("pBlendFunction", &self.pBlendFunction).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for BP_PAINTPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.prcExclude == other.prcExclude && self.pBlendFunction == other.pBlendFunction
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for BP_PAINTPARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for BP_PAINTPARAMS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BP_PAINTPARAMS_FLAGS(pub u32);
pub const BPPF_ERASE: BP_PAINTPARAMS_FLAGS = BP_PAINTPARAMS_FLAGS(1u32);
pub const BPPF_NOCLIP: BP_PAINTPARAMS_FLAGS = BP_PAINTPARAMS_FLAGS(2u32);
pub const BPPF_NONCLIENT: BP_PAINTPARAMS_FLAGS = BP_PAINTPARAMS_FLAGS(4u32);
impl ::std::convert::From<u32> for BP_PAINTPARAMS_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BP_PAINTPARAMS_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for BP_PAINTPARAMS_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for BP_PAINTPARAMS_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for BP_PAINTPARAMS_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for BP_PAINTPARAMS_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for BP_PAINTPARAMS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BST_DROPDOWNPUSHED: u32 = 1024u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BST_HOT: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BS_COMMANDLINK: i32 = 14i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BS_DEFCOMMANDLINK: i32 = 15i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BS_DEFSPLITBUTTON: i32 = 13i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BS_SPLITBUTTON: i32 = 12i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BTNS_AUTOSIZE: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BTNS_BUTTON: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BTNS_CHECK: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BTNS_DROPDOWN: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BTNS_GROUP: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BTNS_NOPREFIX: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BTNS_SEP: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BTNS_SHOWTEXT: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const BTNS_WHOLEDROPDOWN: u32 = 128u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct BUTTON_IMAGELIST {
    pub himl: HIMAGELIST,
    pub margin: super::super::Foundation::RECT,
    pub uAlign: BUTTON_IMAGELIST_ALIGN,
}
#[cfg(feature = "Win32_Foundation")]
impl BUTTON_IMAGELIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for BUTTON_IMAGELIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for BUTTON_IMAGELIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BUTTON_IMAGELIST").field("himl", &self.himl).field("margin", &self.margin).field("uAlign", &self.uAlign).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for BUTTON_IMAGELIST {
    fn eq(&self, other: &Self) -> bool {
        self.himl == other.himl && self.margin == other.margin && self.uAlign == other.uAlign
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for BUTTON_IMAGELIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for BUTTON_IMAGELIST {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BUTTON_IMAGELIST_ALIGN(pub u32);
pub const BUTTON_IMAGELIST_ALIGN_LEFT: BUTTON_IMAGELIST_ALIGN = BUTTON_IMAGELIST_ALIGN(0u32);
pub const BUTTON_IMAGELIST_ALIGN_RIGHT: BUTTON_IMAGELIST_ALIGN = BUTTON_IMAGELIST_ALIGN(1u32);
pub const BUTTON_IMAGELIST_ALIGN_TOP: BUTTON_IMAGELIST_ALIGN = BUTTON_IMAGELIST_ALIGN(2u32);
pub const BUTTON_IMAGELIST_ALIGN_BOTTOM: BUTTON_IMAGELIST_ALIGN = BUTTON_IMAGELIST_ALIGN(3u32);
pub const BUTTON_IMAGELIST_ALIGN_CENTER: BUTTON_IMAGELIST_ALIGN = BUTTON_IMAGELIST_ALIGN(4u32);
impl ::std::convert::From<u32> for BUTTON_IMAGELIST_ALIGN {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BUTTON_IMAGELIST_ALIGN {
    type Abi = Self;
}
impl ::std::ops::BitOr for BUTTON_IMAGELIST_ALIGN {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for BUTTON_IMAGELIST_ALIGN {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for BUTTON_IMAGELIST_ALIGN {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for BUTTON_IMAGELIST_ALIGN {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for BUTTON_IMAGELIST_ALIGN {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct BUTTON_SPLITINFO {
    pub mask: u32,
    pub himlGlyph: HIMAGELIST,
    pub uSplitStyle: u32,
    pub size: super::super::Foundation::SIZE,
}
#[cfg(feature = "Win32_Foundation")]
impl BUTTON_SPLITINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for BUTTON_SPLITINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for BUTTON_SPLITINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BUTTON_SPLITINFO").field("mask", &self.mask).field("himlGlyph", &self.himlGlyph).field("uSplitStyle", &self.uSplitStyle).field("size", &self.size).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for BUTTON_SPLITINFO {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.himlGlyph == other.himlGlyph && self.uSplitStyle == other.uSplitStyle && self.size == other.size
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for BUTTON_SPLITINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for BUTTON_SPLITINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn BeginBufferedAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hwnd: Param0, hdctarget: Param1, prctarget: *const super::super::Foundation::RECT, dwformat: BP_BUFFERFORMAT, ppaintparams: *const BP_PAINTPARAMS, panimationparams: *const BP_ANIMATIONPARAMS, phdcfrom: *mut super::super::Graphics::Gdi::HDC, phdcto: *mut super::super::Graphics::Gdi::HDC) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BeginBufferedAnimation(hwnd: super::super::Foundation::HWND, hdctarget: super::super::Graphics::Gdi::HDC, prctarget: *const super::super::Foundation::RECT, dwformat: BP_BUFFERFORMAT, ppaintparams: *const BP_PAINTPARAMS, panimationparams: *const BP_ANIMATIONPARAMS, phdcfrom: *mut super::super::Graphics::Gdi::HDC, phdcto: *mut super::super::Graphics::Gdi::HDC) -> isize;
        }
        ::std::mem::transmute(BeginBufferedAnimation(hwnd.into_param().abi(), hdctarget.into_param().abi(), ::std::mem::transmute(prctarget), ::std::mem::transmute(dwformat), ::std::mem::transmute(ppaintparams), ::std::mem::transmute(panimationparams), ::std::mem::transmute(phdcfrom), ::std::mem::transmute(phdcto)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn BeginBufferedPaint<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdctarget: Param0, prctarget: *const super::super::Foundation::RECT, dwformat: BP_BUFFERFORMAT, ppaintparams: *const BP_PAINTPARAMS, phdc: *mut super::super::Graphics::Gdi::HDC) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BeginBufferedPaint(hdctarget: super::super::Graphics::Gdi::HDC, prctarget: *const super::super::Foundation::RECT, dwformat: BP_BUFFERFORMAT, ppaintparams: *const BP_PAINTPARAMS, phdc: *mut super::super::Graphics::Gdi::HDC) -> isize;
        }
        ::std::mem::transmute(BeginBufferedPaint(hdctarget.into_param().abi(), ::std::mem::transmute(prctarget), ::std::mem::transmute(dwformat), ::std::mem::transmute(ppaintparams), ::std::mem::transmute(phdc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BeginPanningFeedback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BeginPanningFeedback(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(BeginPanningFeedback(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BufferedPaintClear(hbufferedpaint: isize, prc: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BufferedPaintClear(hbufferedpaint: isize, prc: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT;
        }
        BufferedPaintClear(::std::mem::transmute(hbufferedpaint), ::std::mem::transmute(prc)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn BufferedPaintInit() -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BufferedPaintInit() -> ::windows::runtime::HRESULT;
        }
        BufferedPaintInit().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn BufferedPaintRenderAnimation<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hwnd: Param0, hdctarget: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BufferedPaintRenderAnimation(hwnd: super::super::Foundation::HWND, hdctarget: super::super::Graphics::Gdi::HDC) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(BufferedPaintRenderAnimation(hwnd.into_param().abi(), hdctarget.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BufferedPaintSetAlpha(hbufferedpaint: isize, prc: *const super::super::Foundation::RECT, alpha: u8) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BufferedPaintSetAlpha(hbufferedpaint: isize, prc: *const super::super::Foundation::RECT, alpha: u8) -> ::windows::runtime::HRESULT;
        }
        BufferedPaintSetAlpha(::std::mem::transmute(hbufferedpaint), ::std::mem::transmute(prc), ::std::mem::transmute(alpha)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BufferedPaintStopAllAnimations<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BufferedPaintStopAllAnimations(hwnd: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT;
        }
        BufferedPaintStopAllAnimations(hwnd.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn BufferedPaintUnInit() -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BufferedPaintUnInit() -> ::windows::runtime::HRESULT;
        }
        BufferedPaintUnInit().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBEMAXSTRLEN: u32 = 260u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBEM_GETCOMBOCONTROL: u32 = 1030u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBEM_GETEDITCONTROL: u32 = 1031u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBEM_GETEXSTYLE: u32 = 1033u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBEM_GETEXTENDEDSTYLE: u32 = 1033u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBEM_GETIMAGELIST: u32 = 1027u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBEM_GETITEM: u32 = 1037u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBEM_GETITEMA: u32 = 1028u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBEM_GETITEMW: u32 = 1037u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBEM_GETUNICODEFORMAT: u32 = 8198u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBEM_HASEDITCHANGED: u32 = 1034u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBEM_INSERTITEM: u32 = 1035u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBEM_INSERTITEMA: u32 = 1025u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBEM_INSERTITEMW: u32 = 1035u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBEM_SETEXSTYLE: u32 = 1032u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBEM_SETEXTENDEDSTYLE: u32 = 1038u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBEM_SETIMAGELIST: u32 = 1026u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBEM_SETITEM: u32 = 1036u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBEM_SETITEMA: u32 = 1029u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBEM_SETITEMW: u32 = 1036u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBEM_SETUNICODEFORMAT: u32 = 8197u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBEM_SETWINDOWTHEME: u32 = 8203u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBENF_DROPDOWN: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBENF_ESCAPE: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBENF_KILLFOCUS: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBENF_RETURN: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBES_EX_CASESENSITIVE: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBES_EX_NOEDITIMAGE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBES_EX_NOEDITIMAGEINDENT: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBES_EX_NOSIZELIMIT: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBES_EX_PATHWORDBREAKPROC: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBES_EX_TEXTENDELLIPSIS: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CBM_FIRST: u32 = 5888u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CB_GETCUEBANNER: u32 = 5892u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CB_GETMINVISIBLE: u32 = 5890u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CB_SETCUEBANNER: u32 = 5891u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CB_SETMINVISIBLE: u32 = 5889u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CCF_NOTEXT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CCHCCCLASS: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CCHCCDESC: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CCHCCTEXT: u32 = 256u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct CCINFOA {
    pub szClass: [super::super::Foundation::CHAR; 32],
    pub flOptions: u32,
    pub szDesc: [super::super::Foundation::CHAR; 32],
    pub cxDefault: u32,
    pub cyDefault: u32,
    pub flStyleDefault: u32,
    pub flExtStyleDefault: u32,
    pub flCtrlTypeMask: u32,
    pub szTextDefault: [super::super::Foundation::CHAR; 256],
    pub cStyleFlags: i32,
    pub aStyleFlags: *mut CCSTYLEFLAGA,
    pub lpfnStyle: ::std::option::Option<LPFNCCSTYLEA>,
    pub lpfnSizeToText: ::std::option::Option<LPFNCCSIZETOTEXTA>,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl CCINFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for CCINFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for CCINFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CCINFOA")
            .field("szClass", &self.szClass)
            .field("flOptions", &self.flOptions)
            .field("szDesc", &self.szDesc)
            .field("cxDefault", &self.cxDefault)
            .field("cyDefault", &self.cyDefault)
            .field("flStyleDefault", &self.flStyleDefault)
            .field("flExtStyleDefault", &self.flExtStyleDefault)
            .field("flCtrlTypeMask", &self.flCtrlTypeMask)
            .field("szTextDefault", &self.szTextDefault)
            .field("cStyleFlags", &self.cStyleFlags)
            .field("aStyleFlags", &self.aStyleFlags)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for CCINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.szClass == other.szClass
            && self.flOptions == other.flOptions
            && self.szDesc == other.szDesc
            && self.cxDefault == other.cxDefault
            && self.cyDefault == other.cyDefault
            && self.flStyleDefault == other.flStyleDefault
            && self.flExtStyleDefault == other.flExtStyleDefault
            && self.flCtrlTypeMask == other.flCtrlTypeMask
            && self.szTextDefault == other.szTextDefault
            && self.cStyleFlags == other.cStyleFlags
            && self.aStyleFlags == other.aStyleFlags
            && self.lpfnStyle.map(|f| f as usize) == other.lpfnStyle.map(|f| f as usize)
            && self.lpfnSizeToText.map(|f| f as usize) == other.lpfnSizeToText.map(|f| f as usize)
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for CCINFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for CCINFOA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct CCINFOW {
    pub szClass: [u16; 32],
    pub flOptions: u32,
    pub szDesc: [u16; 32],
    pub cxDefault: u32,
    pub cyDefault: u32,
    pub flStyleDefault: u32,
    pub flExtStyleDefault: u32,
    pub flCtrlTypeMask: u32,
    pub cStyleFlags: i32,
    pub aStyleFlags: *mut CCSTYLEFLAGW,
    pub szTextDefault: [u16; 256],
    pub lpfnStyle: ::std::option::Option<LPFNCCSTYLEW>,
    pub lpfnSizeToText: ::std::option::Option<LPFNCCSIZETOTEXTW>,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl CCINFOW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for CCINFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for CCINFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CCINFOW")
            .field("szClass", &self.szClass)
            .field("flOptions", &self.flOptions)
            .field("szDesc", &self.szDesc)
            .field("cxDefault", &self.cxDefault)
            .field("cyDefault", &self.cyDefault)
            .field("flStyleDefault", &self.flStyleDefault)
            .field("flExtStyleDefault", &self.flExtStyleDefault)
            .field("flCtrlTypeMask", &self.flCtrlTypeMask)
            .field("cStyleFlags", &self.cStyleFlags)
            .field("aStyleFlags", &self.aStyleFlags)
            .field("szTextDefault", &self.szTextDefault)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for CCINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.szClass == other.szClass
            && self.flOptions == other.flOptions
            && self.szDesc == other.szDesc
            && self.cxDefault == other.cxDefault
            && self.cyDefault == other.cyDefault
            && self.flStyleDefault == other.flStyleDefault
            && self.flExtStyleDefault == other.flExtStyleDefault
            && self.flCtrlTypeMask == other.flCtrlTypeMask
            && self.cStyleFlags == other.cStyleFlags
            && self.aStyleFlags == other.aStyleFlags
            && self.szTextDefault == other.szTextDefault
            && self.lpfnStyle.map(|f| f as usize) == other.lpfnStyle.map(|f| f as usize)
            && self.lpfnSizeToText.map(|f| f as usize) == other.lpfnSizeToText.map(|f| f as usize)
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for CCINFOW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for CCINFOW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CCM_DPISCALE: u32 = 8204u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CCM_FIRST: u32 = 8192u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CCM_GETCOLORSCHEME: u32 = 8195u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CCM_GETDROPTARGET: u32 = 8196u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CCM_GETUNICODEFORMAT: u32 = 8198u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CCM_GETVERSION: u32 = 8200u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CCM_LAST: u32 = 8704u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CCM_SETBKCOLOR: u32 = 8193u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CCM_SETCOLORSCHEME: u32 = 8194u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CCM_SETNOTIFYWINDOW: u32 = 8201u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CCM_SETUNICODEFORMAT: u32 = 8197u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CCM_SETVERSION: u32 = 8199u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CCM_SETWINDOWTHEME: u32 = 8203u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct CCSTYLEA {
    pub flStyle: u32,
    pub flExtStyle: u32,
    pub szText: [super::super::Foundation::CHAR; 256],
    pub lgid: u16,
    pub wReserved1: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl CCSTYLEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CCSTYLEA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CCSTYLEA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CCSTYLEA").field("flStyle", &self.flStyle).field("flExtStyle", &self.flExtStyle).field("szText", &self.szText).field("lgid", &self.lgid).field("wReserved1", &self.wReserved1).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CCSTYLEA {
    fn eq(&self, other: &Self) -> bool {
        self.flStyle == other.flStyle && self.flExtStyle == other.flExtStyle && self.szText == other.szText && self.lgid == other.lgid && self.wReserved1 == other.wReserved1
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CCSTYLEA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CCSTYLEA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct CCSTYLEFLAGA {
    pub flStyle: u32,
    pub flStyleMask: u32,
    pub pszStyle: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl CCSTYLEFLAGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CCSTYLEFLAGA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CCSTYLEFLAGA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CCSTYLEFLAGA").field("flStyle", &self.flStyle).field("flStyleMask", &self.flStyleMask).field("pszStyle", &self.pszStyle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CCSTYLEFLAGA {
    fn eq(&self, other: &Self) -> bool {
        self.flStyle == other.flStyle && self.flStyleMask == other.flStyleMask && self.pszStyle == other.pszStyle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CCSTYLEFLAGA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CCSTYLEFLAGA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct CCSTYLEFLAGW {
    pub flStyle: u32,
    pub flStyleMask: u32,
    pub pszStyle: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl CCSTYLEFLAGW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CCSTYLEFLAGW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CCSTYLEFLAGW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CCSTYLEFLAGW").field("flStyle", &self.flStyle).field("flStyleMask", &self.flStyleMask).field("pszStyle", &self.pszStyle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CCSTYLEFLAGW {
    fn eq(&self, other: &Self) -> bool {
        self.flStyle == other.flStyle && self.flStyleMask == other.flStyleMask && self.pszStyle == other.pszStyle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CCSTYLEFLAGW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CCSTYLEFLAGW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct CCSTYLEW {
    pub flStyle: u32,
    pub flExtStyle: u32,
    pub szText: [u16; 256],
    pub lgid: u16,
    pub wReserved1: u16,
}
impl CCSTYLEW {}
impl ::std::default::Default for CCSTYLEW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CCSTYLEW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CCSTYLEW").field("flStyle", &self.flStyle).field("flExtStyle", &self.flExtStyle).field("szText", &self.szText).field("lgid", &self.lgid).field("wReserved1", &self.wReserved1).finish()
    }
}
impl ::std::cmp::PartialEq for CCSTYLEW {
    fn eq(&self, other: &Self) -> bool {
        self.flStyle == other.flStyle && self.flExtStyle == other.flExtStyle && self.szText == other.szText && self.lgid == other.lgid && self.wReserved1 == other.wReserved1
    }
}
impl ::std::cmp::Eq for CCSTYLEW {}
unsafe impl ::windows::runtime::Abi for CCSTYLEW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CCS_ADJUSTABLE: i32 = 32i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CCS_BOTTOM: i32 = 3i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CCS_NODIVIDER: i32 = 64i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CCS_NOMOVEY: i32 = 2i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CCS_NOPARENTALIGN: i32 = 8i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CCS_NORESIZE: i32 = 4i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CCS_TOP: i32 = 1i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CCS_VERT: i32 = 128i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CDDS_ITEM: u32 = 65536u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CDDS_POSTERASE: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CDIS_CHECKED: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CDIS_DEFAULT: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CDIS_DISABLED: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CDIS_DROPHILITED: u32 = 4096u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CDIS_FOCUS: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CDIS_GRAYED: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CDIS_HOT: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CDIS_INDETERMINATE: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CDIS_MARKED: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CDIS_NEARHOT: u32 = 1024u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CDIS_OTHERSIDEHOT: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CDIS_SELECTED: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CDIS_SHOWKEYBOARDCUES: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CDRF_DODEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CDRF_DOERASE: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CDRF_NEWFONT: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CDRF_NOTIFYITEMDRAW: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CDRF_NOTIFYPOSTERASE: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CDRF_NOTIFYPOSTPAINT: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CDRF_NOTIFYSUBITEMDRAW: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CDRF_SKIPDEFAULT: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CDRF_SKIPPOSTPAINT: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CLOCKPARTS(pub i32);
pub const CLP_TIME: CLOCKPARTS = CLOCKPARTS(1i32);
impl ::std::convert::From<i32> for CLOCKPARTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CLOCKPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CLOCKSTATES(pub i32);
pub const CLS_NORMAL: CLOCKSTATES = CLOCKSTATES(1i32);
pub const CLS_HOT: CLOCKSTATES = CLOCKSTATES(2i32);
pub const CLS_PRESSED: CLOCKSTATES = CLOCKSTATES(3i32);
impl ::std::convert::From<i32> for CLOCKSTATES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CLOCKSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CLR_DEFAULT: i32 = -16777216i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CLR_HILIGHT: i32 = -16777216i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CLR_NONE: i32 = -1i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const CMB_MASKED: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct COLORMAP {
    pub from: u32,
    pub to: u32,
}
impl COLORMAP {}
impl ::std::default::Default for COLORMAP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for COLORMAP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COLORMAP").field("from", &self.from).field("to", &self.to).finish()
    }
}
impl ::std::cmp::PartialEq for COLORMAP {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to
    }
}
impl ::std::cmp::Eq for COLORMAP {}
unsafe impl ::windows::runtime::Abi for COLORMAP {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const COLORMGMTDLGORD: u32 = 1551u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct COLORSCHEME {
    pub dwSize: u32,
    pub clrBtnHighlight: u32,
    pub clrBtnShadow: u32,
}
impl COLORSCHEME {}
impl ::std::default::Default for COLORSCHEME {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for COLORSCHEME {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COLORSCHEME").field("dwSize", &self.dwSize).field("clrBtnHighlight", &self.clrBtnHighlight).field("clrBtnShadow", &self.clrBtnShadow).finish()
    }
}
impl ::std::cmp::PartialEq for COLORSCHEME {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.clrBtnHighlight == other.clrBtnHighlight && self.clrBtnShadow == other.clrBtnShadow
    }
}
impl ::std::cmp::Eq for COLORSCHEME {}
unsafe impl ::windows::runtime::Abi for COLORSCHEME {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct COMBOBOXEXITEMA {
    pub mask: COMBOBOX_EX_ITEM_FLAGS,
    pub iItem: isize,
    pub pszText: super::super::Foundation::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub iOverlay: i32,
    pub iIndent: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl COMBOBOXEXITEMA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for COMBOBOXEXITEMA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for COMBOBOXEXITEMA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COMBOBOXEXITEMA")
            .field("mask", &self.mask)
            .field("iItem", &self.iItem)
            .field("pszText", &self.pszText)
            .field("cchTextMax", &self.cchTextMax)
            .field("iImage", &self.iImage)
            .field("iSelectedImage", &self.iSelectedImage)
            .field("iOverlay", &self.iOverlay)
            .field("iIndent", &self.iIndent)
            .field("lParam", &self.lParam)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for COMBOBOXEXITEMA {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.iItem == other.iItem && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iImage == other.iImage && self.iSelectedImage == other.iSelectedImage && self.iOverlay == other.iOverlay && self.iIndent == other.iIndent && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for COMBOBOXEXITEMA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for COMBOBOXEXITEMA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct COMBOBOXEXITEMW {
    pub mask: COMBOBOX_EX_ITEM_FLAGS,
    pub iItem: isize,
    pub pszText: super::super::Foundation::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub iOverlay: i32,
    pub iIndent: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl COMBOBOXEXITEMW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for COMBOBOXEXITEMW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for COMBOBOXEXITEMW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COMBOBOXEXITEMW")
            .field("mask", &self.mask)
            .field("iItem", &self.iItem)
            .field("pszText", &self.pszText)
            .field("cchTextMax", &self.cchTextMax)
            .field("iImage", &self.iImage)
            .field("iSelectedImage", &self.iSelectedImage)
            .field("iOverlay", &self.iOverlay)
            .field("iIndent", &self.iIndent)
            .field("lParam", &self.lParam)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for COMBOBOXEXITEMW {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.iItem == other.iItem && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iImage == other.iImage && self.iSelectedImage == other.iSelectedImage && self.iOverlay == other.iOverlay && self.iIndent == other.iIndent && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for COMBOBOXEXITEMW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for COMBOBOXEXITEMW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct COMBOBOXINFO {
    pub cbSize: u32,
    pub rcItem: super::super::Foundation::RECT,
    pub rcButton: super::super::Foundation::RECT,
    pub stateButton: COMBOBOXINFO_BUTTON_STATE,
    pub hwndCombo: super::super::Foundation::HWND,
    pub hwndItem: super::super::Foundation::HWND,
    pub hwndList: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl COMBOBOXINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for COMBOBOXINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for COMBOBOXINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COMBOBOXINFO").field("cbSize", &self.cbSize).field("rcItem", &self.rcItem).field("rcButton", &self.rcButton).field("stateButton", &self.stateButton).field("hwndCombo", &self.hwndCombo).field("hwndItem", &self.hwndItem).field("hwndList", &self.hwndList).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for COMBOBOXINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.rcItem == other.rcItem && self.rcButton == other.rcButton && self.stateButton == other.stateButton && self.hwndCombo == other.hwndCombo && self.hwndItem == other.hwndItem && self.hwndList == other.hwndList
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for COMBOBOXINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for COMBOBOXINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMBOBOXINFO_BUTTON_STATE(pub u32);
pub const STATE_SYSTEM_INVISIBLE: COMBOBOXINFO_BUTTON_STATE = COMBOBOXINFO_BUTTON_STATE(32768u32);
pub const STATE_SYSTEM_PRESSED: COMBOBOXINFO_BUTTON_STATE = COMBOBOXINFO_BUTTON_STATE(8u32);
pub const STATE_SYSTEM_FOCUSABLE: COMBOBOXINFO_BUTTON_STATE = COMBOBOXINFO_BUTTON_STATE(1048576u32);
pub const STATE_SYSTEM_OFFSCREEN: COMBOBOXINFO_BUTTON_STATE = COMBOBOXINFO_BUTTON_STATE(65536u32);
pub const STATE_SYSTEM_UNAVAILABLE: COMBOBOXINFO_BUTTON_STATE = COMBOBOXINFO_BUTTON_STATE(1u32);
impl ::std::convert::From<u32> for COMBOBOXINFO_BUTTON_STATE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMBOBOXINFO_BUTTON_STATE {
    type Abi = Self;
}
impl ::std::ops::BitOr for COMBOBOXINFO_BUTTON_STATE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for COMBOBOXINFO_BUTTON_STATE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for COMBOBOXINFO_BUTTON_STATE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for COMBOBOXINFO_BUTTON_STATE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for COMBOBOXINFO_BUTTON_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMBOBOX_EX_ITEM_FLAGS(pub u32);
pub const CBEIF_DI_SETITEM: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(268435456u32);
pub const CBEIF_IMAGE: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(2u32);
pub const CBEIF_INDENT: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(16u32);
pub const CBEIF_LPARAM: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(32u32);
pub const CBEIF_OVERLAY: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(8u32);
pub const CBEIF_SELECTEDIMAGE: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(4u32);
pub const CBEIF_TEXT: COMBOBOX_EX_ITEM_FLAGS = COMBOBOX_EX_ITEM_FLAGS(1u32);
impl ::std::convert::From<u32> for COMBOBOX_EX_ITEM_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMBOBOX_EX_ITEM_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for COMBOBOX_EX_ITEM_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for COMBOBOX_EX_ITEM_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for COMBOBOX_EX_ITEM_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for COMBOBOX_EX_ITEM_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for COMBOBOX_EX_ITEM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const COMCTL32_VERSION: u32 = 6u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct COMPAREITEMSTRUCT {
    pub CtlType: u32,
    pub CtlID: u32,
    pub hwndItem: super::super::Foundation::HWND,
    pub itemID1: u32,
    pub itemData1: usize,
    pub itemID2: u32,
    pub itemData2: usize,
    pub dwLocaleId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl COMPAREITEMSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for COMPAREITEMSTRUCT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for COMPAREITEMSTRUCT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COMPAREITEMSTRUCT")
            .field("CtlType", &self.CtlType)
            .field("CtlID", &self.CtlID)
            .field("hwndItem", &self.hwndItem)
            .field("itemID1", &self.itemID1)
            .field("itemData1", &self.itemData1)
            .field("itemID2", &self.itemID2)
            .field("itemData2", &self.itemData2)
            .field("dwLocaleId", &self.dwLocaleId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for COMPAREITEMSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.CtlType == other.CtlType && self.CtlID == other.CtlID && self.hwndItem == other.hwndItem && self.itemID1 == other.itemID1 && self.itemData1 == other.itemData1 && self.itemID2 == other.itemID2 && self.itemData2 == other.itemData2 && self.dwLocaleId == other.dwLocaleId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for COMPAREITEMSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for COMPAREITEMSTRUCT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CONTENTALIGNMENT(pub i32);
pub const CA_LEFT: CONTENTALIGNMENT = CONTENTALIGNMENT(0i32);
pub const CA_CENTER: CONTENTALIGNMENT = CONTENTALIGNMENT(1i32);
pub const CA_RIGHT: CONTENTALIGNMENT = CONTENTALIGNMENT(2i32);
impl ::std::convert::From<i32> for CONTENTALIGNMENT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CONTENTALIGNMENT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckDlgButton<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hdlg: Param0, nidbutton: i32, ucheck: DLG_BUTTON_CHECK_STATE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckDlgButton(hdlg: super::super::Foundation::HWND, nidbutton: i32, ucheck: DLG_BUTTON_CHECK_STATE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CheckDlgButton(hdlg.into_param().abi(), ::std::mem::transmute(nidbutton), ::std::mem::transmute(ucheck)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckRadioButton<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hdlg: Param0, nidfirstbutton: i32, nidlastbutton: i32, nidcheckbutton: i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CheckRadioButton(hdlg: super::super::Foundation::HWND, nidfirstbutton: i32, nidlastbutton: i32, nidcheckbutton: i32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CheckRadioButton(hdlg.into_param().abi(), ::std::mem::transmute(nidfirstbutton), ::std::mem::transmute(nidlastbutton), ::std::mem::transmute(nidcheckbutton)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn CloseThemeData(htheme: isize) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseThemeData(htheme: isize) -> ::windows::runtime::HRESULT;
        }
        CloseThemeData(::std::mem::transmute(htheme)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn CreateMappedBitmap<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>>(hinstance: Param0, idbitmap: isize, wflags: u32, lpcolormap: *const COLORMAP, inummaps: i32) -> super::super::Graphics::Gdi::HBITMAP {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateMappedBitmap(hinstance: super::super::Foundation::HINSTANCE, idbitmap: isize, wflags: u32, lpcolormap: *const COLORMAP, inummaps: i32) -> super::super::Graphics::Gdi::HBITMAP;
        }
        ::std::mem::transmute(CreateMappedBitmap(hinstance.into_param().abi(), ::std::mem::transmute(idbitmap), ::std::mem::transmute(wflags), ::std::mem::transmute(lpcolormap), ::std::mem::transmute(inummaps)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn CreatePropertySheetPageA(constpropsheetpagepointer: *mut PROPSHEETPAGEA) -> HPROPSHEETPAGE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePropertySheetPageA(constpropsheetpagepointer: *mut ::std::mem::ManuallyDrop<PROPSHEETPAGEA>) -> HPROPSHEETPAGE;
        }
        ::std::mem::transmute(CreatePropertySheetPageA(::std::mem::transmute(constpropsheetpagepointer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn CreatePropertySheetPageW(constpropsheetpagepointer: *mut PROPSHEETPAGEW) -> HPROPSHEETPAGE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePropertySheetPageW(constpropsheetpagepointer: *mut ::std::mem::ManuallyDrop<PROPSHEETPAGEW>) -> HPROPSHEETPAGE;
        }
        ::std::mem::transmute(CreatePropertySheetPageW(::std::mem::transmute(constpropsheetpagepointer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateStatusWindowA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(style: i32, lpsztext: Param1, hwndparent: Param2, wid: u32) -> super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateStatusWindowA(style: i32, lpsztext: super::super::Foundation::PSTR, hwndparent: super::super::Foundation::HWND, wid: u32) -> super::super::Foundation::HWND;
        }
        ::std::mem::transmute(CreateStatusWindowA(::std::mem::transmute(style), lpsztext.into_param().abi(), hwndparent.into_param().abi(), ::std::mem::transmute(wid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateStatusWindowW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(style: i32, lpsztext: Param1, hwndparent: Param2, wid: u32) -> super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateStatusWindowW(style: i32, lpsztext: super::super::Foundation::PWSTR, hwndparent: super::super::Foundation::HWND, wid: u32) -> super::super::Foundation::HWND;
        }
        ::std::mem::transmute(CreateStatusWindowW(::std::mem::transmute(style), lpsztext.into_param().abi(), hwndparent.into_param().abi(), ::std::mem::transmute(wid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn CreateSyntheticPointerDevice(pointertype: super::WindowsAndMessaging::POINTER_INPUT_TYPE, maxcount: u32, mode: POINTER_FEEDBACK_MODE) -> HSYNTHETICPOINTERDEVICE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateSyntheticPointerDevice(pointertype: super::WindowsAndMessaging::POINTER_INPUT_TYPE, maxcount: u32, mode: POINTER_FEEDBACK_MODE) -> HSYNTHETICPOINTERDEVICE;
        }
        ::std::mem::transmute(CreateSyntheticPointerDevice(::std::mem::transmute(pointertype), ::std::mem::transmute(maxcount), ::std::mem::transmute(mode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateToolbarEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>>(hwnd: Param0, ws: u32, wid: u32, nbitmaps: i32, hbminst: Param4, wbmid: usize, lpbuttons: *mut TBBUTTON, inumbuttons: i32, dxbutton: i32, dybutton: i32, dxbitmap: i32, dybitmap: i32, ustructsize: u32) -> super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateToolbarEx(hwnd: super::super::Foundation::HWND, ws: u32, wid: u32, nbitmaps: i32, hbminst: super::super::Foundation::HINSTANCE, wbmid: usize, lpbuttons: *mut TBBUTTON, inumbuttons: i32, dxbutton: i32, dybutton: i32, dxbitmap: i32, dybitmap: i32, ustructsize: u32) -> super::super::Foundation::HWND;
        }
        ::std::mem::transmute(CreateToolbarEx(
            hwnd.into_param().abi(),
            ::std::mem::transmute(ws),
            ::std::mem::transmute(wid),
            ::std::mem::transmute(nbitmaps),
            hbminst.into_param().abi(),
            ::std::mem::transmute(wbmid),
            ::std::mem::transmute(lpbuttons),
            ::std::mem::transmute(inumbuttons),
            ::std::mem::transmute(dxbutton),
            ::std::mem::transmute(dybutton),
            ::std::mem::transmute(dxbitmap),
            ::std::mem::transmute(dybitmap),
            ::std::mem::transmute(ustructsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateUpDownControl<'a, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param8: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(dwstyle: u32, x: i32, y: i32, cx: i32, cy: i32, hparent: Param5, nid: i32, hinst: Param7, hbuddy: Param8, nupper: i32, nlower: i32, npos: i32) -> super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUpDownControl(dwstyle: u32, x: i32, y: i32, cx: i32, cy: i32, hparent: super::super::Foundation::HWND, nid: i32, hinst: super::super::Foundation::HINSTANCE, hbuddy: super::super::Foundation::HWND, nupper: i32, nlower: i32, npos: i32) -> super::super::Foundation::HWND;
        }
        ::std::mem::transmute(CreateUpDownControl(
            ::std::mem::transmute(dwstyle),
            ::std::mem::transmute(x),
            ::std::mem::transmute(y),
            ::std::mem::transmute(cx),
            ::std::mem::transmute(cy),
            hparent.into_param().abi(),
            ::std::mem::transmute(nid),
            hinst.into_param().abi(),
            hbuddy.into_param().abi(),
            ::std::mem::transmute(nupper),
            ::std::mem::transmute(nlower),
            ::std::mem::transmute(npos),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct DATETIMEPICKERINFO {
    pub cbSize: u32,
    pub rcCheck: super::super::Foundation::RECT,
    pub stateCheck: u32,
    pub rcButton: super::super::Foundation::RECT,
    pub stateButton: u32,
    pub hwndEdit: super::super::Foundation::HWND,
    pub hwndUD: super::super::Foundation::HWND,
    pub hwndDropDown: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl DATETIMEPICKERINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DATETIMEPICKERINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DATETIMEPICKERINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DATETIMEPICKERINFO")
            .field("cbSize", &self.cbSize)
            .field("rcCheck", &self.rcCheck)
            .field("stateCheck", &self.stateCheck)
            .field("rcButton", &self.rcButton)
            .field("stateButton", &self.stateButton)
            .field("hwndEdit", &self.hwndEdit)
            .field("hwndUD", &self.hwndUD)
            .field("hwndDropDown", &self.hwndDropDown)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DATETIMEPICKERINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.rcCheck == other.rcCheck && self.stateCheck == other.stateCheck && self.rcButton == other.rcButton && self.stateButton == other.stateButton && self.hwndEdit == other.hwndEdit && self.hwndUD == other.hwndUD && self.hwndDropDown == other.hwndDropDown
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DATETIMEPICKERINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DATETIMEPICKERINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DA_ERR: i32 = -1i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DA_LAST: u32 = 2147483647u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct DELETEITEMSTRUCT {
    pub CtlType: DRAWITEMSTRUCT_CTL_TYPE,
    pub CtlID: u32,
    pub itemID: u32,
    pub hwndItem: super::super::Foundation::HWND,
    pub itemData: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl DELETEITEMSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DELETEITEMSTRUCT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DELETEITEMSTRUCT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DELETEITEMSTRUCT").field("CtlType", &self.CtlType).field("CtlID", &self.CtlID).field("itemID", &self.itemID).field("hwndItem", &self.hwndItem).field("itemData", &self.itemData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DELETEITEMSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.CtlType == other.CtlType && self.CtlID == other.CtlID && self.itemID == other.itemID && self.hwndItem == other.hwndItem && self.itemData == other.itemData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DELETEITEMSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DELETEITEMSTRUCT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DLG_BUTTON_CHECK_STATE(pub u32);
pub const BST_CHECKED: DLG_BUTTON_CHECK_STATE = DLG_BUTTON_CHECK_STATE(1u32);
pub const BST_INDETERMINATE: DLG_BUTTON_CHECK_STATE = DLG_BUTTON_CHECK_STATE(2u32);
pub const BST_UNCHECKED: DLG_BUTTON_CHECK_STATE = DLG_BUTTON_CHECK_STATE(0u32);
impl ::std::convert::From<u32> for DLG_BUTTON_CHECK_STATE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DLG_BUTTON_CHECK_STATE {
    type Abi = Self;
}
impl ::std::ops::BitOr for DLG_BUTTON_CHECK_STATE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DLG_BUTTON_CHECK_STATE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DLG_BUTTON_CHECK_STATE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DLG_BUTTON_CHECK_STATE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DLG_BUTTON_CHECK_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DLG_DIR_LIST_FILE_TYPE(pub u32);
pub const DDL_ARCHIVE: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(32u32);
pub const DDL_DIRECTORY: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(16u32);
pub const DDL_DRIVES: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(16384u32);
pub const DDL_EXCLUSIVE: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(32768u32);
pub const DDL_HIDDEN: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(2u32);
pub const DDL_READONLY: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(1u32);
pub const DDL_READWRITE: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(0u32);
pub const DDL_SYSTEM: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(4u32);
pub const DDL_POSTMSGS: DLG_DIR_LIST_FILE_TYPE = DLG_DIR_LIST_FILE_TYPE(8192u32);
impl ::std::convert::From<u32> for DLG_DIR_LIST_FILE_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DLG_DIR_LIST_FILE_TYPE {
    type Abi = Self;
}
impl ::std::ops::BitOr for DLG_DIR_LIST_FILE_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DLG_DIR_LIST_FILE_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DLG_DIR_LIST_FILE_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DLG_DIR_LIST_FILE_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DLG_DIR_LIST_FILE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DL_COPYCURSOR: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DL_CURSORSET: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DL_MOVECURSOR: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DL_STOPCURSOR: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DPAMM_MESSAGE(pub u32);
pub const DPAMM_MERGE: DPAMM_MESSAGE = DPAMM_MESSAGE(1u32);
pub const DPAMM_DELETE: DPAMM_MESSAGE = DPAMM_MESSAGE(2u32);
pub const DPAMM_INSERT: DPAMM_MESSAGE = DPAMM_MESSAGE(3u32);
impl ::std::convert::From<u32> for DPAMM_MESSAGE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DPAMM_MESSAGE {
    type Abi = Self;
}
impl ::std::ops::BitOr for DPAMM_MESSAGE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DPAMM_MESSAGE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DPAMM_MESSAGE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DPAMM_MESSAGE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DPAMM_MESSAGE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DPAM_INTERSECT: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DPAM_NORMAL: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DPAM_SORTED: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DPAM_UNION: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct DPASTREAMINFO {
    pub iPos: i32,
    pub pvItem: *mut ::std::ffi::c_void,
}
impl DPASTREAMINFO {}
impl ::std::default::Default for DPASTREAMINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DPASTREAMINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DPASTREAMINFO").field("iPos", &self.iPos).field("pvItem", &self.pvItem).finish()
    }
}
impl ::std::cmp::PartialEq for DPASTREAMINFO {
    fn eq(&self, other: &Self) -> bool {
        self.iPos == other.iPos && self.pvItem == other.pvItem
    }
}
impl ::std::cmp::Eq for DPASTREAMINFO {}
unsafe impl ::windows::runtime::Abi for DPASTREAMINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DPAS_INSERTAFTER: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DPAS_INSERTBEFORE: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DPAS_SORTED: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DPA_APPEND: u32 = 2147483647u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn DPA_Clone(hdpa: *const _DPA, hdpanew: *mut _DPA) -> *mut _DPA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_Clone(hdpa: *const _DPA, hdpanew: *mut _DPA) -> *mut _DPA;
        }
        ::std::mem::transmute(DPA_Clone(::std::mem::transmute(hdpa), ::std::mem::transmute(hdpanew)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn DPA_Create(citemgrow: i32) -> *mut _DPA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_Create(citemgrow: i32) -> *mut _DPA;
        }
        ::std::mem::transmute(DPA_Create(::std::mem::transmute(citemgrow)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DPA_CreateEx<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(cpgrow: i32, hheap: Param1) -> *mut _DPA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_CreateEx(cpgrow: i32, hheap: super::super::Foundation::HANDLE) -> *mut _DPA;
        }
        ::std::mem::transmute(DPA_CreateEx(::std::mem::transmute(cpgrow), hheap.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DPA_DeleteAllPtrs(hdpa: *mut _DPA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_DeleteAllPtrs(hdpa: *mut _DPA) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DPA_DeleteAllPtrs(::std::mem::transmute(hdpa)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn DPA_DeletePtr(hdpa: *mut _DPA, i: i32) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_DeletePtr(hdpa: *mut _DPA, i: i32) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(DPA_DeletePtr(::std::mem::transmute(hdpa), ::std::mem::transmute(i)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DPA_Destroy(hdpa: *mut _DPA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_Destroy(hdpa: *mut _DPA) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DPA_Destroy(::std::mem::transmute(hdpa)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn DPA_DestroyCallback(hdpa: *mut _DPA, pfncb: ::std::option::Option<PFNDAENUMCALLBACK>, pdata: *const ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_DestroyCallback(hdpa: *mut _DPA, pfncb: ::windows::runtime::RawPtr, pdata: *const ::std::ffi::c_void);
        }
        ::std::mem::transmute(DPA_DestroyCallback(::std::mem::transmute(hdpa), ::std::mem::transmute(pfncb), ::std::mem::transmute(pdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DPA_ERR: i32 = -1i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn DPA_EnumCallback(hdpa: *const _DPA, pfncb: ::std::option::Option<PFNDAENUMCALLBACK>, pdata: *const ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_EnumCallback(hdpa: *const _DPA, pfncb: ::windows::runtime::RawPtr, pdata: *const ::std::ffi::c_void);
        }
        ::std::mem::transmute(DPA_EnumCallback(::std::mem::transmute(hdpa), ::std::mem::transmute(pfncb), ::std::mem::transmute(pdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn DPA_GetPtr(hdpa: *const _DPA, i: isize) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_GetPtr(hdpa: *const _DPA, i: isize) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(DPA_GetPtr(::std::mem::transmute(hdpa), ::std::mem::transmute(i)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn DPA_GetPtrIndex(hdpa: *const _DPA, p: *const ::std::ffi::c_void) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_GetPtrIndex(hdpa: *const _DPA, p: *const ::std::ffi::c_void) -> i32;
        }
        ::std::mem::transmute(DPA_GetPtrIndex(::std::mem::transmute(hdpa), ::std::mem::transmute(p)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn DPA_GetSize(hdpa: *const _DPA) -> u64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_GetSize(hdpa: *const _DPA) -> u64;
        }
        ::std::mem::transmute(DPA_GetSize(::std::mem::transmute(hdpa)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DPA_Grow(pdpa: *mut _DPA, cp: i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_Grow(pdpa: *mut _DPA, cp: i32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DPA_Grow(::std::mem::transmute(pdpa), ::std::mem::transmute(cp)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn DPA_InsertPtr(hdpa: *mut _DPA, i: i32, p: *const ::std::ffi::c_void) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_InsertPtr(hdpa: *mut _DPA, i: i32, p: *const ::std::ffi::c_void) -> i32;
        }
        ::std::mem::transmute(DPA_InsertPtr(::std::mem::transmute(hdpa), ::std::mem::transmute(i), ::std::mem::transmute(p)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn DPA_LoadStream<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(phdpa: *mut *mut _DPA, pfn: ::std::option::Option<PFNDPASTREAM>, pstream: Param2, pvinstdata: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_LoadStream(phdpa: *mut *mut _DPA, pfn: ::windows::runtime::RawPtr, pstream: ::windows::runtime::RawPtr, pvinstdata: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        DPA_LoadStream(::std::mem::transmute(phdpa), ::std::mem::transmute(pfn), pstream.into_param().abi(), ::std::mem::transmute(pvinstdata)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DPA_Merge<'a, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(hdpadest: *mut _DPA, hdpasrc: *const _DPA, dwflags: u32, pfncompare: ::std::option::Option<PFNDACOMPARE>, pfnmerge: ::std::option::Option<PFNDPAMERGE>, lparam: Param5) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_Merge(hdpadest: *mut _DPA, hdpasrc: *const _DPA, dwflags: u32, pfncompare: ::windows::runtime::RawPtr, pfnmerge: ::windows::runtime::RawPtr, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DPA_Merge(::std::mem::transmute(hdpadest), ::std::mem::transmute(hdpasrc), ::std::mem::transmute(dwflags), ::std::mem::transmute(pfncompare), ::std::mem::transmute(pfnmerge), lparam.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn DPA_SaveStream<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(hdpa: *const _DPA, pfn: ::std::option::Option<PFNDPASTREAM>, pstream: Param2, pvinstdata: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_SaveStream(hdpa: *const _DPA, pfn: ::windows::runtime::RawPtr, pstream: ::windows::runtime::RawPtr, pvinstdata: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        DPA_SaveStream(::std::mem::transmute(hdpa), ::std::mem::transmute(pfn), pstream.into_param().abi(), ::std::mem::transmute(pvinstdata)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DPA_Search<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(hdpa: *const _DPA, pfind: *const ::std::ffi::c_void, istart: i32, pfncompare: ::std::option::Option<PFNDACOMPARE>, lparam: Param4, options: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_Search(hdpa: *const _DPA, pfind: *const ::std::ffi::c_void, istart: i32, pfncompare: ::windows::runtime::RawPtr, lparam: super::super::Foundation::LPARAM, options: u32) -> i32;
        }
        ::std::mem::transmute(DPA_Search(::std::mem::transmute(hdpa), ::std::mem::transmute(pfind), ::std::mem::transmute(istart), ::std::mem::transmute(pfncompare), lparam.into_param().abi(), ::std::mem::transmute(options)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DPA_SetPtr(hdpa: *mut _DPA, i: i32, p: *const ::std::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_SetPtr(hdpa: *mut _DPA, i: i32, p: *const ::std::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DPA_SetPtr(::std::mem::transmute(hdpa), ::std::mem::transmute(i), ::std::mem::transmute(p)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DPA_Sort<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(hdpa: *mut _DPA, pfncompare: ::std::option::Option<PFNDACOMPARE>, lparam: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DPA_Sort(hdpa: *mut _DPA, pfncompare: ::windows::runtime::RawPtr, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DPA_Sort(::std::mem::transmute(hdpa), ::std::mem::transmute(pfncompare), lparam.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct DRAGLISTINFO {
    pub uNotification: DRAGLISTINFO_NOTIFICATION_FLAGS,
    pub hWnd: super::super::Foundation::HWND,
    pub ptCursor: super::super::Foundation::POINT,
}
#[cfg(feature = "Win32_Foundation")]
impl DRAGLISTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DRAGLISTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DRAGLISTINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DRAGLISTINFO").field("uNotification", &self.uNotification).field("hWnd", &self.hWnd).field("ptCursor", &self.ptCursor).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DRAGLISTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.uNotification == other.uNotification && self.hWnd == other.hWnd && self.ptCursor == other.ptCursor
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DRAGLISTINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DRAGLISTINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DRAGLISTINFO_NOTIFICATION_FLAGS(pub u32);
pub const DL_BEGINDRAG: DRAGLISTINFO_NOTIFICATION_FLAGS = DRAGLISTINFO_NOTIFICATION_FLAGS(1157u32);
pub const DL_CANCELDRAG: DRAGLISTINFO_NOTIFICATION_FLAGS = DRAGLISTINFO_NOTIFICATION_FLAGS(1160u32);
pub const DL_DRAGGING: DRAGLISTINFO_NOTIFICATION_FLAGS = DRAGLISTINFO_NOTIFICATION_FLAGS(1158u32);
pub const DL_DROPPED: DRAGLISTINFO_NOTIFICATION_FLAGS = DRAGLISTINFO_NOTIFICATION_FLAGS(1159u32);
impl ::std::convert::From<u32> for DRAGLISTINFO_NOTIFICATION_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DRAGLISTINFO_NOTIFICATION_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for DRAGLISTINFO_NOTIFICATION_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DRAGLISTINFO_NOTIFICATION_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DRAGLISTINFO_NOTIFICATION_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DRAGLISTINFO_NOTIFICATION_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DRAGLISTINFO_NOTIFICATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct DRAWITEMSTRUCT {
    pub CtlType: DRAWITEMSTRUCT_CTL_TYPE,
    pub CtlID: u32,
    pub itemID: u32,
    pub itemAction: u32,
    pub itemState: u32,
    pub hwndItem: super::super::Foundation::HWND,
    pub hDC: super::super::Graphics::Gdi::HDC,
    pub rcItem: super::super::Foundation::RECT,
    pub itemData: usize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl DRAWITEMSTRUCT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for DRAWITEMSTRUCT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for DRAWITEMSTRUCT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DRAWITEMSTRUCT")
            .field("CtlType", &self.CtlType)
            .field("CtlID", &self.CtlID)
            .field("itemID", &self.itemID)
            .field("itemAction", &self.itemAction)
            .field("itemState", &self.itemState)
            .field("hwndItem", &self.hwndItem)
            .field("hDC", &self.hDC)
            .field("rcItem", &self.rcItem)
            .field("itemData", &self.itemData)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for DRAWITEMSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.CtlType == other.CtlType && self.CtlID == other.CtlID && self.itemID == other.itemID && self.itemAction == other.itemAction && self.itemState == other.itemState && self.hwndItem == other.hwndItem && self.hDC == other.hDC && self.rcItem == other.rcItem && self.itemData == other.itemData
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for DRAWITEMSTRUCT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for DRAWITEMSTRUCT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DRAWITEMSTRUCT_CTL_TYPE(pub u32);
pub const ODT_BUTTON: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(4u32);
pub const ODT_COMBOBOX: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(3u32);
pub const ODT_LISTBOX: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(2u32);
pub const ODT_LISTVIEW: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(102u32);
pub const ODT_MENU: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(1u32);
pub const ODT_STATIC: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(5u32);
pub const ODT_TAB: DRAWITEMSTRUCT_CTL_TYPE = DRAWITEMSTRUCT_CTL_TYPE(101u32);
impl ::std::convert::From<u32> for DRAWITEMSTRUCT_CTL_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DRAWITEMSTRUCT_CTL_TYPE {
    type Abi = Self;
}
impl ::std::ops::BitOr for DRAWITEMSTRUCT_CTL_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DRAWITEMSTRUCT_CTL_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DRAWITEMSTRUCT_CTL_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DRAWITEMSTRUCT_CTL_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DRAWITEMSTRUCT_CTL_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DRAW_THEME_PARENT_BACKGROUND_FLAGS(pub u32);
pub const DTPB_WINDOWDC: DRAW_THEME_PARENT_BACKGROUND_FLAGS = DRAW_THEME_PARENT_BACKGROUND_FLAGS(1u32);
pub const DTPB_USECTLCOLORSTATIC: DRAW_THEME_PARENT_BACKGROUND_FLAGS = DRAW_THEME_PARENT_BACKGROUND_FLAGS(2u32);
pub const DTPB_USEERASEBKGND: DRAW_THEME_PARENT_BACKGROUND_FLAGS = DRAW_THEME_PARENT_BACKGROUND_FLAGS(4u32);
impl ::std::convert::From<u32> for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DRAW_THEME_PARENT_BACKGROUND_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DSA_APPEND: u32 = 2147483647u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn DSA_Clone(hdsa: *const _DSA) -> *mut _DSA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSA_Clone(hdsa: *const _DSA) -> *mut _DSA;
        }
        ::std::mem::transmute(DSA_Clone(::std::mem::transmute(hdsa)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn DSA_Create(cbitem: i32, citemgrow: i32) -> *mut _DSA {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSA_Create(cbitem: i32, citemgrow: i32) -> *mut _DSA;
        }
        ::std::mem::transmute(DSA_Create(::std::mem::transmute(cbitem), ::std::mem::transmute(citemgrow)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DSA_DeleteAllItems(hdsa: *mut _DSA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSA_DeleteAllItems(hdsa: *mut _DSA) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DSA_DeleteAllItems(::std::mem::transmute(hdsa)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DSA_DeleteItem(hdsa: *mut _DSA, i: i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSA_DeleteItem(hdsa: *mut _DSA, i: i32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DSA_DeleteItem(::std::mem::transmute(hdsa), ::std::mem::transmute(i)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DSA_Destroy(hdsa: *mut _DSA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSA_Destroy(hdsa: *mut _DSA) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DSA_Destroy(::std::mem::transmute(hdsa)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn DSA_DestroyCallback(hdsa: *mut _DSA, pfncb: ::std::option::Option<PFNDAENUMCALLBACK>, pdata: *const ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSA_DestroyCallback(hdsa: *mut _DSA, pfncb: ::windows::runtime::RawPtr, pdata: *const ::std::ffi::c_void);
        }
        ::std::mem::transmute(DSA_DestroyCallback(::std::mem::transmute(hdsa), ::std::mem::transmute(pfncb), ::std::mem::transmute(pdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DSA_ERR: i32 = -1i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn DSA_EnumCallback(hdsa: *const _DSA, pfncb: ::std::option::Option<PFNDAENUMCALLBACK>, pdata: *const ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSA_EnumCallback(hdsa: *const _DSA, pfncb: ::windows::runtime::RawPtr, pdata: *const ::std::ffi::c_void);
        }
        ::std::mem::transmute(DSA_EnumCallback(::std::mem::transmute(hdsa), ::std::mem::transmute(pfncb), ::std::mem::transmute(pdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DSA_GetItem(hdsa: *const _DSA, i: i32, pitem: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSA_GetItem(hdsa: *const _DSA, i: i32, pitem: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DSA_GetItem(::std::mem::transmute(hdsa), ::std::mem::transmute(i), ::std::mem::transmute(pitem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn DSA_GetItemPtr(hdsa: *const _DSA, i: i32) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSA_GetItemPtr(hdsa: *const _DSA, i: i32) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(DSA_GetItemPtr(::std::mem::transmute(hdsa), ::std::mem::transmute(i)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn DSA_GetSize(hdsa: *const _DSA) -> u64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSA_GetSize(hdsa: *const _DSA) -> u64;
        }
        ::std::mem::transmute(DSA_GetSize(::std::mem::transmute(hdsa)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn DSA_InsertItem(hdsa: *mut _DSA, i: i32, pitem: *const ::std::ffi::c_void) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSA_InsertItem(hdsa: *mut _DSA, i: i32, pitem: *const ::std::ffi::c_void) -> i32;
        }
        ::std::mem::transmute(DSA_InsertItem(::std::mem::transmute(hdsa), ::std::mem::transmute(i), ::std::mem::transmute(pitem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DSA_SetItem(hdsa: *mut _DSA, i: i32, pitem: *const ::std::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSA_SetItem(hdsa: *mut _DSA, i: i32, pitem: *const ::std::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DSA_SetItem(::std::mem::transmute(hdsa), ::std::mem::transmute(i), ::std::mem::transmute(pitem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DSA_Sort<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(pdsa: *mut _DSA, pfncompare: ::std::option::Option<PFNDACOMPARE>, lparam: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DSA_Sort(pdsa: *mut _DSA, pfncompare: ::windows::runtime::RawPtr, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DSA_Sort(::std::mem::transmute(pdsa), ::std::mem::transmute(pfncompare), lparam.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct DTBGOPTS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub rcClip: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl DTBGOPTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DTBGOPTS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DTBGOPTS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DTBGOPTS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("rcClip", &self.rcClip).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DTBGOPTS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.rcClip == other.rcClip
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DTBGOPTS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DTBGOPTS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTBG_CLIPRECT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTBG_COMPUTINGREGION: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTBG_DRAWSOLID: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTBG_MIRRORDC: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTBG_NOMIRROR: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTBG_OMITBORDER: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTBG_OMITCONTENT: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTM_CLOSEMONTHCAL: u32 = 4109u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTM_FIRST: u32 = 4096u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTM_GETDATETIMEPICKERINFO: u32 = 4110u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTM_GETIDEALSIZE: u32 = 4111u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTM_GETMCCOLOR: u32 = 4103u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTM_GETMCFONT: u32 = 4106u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTM_GETMCSTYLE: u32 = 4108u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTM_GETMONTHCAL: u32 = 4104u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTM_GETRANGE: u32 = 4099u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTM_GETSYSTEMTIME: u32 = 4097u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTM_SETFORMAT: u32 = 4146u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTM_SETFORMATA: u32 = 4101u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTM_SETFORMATW: u32 = 4146u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTM_SETMCCOLOR: u32 = 4102u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTM_SETMCFONT: u32 = 4105u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTM_SETMCSTYLE: u32 = 4107u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTM_SETRANGE: u32 = 4100u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTM_SETSYSTEMTIME: u32 = 4098u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTS_APPCANPARSE: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTS_LONGDATEFORMAT: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTS_RIGHTALIGN: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTS_SHORTDATECENTURYFORMAT: u32 = 12u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTS_SHORTDATEFORMAT: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTS_SHOWNONE: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTS_TIMEFORMAT: u32 = 9u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTS_UPDOWN: u32 = 1u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct DTTOPTS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub crText: u32,
    pub crBorder: u32,
    pub crShadow: u32,
    pub iTextShadowType: i32,
    pub ptShadowOffset: super::super::Foundation::POINT,
    pub iBorderSize: i32,
    pub iFontPropId: i32,
    pub iColorPropId: i32,
    pub iStateId: i32,
    pub fApplyOverlay: super::super::Foundation::BOOL,
    pub iGlowSize: i32,
    pub pfnDrawTextCallback: ::std::option::Option<DTT_CALLBACK_PROC>,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl DTTOPTS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for DTTOPTS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for DTTOPTS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DTTOPTS")
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("crText", &self.crText)
            .field("crBorder", &self.crBorder)
            .field("crShadow", &self.crShadow)
            .field("iTextShadowType", &self.iTextShadowType)
            .field("ptShadowOffset", &self.ptShadowOffset)
            .field("iBorderSize", &self.iBorderSize)
            .field("iFontPropId", &self.iFontPropId)
            .field("iColorPropId", &self.iColorPropId)
            .field("iStateId", &self.iStateId)
            .field("fApplyOverlay", &self.fApplyOverlay)
            .field("iGlowSize", &self.iGlowSize)
            .field("lParam", &self.lParam)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for DTTOPTS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwFlags == other.dwFlags
            && self.crText == other.crText
            && self.crBorder == other.crBorder
            && self.crShadow == other.crShadow
            && self.iTextShadowType == other.iTextShadowType
            && self.ptShadowOffset == other.ptShadowOffset
            && self.iBorderSize == other.iBorderSize
            && self.iFontPropId == other.iFontPropId
            && self.iColorPropId == other.iColorPropId
            && self.iStateId == other.iStateId
            && self.fApplyOverlay == other.fApplyOverlay
            && self.iGlowSize == other.iGlowSize
            && self.pfnDrawTextCallback.map(|f| f as usize) == other.pfnDrawTextCallback.map(|f| f as usize)
            && self.lParam == other.lParam
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for DTTOPTS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for DTTOPTS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type DTT_CALLBACK_PROC = unsafe extern "system" fn(hdc: super::super::Graphics::Gdi::HDC, psztext: super::super::Foundation::PWSTR, cchtext: i32, prc: *mut super::super::Foundation::RECT, dwflags: u32, lparam: super::super::Foundation::LPARAM) -> i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTT_FLAGS2VALIDBITS: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const DTT_GRAYED: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DestroyPropertySheetPage<'a, Param0: ::windows::runtime::IntoParam<'a, HPROPSHEETPAGE>>(param0: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DestroyPropertySheetPage(param0: HPROPSHEETPAGE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DestroyPropertySheetPage(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn DestroySyntheticPointerDevice<'a, Param0: ::windows::runtime::IntoParam<'a, HSYNTHETICPOINTERDEVICE>>(device: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DestroySyntheticPointerDevice(device: HSYNTHETICPOINTERDEVICE);
        }
        ::std::mem::transmute(DestroySyntheticPointerDevice(device.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DlgDirListA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hdlg: Param0, lppathspec: Param1, nidlistbox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DlgDirListA(hdlg: super::super::Foundation::HWND, lppathspec: super::super::Foundation::PSTR, nidlistbox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32;
        }
        ::std::mem::transmute(DlgDirListA(hdlg.into_param().abi(), lppathspec.into_param().abi(), ::std::mem::transmute(nidlistbox), ::std::mem::transmute(nidstaticpath), ::std::mem::transmute(ufiletype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DlgDirListComboBoxA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hdlg: Param0, lppathspec: Param1, nidcombobox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DlgDirListComboBoxA(hdlg: super::super::Foundation::HWND, lppathspec: super::super::Foundation::PSTR, nidcombobox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32;
        }
        ::std::mem::transmute(DlgDirListComboBoxA(hdlg.into_param().abi(), lppathspec.into_param().abi(), ::std::mem::transmute(nidcombobox), ::std::mem::transmute(nidstaticpath), ::std::mem::transmute(ufiletype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DlgDirListComboBoxW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hdlg: Param0, lppathspec: Param1, nidcombobox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DlgDirListComboBoxW(hdlg: super::super::Foundation::HWND, lppathspec: super::super::Foundation::PWSTR, nidcombobox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32;
        }
        ::std::mem::transmute(DlgDirListComboBoxW(hdlg.into_param().abi(), lppathspec.into_param().abi(), ::std::mem::transmute(nidcombobox), ::std::mem::transmute(nidstaticpath), ::std::mem::transmute(ufiletype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DlgDirListW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hdlg: Param0, lppathspec: Param1, nidlistbox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DlgDirListW(hdlg: super::super::Foundation::HWND, lppathspec: super::super::Foundation::PWSTR, nidlistbox: i32, nidstaticpath: i32, ufiletype: DLG_DIR_LIST_FILE_TYPE) -> i32;
        }
        ::std::mem::transmute(DlgDirListW(hdlg.into_param().abi(), lppathspec.into_param().abi(), ::std::mem::transmute(nidlistbox), ::std::mem::transmute(nidstaticpath), ::std::mem::transmute(ufiletype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DlgDirSelectComboBoxExA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnddlg: Param0, lpstring: super::super::Foundation::PSTR, cchout: i32, idcombobox: i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DlgDirSelectComboBoxExA(hwnddlg: super::super::Foundation::HWND, lpstring: super::super::Foundation::PSTR, cchout: i32, idcombobox: i32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DlgDirSelectComboBoxExA(hwnddlg.into_param().abi(), ::std::mem::transmute(lpstring), ::std::mem::transmute(cchout), ::std::mem::transmute(idcombobox)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DlgDirSelectComboBoxExW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnddlg: Param0, lpstring: super::super::Foundation::PWSTR, cchout: i32, idcombobox: i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DlgDirSelectComboBoxExW(hwnddlg: super::super::Foundation::HWND, lpstring: super::super::Foundation::PWSTR, cchout: i32, idcombobox: i32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DlgDirSelectComboBoxExW(hwnddlg.into_param().abi(), ::std::mem::transmute(lpstring), ::std::mem::transmute(cchout), ::std::mem::transmute(idcombobox)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DlgDirSelectExA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnddlg: Param0, lpstring: super::super::Foundation::PSTR, chcount: i32, idlistbox: i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DlgDirSelectExA(hwnddlg: super::super::Foundation::HWND, lpstring: super::super::Foundation::PSTR, chcount: i32, idlistbox: i32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DlgDirSelectExA(hwnddlg.into_param().abi(), ::std::mem::transmute(lpstring), ::std::mem::transmute(chcount), ::std::mem::transmute(idlistbox)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DlgDirSelectExW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnddlg: Param0, lpstring: super::super::Foundation::PWSTR, chcount: i32, idlistbox: i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DlgDirSelectExW(hwnddlg: super::super::Foundation::HWND, lpstring: super::super::Foundation::PWSTR, chcount: i32, idlistbox: i32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DlgDirSelectExW(hwnddlg.into_param().abi(), ::std::mem::transmute(lpstring), ::std::mem::transmute(chcount), ::std::mem::transmute(idlistbox)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DrawInsert<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(handparent: Param0, hlb: Param1, nitem: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawInsert(handparent: super::super::Foundation::HWND, hlb: super::super::Foundation::HWND, nitem: i32);
        }
        ::std::mem::transmute(DrawInsert(handparent.into_param().abi(), hlb.into_param().abi(), ::std::mem::transmute(nitem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawShadowText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hdc: Param0, psztext: Param1, cch: u32, prc: *const super::super::Foundation::RECT, dwflags: u32, crtext: u32, crshadow: u32, ixoffset: i32, iyoffset: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawShadowText(hdc: super::super::Graphics::Gdi::HDC, psztext: super::super::Foundation::PWSTR, cch: u32, prc: *const super::super::Foundation::RECT, dwflags: u32, crtext: u32, crshadow: u32, ixoffset: i32, iyoffset: i32) -> i32;
        }
        ::std::mem::transmute(DrawShadowText(hdc.into_param().abi(), psztext.into_param().abi(), ::std::mem::transmute(cch), ::std::mem::transmute(prc), ::std::mem::transmute(dwflags), ::std::mem::transmute(crtext), ::std::mem::transmute(crshadow), ::std::mem::transmute(ixoffset), ::std::mem::transmute(iyoffset)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawStatusTextA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hdc: Param0, lprc: *mut super::super::Foundation::RECT, psztext: Param2, uflags: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawStatusTextA(hdc: super::super::Graphics::Gdi::HDC, lprc: *mut super::super::Foundation::RECT, psztext: super::super::Foundation::PSTR, uflags: u32);
        }
        ::std::mem::transmute(DrawStatusTextA(hdc.into_param().abi(), ::std::mem::transmute(lprc), psztext.into_param().abi(), ::std::mem::transmute(uflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawStatusTextW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hdc: Param0, lprc: *mut super::super::Foundation::RECT, psztext: Param2, uflags: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawStatusTextW(hdc: super::super::Graphics::Gdi::HDC, lprc: *mut super::super::Foundation::RECT, psztext: super::super::Foundation::PWSTR, uflags: u32);
        }
        ::std::mem::transmute(DrawStatusTextW(hdc.into_param().abi(), ::std::mem::transmute(lprc), psztext.into_param().abi(), ::std::mem::transmute(uflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawThemeBackground<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, prect: *const super::super::Foundation::RECT, pcliprect: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawThemeBackground(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, prect: *const super::super::Foundation::RECT, pcliprect: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT;
        }
        DrawThemeBackground(::std::mem::transmute(htheme), hdc.into_param().abi(), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), ::std::mem::transmute(prect), ::std::mem::transmute(pcliprect)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawThemeBackgroundEx<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, prect: *const super::super::Foundation::RECT, poptions: *const DTBGOPTS) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawThemeBackgroundEx(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, prect: *const super::super::Foundation::RECT, poptions: *const DTBGOPTS) -> ::windows::runtime::HRESULT;
        }
        DrawThemeBackgroundEx(::std::mem::transmute(htheme), hdc.into_param().abi(), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), ::std::mem::transmute(prect), ::std::mem::transmute(poptions)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawThemeEdge<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, pdestrect: *const super::super::Foundation::RECT, uedge: u32, uflags: u32) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawThemeEdge(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, pdestrect: *const super::super::Foundation::RECT, uedge: u32, uflags: u32, pcontentrect: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DrawThemeEdge(::std::mem::transmute(htheme), hdc.into_param().abi(), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), ::std::mem::transmute(pdestrect), ::std::mem::transmute(uedge), ::std::mem::transmute(uflags), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawThemeIcon<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>, Param5: ::windows::runtime::IntoParam<'a, HIMAGELIST>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, prect: *const super::super::Foundation::RECT, himl: Param5, iimageindex: i32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawThemeIcon(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, prect: *const super::super::Foundation::RECT, himl: HIMAGELIST, iimageindex: i32) -> ::windows::runtime::HRESULT;
        }
        DrawThemeIcon(::std::mem::transmute(htheme), hdc.into_param().abi(), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), ::std::mem::transmute(prect), himl.into_param().abi(), ::std::mem::transmute(iimageindex)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawThemeParentBackground<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hwnd: Param0, hdc: Param1, prc: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawThemeParentBackground(hwnd: super::super::Foundation::HWND, hdc: super::super::Graphics::Gdi::HDC, prc: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT;
        }
        DrawThemeParentBackground(hwnd.into_param().abi(), hdc.into_param().abi(), ::std::mem::transmute(prc)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawThemeParentBackgroundEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hwnd: Param0, hdc: Param1, dwflags: DRAW_THEME_PARENT_BACKGROUND_FLAGS, prc: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawThemeParentBackgroundEx(hwnd: super::super::Foundation::HWND, hdc: super::super::Graphics::Gdi::HDC, dwflags: DRAW_THEME_PARENT_BACKGROUND_FLAGS, prc: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT;
        }
        DrawThemeParentBackgroundEx(hwnd.into_param().abi(), hdc.into_param().abi(), ::std::mem::transmute(dwflags), ::std::mem::transmute(prc)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawThemeText<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, psztext: Param4, cchtext: i32, dwtextflags: u32, dwtextflags2: u32, prect: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawThemeText(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, psztext: super::super::Foundation::PWSTR, cchtext: i32, dwtextflags: u32, dwtextflags2: u32, prect: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT;
        }
        DrawThemeText(::std::mem::transmute(htheme), hdc.into_param().abi(), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), psztext.into_param().abi(), ::std::mem::transmute(cchtext), ::std::mem::transmute(dwtextflags), ::std::mem::transmute(dwtextflags2), ::std::mem::transmute(prect)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DrawThemeTextEx<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, psztext: Param4, cchtext: i32, dwtextflags: u32, prect: *mut super::super::Foundation::RECT, poptions: *const DTTOPTS) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DrawThemeTextEx(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, psztext: super::super::Foundation::PWSTR, cchtext: i32, dwtextflags: u32, prect: *mut super::super::Foundation::RECT, poptions: *const ::std::mem::ManuallyDrop<DTTOPTS>) -> ::windows::runtime::HRESULT;
        }
        DrawThemeTextEx(::std::mem::transmute(htheme), hdc.into_param().abi(), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), psztext.into_param().abi(), ::std::mem::transmute(cchtext), ::std::mem::transmute(dwtextflags), ::std::mem::transmute(prect), ::std::mem::transmute(poptions)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ECM_FIRST: u32 = 5376u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct EC_ENDOFLINE(pub i32);
pub const EC_ENDOFLINE_DETECTFROMCONTENT: EC_ENDOFLINE = EC_ENDOFLINE(0i32);
pub const EC_ENDOFLINE_CRLF: EC_ENDOFLINE = EC_ENDOFLINE(1i32);
pub const EC_ENDOFLINE_CR: EC_ENDOFLINE = EC_ENDOFLINE(2i32);
pub const EC_ENDOFLINE_LF: EC_ENDOFLINE = EC_ENDOFLINE(3i32);
impl ::std::convert::From<i32> for EC_ENDOFLINE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EC_ENDOFLINE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct EC_SEARCHWEB_ENTRYPOINT(pub i32);
pub const EC_SEARCHWEB_ENTRYPOINT_EXTERNAL: EC_SEARCHWEB_ENTRYPOINT = EC_SEARCHWEB_ENTRYPOINT(0i32);
pub const EC_SEARCHWEB_ENTRYPOINT_CONTEXTMENU: EC_SEARCHWEB_ENTRYPOINT = EC_SEARCHWEB_ENTRYPOINT(1i32);
impl ::std::convert::From<i32> for EC_SEARCHWEB_ENTRYPOINT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EC_SEARCHWEB_ENTRYPOINT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct EDITBALLOONTIP {
    pub cbStruct: u32,
    pub pszTitle: super::super::Foundation::PWSTR,
    pub pszText: super::super::Foundation::PWSTR,
    pub ttiIcon: EDITBALLOONTIP_ICON,
}
#[cfg(feature = "Win32_Foundation")]
impl EDITBALLOONTIP {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EDITBALLOONTIP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for EDITBALLOONTIP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EDITBALLOONTIP").field("cbStruct", &self.cbStruct).field("pszTitle", &self.pszTitle).field("pszText", &self.pszText).field("ttiIcon", &self.ttiIcon).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EDITBALLOONTIP {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.pszTitle == other.pszTitle && self.pszText == other.pszText && self.ttiIcon == other.ttiIcon
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EDITBALLOONTIP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EDITBALLOONTIP {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct EDITBALLOONTIP_ICON(pub u32);
pub const TTI_ERROR: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(3u32);
pub const TTI_INFO: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(1u32);
pub const TTI_NONE: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(0u32);
pub const TTI_WARNING: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(2u32);
pub const TTI_INFO_LARGE: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(4u32);
pub const TTI_WARNING_LARGE: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(5u32);
pub const TTI_ERROR_LARGE: EDITBALLOONTIP_ICON = EDITBALLOONTIP_ICON(6u32);
impl ::std::convert::From<u32> for EDITBALLOONTIP_ICON {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EDITBALLOONTIP_ICON {
    type Abi = Self;
}
impl ::std::ops::BitOr for EDITBALLOONTIP_ICON {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for EDITBALLOONTIP_ICON {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for EDITBALLOONTIP_ICON {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for EDITBALLOONTIP_ICON {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for EDITBALLOONTIP_ICON {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
pub type EDITWORDBREAKPROCA = unsafe extern "system" fn(lpch: super::super::Foundation::PSTR, ichcurrent: i32, cch: i32, code: WORD_BREAK_ACTION) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type EDITWORDBREAKPROCW = unsafe extern "system" fn(lpch: super::super::Foundation::PWSTR, ichcurrent: i32, cch: i32, code: WORD_BREAK_ACTION) -> i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct EMPTYMARKUPPARTS(pub i32);
pub const EMP_MARKUPTEXT: EMPTYMARKUPPARTS = EMPTYMARKUPPARTS(1i32);
impl ::std::convert::From<i32> for EMPTYMARKUPPARTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EMPTYMARKUPPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_CANUNDO: u32 = 198u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_CHARFROMPOS: u32 = 215u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_EMPTYUNDOBUFFER: u32 = 205u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_ENABLEFEATURE: u32 = 218u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_ENABLESEARCHWEB: u32 = 5390u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_FILELINEFROMCHAR: u32 = 5395u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_FILELINEINDEX: u32 = 5396u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_FILELINELENGTH: u32 = 5397u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_FMTLINES: u32 = 200u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_GETCARETINDEX: u32 = 5394u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_GETCUEBANNER: u32 = 5378u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_GETENDOFLINE: u32 = 5389u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_GETEXTENDEDSTYLE: u32 = 5387u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_GETFILELINE: u32 = 5398u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_GETFILELINECOUNT: u32 = 5399u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_GETFIRSTVISIBLELINE: u32 = 206u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_GETHANDLE: u32 = 189u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_GETHILITE: u32 = 5382u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_GETIMESTATUS: u32 = 217u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_GETLIMITTEXT: u32 = 213u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_GETLINE: u32 = 196u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_GETLINECOUNT: u32 = 186u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_GETMARGINS: u32 = 212u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_GETMODIFY: u32 = 184u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_GETPASSWORDCHAR: u32 = 210u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_GETRECT: u32 = 178u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_GETSEL: u32 = 176u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_GETTHUMB: u32 = 190u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_GETWORDBREAKPROC: u32 = 209u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_HIDEBALLOONTIP: u32 = 5380u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_LIMITTEXT: u32 = 197u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_LINEFROMCHAR: u32 = 201u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_LINEINDEX: u32 = 187u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_LINELENGTH: u32 = 193u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_LINESCROLL: u32 = 182u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_NOSETFOCUS: u32 = 5383u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_POSFROMCHAR: u32 = 214u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_REPLACESEL: u32 = 194u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_SCROLL: u32 = 181u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_SCROLLCARET: u32 = 183u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_SEARCHWEB: u32 = 5391u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_SETCARETINDEX: u32 = 5393u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_SETCUEBANNER: u32 = 5377u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_SETENDOFLINE: u32 = 5388u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_SETEXTENDEDSTYLE: u32 = 5386u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_SETHANDLE: u32 = 188u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_SETHILITE: u32 = 5381u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_SETIMESTATUS: u32 = 216u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_SETLIMITTEXT: u32 = 197u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_SETMARGINS: u32 = 211u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_SETMODIFY: u32 = 185u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_SETPASSWORDCHAR: u32 = 204u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_SETREADONLY: u32 = 207u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_SETRECT: u32 = 179u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_SETRECTNP: u32 = 180u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_SETSEL: u32 = 177u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_SETTABSTOPS: u32 = 203u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_SETWORDBREAKPROC: u32 = 208u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_SHOWBALLOONTIP: u32 = 5379u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_TAKEFOCUS: u32 = 5384u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const EM_UNDO: u32 = 199u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ENABLE_SCROLL_BAR_ARROWS(pub u32);
pub const ESB_DISABLE_BOTH: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(3u32);
pub const ESB_DISABLE_DOWN: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(2u32);
pub const ESB_DISABLE_LEFT: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(1u32);
pub const ESB_DISABLE_LTUP: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(1u32);
pub const ESB_DISABLE_RIGHT: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(2u32);
pub const ESB_DISABLE_RTDN: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(2u32);
pub const ESB_DISABLE_UP: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(1u32);
pub const ESB_ENABLE_BOTH: ENABLE_SCROLL_BAR_ARROWS = ENABLE_SCROLL_BAR_ARROWS(0u32);
impl ::std::convert::From<u32> for ENABLE_SCROLL_BAR_ARROWS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ENABLE_SCROLL_BAR_ARROWS {
    type Abi = Self;
}
impl ::std::ops::BitOr for ENABLE_SCROLL_BAR_ARROWS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for ENABLE_SCROLL_BAR_ARROWS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for ENABLE_SCROLL_BAR_ARROWS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for ENABLE_SCROLL_BAR_ARROWS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for ENABLE_SCROLL_BAR_ARROWS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ES_EX_ALLOWEOL_CR: i32 = 1i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ES_EX_ALLOWEOL_LF: i32 = 2i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ES_EX_CONVERT_EOL_ON_PASTE: i32 = 4i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ES_EX_ZOOMABLE: i32 = 16i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ETDT_DISABLE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ETDT_ENABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ETDT_USEAEROWIZARDTABTEXTURE: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ETDT_USETABTEXTURE: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn EnableScrollBar<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, wsbflags: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, warrows: ENABLE_SCROLL_BAR_ARROWS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnableScrollBar(hwnd: super::super::Foundation::HWND, wsbflags: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, warrows: ENABLE_SCROLL_BAR_ARROWS) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnableScrollBar(hwnd.into_param().abi(), ::std::mem::transmute(wsbflags), ::std::mem::transmute(warrows)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnableThemeDialogTexture<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, dwflags: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnableThemeDialogTexture(hwnd: super::super::Foundation::HWND, dwflags: u32) -> ::windows::runtime::HRESULT;
        }
        EnableThemeDialogTexture(hwnd.into_param().abi(), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnableTheming<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(fenable: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnableTheming(fenable: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        EnableTheming(fenable.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EndBufferedAnimation<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(hbpanimation: isize, fupdatetarget: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EndBufferedAnimation(hbpanimation: isize, fupdatetarget: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        EndBufferedAnimation(::std::mem::transmute(hbpanimation), fupdatetarget.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EndBufferedPaint<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(hbufferedpaint: isize, fupdatetarget: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EndBufferedPaint(hbufferedpaint: isize, fupdatetarget: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        EndBufferedPaint(::std::mem::transmute(hbufferedpaint), fupdatetarget.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EndPanningFeedback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(hwnd: Param0, fanimateback: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EndPanningFeedback(hwnd: super::super::Foundation::HWND, fanimateback: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EndPanningFeedback(hwnd.into_param().abi(), fanimateback.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvaluateProximityToPolygon(numvertices: u32, controlpolygon: *const super::super::Foundation::POINT, phittestinginput: *const TOUCH_HIT_TESTING_INPUT, pproximityeval: *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvaluateProximityToPolygon(numvertices: u32, controlpolygon: *const super::super::Foundation::POINT, phittestinginput: *const TOUCH_HIT_TESTING_INPUT, pproximityeval: *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EvaluateProximityToPolygon(::std::mem::transmute(numvertices), ::std::mem::transmute(controlpolygon), ::std::mem::transmute(phittestinginput), ::std::mem::transmute(pproximityeval)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EvaluateProximityToRect(controlboundingbox: *const super::super::Foundation::RECT, phittestinginput: *const TOUCH_HIT_TESTING_INPUT, pproximityeval: *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EvaluateProximityToRect(controlboundingbox: *const super::super::Foundation::RECT, phittestinginput: *const TOUCH_HIT_TESTING_INPUT, pproximityeval: *mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EvaluateProximityToRect(::std::mem::transmute(controlboundingbox), ::std::mem::transmute(phittestinginput), ::std::mem::transmute(pproximityeval)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FEEDBACK_TYPE(pub i32);
pub const FEEDBACK_TOUCH_CONTACTVISUALIZATION: FEEDBACK_TYPE = FEEDBACK_TYPE(1i32);
pub const FEEDBACK_PEN_BARRELVISUALIZATION: FEEDBACK_TYPE = FEEDBACK_TYPE(2i32);
pub const FEEDBACK_PEN_TAP: FEEDBACK_TYPE = FEEDBACK_TYPE(3i32);
pub const FEEDBACK_PEN_DOUBLETAP: FEEDBACK_TYPE = FEEDBACK_TYPE(4i32);
pub const FEEDBACK_PEN_PRESSANDHOLD: FEEDBACK_TYPE = FEEDBACK_TYPE(5i32);
pub const FEEDBACK_PEN_RIGHTTAP: FEEDBACK_TYPE = FEEDBACK_TYPE(6i32);
pub const FEEDBACK_TOUCH_TAP: FEEDBACK_TYPE = FEEDBACK_TYPE(7i32);
pub const FEEDBACK_TOUCH_DOUBLETAP: FEEDBACK_TYPE = FEEDBACK_TYPE(8i32);
pub const FEEDBACK_TOUCH_PRESSANDHOLD: FEEDBACK_TYPE = FEEDBACK_TYPE(9i32);
pub const FEEDBACK_TOUCH_RIGHTTAP: FEEDBACK_TYPE = FEEDBACK_TYPE(10i32);
pub const FEEDBACK_GESTURE_PRESSANDTAP: FEEDBACK_TYPE = FEEDBACK_TYPE(11i32);
pub const FEEDBACK_MAX: FEEDBACK_TYPE = FEEDBACK_TYPE(-1i32);
impl ::std::convert::From<i32> for FEEDBACK_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FEEDBACK_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const FILEOPENORD: u32 = 1536u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FILLTYPE(pub i32);
pub const FT_SOLID: FILLTYPE = FILLTYPE(0i32);
pub const FT_VERTGRADIENT: FILLTYPE = FILLTYPE(1i32);
pub const FT_HORZGRADIENT: FILLTYPE = FILLTYPE(2i32);
pub const FT_RADIALGRADIENT: FILLTYPE = FILLTYPE(3i32);
pub const FT_TILEIMAGE: FILLTYPE = FILLTYPE(4i32);
impl ::std::convert::From<i32> for FILLTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FILLTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const FINDDLGORD: u32 = 1540u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const FONTDLGORD: u32 = 1542u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const FORMATDLGORD30: u32 = 1544u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const FORMATDLGORD31: u32 = 1543u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const FSB_ENCARTA_MODE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const FSB_FLAT_MODE: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const FSB_REGULAR_MODE: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlatSB_EnableScrollBar<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(param0: Param0, param1: i32, param2: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlatSB_EnableScrollBar(param0: super::super::Foundation::HWND, param1: i32, param2: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FlatSB_EnableScrollBar(param0.into_param().abi(), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn FlatSB_GetScrollInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(param0: Param0, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, param2: *mut super::WindowsAndMessaging::SCROLLINFO) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlatSB_GetScrollInfo(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, param2: *mut super::WindowsAndMessaging::SCROLLINFO) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FlatSB_GetScrollInfo(param0.into_param().abi(), ::std::mem::transmute(code), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn FlatSB_GetScrollPos<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(param0: Param0, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlatSB_GetScrollPos(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS) -> i32;
        }
        ::std::mem::transmute(FlatSB_GetScrollPos(param0.into_param().abi(), ::std::mem::transmute(code)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlatSB_GetScrollProp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(param0: Param0, propindex: WSB_PROP, param2: *mut i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlatSB_GetScrollProp(param0: super::super::Foundation::HWND, propindex: WSB_PROP, param2: *mut i32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FlatSB_GetScrollProp(param0.into_param().abi(), ::std::mem::transmute(propindex), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn FlatSB_GetScrollRange<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(param0: Param0, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, param2: *mut i32, param3: *mut i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlatSB_GetScrollRange(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, param2: *mut i32, param3: *mut i32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FlatSB_GetScrollRange(param0.into_param().abi(), ::std::mem::transmute(code), ::std::mem::transmute(param2), ::std::mem::transmute(param3)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn FlatSB_SetScrollInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(param0: Param0, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, psi: *mut super::WindowsAndMessaging::SCROLLINFO, fredraw: Param3) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlatSB_SetScrollInfo(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, psi: *mut super::WindowsAndMessaging::SCROLLINFO, fredraw: super::super::Foundation::BOOL) -> i32;
        }
        ::std::mem::transmute(FlatSB_SetScrollInfo(param0.into_param().abi(), ::std::mem::transmute(code), ::std::mem::transmute(psi), fredraw.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn FlatSB_SetScrollPos<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(param0: Param0, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, pos: i32, fredraw: Param3) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlatSB_SetScrollPos(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, pos: i32, fredraw: super::super::Foundation::BOOL) -> i32;
        }
        ::std::mem::transmute(FlatSB_SetScrollPos(param0.into_param().abi(), ::std::mem::transmute(code), ::std::mem::transmute(pos), fredraw.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlatSB_SetScrollProp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(param0: Param0, index: WSB_PROP, newvalue: isize, param3: Param3) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlatSB_SetScrollProp(param0: super::super::Foundation::HWND, index: WSB_PROP, newvalue: isize, param3: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FlatSB_SetScrollProp(param0.into_param().abi(), ::std::mem::transmute(index), ::std::mem::transmute(newvalue), param3.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn FlatSB_SetScrollRange<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(param0: Param0, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, min: i32, max: i32, fredraw: Param4) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlatSB_SetScrollRange(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, min: i32, max: i32, fredraw: super::super::Foundation::BOOL) -> i32;
        }
        ::std::mem::transmute(FlatSB_SetScrollRange(param0.into_param().abi(), ::std::mem::transmute(code), ::std::mem::transmute(min), ::std::mem::transmute(max), fredraw.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn FlatSB_ShowScrollBar<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(param0: Param0, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, param2: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlatSB_ShowScrollBar(param0: super::super::Foundation::HWND, code: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, param2: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FlatSB_ShowScrollBar(param0.into_param().abi(), ::std::mem::transmute(code), param2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const GDTR_MAX: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const GDTR_MIN: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const GDT_ERROR: i32 = -1i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const GDT_NONE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const GDT_VALID: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GET_THEME_BITMAP_FLAGS(pub u32);
pub const GBF_DIRECT: GET_THEME_BITMAP_FLAGS = GET_THEME_BITMAP_FLAGS(1u32);
pub const GBF_COPY: GET_THEME_BITMAP_FLAGS = GET_THEME_BITMAP_FLAGS(2u32);
pub const GBF_VALIDBITS: GET_THEME_BITMAP_FLAGS = GET_THEME_BITMAP_FLAGS(3u32);
impl ::std::convert::From<u32> for GET_THEME_BITMAP_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GET_THEME_BITMAP_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for GET_THEME_BITMAP_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for GET_THEME_BITMAP_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for GET_THEME_BITMAP_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for GET_THEME_BITMAP_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for GET_THEME_BITMAP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GLYPHFONTSIZINGTYPE(pub i32);
pub const GFST_NONE: GLYPHFONTSIZINGTYPE = GLYPHFONTSIZINGTYPE(0i32);
pub const GFST_SIZE: GLYPHFONTSIZINGTYPE = GLYPHFONTSIZINGTYPE(1i32);
pub const GFST_DPI: GLYPHFONTSIZINGTYPE = GLYPHFONTSIZINGTYPE(2i32);
impl ::std::convert::From<i32> for GLYPHFONTSIZINGTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GLYPHFONTSIZINGTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GLYPHTYPE(pub i32);
pub const GT_NONE: GLYPHTYPE = GLYPHTYPE(0i32);
pub const GT_IMAGEGLYPH: GLYPHTYPE = GLYPHTYPE(1i32);
pub const GT_FONTGLYPH: GLYPHTYPE = GLYPHTYPE(2i32);
impl ::std::convert::From<i32> for GLYPHTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GLYPHTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const GMR_DAYSTATE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const GMR_VISIBLE: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GRIDCELLBACKGROUNDSTATES(pub i32);
pub const MCGCB_SELECTED: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(1i32);
pub const MCGCB_HOT: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(2i32);
pub const MCGCB_SELECTEDHOT: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(3i32);
pub const MCGCB_SELECTEDNOTFOCUSED: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(4i32);
pub const MCGCB_TODAY: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(5i32);
pub const MCGCB_TODAYSELECTED: GRIDCELLBACKGROUNDSTATES = GRIDCELLBACKGROUNDSTATES(6i32);
impl ::std::convert::From<i32> for GRIDCELLBACKGROUNDSTATES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GRIDCELLBACKGROUNDSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GRIDCELLSTATES(pub i32);
pub const MCGC_HOT: GRIDCELLSTATES = GRIDCELLSTATES(1i32);
pub const MCGC_HASSTATE: GRIDCELLSTATES = GRIDCELLSTATES(2i32);
pub const MCGC_HASSTATEHOT: GRIDCELLSTATES = GRIDCELLSTATES(3i32);
pub const MCGC_TODAY: GRIDCELLSTATES = GRIDCELLSTATES(4i32);
pub const MCGC_TODAYSELECTED: GRIDCELLSTATES = GRIDCELLSTATES(5i32);
pub const MCGC_SELECTED: GRIDCELLSTATES = GRIDCELLSTATES(6i32);
pub const MCGC_SELECTEDHOT: GRIDCELLSTATES = GRIDCELLSTATES(7i32);
impl ::std::convert::From<i32> for GRIDCELLSTATES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GRIDCELLSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GRIDCELLUPPERSTATES(pub i32);
pub const MCGCU_HOT: GRIDCELLUPPERSTATES = GRIDCELLUPPERSTATES(1i32);
pub const MCGCU_HASSTATE: GRIDCELLUPPERSTATES = GRIDCELLUPPERSTATES(2i32);
pub const MCGCU_HASSTATEHOT: GRIDCELLUPPERSTATES = GRIDCELLUPPERSTATES(3i32);
pub const MCGCU_SELECTED: GRIDCELLUPPERSTATES = GRIDCELLUPPERSTATES(4i32);
pub const MCGCU_SELECTEDHOT: GRIDCELLUPPERSTATES = GRIDCELLUPPERSTATES(5i32);
impl ::std::convert::From<i32> for GRIDCELLUPPERSTATES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GRIDCELLUPPERSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetBufferedPaintBits(hbufferedpaint: isize, ppbbuffer: *mut *mut super::super::Graphics::Gdi::RGBQUAD, pcxrow: *mut i32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetBufferedPaintBits(hbufferedpaint: isize, ppbbuffer: *mut *mut super::super::Graphics::Gdi::RGBQUAD, pcxrow: *mut i32) -> ::windows::runtime::HRESULT;
        }
        GetBufferedPaintBits(::std::mem::transmute(hbufferedpaint), ::std::mem::transmute(ppbbuffer), ::std::mem::transmute(pcxrow)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetBufferedPaintDC(hbufferedpaint: isize) -> super::super::Graphics::Gdi::HDC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetBufferedPaintDC(hbufferedpaint: isize) -> super::super::Graphics::Gdi::HDC;
        }
        ::std::mem::transmute(GetBufferedPaintDC(::std::mem::transmute(hbufferedpaint)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetBufferedPaintTargetDC(hbufferedpaint: isize) -> super::super::Graphics::Gdi::HDC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetBufferedPaintTargetDC(hbufferedpaint: isize) -> super::super::Graphics::Gdi::HDC;
        }
        ::std::mem::transmute(GetBufferedPaintTargetDC(::std::mem::transmute(hbufferedpaint)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetBufferedPaintTargetRect(hbufferedpaint: isize) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetBufferedPaintTargetRect(hbufferedpaint: isize, prc: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetBufferedPaintTargetRect(::std::mem::transmute(hbufferedpaint), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetComboBoxInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwndcombo: Param0, pcbi: *mut COMBOBOXINFO) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetComboBoxInfo(hwndcombo: super::super::Foundation::HWND, pcbi: *mut COMBOBOXINFO) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetComboBoxInfo(hwndcombo.into_param().abi(), ::std::mem::transmute(pcbi)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentThemeName(pszthemefilename: super::super::Foundation::PWSTR, cchmaxnamechars: i32, pszcolorbuff: super::super::Foundation::PWSTR, cchmaxcolorchars: i32, pszsizebuff: super::super::Foundation::PWSTR, cchmaxsizechars: i32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentThemeName(pszthemefilename: super::super::Foundation::PWSTR, cchmaxnamechars: i32, pszcolorbuff: super::super::Foundation::PWSTR, cchmaxcolorchars: i32, pszsizebuff: super::super::Foundation::PWSTR, cchmaxsizechars: i32) -> ::windows::runtime::HRESULT;
        }
        GetCurrentThemeName(::std::mem::transmute(pszthemefilename), ::std::mem::transmute(cchmaxnamechars), ::std::mem::transmute(pszcolorbuff), ::std::mem::transmute(cchmaxcolorchars), ::std::mem::transmute(pszsizebuff), ::std::mem::transmute(cchmaxsizechars)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetEffectiveClientRect<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, lprc: *mut super::super::Foundation::RECT, lpinfo: *const i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetEffectiveClientRect(hwnd: super::super::Foundation::HWND, lprc: *mut super::super::Foundation::RECT, lpinfo: *const i32);
        }
        ::std::mem::transmute(GetEffectiveClientRect(hwnd.into_param().abi(), ::std::mem::transmute(lprc), ::std::mem::transmute(lpinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetListBoxInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetListBoxInfo(hwnd: super::super::Foundation::HWND) -> u32;
        }
        ::std::mem::transmute(GetListBoxInfo(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn GetMUILanguage() -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMUILanguage() -> u16;
        }
        ::std::mem::transmute(GetMUILanguage())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn GetThemeAnimationProperty(htheme: isize, istoryboardid: i32, itargetid: i32, eproperty: TA_PROPERTY, pvproperty: *mut ::std::ffi::c_void, cbsize: u32, pcbsizeout: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeAnimationProperty(htheme: isize, istoryboardid: i32, itargetid: i32, eproperty: TA_PROPERTY, pvproperty: *mut ::std::ffi::c_void, cbsize: u32, pcbsizeout: *mut u32) -> ::windows::runtime::HRESULT;
        }
        GetThemeAnimationProperty(::std::mem::transmute(htheme), ::std::mem::transmute(istoryboardid), ::std::mem::transmute(itargetid), ::std::mem::transmute(eproperty), ::std::mem::transmute(pvproperty), ::std::mem::transmute(cbsize), ::std::mem::transmute(pcbsizeout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn GetThemeAnimationTransform(htheme: isize, istoryboardid: i32, itargetid: i32, dwtransformindex: u32, ptransform: *mut TA_TRANSFORM, cbsize: u32, pcbsizeout: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeAnimationTransform(htheme: isize, istoryboardid: i32, itargetid: i32, dwtransformindex: u32, ptransform: *mut TA_TRANSFORM, cbsize: u32, pcbsizeout: *mut u32) -> ::windows::runtime::HRESULT;
        }
        GetThemeAnimationTransform(::std::mem::transmute(htheme), ::std::mem::transmute(istoryboardid), ::std::mem::transmute(itargetid), ::std::mem::transmute(dwtransformindex), ::std::mem::transmute(ptransform), ::std::mem::transmute(cbsize), ::std::mem::transmute(pcbsizeout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn GetThemeAppProperties() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeAppProperties() -> u32;
        }
        ::std::mem::transmute(GetThemeAppProperties())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetThemeBackgroundContentRect<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, pboundingrect: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeBackgroundContentRect(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, pboundingrect: *const super::super::Foundation::RECT, pcontentrect: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetThemeBackgroundContentRect(::std::mem::transmute(htheme), hdc.into_param().abi(), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), ::std::mem::transmute(pboundingrect), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetThemeBackgroundExtent<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, pcontentrect: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeBackgroundExtent(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, pcontentrect: *const super::super::Foundation::RECT, pextentrect: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetThemeBackgroundExtent(::std::mem::transmute(htheme), hdc.into_param().abi(), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), ::std::mem::transmute(pcontentrect), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetThemeBackgroundRegion<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, prect: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<super::super::Graphics::Gdi::HRGN> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeBackgroundRegion(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, prect: *const super::super::Foundation::RECT, pregion: *mut super::super::Graphics::Gdi::HRGN) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Graphics::Gdi::HRGN as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetThemeBackgroundRegion(::std::mem::transmute(htheme), hdc.into_param().abi(), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), ::std::mem::transmute(prect), &mut result__).from_abi::<super::super::Graphics::Gdi::HRGN>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeBitmap(htheme: isize, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID, dwflags: GET_THEME_BITMAP_FLAGS) -> ::windows::runtime::Result<super::super::Graphics::Gdi::HBITMAP> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeBitmap(htheme: isize, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID, dwflags: GET_THEME_BITMAP_FLAGS, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Graphics::Gdi::HBITMAP as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetThemeBitmap(::std::mem::transmute(htheme), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), ::std::mem::transmute(ipropid), ::std::mem::transmute(dwflags), &mut result__).from_abi::<super::super::Graphics::Gdi::HBITMAP>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThemeBool(htheme: isize, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeBool(htheme: isize, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID, pfval: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetThemeBool(::std::mem::transmute(htheme), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), ::std::mem::transmute(ipropid), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn GetThemeColor(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeColor(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pcolor: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetThemeColor(::std::mem::transmute(htheme), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), ::std::mem::transmute(ipropid), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThemeDocumentationProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pszthemename: Param0, pszpropertyname: Param1, pszvaluebuff: super::super::Foundation::PWSTR, cchmaxvalchars: i32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeDocumentationProperty(pszthemename: super::super::Foundation::PWSTR, pszpropertyname: super::super::Foundation::PWSTR, pszvaluebuff: super::super::Foundation::PWSTR, cchmaxvalchars: i32) -> ::windows::runtime::HRESULT;
        }
        GetThemeDocumentationProperty(pszthemename.into_param().abi(), pszpropertyname.into_param().abi(), ::std::mem::transmute(pszvaluebuff), ::std::mem::transmute(cchmaxvalchars)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn GetThemeEnumValue(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeEnumValue(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pival: *mut i32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetThemeEnumValue(::std::mem::transmute(htheme), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), ::std::mem::transmute(ipropid), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThemeFilename(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pszthemefilename: super::super::Foundation::PWSTR, cchmaxbuffchars: i32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeFilename(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pszthemefilename: super::super::Foundation::PWSTR, cchmaxbuffchars: i32) -> ::windows::runtime::HRESULT;
        }
        GetThemeFilename(::std::mem::transmute(htheme), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), ::std::mem::transmute(ipropid), ::std::mem::transmute(pszthemefilename), ::std::mem::transmute(cchmaxbuffchars)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeFont<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, ipropid: i32) -> ::windows::runtime::Result<super::super::Graphics::Gdi::LOGFONTW> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeFont(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, ipropid: i32, pfont: *mut super::super::Graphics::Gdi::LOGFONTW) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Graphics::Gdi::LOGFONTW as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetThemeFont(::std::mem::transmute(htheme), hdc.into_param().abi(), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), ::std::mem::transmute(ipropid), &mut result__).from_abi::<super::super::Graphics::Gdi::LOGFONTW>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn GetThemeInt(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeInt(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pival: *mut i32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetThemeInt(::std::mem::transmute(htheme), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), ::std::mem::transmute(ipropid), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn GetThemeIntList(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32) -> ::windows::runtime::Result<INTLIST> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeIntList(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pintlist: *mut INTLIST) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <INTLIST as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetThemeIntList(::std::mem::transmute(htheme), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), ::std::mem::transmute(ipropid), &mut result__).from_abi::<INTLIST>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetThemeMargins<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, ipropid: i32, prc: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<MARGINS> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeMargins(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, ipropid: i32, prc: *const super::super::Foundation::RECT, pmargins: *mut MARGINS) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <MARGINS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetThemeMargins(::std::mem::transmute(htheme), hdc.into_param().abi(), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), ::std::mem::transmute(ipropid), ::std::mem::transmute(prc), &mut result__).from_abi::<MARGINS>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeMetric<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeMetric(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, ipropid: THEME_PROPERTY_SYMBOL_ID, pival: *mut i32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetThemeMetric(::std::mem::transmute(htheme), hdc.into_param().abi(), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), ::std::mem::transmute(ipropid), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetThemePartSize<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, prc: *const super::super::Foundation::RECT, esize: THEMESIZE) -> ::windows::runtime::Result<super::super::Foundation::SIZE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemePartSize(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, prc: *const super::super::Foundation::RECT, esize: THEMESIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::SIZE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetThemePartSize(::std::mem::transmute(htheme), hdc.into_param().abi(), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), ::std::mem::transmute(prc), ::std::mem::transmute(esize), &mut result__).from_abi::<super::super::Foundation::SIZE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThemePosition(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32) -> ::windows::runtime::Result<super::super::Foundation::POINT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemePosition(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, ppoint: *mut super::super::Foundation::POINT) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::POINT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetThemePosition(::std::mem::transmute(htheme), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), ::std::mem::transmute(ipropid), &mut result__).from_abi::<super::super::Foundation::POINT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn GetThemePropertyOrigin(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32) -> ::windows::runtime::Result<PROPERTYORIGIN> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemePropertyOrigin(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, porigin: *mut PROPERTYORIGIN) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <PROPERTYORIGIN as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetThemePropertyOrigin(::std::mem::transmute(htheme), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), ::std::mem::transmute(ipropid), &mut result__).from_abi::<PROPERTYORIGIN>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThemeRect(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeRect(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, prect: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetThemeRect(::std::mem::transmute(htheme), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), ::std::mem::transmute(ipropid), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThemeStream<'a, Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>>(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, ppvstream: *mut *mut ::std::ffi::c_void, pcbstream: *mut u32, hinst: Param6) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeStream(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, ppvstream: *mut *mut ::std::ffi::c_void, pcbstream: *mut u32, hinst: super::super::Foundation::HINSTANCE) -> ::windows::runtime::HRESULT;
        }
        GetThemeStream(::std::mem::transmute(htheme), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), ::std::mem::transmute(ipropid), ::std::mem::transmute(ppvstream), ::std::mem::transmute(pcbstream), hinst.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThemeString(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pszbuff: super::super::Foundation::PWSTR, cchmaxbuffchars: i32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeString(htheme: isize, ipartid: i32, istateid: i32, ipropid: i32, pszbuff: super::super::Foundation::PWSTR, cchmaxbuffchars: i32) -> ::windows::runtime::HRESULT;
        }
        GetThemeString(::std::mem::transmute(htheme), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), ::std::mem::transmute(ipropid), ::std::mem::transmute(pszbuff), ::std::mem::transmute(cchmaxbuffchars)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThemeSysBool(htheme: isize, iboolid: i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeSysBool(htheme: isize, iboolid: i32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetThemeSysBool(::std::mem::transmute(htheme), ::std::mem::transmute(iboolid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn GetThemeSysColor(htheme: isize, icolorid: i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeSysColor(htheme: isize, icolorid: i32) -> u32;
        }
        ::std::mem::transmute(GetThemeSysColor(::std::mem::transmute(htheme), ::std::mem::transmute(icolorid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeSysColorBrush(htheme: isize, icolorid: THEME_PROPERTY_SYMBOL_ID) -> super::super::Graphics::Gdi::HBRUSH {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeSysColorBrush(htheme: isize, icolorid: THEME_PROPERTY_SYMBOL_ID) -> super::super::Graphics::Gdi::HBRUSH;
        }
        ::std::mem::transmute(GetThemeSysColorBrush(::std::mem::transmute(htheme), ::std::mem::transmute(icolorid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeSysFont(htheme: isize, ifontid: THEME_PROPERTY_SYMBOL_ID) -> ::windows::runtime::Result<super::super::Graphics::Gdi::LOGFONTW> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeSysFont(htheme: isize, ifontid: THEME_PROPERTY_SYMBOL_ID, plf: *mut super::super::Graphics::Gdi::LOGFONTW) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Graphics::Gdi::LOGFONTW as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetThemeSysFont(::std::mem::transmute(htheme), ::std::mem::transmute(ifontid), &mut result__).from_abi::<super::super::Graphics::Gdi::LOGFONTW>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn GetThemeSysInt(htheme: isize, iintid: i32) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeSysInt(htheme: isize, iintid: i32, pivalue: *mut i32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetThemeSysInt(::std::mem::transmute(htheme), ::std::mem::transmute(iintid), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn GetThemeSysSize(htheme: isize, isizeid: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeSysSize(htheme: isize, isizeid: i32) -> i32;
        }
        ::std::mem::transmute(GetThemeSysSize(::std::mem::transmute(htheme), ::std::mem::transmute(isizeid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThemeSysString(htheme: isize, istringid: THEME_PROPERTY_SYMBOL_ID, pszstringbuff: super::super::Foundation::PWSTR, cchmaxstringchars: i32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeSysString(htheme: isize, istringid: THEME_PROPERTY_SYMBOL_ID, pszstringbuff: super::super::Foundation::PWSTR, cchmaxstringchars: i32) -> ::windows::runtime::HRESULT;
        }
        GetThemeSysString(::std::mem::transmute(htheme), ::std::mem::transmute(istringid), ::std::mem::transmute(pszstringbuff), ::std::mem::transmute(cchmaxstringchars)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetThemeTextExtent<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, psztext: Param4, cchcharcount: i32, dwtextflags: u32, pboundingrect: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeTextExtent(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, psztext: super::super::Foundation::PWSTR, cchcharcount: i32, dwtextflags: u32, pboundingrect: *const super::super::Foundation::RECT, pextentrect: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetThemeTextExtent(::std::mem::transmute(htheme), hdc.into_param().abi(), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), psztext.into_param().abi(), ::std::mem::transmute(cchcharcount), ::std::mem::transmute(dwtextflags), ::std::mem::transmute(pboundingrect), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetThemeTextMetrics<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32) -> ::windows::runtime::Result<super::super::Graphics::Gdi::TEXTMETRICW> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeTextMetrics(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, ptm: *mut super::super::Graphics::Gdi::TEXTMETRICW) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Graphics::Gdi::TEXTMETRICW as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetThemeTextMetrics(::std::mem::transmute(htheme), hdc.into_param().abi(), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), &mut result__).from_abi::<super::super::Graphics::Gdi::TEXTMETRICW>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn GetThemeTimingFunction(htheme: isize, itimingfunctionid: i32, ptimingfunction: *mut TA_TIMINGFUNCTION, cbsize: u32, pcbsizeout: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeTimingFunction(htheme: isize, itimingfunctionid: i32, ptimingfunction: *mut TA_TIMINGFUNCTION, cbsize: u32, pcbsizeout: *mut u32) -> ::windows::runtime::HRESULT;
        }
        GetThemeTimingFunction(::std::mem::transmute(htheme), ::std::mem::transmute(itimingfunctionid), ::std::mem::transmute(ptimingfunction), ::std::mem::transmute(cbsize), ::std::mem::transmute(pcbsizeout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn GetThemeTransitionDuration(htheme: isize, ipartid: i32, istateidfrom: i32, istateidto: i32, ipropid: i32) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThemeTransitionDuration(htheme: isize, ipartid: i32, istateidfrom: i32, istateidto: i32, ipropid: i32, pdwduration: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetThemeTransitionDuration(::std::mem::transmute(htheme), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateidfrom), ::std::mem::transmute(istateidto), ::std::mem::transmute(ipropid), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowFeedbackSetting<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, feedback: FEEDBACK_TYPE, dwflags: u32, psize: *mut u32, config: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetWindowFeedbackSetting(hwnd: super::super::Foundation::HWND, feedback: FEEDBACK_TYPE, dwflags: u32, psize: *mut u32, config: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetWindowFeedbackSetting(hwnd.into_param().abi(), ::std::mem::transmute(feedback), ::std::mem::transmute(dwflags), ::std::mem::transmute(psize), ::std::mem::transmute(config)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetWindowTheme<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetWindowTheme(hwnd: super::super::Foundation::HWND) -> isize;
        }
        ::std::mem::transmute(GetWindowTheme(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HALIGN(pub i32);
pub const HA_LEFT: HALIGN = HALIGN(0i32);
pub const HA_CENTER: HALIGN = HALIGN(1i32);
pub const HA_RIGHT: HALIGN = HALIGN(2i32);
impl ::std::convert::From<i32> for HALIGN {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HALIGN {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDFT_HASNOVALUE: u32 = 32768u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDFT_ISDATE: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDFT_ISNUMBER: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDFT_ISSTRING: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDF_BITMAP: u32 = 8192u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDF_BITMAP_ON_RIGHT: u32 = 4096u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDF_CENTER: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDF_CHECKBOX: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDF_CHECKED: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDF_FIXEDWIDTH: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDF_IMAGE: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDF_JUSTIFYMASK: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDF_LEFT: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDF_OWNERDRAW: u32 = 32768u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDF_RIGHT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDF_RTLREADING: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDF_SORTDOWN: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDF_SORTUP: u32 = 1024u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDF_SPLITBUTTON: u32 = 16777216u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDF_STRING: u32 = 16384u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct HDHITTESTINFO {
    pub pt: super::super::Foundation::POINT,
    pub flags: u32,
    pub iItem: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl HDHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HDHITTESTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HDHITTESTINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HDHITTESTINFO").field("pt", &self.pt).field("flags", &self.flags).field("iItem", &self.iItem).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HDHITTESTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.pt == other.pt && self.flags == other.flags && self.iItem == other.iItem
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HDHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HDHITTESTINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDIS_FOCUSED: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct HDITEMA {
    pub mask: HDI_MASK,
    pub cxy: i32,
    pub pszText: super::super::Foundation::PSTR,
    pub hbm: super::super::Graphics::Gdi::HBITMAP,
    pub cchTextMax: i32,
    pub fmt: i32,
    pub lParam: super::super::Foundation::LPARAM,
    pub iImage: i32,
    pub iOrder: i32,
    pub r#type: u32,
    pub pvFilter: *mut ::std::ffi::c_void,
    pub state: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl HDITEMA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for HDITEMA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for HDITEMA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HDITEMA")
            .field("mask", &self.mask)
            .field("cxy", &self.cxy)
            .field("pszText", &self.pszText)
            .field("hbm", &self.hbm)
            .field("cchTextMax", &self.cchTextMax)
            .field("fmt", &self.fmt)
            .field("lParam", &self.lParam)
            .field("iImage", &self.iImage)
            .field("iOrder", &self.iOrder)
            .field("r#type", &self.r#type)
            .field("pvFilter", &self.pvFilter)
            .field("state", &self.state)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for HDITEMA {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.cxy == other.cxy && self.pszText == other.pszText && self.hbm == other.hbm && self.cchTextMax == other.cchTextMax && self.fmt == other.fmt && self.lParam == other.lParam && self.iImage == other.iImage && self.iOrder == other.iOrder && self.r#type == other.r#type && self.pvFilter == other.pvFilter && self.state == other.state
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for HDITEMA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for HDITEMA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct HDITEMW {
    pub mask: HDI_MASK,
    pub cxy: i32,
    pub pszText: super::super::Foundation::PWSTR,
    pub hbm: super::super::Graphics::Gdi::HBITMAP,
    pub cchTextMax: i32,
    pub fmt: i32,
    pub lParam: super::super::Foundation::LPARAM,
    pub iImage: i32,
    pub iOrder: i32,
    pub r#type: u32,
    pub pvFilter: *mut ::std::ffi::c_void,
    pub state: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl HDITEMW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for HDITEMW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for HDITEMW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HDITEMW")
            .field("mask", &self.mask)
            .field("cxy", &self.cxy)
            .field("pszText", &self.pszText)
            .field("hbm", &self.hbm)
            .field("cchTextMax", &self.cchTextMax)
            .field("fmt", &self.fmt)
            .field("lParam", &self.lParam)
            .field("iImage", &self.iImage)
            .field("iOrder", &self.iOrder)
            .field("r#type", &self.r#type)
            .field("pvFilter", &self.pvFilter)
            .field("state", &self.state)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for HDITEMW {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.cxy == other.cxy && self.pszText == other.pszText && self.hbm == other.hbm && self.cchTextMax == other.cchTextMax && self.fmt == other.fmt && self.lParam == other.lParam && self.iImage == other.iImage && self.iOrder == other.iOrder && self.r#type == other.r#type && self.pvFilter == other.pvFilter && self.state == other.state
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for HDITEMW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for HDITEMW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HDI_MASK(pub u32);
pub const HDI_WIDTH: HDI_MASK = HDI_MASK(1u32);
pub const HDI_HEIGHT: HDI_MASK = HDI_MASK(1u32);
pub const HDI_TEXT: HDI_MASK = HDI_MASK(2u32);
pub const HDI_FORMAT: HDI_MASK = HDI_MASK(4u32);
pub const HDI_LPARAM: HDI_MASK = HDI_MASK(8u32);
pub const HDI_BITMAP: HDI_MASK = HDI_MASK(16u32);
pub const HDI_IMAGE: HDI_MASK = HDI_MASK(32u32);
pub const HDI_DI_SETITEM: HDI_MASK = HDI_MASK(64u32);
pub const HDI_ORDER: HDI_MASK = HDI_MASK(128u32);
pub const HDI_FILTER: HDI_MASK = HDI_MASK(256u32);
pub const HDI_STATE: HDI_MASK = HDI_MASK(512u32);
impl ::std::convert::From<u32> for HDI_MASK {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HDI_MASK {
    type Abi = Self;
}
impl ::std::ops::BitOr for HDI_MASK {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for HDI_MASK {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for HDI_MASK {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for HDI_MASK {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for HDI_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
pub struct HDLAYOUT {
    pub prc: *mut super::super::Foundation::RECT,
    pub pwpos: *mut super::WindowsAndMessaging::WINDOWPOS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl HDLAYOUT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for HDLAYOUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::fmt::Debug for HDLAYOUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HDLAYOUT").field("prc", &self.prc).field("pwpos", &self.pwpos).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for HDLAYOUT {
    fn eq(&self, other: &Self) -> bool {
        self.prc == other.prc && self.pwpos == other.pwpos
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for HDLAYOUT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for HDLAYOUT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_CLEARFILTER: u32 = 4632u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_CREATEDRAGIMAGE: u32 = 4624u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_DELETEITEM: u32 = 4610u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_EDITFILTER: u32 = 4631u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_FIRST: u32 = 4608u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_GETBITMAPMARGIN: u32 = 4629u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_GETFOCUSEDITEM: u32 = 4635u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_GETIMAGELIST: u32 = 4617u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_GETITEM: u32 = 4619u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_GETITEMA: u32 = 4611u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_GETITEMCOUNT: u32 = 4608u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_GETITEMDROPDOWNRECT: u32 = 4633u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_GETITEMRECT: u32 = 4615u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_GETITEMW: u32 = 4619u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_GETORDERARRAY: u32 = 4625u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_GETOVERFLOWRECT: u32 = 4634u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_GETUNICODEFORMAT: u32 = 8198u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_HITTEST: u32 = 4614u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_INSERTITEM: u32 = 4618u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_INSERTITEMA: u32 = 4609u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_INSERTITEMW: u32 = 4618u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_LAYOUT: u32 = 4613u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_ORDERTOINDEX: u32 = 4623u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_SETBITMAPMARGIN: u32 = 4628u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_SETFILTERCHANGETIMEOUT: u32 = 4630u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_SETFOCUSEDITEM: u32 = 4636u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_SETHOTDIVIDER: u32 = 4627u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_SETIMAGELIST: u32 = 4616u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_SETITEM: u32 = 4620u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_SETITEMA: u32 = 4612u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_SETITEMW: u32 = 4620u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_SETORDERARRAY: u32 = 4626u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDM_SETUNICODEFORMAT: u32 = 8197u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDSIL_NORMAL: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDSIL_STATE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDS_BUTTONS: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDS_CHECKBOXES: u32 = 1024u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDS_DRAGDROP: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDS_FILTERBAR: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDS_FLAT: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDS_FULLDRAG: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDS_HIDDEN: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDS_HORZ: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDS_HOTTRACK: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDS_NOSIZING: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HDS_OVERFLOW: u32 = 4096u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct HD_TEXTFILTERA {
    pub pszText: super::super::Foundation::PSTR,
    pub cchTextMax: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl HD_TEXTFILTERA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HD_TEXTFILTERA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HD_TEXTFILTERA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HD_TEXTFILTERA").field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HD_TEXTFILTERA {
    fn eq(&self, other: &Self) -> bool {
        self.pszText == other.pszText && self.cchTextMax == other.cchTextMax
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HD_TEXTFILTERA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HD_TEXTFILTERA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct HD_TEXTFILTERW {
    pub pszText: super::super::Foundation::PWSTR,
    pub cchTextMax: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl HD_TEXTFILTERW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HD_TEXTFILTERW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HD_TEXTFILTERW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HD_TEXTFILTERW").field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HD_TEXTFILTERW {
    fn eq(&self, other: &Self) -> bool {
        self.pszText == other.pszText && self.cchTextMax == other.cchTextMax
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HD_TEXTFILTERW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HD_TEXTFILTERW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HEADER_CONTROL_NOTIFICATION_BUTTON(pub u32);
pub const HEADER_CONTROL_NOTIFICATION_BUTTON_LEFT: HEADER_CONTROL_NOTIFICATION_BUTTON = HEADER_CONTROL_NOTIFICATION_BUTTON(0u32);
pub const HEADER_CONTROL_NOTIFICATION_BUTTON_RIGHT: HEADER_CONTROL_NOTIFICATION_BUTTON = HEADER_CONTROL_NOTIFICATION_BUTTON(1u32);
pub const HEADER_CONTROL_NOTIFICATION_BUTTON_MIDDLE: HEADER_CONTROL_NOTIFICATION_BUTTON = HEADER_CONTROL_NOTIFICATION_BUTTON(2u32);
impl ::std::convert::From<u32> for HEADER_CONTROL_NOTIFICATION_BUTTON {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HEADER_CONTROL_NOTIFICATION_BUTTON {
    type Abi = Self;
}
impl ::std::ops::BitOr for HEADER_CONTROL_NOTIFICATION_BUTTON {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for HEADER_CONTROL_NOTIFICATION_BUTTON {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for HEADER_CONTROL_NOTIFICATION_BUTTON {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for HEADER_CONTROL_NOTIFICATION_BUTTON {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for HEADER_CONTROL_NOTIFICATION_BUTTON {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HHT_ABOVE: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HHT_BELOW: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HHT_NOWHERE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HHT_ONDIVIDER: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HHT_ONDIVOPEN: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HHT_ONDROPDOWN: u32 = 8192u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HHT_ONFILTER: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HHT_ONFILTERBUTTON: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HHT_ONHEADER: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HHT_ONITEMSTATEICON: u32 = 4096u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HHT_ONOVERFLOW: u32 = 16384u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HHT_TOLEFT: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HHT_TORIGHT: u32 = 1024u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HIMAGELIST(pub isize);
impl ::std::default::Default for HIMAGELIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HIMAGELIST {}
unsafe impl ::windows::runtime::Abi for HIMAGELIST {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn HIMAGELIST_QueryInterface<'a, Param0: ::windows::runtime::IntoParam<'a, HIMAGELIST>>(himl: Param0, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HIMAGELIST_QueryInterface(himl: HIMAGELIST, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        HIMAGELIST_QueryInterface(himl.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HIST_ADDTOFAVORITES: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HIST_BACK: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HIST_FAVORITES: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HIST_FORWARD: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HIST_VIEWTREE: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HKCOMB_A: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HKCOMB_C: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HKCOMB_CA: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HKCOMB_NONE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HKCOMB_S: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HKCOMB_SA: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HKCOMB_SC: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HKCOMB_SCA: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HKM_GETHOTKEY: u32 = 1026u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HKM_SETHOTKEY: u32 = 1025u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HKM_SETRULES: u32 = 1027u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HOTKEYF_ALT: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HOTKEYF_CONTROL: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HOTKEYF_EXT: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HOTKEYF_SHIFT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HOVER_DEFAULT: u32 = 4294967295u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HPROPSHEETPAGE(pub isize);
impl ::std::default::Default for HPROPSHEETPAGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HPROPSHEETPAGE {}
unsafe impl ::windows::runtime::Abi for HPROPSHEETPAGE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HSYNTHETICPOINTERDEVICE(pub isize);
impl ::std::default::Default for HSYNTHETICPOINTERDEVICE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HSYNTHETICPOINTERDEVICE {}
unsafe impl ::windows::runtime::Abi for HSYNTHETICPOINTERDEVICE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HTREEITEM(pub isize);
impl ::std::default::Default for HTREEITEM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HTREEITEM {}
unsafe impl ::windows::runtime::Abi for HTREEITEM {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HTTB_BACKGROUNDSEG: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HTTB_CAPTION: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HTTB_FIXEDBORDER: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HTTB_RESIZINGBORDER_BOTTOM: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HTTB_RESIZINGBORDER_LEFT: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HTTB_RESIZINGBORDER_RIGHT: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HTTB_RESIZINGBORDER_TOP: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HTTB_SIZINGTEMPLATE: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const HTTB_SYSTEMSIZINGMARGINS: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HYPERLINKSTATES(pub i32);
pub const HLS_NORMALTEXT: HYPERLINKSTATES = HYPERLINKSTATES(1i32);
pub const HLS_LINKTEXT: HYPERLINKSTATES = HYPERLINKSTATES(2i32);
impl ::std::convert::From<i32> for HYPERLINKSTATES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HYPERLINKSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn HitTestThemeBackground<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>, Param6: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HRGN>, Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::POINT>>(htheme: isize, hdc: Param1, ipartid: i32, istateid: i32, dwoptions: u32, prect: *const super::super::Foundation::RECT, hrgn: Param6, pttest: Param7) -> ::windows::runtime::Result<u16> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HitTestThemeBackground(htheme: isize, hdc: super::super::Graphics::Gdi::HDC, ipartid: i32, istateid: i32, dwoptions: u32, prect: *const super::super::Foundation::RECT, hrgn: super::super::Graphics::Gdi::HRGN, pttest: super::super::Foundation::POINT, pwhittestcode: *mut u16) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        HitTestThemeBackground(::std::mem::transmute(htheme), hdc.into_param().abi(), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid), ::std::mem::transmute(dwoptions), ::std::mem::transmute(prect), hrgn.into_param().abi(), pttest.into_param().abi(), &mut result__).from_abi::<u16>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ICONEFFECT(pub i32);
pub const ICE_NONE: ICONEFFECT = ICONEFFECT(0i32);
pub const ICE_GLOW: ICONEFFECT = ICONEFFECT(1i32);
pub const ICE_SHADOW: ICONEFFECT = ICONEFFECT(2i32);
pub const ICE_PULSE: ICONEFFECT = ICONEFFECT(3i32);
pub const ICE_ALPHA: ICONEFFECT = ICONEFFECT(4i32);
impl ::std::convert::From<i32> for ICONEFFECT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ICONEFFECT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const IDB_HIST_DISABLED: u32 = 14u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const IDB_HIST_HOT: u32 = 13u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const IDB_HIST_LARGE_COLOR: u32 = 9u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const IDB_HIST_NORMAL: u32 = 12u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const IDB_HIST_PRESSED: u32 = 15u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const IDB_HIST_SMALL_COLOR: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const IDB_STD_LARGE_COLOR: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const IDB_STD_SMALL_COLOR: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const IDB_VIEW_LARGE_COLOR: u32 = 5u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const IDB_VIEW_SMALL_COLOR: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const IDC_MANAGE_LINK: u32 = 1592u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ID_PSRESTARTWINDOWS: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IImageList(pub ::windows::runtime::IUnknown);
impl IImageList {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>>(&self, hbmimage: Param0, hbmmask: Param1) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), hbmimage.into_param().abi(), hbmmask.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn ReplaceIcon<'a, Param1: ::windows::runtime::IntoParam<'a, super::WindowsAndMessaging::HICON>>(&self, i: i32, hicon: Param1) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(i), hicon.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn SetOverlayImage(&self, iimage: i32, ioverlay: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(iimage), ::std::mem::transmute(ioverlay)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn Replace<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>, Param2: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>>(&self, i: i32, hbmimage: Param1, hbmmask: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(i), hbmimage.into_param().abi(), hbmmask.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn AddMasked<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>>(&self, hbmimage: Param0, crmask: u32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), hbmimage.into_param().abi(), ::std::mem::transmute(crmask), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn Draw(&self, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pimldp)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn Remove(&self, i: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(i)).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn GetIcon(&self, i: i32, flags: u32) -> ::windows::runtime::Result<super::WindowsAndMessaging::HICON> {
        let mut result__: <super::WindowsAndMessaging::HICON as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(i), ::std::mem::transmute(flags), &mut result__).from_abi::<super::WindowsAndMessaging::HICON>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn GetImageInfo(&self, i: i32) -> ::windows::runtime::Result<IMAGEINFO> {
        let mut result__: <IMAGEINFO as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(i), &mut result__).from_abi::<IMAGEINFO>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn Copy<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, idst: i32, punksrc: Param1, isrc: i32, uflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(idst), punksrc.into_param().abi(), ::std::mem::transmute(isrc), ::std::mem::transmute(uflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn Merge<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, i1: i32, punk2: Param1, i2: i32, dx: i32, dy: i32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(i1), punk2.into_param().abi(), ::std::mem::transmute(i2), ::std::mem::transmute(dx), ::std::mem::transmute(dy), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn Clone(&self, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    pub unsafe fn GetImageRect(&self, i: i32) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(i), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn GetIconSize(&self, cx: *mut i32, cy: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(cx), ::std::mem::transmute(cy)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn SetIconSize(&self, cx: i32, cy: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(cx), ::std::mem::transmute(cy)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn GetImageCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn SetImageCount(&self, unewcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(unewcount)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn SetBkColor(&self, clrbk: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(clrbk), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn GetBkColor(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn BeginDrag(&self, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(itrack), ::std::mem::transmute(dxhotspot), ::std::mem::transmute(dyhotspot)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn EndDrag(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    pub unsafe fn DragEnter<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndlock: Param0, x: i32, y: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), hwndlock.into_param().abi(), ::std::mem::transmute(x), ::std::mem::transmute(y)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    pub unsafe fn DragLeave<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndlock: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), hwndlock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn DragMove(&self, x: i32, y: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(x), ::std::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn SetDragCursorImage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punk: Param0, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), punk.into_param().abi(), ::std::mem::transmute(idrag), ::std::mem::transmute(dxhotspot), ::std::mem::transmute(dyhotspot)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    pub unsafe fn DragShowNolock<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fshow: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), fshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    pub unsafe fn GetDragImage(&self, ppt: *mut super::super::Foundation::POINT, ppthotspot: *mut super::super::Foundation::POINT, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppt), ::std::mem::transmute(ppthotspot), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn GetItemFlags(&self, i: i32) -> ::windows::runtime::Result<IMAGE_LIST_ITEM_FLAGS> {
        let mut result__: <IMAGE_LIST_ITEM_FLAGS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), ::std::mem::transmute(i), &mut result__).from_abi::<IMAGE_LIST_ITEM_FLAGS>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn GetOverlayImage(&self, ioverlay: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), ::std::mem::transmute(ioverlay), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IImageList {
    type Vtable = IImageList_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1189828902, 22574, 16407, [159, 223, 232, 153, 141, 170, 9, 80]);
}
impl ::std::convert::From<IImageList> for ::windows::runtime::IUnknown {
    fn from(value: IImageList) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IImageList> for ::windows::runtime::IUnknown {
    fn from(value: &IImageList) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IImageList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IImageList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageList_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP, pi: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, i: i32, hicon: super::WindowsAndMessaging::HICON, pi: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iimage: i32, ioverlay: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hbmimage: super::super::Graphics::Gdi::HBITMAP, crmask: u32, pi: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, i: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, i: i32, flags: u32, picon: *mut super::WindowsAndMessaging::HICON) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, i: i32, pimageinfo: *mut IMAGEINFO) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, idst: i32, punksrc: ::windows::runtime::RawPtr, isrc: i32, uflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, i1: i32, punk2: ::windows::runtime::RawPtr, i2: i32, dx: i32, dy: i32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, i: i32, prc: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cx: *mut i32, cy: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cx: i32, cy: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pi: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unewcount: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clrbk: u32, pclr: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclr: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndlock: super::super::Foundation::HWND, x: i32, y: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndlock: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: i32, y: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punk: ::windows::runtime::RawPtr, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fshow: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppt: *mut super::super::Foundation::POINT, ppthotspot: *mut super::super::Foundation::POINT, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, i: i32, dwflags: *mut IMAGE_LIST_ITEM_FLAGS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ioverlay: i32, piindex: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IImageList2(pub ::windows::runtime::IUnknown);
impl IImageList2 {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>>(&self, hbmimage: Param0, hbmmask: Param1) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), hbmimage.into_param().abi(), hbmmask.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn ReplaceIcon<'a, Param1: ::windows::runtime::IntoParam<'a, super::WindowsAndMessaging::HICON>>(&self, i: i32, hicon: Param1) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(i), hicon.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn SetOverlayImage(&self, iimage: i32, ioverlay: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(iimage), ::std::mem::transmute(ioverlay)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn Replace<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>, Param2: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>>(&self, i: i32, hbmimage: Param1, hbmmask: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(i), hbmimage.into_param().abi(), hbmmask.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn AddMasked<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>>(&self, hbmimage: Param0, crmask: u32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), hbmimage.into_param().abi(), ::std::mem::transmute(crmask), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn Draw(&self, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pimldp)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn Remove(&self, i: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(i)).ok()
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn GetIcon(&self, i: i32, flags: u32) -> ::windows::runtime::Result<super::WindowsAndMessaging::HICON> {
        let mut result__: <super::WindowsAndMessaging::HICON as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(i), ::std::mem::transmute(flags), &mut result__).from_abi::<super::WindowsAndMessaging::HICON>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn GetImageInfo(&self, i: i32) -> ::windows::runtime::Result<IMAGEINFO> {
        let mut result__: <IMAGEINFO as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(i), &mut result__).from_abi::<IMAGEINFO>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn Copy<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, idst: i32, punksrc: Param1, isrc: i32, uflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(idst), punksrc.into_param().abi(), ::std::mem::transmute(isrc), ::std::mem::transmute(uflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn Merge<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, i1: i32, punk2: Param1, i2: i32, dx: i32, dy: i32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(i1), punk2.into_param().abi(), ::std::mem::transmute(i2), ::std::mem::transmute(dx), ::std::mem::transmute(dy), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn Clone(&self, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    pub unsafe fn GetImageRect(&self, i: i32) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(i), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn GetIconSize(&self, cx: *mut i32, cy: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(cx), ::std::mem::transmute(cy)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn SetIconSize(&self, cx: i32, cy: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(cx), ::std::mem::transmute(cy)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn GetImageCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn SetImageCount(&self, unewcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(unewcount)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn SetBkColor(&self, clrbk: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(clrbk), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn GetBkColor(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn BeginDrag(&self, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(itrack), ::std::mem::transmute(dxhotspot), ::std::mem::transmute(dyhotspot)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn EndDrag(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    pub unsafe fn DragEnter<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndlock: Param0, x: i32, y: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), hwndlock.into_param().abi(), ::std::mem::transmute(x), ::std::mem::transmute(y)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    pub unsafe fn DragLeave<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndlock: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), hwndlock.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn DragMove(&self, x: i32, y: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(x), ::std::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn SetDragCursorImage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punk: Param0, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), punk.into_param().abi(), ::std::mem::transmute(idrag), ::std::mem::transmute(dxhotspot), ::std::mem::transmute(dyhotspot)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    pub unsafe fn DragShowNolock<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fshow: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), fshow.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    pub unsafe fn GetDragImage(&self, ppt: *mut super::super::Foundation::POINT, ppthotspot: *mut super::super::Foundation::POINT, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppt), ::std::mem::transmute(ppthotspot), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn GetItemFlags(&self, i: i32) -> ::windows::runtime::Result<IMAGE_LIST_ITEM_FLAGS> {
        let mut result__: <IMAGE_LIST_ITEM_FLAGS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), ::std::mem::transmute(i), &mut result__).from_abi::<IMAGE_LIST_ITEM_FLAGS>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn GetOverlayImage(&self, ioverlay: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), ::std::mem::transmute(ioverlay), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn Resize(&self, cxnewiconsize: i32, cynewiconsize: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(cxnewiconsize), ::std::mem::transmute(cynewiconsize)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn GetOriginalSize(&self, iimage: i32, dwflags: u32, pcx: *mut i32, pcy: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(iimage), ::std::mem::transmute(dwflags), ::std::mem::transmute(pcx), ::std::mem::transmute(pcy)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn SetOriginalSize(&self, iimage: i32, cx: i32, cy: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), ::std::mem::transmute(iimage), ::std::mem::transmute(cx), ::std::mem::transmute(cy)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn SetCallback<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punk: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn GetCallback(&self, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn ForceImagePresent(&self, iimage: i32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), ::std::mem::transmute(iimage), ::std::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn DiscardImages(&self, ifirstimage: i32, ilastimage: i32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), ::std::mem::transmute(ifirstimage), ::std::mem::transmute(ilastimage), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn PreloadImages(&self, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), ::std::mem::transmute(pimldp)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn GetStatistics(&self, pils: *mut IMAGELISTSTATS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self), ::std::mem::transmute(pils)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn Initialize(&self, cx: i32, cy: i32, flags: IMAGELIST_CREATION_FLAGS, cinitial: i32, cgrow: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::std::mem::transmute_copy(self), ::std::mem::transmute(cx), ::std::mem::transmute(cy), ::std::mem::transmute(flags), ::std::mem::transmute(cinitial), ::std::mem::transmute(cgrow)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn Replace2<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>, Param2: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, i: i32, hbmimage: Param1, hbmmask: Param2, punk: Param3, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(::std::mem::transmute_copy(self), ::std::mem::transmute(i), hbmimage.into_param().abi(), hbmmask.into_param().abi(), punk.into_param().abi(), ::std::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub unsafe fn ReplaceFromImageList<'a, Param1: ::windows::runtime::IntoParam<'a, IImageList>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, i: i32, pil: Param1, isrc: i32, punk: Param3, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::std::mem::transmute_copy(self), ::std::mem::transmute(i), pil.into_param().abi(), ::std::mem::transmute(isrc), punk.into_param().abi(), ::std::mem::transmute(dwflags)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IImageList2 {
    type Vtable = IImageList2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(422288771, 20732, 17787, [144, 160, 43, 130, 168, 181, 218, 225]);
}
impl ::std::convert::From<IImageList2> for ::windows::runtime::IUnknown {
    fn from(value: IImageList2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IImageList2> for ::windows::runtime::IUnknown {
    fn from(value: &IImageList2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IImageList2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IImageList2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IImageList2> for IImageList {
    fn from(value: IImageList2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IImageList2> for IImageList {
    fn from(value: &IImageList2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IImageList> for IImageList2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IImageList> {
        ::windows::runtime::Param::Owned(unsafe { ::std::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IImageList> for &IImageList2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IImageList> {
        ::windows::runtime::Param::Borrowed(unsafe { ::std::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageList2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP, pi: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, i: i32, hicon: super::WindowsAndMessaging::HICON, pi: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iimage: i32, ioverlay: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hbmimage: super::super::Graphics::Gdi::HBITMAP, crmask: u32, pi: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, i: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, i: i32, flags: u32, picon: *mut super::WindowsAndMessaging::HICON) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, i: i32, pimageinfo: *mut IMAGEINFO) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, idst: i32, punksrc: ::windows::runtime::RawPtr, isrc: i32, uflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, i1: i32, punk2: ::windows::runtime::RawPtr, i2: i32, dx: i32, dy: i32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, i: i32, prc: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cx: *mut i32, cy: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cx: i32, cy: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pi: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unewcount: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clrbk: u32, pclr: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclr: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndlock: super::super::Foundation::HWND, x: i32, y: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndlock: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: i32, y: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punk: ::windows::runtime::RawPtr, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fshow: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppt: *mut super::super::Foundation::POINT, ppthotspot: *mut super::super::Foundation::POINT, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, i: i32, dwflags: *mut IMAGE_LIST_ITEM_FLAGS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ioverlay: i32, piindex: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cxnewiconsize: i32, cynewiconsize: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iimage: i32, dwflags: u32, pcx: *mut i32, pcy: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iimage: i32, cx: i32, cy: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iimage: i32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ifirstimage: i32, ilastimage: i32, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pils: *mut IMAGELISTSTATS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cx: i32, cy: i32, flags: IMAGELIST_CREATION_FLAGS, cinitial: i32, cgrow: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP, punk: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, i: i32, pil: ::windows::runtime::RawPtr, isrc: i32, punk: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILDI_PURGE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILDI_QUERYACCESS: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILDI_RESETACCESS: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILDI_STANDBY: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILDRF_IMAGELOWQUALITY: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILDRF_OVERLAYLOWQUALITY: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILD_ASYNC: u32 = 32768u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILD_BLEND25: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILD_DPISCALE: u32 = 16384u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILD_IMAGE: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILD_OVERLAYMASK: u32 = 3840u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILD_PRESERVEALPHA: u32 = 4096u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILD_ROP: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILD_SCALE: u32 = 8192u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILD_TRANSPARENT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILFIP_ALWAYS: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILFIP_FROMSTANDBY: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILGOS_ALWAYS: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILGOS_FROMSTANDBY: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILGT_ASYNC: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILGT_NORMAL: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILP_DOWNLEVEL: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILP_NORMAL: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILR_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILR_HORIZONTAL_CENTER: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILR_HORIZONTAL_LEFT: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILR_HORIZONTAL_RIGHT: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILR_SCALE_ASPECTRATIO: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILR_SCALE_CLIP: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILR_VERTICAL_BOTTOM: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILR_VERTICAL_CENTER: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILR_VERTICAL_TOP: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILS_ALPHA: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILS_GLOW: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILS_NORMAL: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILS_SATURATE: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ILS_SHADOW: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct IMAGEINFO {
    pub hbmImage: super::super::Graphics::Gdi::HBITMAP,
    pub hbmMask: super::super::Graphics::Gdi::HBITMAP,
    pub Unused1: i32,
    pub Unused2: i32,
    pub rcImage: super::super::Foundation::RECT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl IMAGEINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for IMAGEINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for IMAGEINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IMAGEINFO").field("hbmImage", &self.hbmImage).field("hbmMask", &self.hbmMask).field("Unused1", &self.Unused1).field("Unused2", &self.Unused2).field("rcImage", &self.rcImage).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for IMAGEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.hbmImage == other.hbmImage && self.hbmMask == other.hbmMask && self.Unused1 == other.Unused1 && self.Unused2 == other.Unused2 && self.rcImage == other.rcImage
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for IMAGEINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for IMAGEINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMAGELAYOUT(pub i32);
pub const IL_VERTICAL: IMAGELAYOUT = IMAGELAYOUT(0i32);
pub const IL_HORIZONTAL: IMAGELAYOUT = IMAGELAYOUT(1i32);
impl ::std::convert::From<i32> for IMAGELAYOUT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMAGELAYOUT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
pub struct IMAGELISTDRAWPARAMS {
    pub cbSize: u32,
    pub himl: HIMAGELIST,
    pub i: i32,
    pub hdcDst: super::super::Graphics::Gdi::HDC,
    pub x: i32,
    pub y: i32,
    pub cx: i32,
    pub cy: i32,
    pub xBitmap: i32,
    pub yBitmap: i32,
    pub rgbBk: u32,
    pub rgbFg: u32,
    pub fStyle: u32,
    pub dwRop: u32,
    pub fState: u32,
    pub Frame: u32,
    pub crEffect: u32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl IMAGELISTDRAWPARAMS {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::default::Default for IMAGELISTDRAWPARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::fmt::Debug for IMAGELISTDRAWPARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IMAGELISTDRAWPARAMS")
            .field("cbSize", &self.cbSize)
            .field("himl", &self.himl)
            .field("i", &self.i)
            .field("hdcDst", &self.hdcDst)
            .field("x", &self.x)
            .field("y", &self.y)
            .field("cx", &self.cx)
            .field("cy", &self.cy)
            .field("xBitmap", &self.xBitmap)
            .field("yBitmap", &self.yBitmap)
            .field("rgbBk", &self.rgbBk)
            .field("rgbFg", &self.rgbFg)
            .field("fStyle", &self.fStyle)
            .field("dwRop", &self.dwRop)
            .field("fState", &self.fState)
            .field("Frame", &self.Frame)
            .field("crEffect", &self.crEffect)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::cmp::PartialEq for IMAGELISTDRAWPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.himl == other.himl && self.i == other.i && self.hdcDst == other.hdcDst && self.x == other.x && self.y == other.y && self.cx == other.cx && self.cy == other.cy && self.xBitmap == other.xBitmap && self.yBitmap == other.yBitmap && self.rgbBk == other.rgbBk && self.rgbFg == other.rgbFg && self.fStyle == other.fStyle && self.dwRop == other.dwRop && self.fState == other.fState && self.Frame == other.Frame && self.crEffect == other.crEffect
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::std::cmp::Eq for IMAGELISTDRAWPARAMS {}
#[cfg(feature = "Win32_Graphics_Gdi")]
unsafe impl ::windows::runtime::Abi for IMAGELISTDRAWPARAMS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct IMAGELISTSTATS {
    pub cbSize: u32,
    pub cAlloc: i32,
    pub cUsed: i32,
    pub cStandby: i32,
}
impl IMAGELISTSTATS {}
impl ::std::default::Default for IMAGELISTSTATS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IMAGELISTSTATS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IMAGELISTSTATS").field("cbSize", &self.cbSize).field("cAlloc", &self.cAlloc).field("cUsed", &self.cUsed).field("cStandby", &self.cStandby).finish()
    }
}
impl ::std::cmp::PartialEq for IMAGELISTSTATS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.cAlloc == other.cAlloc && self.cUsed == other.cUsed && self.cStandby == other.cStandby
    }
}
impl ::std::cmp::Eq for IMAGELISTSTATS {}
unsafe impl ::windows::runtime::Abi for IMAGELISTSTATS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMAGELIST_CREATION_FLAGS(pub u32);
pub const ILC_MASK: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(1u32);
pub const ILC_COLOR: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(0u32);
pub const ILC_COLORDDB: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(254u32);
pub const ILC_COLOR4: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(4u32);
pub const ILC_COLOR8: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(8u32);
pub const ILC_COLOR16: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(16u32);
pub const ILC_COLOR24: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(24u32);
pub const ILC_COLOR32: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(32u32);
pub const ILC_PALETTE: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(2048u32);
pub const ILC_MIRROR: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(8192u32);
pub const ILC_PERITEMMIRROR: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(32768u32);
pub const ILC_ORIGINALSIZE: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(65536u32);
pub const ILC_HIGHQUALITYSCALE: IMAGELIST_CREATION_FLAGS = IMAGELIST_CREATION_FLAGS(131072u32);
impl ::std::convert::From<u32> for IMAGELIST_CREATION_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMAGELIST_CREATION_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for IMAGELIST_CREATION_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for IMAGELIST_CREATION_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for IMAGELIST_CREATION_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for IMAGELIST_CREATION_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for IMAGELIST_CREATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMAGESELECTTYPE(pub i32);
pub const IST_NONE: IMAGESELECTTYPE = IMAGESELECTTYPE(0i32);
pub const IST_SIZE: IMAGESELECTTYPE = IMAGESELECTTYPE(1i32);
pub const IST_DPI: IMAGESELECTTYPE = IMAGESELECTTYPE(2i32);
impl ::std::convert::From<i32> for IMAGESELECTTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMAGESELECTTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMAGE_LIST_COPY_FLAGS(pub u32);
pub const ILCF_MOVE: IMAGE_LIST_COPY_FLAGS = IMAGE_LIST_COPY_FLAGS(0u32);
pub const ILCF_SWAP: IMAGE_LIST_COPY_FLAGS = IMAGE_LIST_COPY_FLAGS(1u32);
impl ::std::convert::From<u32> for IMAGE_LIST_COPY_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMAGE_LIST_COPY_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for IMAGE_LIST_COPY_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for IMAGE_LIST_COPY_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for IMAGE_LIST_COPY_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for IMAGE_LIST_COPY_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for IMAGE_LIST_COPY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMAGE_LIST_DRAW_STYLE(pub u32);
pub const ILD_BLEND: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(4u32);
pub const ILD_BLEND50: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(4u32);
pub const ILD_FOCUS: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(2u32);
pub const ILD_MASK: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(16u32);
pub const ILD_NORMAL: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(0u32);
pub const ILD_SELECTED: IMAGE_LIST_DRAW_STYLE = IMAGE_LIST_DRAW_STYLE(4u32);
impl ::std::convert::From<u32> for IMAGE_LIST_DRAW_STYLE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMAGE_LIST_DRAW_STYLE {
    type Abi = Self;
}
impl ::std::ops::BitOr for IMAGE_LIST_DRAW_STYLE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for IMAGE_LIST_DRAW_STYLE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for IMAGE_LIST_DRAW_STYLE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for IMAGE_LIST_DRAW_STYLE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for IMAGE_LIST_DRAW_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct IMAGE_LIST_ITEM_FLAGS(pub u32);
pub const ILIF_ALPHA: IMAGE_LIST_ITEM_FLAGS = IMAGE_LIST_ITEM_FLAGS(1u32);
pub const ILIF_LOWQUALITY: IMAGE_LIST_ITEM_FLAGS = IMAGE_LIST_ITEM_FLAGS(2u32);
impl ::std::convert::From<u32> for IMAGE_LIST_ITEM_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IMAGE_LIST_ITEM_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for IMAGE_LIST_ITEM_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for IMAGE_LIST_ITEM_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for IMAGE_LIST_ITEM_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for IMAGE_LIST_ITEM_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for IMAGE_LIST_ITEM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const INFOTIPSIZE: u32 = 1024u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct INITCOMMONCONTROLSEX {
    pub dwSize: u32,
    pub dwICC: INITCOMMONCONTROLSEX_ICC,
}
impl INITCOMMONCONTROLSEX {}
impl ::std::default::Default for INITCOMMONCONTROLSEX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for INITCOMMONCONTROLSEX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("INITCOMMONCONTROLSEX").field("dwSize", &self.dwSize).field("dwICC", &self.dwICC).finish()
    }
}
impl ::std::cmp::PartialEq for INITCOMMONCONTROLSEX {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwICC == other.dwICC
    }
}
impl ::std::cmp::Eq for INITCOMMONCONTROLSEX {}
unsafe impl ::windows::runtime::Abi for INITCOMMONCONTROLSEX {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct INITCOMMONCONTROLSEX_ICC(pub u32);
pub const ICC_ANIMATE_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(128u32);
pub const ICC_BAR_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(4u32);
pub const ICC_COOL_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(1024u32);
pub const ICC_DATE_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(256u32);
pub const ICC_HOTKEY_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(64u32);
pub const ICC_INTERNET_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(2048u32);
pub const ICC_LINK_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(32768u32);
pub const ICC_LISTVIEW_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(1u32);
pub const ICC_NATIVEFNTCTL_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(8192u32);
pub const ICC_PAGESCROLLER_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(4096u32);
pub const ICC_PROGRESS_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(32u32);
pub const ICC_STANDARD_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(16384u32);
pub const ICC_TAB_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(8u32);
pub const ICC_TREEVIEW_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(2u32);
pub const ICC_UPDOWN_CLASS: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(16u32);
pub const ICC_USEREX_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(512u32);
pub const ICC_WIN95_CLASSES: INITCOMMONCONTROLSEX_ICC = INITCOMMONCONTROLSEX_ICC(255u32);
impl ::std::convert::From<u32> for INITCOMMONCONTROLSEX_ICC {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for INITCOMMONCONTROLSEX_ICC {
    type Abi = Self;
}
impl ::std::ops::BitOr for INITCOMMONCONTROLSEX_ICC {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for INITCOMMONCONTROLSEX_ICC {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for INITCOMMONCONTROLSEX_ICC {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for INITCOMMONCONTROLSEX_ICC {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for INITCOMMONCONTROLSEX_ICC {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct INTLIST {
    pub iValueCount: i32,
    pub iValues: [i32; 402],
}
impl INTLIST {}
impl ::std::default::Default for INTLIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for INTLIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("INTLIST").field("iValueCount", &self.iValueCount).field("iValues", &self.iValues).finish()
    }
}
impl ::std::cmp::PartialEq for INTLIST {
    fn eq(&self, other: &Self) -> bool {
        self.iValueCount == other.iValueCount && self.iValues == other.iValues
    }
}
impl ::std::cmp::Eq for INTLIST {}
unsafe impl ::windows::runtime::Abi for INTLIST {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const INVALID_LINK_INDEX: i32 = -1i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const IPM_CLEARADDRESS: u32 = 1124u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const IPM_GETADDRESS: u32 = 1126u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const IPM_ISBLANK: u32 = 1129u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const IPM_SETADDRESS: u32 = 1125u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const IPM_SETFOCUS: u32 = 1128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const IPM_SETRANGE: u32 = 1127u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const I_IMAGECALLBACK: i32 = -1i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const I_IMAGENONE: i32 = -2i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const I_INDENTCALLBACK: i32 = -1i32;
pub const ImageList: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2085055394, 689, 18676, [128, 72, 178, 70, 25, 221, 192, 88]);
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImageList_Add<'a, Param0: ::windows::runtime::IntoParam<'a, HIMAGELIST>, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>, Param2: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>>(himl: Param0, hbmimage: Param1, hbmmask: Param2) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_Add(himl: HIMAGELIST, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP) -> i32;
        }
        ::std::mem::transmute(ImageList_Add(himl.into_param().abi(), hbmimage.into_param().abi(), hbmmask.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ImageList_AddMasked<'a, Param0: ::windows::runtime::IntoParam<'a, HIMAGELIST>, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>>(himl: Param0, hbmimage: Param1, crmask: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_AddMasked(himl: HIMAGELIST, hbmimage: super::super::Graphics::Gdi::HBITMAP, crmask: u32) -> i32;
        }
        ::std::mem::transmute(ImageList_AddMasked(himl.into_param().abi(), hbmimage.into_param().abi(), ::std::mem::transmute(crmask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_BeginDrag<'a, Param0: ::windows::runtime::IntoParam<'a, HIMAGELIST>>(himltrack: Param0, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_BeginDrag(himltrack: HIMAGELIST, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ImageList_BeginDrag(himltrack.into_param().abi(), ::std::mem::transmute(itrack), ::std::mem::transmute(dxhotspot), ::std::mem::transmute(dyhotspot)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn ImageList_CoCreateInstance<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(rclsid: *const ::windows::runtime::GUID, punkouter: Param1, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_CoCreateInstance(rclsid: *const ::windows::runtime::GUID, punkouter: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        ImageList_CoCreateInstance(::std::mem::transmute(rclsid), punkouter.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_Copy<'a, Param0: ::windows::runtime::IntoParam<'a, HIMAGELIST>, Param2: ::windows::runtime::IntoParam<'a, HIMAGELIST>>(himldst: Param0, idst: i32, himlsrc: Param2, isrc: i32, uflags: IMAGE_LIST_COPY_FLAGS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_Copy(himldst: HIMAGELIST, idst: i32, himlsrc: HIMAGELIST, isrc: i32, uflags: IMAGE_LIST_COPY_FLAGS) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ImageList_Copy(himldst.into_param().abi(), ::std::mem::transmute(idst), himlsrc.into_param().abi(), ::std::mem::transmute(isrc), ::std::mem::transmute(uflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn ImageList_Create(cx: i32, cy: i32, flags: IMAGELIST_CREATION_FLAGS, cinitial: i32, cgrow: i32) -> HIMAGELIST {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_Create(cx: i32, cy: i32, flags: IMAGELIST_CREATION_FLAGS, cinitial: i32, cgrow: i32) -> HIMAGELIST;
        }
        ::std::mem::transmute(ImageList_Create(::std::mem::transmute(cx), ::std::mem::transmute(cy), ::std::mem::transmute(flags), ::std::mem::transmute(cinitial), ::std::mem::transmute(cgrow)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_Destroy<'a, Param0: ::windows::runtime::IntoParam<'a, HIMAGELIST>>(himl: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_Destroy(himl: HIMAGELIST) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ImageList_Destroy(himl.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_DragEnter<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwndlock: Param0, x: i32, y: i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_DragEnter(hwndlock: super::super::Foundation::HWND, x: i32, y: i32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ImageList_DragEnter(hwndlock.into_param().abi(), ::std::mem::transmute(x), ::std::mem::transmute(y)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_DragLeave<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwndlock: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_DragLeave(hwndlock: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ImageList_DragLeave(hwndlock.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_DragMove(x: i32, y: i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_DragMove(x: i32, y: i32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ImageList_DragMove(::std::mem::transmute(x), ::std::mem::transmute(y)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_DragShowNolock<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(fshow: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_DragShowNolock(fshow: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ImageList_DragShowNolock(fshow.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImageList_Draw<'a, Param0: ::windows::runtime::IntoParam<'a, HIMAGELIST>, Param2: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(himl: Param0, i: i32, hdcdst: Param2, x: i32, y: i32, fstyle: IMAGE_LIST_DRAW_STYLE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_Draw(himl: HIMAGELIST, i: i32, hdcdst: super::super::Graphics::Gdi::HDC, x: i32, y: i32, fstyle: IMAGE_LIST_DRAW_STYLE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ImageList_Draw(himl.into_param().abi(), ::std::mem::transmute(i), hdcdst.into_param().abi(), ::std::mem::transmute(x), ::std::mem::transmute(y), ::std::mem::transmute(fstyle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImageList_DrawEx<'a, Param0: ::windows::runtime::IntoParam<'a, HIMAGELIST>, Param2: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(himl: Param0, i: i32, hdcdst: Param2, x: i32, y: i32, dx: i32, dy: i32, rgbbk: u32, rgbfg: u32, fstyle: IMAGE_LIST_DRAW_STYLE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_DrawEx(himl: HIMAGELIST, i: i32, hdcdst: super::super::Graphics::Gdi::HDC, x: i32, y: i32, dx: i32, dy: i32, rgbbk: u32, rgbfg: u32, fstyle: IMAGE_LIST_DRAW_STYLE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ImageList_DrawEx(himl.into_param().abi(), ::std::mem::transmute(i), hdcdst.into_param().abi(), ::std::mem::transmute(x), ::std::mem::transmute(y), ::std::mem::transmute(dx), ::std::mem::transmute(dy), ::std::mem::transmute(rgbbk), ::std::mem::transmute(rgbfg), ::std::mem::transmute(fstyle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImageList_DrawIndirect(pimldp: *const IMAGELISTDRAWPARAMS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_DrawIndirect(pimldp: *const IMAGELISTDRAWPARAMS) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ImageList_DrawIndirect(::std::mem::transmute(pimldp)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn ImageList_Duplicate<'a, Param0: ::windows::runtime::IntoParam<'a, HIMAGELIST>>(himl: Param0) -> HIMAGELIST {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_Duplicate(himl: HIMAGELIST) -> HIMAGELIST;
        }
        ::std::mem::transmute(ImageList_Duplicate(himl.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn ImageList_EndDrag() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_EndDrag();
        }
        ::std::mem::transmute(ImageList_EndDrag())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn ImageList_GetBkColor<'a, Param0: ::windows::runtime::IntoParam<'a, HIMAGELIST>>(himl: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_GetBkColor(himl: HIMAGELIST) -> u32;
        }
        ::std::mem::transmute(ImageList_GetBkColor(himl.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_GetDragImage(ppt: *mut super::super::Foundation::POINT, ppthotspot: *mut super::super::Foundation::POINT) -> HIMAGELIST {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_GetDragImage(ppt: *mut super::super::Foundation::POINT, ppthotspot: *mut super::super::Foundation::POINT) -> HIMAGELIST;
        }
        ::std::mem::transmute(ImageList_GetDragImage(::std::mem::transmute(ppt), ::std::mem::transmute(ppthotspot)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn ImageList_GetIcon<'a, Param0: ::windows::runtime::IntoParam<'a, HIMAGELIST>>(himl: Param0, i: i32, flags: u32) -> super::WindowsAndMessaging::HICON {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_GetIcon(himl: HIMAGELIST, i: i32, flags: u32) -> super::WindowsAndMessaging::HICON;
        }
        ::std::mem::transmute(ImageList_GetIcon(himl.into_param().abi(), ::std::mem::transmute(i), ::std::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_GetIconSize<'a, Param0: ::windows::runtime::IntoParam<'a, HIMAGELIST>>(himl: Param0, cx: *mut i32, cy: *mut i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_GetIconSize(himl: HIMAGELIST, cx: *mut i32, cy: *mut i32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ImageList_GetIconSize(himl.into_param().abi(), ::std::mem::transmute(cx), ::std::mem::transmute(cy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn ImageList_GetImageCount<'a, Param0: ::windows::runtime::IntoParam<'a, HIMAGELIST>>(himl: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_GetImageCount(himl: HIMAGELIST) -> i32;
        }
        ::std::mem::transmute(ImageList_GetImageCount(himl.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImageList_GetImageInfo<'a, Param0: ::windows::runtime::IntoParam<'a, HIMAGELIST>>(himl: Param0, i: i32, pimageinfo: *mut IMAGEINFO) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_GetImageInfo(himl: HIMAGELIST, i: i32, pimageinfo: *mut IMAGEINFO) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ImageList_GetImageInfo(himl.into_param().abi(), ::std::mem::transmute(i), ::std::mem::transmute(pimageinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn ImageList_LoadImageA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hi: Param0, lpbmp: Param1, cx: i32, cgrow: i32, crmask: u32, utype: u32, uflags: super::WindowsAndMessaging::IMAGE_FLAGS) -> HIMAGELIST {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_LoadImageA(hi: super::super::Foundation::HINSTANCE, lpbmp: super::super::Foundation::PSTR, cx: i32, cgrow: i32, crmask: u32, utype: u32, uflags: super::WindowsAndMessaging::IMAGE_FLAGS) -> HIMAGELIST;
        }
        ::std::mem::transmute(ImageList_LoadImageA(hi.into_param().abi(), lpbmp.into_param().abi(), ::std::mem::transmute(cx), ::std::mem::transmute(cgrow), ::std::mem::transmute(crmask), ::std::mem::transmute(utype), ::std::mem::transmute(uflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn ImageList_LoadImageW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hi: Param0, lpbmp: Param1, cx: i32, cgrow: i32, crmask: u32, utype: u32, uflags: super::WindowsAndMessaging::IMAGE_FLAGS) -> HIMAGELIST {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_LoadImageW(hi: super::super::Foundation::HINSTANCE, lpbmp: super::super::Foundation::PWSTR, cx: i32, cgrow: i32, crmask: u32, utype: u32, uflags: super::WindowsAndMessaging::IMAGE_FLAGS) -> HIMAGELIST;
        }
        ::std::mem::transmute(ImageList_LoadImageW(hi.into_param().abi(), lpbmp.into_param().abi(), ::std::mem::transmute(cx), ::std::mem::transmute(cgrow), ::std::mem::transmute(crmask), ::std::mem::transmute(utype), ::std::mem::transmute(uflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn ImageList_Merge<'a, Param0: ::windows::runtime::IntoParam<'a, HIMAGELIST>, Param2: ::windows::runtime::IntoParam<'a, HIMAGELIST>>(himl1: Param0, i1: i32, himl2: Param2, i2: i32, dx: i32, dy: i32) -> HIMAGELIST {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_Merge(himl1: HIMAGELIST, i1: i32, himl2: HIMAGELIST, i2: i32, dx: i32, dy: i32) -> HIMAGELIST;
        }
        ::std::mem::transmute(ImageList_Merge(himl1.into_param().abi(), ::std::mem::transmute(i1), himl2.into_param().abi(), ::std::mem::transmute(i2), ::std::mem::transmute(dx), ::std::mem::transmute(dy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ImageList_Read<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(pstm: Param0) -> HIMAGELIST {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_Read(pstm: ::windows::runtime::RawPtr) -> HIMAGELIST;
        }
        ::std::mem::transmute(ImageList_Read(pstm.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ImageList_ReadEx<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(dwflags: u32, pstm: Param1, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_ReadEx(dwflags: u32, pstm: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        ImageList_ReadEx(::std::mem::transmute(dwflags), pstm.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_Remove<'a, Param0: ::windows::runtime::IntoParam<'a, HIMAGELIST>>(himl: Param0, i: i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_Remove(himl: HIMAGELIST, i: i32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ImageList_Remove(himl.into_param().abi(), ::std::mem::transmute(i)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ImageList_Replace<'a, Param0: ::windows::runtime::IntoParam<'a, HIMAGELIST>, Param2: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>, Param3: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HBITMAP>>(himl: Param0, i: i32, hbmimage: Param2, hbmmask: Param3) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_Replace(himl: HIMAGELIST, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ImageList_Replace(himl.into_param().abi(), ::std::mem::transmute(i), hbmimage.into_param().abi(), hbmmask.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn ImageList_ReplaceIcon<'a, Param0: ::windows::runtime::IntoParam<'a, HIMAGELIST>, Param2: ::windows::runtime::IntoParam<'a, super::WindowsAndMessaging::HICON>>(himl: Param0, i: i32, hicon: Param2) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_ReplaceIcon(himl: HIMAGELIST, i: i32, hicon: super::WindowsAndMessaging::HICON) -> i32;
        }
        ::std::mem::transmute(ImageList_ReplaceIcon(himl.into_param().abi(), ::std::mem::transmute(i), hicon.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn ImageList_SetBkColor<'a, Param0: ::windows::runtime::IntoParam<'a, HIMAGELIST>>(himl: Param0, clrbk: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_SetBkColor(himl: HIMAGELIST, clrbk: u32) -> u32;
        }
        ::std::mem::transmute(ImageList_SetBkColor(himl.into_param().abi(), ::std::mem::transmute(clrbk)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_SetDragCursorImage<'a, Param0: ::windows::runtime::IntoParam<'a, HIMAGELIST>>(himldrag: Param0, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_SetDragCursorImage(himldrag: HIMAGELIST, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ImageList_SetDragCursorImage(himldrag.into_param().abi(), ::std::mem::transmute(idrag), ::std::mem::transmute(dxhotspot), ::std::mem::transmute(dyhotspot)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_SetIconSize<'a, Param0: ::windows::runtime::IntoParam<'a, HIMAGELIST>>(himl: Param0, cx: i32, cy: i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_SetIconSize(himl: HIMAGELIST, cx: i32, cy: i32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ImageList_SetIconSize(himl.into_param().abi(), ::std::mem::transmute(cx), ::std::mem::transmute(cy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_SetImageCount<'a, Param0: ::windows::runtime::IntoParam<'a, HIMAGELIST>>(himl: Param0, unewcount: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_SetImageCount(himl: HIMAGELIST, unewcount: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ImageList_SetImageCount(himl.into_param().abi(), ::std::mem::transmute(unewcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ImageList_SetOverlayImage<'a, Param0: ::windows::runtime::IntoParam<'a, HIMAGELIST>>(himl: Param0, iimage: i32, ioverlay: i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_SetOverlayImage(himl: HIMAGELIST, iimage: i32, ioverlay: i32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ImageList_SetOverlayImage(himl.into_param().abi(), ::std::mem::transmute(iimage), ::std::mem::transmute(ioverlay)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_System_Com`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn ImageList_Write<'a, Param0: ::windows::runtime::IntoParam<'a, HIMAGELIST>, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(himl: Param0, pstm: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_Write(himl: HIMAGELIST, pstm: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ImageList_Write(himl.into_param().abi(), pstm.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn ImageList_WriteEx<'a, Param0: ::windows::runtime::IntoParam<'a, HIMAGELIST>, Param2: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(himl: Param0, dwflags: u32, pstm: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ImageList_WriteEx(himl: HIMAGELIST, dwflags: u32, pstm: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        ImageList_WriteEx(himl.into_param().abi(), ::std::mem::transmute(dwflags), pstm.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn InitCommonControls() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitCommonControls();
        }
        ::std::mem::transmute(InitCommonControls())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitCommonControlsEx(picce: *const INITCOMMONCONTROLSEX) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitCommonControlsEx(picce: *const INITCOMMONCONTROLSEX) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(InitCommonControlsEx(::std::mem::transmute(picce)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn InitMUILanguage(uilang: u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitMUILanguage(uilang: u16);
        }
        ::std::mem::transmute(InitMUILanguage(::std::mem::transmute(uilang)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitializeFlatSB<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(param0: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeFlatSB(param0: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(InitializeFlatSB(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsAppThemed() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsAppThemed() -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsAppThemed())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsCharLowerW(ch: u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsCharLowerW(ch: u16) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsCharLowerW(::std::mem::transmute(ch)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsCompositionActive() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsCompositionActive() -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsCompositionActive())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsDlgButtonChecked<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hdlg: Param0, nidbutton: i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsDlgButtonChecked(hdlg: super::super::Foundation::HWND, nidbutton: i32) -> u32;
        }
        ::std::mem::transmute(IsDlgButtonChecked(hdlg.into_param().abi(), ::std::mem::transmute(nidbutton)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsThemeActive() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsThemeActive() -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsThemeActive())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsThemeBackgroundPartiallyTransparent(htheme: isize, ipartid: i32, istateid: i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsThemeBackgroundPartiallyTransparent(htheme: isize, ipartid: i32, istateid: i32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsThemeBackgroundPartiallyTransparent(::std::mem::transmute(htheme), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsThemeDialogTextureEnabled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsThemeDialogTextureEnabled(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsThemeDialogTextureEnabled(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsThemePartDefined(htheme: isize, ipartid: i32, istateid: i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsThemePartDefined(htheme: isize, ipartid: i32, istateid: i32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsThemePartDefined(::std::mem::transmute(htheme), ::std::mem::transmute(ipartid), ::std::mem::transmute(istateid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LBItemFromPt<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::POINT>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(hlb: Param0, pt: Param1, bautoscroll: Param2) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LBItemFromPt(hlb: super::super::Foundation::HWND, pt: super::super::Foundation::POINT, bautoscroll: super::super::Foundation::BOOL) -> i32;
        }
        ::std::mem::transmute(LBItemFromPt(hlb.into_param().abi(), pt.into_param().abi(), bautoscroll.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct LHITTESTINFO {
    pub pt: super::super::Foundation::POINT,
    pub item: LITEM,
}
#[cfg(feature = "Win32_Foundation")]
impl LHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for LHITTESTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for LHITTESTINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LHITTESTINFO").field("pt", &self.pt).field("item", &self.item).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for LHITTESTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.pt == other.pt && self.item == other.item
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for LHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LHITTESTINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LIF_ITEMID: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LIF_ITEMINDEX: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LIF_STATE: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LIF_URL: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LINKPARTS(pub i32);
pub const LP_HYPERLINK: LINKPARTS = LINKPARTS(1i32);
impl ::std::convert::From<i32> for LINKPARTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LINKPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LIS_DEFAULTCOLORS: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LIS_ENABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LIS_FOCUSED: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LIS_HOTTRACK: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LIS_VISITED: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct LITEM {
    pub mask: u32,
    pub iLink: i32,
    pub state: u32,
    pub stateMask: u32,
    pub szID: [u16; 48],
    pub szUrl: [u16; 2084],
}
impl LITEM {}
impl ::std::default::Default for LITEM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for LITEM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LITEM").field("mask", &self.mask).field("iLink", &self.iLink).field("state", &self.state).field("stateMask", &self.stateMask).field("szID", &self.szID).field("szUrl", &self.szUrl).finish()
    }
}
impl ::std::cmp::PartialEq for LITEM {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.iLink == other.iLink && self.state == other.state && self.stateMask == other.stateMask && self.szID == other.szID && self.szUrl == other.szUrl
    }
}
impl ::std::cmp::Eq for LITEM {}
unsafe impl ::windows::runtime::Abi for LITEM {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LM_GETIDEALHEIGHT: u32 = 1793u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LM_GETIDEALSIZE: u32 = 1793u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LM_GETITEM: u32 = 1795u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LM_HITTEST: u32 = 1792u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LM_SETITEM: u32 = 1794u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LOGOFFBUTTONSSTATES(pub i32);
pub const SPLS_NORMAL: LOGOFFBUTTONSSTATES = LOGOFFBUTTONSSTATES(1i32);
pub const SPLS_HOT: LOGOFFBUTTONSSTATES = LOGOFFBUTTONSSTATES(2i32);
pub const SPLS_PRESSED: LOGOFFBUTTONSSTATES = LOGOFFBUTTONSSTATES(3i32);
impl ::std::convert::From<i32> for LOGOFFBUTTONSSTATES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LOGOFFBUTTONSSTATES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type LPFNADDPROPSHEETPAGES = unsafe extern "system" fn(param0: *mut ::std::ffi::c_void, param1: ::windows::runtime::RawPtr, param2: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPFNCCINFOA = unsafe extern "system" fn(acci: *mut ::std::mem::ManuallyDrop<CCINFOA>) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPFNCCINFOW = unsafe extern "system" fn(acci: *mut ::std::mem::ManuallyDrop<CCINFOW>) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPFNCCSIZETOTEXTA = unsafe extern "system" fn(flstyle: u32, flextstyle: u32, hfont: super::super::Graphics::Gdi::HFONT, psztext: super::super::Foundation::PSTR) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPFNCCSIZETOTEXTW = unsafe extern "system" fn(flstyle: u32, flextstyle: u32, hfont: super::super::Graphics::Gdi::HFONT, psztext: super::super::Foundation::PWSTR) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPFNCCSTYLEA = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, pccs: *mut CCSTYLEA) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPFNCCSTYLEW = unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, pccs: *mut CCSTYLEW) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub type LPFNPSPCALLBACKA = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: PSPCB_MESSAGE, ppsp: *mut ::std::mem::ManuallyDrop<PROPSHEETPAGEA>) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub type LPFNPSPCALLBACKW = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, umsg: PSPCB_MESSAGE, ppsp: *mut ::std::mem::ManuallyDrop<PROPSHEETPAGEW>) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type LPFNSVADDPROPSHEETPAGE = unsafe extern "system" fn(param0: HPROPSHEETPAGE, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVA_ALIGNLEFT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVA_ALIGNTOP: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVA_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVA_SNAPTOGRID: u32 = 5u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVBKIF_FLAG_ALPHABLEND: u32 = 536870912u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVBKIF_FLAG_TILEOFFSET: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVBKIF_SOURCE_HBITMAP: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVBKIF_SOURCE_MASK: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVBKIF_SOURCE_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVBKIF_SOURCE_URL: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVBKIF_STYLE_MASK: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVBKIF_STYLE_NORMAL: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVBKIF_STYLE_TILE: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVBKIF_TYPE_WATERMARK: u32 = 268435456u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct LVBKIMAGEA {
    pub ulFlags: u32,
    pub hbm: super::super::Graphics::Gdi::HBITMAP,
    pub pszImage: super::super::Foundation::PSTR,
    pub cchImageMax: u32,
    pub xOffsetPercent: i32,
    pub yOffsetPercent: i32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl LVBKIMAGEA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for LVBKIMAGEA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for LVBKIMAGEA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LVBKIMAGEA").field("ulFlags", &self.ulFlags).field("hbm", &self.hbm).field("pszImage", &self.pszImage).field("cchImageMax", &self.cchImageMax).field("xOffsetPercent", &self.xOffsetPercent).field("yOffsetPercent", &self.yOffsetPercent).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for LVBKIMAGEA {
    fn eq(&self, other: &Self) -> bool {
        self.ulFlags == other.ulFlags && self.hbm == other.hbm && self.pszImage == other.pszImage && self.cchImageMax == other.cchImageMax && self.xOffsetPercent == other.xOffsetPercent && self.yOffsetPercent == other.yOffsetPercent
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for LVBKIMAGEA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for LVBKIMAGEA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct LVBKIMAGEW {
    pub ulFlags: u32,
    pub hbm: super::super::Graphics::Gdi::HBITMAP,
    pub pszImage: super::super::Foundation::PWSTR,
    pub cchImageMax: u32,
    pub xOffsetPercent: i32,
    pub yOffsetPercent: i32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl LVBKIMAGEW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for LVBKIMAGEW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for LVBKIMAGEW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LVBKIMAGEW").field("ulFlags", &self.ulFlags).field("hbm", &self.hbm).field("pszImage", &self.pszImage).field("cchImageMax", &self.cchImageMax).field("xOffsetPercent", &self.xOffsetPercent).field("yOffsetPercent", &self.yOffsetPercent).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for LVBKIMAGEW {
    fn eq(&self, other: &Self) -> bool {
        self.ulFlags == other.ulFlags && self.hbm == other.hbm && self.pszImage == other.pszImage && self.cchImageMax == other.cchImageMax && self.xOffsetPercent == other.xOffsetPercent && self.yOffsetPercent == other.yOffsetPercent
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for LVBKIMAGEW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for LVBKIMAGEW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVCDRF_NOGROUPFRAME: u32 = 131072u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVCDRF_NOSELECT: u32 = 65536u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVCFMT_FILL: u32 = 2097152u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVCFMT_LINE_BREAK: u32 = 1048576u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVCFMT_NO_TITLE: u32 = 8388608u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVCFMT_WRAP: u32 = 4194304u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct LVCOLUMNA {
    pub mask: LVCOLUMNW_MASK,
    pub fmt: LVCOLUMNW_FORMAT,
    pub cx: i32,
    pub pszText: super::super::Foundation::PSTR,
    pub cchTextMax: i32,
    pub iSubItem: i32,
    pub iImage: i32,
    pub iOrder: i32,
    pub cxMin: i32,
    pub cxDefault: i32,
    pub cxIdeal: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl LVCOLUMNA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for LVCOLUMNA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for LVCOLUMNA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LVCOLUMNA")
            .field("mask", &self.mask)
            .field("fmt", &self.fmt)
            .field("cx", &self.cx)
            .field("pszText", &self.pszText)
            .field("cchTextMax", &self.cchTextMax)
            .field("iSubItem", &self.iSubItem)
            .field("iImage", &self.iImage)
            .field("iOrder", &self.iOrder)
            .field("cxMin", &self.cxMin)
            .field("cxDefault", &self.cxDefault)
            .field("cxIdeal", &self.cxIdeal)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for LVCOLUMNA {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.fmt == other.fmt && self.cx == other.cx && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iSubItem == other.iSubItem && self.iImage == other.iImage && self.iOrder == other.iOrder && self.cxMin == other.cxMin && self.cxDefault == other.cxDefault && self.cxIdeal == other.cxIdeal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for LVCOLUMNA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LVCOLUMNA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct LVCOLUMNW {
    pub mask: LVCOLUMNW_MASK,
    pub fmt: LVCOLUMNW_FORMAT,
    pub cx: i32,
    pub pszText: super::super::Foundation::PWSTR,
    pub cchTextMax: i32,
    pub iSubItem: i32,
    pub iImage: i32,
    pub iOrder: i32,
    pub cxMin: i32,
    pub cxDefault: i32,
    pub cxIdeal: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl LVCOLUMNW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for LVCOLUMNW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for LVCOLUMNW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LVCOLUMNW")
            .field("mask", &self.mask)
            .field("fmt", &self.fmt)
            .field("cx", &self.cx)
            .field("pszText", &self.pszText)
            .field("cchTextMax", &self.cchTextMax)
            .field("iSubItem", &self.iSubItem)
            .field("iImage", &self.iImage)
            .field("iOrder", &self.iOrder)
            .field("cxMin", &self.cxMin)
            .field("cxDefault", &self.cxDefault)
            .field("cxIdeal", &self.cxIdeal)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for LVCOLUMNW {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.fmt == other.fmt && self.cx == other.cx && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iSubItem == other.iSubItem && self.iImage == other.iImage && self.iOrder == other.iOrder && self.cxMin == other.cxMin && self.cxDefault == other.cxDefault && self.cxIdeal == other.cxIdeal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for LVCOLUMNW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LVCOLUMNW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LVCOLUMNW_FORMAT(pub u32);
pub const LVCFMT_LEFT: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(0u32);
pub const LVCFMT_RIGHT: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(1u32);
pub const LVCFMT_CENTER: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(2u32);
pub const LVCFMT_JUSTIFYMASK: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(3u32);
pub const LVCFMT_IMAGE: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(2048u32);
pub const LVCFMT_BITMAP_ON_RIGHT: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(4096u32);
pub const LVCFMT_COL_HAS_IMAGES: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(32768u32);
pub const LVCFMT_FIXED_WIDTH: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(256u32);
pub const LVCFMT_NO_DPI_SCALE: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(262144u32);
pub const LVCFMT_FIXED_RATIO: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(524288u32);
pub const LVCFMT_SPLITBUTTON: LVCOLUMNW_FORMAT = LVCOLUMNW_FORMAT(16777216u32);
impl ::std::convert::From<u32> for LVCOLUMNW_FORMAT {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LVCOLUMNW_FORMAT {
    type Abi = Self;
}
impl ::std::ops::BitOr for LVCOLUMNW_FORMAT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for LVCOLUMNW_FORMAT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for LVCOLUMNW_FORMAT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for LVCOLUMNW_FORMAT {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for LVCOLUMNW_FORMAT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LVCOLUMNW_MASK(pub u32);
pub const LVCF_FMT: LVCOLUMNW_MASK = LVCOLUMNW_MASK(1u32);
pub const LVCF_WIDTH: LVCOLUMNW_MASK = LVCOLUMNW_MASK(2u32);
pub const LVCF_TEXT: LVCOLUMNW_MASK = LVCOLUMNW_MASK(4u32);
pub const LVCF_SUBITEM: LVCOLUMNW_MASK = LVCOLUMNW_MASK(8u32);
pub const LVCF_IMAGE: LVCOLUMNW_MASK = LVCOLUMNW_MASK(16u32);
pub const LVCF_ORDER: LVCOLUMNW_MASK = LVCOLUMNW_MASK(32u32);
pub const LVCF_MINWIDTH: LVCOLUMNW_MASK = LVCOLUMNW_MASK(64u32);
pub const LVCF_DEFAULTWIDTH: LVCOLUMNW_MASK = LVCOLUMNW_MASK(128u32);
pub const LVCF_IDEALWIDTH: LVCOLUMNW_MASK = LVCOLUMNW_MASK(256u32);
impl ::std::convert::From<u32> for LVCOLUMNW_MASK {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LVCOLUMNW_MASK {
    type Abi = Self;
}
impl ::std::ops::BitOr for LVCOLUMNW_MASK {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for LVCOLUMNW_MASK {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for LVCOLUMNW_MASK {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for LVCOLUMNW_MASK {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for LVCOLUMNW_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVFF_ITEMCOUNT: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct LVFINDINFOA {
    pub flags: LVFINDINFOW_FLAGS,
    pub psz: super::super::Foundation::PSTR,
    pub lParam: super::super::Foundation::LPARAM,
    pub pt: super::super::Foundation::POINT,
    pub vkDirection: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl LVFINDINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for LVFINDINFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for LVFINDINFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LVFINDINFOA").field("flags", &self.flags).field("psz", &self.psz).field("lParam", &self.lParam).field("pt", &self.pt).field("vkDirection", &self.vkDirection).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for LVFINDINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.psz == other.psz && self.lParam == other.lParam && self.pt == other.pt && self.vkDirection == other.vkDirection
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for LVFINDINFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LVFINDINFOA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct LVFINDINFOW {
    pub flags: LVFINDINFOW_FLAGS,
    pub psz: super::super::Foundation::PWSTR,
    pub lParam: super::super::Foundation::LPARAM,
    pub pt: super::super::Foundation::POINT,
    pub vkDirection: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl LVFINDINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for LVFINDINFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for LVFINDINFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LVFINDINFOW").field("flags", &self.flags).field("psz", &self.psz).field("lParam", &self.lParam).field("pt", &self.pt).field("vkDirection", &self.vkDirection).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for LVFINDINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.psz == other.psz && self.lParam == other.lParam && self.pt == other.pt && self.vkDirection == other.vkDirection
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for LVFINDINFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LVFINDINFOW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LVFINDINFOW_FLAGS(pub u32);
pub const LVFI_PARAM: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(1u32);
pub const LVFI_PARTIAL: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(8u32);
pub const LVFI_STRING: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(2u32);
pub const LVFI_SUBSTRING: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(4u32);
pub const LVFI_WRAP: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(32u32);
pub const LVFI_NEARESTXY: LVFINDINFOW_FLAGS = LVFINDINFOW_FLAGS(64u32);
impl ::std::convert::From<u32> for LVFINDINFOW_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LVFINDINFOW_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for LVFINDINFOW_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for LVFINDINFOW_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for LVFINDINFOW_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for LVFINDINFOW_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for LVFINDINFOW_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVFIS_FOCUSED: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct LVFOOTERINFO {
    pub mask: u32,
    pub pszText: super::super::Foundation::PWSTR,
    pub cchTextMax: i32,
    pub cItems: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl LVFOOTERINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for LVFOOTERINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for LVFOOTERINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LVFOOTERINFO").field("mask", &self.mask).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("cItems", &self.cItems).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for LVFOOTERINFO {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.cItems == other.cItems
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for LVFOOTERINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LVFOOTERINFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct LVFOOTERITEM {
    pub mask: LVFOOTERITEM_MASK,
    pub iItem: i32,
    pub pszText: super::super::Foundation::PWSTR,
    pub cchTextMax: i32,
    pub state: u32,
    pub stateMask: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl LVFOOTERITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for LVFOOTERITEM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for LVFOOTERITEM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LVFOOTERITEM").field("mask", &self.mask).field("iItem", &self.iItem).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("state", &self.state).field("stateMask", &self.stateMask).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for LVFOOTERITEM {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.iItem == other.iItem && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.state == other.state && self.stateMask == other.stateMask
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for LVFOOTERITEM {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LVFOOTERITEM {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LVFOOTERITEM_MASK(pub u32);
pub const LVFIF_TEXT: LVFOOTERITEM_MASK = LVFOOTERITEM_MASK(1u32);
pub const LVFIF_STATE: LVFOOTERITEM_MASK = LVFOOTERITEM_MASK(2u32);
impl ::std::convert::From<u32> for LVFOOTERITEM_MASK {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LVFOOTERITEM_MASK {
    type Abi = Self;
}
impl ::std::ops::BitOr for LVFOOTERITEM_MASK {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for LVFOOTERITEM_MASK {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for LVFOOTERITEM_MASK {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for LVFOOTERITEM_MASK {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for LVFOOTERITEM_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGA_FOOTER_CENTER: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGA_FOOTER_LEFT: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGA_FOOTER_RIGHT: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGF_ALIGN: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGF_DESCRIPTIONBOTTOM: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGF_DESCRIPTIONTOP: u32 = 1024u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGF_EXTENDEDIMAGE: u32 = 8192u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGF_GROUPID: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGF_ITEMS: u32 = 16384u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGF_SUBSET: u32 = 32768u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGF_SUBSETITEMS: u32 = 65536u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGF_SUBTITLE: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGF_TASK: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGF_TITLEIMAGE: u32 = 4096u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGGR_GROUP: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGGR_HEADER: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGGR_LABEL: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGGR_SUBSETLINK: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGIT_UNFOLDED: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGMF_BORDERCOLOR: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGMF_BORDERSIZE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGMF_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGMF_TEXTCOLOR: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct LVGROUP {
    pub cbSize: u32,
    pub mask: LVGROUP_MASK,
    pub pszHeader: super::super::Foundation::PWSTR,
    pub cchHeader: i32,
    pub pszFooter: super::super::Foundation::PWSTR,
    pub cchFooter: i32,
    pub iGroupId: i32,
    pub stateMask: u32,
    pub state: u32,
    pub uAlign: u32,
    pub pszSubtitle: super::super::Foundation::PWSTR,
    pub cchSubtitle: u32,
    pub pszTask: super::super::Foundation::PWSTR,
    pub cchTask: u32,
    pub pszDescriptionTop: super::super::Foundation::PWSTR,
    pub cchDescriptionTop: u32,
    pub pszDescriptionBottom: super::super::Foundation::PWSTR,
    pub cchDescriptionBottom: u32,
    pub iTitleImage: i32,
    pub iExtendedImage: i32,
    pub iFirstItem: i32,
    pub cItems: u32,
    pub pszSubsetTitle: super::super::Foundation::PWSTR,
    pub cchSubsetTitle: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl LVGROUP {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for LVGROUP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for LVGROUP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LVGROUP")
            .field("cbSize", &self.cbSize)
            .field("mask", &self.mask)
            .field("pszHeader", &self.pszHeader)
            .field("cchHeader", &self.cchHeader)
            .field("pszFooter", &self.pszFooter)
            .field("cchFooter", &self.cchFooter)
            .field("iGroupId", &self.iGroupId)
            .field("stateMask", &self.stateMask)
            .field("state", &self.state)
            .field("uAlign", &self.uAlign)
            .field("pszSubtitle", &self.pszSubtitle)
            .field("cchSubtitle", &self.cchSubtitle)
            .field("pszTask", &self.pszTask)
            .field("cchTask", &self.cchTask)
            .field("pszDescriptionTop", &self.pszDescriptionTop)
            .field("cchDescriptionTop", &self.cchDescriptionTop)
            .field("pszDescriptionBottom", &self.pszDescriptionBottom)
            .field("cchDescriptionBottom", &self.cchDescriptionBottom)
            .field("iTitleImage", &self.iTitleImage)
            .field("iExtendedImage", &self.iExtendedImage)
            .field("iFirstItem", &self.iFirstItem)
            .field("cItems", &self.cItems)
            .field("pszSubsetTitle", &self.pszSubsetTitle)
            .field("cchSubsetTitle", &self.cchSubsetTitle)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for LVGROUP {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.mask == other.mask
            && self.pszHeader == other.pszHeader
            && self.cchHeader == other.cchHeader
            && self.pszFooter == other.pszFooter
            && self.cchFooter == other.cchFooter
            && self.iGroupId == other.iGroupId
            && self.stateMask == other.stateMask
            && self.state == other.state
            && self.uAlign == other.uAlign
            && self.pszSubtitle == other.pszSubtitle
            && self.cchSubtitle == other.cchSubtitle
            && self.pszTask == other.pszTask
            && self.cchTask == other.cchTask
            && self.pszDescriptionTop == other.pszDescriptionTop
            && self.cchDescriptionTop == other.cchDescriptionTop
            && self.pszDescriptionBottom == other.pszDescriptionBottom
            && self.cchDescriptionBottom == other.cchDescriptionBottom
            && self.iTitleImage == other.iTitleImage
            && self.iExtendedImage == other.iExtendedImage
            && self.iFirstItem == other.iFirstItem
            && self.cItems == other.cItems
            && self.pszSubsetTitle == other.pszSubsetTitle
            && self.cchSubsetTitle == other.cchSubsetTitle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for LVGROUP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LVGROUP {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct LVGROUPMETRICS {
    pub cbSize: u32,
    pub mask: u32,
    pub Left: u32,
    pub Top: u32,
    pub Right: u32,
    pub Bottom: u32,
    pub crLeft: u32,
    pub crTop: u32,
    pub crRight: u32,
    pub crBottom: u32,
    pub crHeader: u32,
    pub crFooter: u32,
}
impl LVGROUPMETRICS {}
impl ::std::default::Default for LVGROUPMETRICS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for LVGROUPMETRICS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LVGROUPMETRICS")
            .field("cbSize", &self.cbSize)
            .field("mask", &self.mask)
            .field("Left", &self.Left)
            .field("Top", &self.Top)
            .field("Right", &self.Right)
            .field("Bottom", &self.Bottom)
            .field("crLeft", &self.crLeft)
            .field("crTop", &self.crTop)
            .field("crRight", &self.crRight)
            .field("crBottom", &self.crBottom)
            .field("crHeader", &self.crHeader)
            .field("crFooter", &self.crFooter)
            .finish()
    }
}
impl ::std::cmp::PartialEq for LVGROUPMETRICS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.mask == other.mask && self.Left == other.Left && self.Top == other.Top && self.Right == other.Right && self.Bottom == other.Bottom && self.crLeft == other.crLeft && self.crTop == other.crTop && self.crRight == other.crRight && self.crBottom == other.crBottom && self.crHeader == other.crHeader && self.crFooter == other.crFooter
    }
}
impl ::std::cmp::Eq for LVGROUPMETRICS {}
unsafe impl ::windows::runtime::Abi for LVGROUPMETRICS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LVGROUP_MASK(pub u32);
pub const LVGF_NONE: LVGROUP_MASK = LVGROUP_MASK(0u32);
pub const LVGF_HEADER: LVGROUP_MASK = LVGROUP_MASK(1u32);
pub const LVGF_FOOTER: LVGROUP_MASK = LVGROUP_MASK(2u32);
pub const LVGF_STATE: LVGROUP_MASK = LVGROUP_MASK(4u32);
impl ::std::convert::From<u32> for LVGROUP_MASK {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LVGROUP_MASK {
    type Abi = Self;
}
impl ::std::ops::BitOr for LVGROUP_MASK {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for LVGROUP_MASK {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for LVGROUP_MASK {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for LVGROUP_MASK {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for LVGROUP_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGS_COLLAPSED: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGS_COLLAPSIBLE: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGS_FOCUSED: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGS_HIDDEN: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGS_NOHEADER: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGS_NORMAL: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGS_SELECTED: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGS_SUBSETED: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVGS_SUBSETLINKFOCUSED: u32 = 128u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct LVHITTESTINFO {
    pub pt: super::super::Foundation::POINT,
    pub flags: LVHITTESTINFO_FLAGS,
    pub iItem: i32,
    pub iSubItem: i32,
    pub iGroup: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl LVHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for LVHITTESTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for LVHITTESTINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LVHITTESTINFO").field("pt", &self.pt).field("flags", &self.flags).field("iItem", &self.iItem).field("iSubItem", &self.iSubItem).field("iGroup", &self.iGroup).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for LVHITTESTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.pt == other.pt && self.flags == other.flags && self.iItem == other.iItem && self.iSubItem == other.iSubItem && self.iGroup == other.iGroup
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for LVHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LVHITTESTINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LVHITTESTINFO_FLAGS(pub u32);
pub const LVHT_ABOVE: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(8u32);
pub const LVHT_BELOW: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(16u32);
pub const LVHT_NOWHERE: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(1u32);
pub const LVHT_ONITEMICON: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(2u32);
pub const LVHT_ONITEMLABEL: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(4u32);
pub const LVHT_ONITEMSTATEICON: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(8u32);
pub const LVHT_TOLEFT: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(64u32);
pub const LVHT_TORIGHT: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(32u32);
pub const LVHT_EX_GROUP_HEADER: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(268435456u32);
pub const LVHT_EX_GROUP_FOOTER: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(536870912u32);
pub const LVHT_EX_GROUP_COLLAPSE: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(1073741824u32);
pub const LVHT_EX_GROUP_BACKGROUND: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(2147483648u32);
pub const LVHT_EX_GROUP_STATEICON: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(16777216u32);
pub const LVHT_EX_GROUP_SUBSETLINK: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(33554432u32);
pub const LVHT_EX_GROUP: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(4076863488u32);
pub const LVHT_EX_ONCONTENTS: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(67108864u32);
pub const LVHT_EX_FOOTER: LVHITTESTINFO_FLAGS = LVHITTESTINFO_FLAGS(134217728u32);
impl ::std::convert::From<u32> for LVHITTESTINFO_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LVHITTESTINFO_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for LVHITTESTINFO_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for LVHITTESTINFO_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for LVHITTESTINFO_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for LVHITTESTINFO_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for LVHITTESTINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVIF_COLFMT: u32 = 65536u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVIF_COLUMNS: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVIF_DI_SETITEM: u32 = 4096u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVIF_GROUPID: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVIF_IMAGE: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVIF_INDENT: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVIF_NORECOMPUTE: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVIF_PARAM: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVIF_STATE: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVIF_TEXT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVIM_AFTER: u32 = 1u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct LVINSERTGROUPSORTED {
    pub pfnGroupCompare: ::std::option::Option<PFNLVGROUPCOMPARE>,
    pub pvData: *mut ::std::ffi::c_void,
    pub lvGroup: LVGROUP,
}
#[cfg(feature = "Win32_Foundation")]
impl LVINSERTGROUPSORTED {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for LVINSERTGROUPSORTED {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for LVINSERTGROUPSORTED {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LVINSERTGROUPSORTED").field("pvData", &self.pvData).field("lvGroup", &self.lvGroup).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for LVINSERTGROUPSORTED {
    fn eq(&self, other: &Self) -> bool {
        self.pfnGroupCompare.map(|f| f as usize) == other.pfnGroupCompare.map(|f| f as usize) && self.pvData == other.pvData && self.lvGroup == other.lvGroup
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for LVINSERTGROUPSORTED {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LVINSERTGROUPSORTED {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct LVINSERTMARK {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub iItem: i32,
    pub dwReserved: u32,
}
impl LVINSERTMARK {}
impl ::std::default::Default for LVINSERTMARK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for LVINSERTMARK {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LVINSERTMARK").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("iItem", &self.iItem).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::std::cmp::PartialEq for LVINSERTMARK {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.iItem == other.iItem && self.dwReserved == other.dwReserved
    }
}
impl ::std::cmp::Eq for LVINSERTMARK {}
unsafe impl ::windows::runtime::Abi for LVINSERTMARK {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVIR_BOUNDS: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVIR_ICON: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVIR_LABEL: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVIR_SELECTBOUNDS: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVIS_ACTIVATING: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVIS_CUT: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVIS_DROPHILITED: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVIS_FOCUSED: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVIS_GLOW: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVIS_OVERLAYMASK: u32 = 3840u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVIS_SELECTED: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVIS_STATEIMAGEMASK: u32 = 61440u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct LVITEMA {
    pub mask: u32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: super::super::Foundation::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::super::Foundation::LPARAM,
    pub iIndent: i32,
    pub iGroupId: LVITEMA_GROUP_ID,
    pub cColumns: u32,
    pub puColumns: *mut u32,
    pub piColFmt: *mut i32,
    pub iGroup: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl LVITEMA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for LVITEMA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for LVITEMA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LVITEMA")
            .field("mask", &self.mask)
            .field("iItem", &self.iItem)
            .field("iSubItem", &self.iSubItem)
            .field("state", &self.state)
            .field("stateMask", &self.stateMask)
            .field("pszText", &self.pszText)
            .field("cchTextMax", &self.cchTextMax)
            .field("iImage", &self.iImage)
            .field("lParam", &self.lParam)
            .field("iIndent", &self.iIndent)
            .field("iGroupId", &self.iGroupId)
            .field("cColumns", &self.cColumns)
            .field("puColumns", &self.puColumns)
            .field("piColFmt", &self.piColFmt)
            .field("iGroup", &self.iGroup)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for LVITEMA {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask
            && self.iItem == other.iItem
            && self.iSubItem == other.iSubItem
            && self.state == other.state
            && self.stateMask == other.stateMask
            && self.pszText == other.pszText
            && self.cchTextMax == other.cchTextMax
            && self.iImage == other.iImage
            && self.lParam == other.lParam
            && self.iIndent == other.iIndent
            && self.iGroupId == other.iGroupId
            && self.cColumns == other.cColumns
            && self.puColumns == other.puColumns
            && self.piColFmt == other.piColFmt
            && self.iGroup == other.iGroup
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for LVITEMA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LVITEMA {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LVITEMA_GROUP_ID(pub i32);
pub const I_GROUPIDCALLBACK: LVITEMA_GROUP_ID = LVITEMA_GROUP_ID(-1i32);
pub const I_GROUPIDNONE: LVITEMA_GROUP_ID = LVITEMA_GROUP_ID(-2i32);
impl ::std::convert::From<i32> for LVITEMA_GROUP_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LVITEMA_GROUP_ID {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct LVITEMINDEX {
    pub iItem: i32,
    pub iGroup: i32,
}
impl LVITEMINDEX {}
impl ::std::default::Default for LVITEMINDEX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for LVITEMINDEX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LVITEMINDEX").field("iItem", &self.iItem).field("iGroup", &self.iGroup).finish()
    }
}
impl ::std::cmp::PartialEq for LVITEMINDEX {
    fn eq(&self, other: &Self) -> bool {
        self.iItem == other.iItem && self.iGroup == other.iGroup
    }
}
impl ::std::cmp::Eq for LVITEMINDEX {}
unsafe impl ::windows::runtime::Abi for LVITEMINDEX {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct LVITEMW {
    pub mask: u32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: super::super::Foundation::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::super::Foundation::LPARAM,
    pub iIndent: i32,
    pub iGroupId: LVITEMA_GROUP_ID,
    pub cColumns: u32,
    pub puColumns: *mut u32,
    pub piColFmt: *mut i32,
    pub iGroup: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl LVITEMW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for LVITEMW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for LVITEMW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LVITEMW")
            .field("mask", &self.mask)
            .field("iItem", &self.iItem)
            .field("iSubItem", &self.iSubItem)
            .field("state", &self.state)
            .field("stateMask", &self.stateMask)
            .field("pszText", &self.pszText)
            .field("cchTextMax", &self.cchTextMax)
            .field("iImage", &self.iImage)
            .field("lParam", &self.lParam)
            .field("iIndent", &self.iIndent)
            .field("iGroupId", &self.iGroupId)
            .field("cColumns", &self.cColumns)
            .field("puColumns", &self.puColumns)
            .field("piColFmt", &self.piColFmt)
            .field("iGroup", &self.iGroup)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for LVITEMW {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask
            && self.iItem == other.iItem
            && self.iSubItem == other.iSubItem
            && self.state == other.state
            && self.stateMask == other.stateMask
            && self.pszText == other.pszText
            && self.cchTextMax == other.cchTextMax
            && self.iImage == other.iImage
            && self.lParam == other.lParam
            && self.iIndent == other.iIndent
            && self.iGroupId == other.iGroupId
            && self.cColumns == other.cColumns
            && self.puColumns == other.puColumns
            && self.piColFmt == other.piColFmt
            && self.iGroup == other.iGroup
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for LVITEMW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LVITEMW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVKF_ALT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVKF_CONTROL: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVKF_SHIFT: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_APPROXIMATEVIEWRECT: u32 = 4160u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_ARRANGE: u32 = 4118u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_CANCELEDITLABEL: u32 = 4275u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_CREATEDRAGIMAGE: u32 = 4129u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_DELETEALLITEMS: u32 = 4105u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_DELETECOLUMN: u32 = 4124u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_DELETEITEM: u32 = 4104u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_EDITLABEL: u32 = 4214u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_EDITLABELA: u32 = 4119u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_EDITLABELW: u32 = 4214u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_ENABLEGROUPVIEW: u32 = 4253u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_ENSUREVISIBLE: u32 = 4115u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_FINDITEM: u32 = 4179u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_FINDITEMA: u32 = 4109u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_FINDITEMW: u32 = 4179u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_FIRST: u32 = 4096u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETBKCOLOR: u32 = 4096u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETBKIMAGE: u32 = 4235u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETBKIMAGEA: u32 = 4165u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETBKIMAGEW: u32 = 4235u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETCALLBACKMASK: u32 = 4106u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETCOLUMN: u32 = 4191u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETCOLUMNA: u32 = 4121u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETCOLUMNORDERARRAY: u32 = 4155u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETCOLUMNW: u32 = 4191u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETCOLUMNWIDTH: u32 = 4125u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETCOUNTPERPAGE: u32 = 4136u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETEDITCONTROL: u32 = 4120u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETEMPTYTEXT: u32 = 4300u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETEXTENDEDLISTVIEWSTYLE: u32 = 4151u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETFOCUSEDGROUP: u32 = 4189u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETFOOTERINFO: u32 = 4302u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETFOOTERITEM: u32 = 4304u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETFOOTERITEMRECT: u32 = 4303u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETFOOTERRECT: u32 = 4301u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETGROUPCOUNT: u32 = 4248u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETGROUPINFO: u32 = 4245u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETGROUPINFOBYINDEX: u32 = 4249u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETGROUPMETRICS: u32 = 4252u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETGROUPRECT: u32 = 4194u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETGROUPSTATE: u32 = 4188u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETHEADER: u32 = 4127u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETHOTCURSOR: u32 = 4159u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETHOTITEM: u32 = 4157u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETHOVERTIME: u32 = 4168u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETIMAGELIST: u32 = 4098u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETINSERTMARK: u32 = 4263u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETINSERTMARKCOLOR: u32 = 4267u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETINSERTMARKRECT: u32 = 4265u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETISEARCHSTRING: u32 = 4213u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETISEARCHSTRINGA: u32 = 4148u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETISEARCHSTRINGW: u32 = 4213u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETITEM: u32 = 4171u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETITEMA: u32 = 4101u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETITEMCOUNT: u32 = 4100u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETITEMINDEXRECT: u32 = 4305u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETITEMPOSITION: u32 = 4112u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETITEMRECT: u32 = 4110u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETITEMSPACING: u32 = 4147u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETITEMSTATE: u32 = 4140u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETITEMTEXT: u32 = 4211u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETITEMTEXTA: u32 = 4141u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETITEMTEXTW: u32 = 4211u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETITEMW: u32 = 4171u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETNEXTITEM: u32 = 4108u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETNEXTITEMINDEX: u32 = 4307u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETNUMBEROFWORKAREAS: u32 = 4169u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETORIGIN: u32 = 4137u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETOUTLINECOLOR: u32 = 4272u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETSELECTEDCOLUMN: u32 = 4270u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETSELECTEDCOUNT: u32 = 4146u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETSELECTIONMARK: u32 = 4162u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETSTRINGWIDTH: u32 = 4183u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETSTRINGWIDTHA: u32 = 4113u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETSTRINGWIDTHW: u32 = 4183u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETSUBITEMRECT: u32 = 4152u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETTEXTBKCOLOR: u32 = 4133u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETTEXTCOLOR: u32 = 4131u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETTILEINFO: u32 = 4261u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETTILEVIEWINFO: u32 = 4259u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETTOOLTIPS: u32 = 4174u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETTOPINDEX: u32 = 4135u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETUNICODEFORMAT: u32 = 8198u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETVIEW: u32 = 4239u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETVIEWRECT: u32 = 4130u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_GETWORKAREAS: u32 = 4166u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_HASGROUP: u32 = 4257u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_HITTEST: u32 = 4114u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_INSERTCOLUMNA: u32 = 4123u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_INSERTCOLUMNW: u32 = 4193u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_INSERTGROUP: u32 = 4241u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_INSERTGROUPSORTED: u32 = 4255u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_INSERTITEM: u32 = 4173u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_INSERTITEMA: u32 = 4103u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_INSERTITEMW: u32 = 4173u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_INSERTMARKHITTEST: u32 = 4264u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_ISGROUPVIEWENABLED: u32 = 4271u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_ISITEMVISIBLE: u32 = 4278u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_MAPIDTOINDEX: u32 = 4277u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_MAPINDEXTOID: u32 = 4276u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_MOVEGROUP: u32 = 4247u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_MOVEITEMTOGROUP: u32 = 4250u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_REDRAWITEMS: u32 = 4117u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_REMOVEALLGROUPS: u32 = 4256u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_REMOVEGROUP: u32 = 4246u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SCROLL: u32 = 4116u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETBKCOLOR: u32 = 4097u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETBKIMAGE: u32 = 4234u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETBKIMAGEA: u32 = 4164u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETBKIMAGEW: u32 = 4234u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETCALLBACKMASK: u32 = 4107u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETCOLUMN: u32 = 4192u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETCOLUMNA: u32 = 4122u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETCOLUMNORDERARRAY: u32 = 4154u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETCOLUMNW: u32 = 4192u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETCOLUMNWIDTH: u32 = 4126u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETEXTENDEDLISTVIEWSTYLE: u32 = 4150u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETGROUPINFO: u32 = 4243u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETGROUPMETRICS: u32 = 4251u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETHOTCURSOR: u32 = 4158u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETHOTITEM: u32 = 4156u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETHOVERTIME: u32 = 4167u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETICONSPACING: u32 = 4149u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETIMAGELIST: u32 = 4099u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETINFOTIP: u32 = 4269u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETINSERTMARK: u32 = 4262u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETINSERTMARKCOLOR: u32 = 4266u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETITEM: u32 = 4172u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETITEMA: u32 = 4102u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETITEMCOUNT: u32 = 4143u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETITEMINDEXSTATE: u32 = 4306u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETITEMPOSITION: u32 = 4111u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETITEMPOSITION32: u32 = 4145u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETITEMSTATE: u32 = 4139u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETITEMTEXT: u32 = 4212u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETITEMTEXTA: u32 = 4142u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETITEMTEXTW: u32 = 4212u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETITEMW: u32 = 4172u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETOUTLINECOLOR: u32 = 4273u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETSELECTEDCOLUMN: u32 = 4236u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETSELECTIONMARK: u32 = 4163u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETTEXTBKCOLOR: u32 = 4134u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETTEXTCOLOR: u32 = 4132u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETTILEINFO: u32 = 4260u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETTILEVIEWINFO: u32 = 4258u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETTOOLTIPS: u32 = 4170u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETUNICODEFORMAT: u32 = 8197u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETVIEW: u32 = 4238u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SETWORKAREAS: u32 = 4161u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SORTGROUPS: u32 = 4254u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SORTITEMS: u32 = 4144u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SORTITEMSEX: u32 = 4177u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_SUBITEMHITTEST: u32 = 4153u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVM_UPDATE: u32 = 4138u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVNI_ABOVE: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVNI_ALL: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVNI_BELOW: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVNI_CUT: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVNI_DROPHILITED: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVNI_FOCUSED: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVNI_PREVIOUS: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVNI_SAMEGROUPONLY: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVNI_SELECTED: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVNI_TOLEFT: u32 = 1024u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVNI_TORIGHT: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVNI_VISIBLEONLY: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVNI_VISIBLEORDER: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVNSCH_DEFAULT: i32 = -1i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVNSCH_ERROR: i32 = -2i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVNSCH_IGNORE: i32 = -3i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVSCW_AUTOSIZE: i32 = -1i32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVSCW_AUTOSIZE_USEHEADER: i32 = -2i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct LVSETINFOTIP {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub pszText: super::super::Foundation::PWSTR,
    pub iItem: i32,
    pub iSubItem: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl LVSETINFOTIP {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for LVSETINFOTIP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for LVSETINFOTIP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LVSETINFOTIP").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("pszText", &self.pszText).field("iItem", &self.iItem).field("iSubItem", &self.iSubItem).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for LVSETINFOTIP {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.pszText == other.pszText && self.iItem == other.iItem && self.iSubItem == other.iSubItem
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for LVSETINFOTIP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LVSETINFOTIP {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVSICF_NOINVALIDATEALL: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVSICF_NOSCROLL: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVSIL_GROUPHEADER: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVSIL_NORMAL: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVSIL_SMALL: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVSIL_STATE: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_ALIGNLEFT: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_ALIGNMASK: u32 = 3072u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_ALIGNTOP: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_AUTOARRANGE: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EDITLABELS: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_AUTOAUTOARRANGE: u32 = 16777216u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_AUTOCHECKSELECT: u32 = 134217728u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_AUTOSIZECOLUMNS: u32 = 268435456u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_BORDERSELECT: u32 = 32768u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_CHECKBOXES: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_COLUMNOVERFLOW: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_COLUMNSNAPPOINTS: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_DOUBLEBUFFER: u32 = 65536u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_FLATSB: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_FULLROWSELECT: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_GRIDLINES: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_HEADERDRAGDROP: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_HEADERINALLVIEWS: u32 = 33554432u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_HIDELABELS: u32 = 131072u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_INFOTIP: u32 = 1024u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_JUSTIFYCOLUMNS: u32 = 2097152u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_LABELTIP: u32 = 16384u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_MULTIWORKAREAS: u32 = 8192u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_ONECLICKACTIVATE: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_REGIONAL: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_SIMPLESELECT: u32 = 1048576u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_SINGLEROW: u32 = 262144u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_SNAPTOGRID: u32 = 524288u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_SUBITEMIMAGES: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_TRACKSELECT: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_TRANSPARENTBKGND: u32 = 4194304u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_TRANSPARENTSHADOWTEXT: u32 = 8388608u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_TWOCLICKACTIVATE: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_UNDERLINECOLD: u32 = 4096u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_EX_UNDERLINEHOT: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_ICON: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_LIST: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_NOCOLUMNHEADER: u32 = 16384u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_NOLABELWRAP: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_NOSCROLL: u32 = 8192u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_NOSORTHEADER: u32 = 32768u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_OWNERDATA: u32 = 4096u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_OWNERDRAWFIXED: u32 = 1024u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_REPORT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_SHAREIMAGELISTS: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_SHOWSELALWAYS: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_SINGLESEL: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_SMALLICON: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_SORTASCENDING: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_SORTDESCENDING: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_TYPEMASK: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVS_TYPESTYLEMASK: u32 = 64512u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct LVTILEINFO {
    pub cbSize: u32,
    pub iItem: i32,
    pub cColumns: u32,
    pub puColumns: *mut u32,
    pub piColFmt: *mut i32,
}
impl LVTILEINFO {}
impl ::std::default::Default for LVTILEINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for LVTILEINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LVTILEINFO").field("cbSize", &self.cbSize).field("iItem", &self.iItem).field("cColumns", &self.cColumns).field("puColumns", &self.puColumns).field("piColFmt", &self.piColFmt).finish()
    }
}
impl ::std::cmp::PartialEq for LVTILEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.iItem == other.iItem && self.cColumns == other.cColumns && self.puColumns == other.puColumns && self.piColFmt == other.piColFmt
    }
}
impl ::std::cmp::Eq for LVTILEINFO {}
unsafe impl ::windows::runtime::Abi for LVTILEINFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct LVTILEVIEWINFO {
    pub cbSize: u32,
    pub dwMask: u32,
    pub dwFlags: LVTILEVIEWINFO_FLAGS,
    pub sizeTile: super::super::Foundation::SIZE,
    pub cLines: i32,
    pub rcLabelMargin: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl LVTILEVIEWINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for LVTILEVIEWINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for LVTILEVIEWINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LVTILEVIEWINFO").field("cbSize", &self.cbSize).field("dwMask", &self.dwMask).field("dwFlags", &self.dwFlags).field("sizeTile", &self.sizeTile).field("cLines", &self.cLines).field("rcLabelMargin", &self.rcLabelMargin).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for LVTILEVIEWINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwMask == other.dwMask && self.dwFlags == other.dwFlags && self.sizeTile == other.sizeTile && self.cLines == other.cLines && self.rcLabelMargin == other.rcLabelMargin
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for LVTILEVIEWINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LVTILEVIEWINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LVTILEVIEWINFO_FLAGS(pub u32);
pub const LVTVIF_EXTENDED: LVTILEVIEWINFO_FLAGS = LVTILEVIEWINFO_FLAGS(4u32);
impl ::std::convert::From<u32> for LVTILEVIEWINFO_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LVTILEVIEWINFO_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for LVTILEVIEWINFO_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for LVTILEVIEWINFO_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for LVTILEVIEWINFO_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for LVTILEVIEWINFO_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for LVTILEVIEWINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVTVIF_AUTOSIZE: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVTVIF_FIXEDHEIGHT: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVTVIF_FIXEDSIZE: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVTVIF_FIXEDWIDTH: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVTVIM_COLUMNS: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVTVIM_LABELMARGIN: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LVTVIM_TILESIZE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LV_MAX_WORKAREAS: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LV_VIEW_DETAILS: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LV_VIEW_ICON: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LV_VIEW_LIST: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LV_VIEW_MAX: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LV_VIEW_SMALLICON: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LV_VIEW_TILE: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LWS_IGNORERETURN: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LWS_NOPREFIX: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LWS_RIGHT: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LWS_TRANSPARENT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LWS_USECUSTOMTEXT: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const LWS_USEVISUALSTYLE: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn LoadIconMetric<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hinst: Param0, pszname: Param1, lims: _LI_METRIC) -> ::windows::runtime::Result<super::WindowsAndMessaging::HICON> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadIconMetric(hinst: super::super::Foundation::HINSTANCE, pszname: super::super::Foundation::PWSTR, lims: _LI_METRIC, phico: *mut super::WindowsAndMessaging::HICON) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::WindowsAndMessaging::HICON as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        LoadIconMetric(hinst.into_param().abi(), pszname.into_param().abi(), ::std::mem::transmute(lims), &mut result__).from_abi::<super::WindowsAndMessaging::HICON>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn LoadIconWithScaleDown<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hinst: Param0, pszname: Param1, cx: i32, cy: i32) -> ::windows::runtime::Result<super::WindowsAndMessaging::HICON> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadIconWithScaleDown(hinst: super::super::Foundation::HINSTANCE, pszname: super::super::Foundation::PWSTR, cx: i32, cy: i32, phico: *mut super::WindowsAndMessaging::HICON) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::WindowsAndMessaging::HICON as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        LoadIconWithScaleDown(hinst.into_param().abi(), pszname.into_param().abi(), ::std::mem::transmute(cx), ::std::mem::transmute(cy), &mut result__).from_abi::<super::WindowsAndMessaging::HICON>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct MARGINS {
    pub cxLeftWidth: i32,
    pub cxRightWidth: i32,
    pub cyTopHeight: i32,
    pub cyBottomHeight: i32,
}
impl MARGINS {}
impl ::std::default::Default for MARGINS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MARGINS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MARGINS").field("cxLeftWidth", &self.cxLeftWidth).field("cxRightWidth", &self.cxRightWidth).field("cyTopHeight", &self.cyTopHeight).field("cyBottomHeight", &self.cyBottomHeight).finish()
    }
}
impl ::std::cmp::PartialEq for MARGINS {
    fn eq(&self, other: &Self) -> bool {
        self.cxLeftWidth == other.cxLeftWidth && self.cxRightWidth == other.cxRightWidth && self.cyTopHeight == other.cyTopHeight && self.cyBottomHeight == other.cyBottomHeight
    }
}
impl ::std::cmp::Eq for MARGINS {}
unsafe impl ::windows::runtime::Abi for MARGINS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MARKUPTEXTSTATES(pub i32);
pub const EMT_NORMALTEXT: MARKUPTEXTSTATES = MARKUPTEXTSTATES(1i32);
pub const EMT_LINKTEXT: MARKUPTEXTSTATES = MARKUPTEXTSTATES(2i32);
impl ::std::convert::From<i32> for MARKUPTEXTSTATES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MARKUPTEXTSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MAXPROPPAGES: u32 = 100u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MAX_INTLIST_COUNT: u32 = 402u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MAX_LINKID_TEXT: u32 = 48u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MAX_THEMECOLOR: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MAX_THEMESIZE: u32 = 64u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct MCGRIDINFO {
    pub cbSize: u32,
    pub dwPart: MCGRIDINFO_PART,
    pub dwFlags: MCGRIDINFO_FLAGS,
    pub iCalendar: i32,
    pub iRow: i32,
    pub iCol: i32,
    pub bSelected: super::super::Foundation::BOOL,
    pub stStart: super::super::Foundation::SYSTEMTIME,
    pub stEnd: super::super::Foundation::SYSTEMTIME,
    pub rc: super::super::Foundation::RECT,
    pub pszName: super::super::Foundation::PWSTR,
    pub cchName: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl MCGRIDINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCGRIDINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MCGRIDINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MCGRIDINFO")
            .field("cbSize", &self.cbSize)
            .field("dwPart", &self.dwPart)
            .field("dwFlags", &self.dwFlags)
            .field("iCalendar", &self.iCalendar)
            .field("iRow", &self.iRow)
            .field("iCol", &self.iCol)
            .field("bSelected", &self.bSelected)
            .field("stStart", &self.stStart)
            .field("stEnd", &self.stEnd)
            .field("rc", &self.rc)
            .field("pszName", &self.pszName)
            .field("cchName", &self.cchName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCGRIDINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwPart == other.dwPart && self.dwFlags == other.dwFlags && self.iCalendar == other.iCalendar && self.iRow == other.iRow && self.iCol == other.iCol && self.bSelected == other.bSelected && self.stStart == other.stStart && self.stEnd == other.stEnd && self.rc == other.rc && self.pszName == other.pszName && self.cchName == other.cchName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCGRIDINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCGRIDINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MCGRIDINFO_FLAGS(pub u32);
pub const MCGIF_DATE: MCGRIDINFO_FLAGS = MCGRIDINFO_FLAGS(1u32);
pub const MCGIF_RECT: MCGRIDINFO_FLAGS = MCGRIDINFO_FLAGS(2u32);
pub const MCGIF_NAME: MCGRIDINFO_FLAGS = MCGRIDINFO_FLAGS(4u32);
impl ::std::convert::From<u32> for MCGRIDINFO_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MCGRIDINFO_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for MCGRIDINFO_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MCGRIDINFO_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MCGRIDINFO_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MCGRIDINFO_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MCGRIDINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MCGRIDINFO_PART(pub u32);
pub const MCGIP_CALENDARCONTROL: MCGRIDINFO_PART = MCGRIDINFO_PART(0u32);
pub const MCGIP_NEXT: MCGRIDINFO_PART = MCGRIDINFO_PART(1u32);
pub const MCGIP_PREV: MCGRIDINFO_PART = MCGRIDINFO_PART(2u32);
pub const MCGIP_FOOTER: MCGRIDINFO_PART = MCGRIDINFO_PART(3u32);
pub const MCGIP_CALENDAR: MCGRIDINFO_PART = MCGRIDINFO_PART(4u32);
pub const MCGIP_CALENDARHEADER: MCGRIDINFO_PART = MCGRIDINFO_PART(5u32);
pub const MCGIP_CALENDARBODY: MCGRIDINFO_PART = MCGRIDINFO_PART(6u32);
pub const MCGIP_CALENDARROW: MCGRIDINFO_PART = MCGRIDINFO_PART(7u32);
pub const MCGIP_CALENDARCELL: MCGRIDINFO_PART = MCGRIDINFO_PART(8u32);
impl ::std::convert::From<u32> for MCGRIDINFO_PART {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MCGRIDINFO_PART {
    type Abi = Self;
}
impl ::std::ops::BitOr for MCGRIDINFO_PART {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MCGRIDINFO_PART {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MCGRIDINFO_PART {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MCGRIDINFO_PART {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MCGRIDINFO_PART {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct MCHITTESTINFO {
    pub cbSize: u32,
    pub pt: super::super::Foundation::POINT,
    pub uHit: u32,
    pub st: super::super::Foundation::SYSTEMTIME,
    pub rc: super::super::Foundation::RECT,
    pub iOffset: i32,
    pub iRow: i32,
    pub iCol: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl MCHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCHITTESTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MCHITTESTINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MCHITTESTINFO").field("cbSize", &self.cbSize).field("pt", &self.pt).field("uHit", &self.uHit).field("st", &self.st).field("rc", &self.rc).field("iOffset", &self.iOffset).field("iRow", &self.iRow).field("iCol", &self.iCol).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCHITTESTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pt == other.pt && self.uHit == other.uHit && self.st == other.st && self.rc == other.rc && self.iOffset == other.iOffset && self.iRow == other.iRow && self.iCol == other.iCol
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCHITTESTINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCHT_CALENDAR: u32 = 131072u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCHT_CALENDARBK: u32 = 131072u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCHT_CALENDARCONTROL: u32 = 1048576u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCHT_NEXT: u32 = 16777216u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCHT_NOWHERE: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCHT_PREV: u32 = 33554432u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCHT_TITLE: u32 = 65536u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCHT_TITLEBK: u32 = 65536u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCHT_TODAYLINK: u32 = 196608u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCMV_CENTURY: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCMV_DECADE: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCMV_MAX: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCMV_MONTH: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCMV_YEAR: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_FIRST: u32 = 4096u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_GETCALENDARBORDER: u32 = 4127u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_GETCALENDARCOUNT: u32 = 4119u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_GETCALENDARGRIDINFO: u32 = 4120u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_GETCALID: u32 = 4123u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_GETCOLOR: u32 = 4107u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_GETCURRENTVIEW: u32 = 4118u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_GETCURSEL: u32 = 4097u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_GETFIRSTDAYOFWEEK: u32 = 4112u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_GETMAXSELCOUNT: u32 = 4099u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_GETMAXTODAYWIDTH: u32 = 4117u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_GETMINREQRECT: u32 = 4105u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_GETMONTHDELTA: u32 = 4115u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_GETMONTHRANGE: u32 = 4103u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_GETRANGE: u32 = 4113u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_GETSELRANGE: u32 = 4101u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_GETTODAY: u32 = 4109u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_GETUNICODEFORMAT: u32 = 8198u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_HITTEST: u32 = 4110u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_SETCALENDARBORDER: u32 = 4126u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_SETCALID: u32 = 4124u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_SETCOLOR: u32 = 4106u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_SETCURRENTVIEW: u32 = 4128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_SETCURSEL: u32 = 4098u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_SETDAYSTATE: u32 = 4104u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_SETFIRSTDAYOFWEEK: u32 = 4111u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_SETMAXSELCOUNT: u32 = 4100u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_SETMONTHDELTA: u32 = 4116u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_SETRANGE: u32 = 4114u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_SETSELRANGE: u32 = 4102u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_SETTODAY: u32 = 4108u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_SETUNICODEFORMAT: u32 = 8197u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCM_SIZERECTTOMIN: u32 = 4125u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCSC_BACKGROUND: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCSC_MONTHBK: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCSC_TEXT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCSC_TITLEBK: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCSC_TITLETEXT: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCSC_TRAILINGTEXT: u32 = 5u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCS_DAYSTATE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCS_MULTISELECT: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCS_NOSELCHANGEONNAV: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCS_NOTODAY: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCS_NOTODAYCIRCLE: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCS_NOTRAILINGDATES: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCS_SHORTDAYSOFWEEK: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MCS_WEEKNUMBERS: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct MEASUREITEMSTRUCT {
    pub CtlType: u32,
    pub CtlID: u32,
    pub itemID: u32,
    pub itemWidth: u32,
    pub itemHeight: u32,
    pub itemData: usize,
}
impl MEASUREITEMSTRUCT {}
impl ::std::default::Default for MEASUREITEMSTRUCT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MEASUREITEMSTRUCT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MEASUREITEMSTRUCT").field("CtlType", &self.CtlType).field("CtlID", &self.CtlID).field("itemID", &self.itemID).field("itemWidth", &self.itemWidth).field("itemHeight", &self.itemHeight).field("itemData", &self.itemData).finish()
    }
}
impl ::std::cmp::PartialEq for MEASUREITEMSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.CtlType == other.CtlType && self.CtlID == other.CtlID && self.itemID == other.itemID && self.itemWidth == other.itemWidth && self.itemHeight == other.itemHeight && self.itemData == other.itemData
    }
}
impl ::std::cmp::Eq for MEASUREITEMSTRUCT {}
unsafe impl ::windows::runtime::Abi for MEASUREITEMSTRUCT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MENUBANDPARTS(pub i32);
pub const MDP_NEWAPPBUTTON: MENUBANDPARTS = MENUBANDPARTS(1i32);
pub const MDP_SEPERATOR: MENUBANDPARTS = MENUBANDPARTS(2i32);
impl ::std::convert::From<i32> for MENUBANDPARTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MENUBANDPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MENUBANDSTATES(pub i32);
pub const MDS_NORMAL: MENUBANDSTATES = MENUBANDSTATES(1i32);
pub const MDS_HOT: MENUBANDSTATES = MENUBANDSTATES(2i32);
pub const MDS_PRESSED: MENUBANDSTATES = MENUBANDSTATES(3i32);
pub const MDS_DISABLED: MENUBANDSTATES = MENUBANDSTATES(4i32);
pub const MDS_CHECKED: MENUBANDSTATES = MENUBANDSTATES(5i32);
pub const MDS_HOTCHECKED: MENUBANDSTATES = MENUBANDSTATES(6i32);
impl ::std::convert::From<i32> for MENUBANDSTATES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MENUBANDSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MONTHCALPARTS(pub i32);
pub const MC_BACKGROUND: MONTHCALPARTS = MONTHCALPARTS(1i32);
pub const MC_BORDERS: MONTHCALPARTS = MONTHCALPARTS(2i32);
pub const MC_GRIDBACKGROUND: MONTHCALPARTS = MONTHCALPARTS(3i32);
pub const MC_COLHEADERSPLITTER: MONTHCALPARTS = MONTHCALPARTS(4i32);
pub const MC_GRIDCELLBACKGROUND: MONTHCALPARTS = MONTHCALPARTS(5i32);
pub const MC_GRIDCELL: MONTHCALPARTS = MONTHCALPARTS(6i32);
pub const MC_GRIDCELLUPPER: MONTHCALPARTS = MONTHCALPARTS(7i32);
pub const MC_TRAILINGGRIDCELL: MONTHCALPARTS = MONTHCALPARTS(8i32);
pub const MC_TRAILINGGRIDCELLUPPER: MONTHCALPARTS = MONTHCALPARTS(9i32);
pub const MC_NAVNEXT: MONTHCALPARTS = MONTHCALPARTS(10i32);
pub const MC_NAVPREV: MONTHCALPARTS = MONTHCALPARTS(11i32);
impl ::std::convert::From<i32> for MONTHCALPARTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MONTHCALPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MOREPROGRAMSARROWBACKSTATES(pub i32);
pub const SPSB_NORMAL: MOREPROGRAMSARROWBACKSTATES = MOREPROGRAMSARROWBACKSTATES(1i32);
pub const SPSB_HOT: MOREPROGRAMSARROWBACKSTATES = MOREPROGRAMSARROWBACKSTATES(2i32);
pub const SPSB_PRESSED: MOREPROGRAMSARROWBACKSTATES = MOREPROGRAMSARROWBACKSTATES(3i32);
impl ::std::convert::From<i32> for MOREPROGRAMSARROWBACKSTATES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MOREPROGRAMSARROWBACKSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MOREPROGRAMSARROWSTATES(pub i32);
pub const SPS_NORMAL: MOREPROGRAMSARROWSTATES = MOREPROGRAMSARROWSTATES(1i32);
pub const SPS_HOT: MOREPROGRAMSARROWSTATES = MOREPROGRAMSARROWSTATES(2i32);
pub const SPS_PRESSED: MOREPROGRAMSARROWSTATES = MOREPROGRAMSARROWSTATES(3i32);
impl ::std::convert::From<i32> for MOREPROGRAMSARROWSTATES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MOREPROGRAMSARROWSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MOREPROGRAMSTABSTATES(pub i32);
pub const SPMPT_NORMAL: MOREPROGRAMSTABSTATES = MOREPROGRAMSTABSTATES(1i32);
pub const SPMPT_HOT: MOREPROGRAMSTABSTATES = MOREPROGRAMSTABSTATES(2i32);
pub const SPMPT_SELECTED: MOREPROGRAMSTABSTATES = MOREPROGRAMSTABSTATES(3i32);
pub const SPMPT_DISABLED: MOREPROGRAMSTABSTATES = MOREPROGRAMSTABSTATES(4i32);
pub const SPMPT_FOCUSED: MOREPROGRAMSTABSTATES = MOREPROGRAMSTABSTATES(5i32);
impl ::std::convert::From<i32> for MOREPROGRAMSTABSTATES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MOREPROGRAMSTABSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MSGF_COMMCTRL_BEGINDRAG: u32 = 16896u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MSGF_COMMCTRL_DRAGSELECT: u32 = 16898u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MSGF_COMMCTRL_SIZEHEADER: u32 = 16897u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MSGF_COMMCTRL_TOOLBARCUST: u32 = 16899u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const MULTIFILEOPENORD: u32 = 1537u32;
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MakeDragList<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hlb: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MakeDragList(hlb: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(MakeDragList(hlb.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn MenuHelp<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::WPARAM>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>, Param3: ::windows::runtime::IntoParam<'a, super::WindowsAndMessaging::HMENU>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(
    umsg: u32,
    wparam: Param1,
    lparam: Param2,
    hmainmenu: Param3,
    hinst: Param4,
    hwndstatus: Param5,
    lpwids: *const u32,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MenuHelp(umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, hmainmenu: super::WindowsAndMessaging::HMENU, hinst: super::super::Foundation::HINSTANCE, hwndstatus: super::super::Foundation::HWND, lpwids: *const u32);
        }
        ::std::mem::transmute(MenuHelp(::std::mem::transmute(umsg), wparam.into_param().abi(), lparam.into_param().abi(), hmainmenu.into_param().abi(), hinst.into_param().abi(), hwndstatus.into_param().abi(), ::std::mem::transmute(lpwids)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NAVNEXTSTATES(pub i32);
pub const MCNN_NORMAL: NAVNEXTSTATES = NAVNEXTSTATES(1i32);
pub const MCNN_HOT: NAVNEXTSTATES = NAVNEXTSTATES(2i32);
pub const MCNN_PRESSED: NAVNEXTSTATES = NAVNEXTSTATES(3i32);
pub const MCNN_DISABLED: NAVNEXTSTATES = NAVNEXTSTATES(4i32);
impl ::std::convert::From<i32> for NAVNEXTSTATES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NAVNEXTSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NAVPREVSTATES(pub i32);
pub const MCNP_NORMAL: NAVPREVSTATES = NAVPREVSTATES(1i32);
pub const MCNP_HOT: NAVPREVSTATES = NAVPREVSTATES(2i32);
pub const MCNP_PRESSED: NAVPREVSTATES = NAVPREVSTATES(3i32);
pub const MCNP_DISABLED: NAVPREVSTATES = NAVPREVSTATES(4i32);
impl ::std::convert::From<i32> for NAVPREVSTATES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NAVPREVSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const NEWFILEOPENORD: u32 = 1547u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const NEWFILEOPENV2ORD: u32 = 1552u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const NEWFILEOPENV3ORD: u32 = 1553u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const NEWFORMATDLGWITHLINK: u32 = 1591u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const NFS_ALL: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const NFS_BUTTON: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const NFS_EDIT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const NFS_LISTCOMBO: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const NFS_STATIC: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const NFS_USEFONTASSOC: u32 = 32u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMBCDROPDOWN {
    pub hdr: NMHDR,
    pub rcButton: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl NMBCDROPDOWN {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMBCDROPDOWN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMBCDROPDOWN {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMBCDROPDOWN").field("hdr", &self.hdr).field("rcButton", &self.rcButton).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMBCDROPDOWN {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.rcButton == other.rcButton
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMBCDROPDOWN {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMBCDROPDOWN {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMBCHOTITEM {
    pub hdr: NMHDR,
    pub dwFlags: NMTBHOTITEM_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl NMBCHOTITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMBCHOTITEM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMBCHOTITEM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMBCHOTITEM").field("hdr", &self.hdr).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMBCHOTITEM {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMBCHOTITEM {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMBCHOTITEM {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMCBEDRAGBEGINA {
    pub hdr: NMHDR,
    pub iItemid: i32,
    pub szText: [super::super::Foundation::CHAR; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl NMCBEDRAGBEGINA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMCBEDRAGBEGINA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMCBEDRAGBEGINA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMCBEDRAGBEGINA").field("hdr", &self.hdr).field("iItemid", &self.iItemid).field("szText", &self.szText).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMCBEDRAGBEGINA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iItemid == other.iItemid && self.szText == other.szText
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMCBEDRAGBEGINA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMCBEDRAGBEGINA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMCBEDRAGBEGINW {
    pub hdr: NMHDR,
    pub iItemid: i32,
    pub szText: [u16; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl NMCBEDRAGBEGINW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMCBEDRAGBEGINW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMCBEDRAGBEGINW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMCBEDRAGBEGINW").field("hdr", &self.hdr).field("iItemid", &self.iItemid).field("szText", &self.szText).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMCBEDRAGBEGINW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iItemid == other.iItemid && self.szText == other.szText
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMCBEDRAGBEGINW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMCBEDRAGBEGINW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMCBEENDEDITA {
    pub hdr: NMHDR,
    pub fChanged: super::super::Foundation::BOOL,
    pub iNewSelection: i32,
    pub szText: [super::super::Foundation::CHAR; 260],
    pub iWhy: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl NMCBEENDEDITA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMCBEENDEDITA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMCBEENDEDITA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMCBEENDEDITA").field("hdr", &self.hdr).field("fChanged", &self.fChanged).field("iNewSelection", &self.iNewSelection).field("szText", &self.szText).field("iWhy", &self.iWhy).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMCBEENDEDITA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.fChanged == other.fChanged && self.iNewSelection == other.iNewSelection && self.szText == other.szText && self.iWhy == other.iWhy
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMCBEENDEDITA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMCBEENDEDITA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMCBEENDEDITW {
    pub hdr: NMHDR,
    pub fChanged: super::super::Foundation::BOOL,
    pub iNewSelection: i32,
    pub szText: [u16; 260],
    pub iWhy: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl NMCBEENDEDITW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMCBEENDEDITW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMCBEENDEDITW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMCBEENDEDITW").field("hdr", &self.hdr).field("fChanged", &self.fChanged).field("iNewSelection", &self.iNewSelection).field("szText", &self.szText).field("iWhy", &self.iWhy).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMCBEENDEDITW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.fChanged == other.fChanged && self.iNewSelection == other.iNewSelection && self.szText == other.szText && self.iWhy == other.iWhy
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMCBEENDEDITW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMCBEENDEDITW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMCHAR {
    pub hdr: NMHDR,
    pub ch: u32,
    pub dwItemPrev: u32,
    pub dwItemNext: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl NMCHAR {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMCHAR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMCHAR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMCHAR").field("hdr", &self.hdr).field("ch", &self.ch).field("dwItemPrev", &self.dwItemPrev).field("dwItemNext", &self.dwItemNext).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMCHAR {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.ch == other.ch && self.dwItemPrev == other.dwItemPrev && self.dwItemNext == other.dwItemNext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMCHAR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMCHAR {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMCOMBOBOXEXA {
    pub hdr: NMHDR,
    pub ceItem: COMBOBOXEXITEMA,
}
#[cfg(feature = "Win32_Foundation")]
impl NMCOMBOBOXEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMCOMBOBOXEXA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMCOMBOBOXEXA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMCOMBOBOXEXA").field("hdr", &self.hdr).field("ceItem", &self.ceItem).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMCOMBOBOXEXA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.ceItem == other.ceItem
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMCOMBOBOXEXA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMCOMBOBOXEXA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMCOMBOBOXEXW {
    pub hdr: NMHDR,
    pub ceItem: COMBOBOXEXITEMW,
}
#[cfg(feature = "Win32_Foundation")]
impl NMCOMBOBOXEXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMCOMBOBOXEXW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMCOMBOBOXEXW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMCOMBOBOXEXW").field("hdr", &self.hdr).field("ceItem", &self.ceItem).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMCOMBOBOXEXW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.ceItem == other.ceItem
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMCOMBOBOXEXW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMCOMBOBOXEXW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct NMCUSTOMDRAW {
    pub hdr: NMHDR,
    pub dwDrawStage: NMCUSTOMDRAW_DRAW_STAGE,
    pub hdc: super::super::Graphics::Gdi::HDC,
    pub rc: super::super::Foundation::RECT,
    pub dwItemSpec: usize,
    pub uItemState: u32,
    pub lItemlParam: super::super::Foundation::LPARAM,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl NMCUSTOMDRAW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for NMCUSTOMDRAW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for NMCUSTOMDRAW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMCUSTOMDRAW").field("hdr", &self.hdr).field("dwDrawStage", &self.dwDrawStage).field("hdc", &self.hdc).field("rc", &self.rc).field("dwItemSpec", &self.dwItemSpec).field("uItemState", &self.uItemState).field("lItemlParam", &self.lItemlParam).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for NMCUSTOMDRAW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwDrawStage == other.dwDrawStage && self.hdc == other.hdc && self.rc == other.rc && self.dwItemSpec == other.dwItemSpec && self.uItemState == other.uItemState && self.lItemlParam == other.lItemlParam
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for NMCUSTOMDRAW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for NMCUSTOMDRAW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NMCUSTOMDRAW_DRAW_STAGE(pub u32);
pub const CDDS_POSTPAINT: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(2u32);
pub const CDDS_PREERASE: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(3u32);
pub const CDDS_PREPAINT: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(1u32);
pub const CDDS_ITEMPOSTERASE: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(65540u32);
pub const CDDS_ITEMPOSTPAINT: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(65538u32);
pub const CDDS_ITEMPREERASE: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(65539u32);
pub const CDDS_ITEMPREPAINT: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(65537u32);
pub const CDDS_SUBITEM: NMCUSTOMDRAW_DRAW_STAGE = NMCUSTOMDRAW_DRAW_STAGE(131072u32);
impl ::std::convert::From<u32> for NMCUSTOMDRAW_DRAW_STAGE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NMCUSTOMDRAW_DRAW_STAGE {
    type Abi = Self;
}
impl ::std::ops::BitOr for NMCUSTOMDRAW_DRAW_STAGE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for NMCUSTOMDRAW_DRAW_STAGE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for NMCUSTOMDRAW_DRAW_STAGE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for NMCUSTOMDRAW_DRAW_STAGE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for NMCUSTOMDRAW_DRAW_STAGE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMCUSTOMSPLITRECTINFO {
    pub hdr: NMHDR,
    pub rcClient: super::super::Foundation::RECT,
    pub rcButton: super::super::Foundation::RECT,
    pub rcSplit: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl NMCUSTOMSPLITRECTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMCUSTOMSPLITRECTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMCUSTOMSPLITRECTINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMCUSTOMSPLITRECTINFO").field("hdr", &self.hdr).field("rcClient", &self.rcClient).field("rcButton", &self.rcButton).field("rcSplit", &self.rcSplit).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMCUSTOMSPLITRECTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.rcClient == other.rcClient && self.rcButton == other.rcButton && self.rcSplit == other.rcSplit
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMCUSTOMSPLITRECTINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMCUSTOMSPLITRECTINFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct NMCUSTOMTEXT {
    pub hdr: NMHDR,
    pub hDC: super::super::Graphics::Gdi::HDC,
    pub lpString: super::super::Foundation::PWSTR,
    pub nCount: i32,
    pub lpRect: *mut super::super::Foundation::RECT,
    pub uFormat: u32,
    pub fLink: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl NMCUSTOMTEXT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for NMCUSTOMTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for NMCUSTOMTEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMCUSTOMTEXT").field("hdr", &self.hdr).field("hDC", &self.hDC).field("lpString", &self.lpString).field("nCount", &self.nCount).field("lpRect", &self.lpRect).field("uFormat", &self.uFormat).field("fLink", &self.fLink).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for NMCUSTOMTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.hDC == other.hDC && self.lpString == other.lpString && self.nCount == other.nCount && self.lpRect == other.lpRect && self.uFormat == other.uFormat && self.fLink == other.fLink
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for NMCUSTOMTEXT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for NMCUSTOMTEXT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMDATETIMECHANGE {
    pub nmhdr: NMHDR,
    pub dwFlags: u32,
    pub st: super::super::Foundation::SYSTEMTIME,
}
#[cfg(feature = "Win32_Foundation")]
impl NMDATETIMECHANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMDATETIMECHANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMDATETIMECHANGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMDATETIMECHANGE").field("nmhdr", &self.nmhdr).field("dwFlags", &self.dwFlags).field("st", &self.st).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMDATETIMECHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.dwFlags == other.dwFlags && self.st == other.st
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMDATETIMECHANGE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMDATETIMECHANGE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMDATETIMEFORMATA {
    pub nmhdr: NMHDR,
    pub pszFormat: super::super::Foundation::PSTR,
    pub st: super::super::Foundation::SYSTEMTIME,
    pub pszDisplay: super::super::Foundation::PSTR,
    pub szDisplay: [super::super::Foundation::CHAR; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl NMDATETIMEFORMATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMDATETIMEFORMATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMDATETIMEFORMATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMDATETIMEFORMATA").field("nmhdr", &self.nmhdr).field("pszFormat", &self.pszFormat).field("st", &self.st).field("pszDisplay", &self.pszDisplay).field("szDisplay", &self.szDisplay).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMDATETIMEFORMATA {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.pszFormat == other.pszFormat && self.st == other.st && self.pszDisplay == other.pszDisplay && self.szDisplay == other.szDisplay
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMDATETIMEFORMATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMDATETIMEFORMATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMDATETIMEFORMATQUERYA {
    pub nmhdr: NMHDR,
    pub pszFormat: super::super::Foundation::PSTR,
    pub szMax: super::super::Foundation::SIZE,
}
#[cfg(feature = "Win32_Foundation")]
impl NMDATETIMEFORMATQUERYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMDATETIMEFORMATQUERYA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMDATETIMEFORMATQUERYA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMDATETIMEFORMATQUERYA").field("nmhdr", &self.nmhdr).field("pszFormat", &self.pszFormat).field("szMax", &self.szMax).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMDATETIMEFORMATQUERYA {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.pszFormat == other.pszFormat && self.szMax == other.szMax
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMDATETIMEFORMATQUERYA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMDATETIMEFORMATQUERYA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMDATETIMEFORMATQUERYW {
    pub nmhdr: NMHDR,
    pub pszFormat: super::super::Foundation::PWSTR,
    pub szMax: super::super::Foundation::SIZE,
}
#[cfg(feature = "Win32_Foundation")]
impl NMDATETIMEFORMATQUERYW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMDATETIMEFORMATQUERYW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMDATETIMEFORMATQUERYW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMDATETIMEFORMATQUERYW").field("nmhdr", &self.nmhdr).field("pszFormat", &self.pszFormat).field("szMax", &self.szMax).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMDATETIMEFORMATQUERYW {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.pszFormat == other.pszFormat && self.szMax == other.szMax
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMDATETIMEFORMATQUERYW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMDATETIMEFORMATQUERYW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMDATETIMEFORMATW {
    pub nmhdr: NMHDR,
    pub pszFormat: super::super::Foundation::PWSTR,
    pub st: super::super::Foundation::SYSTEMTIME,
    pub pszDisplay: super::super::Foundation::PWSTR,
    pub szDisplay: [u16; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl NMDATETIMEFORMATW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMDATETIMEFORMATW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMDATETIMEFORMATW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMDATETIMEFORMATW").field("nmhdr", &self.nmhdr).field("pszFormat", &self.pszFormat).field("st", &self.st).field("pszDisplay", &self.pszDisplay).field("szDisplay", &self.szDisplay).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMDATETIMEFORMATW {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.pszFormat == other.pszFormat && self.st == other.st && self.pszDisplay == other.pszDisplay && self.szDisplay == other.szDisplay
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMDATETIMEFORMATW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMDATETIMEFORMATW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMDATETIMESTRINGA {
    pub nmhdr: NMHDR,
    pub pszUserString: super::super::Foundation::PSTR,
    pub st: super::super::Foundation::SYSTEMTIME,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl NMDATETIMESTRINGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMDATETIMESTRINGA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMDATETIMESTRINGA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMDATETIMESTRINGA").field("nmhdr", &self.nmhdr).field("pszUserString", &self.pszUserString).field("st", &self.st).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMDATETIMESTRINGA {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.pszUserString == other.pszUserString && self.st == other.st && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMDATETIMESTRINGA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMDATETIMESTRINGA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMDATETIMESTRINGW {
    pub nmhdr: NMHDR,
    pub pszUserString: super::super::Foundation::PWSTR,
    pub st: super::super::Foundation::SYSTEMTIME,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl NMDATETIMESTRINGW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMDATETIMESTRINGW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMDATETIMESTRINGW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMDATETIMESTRINGW").field("nmhdr", &self.nmhdr).field("pszUserString", &self.pszUserString).field("st", &self.st).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMDATETIMESTRINGW {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.pszUserString == other.pszUserString && self.st == other.st && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMDATETIMESTRINGW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMDATETIMESTRINGW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMDATETIMEWMKEYDOWNA {
    pub nmhdr: NMHDR,
    pub nVirtKey: i32,
    pub pszFormat: super::super::Foundation::PSTR,
    pub st: super::super::Foundation::SYSTEMTIME,
}
#[cfg(feature = "Win32_Foundation")]
impl NMDATETIMEWMKEYDOWNA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMDATETIMEWMKEYDOWNA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMDATETIMEWMKEYDOWNA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMDATETIMEWMKEYDOWNA").field("nmhdr", &self.nmhdr).field("nVirtKey", &self.nVirtKey).field("pszFormat", &self.pszFormat).field("st", &self.st).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMDATETIMEWMKEYDOWNA {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.nVirtKey == other.nVirtKey && self.pszFormat == other.pszFormat && self.st == other.st
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMDATETIMEWMKEYDOWNA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMDATETIMEWMKEYDOWNA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMDATETIMEWMKEYDOWNW {
    pub nmhdr: NMHDR,
    pub nVirtKey: i32,
    pub pszFormat: super::super::Foundation::PWSTR,
    pub st: super::super::Foundation::SYSTEMTIME,
}
#[cfg(feature = "Win32_Foundation")]
impl NMDATETIMEWMKEYDOWNW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMDATETIMEWMKEYDOWNW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMDATETIMEWMKEYDOWNW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMDATETIMEWMKEYDOWNW").field("nmhdr", &self.nmhdr).field("nVirtKey", &self.nVirtKey).field("pszFormat", &self.pszFormat).field("st", &self.st).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMDATETIMEWMKEYDOWNW {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.nVirtKey == other.nVirtKey && self.pszFormat == other.pszFormat && self.st == other.st
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMDATETIMEWMKEYDOWNW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMDATETIMEWMKEYDOWNW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMDAYSTATE {
    pub nmhdr: NMHDR,
    pub stStart: super::super::Foundation::SYSTEMTIME,
    pub cDayState: i32,
    pub prgDayState: *mut u32,
}
#[cfg(feature = "Win32_Foundation")]
impl NMDAYSTATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMDAYSTATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMDAYSTATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMDAYSTATE").field("nmhdr", &self.nmhdr).field("stStart", &self.stStart).field("cDayState", &self.cDayState).field("prgDayState", &self.prgDayState).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMDAYSTATE {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.stStart == other.stStart && self.cDayState == other.cDayState && self.prgDayState == other.prgDayState
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMDAYSTATE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMDAYSTATE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMHDDISPINFOA {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub mask: HDI_MASK,
    pub pszText: super::super::Foundation::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl NMHDDISPINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMHDDISPINFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMHDDISPINFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMHDDISPINFOA").field("hdr", &self.hdr).field("iItem", &self.iItem).field("mask", &self.mask).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iImage", &self.iImage).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMHDDISPINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iItem == other.iItem && self.mask == other.mask && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iImage == other.iImage && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMHDDISPINFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMHDDISPINFOA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMHDDISPINFOW {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub mask: HDI_MASK,
    pub pszText: super::super::Foundation::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl NMHDDISPINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMHDDISPINFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMHDDISPINFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMHDDISPINFOW").field("hdr", &self.hdr).field("iItem", &self.iItem).field("mask", &self.mask).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iImage", &self.iImage).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMHDDISPINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iItem == other.iItem && self.mask == other.mask && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iImage == other.iImage && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMHDDISPINFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMHDDISPINFOW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMHDFILTERBTNCLICK {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub rc: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl NMHDFILTERBTNCLICK {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMHDFILTERBTNCLICK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMHDFILTERBTNCLICK {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMHDFILTERBTNCLICK").field("hdr", &self.hdr).field("iItem", &self.iItem).field("rc", &self.rc).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMHDFILTERBTNCLICK {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iItem == other.iItem && self.rc == other.rc
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMHDFILTERBTNCLICK {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMHDFILTERBTNCLICK {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMHDR {
    pub hwndFrom: super::super::Foundation::HWND,
    pub idFrom: usize,
    pub code: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl NMHDR {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMHDR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMHDR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMHDR").field("hwndFrom", &self.hwndFrom).field("idFrom", &self.idFrom).field("code", &self.code).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMHDR {
    fn eq(&self, other: &Self) -> bool {
        self.hwndFrom == other.hwndFrom && self.idFrom == other.idFrom && self.code == other.code
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMHDR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMHDR {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct NMHEADERA {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub iButton: HEADER_CONTROL_NOTIFICATION_BUTTON,
    pub pitem: *mut HDITEMA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl NMHEADERA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for NMHEADERA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for NMHEADERA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMHEADERA").field("hdr", &self.hdr).field("iItem", &self.iItem).field("iButton", &self.iButton).field("pitem", &self.pitem).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for NMHEADERA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iItem == other.iItem && self.iButton == other.iButton && self.pitem == other.pitem
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for NMHEADERA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for NMHEADERA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct NMHEADERW {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub iButton: HEADER_CONTROL_NOTIFICATION_BUTTON,
    pub pitem: *mut HDITEMW,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl NMHEADERW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for NMHEADERW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for NMHEADERW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMHEADERW").field("hdr", &self.hdr).field("iItem", &self.iItem).field("iButton", &self.iButton).field("pitem", &self.pitem).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for NMHEADERW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iItem == other.iItem && self.iButton == other.iButton && self.pitem == other.pitem
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for NMHEADERW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for NMHEADERW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMIPADDRESS {
    pub hdr: NMHDR,
    pub iField: i32,
    pub iValue: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl NMIPADDRESS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMIPADDRESS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMIPADDRESS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMIPADDRESS").field("hdr", &self.hdr).field("iField", &self.iField).field("iValue", &self.iValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMIPADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iField == other.iField && self.iValue == other.iValue
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMIPADDRESS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMIPADDRESS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMITEMACTIVATE {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub iSubItem: i32,
    pub uNewState: u32,
    pub uOldState: u32,
    pub uChanged: u32,
    pub ptAction: super::super::Foundation::POINT,
    pub lParam: super::super::Foundation::LPARAM,
    pub uKeyFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl NMITEMACTIVATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMITEMACTIVATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMITEMACTIVATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMITEMACTIVATE")
            .field("hdr", &self.hdr)
            .field("iItem", &self.iItem)
            .field("iSubItem", &self.iSubItem)
            .field("uNewState", &self.uNewState)
            .field("uOldState", &self.uOldState)
            .field("uChanged", &self.uChanged)
            .field("ptAction", &self.ptAction)
            .field("lParam", &self.lParam)
            .field("uKeyFlags", &self.uKeyFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMITEMACTIVATE {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iItem == other.iItem && self.iSubItem == other.iSubItem && self.uNewState == other.uNewState && self.uOldState == other.uOldState && self.uChanged == other.uChanged && self.ptAction == other.ptAction && self.lParam == other.lParam && self.uKeyFlags == other.uKeyFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMITEMACTIVATE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMITEMACTIVATE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMKEY {
    pub hdr: NMHDR,
    pub nVKey: u32,
    pub uFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl NMKEY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMKEY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMKEY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMKEY").field("hdr", &self.hdr).field("nVKey", &self.nVKey).field("uFlags", &self.uFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMKEY {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.nVKey == other.nVKey && self.uFlags == other.uFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMKEY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMKEY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMLINK {
    pub hdr: NMHDR,
    pub item: LITEM,
}
#[cfg(feature = "Win32_Foundation")]
impl NMLINK {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMLINK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMLINK {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMLINK").field("hdr", &self.hdr).field("item", &self.item).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMLINK {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.item == other.item
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMLINK {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMLINK {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMLISTVIEW {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub iSubItem: i32,
    pub uNewState: u32,
    pub uOldState: u32,
    pub uChanged: u32,
    pub ptAction: super::super::Foundation::POINT,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl NMLISTVIEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMLISTVIEW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMLISTVIEW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMLISTVIEW").field("hdr", &self.hdr).field("iItem", &self.iItem).field("iSubItem", &self.iSubItem).field("uNewState", &self.uNewState).field("uOldState", &self.uOldState).field("uChanged", &self.uChanged).field("ptAction", &self.ptAction).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMLISTVIEW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iItem == other.iItem && self.iSubItem == other.iSubItem && self.uNewState == other.uNewState && self.uOldState == other.uOldState && self.uChanged == other.uChanged && self.ptAction == other.ptAction && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMLISTVIEW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMLISTVIEW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMLVCACHEHINT {
    pub hdr: NMHDR,
    pub iFrom: i32,
    pub iTo: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl NMLVCACHEHINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMLVCACHEHINT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMLVCACHEHINT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMLVCACHEHINT").field("hdr", &self.hdr).field("iFrom", &self.iFrom).field("iTo", &self.iTo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMLVCACHEHINT {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iFrom == other.iFrom && self.iTo == other.iTo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMLVCACHEHINT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMLVCACHEHINT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct NMLVCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub clrText: u32,
    pub clrTextBk: u32,
    pub iSubItem: i32,
    pub dwItemType: NMLVCUSTOMDRAW_ITEM_TYPE,
    pub clrFace: u32,
    pub iIconEffect: i32,
    pub iIconPhase: i32,
    pub iPartId: i32,
    pub iStateId: i32,
    pub rcText: super::super::Foundation::RECT,
    pub uAlign: NMLVCUSTOMDRAW_ALIGN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl NMLVCUSTOMDRAW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for NMLVCUSTOMDRAW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for NMLVCUSTOMDRAW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMLVCUSTOMDRAW")
            .field("nmcd", &self.nmcd)
            .field("clrText", &self.clrText)
            .field("clrTextBk", &self.clrTextBk)
            .field("iSubItem", &self.iSubItem)
            .field("dwItemType", &self.dwItemType)
            .field("clrFace", &self.clrFace)
            .field("iIconEffect", &self.iIconEffect)
            .field("iIconPhase", &self.iIconPhase)
            .field("iPartId", &self.iPartId)
            .field("iStateId", &self.iStateId)
            .field("rcText", &self.rcText)
            .field("uAlign", &self.uAlign)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for NMLVCUSTOMDRAW {
    fn eq(&self, other: &Self) -> bool {
        self.nmcd == other.nmcd && self.clrText == other.clrText && self.clrTextBk == other.clrTextBk && self.iSubItem == other.iSubItem && self.dwItemType == other.dwItemType && self.clrFace == other.clrFace && self.iIconEffect == other.iIconEffect && self.iIconPhase == other.iIconPhase && self.iPartId == other.iPartId && self.iStateId == other.iStateId && self.rcText == other.rcText && self.uAlign == other.uAlign
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for NMLVCUSTOMDRAW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for NMLVCUSTOMDRAW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NMLVCUSTOMDRAW_ALIGN(pub u32);
pub const LVGA_HEADER_CENTER: NMLVCUSTOMDRAW_ALIGN = NMLVCUSTOMDRAW_ALIGN(2u32);
pub const LVGA_HEADER_LEFT: NMLVCUSTOMDRAW_ALIGN = NMLVCUSTOMDRAW_ALIGN(1u32);
pub const LVGA_HEADER_RIGHT: NMLVCUSTOMDRAW_ALIGN = NMLVCUSTOMDRAW_ALIGN(4u32);
impl ::std::convert::From<u32> for NMLVCUSTOMDRAW_ALIGN {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NMLVCUSTOMDRAW_ALIGN {
    type Abi = Self;
}
impl ::std::ops::BitOr for NMLVCUSTOMDRAW_ALIGN {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for NMLVCUSTOMDRAW_ALIGN {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for NMLVCUSTOMDRAW_ALIGN {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for NMLVCUSTOMDRAW_ALIGN {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for NMLVCUSTOMDRAW_ALIGN {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NMLVCUSTOMDRAW_ITEM_TYPE(pub u32);
pub const LVCDI_ITEM: NMLVCUSTOMDRAW_ITEM_TYPE = NMLVCUSTOMDRAW_ITEM_TYPE(0u32);
pub const LVCDI_GROUP: NMLVCUSTOMDRAW_ITEM_TYPE = NMLVCUSTOMDRAW_ITEM_TYPE(1u32);
pub const LVCDI_ITEMSLIST: NMLVCUSTOMDRAW_ITEM_TYPE = NMLVCUSTOMDRAW_ITEM_TYPE(2u32);
impl ::std::convert::From<u32> for NMLVCUSTOMDRAW_ITEM_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NMLVCUSTOMDRAW_ITEM_TYPE {
    type Abi = Self;
}
impl ::std::ops::BitOr for NMLVCUSTOMDRAW_ITEM_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for NMLVCUSTOMDRAW_ITEM_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for NMLVCUSTOMDRAW_ITEM_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for NMLVCUSTOMDRAW_ITEM_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for NMLVCUSTOMDRAW_ITEM_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMLVDISPINFOA {
    pub hdr: NMHDR,
    pub item: LVITEMA,
}
#[cfg(feature = "Win32_Foundation")]
impl NMLVDISPINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMLVDISPINFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMLVDISPINFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMLVDISPINFOA").field("hdr", &self.hdr).field("item", &self.item).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMLVDISPINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.item == other.item
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMLVDISPINFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMLVDISPINFOA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMLVDISPINFOW {
    pub hdr: NMHDR,
    pub item: LVITEMW,
}
#[cfg(feature = "Win32_Foundation")]
impl NMLVDISPINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMLVDISPINFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMLVDISPINFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMLVDISPINFOW").field("hdr", &self.hdr).field("item", &self.item).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMLVDISPINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.item == other.item
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMLVDISPINFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMLVDISPINFOW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMLVEMPTYMARKUP {
    pub hdr: NMHDR,
    pub dwFlags: NMLVEMPTYMARKUP_FLAGS,
    pub szMarkup: [u16; 2084],
}
#[cfg(feature = "Win32_Foundation")]
impl NMLVEMPTYMARKUP {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMLVEMPTYMARKUP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMLVEMPTYMARKUP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMLVEMPTYMARKUP").field("hdr", &self.hdr).field("dwFlags", &self.dwFlags).field("szMarkup", &self.szMarkup).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMLVEMPTYMARKUP {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwFlags == other.dwFlags && self.szMarkup == other.szMarkup
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMLVEMPTYMARKUP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMLVEMPTYMARKUP {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NMLVEMPTYMARKUP_FLAGS(pub u32);
pub const EMF_CENTERED: NMLVEMPTYMARKUP_FLAGS = NMLVEMPTYMARKUP_FLAGS(1u32);
impl ::std::convert::From<u32> for NMLVEMPTYMARKUP_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NMLVEMPTYMARKUP_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for NMLVEMPTYMARKUP_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for NMLVEMPTYMARKUP_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for NMLVEMPTYMARKUP_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for NMLVEMPTYMARKUP_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for NMLVEMPTYMARKUP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMLVFINDITEMA {
    pub hdr: NMHDR,
    pub iStart: i32,
    pub lvfi: LVFINDINFOA,
}
#[cfg(feature = "Win32_Foundation")]
impl NMLVFINDITEMA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMLVFINDITEMA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMLVFINDITEMA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMLVFINDITEMA").field("hdr", &self.hdr).field("iStart", &self.iStart).field("lvfi", &self.lvfi).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMLVFINDITEMA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iStart == other.iStart && self.lvfi == other.lvfi
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMLVFINDITEMA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMLVFINDITEMA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMLVFINDITEMW {
    pub hdr: NMHDR,
    pub iStart: i32,
    pub lvfi: LVFINDINFOW,
}
#[cfg(feature = "Win32_Foundation")]
impl NMLVFINDITEMW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMLVFINDITEMW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMLVFINDITEMW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMLVFINDITEMW").field("hdr", &self.hdr).field("iStart", &self.iStart).field("lvfi", &self.lvfi).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMLVFINDITEMW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iStart == other.iStart && self.lvfi == other.lvfi
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMLVFINDITEMW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMLVFINDITEMW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMLVGETINFOTIPA {
    pub hdr: NMHDR,
    pub dwFlags: u32,
    pub pszText: super::super::Foundation::PSTR,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl NMLVGETINFOTIPA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMLVGETINFOTIPA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMLVGETINFOTIPA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMLVGETINFOTIPA").field("hdr", &self.hdr).field("dwFlags", &self.dwFlags).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iItem", &self.iItem).field("iSubItem", &self.iSubItem).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMLVGETINFOTIPA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwFlags == other.dwFlags && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iItem == other.iItem && self.iSubItem == other.iSubItem && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMLVGETINFOTIPA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMLVGETINFOTIPA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMLVGETINFOTIPW {
    pub hdr: NMHDR,
    pub dwFlags: u32,
    pub pszText: super::super::Foundation::PWSTR,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub iSubItem: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl NMLVGETINFOTIPW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMLVGETINFOTIPW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMLVGETINFOTIPW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMLVGETINFOTIPW").field("hdr", &self.hdr).field("dwFlags", &self.dwFlags).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iItem", &self.iItem).field("iSubItem", &self.iSubItem).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMLVGETINFOTIPW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwFlags == other.dwFlags && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iItem == other.iItem && self.iSubItem == other.iSubItem && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMLVGETINFOTIPW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMLVGETINFOTIPW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMLVKEYDOWN {
    pub hdr: NMHDR,
    pub wVKey: u16,
    pub flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl NMLVKEYDOWN {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMLVKEYDOWN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMLVKEYDOWN {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMLVKEYDOWN {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMLVKEYDOWN {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMLVLINK {
    pub hdr: NMHDR,
    pub link: LITEM,
    pub iItem: i32,
    pub iSubItem: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl NMLVLINK {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMLVLINK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMLVLINK {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMLVLINK").field("hdr", &self.hdr).field("link", &self.link).field("iItem", &self.iItem).field("iSubItem", &self.iSubItem).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMLVLINK {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.link == other.link && self.iItem == other.iItem && self.iSubItem == other.iSubItem
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMLVLINK {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMLVLINK {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMLVODSTATECHANGE {
    pub hdr: NMHDR,
    pub iFrom: i32,
    pub iTo: i32,
    pub uNewState: u32,
    pub uOldState: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl NMLVODSTATECHANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMLVODSTATECHANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMLVODSTATECHANGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMLVODSTATECHANGE").field("hdr", &self.hdr).field("iFrom", &self.iFrom).field("iTo", &self.iTo).field("uNewState", &self.uNewState).field("uOldState", &self.uOldState).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMLVODSTATECHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iFrom == other.iFrom && self.iTo == other.iTo && self.uNewState == other.uNewState && self.uOldState == other.uOldState
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMLVODSTATECHANGE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMLVODSTATECHANGE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMLVSCROLL {
    pub hdr: NMHDR,
    pub dx: i32,
    pub dy: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl NMLVSCROLL {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMLVSCROLL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMLVSCROLL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMLVSCROLL").field("hdr", &self.hdr).field("dx", &self.dx).field("dy", &self.dy).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMLVSCROLL {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dx == other.dx && self.dy == other.dy
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMLVSCROLL {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMLVSCROLL {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMMOUSE {
    pub hdr: NMHDR,
    pub dwItemSpec: usize,
    pub dwItemData: usize,
    pub pt: super::super::Foundation::POINT,
    pub dwHitInfo: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl NMMOUSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMMOUSE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMMOUSE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMMOUSE").field("hdr", &self.hdr).field("dwItemSpec", &self.dwItemSpec).field("dwItemData", &self.dwItemData).field("pt", &self.pt).field("dwHitInfo", &self.dwHitInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMMOUSE {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwItemSpec == other.dwItemSpec && self.dwItemData == other.dwItemData && self.pt == other.pt && self.dwHitInfo == other.dwHitInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMMOUSE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMMOUSE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMOBJECTNOTIFY {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub piid: *mut ::windows::runtime::GUID,
    pub pObject: *mut ::std::ffi::c_void,
    pub hResult: ::windows::runtime::HRESULT,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl NMOBJECTNOTIFY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMOBJECTNOTIFY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMOBJECTNOTIFY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMOBJECTNOTIFY").field("hdr", &self.hdr).field("iItem", &self.iItem).field("piid", &self.piid).field("pObject", &self.pObject).field("hResult", &self.hResult).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMOBJECTNOTIFY {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iItem == other.iItem && self.piid == other.piid && self.pObject == other.pObject && self.hResult == other.hResult && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMOBJECTNOTIFY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMOBJECTNOTIFY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMPGCALCSIZE {
    pub hdr: NMHDR,
    pub dwFlag: NMPGCALCSIZE_FLAGS,
    pub iWidth: i32,
    pub iHeight: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl NMPGCALCSIZE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMPGCALCSIZE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMPGCALCSIZE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMPGCALCSIZE").field("hdr", &self.hdr).field("dwFlag", &self.dwFlag).field("iWidth", &self.iWidth).field("iHeight", &self.iHeight).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMPGCALCSIZE {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwFlag == other.dwFlag && self.iWidth == other.iWidth && self.iHeight == other.iHeight
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMPGCALCSIZE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMPGCALCSIZE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NMPGCALCSIZE_FLAGS(pub u32);
pub const PGF_CALCHEIGHT: NMPGCALCSIZE_FLAGS = NMPGCALCSIZE_FLAGS(2u32);
pub const PGF_CALCWIDTH: NMPGCALCSIZE_FLAGS = NMPGCALCSIZE_FLAGS(1u32);
impl ::std::convert::From<u32> for NMPGCALCSIZE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NMPGCALCSIZE_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for NMPGCALCSIZE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for NMPGCALCSIZE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for NMPGCALCSIZE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for NMPGCALCSIZE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for NMPGCALCSIZE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMPGHOTITEM {
    pub hdr: NMHDR,
    pub idOld: i32,
    pub idNew: i32,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl NMPGHOTITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMPGHOTITEM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMPGHOTITEM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMPGHOTITEM").field("hdr", &self.hdr).field("idOld", &self.idOld).field("idNew", &self.idNew).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMPGHOTITEM {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.idOld == other.idOld && self.idNew == other.idNew && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMPGHOTITEM {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMPGHOTITEM {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMPGSCROLL {
    pub hdr: NMHDR,
    pub fwKeys: NMPGSCROLL_KEYS,
    pub rcParent: super::super::Foundation::RECT,
    pub iDir: NMPGSCROLL_DIR,
    pub iXpos: i32,
    pub iYpos: i32,
    pub iScroll: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl NMPGSCROLL {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMPGSCROLL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMPGSCROLL {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMPGSCROLL {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMPGSCROLL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NMPGSCROLL_DIR(pub u32);
pub const PGF_SCROLLDOWN: NMPGSCROLL_DIR = NMPGSCROLL_DIR(2u32);
pub const PGF_SCROLLLEFT: NMPGSCROLL_DIR = NMPGSCROLL_DIR(4u32);
pub const PGF_SCROLLRIGHT: NMPGSCROLL_DIR = NMPGSCROLL_DIR(8u32);
pub const PGF_SCROLLUP: NMPGSCROLL_DIR = NMPGSCROLL_DIR(1u32);
impl ::std::convert::From<u32> for NMPGSCROLL_DIR {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NMPGSCROLL_DIR {
    type Abi = Self;
}
impl ::std::ops::BitOr for NMPGSCROLL_DIR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for NMPGSCROLL_DIR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for NMPGSCROLL_DIR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for NMPGSCROLL_DIR {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for NMPGSCROLL_DIR {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NMPGSCROLL_KEYS(pub u16);
pub const PGK_NONE: NMPGSCROLL_KEYS = NMPGSCROLL_KEYS(0u16);
pub const PGK_SHIFT: NMPGSCROLL_KEYS = NMPGSCROLL_KEYS(1u16);
pub const PGK_CONTROL: NMPGSCROLL_KEYS = NMPGSCROLL_KEYS(2u16);
pub const PGK_MENU: NMPGSCROLL_KEYS = NMPGSCROLL_KEYS(4u16);
impl ::std::convert::From<u16> for NMPGSCROLL_KEYS {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NMPGSCROLL_KEYS {
    type Abi = Self;
}
impl ::std::ops::BitOr for NMPGSCROLL_KEYS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for NMPGSCROLL_KEYS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for NMPGSCROLL_KEYS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for NMPGSCROLL_KEYS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for NMPGSCROLL_KEYS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMRBAUTOSIZE {
    pub hdr: NMHDR,
    pub fChanged: super::super::Foundation::BOOL,
    pub rcTarget: super::super::Foundation::RECT,
    pub rcActual: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl NMRBAUTOSIZE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMRBAUTOSIZE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMRBAUTOSIZE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMRBAUTOSIZE").field("hdr", &self.hdr).field("fChanged", &self.fChanged).field("rcTarget", &self.rcTarget).field("rcActual", &self.rcActual).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMRBAUTOSIZE {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.fChanged == other.fChanged && self.rcTarget == other.rcTarget && self.rcActual == other.rcActual
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMRBAUTOSIZE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMRBAUTOSIZE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMREBAR {
    pub hdr: NMHDR,
    pub dwMask: NMREBAR_MASK_FLAGS,
    pub uBand: u32,
    pub fStyle: u32,
    pub wID: u32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl NMREBAR {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMREBAR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMREBAR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMREBAR").field("hdr", &self.hdr).field("dwMask", &self.dwMask).field("uBand", &self.uBand).field("fStyle", &self.fStyle).field("wID", &self.wID).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMREBAR {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwMask == other.dwMask && self.uBand == other.uBand && self.fStyle == other.fStyle && self.wID == other.wID && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMREBAR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMREBAR {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMREBARAUTOBREAK {
    pub hdr: NMHDR,
    pub uBand: u32,
    pub wID: u32,
    pub lParam: super::super::Foundation::LPARAM,
    pub uMsg: u32,
    pub fStyleCurrent: u32,
    pub fAutoBreak: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl NMREBARAUTOBREAK {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMREBARAUTOBREAK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMREBARAUTOBREAK {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMREBARAUTOBREAK").field("hdr", &self.hdr).field("uBand", &self.uBand).field("wID", &self.wID).field("lParam", &self.lParam).field("uMsg", &self.uMsg).field("fStyleCurrent", &self.fStyleCurrent).field("fAutoBreak", &self.fAutoBreak).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMREBARAUTOBREAK {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.uBand == other.uBand && self.wID == other.wID && self.lParam == other.lParam && self.uMsg == other.uMsg && self.fStyleCurrent == other.fStyleCurrent && self.fAutoBreak == other.fAutoBreak
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMREBARAUTOBREAK {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMREBARAUTOBREAK {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMREBARCHEVRON {
    pub hdr: NMHDR,
    pub uBand: u32,
    pub wID: u32,
    pub lParam: super::super::Foundation::LPARAM,
    pub rc: super::super::Foundation::RECT,
    pub lParamNM: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl NMREBARCHEVRON {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMREBARCHEVRON {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMREBARCHEVRON {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMREBARCHEVRON").field("hdr", &self.hdr).field("uBand", &self.uBand).field("wID", &self.wID).field("lParam", &self.lParam).field("rc", &self.rc).field("lParamNM", &self.lParamNM).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMREBARCHEVRON {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.uBand == other.uBand && self.wID == other.wID && self.lParam == other.lParam && self.rc == other.rc && self.lParamNM == other.lParamNM
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMREBARCHEVRON {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMREBARCHEVRON {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMREBARCHILDSIZE {
    pub hdr: NMHDR,
    pub uBand: u32,
    pub wID: u32,
    pub rcChild: super::super::Foundation::RECT,
    pub rcBand: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl NMREBARCHILDSIZE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMREBARCHILDSIZE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMREBARCHILDSIZE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMREBARCHILDSIZE").field("hdr", &self.hdr).field("uBand", &self.uBand).field("wID", &self.wID).field("rcChild", &self.rcChild).field("rcBand", &self.rcBand).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMREBARCHILDSIZE {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.uBand == other.uBand && self.wID == other.wID && self.rcChild == other.rcChild && self.rcBand == other.rcBand
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMREBARCHILDSIZE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMREBARCHILDSIZE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMREBARSPLITTER {
    pub hdr: NMHDR,
    pub rcSizing: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl NMREBARSPLITTER {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMREBARSPLITTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMREBARSPLITTER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMREBARSPLITTER").field("hdr", &self.hdr).field("rcSizing", &self.rcSizing).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMREBARSPLITTER {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.rcSizing == other.rcSizing
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMREBARSPLITTER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMREBARSPLITTER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NMREBAR_MASK_FLAGS(pub u32);
pub const RBNM_ID: NMREBAR_MASK_FLAGS = NMREBAR_MASK_FLAGS(1u32);
pub const RBNM_LPARAM: NMREBAR_MASK_FLAGS = NMREBAR_MASK_FLAGS(4u32);
pub const RBNM_STYLE: NMREBAR_MASK_FLAGS = NMREBAR_MASK_FLAGS(2u32);
impl ::std::convert::From<u32> for NMREBAR_MASK_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NMREBAR_MASK_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for NMREBAR_MASK_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for NMREBAR_MASK_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for NMREBAR_MASK_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for NMREBAR_MASK_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for NMREBAR_MASK_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMSEARCHWEB {
    pub hdr: NMHDR,
    pub entrypoint: EC_SEARCHWEB_ENTRYPOINT,
    pub hasQueryText: super::super::Foundation::BOOL,
    pub invokeSucceeded: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl NMSEARCHWEB {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMSEARCHWEB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMSEARCHWEB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMSEARCHWEB").field("hdr", &self.hdr).field("entrypoint", &self.entrypoint).field("hasQueryText", &self.hasQueryText).field("invokeSucceeded", &self.invokeSucceeded).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMSEARCHWEB {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.entrypoint == other.entrypoint && self.hasQueryText == other.hasQueryText && self.invokeSucceeded == other.invokeSucceeded
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMSEARCHWEB {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMSEARCHWEB {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMSELCHANGE {
    pub nmhdr: NMHDR,
    pub stSelStart: super::super::Foundation::SYSTEMTIME,
    pub stSelEnd: super::super::Foundation::SYSTEMTIME,
}
#[cfg(feature = "Win32_Foundation")]
impl NMSELCHANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMSELCHANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMSELCHANGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMSELCHANGE").field("nmhdr", &self.nmhdr).field("stSelStart", &self.stSelStart).field("stSelEnd", &self.stSelEnd).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMSELCHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.stSelStart == other.stSelStart && self.stSelEnd == other.stSelEnd
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMSELCHANGE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMSELCHANGE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct NMTBCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub hbrMonoDither: super::super::Graphics::Gdi::HBRUSH,
    pub hbrLines: super::super::Graphics::Gdi::HBRUSH,
    pub hpenLines: super::super::Graphics::Gdi::HPEN,
    pub clrText: u32,
    pub clrMark: u32,
    pub clrTextHighlight: u32,
    pub clrBtnFace: u32,
    pub clrBtnHighlight: u32,
    pub clrHighlightHotTrack: u32,
    pub rcText: super::super::Foundation::RECT,
    pub nStringBkMode: i32,
    pub nHLStringBkMode: i32,
    pub iListGap: i32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl NMTBCUSTOMDRAW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for NMTBCUSTOMDRAW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for NMTBCUSTOMDRAW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMTBCUSTOMDRAW")
            .field("nmcd", &self.nmcd)
            .field("hbrMonoDither", &self.hbrMonoDither)
            .field("hbrLines", &self.hbrLines)
            .field("hpenLines", &self.hpenLines)
            .field("clrText", &self.clrText)
            .field("clrMark", &self.clrMark)
            .field("clrTextHighlight", &self.clrTextHighlight)
            .field("clrBtnFace", &self.clrBtnFace)
            .field("clrBtnHighlight", &self.clrBtnHighlight)
            .field("clrHighlightHotTrack", &self.clrHighlightHotTrack)
            .field("rcText", &self.rcText)
            .field("nStringBkMode", &self.nStringBkMode)
            .field("nHLStringBkMode", &self.nHLStringBkMode)
            .field("iListGap", &self.iListGap)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for NMTBCUSTOMDRAW {
    fn eq(&self, other: &Self) -> bool {
        self.nmcd == other.nmcd
            && self.hbrMonoDither == other.hbrMonoDither
            && self.hbrLines == other.hbrLines
            && self.hpenLines == other.hpenLines
            && self.clrText == other.clrText
            && self.clrMark == other.clrMark
            && self.clrTextHighlight == other.clrTextHighlight
            && self.clrBtnFace == other.clrBtnFace
            && self.clrBtnHighlight == other.clrBtnHighlight
            && self.clrHighlightHotTrack == other.clrHighlightHotTrack
            && self.rcText == other.rcText
            && self.nStringBkMode == other.nStringBkMode
            && self.nHLStringBkMode == other.nHLStringBkMode
            && self.iListGap == other.iListGap
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for NMTBCUSTOMDRAW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for NMTBCUSTOMDRAW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMTBDISPINFOA {
    pub hdr: NMHDR,
    pub dwMask: NMTBDISPINFOW_MASK,
    pub idCommand: i32,
    pub lParam: usize,
    pub iImage: i32,
    pub pszText: super::super::Foundation::PSTR,
    pub cchText: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl NMTBDISPINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMTBDISPINFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMTBDISPINFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMTBDISPINFOA").field("hdr", &self.hdr).field("dwMask", &self.dwMask).field("idCommand", &self.idCommand).field("lParam", &self.lParam).field("iImage", &self.iImage).field("pszText", &self.pszText).field("cchText", &self.cchText).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMTBDISPINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwMask == other.dwMask && self.idCommand == other.idCommand && self.lParam == other.lParam && self.iImage == other.iImage && self.pszText == other.pszText && self.cchText == other.cchText
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMTBDISPINFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMTBDISPINFOA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMTBDISPINFOW {
    pub hdr: NMHDR,
    pub dwMask: NMTBDISPINFOW_MASK,
    pub idCommand: i32,
    pub lParam: usize,
    pub iImage: i32,
    pub pszText: super::super::Foundation::PWSTR,
    pub cchText: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl NMTBDISPINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMTBDISPINFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMTBDISPINFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMTBDISPINFOW").field("hdr", &self.hdr).field("dwMask", &self.dwMask).field("idCommand", &self.idCommand).field("lParam", &self.lParam).field("iImage", &self.iImage).field("pszText", &self.pszText).field("cchText", &self.cchText).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMTBDISPINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwMask == other.dwMask && self.idCommand == other.idCommand && self.lParam == other.lParam && self.iImage == other.iImage && self.pszText == other.pszText && self.cchText == other.cchText
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMTBDISPINFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMTBDISPINFOW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NMTBDISPINFOW_MASK(pub u32);
pub const TBNF_IMAGE: NMTBDISPINFOW_MASK = NMTBDISPINFOW_MASK(1u32);
pub const TBNF_TEXT: NMTBDISPINFOW_MASK = NMTBDISPINFOW_MASK(2u32);
pub const TBNF_DI_SETITEM: NMTBDISPINFOW_MASK = NMTBDISPINFOW_MASK(268435456u32);
impl ::std::convert::From<u32> for NMTBDISPINFOW_MASK {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NMTBDISPINFOW_MASK {
    type Abi = Self;
}
impl ::std::ops::BitOr for NMTBDISPINFOW_MASK {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for NMTBDISPINFOW_MASK {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for NMTBDISPINFOW_MASK {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for NMTBDISPINFOW_MASK {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for NMTBDISPINFOW_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMTBGETINFOTIPA {
    pub hdr: NMHDR,
    pub pszText: super::super::Foundation::PSTR,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl NMTBGETINFOTIPA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMTBGETINFOTIPA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMTBGETINFOTIPA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMTBGETINFOTIPA").field("hdr", &self.hdr).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iItem", &self.iItem).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMTBGETINFOTIPA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iItem == other.iItem && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMTBGETINFOTIPA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMTBGETINFOTIPA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMTBGETINFOTIPW {
    pub hdr: NMHDR,
    pub pszText: super::super::Foundation::PWSTR,
    pub cchTextMax: i32,
    pub iItem: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl NMTBGETINFOTIPW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMTBGETINFOTIPW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMTBGETINFOTIPW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMTBGETINFOTIPW").field("hdr", &self.hdr).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iItem", &self.iItem).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMTBGETINFOTIPW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iItem == other.iItem && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMTBGETINFOTIPW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMTBGETINFOTIPW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMTBHOTITEM {
    pub hdr: NMHDR,
    pub idOld: i32,
    pub idNew: i32,
    pub dwFlags: NMTBHOTITEM_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl NMTBHOTITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMTBHOTITEM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMTBHOTITEM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMTBHOTITEM").field("hdr", &self.hdr).field("idOld", &self.idOld).field("idNew", &self.idNew).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMTBHOTITEM {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.idOld == other.idOld && self.idNew == other.idNew && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMTBHOTITEM {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMTBHOTITEM {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NMTBHOTITEM_FLAGS(pub u32);
pub const HICF_ACCELERATOR: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(4u32);
pub const HICF_ARROWKEYS: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(2u32);
pub const HICF_DUPACCEL: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(8u32);
pub const HICF_ENTERING: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(16u32);
pub const HICF_LEAVING: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(32u32);
pub const HICF_LMOUSE: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(128u32);
pub const HICF_MOUSE: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(1u32);
pub const HICF_OTHER: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(0u32);
pub const HICF_RESELECT: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(64u32);
pub const HICF_TOGGLEDROPDOWN: NMTBHOTITEM_FLAGS = NMTBHOTITEM_FLAGS(256u32);
impl ::std::convert::From<u32> for NMTBHOTITEM_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NMTBHOTITEM_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for NMTBHOTITEM_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for NMTBHOTITEM_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for NMTBHOTITEM_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for NMTBHOTITEM_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for NMTBHOTITEM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMTBRESTORE {
    pub hdr: NMHDR,
    pub pData: *mut u32,
    pub pCurrent: *mut u32,
    pub cbData: u32,
    pub iItem: i32,
    pub cButtons: i32,
    pub cbBytesPerRecord: i32,
    pub tbButton: TBBUTTON,
}
#[cfg(feature = "Win32_Foundation")]
impl NMTBRESTORE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMTBRESTORE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMTBRESTORE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMTBRESTORE")
            .field("hdr", &self.hdr)
            .field("pData", &self.pData)
            .field("pCurrent", &self.pCurrent)
            .field("cbData", &self.cbData)
            .field("iItem", &self.iItem)
            .field("cButtons", &self.cButtons)
            .field("cbBytesPerRecord", &self.cbBytesPerRecord)
            .field("tbButton", &self.tbButton)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMTBRESTORE {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.pData == other.pData && self.pCurrent == other.pCurrent && self.cbData == other.cbData && self.iItem == other.iItem && self.cButtons == other.cButtons && self.cbBytesPerRecord == other.cbBytesPerRecord && self.tbButton == other.tbButton
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMTBRESTORE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMTBRESTORE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMTBSAVE {
    pub hdr: NMHDR,
    pub pData: *mut u32,
    pub pCurrent: *mut u32,
    pub cbData: u32,
    pub iItem: i32,
    pub cButtons: i32,
    pub tbButton: TBBUTTON,
}
#[cfg(feature = "Win32_Foundation")]
impl NMTBSAVE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMTBSAVE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMTBSAVE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMTBSAVE").field("hdr", &self.hdr).field("pData", &self.pData).field("pCurrent", &self.pCurrent).field("cbData", &self.cbData).field("iItem", &self.iItem).field("cButtons", &self.cButtons).field("tbButton", &self.tbButton).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMTBSAVE {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.pData == other.pData && self.pCurrent == other.pCurrent && self.cbData == other.cbData && self.iItem == other.iItem && self.cButtons == other.cButtons && self.tbButton == other.tbButton
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMTBSAVE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMTBSAVE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMTCKEYDOWN {
    pub hdr: NMHDR,
    pub wVKey: u16,
    pub flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl NMTCKEYDOWN {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMTCKEYDOWN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMTCKEYDOWN {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMTCKEYDOWN {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMTCKEYDOWN {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMTOOLBARA {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub tbButton: TBBUTTON,
    pub cchText: i32,
    pub pszText: super::super::Foundation::PSTR,
    pub rcButton: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl NMTOOLBARA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMTOOLBARA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMTOOLBARA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMTOOLBARA").field("hdr", &self.hdr).field("iItem", &self.iItem).field("tbButton", &self.tbButton).field("cchText", &self.cchText).field("pszText", &self.pszText).field("rcButton", &self.rcButton).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMTOOLBARA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iItem == other.iItem && self.tbButton == other.tbButton && self.cchText == other.cchText && self.pszText == other.pszText && self.rcButton == other.rcButton
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMTOOLBARA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMTOOLBARA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMTOOLBARW {
    pub hdr: NMHDR,
    pub iItem: i32,
    pub tbButton: TBBUTTON,
    pub cchText: i32,
    pub pszText: super::super::Foundation::PWSTR,
    pub rcButton: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl NMTOOLBARW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMTOOLBARW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMTOOLBARW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMTOOLBARW").field("hdr", &self.hdr).field("iItem", &self.iItem).field("tbButton", &self.tbButton).field("cchText", &self.cchText).field("pszText", &self.pszText).field("rcButton", &self.rcButton).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMTOOLBARW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iItem == other.iItem && self.tbButton == other.tbButton && self.cchText == other.cchText && self.pszText == other.pszText && self.rcButton == other.rcButton
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMTOOLBARW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMTOOLBARW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMTOOLTIPSCREATED {
    pub hdr: NMHDR,
    pub hwndToolTips: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl NMTOOLTIPSCREATED {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMTOOLTIPSCREATED {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMTOOLTIPSCREATED {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMTOOLTIPSCREATED").field("hdr", &self.hdr).field("hwndToolTips", &self.hwndToolTips).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMTOOLTIPSCREATED {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.hwndToolTips == other.hwndToolTips
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMTOOLTIPSCREATED {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMTOOLTIPSCREATED {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMTRBTHUMBPOSCHANGING {
    pub hdr: NMHDR,
    pub dwPos: u32,
    pub nReason: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl NMTRBTHUMBPOSCHANGING {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMTRBTHUMBPOSCHANGING {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMTRBTHUMBPOSCHANGING {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMTRBTHUMBPOSCHANGING").field("hdr", &self.hdr).field("dwPos", &self.dwPos).field("nReason", &self.nReason).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMTRBTHUMBPOSCHANGING {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwPos == other.dwPos && self.nReason == other.nReason
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMTRBTHUMBPOSCHANGING {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMTRBTHUMBPOSCHANGING {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMTREEVIEWA {
    pub hdr: NMHDR,
    pub action: u32,
    pub itemOld: TVITEMA,
    pub itemNew: TVITEMA,
    pub ptDrag: super::super::Foundation::POINT,
}
#[cfg(feature = "Win32_Foundation")]
impl NMTREEVIEWA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMTREEVIEWA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMTREEVIEWA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMTREEVIEWA").field("hdr", &self.hdr).field("action", &self.action).field("itemOld", &self.itemOld).field("itemNew", &self.itemNew).field("ptDrag", &self.ptDrag).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMTREEVIEWA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.action == other.action && self.itemOld == other.itemOld && self.itemNew == other.itemNew && self.ptDrag == other.ptDrag
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMTREEVIEWA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMTREEVIEWA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMTREEVIEWW {
    pub hdr: NMHDR,
    pub action: u32,
    pub itemOld: TVITEMW,
    pub itemNew: TVITEMW,
    pub ptDrag: super::super::Foundation::POINT,
}
#[cfg(feature = "Win32_Foundation")]
impl NMTREEVIEWW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMTREEVIEWW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMTREEVIEWW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMTREEVIEWW").field("hdr", &self.hdr).field("action", &self.action).field("itemOld", &self.itemOld).field("itemNew", &self.itemNew).field("ptDrag", &self.ptDrag).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMTREEVIEWW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.action == other.action && self.itemOld == other.itemOld && self.itemNew == other.itemNew && self.ptDrag == other.ptDrag
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMTREEVIEWW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMTREEVIEWW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct NMTTCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub uDrawFlags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl NMTTCUSTOMDRAW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for NMTTCUSTOMDRAW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for NMTTCUSTOMDRAW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMTTCUSTOMDRAW").field("nmcd", &self.nmcd).field("uDrawFlags", &self.uDrawFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for NMTTCUSTOMDRAW {
    fn eq(&self, other: &Self) -> bool {
        self.nmcd == other.nmcd && self.uDrawFlags == other.uDrawFlags
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for NMTTCUSTOMDRAW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for NMTTCUSTOMDRAW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMTTDISPINFOA {
    pub hdr: NMHDR,
    pub lpszText: super::super::Foundation::PSTR,
    pub szText: [super::super::Foundation::CHAR; 80],
    pub hinst: super::super::Foundation::HINSTANCE,
    pub uFlags: u32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl NMTTDISPINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMTTDISPINFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMTTDISPINFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMTTDISPINFOA").field("hdr", &self.hdr).field("lpszText", &self.lpszText).field("szText", &self.szText).field("hinst", &self.hinst).field("uFlags", &self.uFlags).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMTTDISPINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.lpszText == other.lpszText && self.szText == other.szText && self.hinst == other.hinst && self.uFlags == other.uFlags && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMTTDISPINFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMTTDISPINFOA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMTTDISPINFOW {
    pub hdr: NMHDR,
    pub lpszText: super::super::Foundation::PWSTR,
    pub szText: [u16; 80],
    pub hinst: super::super::Foundation::HINSTANCE,
    pub uFlags: u32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl NMTTDISPINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMTTDISPINFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMTTDISPINFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMTTDISPINFOW").field("hdr", &self.hdr).field("lpszText", &self.lpszText).field("szText", &self.szText).field("hinst", &self.hinst).field("uFlags", &self.uFlags).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMTTDISPINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.lpszText == other.lpszText && self.szText == other.szText && self.hinst == other.hinst && self.uFlags == other.uFlags && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMTTDISPINFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMTTDISPINFOW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct NMTVASYNCDRAW {
    pub hdr: NMHDR,
    pub pimldp: *mut IMAGELISTDRAWPARAMS,
    pub hr: ::windows::runtime::HRESULT,
    pub hItem: *mut _TREEITEM,
    pub lParam: super::super::Foundation::LPARAM,
    pub dwRetFlags: u32,
    pub iRetImageIndex: i32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl NMTVASYNCDRAW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for NMTVASYNCDRAW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for NMTVASYNCDRAW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMTVASYNCDRAW").field("hdr", &self.hdr).field("pimldp", &self.pimldp).field("hr", &self.hr).field("hItem", &self.hItem).field("lParam", &self.lParam).field("dwRetFlags", &self.dwRetFlags).field("iRetImageIndex", &self.iRetImageIndex).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for NMTVASYNCDRAW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.pimldp == other.pimldp && self.hr == other.hr && self.hItem == other.hItem && self.lParam == other.lParam && self.dwRetFlags == other.dwRetFlags && self.iRetImageIndex == other.iRetImageIndex
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for NMTVASYNCDRAW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for NMTVASYNCDRAW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct NMTVCUSTOMDRAW {
    pub nmcd: NMCUSTOMDRAW,
    pub clrText: u32,
    pub clrTextBk: u32,
    pub iLevel: i32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl NMTVCUSTOMDRAW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for NMTVCUSTOMDRAW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for NMTVCUSTOMDRAW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMTVCUSTOMDRAW").field("nmcd", &self.nmcd).field("clrText", &self.clrText).field("clrTextBk", &self.clrTextBk).field("iLevel", &self.iLevel).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for NMTVCUSTOMDRAW {
    fn eq(&self, other: &Self) -> bool {
        self.nmcd == other.nmcd && self.clrText == other.clrText && self.clrTextBk == other.clrTextBk && self.iLevel == other.iLevel
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for NMTVCUSTOMDRAW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for NMTVCUSTOMDRAW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMTVDISPINFOA {
    pub hdr: NMHDR,
    pub item: TVITEMA,
}
#[cfg(feature = "Win32_Foundation")]
impl NMTVDISPINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMTVDISPINFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMTVDISPINFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMTVDISPINFOA").field("hdr", &self.hdr).field("item", &self.item).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMTVDISPINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.item == other.item
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMTVDISPINFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMTVDISPINFOA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMTVDISPINFOEXA {
    pub hdr: NMHDR,
    pub item: TVITEMEXA,
}
#[cfg(feature = "Win32_Foundation")]
impl NMTVDISPINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMTVDISPINFOEXA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMTVDISPINFOEXA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMTVDISPINFOEXA").field("hdr", &self.hdr).field("item", &self.item).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMTVDISPINFOEXA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.item == other.item
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMTVDISPINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMTVDISPINFOEXA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMTVDISPINFOEXW {
    pub hdr: NMHDR,
    pub item: TVITEMEXW,
}
#[cfg(feature = "Win32_Foundation")]
impl NMTVDISPINFOEXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMTVDISPINFOEXW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMTVDISPINFOEXW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMTVDISPINFOEXW").field("hdr", &self.hdr).field("item", &self.item).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMTVDISPINFOEXW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.item == other.item
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMTVDISPINFOEXW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMTVDISPINFOEXW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMTVDISPINFOW {
    pub hdr: NMHDR,
    pub item: TVITEMW,
}
#[cfg(feature = "Win32_Foundation")]
impl NMTVDISPINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMTVDISPINFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMTVDISPINFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMTVDISPINFOW").field("hdr", &self.hdr).field("item", &self.item).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMTVDISPINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.item == other.item
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMTVDISPINFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMTVDISPINFOW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMTVGETINFOTIPA {
    pub hdr: NMHDR,
    pub pszText: super::super::Foundation::PSTR,
    pub cchTextMax: i32,
    pub hItem: *mut _TREEITEM,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl NMTVGETINFOTIPA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMTVGETINFOTIPA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMTVGETINFOTIPA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMTVGETINFOTIPA").field("hdr", &self.hdr).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("hItem", &self.hItem).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMTVGETINFOTIPA {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.hItem == other.hItem && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMTVGETINFOTIPA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMTVGETINFOTIPA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMTVGETINFOTIPW {
    pub hdr: NMHDR,
    pub pszText: super::super::Foundation::PWSTR,
    pub cchTextMax: i32,
    pub hItem: *mut _TREEITEM,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl NMTVGETINFOTIPW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMTVGETINFOTIPW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMTVGETINFOTIPW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMTVGETINFOTIPW").field("hdr", &self.hdr).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("hItem", &self.hItem).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMTVGETINFOTIPW {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.hItem == other.hItem && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMTVGETINFOTIPW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMTVGETINFOTIPW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMTVITEMCHANGE {
    pub hdr: NMHDR,
    pub uChanged: u32,
    pub hItem: *mut _TREEITEM,
    pub uStateNew: u32,
    pub uStateOld: u32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl NMTVITEMCHANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMTVITEMCHANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMTVITEMCHANGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMTVITEMCHANGE").field("hdr", &self.hdr).field("uChanged", &self.uChanged).field("hItem", &self.hItem).field("uStateNew", &self.uStateNew).field("uStateOld", &self.uStateOld).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMTVITEMCHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.uChanged == other.uChanged && self.hItem == other.hItem && self.uStateNew == other.uStateNew && self.uStateOld == other.uStateOld && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMTVITEMCHANGE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMTVITEMCHANGE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMTVKEYDOWN {
    pub hdr: NMHDR,
    pub wVKey: u16,
    pub flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl NMTVKEYDOWN {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMTVKEYDOWN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMTVKEYDOWN {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMTVKEYDOWN {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMTVKEYDOWN {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMTVSTATEIMAGECHANGING {
    pub hdr: NMHDR,
    pub hti: *mut _TREEITEM,
    pub iOldStateImageIndex: i32,
    pub iNewStateImageIndex: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl NMTVSTATEIMAGECHANGING {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMTVSTATEIMAGECHANGING {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMTVSTATEIMAGECHANGING {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMTVSTATEIMAGECHANGING").field("hdr", &self.hdr).field("hti", &self.hti).field("iOldStateImageIndex", &self.iOldStateImageIndex).field("iNewStateImageIndex", &self.iNewStateImageIndex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMTVSTATEIMAGECHANGING {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.hti == other.hti && self.iOldStateImageIndex == other.iOldStateImageIndex && self.iNewStateImageIndex == other.iNewStateImageIndex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMTVSTATEIMAGECHANGING {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMTVSTATEIMAGECHANGING {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMUPDOWN {
    pub hdr: NMHDR,
    pub iPos: i32,
    pub iDelta: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl NMUPDOWN {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMUPDOWN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMUPDOWN {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMUPDOWN").field("hdr", &self.hdr).field("iPos", &self.iPos).field("iDelta", &self.iDelta).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMUPDOWN {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.iPos == other.iPos && self.iDelta == other.iDelta
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMUPDOWN {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMUPDOWN {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct NMVIEWCHANGE {
    pub nmhdr: NMHDR,
    pub dwOldView: u32,
    pub dwNewView: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl NMVIEWCHANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NMVIEWCHANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NMVIEWCHANGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NMVIEWCHANGE").field("nmhdr", &self.nmhdr).field("dwOldView", &self.dwOldView).field("dwNewView", &self.dwNewView).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NMVIEWCHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.dwOldView == other.dwOldView && self.dwNewView == other.dwNewView
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NMVIEWCHANGE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NMVIEWCHANGE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const NM_GETCUSTOMSPLITRECT: u32 = 4294966049u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const ODT_HEADER: u32 = 100u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OFFSETTYPE(pub i32);
pub const OT_TOPLEFT: OFFSETTYPE = OFFSETTYPE(0i32);
pub const OT_TOPRIGHT: OFFSETTYPE = OFFSETTYPE(1i32);
pub const OT_TOPMIDDLE: OFFSETTYPE = OFFSETTYPE(2i32);
pub const OT_BOTTOMLEFT: OFFSETTYPE = OFFSETTYPE(3i32);
pub const OT_BOTTOMRIGHT: OFFSETTYPE = OFFSETTYPE(4i32);
pub const OT_BOTTOMMIDDLE: OFFSETTYPE = OFFSETTYPE(5i32);
pub const OT_MIDDLELEFT: OFFSETTYPE = OFFSETTYPE(6i32);
pub const OT_MIDDLERIGHT: OFFSETTYPE = OFFSETTYPE(7i32);
pub const OT_LEFTOFCAPTION: OFFSETTYPE = OFFSETTYPE(8i32);
pub const OT_RIGHTOFCAPTION: OFFSETTYPE = OFFSETTYPE(9i32);
pub const OT_LEFTOFLASTBUTTON: OFFSETTYPE = OFFSETTYPE(10i32);
pub const OT_RIGHTOFLASTBUTTON: OFFSETTYPE = OFFSETTYPE(11i32);
pub const OT_ABOVELASTBUTTON: OFFSETTYPE = OFFSETTYPE(12i32);
pub const OT_BELOWLASTBUTTON: OFFSETTYPE = OFFSETTYPE(13i32);
impl ::std::convert::From<i32> for OFFSETTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OFFSETTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPENBOXSTATES(pub i32);
pub const SPOB_NORMAL: OPENBOXSTATES = OPENBOXSTATES(1i32);
pub const SPOB_HOT: OPENBOXSTATES = OPENBOXSTATES(2i32);
pub const SPOB_SELECTED: OPENBOXSTATES = OPENBOXSTATES(3i32);
pub const SPOB_DISABLED: OPENBOXSTATES = OPENBOXSTATES(4i32);
pub const SPOB_FOCUSED: OPENBOXSTATES = OPENBOXSTATES(5i32);
impl ::std::convert::From<i32> for OPENBOXSTATES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OPENBOXSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OPEN_THEME_DATA_FLAGS(pub u32);
pub const OTD_FORCE_RECT_SIZING: OPEN_THEME_DATA_FLAGS = OPEN_THEME_DATA_FLAGS(1u32);
pub const OTD_NONCLIENT: OPEN_THEME_DATA_FLAGS = OPEN_THEME_DATA_FLAGS(2u32);
impl ::std::convert::From<u32> for OPEN_THEME_DATA_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OPEN_THEME_DATA_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for OPEN_THEME_DATA_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for OPEN_THEME_DATA_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for OPEN_THEME_DATA_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for OPEN_THEME_DATA_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for OPEN_THEME_DATA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenThemeData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hwnd: Param0, pszclasslist: Param1) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenThemeData(hwnd: super::super::Foundation::HWND, pszclasslist: super::super::Foundation::PWSTR) -> isize;
        }
        ::std::mem::transmute(OpenThemeData(hwnd.into_param().abi(), pszclasslist.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenThemeDataEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hwnd: Param0, pszclasslist: Param1, dwflags: OPEN_THEME_DATA_FLAGS) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenThemeDataEx(hwnd: super::super::Foundation::HWND, pszclasslist: super::super::Foundation::PWSTR, dwflags: OPEN_THEME_DATA_FLAGS) -> isize;
        }
        ::std::mem::transmute(OpenThemeDataEx(hwnd.into_param().abi(), pszclasslist.into_param().abi(), ::std::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PAGEPARTS(pub i32);
pub const PGRP_UP: PAGEPARTS = PAGEPARTS(1i32);
pub const PGRP_DOWN: PAGEPARTS = PAGEPARTS(2i32);
pub const PGRP_UPHORZ: PAGEPARTS = PAGEPARTS(3i32);
pub const PGRP_DOWNHORZ: PAGEPARTS = PAGEPARTS(4i32);
impl ::std::convert::From<i32> for PAGEPARTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PAGEPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PAGESETUPDLGORD: u32 = 1546u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PAGESETUPDLGORDMOTIF: u32 = 1550u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PBM_DELTAPOS: u32 = 1027u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PBM_GETBARCOLOR: u32 = 1039u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PBM_GETBKCOLOR: u32 = 1038u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PBM_GETPOS: u32 = 1032u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PBM_GETRANGE: u32 = 1031u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PBM_GETSTATE: u32 = 1041u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PBM_GETSTEP: u32 = 1037u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PBM_SETBARCOLOR: u32 = 1033u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PBM_SETBKCOLOR: u32 = 8193u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PBM_SETMARQUEE: u32 = 1034u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PBM_SETPOS: u32 = 1026u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PBM_SETRANGE: u32 = 1025u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PBM_SETRANGE32: u32 = 1030u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PBM_SETSTATE: u32 = 1040u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PBM_SETSTEP: u32 = 1028u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PBM_STEPIT: u32 = 1029u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct PBRANGE {
    pub iLow: i32,
    pub iHigh: i32,
}
impl PBRANGE {}
impl ::std::default::Default for PBRANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PBRANGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PBRANGE").field("iLow", &self.iLow).field("iHigh", &self.iHigh).finish()
    }
}
impl ::std::cmp::PartialEq for PBRANGE {
    fn eq(&self, other: &Self) -> bool {
        self.iLow == other.iLow && self.iHigh == other.iHigh
    }
}
impl ::std::cmp::Eq for PBRANGE {}
unsafe impl ::windows::runtime::Abi for PBRANGE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PBST_ERROR: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PBST_NORMAL: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PBST_PAUSED: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PBS_MARQUEE: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PBS_SMOOTH: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PBS_SMOOTHREVERSE: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PBS_VERTICAL: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFNDACOMPARE = unsafe extern "system" fn(p1: *const ::std::ffi::c_void, p2: *const ::std::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFNDACOMPARECONST = unsafe extern "system" fn(p1: *const ::std::ffi::c_void, p2: *const ::std::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> i32;
pub type PFNDAENUMCALLBACK = unsafe extern "system" fn(p: *const ::std::ffi::c_void, pdata: *const ::std::ffi::c_void) -> i32;
pub type PFNDAENUMCALLBACKCONST = unsafe extern "system" fn(p: *const ::std::ffi::c_void, pdata: *const ::std::ffi::c_void) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFNDPAMERGE = unsafe extern "system" fn(umsg: DPAMM_MESSAGE, pvdest: *const ::std::ffi::c_void, pvsrc: *const ::std::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> *mut ::std::ffi::c_void;
#[cfg(feature = "Win32_Foundation")]
pub type PFNDPAMERGECONST = unsafe extern "system" fn(umsg: DPAMM_MESSAGE, pvdest: *const ::std::ffi::c_void, pvsrc: *const ::std::ffi::c_void, lparam: super::super::Foundation::LPARAM) -> *mut ::std::ffi::c_void;
#[cfg(feature = "Win32_System_Com")]
pub type PFNDPASTREAM = unsafe extern "system" fn(pinfo: *const DPASTREAMINFO, pstream: ::windows::runtime::RawPtr, pvinstdata: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PFNLVCOMPARE = unsafe extern "system" fn(param0: super::super::Foundation::LPARAM, param1: super::super::Foundation::LPARAM, param2: super::super::Foundation::LPARAM) -> i32;
pub type PFNLVGROUPCOMPARE = unsafe extern "system" fn(param0: i32, param1: i32, param2: *mut ::std::ffi::c_void) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFNPROPSHEETCALLBACK = unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: u32, param2: super::super::Foundation::LPARAM) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFNTVCOMPARE = unsafe extern "system" fn(lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM, lparamsort: super::super::Foundation::LPARAM) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PFTASKDIALOGCALLBACK = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, lprefdata: isize) -> ::windows::runtime::HRESULT;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PGB_BOTTOMORRIGHT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PGB_TOPORLEFT: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PGF_DEPRESSED: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PGF_GRAYED: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PGF_HOT: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PGF_INVISIBLE: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PGF_NORMAL: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PGM_FIRST: u32 = 5120u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PGM_FORWARDMOUSE: u32 = 5123u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PGM_GETBKCOLOR: u32 = 5125u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PGM_GETBORDER: u32 = 5127u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PGM_GETBUTTONSIZE: u32 = 5131u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PGM_GETBUTTONSTATE: u32 = 5132u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PGM_GETDROPTARGET: u32 = 8196u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PGM_GETPOS: u32 = 5129u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PGM_RECALCSIZE: u32 = 5122u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PGM_SETBKCOLOR: u32 = 5124u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PGM_SETBORDER: u32 = 5126u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PGM_SETBUTTONSIZE: u32 = 5130u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PGM_SETCHILD: u32 = 5121u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PGM_SETPOS: u32 = 5128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PGM_SETSCROLLINFO: u32 = 5133u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PGS_AUTOSCROLL: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PGS_DRAGNDROP: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PGS_HORZ: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PGS_VERT: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct POINTER_DEVICE_CURSOR_INFO {
    pub cursorId: u32,
    pub cursor: POINTER_DEVICE_CURSOR_TYPE,
}
impl POINTER_DEVICE_CURSOR_INFO {}
impl ::std::default::Default for POINTER_DEVICE_CURSOR_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for POINTER_DEVICE_CURSOR_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("POINTER_DEVICE_CURSOR_INFO").field("cursorId", &self.cursorId).field("cursor", &self.cursor).finish()
    }
}
impl ::std::cmp::PartialEq for POINTER_DEVICE_CURSOR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cursorId == other.cursorId && self.cursor == other.cursor
    }
}
impl ::std::cmp::Eq for POINTER_DEVICE_CURSOR_INFO {}
unsafe impl ::windows::runtime::Abi for POINTER_DEVICE_CURSOR_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct POINTER_DEVICE_CURSOR_TYPE(pub i32);
pub const POINTER_DEVICE_CURSOR_TYPE_UNKNOWN: POINTER_DEVICE_CURSOR_TYPE = POINTER_DEVICE_CURSOR_TYPE(0i32);
pub const POINTER_DEVICE_CURSOR_TYPE_TIP: POINTER_DEVICE_CURSOR_TYPE = POINTER_DEVICE_CURSOR_TYPE(1i32);
pub const POINTER_DEVICE_CURSOR_TYPE_ERASER: POINTER_DEVICE_CURSOR_TYPE = POINTER_DEVICE_CURSOR_TYPE(2i32);
pub const POINTER_DEVICE_CURSOR_TYPE_MAX: POINTER_DEVICE_CURSOR_TYPE = POINTER_DEVICE_CURSOR_TYPE(-1i32);
impl ::std::convert::From<i32> for POINTER_DEVICE_CURSOR_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for POINTER_DEVICE_CURSOR_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct POINTER_DEVICE_INFO {
    pub displayOrientation: u32,
    pub device: super::super::Foundation::HANDLE,
    pub pointerDeviceType: POINTER_DEVICE_TYPE,
    pub monitor: super::super::Graphics::Gdi::HMONITOR,
    pub startingCursorId: u32,
    pub maxActiveContacts: u16,
    pub productString: [u16; 520],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl POINTER_DEVICE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for POINTER_DEVICE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for POINTER_DEVICE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("POINTER_DEVICE_INFO")
            .field("displayOrientation", &self.displayOrientation)
            .field("device", &self.device)
            .field("pointerDeviceType", &self.pointerDeviceType)
            .field("monitor", &self.monitor)
            .field("startingCursorId", &self.startingCursorId)
            .field("maxActiveContacts", &self.maxActiveContacts)
            .field("productString", &self.productString)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for POINTER_DEVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.displayOrientation == other.displayOrientation && self.device == other.device && self.pointerDeviceType == other.pointerDeviceType && self.monitor == other.monitor && self.startingCursorId == other.startingCursorId && self.maxActiveContacts == other.maxActiveContacts && self.productString == other.productString
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for POINTER_DEVICE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for POINTER_DEVICE_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct POINTER_DEVICE_PROPERTY {
    pub logicalMin: i32,
    pub logicalMax: i32,
    pub physicalMin: i32,
    pub physicalMax: i32,
    pub unit: u32,
    pub unitExponent: u32,
    pub usagePageId: u16,
    pub usageId: u16,
}
impl POINTER_DEVICE_PROPERTY {}
impl ::std::default::Default for POINTER_DEVICE_PROPERTY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for POINTER_DEVICE_PROPERTY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("POINTER_DEVICE_PROPERTY")
            .field("logicalMin", &self.logicalMin)
            .field("logicalMax", &self.logicalMax)
            .field("physicalMin", &self.physicalMin)
            .field("physicalMax", &self.physicalMax)
            .field("unit", &self.unit)
            .field("unitExponent", &self.unitExponent)
            .field("usagePageId", &self.usagePageId)
            .field("usageId", &self.usageId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for POINTER_DEVICE_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.logicalMin == other.logicalMin && self.logicalMax == other.logicalMax && self.physicalMin == other.physicalMin && self.physicalMax == other.physicalMax && self.unit == other.unit && self.unitExponent == other.unitExponent && self.usagePageId == other.usagePageId && self.usageId == other.usageId
    }
}
impl ::std::cmp::Eq for POINTER_DEVICE_PROPERTY {}
unsafe impl ::windows::runtime::Abi for POINTER_DEVICE_PROPERTY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct POINTER_DEVICE_TYPE(pub i32);
pub const POINTER_DEVICE_TYPE_INTEGRATED_PEN: POINTER_DEVICE_TYPE = POINTER_DEVICE_TYPE(1i32);
pub const POINTER_DEVICE_TYPE_EXTERNAL_PEN: POINTER_DEVICE_TYPE = POINTER_DEVICE_TYPE(2i32);
pub const POINTER_DEVICE_TYPE_TOUCH: POINTER_DEVICE_TYPE = POINTER_DEVICE_TYPE(3i32);
pub const POINTER_DEVICE_TYPE_TOUCH_PAD: POINTER_DEVICE_TYPE = POINTER_DEVICE_TYPE(4i32);
pub const POINTER_DEVICE_TYPE_MAX: POINTER_DEVICE_TYPE = POINTER_DEVICE_TYPE(-1i32);
impl ::std::convert::From<i32> for POINTER_DEVICE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for POINTER_DEVICE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct POINTER_FEEDBACK_MODE(pub i32);
pub const POINTER_FEEDBACK_DEFAULT: POINTER_FEEDBACK_MODE = POINTER_FEEDBACK_MODE(1i32);
pub const POINTER_FEEDBACK_INDIRECT: POINTER_FEEDBACK_MODE = POINTER_FEEDBACK_MODE(2i32);
pub const POINTER_FEEDBACK_NONE: POINTER_FEEDBACK_MODE = POINTER_FEEDBACK_MODE(3i32);
impl ::std::convert::From<i32> for POINTER_FEEDBACK_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for POINTER_FEEDBACK_MODE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_Input_Pointer`, `Win32_UI_WindowsAndMessaging`*"]
pub struct POINTER_TYPE_INFO {
    pub r#type: super::WindowsAndMessaging::POINTER_INPUT_TYPE,
    pub Anonymous: POINTER_TYPE_INFO_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl POINTER_TYPE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for POINTER_TYPE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for POINTER_TYPE_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for POINTER_TYPE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for POINTER_TYPE_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_Input_Pointer`, `Win32_UI_WindowsAndMessaging`*"]
pub union POINTER_TYPE_INFO_0 {
    pub touchInfo: super::Input::Pointer::POINTER_TOUCH_INFO,
    pub penInfo: super::Input::Pointer::POINTER_PEN_INFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl POINTER_TYPE_INFO_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for POINTER_TYPE_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for POINTER_TYPE_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for POINTER_TYPE_INFO_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for POINTER_TYPE_INFO_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PRINTDLGEXORD: u32 = 1549u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PRINTDLGORD: u32 = 1538u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PRNSETUPDLGORD: u32 = 1539u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROPERTYORIGIN(pub i32);
pub const PO_STATE: PROPERTYORIGIN = PROPERTYORIGIN(0i32);
pub const PO_PART: PROPERTYORIGIN = PROPERTYORIGIN(1i32);
pub const PO_CLASS: PROPERTYORIGIN = PROPERTYORIGIN(2i32);
pub const PO_GLOBAL: PROPERTYORIGIN = PROPERTYORIGIN(3i32);
pub const PO_NOTFOUND: PROPERTYORIGIN = PROPERTYORIGIN(4i32);
impl ::std::convert::From<i32> for PROPERTYORIGIN {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROPERTYORIGIN {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::clone::Clone for PROPSHEETHEADERA_V1 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
pub struct PROPSHEETHEADERA_V1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETHEADERA_V1_0,
    pub pszCaption: super::super::Foundation::PSTR,
    pub nPages: u32,
    pub Anonymous2: PROPSHEETHEADERA_V1_1,
    pub Anonymous3: PROPSHEETHEADERA_V1_2,
    pub pfnCallback: ::std::option::Option<PFNPROPSHEETCALLBACK>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETHEADERA_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETHEADERA_V1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETHEADERA_V1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETHEADERA_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETHEADERA_V1 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
pub union PROPSHEETHEADERA_V1_0 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: super::super::Foundation::PSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETHEADERA_V1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETHEADERA_V1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETHEADERA_V1_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETHEADERA_V1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETHEADERA_V1_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub union PROPSHEETHEADERA_V1_1 {
    pub nStartPage: u32,
    pub pStartPage: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PROPSHEETHEADERA_V1_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PROPSHEETHEADERA_V1_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PROPSHEETHEADERA_V1_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PROPSHEETHEADERA_V1_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PROPSHEETHEADERA_V1_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
pub union PROPSHEETHEADERA_V1_2 {
    pub ppsp: *mut ::std::mem::ManuallyDrop<PROPSHEETPAGEA>,
    pub phpage: *mut HPROPSHEETPAGE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETHEADERA_V1_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETHEADERA_V1_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETHEADERA_V1_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETHEADERA_V1_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETHEADERA_V1_2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::clone::Clone for PROPSHEETHEADERA_V2 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
pub struct PROPSHEETHEADERA_V2 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETHEADERA_V2_0,
    pub pszCaption: super::super::Foundation::PSTR,
    pub nPages: u32,
    pub Anonymous2: PROPSHEETHEADERA_V2_1,
    pub Anonymous3: PROPSHEETHEADERA_V2_2,
    pub pfnCallback: ::std::option::Option<PFNPROPSHEETCALLBACK>,
    pub Anonymous4: PROPSHEETHEADERA_V2_3,
    pub hplWatermark: super::super::Graphics::Gdi::HPALETTE,
    pub Anonymous5: PROPSHEETHEADERA_V2_4,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETHEADERA_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETHEADERA_V2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETHEADERA_V2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETHEADERA_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETHEADERA_V2 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
pub union PROPSHEETHEADERA_V2_0 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: super::super::Foundation::PSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETHEADERA_V2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETHEADERA_V2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETHEADERA_V2_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETHEADERA_V2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETHEADERA_V2_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub union PROPSHEETHEADERA_V2_1 {
    pub nStartPage: u32,
    pub pStartPage: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PROPSHEETHEADERA_V2_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PROPSHEETHEADERA_V2_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PROPSHEETHEADERA_V2_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PROPSHEETHEADERA_V2_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PROPSHEETHEADERA_V2_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
pub union PROPSHEETHEADERA_V2_2 {
    pub ppsp: *mut ::std::mem::ManuallyDrop<PROPSHEETPAGEA>,
    pub phpage: *mut HPROPSHEETPAGE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETHEADERA_V2_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETHEADERA_V2_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETHEADERA_V2_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETHEADERA_V2_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETHEADERA_V2_2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub union PROPSHEETHEADERA_V2_3 {
    pub hbmWatermark: super::super::Graphics::Gdi::HBITMAP,
    pub pszbmWatermark: super::super::Foundation::PSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl PROPSHEETHEADERA_V2_3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for PROPSHEETHEADERA_V2_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for PROPSHEETHEADERA_V2_3 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for PROPSHEETHEADERA_V2_3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETHEADERA_V2_3 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub union PROPSHEETHEADERA_V2_4 {
    pub hbmHeader: super::super::Graphics::Gdi::HBITMAP,
    pub pszbmHeader: super::super::Foundation::PSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl PROPSHEETHEADERA_V2_4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for PROPSHEETHEADERA_V2_4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for PROPSHEETHEADERA_V2_4 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for PROPSHEETHEADERA_V2_4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETHEADERA_V2_4 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::clone::Clone for PROPSHEETHEADERW_V1 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
pub struct PROPSHEETHEADERW_V1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETHEADERW_V1_0,
    pub pszCaption: super::super::Foundation::PWSTR,
    pub nPages: u32,
    pub Anonymous2: PROPSHEETHEADERW_V1_1,
    pub Anonymous3: PROPSHEETHEADERW_V1_2,
    pub pfnCallback: ::std::option::Option<PFNPROPSHEETCALLBACK>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETHEADERW_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETHEADERW_V1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETHEADERW_V1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETHEADERW_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETHEADERW_V1 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
pub union PROPSHEETHEADERW_V1_0 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETHEADERW_V1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETHEADERW_V1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETHEADERW_V1_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETHEADERW_V1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETHEADERW_V1_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub union PROPSHEETHEADERW_V1_1 {
    pub nStartPage: u32,
    pub pStartPage: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PROPSHEETHEADERW_V1_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PROPSHEETHEADERW_V1_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PROPSHEETHEADERW_V1_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PROPSHEETHEADERW_V1_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PROPSHEETHEADERW_V1_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
pub union PROPSHEETHEADERW_V1_2 {
    pub ppsp: *mut ::std::mem::ManuallyDrop<PROPSHEETPAGEW>,
    pub phpage: *mut HPROPSHEETPAGE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETHEADERW_V1_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETHEADERW_V1_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETHEADERW_V1_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETHEADERW_V1_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETHEADERW_V1_2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::clone::Clone for PROPSHEETHEADERW_V2 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
pub struct PROPSHEETHEADERW_V2 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETHEADERW_V2_0,
    pub pszCaption: super::super::Foundation::PWSTR,
    pub nPages: u32,
    pub Anonymous2: PROPSHEETHEADERW_V2_1,
    pub Anonymous3: PROPSHEETHEADERW_V2_2,
    pub pfnCallback: ::std::option::Option<PFNPROPSHEETCALLBACK>,
    pub Anonymous4: PROPSHEETHEADERW_V2_3,
    pub hplWatermark: super::super::Graphics::Gdi::HPALETTE,
    pub Anonymous5: PROPSHEETHEADERW_V2_4,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETHEADERW_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETHEADERW_V2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETHEADERW_V2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETHEADERW_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETHEADERW_V2 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
pub union PROPSHEETHEADERW_V2_0 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETHEADERW_V2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETHEADERW_V2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETHEADERW_V2_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETHEADERW_V2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETHEADERW_V2_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub union PROPSHEETHEADERW_V2_1 {
    pub nStartPage: u32,
    pub pStartPage: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl PROPSHEETHEADERW_V2_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PROPSHEETHEADERW_V2_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PROPSHEETHEADERW_V2_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PROPSHEETHEADERW_V2_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PROPSHEETHEADERW_V2_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
pub union PROPSHEETHEADERW_V2_2 {
    pub ppsp: *mut ::std::mem::ManuallyDrop<PROPSHEETPAGEW>,
    pub phpage: *mut HPROPSHEETPAGE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETHEADERW_V2_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETHEADERW_V2_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETHEADERW_V2_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETHEADERW_V2_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETHEADERW_V2_2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub union PROPSHEETHEADERW_V2_3 {
    pub hbmWatermark: super::super::Graphics::Gdi::HBITMAP,
    pub pszbmWatermark: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl PROPSHEETHEADERW_V2_3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for PROPSHEETHEADERW_V2_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for PROPSHEETHEADERW_V2_3 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for PROPSHEETHEADERW_V2_3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETHEADERW_V2_3 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub union PROPSHEETHEADERW_V2_4 {
    pub hbmHeader: super::super::Graphics::Gdi::HBITMAP,
    pub pszbmHeader: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl PROPSHEETHEADERW_V2_4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for PROPSHEETHEADERW_V2_4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for PROPSHEETHEADERW_V2_4 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for PROPSHEETHEADERW_V2_4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETHEADERW_V2_4 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::clone::Clone for PROPSHEETPAGEA {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
pub struct PROPSHEETPAGEA {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEA_0,
    pub Anonymous2: PROPSHEETPAGEA_1,
    pub pszTitle: super::super::Foundation::PSTR,
    pub pfnDlgProc: ::std::option::Option<super::WindowsAndMessaging::DLGPROC>,
    pub lParam: super::super::Foundation::LPARAM,
    pub pfnCallback: ::std::option::Option<LPFNPSPCALLBACKA>,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: super::super::Foundation::PSTR,
    pub pszHeaderSubTitle: super::super::Foundation::PSTR,
    pub hActCtx: super::super::Foundation::HANDLE,
    pub Anonymous3: PROPSHEETPAGEA_2,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETPAGEA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETPAGEA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETPAGEA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETPAGEA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETPAGEA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
pub union PROPSHEETPAGEA_0 {
    pub pszTemplate: super::super::Foundation::PSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETPAGEA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETPAGEA_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETPAGEA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETPAGEA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETPAGEA_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
pub union PROPSHEETPAGEA_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: super::super::Foundation::PSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETPAGEA_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETPAGEA_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETPAGEA_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETPAGEA_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETPAGEA_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub union PROPSHEETPAGEA_2 {
    pub hbmHeader: super::super::Graphics::Gdi::HBITMAP,
    pub pszbmHeader: super::super::Foundation::PSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl PROPSHEETPAGEA_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for PROPSHEETPAGEA_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for PROPSHEETPAGEA_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for PROPSHEETPAGEA_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETPAGEA_2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::clone::Clone for PROPSHEETPAGEA_V1 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
pub struct PROPSHEETPAGEA_V1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEA_V1_0,
    pub Anonymous2: PROPSHEETPAGEA_V1_1,
    pub pszTitle: super::super::Foundation::PSTR,
    pub pfnDlgProc: ::std::option::Option<super::WindowsAndMessaging::DLGPROC>,
    pub lParam: super::super::Foundation::LPARAM,
    pub pfnCallback: ::std::option::Option<LPFNPSPCALLBACKA>,
    pub pcRefParent: *mut u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETPAGEA_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETPAGEA_V1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETPAGEA_V1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETPAGEA_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETPAGEA_V1 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
pub union PROPSHEETPAGEA_V1_0 {
    pub pszTemplate: super::super::Foundation::PSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETPAGEA_V1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETPAGEA_V1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETPAGEA_V1_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETPAGEA_V1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETPAGEA_V1_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
pub union PROPSHEETPAGEA_V1_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: super::super::Foundation::PSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETPAGEA_V1_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETPAGEA_V1_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETPAGEA_V1_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETPAGEA_V1_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETPAGEA_V1_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::clone::Clone for PROPSHEETPAGEA_V2 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
pub struct PROPSHEETPAGEA_V2 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEA_V2_0,
    pub Anonymous2: PROPSHEETPAGEA_V2_1,
    pub pszTitle: super::super::Foundation::PSTR,
    pub pfnDlgProc: ::std::option::Option<super::WindowsAndMessaging::DLGPROC>,
    pub lParam: super::super::Foundation::LPARAM,
    pub pfnCallback: ::std::option::Option<LPFNPSPCALLBACKA>,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: super::super::Foundation::PSTR,
    pub pszHeaderSubTitle: super::super::Foundation::PSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETPAGEA_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETPAGEA_V2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETPAGEA_V2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETPAGEA_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETPAGEA_V2 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
pub union PROPSHEETPAGEA_V2_0 {
    pub pszTemplate: super::super::Foundation::PSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETPAGEA_V2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETPAGEA_V2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETPAGEA_V2_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETPAGEA_V2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETPAGEA_V2_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
pub union PROPSHEETPAGEA_V2_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: super::super::Foundation::PSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETPAGEA_V2_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETPAGEA_V2_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETPAGEA_V2_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETPAGEA_V2_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETPAGEA_V2_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::clone::Clone for PROPSHEETPAGEA_V3 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
pub struct PROPSHEETPAGEA_V3 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEA_V3_0,
    pub Anonymous2: PROPSHEETPAGEA_V3_1,
    pub pszTitle: super::super::Foundation::PSTR,
    pub pfnDlgProc: ::std::option::Option<super::WindowsAndMessaging::DLGPROC>,
    pub lParam: super::super::Foundation::LPARAM,
    pub pfnCallback: ::std::option::Option<LPFNPSPCALLBACKA>,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: super::super::Foundation::PSTR,
    pub pszHeaderSubTitle: super::super::Foundation::PSTR,
    pub hActCtx: super::super::Foundation::HANDLE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETPAGEA_V3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETPAGEA_V3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETPAGEA_V3 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETPAGEA_V3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETPAGEA_V3 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
pub union PROPSHEETPAGEA_V3_0 {
    pub pszTemplate: super::super::Foundation::PSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETPAGEA_V3_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETPAGEA_V3_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETPAGEA_V3_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETPAGEA_V3_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETPAGEA_V3_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
pub union PROPSHEETPAGEA_V3_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: super::super::Foundation::PSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETPAGEA_V3_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETPAGEA_V3_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETPAGEA_V3_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETPAGEA_V3_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETPAGEA_V3_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::clone::Clone for PROPSHEETPAGEW {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
pub struct PROPSHEETPAGEW {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEW_0,
    pub Anonymous2: PROPSHEETPAGEW_1,
    pub pszTitle: super::super::Foundation::PWSTR,
    pub pfnDlgProc: ::std::option::Option<super::WindowsAndMessaging::DLGPROC>,
    pub lParam: super::super::Foundation::LPARAM,
    pub pfnCallback: ::std::option::Option<LPFNPSPCALLBACKW>,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: super::super::Foundation::PWSTR,
    pub pszHeaderSubTitle: super::super::Foundation::PWSTR,
    pub hActCtx: super::super::Foundation::HANDLE,
    pub Anonymous3: PROPSHEETPAGEW_2,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETPAGEW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETPAGEW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETPAGEW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETPAGEW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETPAGEW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
pub union PROPSHEETPAGEW_0 {
    pub pszTemplate: super::super::Foundation::PWSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETPAGEW_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETPAGEW_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETPAGEW_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETPAGEW_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETPAGEW_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
pub union PROPSHEETPAGEW_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETPAGEW_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETPAGEW_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETPAGEW_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETPAGEW_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETPAGEW_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub union PROPSHEETPAGEW_2 {
    pub hbmHeader: super::super::Graphics::Gdi::HBITMAP,
    pub pszbmHeader: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl PROPSHEETPAGEW_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for PROPSHEETPAGEW_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for PROPSHEETPAGEW_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for PROPSHEETPAGEW_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETPAGEW_2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::clone::Clone for PROPSHEETPAGEW_V1 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
pub struct PROPSHEETPAGEW_V1 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEW_V1_0,
    pub Anonymous2: PROPSHEETPAGEW_V1_1,
    pub pszTitle: super::super::Foundation::PWSTR,
    pub pfnDlgProc: ::std::option::Option<super::WindowsAndMessaging::DLGPROC>,
    pub lParam: super::super::Foundation::LPARAM,
    pub pfnCallback: ::std::option::Option<LPFNPSPCALLBACKW>,
    pub pcRefParent: *mut u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETPAGEW_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETPAGEW_V1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETPAGEW_V1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETPAGEW_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETPAGEW_V1 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
pub union PROPSHEETPAGEW_V1_0 {
    pub pszTemplate: super::super::Foundation::PWSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETPAGEW_V1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETPAGEW_V1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETPAGEW_V1_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETPAGEW_V1_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETPAGEW_V1_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
pub union PROPSHEETPAGEW_V1_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETPAGEW_V1_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETPAGEW_V1_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETPAGEW_V1_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETPAGEW_V1_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETPAGEW_V1_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::clone::Clone for PROPSHEETPAGEW_V2 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
pub struct PROPSHEETPAGEW_V2 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEW_V2_0,
    pub Anonymous2: PROPSHEETPAGEW_V2_1,
    pub pszTitle: super::super::Foundation::PWSTR,
    pub pfnDlgProc: ::std::option::Option<super::WindowsAndMessaging::DLGPROC>,
    pub lParam: super::super::Foundation::LPARAM,
    pub pfnCallback: ::std::option::Option<LPFNPSPCALLBACKW>,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: super::super::Foundation::PWSTR,
    pub pszHeaderSubTitle: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETPAGEW_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETPAGEW_V2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETPAGEW_V2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETPAGEW_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETPAGEW_V2 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
pub union PROPSHEETPAGEW_V2_0 {
    pub pszTemplate: super::super::Foundation::PWSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETPAGEW_V2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETPAGEW_V2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETPAGEW_V2_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETPAGEW_V2_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETPAGEW_V2_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
pub union PROPSHEETPAGEW_V2_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETPAGEW_V2_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETPAGEW_V2_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETPAGEW_V2_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETPAGEW_V2_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETPAGEW_V2_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::clone::Clone for PROPSHEETPAGEW_V3 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
pub struct PROPSHEETPAGEW_V3 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub Anonymous1: PROPSHEETPAGEW_V3_0,
    pub Anonymous2: PROPSHEETPAGEW_V3_1,
    pub pszTitle: super::super::Foundation::PWSTR,
    pub pfnDlgProc: ::std::option::Option<super::WindowsAndMessaging::DLGPROC>,
    pub lParam: super::super::Foundation::LPARAM,
    pub pfnCallback: ::std::option::Option<LPFNPSPCALLBACKW>,
    pub pcRefParent: *mut u32,
    pub pszHeaderTitle: super::super::Foundation::PWSTR,
    pub pszHeaderSubTitle: super::super::Foundation::PWSTR,
    pub hActCtx: super::super::Foundation::HANDLE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETPAGEW_V3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETPAGEW_V3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETPAGEW_V3 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETPAGEW_V3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETPAGEW_V3 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
pub union PROPSHEETPAGEW_V3_0 {
    pub pszTemplate: super::super::Foundation::PWSTR,
    pub pResource: *mut super::WindowsAndMessaging::DLGTEMPLATE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETPAGEW_V3_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETPAGEW_V3_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETPAGEW_V3_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETPAGEW_V3_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETPAGEW_V3_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
pub union PROPSHEETPAGEW_V3_1 {
    pub hIcon: super::WindowsAndMessaging::HICON,
    pub pszIcon: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl PROPSHEETPAGEW_V3_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for PROPSHEETPAGEW_V3_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for PROPSHEETPAGEW_V3_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for PROPSHEETPAGEW_V3_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for PROPSHEETPAGEW_V3_1 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PROP_LG_CXDLG: u32 = 252u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PROP_LG_CYDLG: u32 = 218u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PROP_MED_CXDLG: u32 = 227u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PROP_MED_CYDLG: u32 = 215u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PROP_SM_CXDLG: u32 = 212u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PROP_SM_CYDLG: u32 = 188u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSBTN_APPLYNOW: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSBTN_BACK: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSBTN_CANCEL: u32 = 5u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSBTN_FINISH: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSBTN_HELP: u32 = 6u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSBTN_MAX: u32 = 6u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSBTN_NEXT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSBTN_OK: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSCB_BUTTONPRESSED: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSCB_INITIALIZED: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSCB_PRECREATE: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct PSHNOTIFY {
    pub hdr: NMHDR,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl PSHNOTIFY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PSHNOTIFY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PSHNOTIFY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PSHNOTIFY").field("hdr", &self.hdr).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PSHNOTIFY {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PSHNOTIFY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PSHNOTIFY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_AEROWIZARD: u32 = 16384u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_HASHELP: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_HEADER: u32 = 524288u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_HEADERBITMAP: u32 = 134217728u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_MODELESS: u32 = 1024u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_NOAPPLYNOW: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_NOCONTEXTHELP: u32 = 33554432u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_NOMARGIN: u32 = 268435456u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_PROPSHEETPAGE: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_PROPTITLE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_RESIZABLE: u32 = 67108864u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_RTLREADING: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_STRETCHWATERMARK: u32 = 262144u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_USECALLBACK: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_USEHBMHEADER: u32 = 1048576u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_USEHBMWATERMARK: u32 = 65536u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_USEHICON: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_USEHPLWATERMARK: u32 = 131072u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_USEICONID: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_USEPAGELANG: u32 = 2097152u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_USEPSTARTPAGE: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_WATERMARK: u32 = 32768u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_WIZARD: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_WIZARD97: u32 = 8192u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_WIZARDCONTEXTHELP: u32 = 4096u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_WIZARDHASFINISH: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSH_WIZARD_LITE: u32 = 4194304u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_ADDPAGE: u32 = 1127u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_APPLY: u32 = 1134u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_CANCELTOCLOSE: u32 = 1131u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_CHANGED: u32 = 1128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_ENABLEWIZBUTTONS: u32 = 1163u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_GETCURRENTPAGEHWND: u32 = 1142u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_GETRESULT: u32 = 1159u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_GETTABCONTROL: u32 = 1140u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_HWNDTOINDEX: u32 = 1153u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_IDTOINDEX: u32 = 1157u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_INDEXTOHWND: u32 = 1154u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_INDEXTOID: u32 = 1158u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_INDEXTOPAGE: u32 = 1156u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_INSERTPAGE: u32 = 1143u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_ISDIALOGMESSAGE: u32 = 1141u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_PAGETOINDEX: u32 = 1155u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_PRESSBUTTON: u32 = 1137u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_QUERYSIBLINGS: u32 = 1132u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_REBOOTSYSTEM: u32 = 1130u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_RECALCPAGESIZES: u32 = 1160u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_REMOVEPAGE: u32 = 1126u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_RESTARTWINDOWS: u32 = 1129u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_SETBUTTONTEXT: u32 = 1164u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_SETBUTTONTEXTW: u32 = 1164u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_SETCURSEL: u32 = 1125u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_SETCURSELID: u32 = 1138u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_SETFINISHTEXT: u32 = 1145u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_SETFINISHTEXTA: u32 = 1139u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_SETFINISHTEXTW: u32 = 1145u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_SETHEADERSUBTITLE: u32 = 1152u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_SETHEADERSUBTITLEA: u32 = 1151u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_SETHEADERSUBTITLEW: u32 = 1152u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_SETHEADERTITLE: u32 = 1150u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_SETHEADERTITLEA: u32 = 1149u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_SETHEADERTITLEW: u32 = 1150u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_SETNEXTTEXT: u32 = 1161u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_SETNEXTTEXTW: u32 = 1161u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_SETTITLE: u32 = 1144u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_SETTITLEA: u32 = 1135u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_SETTITLEW: u32 = 1144u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_SETWIZBUTTONS: u32 = 1136u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_SHOWWIZBUTTONS: u32 = 1162u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSM_UNCHANGED: u32 = 1133u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSNRET_INVALID: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSNRET_INVALID_NOCHANGEPAGE: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSNRET_MESSAGEHANDLED: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSNRET_NOERROR: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PSPCB_MESSAGE(pub u32);
pub const PSPCB_ADDREF: PSPCB_MESSAGE = PSPCB_MESSAGE(0u32);
pub const PSPCB_CREATE: PSPCB_MESSAGE = PSPCB_MESSAGE(2u32);
pub const PSPCB_RELEASE: PSPCB_MESSAGE = PSPCB_MESSAGE(1u32);
pub const PSPCB_SI_INITDIALOG: PSPCB_MESSAGE = PSPCB_MESSAGE(1025u32);
impl ::std::convert::From<u32> for PSPCB_MESSAGE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PSPCB_MESSAGE {
    type Abi = Self;
}
impl ::std::ops::BitOr for PSPCB_MESSAGE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PSPCB_MESSAGE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PSPCB_MESSAGE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PSPCB_MESSAGE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PSPCB_MESSAGE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSP_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSP_DLGINDIRECT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSP_HASHELP: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSP_HIDEHEADER: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSP_PREMATURE: u32 = 1024u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSP_RTLREADING: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSP_USECALLBACK: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSP_USEFUSIONCONTEXT: u32 = 16384u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSP_USEHEADERSUBTITLE: u32 = 8192u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSP_USEHEADERTITLE: u32 = 4096u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSP_USEHICON: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSP_USEICONID: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSP_USEREFPARENT: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSP_USETITLE: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSWIZBF_ELEVATIONREQUIRED: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSWIZB_BACK: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSWIZB_CANCEL: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSWIZB_DISABLEDFINISH: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSWIZB_FINISH: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSWIZB_NEXT: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSWIZB_RESTORE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const PSWIZB_SHOW: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PackTouchHitTestingProximityEvaluation(phittestinginput: *const TOUCH_HIT_TESTING_INPUT, pproximityeval: *const TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PackTouchHitTestingProximityEvaluation(phittestinginput: *const TOUCH_HIT_TESTING_INPUT, pproximityeval: *const TOUCH_HIT_TESTING_PROXIMITY_EVALUATION) -> super::super::Foundation::LRESULT;
        }
        ::std::mem::transmute(PackTouchHitTestingProximityEvaluation(::std::mem::transmute(phittestinginput), ::std::mem::transmute(pproximityeval)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn PropertySheetA(param0: *mut PROPSHEETHEADERA_V2) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropertySheetA(param0: *mut ::std::mem::ManuallyDrop<PROPSHEETHEADERA_V2>) -> isize;
        }
        ::std::mem::transmute(PropertySheetA(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn PropertySheetW(param0: *mut PROPSHEETHEADERW_V2) -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PropertySheetW(param0: *mut ::std::mem::ManuallyDrop<PROPSHEETHEADERW_V2>) -> isize;
        }
        ::std::mem::transmute(PropertySheetW(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBAB_ADDBAND: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBAB_AUTOSIZE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBBIM_BACKGROUND: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBBIM_CHEVRONLOCATION: u32 = 4096u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBBIM_CHEVRONSTATE: u32 = 8192u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBBIM_CHILD: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBBIM_CHILDSIZE: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBBIM_COLORS: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBBIM_HEADERSIZE: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBBIM_ID: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBBIM_IDEALSIZE: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBBIM_IMAGE: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBBIM_LPARAM: u32 = 1024u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBBIM_SIZE: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBBIM_STYLE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBBIM_TEXT: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBBS_BREAK: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBBS_CHILDEDGE: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBBS_FIXEDBMP: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBBS_FIXEDSIZE: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBBS_GRIPPERALWAYS: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBBS_HIDDEN: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBBS_HIDETITLE: u32 = 1024u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBBS_NOGRIPPER: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBBS_NOVERT: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBBS_TOPALIGN: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBBS_USECHEVRON: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBBS_VARIABLEHEIGHT: u32 = 64u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct RBHITTESTINFO {
    pub pt: super::super::Foundation::POINT,
    pub flags: u32,
    pub iBand: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl RBHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RBHITTESTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RBHITTESTINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RBHITTESTINFO").field("pt", &self.pt).field("flags", &self.flags).field("iBand", &self.iBand).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RBHITTESTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.pt == other.pt && self.flags == other.flags && self.iBand == other.iBand
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RBHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RBHITTESTINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBHT_CAPTION: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBHT_CHEVRON: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBHT_CLIENT: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBHT_GRABBER: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBHT_NOWHERE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBHT_SPLITTER: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBIM_IMAGELIST: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBSTR_CHANGERECT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBS_AUTOSIZE: u32 = 8192u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBS_BANDBORDERS: u32 = 1024u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBS_DBLCLKTOGGLE: u32 = 32768u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBS_FIXEDORDER: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBS_REGISTERDROP: u32 = 4096u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBS_TOOLTIPS: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBS_VARHEIGHT: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RBS_VERTICALGRIPPER: u32 = 16384u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_BEGINDRAG: u32 = 1048u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_DELETEBAND: u32 = 1026u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_DRAGMOVE: u32 = 1050u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_ENDDRAG: u32 = 1049u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_GETBANDBORDERS: u32 = 1058u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_GETBANDCOUNT: u32 = 1036u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_GETBANDINFO: u32 = 1052u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_GETBANDINFOA: u32 = 1053u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_GETBANDINFOW: u32 = 1052u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_GETBANDMARGINS: u32 = 1064u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_GETBARHEIGHT: u32 = 1051u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_GETBARINFO: u32 = 1027u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_GETBKCOLOR: u32 = 1044u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_GETCOLORSCHEME: u32 = 8195u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_GETDROPTARGET: u32 = 8196u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_GETEXTENDEDSTYLE: u32 = 1066u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_GETPALETTE: u32 = 1062u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_GETRECT: u32 = 1033u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_GETROWCOUNT: u32 = 1037u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_GETROWHEIGHT: u32 = 1038u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_GETTEXTCOLOR: u32 = 1046u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_GETTOOLTIPS: u32 = 1041u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_GETUNICODEFORMAT: u32 = 8198u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_HITTEST: u32 = 1032u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_IDTOINDEX: u32 = 1040u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_INSERTBAND: u32 = 1034u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_INSERTBANDA: u32 = 1025u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_INSERTBANDW: u32 = 1034u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_MAXIMIZEBAND: u32 = 1055u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_MINIMIZEBAND: u32 = 1054u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_MOVEBAND: u32 = 1063u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_PUSHCHEVRON: u32 = 1067u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_SETBANDINFO: u32 = 1035u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_SETBANDINFOA: u32 = 1030u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_SETBANDINFOW: u32 = 1035u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_SETBANDWIDTH: u32 = 1068u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_SETBARINFO: u32 = 1028u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_SETBKCOLOR: u32 = 1043u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_SETCOLORSCHEME: u32 = 8194u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_SETEXTENDEDSTYLE: u32 = 1065u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_SETPALETTE: u32 = 1061u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_SETPARENT: u32 = 1031u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_SETTEXTCOLOR: u32 = 1045u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_SETTOOLTIPS: u32 = 1042u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_SETUNICODEFORMAT: u32 = 8197u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_SETWINDOWTHEME: u32 = 8203u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_SHOWBAND: u32 = 1059u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RB_SIZETORECT: u32 = 1047u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct REBARBANDINFOA {
    pub cbSize: u32,
    pub fMask: u32,
    pub fStyle: u32,
    pub clrFore: u32,
    pub clrBack: u32,
    pub lpText: super::super::Foundation::PSTR,
    pub cch: u32,
    pub iImage: i32,
    pub hwndChild: super::super::Foundation::HWND,
    pub cxMinChild: u32,
    pub cyMinChild: u32,
    pub cx: u32,
    pub hbmBack: super::super::Graphics::Gdi::HBITMAP,
    pub wID: u32,
    pub cyChild: u32,
    pub cyMaxChild: u32,
    pub cyIntegral: u32,
    pub cxIdeal: u32,
    pub lParam: super::super::Foundation::LPARAM,
    pub cxHeader: u32,
    pub rcChevronLocation: super::super::Foundation::RECT,
    pub uChevronState: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl REBARBANDINFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for REBARBANDINFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for REBARBANDINFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("REBARBANDINFOA")
            .field("cbSize", &self.cbSize)
            .field("fMask", &self.fMask)
            .field("fStyle", &self.fStyle)
            .field("clrFore", &self.clrFore)
            .field("clrBack", &self.clrBack)
            .field("lpText", &self.lpText)
            .field("cch", &self.cch)
            .field("iImage", &self.iImage)
            .field("hwndChild", &self.hwndChild)
            .field("cxMinChild", &self.cxMinChild)
            .field("cyMinChild", &self.cyMinChild)
            .field("cx", &self.cx)
            .field("hbmBack", &self.hbmBack)
            .field("wID", &self.wID)
            .field("cyChild", &self.cyChild)
            .field("cyMaxChild", &self.cyMaxChild)
            .field("cyIntegral", &self.cyIntegral)
            .field("cxIdeal", &self.cxIdeal)
            .field("lParam", &self.lParam)
            .field("cxHeader", &self.cxHeader)
            .field("rcChevronLocation", &self.rcChevronLocation)
            .field("uChevronState", &self.uChevronState)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for REBARBANDINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.fMask == other.fMask
            && self.fStyle == other.fStyle
            && self.clrFore == other.clrFore
            && self.clrBack == other.clrBack
            && self.lpText == other.lpText
            && self.cch == other.cch
            && self.iImage == other.iImage
            && self.hwndChild == other.hwndChild
            && self.cxMinChild == other.cxMinChild
            && self.cyMinChild == other.cyMinChild
            && self.cx == other.cx
            && self.hbmBack == other.hbmBack
            && self.wID == other.wID
            && self.cyChild == other.cyChild
            && self.cyMaxChild == other.cyMaxChild
            && self.cyIntegral == other.cyIntegral
            && self.cxIdeal == other.cxIdeal
            && self.lParam == other.lParam
            && self.cxHeader == other.cxHeader
            && self.rcChevronLocation == other.rcChevronLocation
            && self.uChevronState == other.uChevronState
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for REBARBANDINFOA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for REBARBANDINFOA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
pub struct REBARBANDINFOW {
    pub cbSize: u32,
    pub fMask: u32,
    pub fStyle: u32,
    pub clrFore: u32,
    pub clrBack: u32,
    pub lpText: super::super::Foundation::PWSTR,
    pub cch: u32,
    pub iImage: i32,
    pub hwndChild: super::super::Foundation::HWND,
    pub cxMinChild: u32,
    pub cyMinChild: u32,
    pub cx: u32,
    pub hbmBack: super::super::Graphics::Gdi::HBITMAP,
    pub wID: u32,
    pub cyChild: u32,
    pub cyMaxChild: u32,
    pub cyIntegral: u32,
    pub cxIdeal: u32,
    pub lParam: super::super::Foundation::LPARAM,
    pub cxHeader: u32,
    pub rcChevronLocation: super::super::Foundation::RECT,
    pub uChevronState: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl REBARBANDINFOW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::default::Default for REBARBANDINFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::fmt::Debug for REBARBANDINFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("REBARBANDINFOW")
            .field("cbSize", &self.cbSize)
            .field("fMask", &self.fMask)
            .field("fStyle", &self.fStyle)
            .field("clrFore", &self.clrFore)
            .field("clrBack", &self.clrBack)
            .field("lpText", &self.lpText)
            .field("cch", &self.cch)
            .field("iImage", &self.iImage)
            .field("hwndChild", &self.hwndChild)
            .field("cxMinChild", &self.cxMinChild)
            .field("cyMinChild", &self.cyMinChild)
            .field("cx", &self.cx)
            .field("hbmBack", &self.hbmBack)
            .field("wID", &self.wID)
            .field("cyChild", &self.cyChild)
            .field("cyMaxChild", &self.cyMaxChild)
            .field("cyIntegral", &self.cyIntegral)
            .field("cxIdeal", &self.cxIdeal)
            .field("lParam", &self.lParam)
            .field("cxHeader", &self.cxHeader)
            .field("rcChevronLocation", &self.rcChevronLocation)
            .field("uChevronState", &self.uChevronState)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::PartialEq for REBARBANDINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.fMask == other.fMask
            && self.fStyle == other.fStyle
            && self.clrFore == other.clrFore
            && self.clrBack == other.clrBack
            && self.lpText == other.lpText
            && self.cch == other.cch
            && self.iImage == other.iImage
            && self.hwndChild == other.hwndChild
            && self.cxMinChild == other.cxMinChild
            && self.cyMinChild == other.cyMinChild
            && self.cx == other.cx
            && self.hbmBack == other.hbmBack
            && self.wID == other.wID
            && self.cyChild == other.cyChild
            && self.cyMaxChild == other.cyMaxChild
            && self.cyIntegral == other.cyIntegral
            && self.cxIdeal == other.cxIdeal
            && self.lParam == other.lParam
            && self.cxHeader == other.cxHeader
            && self.rcChevronLocation == other.rcChevronLocation
            && self.uChevronState == other.uChevronState
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::std::cmp::Eq for REBARBANDINFOW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
unsafe impl ::windows::runtime::Abi for REBARBANDINFOW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct REBARINFO {
    pub cbSize: u32,
    pub fMask: u32,
    pub himl: HIMAGELIST,
}
impl REBARINFO {}
impl ::std::default::Default for REBARINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for REBARINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("REBARINFO").field("cbSize", &self.cbSize).field("fMask", &self.fMask).field("himl", &self.himl).finish()
    }
}
impl ::std::cmp::PartialEq for REBARINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.fMask == other.fMask && self.himl == other.himl
    }
}
impl ::std::cmp::Eq for REBARINFO {}
unsafe impl ::windows::runtime::Abi for REBARINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const REPLACEDLGORD: u32 = 1541u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const RUNDLGORD: u32 = 1545u32;
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterPointerDeviceNotifications<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(window: Param0, notifyrange: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterPointerDeviceNotifications(window: super::super::Foundation::HWND, notifyrange: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RegisterPointerDeviceNotifications(window.into_param().abi(), notifyrange.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterTouchHitTestingWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, value: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterTouchHitTestingWindow(hwnd: super::super::Foundation::HWND, value: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RegisterTouchHitTestingWindow(hwnd.into_param().abi(), ::std::mem::transmute(value)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SBARS_SIZEGRIP: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SBARS_TOOLTIPS: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SBT_NOBORDERS: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SBT_NOTABPARSING: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SBT_OWNERDRAW: u32 = 4096u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SBT_POPOUT: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SBT_RTLREADING: u32 = 1024u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SBT_TOOLTIPS: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SB_GETBORDERS: u32 = 1031u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SB_GETICON: u32 = 1044u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SB_GETPARTS: u32 = 1030u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SB_GETRECT: u32 = 1034u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SB_GETTEXT: u32 = 1037u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SB_GETTEXTA: u32 = 1026u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SB_GETTEXTLENGTH: u32 = 1036u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SB_GETTEXTLENGTHA: u32 = 1027u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SB_GETTEXTLENGTHW: u32 = 1036u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SB_GETTEXTW: u32 = 1037u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SB_GETTIPTEXTA: u32 = 1042u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SB_GETTIPTEXTW: u32 = 1043u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SB_GETUNICODEFORMAT: u32 = 8198u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SB_ISSIMPLE: u32 = 1038u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SB_SETBKCOLOR: u32 = 8193u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SB_SETICON: u32 = 1039u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SB_SETMINHEIGHT: u32 = 1032u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SB_SETPARTS: u32 = 1028u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SB_SETTEXT: u32 = 1035u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SB_SETTEXTA: u32 = 1025u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SB_SETTEXTW: u32 = 1035u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SB_SETTIPTEXTA: u32 = 1040u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SB_SETTIPTEXTW: u32 = 1041u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SB_SETUNICODEFORMAT: u32 = 8197u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SB_SIMPLE: u32 = 1033u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const SB_SIMPLEID: u32 = 255u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SIZINGTYPE(pub i32);
pub const ST_TRUESIZE: SIZINGTYPE = SIZINGTYPE(0i32);
pub const ST_STRETCH: SIZINGTYPE = SIZINGTYPE(1i32);
pub const ST_TILE: SIZINGTYPE = SIZINGTYPE(2i32);
impl ::std::convert::From<i32> for SIZINGTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SIZINGTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SOFTWAREEXPLORERSTATES(pub i32);
pub const SPSE_NORMAL: SOFTWAREEXPLORERSTATES = SOFTWAREEXPLORERSTATES(1i32);
pub const SPSE_HOT: SOFTWAREEXPLORERSTATES = SOFTWAREEXPLORERSTATES(2i32);
pub const SPSE_SELECTED: SOFTWAREEXPLORERSTATES = SOFTWAREEXPLORERSTATES(3i32);
pub const SPSE_DISABLED: SOFTWAREEXPLORERSTATES = SOFTWAREEXPLORERSTATES(4i32);
pub const SPSE_FOCUSED: SOFTWAREEXPLORERSTATES = SOFTWAREEXPLORERSTATES(5i32);
impl ::std::convert::From<i32> for SOFTWAREEXPLORERSTATES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SOFTWAREEXPLORERSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct STARTPANELPARTS(pub i32);
pub const SPP_USERPANE: STARTPANELPARTS = STARTPANELPARTS(1i32);
pub const SPP_MOREPROGRAMS: STARTPANELPARTS = STARTPANELPARTS(2i32);
pub const SPP_MOREPROGRAMSARROW: STARTPANELPARTS = STARTPANELPARTS(3i32);
pub const SPP_PROGLIST: STARTPANELPARTS = STARTPANELPARTS(4i32);
pub const SPP_PROGLISTSEPARATOR: STARTPANELPARTS = STARTPANELPARTS(5i32);
pub const SPP_PLACESLIST: STARTPANELPARTS = STARTPANELPARTS(6i32);
pub const SPP_PLACESLISTSEPARATOR: STARTPANELPARTS = STARTPANELPARTS(7i32);
pub const SPP_LOGOFF: STARTPANELPARTS = STARTPANELPARTS(8i32);
pub const SPP_LOGOFFBUTTONS: STARTPANELPARTS = STARTPANELPARTS(9i32);
pub const SPP_USERPICTURE: STARTPANELPARTS = STARTPANELPARTS(10i32);
pub const SPP_PREVIEW: STARTPANELPARTS = STARTPANELPARTS(11i32);
pub const SPP_MOREPROGRAMSTAB: STARTPANELPARTS = STARTPANELPARTS(12i32);
pub const SPP_NSCHOST: STARTPANELPARTS = STARTPANELPARTS(13i32);
pub const SPP_SOFTWAREEXPLORER: STARTPANELPARTS = STARTPANELPARTS(14i32);
pub const SPP_OPENBOX: STARTPANELPARTS = STARTPANELPARTS(15i32);
pub const SPP_SEARCHVIEW: STARTPANELPARTS = STARTPANELPARTS(16i32);
pub const SPP_MOREPROGRAMSARROWBACK: STARTPANELPARTS = STARTPANELPARTS(17i32);
pub const SPP_TOPMATCH: STARTPANELPARTS = STARTPANELPARTS(18i32);
pub const SPP_LOGOFFSPLITBUTTONDROPDOWN: STARTPANELPARTS = STARTPANELPARTS(19i32);
impl ::std::convert::From<i32> for STARTPANELPARTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for STARTPANELPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct STATICPARTS(pub i32);
pub const STAT_TEXT: STATICPARTS = STATICPARTS(1i32);
impl ::std::convert::From<i32> for STATICPARTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for STATICPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const STD_COPY: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const STD_CUT: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const STD_DELETE: u32 = 5u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const STD_FILENEW: u32 = 6u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const STD_FILEOPEN: u32 = 7u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const STD_FILESAVE: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const STD_FIND: u32 = 12u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const STD_HELP: u32 = 11u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const STD_PASTE: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const STD_PRINT: u32 = 14u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const STD_PRINTPRE: u32 = 9u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const STD_PROPERTIES: u32 = 10u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const STD_REDOW: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const STD_REPLACE: u32 = 13u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const STD_UNDO: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn SetScrollInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(hwnd: Param0, nbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, lpsi: *const super::WindowsAndMessaging::SCROLLINFO, redraw: Param3) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetScrollInfo(hwnd: super::super::Foundation::HWND, nbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, lpsi: *const super::WindowsAndMessaging::SCROLLINFO, redraw: super::super::Foundation::BOOL) -> i32;
        }
        ::std::mem::transmute(SetScrollInfo(hwnd.into_param().abi(), ::std::mem::transmute(nbar), ::std::mem::transmute(lpsi), redraw.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn SetScrollPos<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(hwnd: Param0, nbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, npos: i32, bredraw: Param3) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetScrollPos(hwnd: super::super::Foundation::HWND, nbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, npos: i32, bredraw: super::super::Foundation::BOOL) -> i32;
        }
        ::std::mem::transmute(SetScrollPos(hwnd.into_param().abi(), ::std::mem::transmute(nbar), ::std::mem::transmute(npos), bredraw.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn SetScrollRange<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(hwnd: Param0, nbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, nminpos: i32, nmaxpos: i32, bredraw: Param4) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetScrollRange(hwnd: super::super::Foundation::HWND, nbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, nminpos: i32, nmaxpos: i32, bredraw: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetScrollRange(hwnd.into_param().abi(), ::std::mem::transmute(nbar), ::std::mem::transmute(nminpos), ::std::mem::transmute(nmaxpos), bredraw.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[inline]
pub unsafe fn SetThemeAppProperties(dwflags: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThemeAppProperties(dwflags: u32);
        }
        ::std::mem::transmute(SetThemeAppProperties(::std::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWindowFeedbackSetting<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, feedback: FEEDBACK_TYPE, dwflags: u32, size: u32, configuration: *const ::std::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetWindowFeedbackSetting(hwnd: super::super::Foundation::HWND, feedback: FEEDBACK_TYPE, dwflags: u32, size: u32, configuration: *const ::std::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetWindowFeedbackSetting(hwnd.into_param().abi(), ::std::mem::transmute(feedback), ::std::mem::transmute(dwflags), ::std::mem::transmute(size), ::std::mem::transmute(configuration)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWindowTheme<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hwnd: Param0, pszsubappname: Param1, pszsubidlist: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetWindowTheme(hwnd: super::super::Foundation::HWND, pszsubappname: super::super::Foundation::PWSTR, pszsubidlist: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        SetWindowTheme(hwnd.into_param().abi(), pszsubappname.into_param().abi(), pszsubidlist.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWindowThemeAttribute<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, eattribute: WINDOWTHEMEATTRIBUTETYPE, pvattribute: *const ::std::ffi::c_void, cbattribute: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetWindowThemeAttribute(hwnd: super::super::Foundation::HWND, eattribute: WINDOWTHEMEATTRIBUTETYPE, pvattribute: *const ::std::ffi::c_void, cbattribute: u32) -> ::windows::runtime::HRESULT;
        }
        SetWindowThemeAttribute(hwnd.into_param().abi(), ::std::mem::transmute(eattribute), ::std::mem::transmute(pvattribute), ::std::mem::transmute(cbattribute)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ShowHideMenuCtl<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, uflags: usize, lpinfo: *const i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowHideMenuCtl(hwnd: super::super::Foundation::HWND, uflags: usize, lpinfo: *const i32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ShowHideMenuCtl(hwnd.into_param().abi(), ::std::mem::transmute(uflags), ::std::mem::transmute(lpinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn ShowScrollBar<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(hwnd: Param0, wbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, bshow: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ShowScrollBar(hwnd: super::super::Foundation::HWND, wbar: super::WindowsAndMessaging::SCROLLBAR_CONSTANTS, bshow: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ShowScrollBar(hwnd.into_param().abi(), ::std::mem::transmute(wbar), bshow.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Str_SetPtrW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(ppsz: *mut super::super::Foundation::PWSTR, psz: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Str_SetPtrW(ppsz: *mut super::super::Foundation::PWSTR, psz: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(Str_SetPtrW(::std::mem::transmute(ppsz), psz.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TASKBANDPARTS(pub i32);
pub const TDP_GROUPCOUNT: TASKBANDPARTS = TASKBANDPARTS(1i32);
pub const TDP_FLASHBUTTON: TASKBANDPARTS = TASKBANDPARTS(2i32);
pub const TDP_FLASHBUTTONGROUPMENU: TASKBANDPARTS = TASKBANDPARTS(3i32);
impl ::std::convert::From<i32> for TASKBANDPARTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TASKBANDPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TASKBARPARTS(pub i32);
pub const TBP_BACKGROUNDBOTTOM: TASKBARPARTS = TASKBARPARTS(1i32);
pub const TBP_BACKGROUNDRIGHT: TASKBARPARTS = TASKBARPARTS(2i32);
pub const TBP_BACKGROUNDTOP: TASKBARPARTS = TASKBARPARTS(3i32);
pub const TBP_BACKGROUNDLEFT: TASKBARPARTS = TASKBARPARTS(4i32);
pub const TBP_SIZINGBARBOTTOM: TASKBARPARTS = TASKBARPARTS(5i32);
pub const TBP_SIZINGBARRIGHT: TASKBARPARTS = TASKBARPARTS(6i32);
pub const TBP_SIZINGBARTOP: TASKBARPARTS = TASKBARPARTS(7i32);
pub const TBP_SIZINGBARLEFT: TASKBARPARTS = TASKBARPARTS(8i32);
impl ::std::convert::From<i32> for TASKBARPARTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TASKBARPARTS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::clone::Clone for TASKDIALOGCONFIG {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
pub struct TASKDIALOGCONFIG {
    pub cbSize: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub hInstance: super::super::Foundation::HINSTANCE,
    pub dwFlags: TASKDIALOG_FLAGS,
    pub dwCommonButtons: TASKDIALOG_COMMON_BUTTON_FLAGS,
    pub pszWindowTitle: super::super::Foundation::PWSTR,
    pub Anonymous1: TASKDIALOGCONFIG_0,
    pub pszMainInstruction: super::super::Foundation::PWSTR,
    pub pszContent: super::super::Foundation::PWSTR,
    pub cButtons: u32,
    pub pButtons: *mut TASKDIALOG_BUTTON,
    pub nDefaultButton: i32,
    pub cRadioButtons: u32,
    pub pRadioButtons: *mut TASKDIALOG_BUTTON,
    pub nDefaultRadioButton: i32,
    pub pszVerificationText: super::super::Foundation::PWSTR,
    pub pszExpandedInformation: super::super::Foundation::PWSTR,
    pub pszExpandedControlText: super::super::Foundation::PWSTR,
    pub pszCollapsedControlText: super::super::Foundation::PWSTR,
    pub Anonymous2: TASKDIALOGCONFIG_1,
    pub pszFooter: super::super::Foundation::PWSTR,
    pub pfCallback: ::std::option::Option<PFTASKDIALOGCALLBACK>,
    pub lpCallbackData: isize,
    pub cxWidth: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl TASKDIALOGCONFIG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for TASKDIALOGCONFIG {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for TASKDIALOGCONFIG {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for TASKDIALOGCONFIG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for TASKDIALOGCONFIG {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
pub union TASKDIALOGCONFIG_0 {
    pub hMainIcon: super::WindowsAndMessaging::HICON,
    pub pszMainIcon: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl TASKDIALOGCONFIG_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for TASKDIALOGCONFIG_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for TASKDIALOGCONFIG_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for TASKDIALOGCONFIG_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for TASKDIALOGCONFIG_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
pub union TASKDIALOGCONFIG_1 {
    pub hFooterIcon: super::WindowsAndMessaging::HICON,
    pub pszFooterIcon: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl TASKDIALOGCONFIG_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::default::Default for TASKDIALOGCONFIG_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::PartialEq for TASKDIALOGCONFIG_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::std::cmp::Eq for TASKDIALOGCONFIG_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
unsafe impl ::windows::runtime::Abi for TASKDIALOGCONFIG_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct TASKDIALOG_BUTTON {
    pub nButtonID: i32,
    pub pszButtonText: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl TASKDIALOG_BUTTON {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TASKDIALOG_BUTTON {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TASKDIALOG_BUTTON {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TASKDIALOG_BUTTON {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TASKDIALOG_BUTTON {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TASKDIALOG_COMMON_BUTTON_FLAGS(pub i32);
pub const TDCBF_OK_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(1i32);
pub const TDCBF_YES_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(2i32);
pub const TDCBF_NO_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(4i32);
pub const TDCBF_CANCEL_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(8i32);
pub const TDCBF_RETRY_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(16i32);
pub const TDCBF_CLOSE_BUTTON: TASKDIALOG_COMMON_BUTTON_FLAGS = TASKDIALOG_COMMON_BUTTON_FLAGS(32i32);
impl ::std::convert::From<i32> for TASKDIALOG_COMMON_BUTTON_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TASKDIALOG_COMMON_BUTTON_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TASKDIALOG_ELEMENTS(pub i32);
pub const TDE_CONTENT: TASKDIALOG_ELEMENTS = TASKDIALOG_ELEMENTS(0i32);
pub const TDE_EXPANDED_INFORMATION: TASKDIALOG_ELEMENTS = TASKDIALOG_ELEMENTS(1i32);
pub const TDE_FOOTER: TASKDIALOG_ELEMENTS = TASKDIALOG_ELEMENTS(2i32);
pub const TDE_MAIN_INSTRUCTION: TASKDIALOG_ELEMENTS = TASKDIALOG_ELEMENTS(3i32);
impl ::std::convert::From<i32> for TASKDIALOG_ELEMENTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TASKDIALOG_ELEMENTS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TASKDIALOG_FLAGS(pub i32);
pub const TDF_ENABLE_HYPERLINKS: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(1i32);
pub const TDF_USE_HICON_MAIN: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(2i32);
pub const TDF_USE_HICON_FOOTER: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(4i32);
pub const TDF_ALLOW_DIALOG_CANCELLATION: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(8i32);
pub const TDF_USE_COMMAND_LINKS: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(16i32);
pub const TDF_USE_COMMAND_LINKS_NO_ICON: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(32i32);
pub const TDF_EXPAND_FOOTER_AREA: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(64i32);
pub const TDF_EXPANDED_BY_DEFAULT: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(128i32);
pub const TDF_VERIFICATION_FLAG_CHECKED: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(256i32);
pub const TDF_SHOW_PROGRESS_BAR: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(512i32);
pub const TDF_SHOW_MARQUEE_PROGRESS_BAR: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(1024i32);
pub const TDF_CALLBACK_TIMER: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(2048i32);
pub const TDF_POSITION_RELATIVE_TO_WINDOW: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(4096i32);
pub const TDF_RTL_LAYOUT: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(8192i32);
pub const TDF_NO_DEFAULT_RADIO_BUTTON: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(16384i32);
pub const TDF_CAN_BE_MINIMIZED: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(32768i32);
pub const TDF_NO_SET_FOREGROUND: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(65536i32);
pub const TDF_SIZE_TO_CONTENT: TASKDIALOG_FLAGS = TASKDIALOG_FLAGS(16777216i32);
impl ::std::convert::From<i32> for TASKDIALOG_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TASKDIALOG_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TASKDIALOG_ICON_ELEMENTS(pub i32);
pub const TDIE_ICON_MAIN: TASKDIALOG_ICON_ELEMENTS = TASKDIALOG_ICON_ELEMENTS(0i32);
pub const TDIE_ICON_FOOTER: TASKDIALOG_ICON_ELEMENTS = TASKDIALOG_ICON_ELEMENTS(1i32);
impl ::std::convert::From<i32> for TASKDIALOG_ICON_ELEMENTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TASKDIALOG_ICON_ELEMENTS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TASKDIALOG_MESSAGES(pub i32);
pub const TDM_NAVIGATE_PAGE: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1125i32);
pub const TDM_CLICK_BUTTON: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1126i32);
pub const TDM_SET_MARQUEE_PROGRESS_BAR: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1127i32);
pub const TDM_SET_PROGRESS_BAR_STATE: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1128i32);
pub const TDM_SET_PROGRESS_BAR_RANGE: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1129i32);
pub const TDM_SET_PROGRESS_BAR_POS: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1130i32);
pub const TDM_SET_PROGRESS_BAR_MARQUEE: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1131i32);
pub const TDM_SET_ELEMENT_TEXT: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1132i32);
pub const TDM_CLICK_RADIO_BUTTON: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1134i32);
pub const TDM_ENABLE_BUTTON: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1135i32);
pub const TDM_ENABLE_RADIO_BUTTON: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1136i32);
pub const TDM_CLICK_VERIFICATION: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1137i32);
pub const TDM_UPDATE_ELEMENT_TEXT: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1138i32);
pub const TDM_SET_BUTTON_ELEVATION_REQUIRED_STATE: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1139i32);
pub const TDM_UPDATE_ICON: TASKDIALOG_MESSAGES = TASKDIALOG_MESSAGES(1140i32);
impl ::std::convert::From<i32> for TASKDIALOG_MESSAGES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TASKDIALOG_MESSAGES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TASKDIALOG_NOTIFICATIONS(pub i32);
pub const TDN_CREATED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(0i32);
pub const TDN_NAVIGATED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(1i32);
pub const TDN_BUTTON_CLICKED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(2i32);
pub const TDN_HYPERLINK_CLICKED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(3i32);
pub const TDN_TIMER: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(4i32);
pub const TDN_DESTROYED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(5i32);
pub const TDN_RADIO_BUTTON_CLICKED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(6i32);
pub const TDN_DIALOG_CONSTRUCTED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(7i32);
pub const TDN_VERIFICATION_CLICKED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(8i32);
pub const TDN_HELP: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(9i32);
pub const TDN_EXPANDO_BUTTON_CLICKED: TASKDIALOG_NOTIFICATIONS = TASKDIALOG_NOTIFICATIONS(10i32);
impl ::std::convert::From<i32> for TASKDIALOG_NOTIFICATIONS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TASKDIALOG_NOTIFICATIONS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct TA_CUBIC_BEZIER {
    pub header: TA_TIMINGFUNCTION,
    pub rX0: f32,
    pub rY0: f32,
    pub rX1: f32,
    pub rY1: f32,
}
impl TA_CUBIC_BEZIER {}
impl ::std::default::Default for TA_CUBIC_BEZIER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TA_CUBIC_BEZIER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TA_CUBIC_BEZIER").field("header", &self.header).field("rX0", &self.rX0).field("rY0", &self.rY0).field("rX1", &self.rX1).field("rY1", &self.rY1).finish()
    }
}
impl ::std::cmp::PartialEq for TA_CUBIC_BEZIER {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.rX0 == other.rX0 && self.rY0 == other.rY0 && self.rX1 == other.rX1 && self.rY1 == other.rY1
    }
}
impl ::std::cmp::Eq for TA_CUBIC_BEZIER {}
unsafe impl ::windows::runtime::Abi for TA_CUBIC_BEZIER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TA_PROPERTY(pub i32);
pub const TAP_FLAGS: TA_PROPERTY = TA_PROPERTY(0i32);
pub const TAP_TRANSFORMCOUNT: TA_PROPERTY = TA_PROPERTY(1i32);
pub const TAP_STAGGERDELAY: TA_PROPERTY = TA_PROPERTY(2i32);
pub const TAP_STAGGERDELAYCAP: TA_PROPERTY = TA_PROPERTY(3i32);
pub const TAP_STAGGERDELAYFACTOR: TA_PROPERTY = TA_PROPERTY(4i32);
pub const TAP_ZORDER: TA_PROPERTY = TA_PROPERTY(5i32);
impl ::std::convert::From<i32> for TA_PROPERTY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TA_PROPERTY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TA_PROPERTY_FLAG(pub u32);
pub const TAPF_NONE: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(0u32);
pub const TAPF_HASSTAGGER: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(1u32);
pub const TAPF_ISRTLAWARE: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(2u32);
pub const TAPF_ALLOWCOLLECTION: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(4u32);
pub const TAPF_HASBACKGROUND: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(8u32);
pub const TAPF_HASPERSPECTIVE: TA_PROPERTY_FLAG = TA_PROPERTY_FLAG(16u32);
impl ::std::convert::From<u32> for TA_PROPERTY_FLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TA_PROPERTY_FLAG {
    type Abi = Self;
}
impl ::std::ops::BitOr for TA_PROPERTY_FLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for TA_PROPERTY_FLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for TA_PROPERTY_FLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for TA_PROPERTY_FLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for TA_PROPERTY_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct TA_TIMINGFUNCTION {
    pub eTimingFunctionType: TA_TIMINGFUNCTION_TYPE,
}
impl TA_TIMINGFUNCTION {}
impl ::std::default::Default for TA_TIMINGFUNCTION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TA_TIMINGFUNCTION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TA_TIMINGFUNCTION").field("eTimingFunctionType", &self.eTimingFunctionType).finish()
    }
}
impl ::std::cmp::PartialEq for TA_TIMINGFUNCTION {
    fn eq(&self, other: &Self) -> bool {
        self.eTimingFunctionType == other.eTimingFunctionType
    }
}
impl ::std::cmp::Eq for TA_TIMINGFUNCTION {}
unsafe impl ::windows::runtime::Abi for TA_TIMINGFUNCTION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TA_TIMINGFUNCTION_TYPE(pub i32);
pub const TTFT_UNDEFINED: TA_TIMINGFUNCTION_TYPE = TA_TIMINGFUNCTION_TYPE(0i32);
pub const TTFT_CUBIC_BEZIER: TA_TIMINGFUNCTION_TYPE = TA_TIMINGFUNCTION_TYPE(1i32);
impl ::std::convert::From<i32> for TA_TIMINGFUNCTION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TA_TIMINGFUNCTION_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct TA_TRANSFORM {
    pub eTransformType: TA_TRANSFORM_TYPE,
    pub dwTimingFunctionId: u32,
    pub dwStartTime: u32,
    pub dwDurationTime: u32,
    pub eFlags: TA_TRANSFORM_FLAG,
}
impl TA_TRANSFORM {}
impl ::std::default::Default for TA_TRANSFORM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TA_TRANSFORM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TA_TRANSFORM").field("eTransformType", &self.eTransformType).field("dwTimingFunctionId", &self.dwTimingFunctionId).field("dwStartTime", &self.dwStartTime).field("dwDurationTime", &self.dwDurationTime).field("eFlags", &self.eFlags).finish()
    }
}
impl ::std::cmp::PartialEq for TA_TRANSFORM {
    fn eq(&self, other: &Self) -> bool {
        self.eTransformType == other.eTransformType && self.dwTimingFunctionId == other.dwTimingFunctionId && self.dwStartTime == other.dwStartTime && self.dwDurationTime == other.dwDurationTime && self.eFlags == other.eFlags
    }
}
impl ::std::cmp::Eq for TA_TRANSFORM {}
unsafe impl ::windows::runtime::Abi for TA_TRANSFORM {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct TA_TRANSFORM_2D {
    pub header: TA_TRANSFORM,
    pub rX: f32,
    pub rY: f32,
    pub rInitialX: f32,
    pub rInitialY: f32,
    pub rOriginX: f32,
    pub rOriginY: f32,
}
impl TA_TRANSFORM_2D {}
impl ::std::default::Default for TA_TRANSFORM_2D {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TA_TRANSFORM_2D {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TA_TRANSFORM_2D").field("header", &self.header).field("rX", &self.rX).field("rY", &self.rY).field("rInitialX", &self.rInitialX).field("rInitialY", &self.rInitialY).field("rOriginX", &self.rOriginX).field("rOriginY", &self.rOriginY).finish()
    }
}
impl ::std::cmp::PartialEq for TA_TRANSFORM_2D {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.rX == other.rX && self.rY == other.rY && self.rInitialX == other.rInitialX && self.rInitialY == other.rInitialY && self.rOriginX == other.rOriginX && self.rOriginY == other.rOriginY
    }
}
impl ::std::cmp::Eq for TA_TRANSFORM_2D {}
unsafe impl ::windows::runtime::Abi for TA_TRANSFORM_2D {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct TA_TRANSFORM_CLIP {
    pub header: TA_TRANSFORM,
    pub rLeft: f32,
    pub rTop: f32,
    pub rRight: f32,
    pub rBottom: f32,
    pub rInitialLeft: f32,
    pub rInitialTop: f32,
    pub rInitialRight: f32,
    pub rInitialBottom: f32,
}
impl TA_TRANSFORM_CLIP {}
impl ::std::default::Default for TA_TRANSFORM_CLIP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TA_TRANSFORM_CLIP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TA_TRANSFORM_CLIP")
            .field("header", &self.header)
            .field("rLeft", &self.rLeft)
            .field("rTop", &self.rTop)
            .field("rRight", &self.rRight)
            .field("rBottom", &self.rBottom)
            .field("rInitialLeft", &self.rInitialLeft)
            .field("rInitialTop", &self.rInitialTop)
            .field("rInitialRight", &self.rInitialRight)
            .field("rInitialBottom", &self.rInitialBottom)
            .finish()
    }
}
impl ::std::cmp::PartialEq for TA_TRANSFORM_CLIP {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.rLeft == other.rLeft && self.rTop == other.rTop && self.rRight == other.rRight && self.rBottom == other.rBottom && self.rInitialLeft == other.rInitialLeft && self.rInitialTop == other.rInitialTop && self.rInitialRight == other.rInitialRight && self.rInitialBottom == other.rInitialBottom
    }
}
impl ::std::cmp::Eq for TA_TRANSFORM_CLIP {}
unsafe impl ::windows::runtime::Abi for TA_TRANSFORM_CLIP {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TA_TRANSFORM_FLAG(pub i32);
pub const TATF_NONE: TA_TRANSFORM_FLAG = TA_TRANSFORM_FLAG(0i32);
pub const TATF_TARGETVALUES_USER: TA_TRANSFORM_FLAG = TA_TRANSFORM_FLAG(1i32);
pub const TATF_HASINITIALVALUES: TA_TRANSFORM_FLAG = TA_TRANSFORM_FLAG(2i32);
pub const TATF_HASORIGINVALUES: TA_TRANSFORM_FLAG = TA_TRANSFORM_FLAG(4i32);
impl ::std::convert::From<i32> for TA_TRANSFORM_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TA_TRANSFORM_FLAG {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct TA_TRANSFORM_OPACITY {
    pub header: TA_TRANSFORM,
    pub rOpacity: f32,
    pub rInitialOpacity: f32,
}
impl TA_TRANSFORM_OPACITY {}
impl ::std::default::Default for TA_TRANSFORM_OPACITY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TA_TRANSFORM_OPACITY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TA_TRANSFORM_OPACITY").field("header", &self.header).field("rOpacity", &self.rOpacity).field("rInitialOpacity", &self.rInitialOpacity).finish()
    }
}
impl ::std::cmp::PartialEq for TA_TRANSFORM_OPACITY {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.rOpacity == other.rOpacity && self.rInitialOpacity == other.rInitialOpacity
    }
}
impl ::std::cmp::Eq for TA_TRANSFORM_OPACITY {}
unsafe impl ::windows::runtime::Abi for TA_TRANSFORM_OPACITY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TA_TRANSFORM_TYPE(pub i32);
pub const TATT_TRANSLATE_2D: TA_TRANSFORM_TYPE = TA_TRANSFORM_TYPE(0i32);
pub const TATT_SCALE_2D: TA_TRANSFORM_TYPE = TA_TRANSFORM_TYPE(1i32);
pub const TATT_OPACITY: TA_TRANSFORM_TYPE = TA_TRANSFORM_TYPE(2i32);
pub const TATT_CLIP: TA_TRANSFORM_TYPE = TA_TRANSFORM_TYPE(3i32);
impl ::std::convert::From<i32> for TA_TRANSFORM_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TA_TRANSFORM_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct TBADDBITMAP {
    pub hInst: super::super::Foundation::HINSTANCE,
    pub nID: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl TBADDBITMAP {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TBADDBITMAP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TBADDBITMAP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TBADDBITMAP").field("hInst", &self.hInst).field("nID", &self.nID).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TBADDBITMAP {
    fn eq(&self, other: &Self) -> bool {
        self.hInst == other.hInst && self.nID == other.nID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TBADDBITMAP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TBADDBITMAP {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBBF_LARGE: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct TBBUTTON {
    pub iBitmap: i32,
    pub idCommand: i32,
    pub fsState: u8,
    pub fsStyle: u8,
    pub bReserved: [u8; 6],
    pub dwData: usize,
    pub iString: isize,
}
impl TBBUTTON {}
impl ::std::default::Default for TBBUTTON {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TBBUTTON {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TBBUTTON").field("iBitmap", &self.iBitmap).field("idCommand", &self.idCommand).field("fsState", &self.fsState).field("fsStyle", &self.fsStyle).field("bReserved", &self.bReserved).field("dwData", &self.dwData).field("iString", &self.iString).finish()
    }
}
impl ::std::cmp::PartialEq for TBBUTTON {
    fn eq(&self, other: &Self) -> bool {
        self.iBitmap == other.iBitmap && self.idCommand == other.idCommand && self.fsState == other.fsState && self.fsStyle == other.fsStyle && self.bReserved == other.bReserved && self.dwData == other.dwData && self.iString == other.iString
    }
}
impl ::std::cmp::Eq for TBBUTTON {}
unsafe impl ::windows::runtime::Abi for TBBUTTON {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct TBBUTTON {
    pub iBitmap: i32,
    pub idCommand: i32,
    pub fsState: u8,
    pub fsStyle: u8,
    pub bReserved: [u8; 2],
    pub dwData: usize,
    pub iString: isize,
}
impl TBBUTTON {}
impl ::std::default::Default for TBBUTTON {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TBBUTTON {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TBBUTTON").field("iBitmap", &self.iBitmap).field("idCommand", &self.idCommand).field("fsState", &self.fsState).field("fsStyle", &self.fsStyle).field("bReserved", &self.bReserved).field("dwData", &self.dwData).field("iString", &self.iString).finish()
    }
}
impl ::std::cmp::PartialEq for TBBUTTON {
    fn eq(&self, other: &Self) -> bool {
        self.iBitmap == other.iBitmap && self.idCommand == other.idCommand && self.fsState == other.fsState && self.fsStyle == other.fsStyle && self.bReserved == other.bReserved && self.dwData == other.dwData && self.iString == other.iString
    }
}
impl ::std::cmp::Eq for TBBUTTON {}
unsafe impl ::windows::runtime::Abi for TBBUTTON {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct TBBUTTONINFOA {
    pub cbSize: u32,
    pub dwMask: TBBUTTONINFOW_MASK,
    pub idCommand: i32,
    pub iImage: i32,
    pub fsState: u8,
    pub fsStyle: u8,
    pub cx: u16,
    pub lParam: usize,
    pub pszText: super::super::Foundation::PSTR,
    pub cchText: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl TBBUTTONINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TBBUTTONINFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TBBUTTONINFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TBBUTTONINFOA")
            .field("cbSize", &self.cbSize)
            .field("dwMask", &self.dwMask)
            .field("idCommand", &self.idCommand)
            .field("iImage", &self.iImage)
            .field("fsState", &self.fsState)
            .field("fsStyle", &self.fsStyle)
            .field("cx", &self.cx)
            .field("lParam", &self.lParam)
            .field("pszText", &self.pszText)
            .field("cchText", &self.cchText)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TBBUTTONINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwMask == other.dwMask && self.idCommand == other.idCommand && self.iImage == other.iImage && self.fsState == other.fsState && self.fsStyle == other.fsStyle && self.cx == other.cx && self.lParam == other.lParam && self.pszText == other.pszText && self.cchText == other.cchText
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TBBUTTONINFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TBBUTTONINFOA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct TBBUTTONINFOW {
    pub cbSize: u32,
    pub dwMask: TBBUTTONINFOW_MASK,
    pub idCommand: i32,
    pub iImage: i32,
    pub fsState: u8,
    pub fsStyle: u8,
    pub cx: u16,
    pub lParam: usize,
    pub pszText: super::super::Foundation::PWSTR,
    pub cchText: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl TBBUTTONINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TBBUTTONINFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TBBUTTONINFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TBBUTTONINFOW")
            .field("cbSize", &self.cbSize)
            .field("dwMask", &self.dwMask)
            .field("idCommand", &self.idCommand)
            .field("iImage", &self.iImage)
            .field("fsState", &self.fsState)
            .field("fsStyle", &self.fsStyle)
            .field("cx", &self.cx)
            .field("lParam", &self.lParam)
            .field("pszText", &self.pszText)
            .field("cchText", &self.cchText)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TBBUTTONINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwMask == other.dwMask && self.idCommand == other.idCommand && self.iImage == other.iImage && self.fsState == other.fsState && self.fsStyle == other.fsStyle && self.cx == other.cx && self.lParam == other.lParam && self.pszText == other.pszText && self.cchText == other.cchText
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TBBUTTONINFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TBBUTTONINFOW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TBBUTTONINFOW_MASK(pub u32);
pub const TBIF_BYINDEX: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(2147483648u32);
pub const TBIF_COMMAND: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(32u32);
pub const TBIF_IMAGE: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(1u32);
pub const TBIF_LPARAM: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(16u32);
pub const TBIF_SIZE: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(64u32);
pub const TBIF_STATE: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(4u32);
pub const TBIF_STYLE: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(8u32);
pub const TBIF_TEXT: TBBUTTONINFOW_MASK = TBBUTTONINFOW_MASK(2u32);
impl ::std::convert::From<u32> for TBBUTTONINFOW_MASK {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TBBUTTONINFOW_MASK {
    type Abi = Self;
}
impl ::std::ops::BitOr for TBBUTTONINFOW_MASK {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for TBBUTTONINFOW_MASK {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for TBBUTTONINFOW_MASK {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for TBBUTTONINFOW_MASK {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for TBBUTTONINFOW_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBCDRF_BLENDICON: u32 = 2097152u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBCDRF_HILITEHOTTRACK: u32 = 131072u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBCDRF_NOBACKGROUND: u32 = 4194304u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBCDRF_NOEDGES: u32 = 65536u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBCDRF_NOETCHEDEFFECT: u32 = 1048576u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBCDRF_NOMARK: u32 = 524288u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBCDRF_NOOFFSET: u32 = 262144u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBCDRF_USECDCOLORS: u32 = 8388608u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBCD_CHANNEL: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBCD_THUMB: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBCD_TICS: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBDDRET_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBDDRET_NODEFAULT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBDDRET_TREATPRESSED: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct TBINSERTMARK {
    pub iButton: i32,
    pub dwFlags: TBINSERTMARK_FLAGS,
}
impl TBINSERTMARK {}
impl ::std::default::Default for TBINSERTMARK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TBINSERTMARK {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TBINSERTMARK").field("iButton", &self.iButton).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::std::cmp::PartialEq for TBINSERTMARK {
    fn eq(&self, other: &Self) -> bool {
        self.iButton == other.iButton && self.dwFlags == other.dwFlags
    }
}
impl ::std::cmp::Eq for TBINSERTMARK {}
unsafe impl ::windows::runtime::Abi for TBINSERTMARK {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TBINSERTMARK_FLAGS(pub u32);
pub const TBIMHT_NONE: TBINSERTMARK_FLAGS = TBINSERTMARK_FLAGS(0u32);
pub const TBIMHT_AFTER: TBINSERTMARK_FLAGS = TBINSERTMARK_FLAGS(1u32);
pub const TBIMHT_BACKGROUND: TBINSERTMARK_FLAGS = TBINSERTMARK_FLAGS(2u32);
impl ::std::convert::From<u32> for TBINSERTMARK_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TBINSERTMARK_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for TBINSERTMARK_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for TBINSERTMARK_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for TBINSERTMARK_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for TBINSERTMARK_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for TBINSERTMARK_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct TBMETRICS {
    pub cbSize: u32,
    pub dwMask: u32,
    pub cxPad: i32,
    pub cyPad: i32,
    pub cxBarPad: i32,
    pub cyBarPad: i32,
    pub cxButtonSpacing: i32,
    pub cyButtonSpacing: i32,
}
impl TBMETRICS {}
impl ::std::default::Default for TBMETRICS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TBMETRICS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TBMETRICS")
            .field("cbSize", &self.cbSize)
            .field("dwMask", &self.dwMask)
            .field("cxPad", &self.cxPad)
            .field("cyPad", &self.cyPad)
            .field("cxBarPad", &self.cxBarPad)
            .field("cyBarPad", &self.cyBarPad)
            .field("cxButtonSpacing", &self.cxButtonSpacing)
            .field("cyButtonSpacing", &self.cyButtonSpacing)
            .finish()
    }
}
impl ::std::cmp::PartialEq for TBMETRICS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwMask == other.dwMask && self.cxPad == other.cxPad && self.cyPad == other.cyPad && self.cxBarPad == other.cxBarPad && self.cyBarPad == other.cyBarPad && self.cxButtonSpacing == other.cxButtonSpacing && self.cyButtonSpacing == other.cyButtonSpacing
    }
}
impl ::std::cmp::Eq for TBMETRICS {}
unsafe impl ::windows::runtime::Abi for TBMETRICS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBMF_BARPAD: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBMF_BUTTONSPACING: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBMF_PAD: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_CLEARSEL: u32 = 1043u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_CLEARTICS: u32 = 1033u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_GETBUDDY: u32 = 1057u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_GETCHANNELRECT: u32 = 1050u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_GETLINESIZE: u32 = 1048u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_GETNUMTICS: u32 = 1040u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_GETPAGESIZE: u32 = 1046u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_GETPTICS: u32 = 1038u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_GETRANGEMAX: u32 = 1026u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_GETRANGEMIN: u32 = 1025u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_GETSELEND: u32 = 1042u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_GETSELSTART: u32 = 1041u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_GETTHUMBLENGTH: u32 = 1052u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_GETTHUMBRECT: u32 = 1049u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_GETTIC: u32 = 1027u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_GETTICPOS: u32 = 1039u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_GETTOOLTIPS: u32 = 1054u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_GETUNICODEFORMAT: u32 = 8198u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_SETBUDDY: u32 = 1056u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_SETLINESIZE: u32 = 1047u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_SETPAGESIZE: u32 = 1045u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_SETPOS: u32 = 1029u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_SETPOSNOTIFY: u32 = 1058u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_SETRANGE: u32 = 1030u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_SETRANGEMAX: u32 = 1032u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_SETRANGEMIN: u32 = 1031u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_SETSEL: u32 = 1034u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_SETSELEND: u32 = 1036u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_SETSELSTART: u32 = 1035u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_SETTHUMBLENGTH: u32 = 1051u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_SETTIC: u32 = 1028u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_SETTICFREQ: u32 = 1044u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_SETTIPSIDE: u32 = 1055u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_SETTOOLTIPS: u32 = 1053u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBM_SETUNICODEFORMAT: u32 = 8197u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBNRF_ENDCUSTOMIZE: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBNRF_HIDEHELP: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct TBREPLACEBITMAP {
    pub hInstOld: super::super::Foundation::HINSTANCE,
    pub nIDOld: usize,
    pub hInstNew: super::super::Foundation::HINSTANCE,
    pub nIDNew: usize,
    pub nButtons: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl TBREPLACEBITMAP {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TBREPLACEBITMAP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TBREPLACEBITMAP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TBREPLACEBITMAP").field("hInstOld", &self.hInstOld).field("nIDOld", &self.nIDOld).field("hInstNew", &self.hInstNew).field("nIDNew", &self.nIDNew).field("nButtons", &self.nButtons).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TBREPLACEBITMAP {
    fn eq(&self, other: &Self) -> bool {
        self.hInstOld == other.hInstOld && self.nIDOld == other.nIDOld && self.hInstNew == other.hInstNew && self.nIDNew == other.nIDNew && self.nButtons == other.nButtons
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TBREPLACEBITMAP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TBREPLACEBITMAP {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_System_Registry`*"]
pub struct TBSAVEPARAMSA {
    pub hkr: super::super::System::Registry::HKEY,
    pub pszSubKey: super::super::Foundation::PSTR,
    pub pszValueName: super::super::Foundation::PSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl TBSAVEPARAMSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::std::default::Default for TBSAVEPARAMSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::std::fmt::Debug for TBSAVEPARAMSA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TBSAVEPARAMSA").field("hkr", &self.hkr).field("pszSubKey", &self.pszSubKey).field("pszValueName", &self.pszValueName).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::std::cmp::PartialEq for TBSAVEPARAMSA {
    fn eq(&self, other: &Self) -> bool {
        self.hkr == other.hkr && self.pszSubKey == other.pszSubKey && self.pszValueName == other.pszValueName
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::std::cmp::Eq for TBSAVEPARAMSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
unsafe impl ::windows::runtime::Abi for TBSAVEPARAMSA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_System_Registry`*"]
pub struct TBSAVEPARAMSW {
    pub hkr: super::super::System::Registry::HKEY,
    pub pszSubKey: super::super::Foundation::PWSTR,
    pub pszValueName: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl TBSAVEPARAMSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::std::default::Default for TBSAVEPARAMSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::std::fmt::Debug for TBSAVEPARAMSW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TBSAVEPARAMSW").field("hkr", &self.hkr).field("pszSubKey", &self.pszSubKey).field("pszValueName", &self.pszValueName).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::std::cmp::PartialEq for TBSAVEPARAMSW {
    fn eq(&self, other: &Self) -> bool {
        self.hkr == other.hkr && self.pszSubKey == other.pszSubKey && self.pszValueName == other.pszValueName
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::std::cmp::Eq for TBSAVEPARAMSW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
unsafe impl ::windows::runtime::Abi for TBSAVEPARAMSW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTATE_CHECKED: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTATE_ELLIPSES: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTATE_ENABLED: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTATE_HIDDEN: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTATE_INDETERMINATE: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTATE_MARKED: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTATE_PRESSED: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTATE_WRAP: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTYLE_ALTDRAG: u32 = 1024u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTYLE_AUTOSIZE: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTYLE_BUTTON: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTYLE_CHECK: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTYLE_CUSTOMERASE: u32 = 8192u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTYLE_DROPDOWN: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTYLE_EX_DOUBLEBUFFER: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTYLE_EX_DRAWDDARROWS: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTYLE_EX_HIDECLIPPEDBUTTONS: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTYLE_EX_MIXEDBUTTONS: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTYLE_EX_MULTICOLUMN: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTYLE_EX_VERTICAL: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTYLE_FLAT: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTYLE_GROUP: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTYLE_LIST: u32 = 4096u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTYLE_NOPREFIX: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTYLE_REGISTERDROP: u32 = 16384u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTYLE_SEP: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTYLE_TOOLTIPS: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTYLE_TRANSPARENT: u32 = 32768u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBSTYLE_WRAPABLE: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBS_AUTOTICKS: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBS_BOTH: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBS_BOTTOM: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBS_DOWNISLEFT: u32 = 1024u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBS_ENABLESELRANGE: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBS_FIXEDLENGTH: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBS_HORZ: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBS_LEFT: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBS_NOTHUMB: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBS_NOTICKS: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBS_NOTIFYBEFOREMOVE: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBS_REVERSED: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBS_RIGHT: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBS_TOOLTIPS: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBS_TOP: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBS_TRANSPARENTBKGND: u32 = 4096u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBS_VERT: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBTS_BOTTOM: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBTS_LEFT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBTS_RIGHT: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TBTS_TOP: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_ADDBITMAP: u32 = 1043u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_ADDBUTTONS: u32 = 1092u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_ADDBUTTONSA: u32 = 1044u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_ADDBUTTONSW: u32 = 1092u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_ADDSTRING: u32 = 1101u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_ADDSTRINGA: u32 = 1052u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_ADDSTRINGW: u32 = 1101u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_AUTOSIZE: u32 = 1057u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_BOTTOM: u32 = 7u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_BUTTONCOUNT: u32 = 1048u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_BUTTONSTRUCTSIZE: u32 = 1054u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_CHANGEBITMAP: u32 = 1067u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_CHECKBUTTON: u32 = 1026u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_COMMANDTOINDEX: u32 = 1049u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_CUSTOMIZE: u32 = 1051u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_DELETEBUTTON: u32 = 1046u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_ENABLEBUTTON: u32 = 1025u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_ENDTRACK: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETANCHORHIGHLIGHT: u32 = 1098u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETBITMAP: u32 = 1068u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETBITMAPFLAGS: u32 = 1065u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETBUTTON: u32 = 1047u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETBUTTONINFO: u32 = 1087u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETBUTTONINFOA: u32 = 1089u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETBUTTONINFOW: u32 = 1087u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETBUTTONSIZE: u32 = 1082u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETBUTTONTEXT: u32 = 1099u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETBUTTONTEXTA: u32 = 1069u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETBUTTONTEXTW: u32 = 1099u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETCOLORSCHEME: u32 = 8195u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETDISABLEDIMAGELIST: u32 = 1079u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETEXTENDEDSTYLE: u32 = 1109u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETHOTIMAGELIST: u32 = 1077u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETHOTITEM: u32 = 1095u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETIDEALSIZE: u32 = 1123u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETIMAGELIST: u32 = 1073u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETIMAGELISTCOUNT: u32 = 1122u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETINSERTMARK: u32 = 1103u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETINSERTMARKCOLOR: u32 = 1113u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETITEMDROPDOWNRECT: u32 = 1127u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETITEMRECT: u32 = 1053u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETMAXSIZE: u32 = 1107u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETMETRICS: u32 = 1125u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETOBJECT: u32 = 1086u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETPADDING: u32 = 1110u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETPRESSEDIMAGELIST: u32 = 1129u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETRECT: u32 = 1075u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETROWS: u32 = 1064u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETSTATE: u32 = 1042u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETSTRING: u32 = 1115u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETSTRINGA: u32 = 1116u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETSTRINGW: u32 = 1115u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETSTYLE: u32 = 1081u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETTEXTROWS: u32 = 1085u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETTOOLTIPS: u32 = 1059u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_GETUNICODEFORMAT: u32 = 8198u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_HASACCELERATOR: u32 = 1119u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_HIDEBUTTON: u32 = 1028u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_HITTEST: u32 = 1093u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_INDETERMINATE: u32 = 1029u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_INSERTBUTTON: u32 = 1091u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_INSERTBUTTONA: u32 = 1045u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_INSERTBUTTONW: u32 = 1091u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_INSERTMARKHITTEST: u32 = 1105u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_ISBUTTONCHECKED: u32 = 1034u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_ISBUTTONENABLED: u32 = 1033u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_ISBUTTONHIDDEN: u32 = 1036u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_ISBUTTONHIGHLIGHTED: u32 = 1038u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_ISBUTTONINDETERMINATE: u32 = 1037u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_ISBUTTONPRESSED: u32 = 1035u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_LINEDOWN: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_LINEUP: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_LOADIMAGES: u32 = 1074u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_MAPACCELERATOR: u32 = 1114u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_MAPACCELERATORA: u32 = 1102u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_MAPACCELERATORW: u32 = 1114u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_MARKBUTTON: u32 = 1030u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_MOVEBUTTON: u32 = 1106u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_PAGEDOWN: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_PAGEUP: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_PRESSBUTTON: u32 = 1027u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_REPLACEBITMAP: u32 = 1070u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SAVERESTORE: u32 = 1100u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SAVERESTOREA: u32 = 1050u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SAVERESTOREW: u32 = 1100u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETANCHORHIGHLIGHT: u32 = 1097u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETBITMAPSIZE: u32 = 1056u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETBOUNDINGSIZE: u32 = 1117u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETBUTTONINFO: u32 = 1088u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETBUTTONINFOA: u32 = 1090u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETBUTTONINFOW: u32 = 1088u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETBUTTONSIZE: u32 = 1055u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETBUTTONWIDTH: u32 = 1083u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETCMDID: u32 = 1066u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETCOLORSCHEME: u32 = 8194u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETDISABLEDIMAGELIST: u32 = 1078u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETDRAWTEXTFLAGS: u32 = 1094u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETEXTENDEDSTYLE: u32 = 1108u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETHOTIMAGELIST: u32 = 1076u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETHOTITEM: u32 = 1096u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETHOTITEM2: u32 = 1118u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETIMAGELIST: u32 = 1072u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETINDENT: u32 = 1071u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETINSERTMARK: u32 = 1104u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETINSERTMARKCOLOR: u32 = 1112u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETLISTGAP: u32 = 1120u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETMAXTEXTROWS: u32 = 1084u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETMETRICS: u32 = 1126u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETPADDING: u32 = 1111u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETPARENT: u32 = 1061u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETPRESSEDIMAGELIST: u32 = 1128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETROWS: u32 = 1063u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETSTATE: u32 = 1041u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETSTYLE: u32 = 1080u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETTOOLTIPS: u32 = 1060u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETUNICODEFORMAT: u32 = 8197u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_SETWINDOWTHEME: u32 = 8203u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_THUMBPOSITION: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_THUMBTRACK: u32 = 5u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TB_TOP: u32 = 6u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct TCHITTESTINFO {
    pub pt: super::super::Foundation::POINT,
    pub flags: TCHITTESTINFO_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl TCHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TCHITTESTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TCHITTESTINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TCHITTESTINFO").field("pt", &self.pt).field("flags", &self.flags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TCHITTESTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.pt == other.pt && self.flags == other.flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TCHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TCHITTESTINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TCHITTESTINFO_FLAGS(pub u32);
pub const TCHT_NOWHERE: TCHITTESTINFO_FLAGS = TCHITTESTINFO_FLAGS(1u32);
pub const TCHT_ONITEM: TCHITTESTINFO_FLAGS = TCHITTESTINFO_FLAGS(6u32);
pub const TCHT_ONITEMICON: TCHITTESTINFO_FLAGS = TCHITTESTINFO_FLAGS(2u32);
pub const TCHT_ONITEMLABEL: TCHITTESTINFO_FLAGS = TCHITTESTINFO_FLAGS(4u32);
impl ::std::convert::From<u32> for TCHITTESTINFO_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TCHITTESTINFO_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for TCHITTESTINFO_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for TCHITTESTINFO_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for TCHITTESTINFO_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for TCHITTESTINFO_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for TCHITTESTINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCIS_BUTTONPRESSED: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCIS_HIGHLIGHTED: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct TCITEMA {
    pub mask: TCITEMHEADERA_MASK,
    pub dwState: u32,
    pub dwStateMask: u32,
    pub pszText: super::super::Foundation::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl TCITEMA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TCITEMA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TCITEMA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TCITEMA").field("mask", &self.mask).field("dwState", &self.dwState).field("dwStateMask", &self.dwStateMask).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iImage", &self.iImage).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TCITEMA {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.dwState == other.dwState && self.dwStateMask == other.dwStateMask && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iImage == other.iImage && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TCITEMA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TCITEMA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct TCITEMHEADERA {
    pub mask: TCITEMHEADERA_MASK,
    pub lpReserved1: u32,
    pub lpReserved2: u32,
    pub pszText: super::super::Foundation::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl TCITEMHEADERA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TCITEMHEADERA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TCITEMHEADERA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TCITEMHEADERA").field("mask", &self.mask).field("lpReserved1", &self.lpReserved1).field("lpReserved2", &self.lpReserved2).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iImage", &self.iImage).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TCITEMHEADERA {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.lpReserved1 == other.lpReserved1 && self.lpReserved2 == other.lpReserved2 && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iImage == other.iImage
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TCITEMHEADERA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TCITEMHEADERA {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TCITEMHEADERA_MASK(pub u32);
pub const TCIF_IMAGE: TCITEMHEADERA_MASK = TCITEMHEADERA_MASK(2u32);
pub const TCIF_RTLREADING: TCITEMHEADERA_MASK = TCITEMHEADERA_MASK(4u32);
pub const TCIF_TEXT: TCITEMHEADERA_MASK = TCITEMHEADERA_MASK(1u32);
pub const TCIF_PARAM: TCITEMHEADERA_MASK = TCITEMHEADERA_MASK(8u32);
pub const TCIF_STATE: TCITEMHEADERA_MASK = TCITEMHEADERA_MASK(16u32);
impl ::std::convert::From<u32> for TCITEMHEADERA_MASK {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TCITEMHEADERA_MASK {
    type Abi = Self;
}
impl ::std::ops::BitOr for TCITEMHEADERA_MASK {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for TCITEMHEADERA_MASK {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for TCITEMHEADERA_MASK {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for TCITEMHEADERA_MASK {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for TCITEMHEADERA_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct TCITEMHEADERW {
    pub mask: TCITEMHEADERA_MASK,
    pub lpReserved1: u32,
    pub lpReserved2: u32,
    pub pszText: super::super::Foundation::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl TCITEMHEADERW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TCITEMHEADERW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TCITEMHEADERW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TCITEMHEADERW").field("mask", &self.mask).field("lpReserved1", &self.lpReserved1).field("lpReserved2", &self.lpReserved2).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iImage", &self.iImage).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TCITEMHEADERW {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.lpReserved1 == other.lpReserved1 && self.lpReserved2 == other.lpReserved2 && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iImage == other.iImage
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TCITEMHEADERW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TCITEMHEADERW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct TCITEMW {
    pub mask: TCITEMHEADERA_MASK,
    pub dwState: u32,
    pub dwStateMask: u32,
    pub pszText: super::super::Foundation::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl TCITEMW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TCITEMW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TCITEMW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TCITEMW").field("mask", &self.mask).field("dwState", &self.dwState).field("dwStateMask", &self.dwStateMask).field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("iImage", &self.iImage).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TCITEMW {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.dwState == other.dwState && self.dwStateMask == other.dwStateMask && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iImage == other.iImage && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TCITEMW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TCITEMW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_ADJUSTRECT: u32 = 4904u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_DELETEALLITEMS: u32 = 4873u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_DELETEITEM: u32 = 4872u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_DESELECTALL: u32 = 4914u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_FIRST: u32 = 4864u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_GETCURFOCUS: u32 = 4911u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_GETCURSEL: u32 = 4875u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_GETEXTENDEDSTYLE: u32 = 4917u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_GETIMAGELIST: u32 = 4866u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_GETITEM: u32 = 4924u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_GETITEMA: u32 = 4869u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_GETITEMCOUNT: u32 = 4868u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_GETITEMRECT: u32 = 4874u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_GETITEMW: u32 = 4924u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_GETROWCOUNT: u32 = 4908u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_GETTOOLTIPS: u32 = 4909u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_GETUNICODEFORMAT: u32 = 8198u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_HIGHLIGHTITEM: u32 = 4915u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_HITTEST: u32 = 4877u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_INSERTITEM: u32 = 4926u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_INSERTITEMA: u32 = 4871u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_INSERTITEMW: u32 = 4926u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_REMOVEIMAGE: u32 = 4906u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_SETCURFOCUS: u32 = 4912u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_SETCURSEL: u32 = 4876u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_SETEXTENDEDSTYLE: u32 = 4916u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_SETIMAGELIST: u32 = 4867u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_SETITEM: u32 = 4925u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_SETITEMA: u32 = 4870u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_SETITEMEXTRA: u32 = 4878u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_SETITEMSIZE: u32 = 4905u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_SETITEMW: u32 = 4925u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_SETMINTABWIDTH: u32 = 4913u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_SETPADDING: u32 = 4907u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_SETTOOLTIPS: u32 = 4910u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCM_SETUNICODEFORMAT: u32 = 8197u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCS_BOTTOM: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCS_BUTTONS: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCS_EX_FLATSEPARATORS: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCS_EX_REGISTERDROP: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCS_FIXEDWIDTH: u32 = 1024u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCS_FLATBUTTONS: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCS_FOCUSNEVER: u32 = 32768u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCS_FOCUSONBUTTONDOWN: u32 = 4096u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCS_FORCEICONLEFT: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCS_FORCELABELLEFT: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCS_HOTTRACK: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCS_MULTILINE: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCS_MULTISELECT: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCS_OWNERDRAWFIXED: u32 = 8192u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCS_RAGGEDRIGHT: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCS_RIGHT: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCS_RIGHTJUSTIFY: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCS_SCROLLOPPOSITE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCS_SINGLELINE: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCS_TABS: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCS_TOOLTIPS: u32 = 16384u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TCS_VERTICAL: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TEXTSHADOWTYPE(pub i32);
pub const TST_NONE: TEXTSHADOWTYPE = TEXTSHADOWTYPE(0i32);
pub const TST_SINGLE: TEXTSHADOWTYPE = TEXTSHADOWTYPE(1i32);
pub const TST_CONTINUOUS: TEXTSHADOWTYPE = TEXTSHADOWTYPE(2i32);
impl ::std::convert::From<i32> for TEXTSHADOWTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TEXTSHADOWTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct THEMESIZE(pub i32);
pub const TS_MIN: THEMESIZE = THEMESIZE(0i32);
pub const TS_TRUE: THEMESIZE = THEMESIZE(1i32);
pub const TS_DRAW: THEMESIZE = THEMESIZE(2i32);
impl ::std::convert::From<i32> for THEMESIZE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for THEMESIZE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct THEME_PROPERTY_SYMBOL_ID(pub u32);
pub const TMT_RESERVEDLOW: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(0u32);
pub const TMT_RESERVEDHIGH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(7999u32);
pub const TMT_DIBDATA: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2u32);
pub const TMT_GLYPHDIBDATA: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(8u32);
pub const TMT_ENUM: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(200u32);
pub const TMT_STRING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(201u32);
pub const TMT_INT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(202u32);
pub const TMT_BOOL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(203u32);
pub const TMT_COLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(204u32);
pub const TMT_MARGINS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(205u32);
pub const TMT_FILENAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(206u32);
pub const TMT_SIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(207u32);
pub const TMT_POSITION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(208u32);
pub const TMT_RECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(209u32);
pub const TMT_FONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(210u32);
pub const TMT_INTLIST: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(211u32);
pub const TMT_HBITMAP: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(212u32);
pub const TMT_DISKSTREAM: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(213u32);
pub const TMT_STREAM: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(214u32);
pub const TMT_BITMAPREF: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(215u32);
pub const TMT_FLOAT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(216u32);
pub const TMT_FLOATLIST: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(217u32);
pub const TMT_COLORSCHEMES: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(401u32);
pub const TMT_SIZES: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(402u32);
pub const TMT_CHARSET: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(403u32);
pub const TMT_NAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(600u32);
pub const TMT_DISPLAYNAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(601u32);
pub const TMT_TOOLTIP: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(602u32);
pub const TMT_COMPANY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(603u32);
pub const TMT_AUTHOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(604u32);
pub const TMT_COPYRIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(605u32);
pub const TMT_URL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(606u32);
pub const TMT_VERSION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(607u32);
pub const TMT_DESCRIPTION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(608u32);
pub const TMT_FIRST_RCSTRING_NAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(601u32);
pub const TMT_LAST_RCSTRING_NAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(608u32);
pub const TMT_CAPTIONFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(801u32);
pub const TMT_SMALLCAPTIONFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(802u32);
pub const TMT_MENUFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(803u32);
pub const TMT_STATUSFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(804u32);
pub const TMT_MSGBOXFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(805u32);
pub const TMT_ICONTITLEFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(806u32);
pub const TMT_HEADING1FONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(807u32);
pub const TMT_HEADING2FONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(808u32);
pub const TMT_BODYFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(809u32);
pub const TMT_FIRSTFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(801u32);
pub const TMT_LASTFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(809u32);
pub const TMT_FLATMENUS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1001u32);
pub const TMT_FIRSTBOOL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1001u32);
pub const TMT_LASTBOOL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1001u32);
pub const TMT_SIZINGBORDERWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1201u32);
pub const TMT_SCROLLBARWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1202u32);
pub const TMT_SCROLLBARHEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1203u32);
pub const TMT_CAPTIONBARWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1204u32);
pub const TMT_CAPTIONBARHEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1205u32);
pub const TMT_SMCAPTIONBARWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1206u32);
pub const TMT_SMCAPTIONBARHEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1207u32);
pub const TMT_MENUBARWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1208u32);
pub const TMT_MENUBARHEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1209u32);
pub const TMT_PADDEDBORDERWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1210u32);
pub const TMT_FIRSTSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1201u32);
pub const TMT_LASTSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1210u32);
pub const TMT_MINCOLORDEPTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1301u32);
pub const TMT_FIRSTINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1301u32);
pub const TMT_LASTINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1301u32);
pub const TMT_CSSNAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1401u32);
pub const TMT_XMLNAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1402u32);
pub const TMT_LASTUPDATED: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1403u32);
pub const TMT_ALIAS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1404u32);
pub const TMT_FIRSTSTRING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1401u32);
pub const TMT_LASTSTRING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1404u32);
pub const TMT_SCROLLBAR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1601u32);
pub const TMT_BACKGROUND: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1602u32);
pub const TMT_ACTIVECAPTION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1603u32);
pub const TMT_INACTIVECAPTION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1604u32);
pub const TMT_MENU: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1605u32);
pub const TMT_WINDOW: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1606u32);
pub const TMT_WINDOWFRAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1607u32);
pub const TMT_MENUTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1608u32);
pub const TMT_WINDOWTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1609u32);
pub const TMT_CAPTIONTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1610u32);
pub const TMT_ACTIVEBORDER: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1611u32);
pub const TMT_INACTIVEBORDER: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1612u32);
pub const TMT_APPWORKSPACE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1613u32);
pub const TMT_HIGHLIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1614u32);
pub const TMT_HIGHLIGHTTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1615u32);
pub const TMT_BTNFACE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1616u32);
pub const TMT_BTNSHADOW: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1617u32);
pub const TMT_GRAYTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1618u32);
pub const TMT_BTNTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1619u32);
pub const TMT_INACTIVECAPTIONTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1620u32);
pub const TMT_BTNHIGHLIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1621u32);
pub const TMT_DKSHADOW3D: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1622u32);
pub const TMT_LIGHT3D: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1623u32);
pub const TMT_INFOTEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1624u32);
pub const TMT_INFOBK: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1625u32);
pub const TMT_BUTTONALTERNATEFACE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1626u32);
pub const TMT_HOTTRACKING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1627u32);
pub const TMT_GRADIENTACTIVECAPTION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1628u32);
pub const TMT_GRADIENTINACTIVECAPTION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1629u32);
pub const TMT_MENUHILIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1630u32);
pub const TMT_MENUBAR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1631u32);
pub const TMT_FIRSTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1601u32);
pub const TMT_LASTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1631u32);
pub const TMT_FROMHUE1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1801u32);
pub const TMT_FROMHUE2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1802u32);
pub const TMT_FROMHUE3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1803u32);
pub const TMT_FROMHUE4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1804u32);
pub const TMT_FROMHUE5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1805u32);
pub const TMT_TOHUE1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1806u32);
pub const TMT_TOHUE2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1807u32);
pub const TMT_TOHUE3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1808u32);
pub const TMT_TOHUE4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1809u32);
pub const TMT_TOHUE5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(1810u32);
pub const TMT_FROMCOLOR1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2001u32);
pub const TMT_FROMCOLOR2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2002u32);
pub const TMT_FROMCOLOR3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2003u32);
pub const TMT_FROMCOLOR4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2004u32);
pub const TMT_FROMCOLOR5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2005u32);
pub const TMT_TOCOLOR1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2006u32);
pub const TMT_TOCOLOR2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2007u32);
pub const TMT_TOCOLOR3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2008u32);
pub const TMT_TOCOLOR4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2009u32);
pub const TMT_TOCOLOR5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2010u32);
pub const TMT_TRANSPARENT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2201u32);
pub const TMT_AUTOSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2202u32);
pub const TMT_BORDERONLY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2203u32);
pub const TMT_COMPOSITED: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2204u32);
pub const TMT_BGFILL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2205u32);
pub const TMT_GLYPHTRANSPARENT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2206u32);
pub const TMT_GLYPHONLY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2207u32);
pub const TMT_ALWAYSSHOWSIZINGBAR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2208u32);
pub const TMT_MIRRORIMAGE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2209u32);
pub const TMT_UNIFORMSIZING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2210u32);
pub const TMT_INTEGRALSIZING: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2211u32);
pub const TMT_SOURCEGROW: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2212u32);
pub const TMT_SOURCESHRINK: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2213u32);
pub const TMT_DRAWBORDERS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2214u32);
pub const TMT_NOETCHEDEFFECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2215u32);
pub const TMT_TEXTAPPLYOVERLAY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2216u32);
pub const TMT_TEXTGLOW: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2217u32);
pub const TMT_TEXTITALIC: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2218u32);
pub const TMT_COMPOSITEDOPAQUE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2219u32);
pub const TMT_LOCALIZEDMIRRORIMAGE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2220u32);
pub const TMT_IMAGECOUNT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2401u32);
pub const TMT_ALPHALEVEL: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2402u32);
pub const TMT_BORDERSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2403u32);
pub const TMT_ROUNDCORNERWIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2404u32);
pub const TMT_ROUNDCORNERHEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2405u32);
pub const TMT_GRADIENTRATIO1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2406u32);
pub const TMT_GRADIENTRATIO2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2407u32);
pub const TMT_GRADIENTRATIO3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2408u32);
pub const TMT_GRADIENTRATIO4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2409u32);
pub const TMT_GRADIENTRATIO5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2410u32);
pub const TMT_PROGRESSCHUNKSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2411u32);
pub const TMT_PROGRESSSPACESIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2412u32);
pub const TMT_SATURATION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2413u32);
pub const TMT_TEXTBORDERSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2414u32);
pub const TMT_ALPHATHRESHOLD: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2415u32);
pub const TMT_WIDTH: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2416u32);
pub const TMT_HEIGHT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2417u32);
pub const TMT_GLYPHINDEX: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2418u32);
pub const TMT_TRUESIZESTRETCHMARK: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2419u32);
pub const TMT_MINDPI1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2420u32);
pub const TMT_MINDPI2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2421u32);
pub const TMT_MINDPI3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2422u32);
pub const TMT_MINDPI4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2423u32);
pub const TMT_MINDPI5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2424u32);
pub const TMT_TEXTGLOWSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2425u32);
pub const TMT_FRAMESPERSECOND: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2426u32);
pub const TMT_PIXELSPERFRAME: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2427u32);
pub const TMT_ANIMATIONDELAY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2428u32);
pub const TMT_GLOWINTENSITY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2429u32);
pub const TMT_OPACITY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2430u32);
pub const TMT_COLORIZATIONCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2431u32);
pub const TMT_COLORIZATIONOPACITY: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2432u32);
pub const TMT_MINDPI6: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2433u32);
pub const TMT_MINDPI7: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2434u32);
pub const TMT_GLYPHFONT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(2601u32);
pub const TMT_IMAGEFILE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3001u32);
pub const TMT_IMAGEFILE1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3002u32);
pub const TMT_IMAGEFILE2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3003u32);
pub const TMT_IMAGEFILE3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3004u32);
pub const TMT_IMAGEFILE4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3005u32);
pub const TMT_IMAGEFILE5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3006u32);
pub const TMT_GLYPHIMAGEFILE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3008u32);
pub const TMT_IMAGEFILE6: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3009u32);
pub const TMT_IMAGEFILE7: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3010u32);
pub const TMT_TEXT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3201u32);
pub const TMT_CLASSICVALUE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3202u32);
pub const TMT_OFFSET: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3401u32);
pub const TMT_TEXTSHADOWOFFSET: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3402u32);
pub const TMT_MINSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3403u32);
pub const TMT_MINSIZE1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3404u32);
pub const TMT_MINSIZE2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3405u32);
pub const TMT_MINSIZE3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3406u32);
pub const TMT_MINSIZE4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3407u32);
pub const TMT_MINSIZE5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3408u32);
pub const TMT_NORMALSIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3409u32);
pub const TMT_MINSIZE6: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3410u32);
pub const TMT_MINSIZE7: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3411u32);
pub const TMT_SIZINGMARGINS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3601u32);
pub const TMT_CONTENTMARGINS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3602u32);
pub const TMT_CAPTIONMARGINS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3603u32);
pub const TMT_BORDERCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3801u32);
pub const TMT_FILLCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3802u32);
pub const TMT_TEXTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3803u32);
pub const TMT_EDGELIGHTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3804u32);
pub const TMT_EDGEHIGHLIGHTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3805u32);
pub const TMT_EDGESHADOWCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3806u32);
pub const TMT_EDGEDKSHADOWCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3807u32);
pub const TMT_EDGEFILLCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3808u32);
pub const TMT_TRANSPARENTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3809u32);
pub const TMT_GRADIENTCOLOR1: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3810u32);
pub const TMT_GRADIENTCOLOR2: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3811u32);
pub const TMT_GRADIENTCOLOR3: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3812u32);
pub const TMT_GRADIENTCOLOR4: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3813u32);
pub const TMT_GRADIENTCOLOR5: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3814u32);
pub const TMT_SHADOWCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3815u32);
pub const TMT_GLOWCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3816u32);
pub const TMT_TEXTBORDERCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3817u32);
pub const TMT_TEXTSHADOWCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3818u32);
pub const TMT_GLYPHTEXTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3819u32);
pub const TMT_GLYPHTRANSPARENTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3820u32);
pub const TMT_FILLCOLORHINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3821u32);
pub const TMT_BORDERCOLORHINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3822u32);
pub const TMT_ACCENTCOLORHINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3823u32);
pub const TMT_TEXTCOLORHINT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3824u32);
pub const TMT_HEADING1TEXTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3825u32);
pub const TMT_HEADING2TEXTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3826u32);
pub const TMT_BODYTEXTCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(3827u32);
pub const TMT_BGTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4001u32);
pub const TMT_BORDERTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4002u32);
pub const TMT_FILLTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4003u32);
pub const TMT_SIZINGTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4004u32);
pub const TMT_HALIGN: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4005u32);
pub const TMT_CONTENTALIGNMENT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4006u32);
pub const TMT_VALIGN: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4007u32);
pub const TMT_OFFSETTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4008u32);
pub const TMT_ICONEFFECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4009u32);
pub const TMT_TEXTSHADOWTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4010u32);
pub const TMT_IMAGELAYOUT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4011u32);
pub const TMT_GLYPHTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4012u32);
pub const TMT_IMAGESELECTTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4013u32);
pub const TMT_GLYPHFONTSIZINGTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4014u32);
pub const TMT_TRUESIZESCALINGTYPE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(4015u32);
pub const TMT_USERPICTURE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5001u32);
pub const TMT_DEFAULTPANESIZE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5002u32);
pub const TMT_BLENDCOLOR: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5003u32);
pub const TMT_CUSTOMSPLITRECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5004u32);
pub const TMT_ANIMATIONBUTTONRECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5005u32);
pub const TMT_ANIMATIONDURATION: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(5006u32);
pub const TMT_TRANSITIONDURATIONS: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(6000u32);
pub const TMT_SCALEDBACKGROUND: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(7001u32);
pub const TMT_ATLASIMAGE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(8000u32);
pub const TMT_ATLASINPUTIMAGE: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(8001u32);
pub const TMT_ATLASRECT: THEME_PROPERTY_SYMBOL_ID = THEME_PROPERTY_SYMBOL_ID(8002u32);
impl ::std::convert::From<u32> for THEME_PROPERTY_SYMBOL_ID {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for THEME_PROPERTY_SYMBOL_ID {
    type Abi = Self;
}
impl ::std::ops::BitOr for THEME_PROPERTY_SYMBOL_ID {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for THEME_PROPERTY_SYMBOL_ID {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for THEME_PROPERTY_SYMBOL_ID {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for THEME_PROPERTY_SYMBOL_ID {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for THEME_PROPERTY_SYMBOL_ID {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct TOUCH_HIT_TESTING_INPUT {
    pub pointerId: u32,
    pub point: super::super::Foundation::POINT,
    pub boundingBox: super::super::Foundation::RECT,
    pub nonOccludedBoundingBox: super::super::Foundation::RECT,
    pub orientation: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl TOUCH_HIT_TESTING_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TOUCH_HIT_TESTING_INPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TOUCH_HIT_TESTING_INPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TOUCH_HIT_TESTING_INPUT").field("pointerId", &self.pointerId).field("point", &self.point).field("boundingBox", &self.boundingBox).field("nonOccludedBoundingBox", &self.nonOccludedBoundingBox).field("orientation", &self.orientation).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TOUCH_HIT_TESTING_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.pointerId == other.pointerId && self.point == other.point && self.boundingBox == other.boundingBox && self.nonOccludedBoundingBox == other.nonOccludedBoundingBox && self.orientation == other.orientation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TOUCH_HIT_TESTING_INPUT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TOUCH_HIT_TESTING_INPUT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct TOUCH_HIT_TESTING_PROXIMITY_EVALUATION {
    pub score: u16,
    pub adjustedPoint: super::super::Foundation::POINT,
}
#[cfg(feature = "Win32_Foundation")]
impl TOUCH_HIT_TESTING_PROXIMITY_EVALUATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TOUCH_HIT_TESTING_PROXIMITY_EVALUATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TOUCH_HIT_TESTING_PROXIMITY_EVALUATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TOUCH_HIT_TESTING_PROXIMITY_EVALUATION").field("score", &self.score).field("adjustedPoint", &self.adjustedPoint).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TOUCH_HIT_TESTING_PROXIMITY_EVALUATION {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score && self.adjustedPoint == other.adjustedPoint
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TOUCH_HIT_TESTING_PROXIMITY_EVALUATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TOUCH_HIT_TESTING_PROXIMITY_EVALUATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TRAILINGGRIDCELLSTATES(pub i32);
pub const MCTGC_HOT: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(1i32);
pub const MCTGC_HASSTATE: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(2i32);
pub const MCTGC_HASSTATEHOT: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(3i32);
pub const MCTGC_TODAY: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(4i32);
pub const MCTGC_TODAYSELECTED: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(5i32);
pub const MCTGC_SELECTED: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(6i32);
pub const MCTGC_SELECTEDHOT: TRAILINGGRIDCELLSTATES = TRAILINGGRIDCELLSTATES(7i32);
impl ::std::convert::From<i32> for TRAILINGGRIDCELLSTATES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TRAILINGGRIDCELLSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TRAILINGGRIDCELLUPPERSTATES(pub i32);
pub const MCTGCU_HOT: TRAILINGGRIDCELLUPPERSTATES = TRAILINGGRIDCELLUPPERSTATES(1i32);
pub const MCTGCU_HASSTATE: TRAILINGGRIDCELLUPPERSTATES = TRAILINGGRIDCELLUPPERSTATES(2i32);
pub const MCTGCU_HASSTATEHOT: TRAILINGGRIDCELLUPPERSTATES = TRAILINGGRIDCELLUPPERSTATES(3i32);
pub const MCTGCU_SELECTED: TRAILINGGRIDCELLUPPERSTATES = TRAILINGGRIDCELLUPPERSTATES(4i32);
pub const MCTGCU_SELECTEDHOT: TRAILINGGRIDCELLUPPERSTATES = TRAILINGGRIDCELLUPPERSTATES(5i32);
impl ::std::convert::From<i32> for TRAILINGGRIDCELLUPPERSTATES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TRAILINGGRIDCELLUPPERSTATES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TRAYNOTIFYPARTS(pub i32);
pub const TNP_BACKGROUND: TRAYNOTIFYPARTS = TRAYNOTIFYPARTS(1i32);
pub const TNP_ANIMBACKGROUND: TRAYNOTIFYPARTS = TRAYNOTIFYPARTS(2i32);
impl ::std::convert::From<i32> for TRAYNOTIFYPARTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TRAYNOTIFYPARTS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TRUESIZESCALINGTYPE(pub i32);
pub const TSST_NONE: TRUESIZESCALINGTYPE = TRUESIZESCALINGTYPE(0i32);
pub const TSST_SIZE: TRUESIZESCALINGTYPE = TRUESIZESCALINGTYPE(1i32);
pub const TSST_DPI: TRUESIZESCALINGTYPE = TRUESIZESCALINGTYPE(2i32);
impl ::std::convert::From<i32> for TRUESIZESCALINGTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TRUESIZESCALINGTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTDT_AUTOMATIC: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTDT_AUTOPOP: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTDT_INITIAL: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTDT_RESHOW: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTF_DI_SETITEM: u32 = 32768u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct TTGETTITLE {
    pub dwSize: u32,
    pub uTitleBitmap: u32,
    pub cch: u32,
    pub pszTitle: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl TTGETTITLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TTGETTITLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TTGETTITLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TTGETTITLE").field("dwSize", &self.dwSize).field("uTitleBitmap", &self.uTitleBitmap).field("cch", &self.cch).field("pszTitle", &self.pszTitle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TTGETTITLE {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.uTitleBitmap == other.uTitleBitmap && self.cch == other.cch && self.pszTitle == other.pszTitle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TTGETTITLE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TTGETTITLE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct TTHITTESTINFOA {
    pub hwnd: super::super::Foundation::HWND,
    pub pt: super::super::Foundation::POINT,
    pub ti: TTTOOLINFOA,
}
#[cfg(feature = "Win32_Foundation")]
impl TTHITTESTINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TTHITTESTINFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TTHITTESTINFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TTHITTESTINFOA").field("hwnd", &self.hwnd).field("pt", &self.pt).field("ti", &self.ti).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TTHITTESTINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.hwnd == other.hwnd && self.pt == other.pt && self.ti == other.ti
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TTHITTESTINFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TTHITTESTINFOA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct TTHITTESTINFOW {
    pub hwnd: super::super::Foundation::HWND,
    pub pt: super::super::Foundation::POINT,
    pub ti: TTTOOLINFOW,
}
#[cfg(feature = "Win32_Foundation")]
impl TTHITTESTINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TTHITTESTINFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TTHITTESTINFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TTHITTESTINFOW").field("hwnd", &self.hwnd).field("pt", &self.pt).field("ti", &self.ti).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TTHITTESTINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.hwnd == other.hwnd && self.pt == other.pt && self.ti == other.ti
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TTHITTESTINFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TTHITTESTINFOW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_ACTIVATE: u32 = 1025u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_ADDTOOL: u32 = 1074u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_ADDTOOLA: u32 = 1028u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_ADDTOOLW: u32 = 1074u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_ADJUSTRECT: u32 = 1055u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_DELTOOL: u32 = 1075u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_DELTOOLA: u32 = 1029u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_DELTOOLW: u32 = 1075u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_ENUMTOOLS: u32 = 1082u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_ENUMTOOLSA: u32 = 1038u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_ENUMTOOLSW: u32 = 1082u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_GETBUBBLESIZE: u32 = 1054u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_GETCURRENTTOOL: u32 = 1083u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_GETCURRENTTOOLA: u32 = 1039u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_GETCURRENTTOOLW: u32 = 1083u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_GETDELAYTIME: u32 = 1045u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_GETMARGIN: u32 = 1051u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_GETMAXTIPWIDTH: u32 = 1049u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_GETTEXT: u32 = 1080u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_GETTEXTA: u32 = 1035u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_GETTEXTW: u32 = 1080u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_GETTIPBKCOLOR: u32 = 1046u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_GETTIPTEXTCOLOR: u32 = 1047u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_GETTITLE: u32 = 1059u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_GETTOOLCOUNT: u32 = 1037u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_GETTOOLINFO: u32 = 1077u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_GETTOOLINFOA: u32 = 1032u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_GETTOOLINFOW: u32 = 1077u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_HITTEST: u32 = 1079u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_HITTESTA: u32 = 1034u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_HITTESTW: u32 = 1079u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_NEWTOOLRECT: u32 = 1076u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_NEWTOOLRECTA: u32 = 1030u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_NEWTOOLRECTW: u32 = 1076u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_POP: u32 = 1052u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_POPUP: u32 = 1058u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_RELAYEVENT: u32 = 1031u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_SETDELAYTIME: u32 = 1027u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_SETMARGIN: u32 = 1050u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_SETMAXTIPWIDTH: u32 = 1048u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_SETTIPBKCOLOR: u32 = 1043u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_SETTIPTEXTCOLOR: u32 = 1044u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_SETTITLE: u32 = 1057u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_SETTITLEA: u32 = 1056u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_SETTITLEW: u32 = 1057u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_SETTOOLINFO: u32 = 1078u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_SETTOOLINFOA: u32 = 1033u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_SETTOOLINFOW: u32 = 1078u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_SETWINDOWTHEME: u32 = 8203u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_TRACKACTIVATE: u32 = 1041u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_TRACKPOSITION: u32 = 1042u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_UPDATE: u32 = 1053u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_UPDATETIPTEXT: u32 = 1081u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_UPDATETIPTEXTA: u32 = 1036u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_UPDATETIPTEXTW: u32 = 1081u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTM_WINDOWFROMPOINT: u32 = 1040u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTS_ALWAYSTIP: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTS_BALLOON: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTS_CLOSE: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTS_NOANIMATE: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTS_NOFADE: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTS_NOPREFIX: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TTS_USEVISUALSTYLE: u32 = 256u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct TTTOOLINFOA {
    pub cbSize: u32,
    pub uFlags: TTTOOLINFO_FLAGS,
    pub hwnd: super::super::Foundation::HWND,
    pub uId: usize,
    pub rect: super::super::Foundation::RECT,
    pub hinst: super::super::Foundation::HINSTANCE,
    pub lpszText: super::super::Foundation::PSTR,
    pub lParam: super::super::Foundation::LPARAM,
    pub lpReserved: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl TTTOOLINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TTTOOLINFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TTTOOLINFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TTTOOLINFOA")
            .field("cbSize", &self.cbSize)
            .field("uFlags", &self.uFlags)
            .field("hwnd", &self.hwnd)
            .field("uId", &self.uId)
            .field("rect", &self.rect)
            .field("hinst", &self.hinst)
            .field("lpszText", &self.lpszText)
            .field("lParam", &self.lParam)
            .field("lpReserved", &self.lpReserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TTTOOLINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.uFlags == other.uFlags && self.hwnd == other.hwnd && self.uId == other.uId && self.rect == other.rect && self.hinst == other.hinst && self.lpszText == other.lpszText && self.lParam == other.lParam && self.lpReserved == other.lpReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TTTOOLINFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TTTOOLINFOA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct TTTOOLINFOW {
    pub cbSize: u32,
    pub uFlags: TTTOOLINFO_FLAGS,
    pub hwnd: super::super::Foundation::HWND,
    pub uId: usize,
    pub rect: super::super::Foundation::RECT,
    pub hinst: super::super::Foundation::HINSTANCE,
    pub lpszText: super::super::Foundation::PWSTR,
    pub lParam: super::super::Foundation::LPARAM,
    pub lpReserved: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl TTTOOLINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TTTOOLINFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TTTOOLINFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TTTOOLINFOW")
            .field("cbSize", &self.cbSize)
            .field("uFlags", &self.uFlags)
            .field("hwnd", &self.hwnd)
            .field("uId", &self.uId)
            .field("rect", &self.rect)
            .field("hinst", &self.hinst)
            .field("lpszText", &self.lpszText)
            .field("lParam", &self.lParam)
            .field("lpReserved", &self.lpReserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TTTOOLINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.uFlags == other.uFlags && self.hwnd == other.hwnd && self.uId == other.uId && self.rect == other.rect && self.hinst == other.hinst && self.lpszText == other.lpszText && self.lParam == other.lParam && self.lpReserved == other.lpReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TTTOOLINFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TTTOOLINFOW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TTTOOLINFO_FLAGS(pub u32);
pub const TTF_ABSOLUTE: TTTOOLINFO_FLAGS = TTTOOLINFO_FLAGS(128u32);
pub const TTF_CENTERTIP: TTTOOLINFO_FLAGS = TTTOOLINFO_FLAGS(2u32);
pub const TTF_IDISHWND: TTTOOLINFO_FLAGS = TTTOOLINFO_FLAGS(1u32);
pub const TTF_PARSELINKS: TTTOOLINFO_FLAGS = TTTOOLINFO_FLAGS(4096u32);
pub const TTF_RTLREADING: TTTOOLINFO_FLAGS = TTTOOLINFO_FLAGS(4u32);
pub const TTF_SUBCLASS: TTTOOLINFO_FLAGS = TTTOOLINFO_FLAGS(16u32);
pub const TTF_TRACK: TTTOOLINFO_FLAGS = TTTOOLINFO_FLAGS(32u32);
pub const TTF_TRANSPARENT: TTTOOLINFO_FLAGS = TTTOOLINFO_FLAGS(256u32);
impl ::std::convert::From<u32> for TTTOOLINFO_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TTTOOLINFO_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for TTTOOLINFO_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for TTTOOLINFO_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for TTTOOLINFO_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for TTTOOLINFO_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for TTTOOLINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVCDRF_NOIMAGES: u32 = 65536u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVC_BYKEYBOARD: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVC_BYMOUSE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVC_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVE_COLLAPSE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVE_COLLAPSERESET: u32 = 32768u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVE_EXPAND: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVE_EXPANDPARTIAL: u32 = 16384u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVE_TOGGLE: u32 = 3u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct TVGETITEMPARTRECTINFO {
    pub hti: *mut _TREEITEM,
    pub prc: *mut super::super::Foundation::RECT,
    pub partID: TVITEMPART,
}
#[cfg(feature = "Win32_Foundation")]
impl TVGETITEMPARTRECTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TVGETITEMPARTRECTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TVGETITEMPARTRECTINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TVGETITEMPARTRECTINFO").field("hti", &self.hti).field("prc", &self.prc).field("partID", &self.partID).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TVGETITEMPARTRECTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.hti == other.hti && self.prc == other.prc && self.partID == other.partID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TVGETITEMPARTRECTINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TVGETITEMPARTRECTINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVGN_CARET: u32 = 9u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVGN_CHILD: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVGN_DROPHILITE: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVGN_FIRSTVISIBLE: u32 = 5u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVGN_LASTVISIBLE: u32 = 10u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVGN_NEXT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVGN_NEXTSELECTED: u32 = 11u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVGN_NEXTVISIBLE: u32 = 6u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVGN_PARENT: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVGN_PREVIOUS: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVGN_PREVIOUSVISIBLE: u32 = 7u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVGN_ROOT: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct TVHITTESTINFO {
    pub pt: super::super::Foundation::POINT,
    pub flags: TVHITTESTINFO_FLAGS,
    pub hItem: *mut _TREEITEM,
}
#[cfg(feature = "Win32_Foundation")]
impl TVHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TVHITTESTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TVHITTESTINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TVHITTESTINFO").field("pt", &self.pt).field("flags", &self.flags).field("hItem", &self.hItem).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TVHITTESTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.pt == other.pt && self.flags == other.flags && self.hItem == other.hItem
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TVHITTESTINFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TVHITTESTINFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TVHITTESTINFO_FLAGS(pub u32);
pub const TVHT_ABOVE: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(256u32);
pub const TVHT_BELOW: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(512u32);
pub const TVHT_NOWHERE: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(1u32);
pub const TVHT_ONITEM: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(70u32);
pub const TVHT_ONITEMBUTTON: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(16u32);
pub const TVHT_ONITEMICON: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(2u32);
pub const TVHT_ONITEMINDENT: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(8u32);
pub const TVHT_ONITEMLABEL: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(4u32);
pub const TVHT_ONITEMRIGHT: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(32u32);
pub const TVHT_ONITEMSTATEICON: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(64u32);
pub const TVHT_TOLEFT: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(2048u32);
pub const TVHT_TORIGHT: TVHITTESTINFO_FLAGS = TVHITTESTINFO_FLAGS(1024u32);
impl ::std::convert::From<u32> for TVHITTESTINFO_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TVHITTESTINFO_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for TVHITTESTINFO_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for TVHITTESTINFO_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for TVHITTESTINFO_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for TVHITTESTINFO_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for TVHITTESTINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct TVINSERTSTRUCTA {
    pub hParent: *mut _TREEITEM,
    pub hInsertAfter: *mut _TREEITEM,
    pub Anonymous: TVINSERTSTRUCTA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl TVINSERTSTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TVINSERTSTRUCTA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TVINSERTSTRUCTA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TVINSERTSTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TVINSERTSTRUCTA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub union TVINSERTSTRUCTA_0 {
    pub itemex: TVITEMEXA,
    pub item: TVITEMA,
}
#[cfg(feature = "Win32_Foundation")]
impl TVINSERTSTRUCTA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TVINSERTSTRUCTA_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TVINSERTSTRUCTA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TVINSERTSTRUCTA_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TVINSERTSTRUCTA_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct TVINSERTSTRUCTW {
    pub hParent: *mut _TREEITEM,
    pub hInsertAfter: *mut _TREEITEM,
    pub Anonymous: TVINSERTSTRUCTW_0,
}
#[cfg(feature = "Win32_Foundation")]
impl TVINSERTSTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TVINSERTSTRUCTW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TVINSERTSTRUCTW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TVINSERTSTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TVINSERTSTRUCTW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub union TVINSERTSTRUCTW_0 {
    pub itemex: TVITEMEXW,
    pub item: TVITEMW,
}
#[cfg(feature = "Win32_Foundation")]
impl TVINSERTSTRUCTW_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TVINSERTSTRUCTW_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TVINSERTSTRUCTW_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TVINSERTSTRUCTW_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TVINSERTSTRUCTW_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVIS_BOLD: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVIS_CUT: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVIS_DROPHILITED: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVIS_EXPANDED: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVIS_EXPANDEDONCE: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVIS_EXPANDPARTIAL: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVIS_EX_ALL: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVIS_EX_DISABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVIS_EX_FLAT: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVIS_OVERLAYMASK: u32 = 3840u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVIS_SELECTED: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVIS_STATEIMAGEMASK: u32 = 61440u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVIS_USERMASK: u32 = 61440u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct TVITEMA {
    pub mask: TVITEM_MASK,
    pub hItem: *mut _TREEITEM,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: super::super::Foundation::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: TVITEMEXW_CHILDREN,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl TVITEMA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TVITEMA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TVITEMA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TVITEMA")
            .field("mask", &self.mask)
            .field("hItem", &self.hItem)
            .field("state", &self.state)
            .field("stateMask", &self.stateMask)
            .field("pszText", &self.pszText)
            .field("cchTextMax", &self.cchTextMax)
            .field("iImage", &self.iImage)
            .field("iSelectedImage", &self.iSelectedImage)
            .field("cChildren", &self.cChildren)
            .field("lParam", &self.lParam)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TVITEMA {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.hItem == other.hItem && self.state == other.state && self.stateMask == other.stateMask && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iImage == other.iImage && self.iSelectedImage == other.iSelectedImage && self.cChildren == other.cChildren && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TVITEMA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TVITEMA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct TVITEMEXA {
    pub mask: TVITEM_MASK,
    pub hItem: *mut _TREEITEM,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: super::super::Foundation::PSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: TVITEMEXW_CHILDREN,
    pub lParam: super::super::Foundation::LPARAM,
    pub iIntegral: i32,
    pub uStateEx: u32,
    pub hwnd: super::super::Foundation::HWND,
    pub iExpandedImage: i32,
    pub iReserved: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl TVITEMEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TVITEMEXA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TVITEMEXA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TVITEMEXA")
            .field("mask", &self.mask)
            .field("hItem", &self.hItem)
            .field("state", &self.state)
            .field("stateMask", &self.stateMask)
            .field("pszText", &self.pszText)
            .field("cchTextMax", &self.cchTextMax)
            .field("iImage", &self.iImage)
            .field("iSelectedImage", &self.iSelectedImage)
            .field("cChildren", &self.cChildren)
            .field("lParam", &self.lParam)
            .field("iIntegral", &self.iIntegral)
            .field("uStateEx", &self.uStateEx)
            .field("hwnd", &self.hwnd)
            .field("iExpandedImage", &self.iExpandedImage)
            .field("iReserved", &self.iReserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TVITEMEXA {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask
            && self.hItem == other.hItem
            && self.state == other.state
            && self.stateMask == other.stateMask
            && self.pszText == other.pszText
            && self.cchTextMax == other.cchTextMax
            && self.iImage == other.iImage
            && self.iSelectedImage == other.iSelectedImage
            && self.cChildren == other.cChildren
            && self.lParam == other.lParam
            && self.iIntegral == other.iIntegral
            && self.uStateEx == other.uStateEx
            && self.hwnd == other.hwnd
            && self.iExpandedImage == other.iExpandedImage
            && self.iReserved == other.iReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TVITEMEXA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TVITEMEXA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct TVITEMEXW {
    pub mask: TVITEM_MASK,
    pub hItem: *mut _TREEITEM,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: super::super::Foundation::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: TVITEMEXW_CHILDREN,
    pub lParam: super::super::Foundation::LPARAM,
    pub iIntegral: i32,
    pub uStateEx: u32,
    pub hwnd: super::super::Foundation::HWND,
    pub iExpandedImage: i32,
    pub iReserved: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl TVITEMEXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TVITEMEXW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TVITEMEXW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TVITEMEXW")
            .field("mask", &self.mask)
            .field("hItem", &self.hItem)
            .field("state", &self.state)
            .field("stateMask", &self.stateMask)
            .field("pszText", &self.pszText)
            .field("cchTextMax", &self.cchTextMax)
            .field("iImage", &self.iImage)
            .field("iSelectedImage", &self.iSelectedImage)
            .field("cChildren", &self.cChildren)
            .field("lParam", &self.lParam)
            .field("iIntegral", &self.iIntegral)
            .field("uStateEx", &self.uStateEx)
            .field("hwnd", &self.hwnd)
            .field("iExpandedImage", &self.iExpandedImage)
            .field("iReserved", &self.iReserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TVITEMEXW {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask
            && self.hItem == other.hItem
            && self.state == other.state
            && self.stateMask == other.stateMask
            && self.pszText == other.pszText
            && self.cchTextMax == other.cchTextMax
            && self.iImage == other.iImage
            && self.iSelectedImage == other.iSelectedImage
            && self.cChildren == other.cChildren
            && self.lParam == other.lParam
            && self.iIntegral == other.iIntegral
            && self.uStateEx == other.uStateEx
            && self.hwnd == other.hwnd
            && self.iExpandedImage == other.iExpandedImage
            && self.iReserved == other.iReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TVITEMEXW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TVITEMEXW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TVITEMEXW_CHILDREN(pub i32);
pub const I_ZERO: TVITEMEXW_CHILDREN = TVITEMEXW_CHILDREN(0i32);
pub const I_ONE_OR_MORE: TVITEMEXW_CHILDREN = TVITEMEXW_CHILDREN(1i32);
pub const I_CHILDRENCALLBACK: TVITEMEXW_CHILDREN = TVITEMEXW_CHILDREN(-1i32);
pub const I_CHILDRENAUTO: TVITEMEXW_CHILDREN = TVITEMEXW_CHILDREN(-2i32);
impl ::std::convert::From<i32> for TVITEMEXW_CHILDREN {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TVITEMEXW_CHILDREN {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TVITEMPART(pub i32);
pub const TVGIPR_BUTTON: TVITEMPART = TVITEMPART(1i32);
impl ::std::convert::From<i32> for TVITEMPART {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TVITEMPART {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct TVITEMW {
    pub mask: TVITEM_MASK,
    pub hItem: *mut _TREEITEM,
    pub state: u32,
    pub stateMask: u32,
    pub pszText: super::super::Foundation::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub iSelectedImage: i32,
    pub cChildren: TVITEMEXW_CHILDREN,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl TVITEMW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TVITEMW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TVITEMW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TVITEMW")
            .field("mask", &self.mask)
            .field("hItem", &self.hItem)
            .field("state", &self.state)
            .field("stateMask", &self.stateMask)
            .field("pszText", &self.pszText)
            .field("cchTextMax", &self.cchTextMax)
            .field("iImage", &self.iImage)
            .field("iSelectedImage", &self.iSelectedImage)
            .field("cChildren", &self.cChildren)
            .field("lParam", &self.lParam)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TVITEMW {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.hItem == other.hItem && self.state == other.state && self.stateMask == other.stateMask && self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.iImage == other.iImage && self.iSelectedImage == other.iSelectedImage && self.cChildren == other.cChildren && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TVITEMW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TVITEMW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TVITEM_MASK(pub u32);
pub const TVIF_CHILDREN: TVITEM_MASK = TVITEM_MASK(64u32);
pub const TVIF_DI_SETITEM: TVITEM_MASK = TVITEM_MASK(4096u32);
pub const TVIF_HANDLE: TVITEM_MASK = TVITEM_MASK(16u32);
pub const TVIF_IMAGE: TVITEM_MASK = TVITEM_MASK(2u32);
pub const TVIF_PARAM: TVITEM_MASK = TVITEM_MASK(4u32);
pub const TVIF_SELECTEDIMAGE: TVITEM_MASK = TVITEM_MASK(32u32);
pub const TVIF_STATE: TVITEM_MASK = TVITEM_MASK(8u32);
pub const TVIF_TEXT: TVITEM_MASK = TVITEM_MASK(1u32);
pub const TVIF_EXPANDEDIMAGE: TVITEM_MASK = TVITEM_MASK(512u32);
pub const TVIF_INTEGRAL: TVITEM_MASK = TVITEM_MASK(128u32);
pub const TVIF_STATEEX: TVITEM_MASK = TVITEM_MASK(256u32);
impl ::std::convert::From<u32> for TVITEM_MASK {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TVITEM_MASK {
    type Abi = Self;
}
impl ::std::ops::BitOr for TVITEM_MASK {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for TVITEM_MASK {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for TVITEM_MASK {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for TVITEM_MASK {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for TVITEM_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVI_FIRST: HTREEITEM = HTREEITEM(-65535i32 as _);
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVI_LAST: HTREEITEM = HTREEITEM(-65534i32 as _);
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVI_ROOT: HTREEITEM = HTREEITEM(-65536i32 as _);
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVI_SORT: HTREEITEM = HTREEITEM(-65533i32 as _);
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_CREATEDRAGIMAGE: u32 = 4370u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_DELETEITEM: u32 = 4353u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_EDITLABEL: u32 = 4417u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_EDITLABELA: u32 = 4366u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_EDITLABELW: u32 = 4417u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_ENDEDITLABELNOW: u32 = 4374u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_ENSUREVISIBLE: u32 = 4372u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_EXPAND: u32 = 4354u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_GETBKCOLOR: u32 = 4383u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_GETCOUNT: u32 = 4357u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_GETEDITCONTROL: u32 = 4367u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_GETEXTENDEDSTYLE: u32 = 4397u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_GETIMAGELIST: u32 = 4360u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_GETINDENT: u32 = 4358u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_GETINSERTMARKCOLOR: u32 = 4390u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_GETISEARCHSTRING: u32 = 4416u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_GETISEARCHSTRINGA: u32 = 4375u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_GETISEARCHSTRINGW: u32 = 4416u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_GETITEM: u32 = 4414u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_GETITEMA: u32 = 4364u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_GETITEMHEIGHT: u32 = 4380u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_GETITEMPARTRECT: u32 = 4424u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_GETITEMRECT: u32 = 4356u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_GETITEMSTATE: u32 = 4391u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_GETITEMW: u32 = 4414u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_GETLINECOLOR: u32 = 4393u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_GETNEXTITEM: u32 = 4362u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_GETSCROLLTIME: u32 = 4386u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_GETSELECTEDCOUNT: u32 = 4422u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_GETTEXTCOLOR: u32 = 4384u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_GETTOOLTIPS: u32 = 4377u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_GETUNICODEFORMAT: u32 = 8198u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_GETVISIBLECOUNT: u32 = 4368u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_HITTEST: u32 = 4369u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_INSERTITEM: u32 = 4402u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_INSERTITEMA: u32 = 4352u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_INSERTITEMW: u32 = 4402u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_MAPACCIDTOHTREEITEM: u32 = 4394u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_MAPHTREEITEMTOACCID: u32 = 4395u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_SELECTITEM: u32 = 4363u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_SETAUTOSCROLLINFO: u32 = 4411u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_SETBKCOLOR: u32 = 4381u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_SETBORDER: u32 = 4387u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_SETEXTENDEDSTYLE: u32 = 4396u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_SETHOT: u32 = 4410u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_SETIMAGELIST: u32 = 4361u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_SETINDENT: u32 = 4359u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_SETINSERTMARK: u32 = 4378u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_SETINSERTMARKCOLOR: u32 = 4389u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_SETITEM: u32 = 4415u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_SETITEMA: u32 = 4365u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_SETITEMHEIGHT: u32 = 4379u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_SETITEMW: u32 = 4415u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_SETLINECOLOR: u32 = 4392u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_SETSCROLLTIME: u32 = 4385u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_SETTEXTCOLOR: u32 = 4382u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_SETTOOLTIPS: u32 = 4376u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_SETUNICODEFORMAT: u32 = 8197u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_SHOWINFOTIP: u32 = 4423u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_SORTCHILDREN: u32 = 4371u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVM_SORTCHILDRENCB: u32 = 4373u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVNRET_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVNRET_SKIPNEW: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVNRET_SKIPOLD: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVSBF_XBORDER: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVSBF_YBORDER: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVSIL_NORMAL: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVSIL_STATE: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVSI_NOSINGLEEXPAND: u32 = 32768u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
pub struct TVSORTCB {
    pub hParent: *mut _TREEITEM,
    pub lpfnCompare: ::std::option::Option<PFNTVCOMPARE>,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl TVSORTCB {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TVSORTCB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TVSORTCB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TVSORTCB").field("hParent", &self.hParent).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TVSORTCB {
    fn eq(&self, other: &Self) -> bool {
        self.hParent == other.hParent && self.lpfnCompare.map(|f| f as usize) == other.lpfnCompare.map(|f| f as usize) && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TVSORTCB {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TVSORTCB {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVS_CHECKBOXES: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVS_DISABLEDRAGDROP: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVS_EDITLABELS: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVS_EX_AUTOHSCROLL: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVS_EX_DIMMEDCHECKBOXES: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVS_EX_DOUBLEBUFFER: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVS_EX_DRAWIMAGEASYNC: u32 = 1024u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVS_EX_EXCLUSIONCHECKBOXES: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVS_EX_FADEINOUTEXPANDOS: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVS_EX_MULTISELECT: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVS_EX_NOINDENTSTATE: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVS_EX_NOSINGLECOLLAPSE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVS_EX_PARTIALCHECKBOXES: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVS_EX_RICHTOOLTIP: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVS_FULLROWSELECT: u32 = 4096u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVS_HASBUTTONS: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVS_HASLINES: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVS_INFOTIP: u32 = 2048u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVS_LINESATROOT: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVS_NOHSCROLL: u32 = 32768u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVS_NONEVENHEIGHT: u32 = 16384u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVS_NOSCROLL: u32 = 8192u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVS_NOTOOLTIPS: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVS_RTLREADING: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVS_SHOWSELALWAYS: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVS_SINGLEEXPAND: u32 = 1024u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TVS_TRACKSELECT: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const TV_FIRST: u32 = 4352u32;
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TaskDialog<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
    hwndowner: Param0,
    hinstance: Param1,
    pszwindowtitle: Param2,
    pszmaininstruction: Param3,
    pszcontent: Param4,
    dwcommonbuttons: TASKDIALOG_COMMON_BUTTON_FLAGS,
    pszicon: Param6,
) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TaskDialog(hwndowner: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszwindowtitle: super::super::Foundation::PWSTR, pszmaininstruction: super::super::Foundation::PWSTR, pszcontent: super::super::Foundation::PWSTR, dwcommonbuttons: TASKDIALOG_COMMON_BUTTON_FLAGS, pszicon: super::super::Foundation::PWSTR, pnbutton: *mut i32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        TaskDialog(hwndowner.into_param().abi(), hinstance.into_param().abi(), pszwindowtitle.into_param().abi(), pszmaininstruction.into_param().abi(), pszcontent.into_param().abi(), ::std::mem::transmute(dwcommonbuttons), pszicon.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn TaskDialogIndirect(ptaskconfig: *const TASKDIALOGCONFIG, pnbutton: *mut i32, pnradiobutton: *mut i32, pfverificationflagchecked: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TaskDialogIndirect(ptaskconfig: *const ::std::mem::ManuallyDrop<TASKDIALOGCONFIG>, pnbutton: *mut i32, pnradiobutton: *mut i32, pfverificationflagchecked: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        TaskDialogIndirect(::std::mem::transmute(ptaskconfig), ::std::mem::transmute(pnbutton), ::std::mem::transmute(pnradiobutton), ::std::mem::transmute(pfverificationflagchecked)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct UDACCEL {
    pub nSec: u32,
    pub nInc: u32,
}
impl UDACCEL {}
impl ::std::default::Default for UDACCEL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for UDACCEL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("UDACCEL").field("nSec", &self.nSec).field("nInc", &self.nInc).finish()
    }
}
impl ::std::cmp::PartialEq for UDACCEL {
    fn eq(&self, other: &Self) -> bool {
        self.nSec == other.nSec && self.nInc == other.nInc
    }
}
impl ::std::cmp::Eq for UDACCEL {}
unsafe impl ::windows::runtime::Abi for UDACCEL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const UDM_GETACCEL: u32 = 1132u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const UDM_GETBASE: u32 = 1134u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const UDM_GETBUDDY: u32 = 1130u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const UDM_GETPOS: u32 = 1128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const UDM_GETPOS32: u32 = 1138u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const UDM_GETRANGE: u32 = 1126u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const UDM_GETRANGE32: u32 = 1136u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const UDM_GETUNICODEFORMAT: u32 = 8198u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const UDM_SETACCEL: u32 = 1131u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const UDM_SETBASE: u32 = 1133u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const UDM_SETBUDDY: u32 = 1129u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const UDM_SETPOS: u32 = 1127u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const UDM_SETPOS32: u32 = 1137u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const UDM_SETRANGE: u32 = 1125u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const UDM_SETRANGE32: u32 = 1135u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const UDM_SETUNICODEFORMAT: u32 = 8197u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const UDS_ALIGNLEFT: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const UDS_ALIGNRIGHT: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const UDS_ARROWKEYS: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const UDS_AUTOBUDDY: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const UDS_HORZ: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const UDS_HOTTRACK: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const UDS_NOTHOUSANDS: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const UDS_SETBUDDYINT: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const UDS_WRAP: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const UD_MAXVAL: u32 = 32767u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct USAGE_PROPERTIES {
    pub level: u16,
    pub page: u16,
    pub usage: u16,
    pub logicalMinimum: i32,
    pub logicalMaximum: i32,
    pub unit: u16,
    pub exponent: u16,
    pub count: u8,
    pub physicalMinimum: i32,
    pub physicalMaximum: i32,
}
impl USAGE_PROPERTIES {}
impl ::std::default::Default for USAGE_PROPERTIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for USAGE_PROPERTIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("USAGE_PROPERTIES")
            .field("level", &self.level)
            .field("page", &self.page)
            .field("usage", &self.usage)
            .field("logicalMinimum", &self.logicalMinimum)
            .field("logicalMaximum", &self.logicalMaximum)
            .field("unit", &self.unit)
            .field("exponent", &self.exponent)
            .field("count", &self.count)
            .field("physicalMinimum", &self.physicalMinimum)
            .field("physicalMaximum", &self.physicalMaximum)
            .finish()
    }
}
impl ::std::cmp::PartialEq for USAGE_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.level == other.level && self.page == other.page && self.usage == other.usage && self.logicalMinimum == other.logicalMinimum && self.logicalMaximum == other.logicalMaximum && self.unit == other.unit && self.exponent == other.exponent && self.count == other.count && self.physicalMinimum == other.physicalMinimum && self.physicalMaximum == other.physicalMaximum
    }
}
impl ::std::cmp::Eq for USAGE_PROPERTIES {}
unsafe impl ::windows::runtime::Abi for USAGE_PROPERTIES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UninitializeFlatSB<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(param0: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UninitializeFlatSB(param0: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT;
        }
        UninitializeFlatSB(param0.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdatePanningFeedback<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(hwnd: Param0, ltotaloverpanoffsetx: i32, ltotaloverpanoffsety: i32, fininertia: Param3) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UpdatePanningFeedback(hwnd: super::super::Foundation::HWND, ltotaloverpanoffsetx: i32, ltotaloverpanoffsety: i32, fininertia: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(UpdatePanningFeedback(hwnd.into_param().abi(), ::std::mem::transmute(ltotaloverpanoffsetx), ::std::mem::transmute(ltotaloverpanoffsety), fininertia.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VALIGN(pub i32);
pub const VA_TOP: VALIGN = VALIGN(0i32);
pub const VA_CENTER: VALIGN = VALIGN(1i32);
pub const VA_BOTTOM: VALIGN = VALIGN(2i32);
impl ::std::convert::From<i32> for VALIGN {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VALIGN {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const VIEW_DETAILS: u32 = 3u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const VIEW_LARGEICONS: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const VIEW_LIST: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const VIEW_NETCONNECT: u32 = 9u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const VIEW_NETDISCONNECT: u32 = 10u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const VIEW_NEWFOLDER: u32 = 11u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const VIEW_PARENTFOLDER: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const VIEW_SMALLICONS: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const VIEW_SORTDATE: u32 = 6u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const VIEW_SORTNAME: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const VIEW_SORTSIZE: u32 = 5u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const VIEW_SORTTYPE: u32 = 7u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const VIEW_VIEWMENU: u32 = 12u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WINDOWTHEMEATTRIBUTETYPE(pub i32);
pub const WTA_NONCLIENT: WINDOWTHEMEATTRIBUTETYPE = WINDOWTHEMEATTRIBUTETYPE(1i32);
impl ::std::convert::From<i32> for WINDOWTHEMEATTRIBUTETYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINDOWTHEMEATTRIBUTETYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const WIZ_BODYCX: u32 = 184u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const WIZ_BODYX: u32 = 92u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const WIZ_CXBMP: u32 = 80u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const WIZ_CXDLG: u32 = 276u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const WIZ_CYDLG: u32 = 140u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const WM_CTLCOLOR: u32 = 25u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const WM_MOUSEHOVER: u32 = 673u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const WM_MOUSELEAVE: u32 = 675u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WORD_BREAK_ACTION(pub u32);
pub const WB_CLASSIFY: WORD_BREAK_ACTION = WORD_BREAK_ACTION(3u32);
pub const WB_ISDELIMITER: WORD_BREAK_ACTION = WORD_BREAK_ACTION(2u32);
pub const WB_LEFT: WORD_BREAK_ACTION = WORD_BREAK_ACTION(0u32);
pub const WB_LEFTBREAK: WORD_BREAK_ACTION = WORD_BREAK_ACTION(6u32);
pub const WB_MOVEWORDLEFT: WORD_BREAK_ACTION = WORD_BREAK_ACTION(4u32);
pub const WB_MOVEWORDRIGHT: WORD_BREAK_ACTION = WORD_BREAK_ACTION(5u32);
pub const WB_RIGHT: WORD_BREAK_ACTION = WORD_BREAK_ACTION(1u32);
pub const WB_RIGHTBREAK: WORD_BREAK_ACTION = WORD_BREAK_ACTION(7u32);
impl ::std::convert::From<u32> for WORD_BREAK_ACTION {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WORD_BREAK_ACTION {
    type Abi = Self;
}
impl ::std::ops::BitOr for WORD_BREAK_ACTION {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WORD_BREAK_ACTION {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WORD_BREAK_ACTION {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WORD_BREAK_ACTION {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WORD_BREAK_ACTION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WSB_PROP(pub i32);
pub const WSB_PROP_CXHSCROLL: WSB_PROP = WSB_PROP(2i32);
pub const WSB_PROP_CXHTHUMB: WSB_PROP = WSB_PROP(16i32);
pub const WSB_PROP_CXVSCROLL: WSB_PROP = WSB_PROP(8i32);
pub const WSB_PROP_CYHSCROLL: WSB_PROP = WSB_PROP(4i32);
pub const WSB_PROP_CYVSCROLL: WSB_PROP = WSB_PROP(1i32);
pub const WSB_PROP_CYVTHUMB: WSB_PROP = WSB_PROP(32i32);
pub const WSB_PROP_HBKGCOLOR: WSB_PROP = WSB_PROP(128i32);
pub const WSB_PROP_HSTYLE: WSB_PROP = WSB_PROP(512i32);
pub const WSB_PROP_PALETTE: WSB_PROP = WSB_PROP(2048i32);
pub const WSB_PROP_VBKGCOLOR: WSB_PROP = WSB_PROP(64i32);
pub const WSB_PROP_VSTYLE: WSB_PROP = WSB_PROP(256i32);
pub const WSB_PROP_WINSTYLE: WSB_PROP = WSB_PROP(1024i32);
impl ::std::convert::From<i32> for WSB_PROP {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WSB_PROP {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const WSB_PROP_MASK: i32 = 4095i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub struct WTA_OPTIONS {
    pub dwFlags: u32,
    pub dwMask: u32,
}
impl WTA_OPTIONS {}
impl ::std::default::Default for WTA_OPTIONS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WTA_OPTIONS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WTA_OPTIONS").field("dwFlags", &self.dwFlags).field("dwMask", &self.dwMask).finish()
    }
}
impl ::std::cmp::PartialEq for WTA_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.dwMask == other.dwMask
    }
}
impl ::std::cmp::Eq for WTA_OPTIONS {}
unsafe impl ::windows::runtime::Abi for WTA_OPTIONS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const WTNCA_NODRAWCAPTION: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const WTNCA_NODRAWICON: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const WTNCA_NOMIRRORHELP: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_Controls`*"]
pub const WTNCA_NOSYSMENU: u32 = 4u32;
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct _DPA(pub u8);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct _DSA(pub u8);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct _IMAGELIST(pub u8);
#[doc = "*Required features: `Win32_UI_Controls`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct _LI_METRIC(pub i32);
pub const LIM_SMALL: _LI_METRIC = _LI_METRIC(0i32);
pub const LIM_LARGE: _LI_METRIC = _LI_METRIC(1i32);
impl ::std::convert::From<i32> for _LI_METRIC {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for _LI_METRIC {
    type Abi = Self;
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct _TREEITEM(pub u8);
