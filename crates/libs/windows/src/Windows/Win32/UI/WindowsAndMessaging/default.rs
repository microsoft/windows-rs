impl ::core::default::Default for ACCEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ACCEL {
    fn eq(&self, other: &Self) -> bool {
        self.fVirt == other.fVirt && self.key == other.key && self.cmd == other.cmd
    }
}
impl ::core::cmp::Eq for ACCEL {}
impl ::core::fmt::Debug for ACCEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCEL").field("fVirt", &self.fVirt).field("key", &self.key).field("cmd", &self.cmd).finish()
    }
}
impl ::core::default::Default for ACCEL_VIRT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ACCEL_VIRT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACCEL_VIRT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ACCEL_VIRT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ACCEL_VIRT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ACCEL_VIRT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ACCEL_VIRT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ACCEL_VIRT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ALTTABINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ALTTABINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.cItems == other.cItems && self.cColumns == other.cColumns && self.cRows == other.cRows && self.iColFocus == other.iColFocus && self.iRowFocus == other.iRowFocus && self.cxItem == other.cxItem && self.cyItem == other.cyItem && self.ptStart == other.ptStart
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ALTTABINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ALTTABINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ALTTABINFO").field("cbSize", &self.cbSize).field("cItems", &self.cItems).field("cColumns", &self.cColumns).field("cRows", &self.cRows).field("iColFocus", &self.iColFocus).field("iRowFocus", &self.iRowFocus).field("cxItem", &self.cxItem).field("cyItem", &self.cyItem).field("ptStart", &self.ptStart).finish()
    }
}
impl ::core::default::Default for ANIMATE_WINDOW_FLAGS {
    fn default() -> Self {
        Self(0)
    }
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
impl ::core::default::Default for ANIMATIONINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ANIMATIONINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.iMinAnimate == other.iMinAnimate
    }
}
impl ::core::cmp::Eq for ANIMATIONINFO {}
impl ::core::fmt::Debug for ANIMATIONINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ANIMATIONINFO").field("cbSize", &self.cbSize).field("iMinAnimate", &self.iMinAnimate).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUDIODESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUDIODESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.Enabled == other.Enabled && self.Locale == other.Locale
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUDIODESCRIPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUDIODESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUDIODESCRIPTION").field("cbSize", &self.cbSize).field("Enabled", &self.Enabled).field("Locale", &self.Locale).finish()
    }
}
impl ::core::default::Default for CASCADE_WINDOWS_HOW {
    fn default() -> Self {
        Self(0)
    }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CBTACTIVATESTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CBTACTIVATESTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.fMouse == other.fMouse && self.hWndActive == other.hWndActive
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CBTACTIVATESTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CBTACTIVATESTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CBTACTIVATESTRUCT").field("fMouse", &self.fMouse).field("hWndActive", &self.hWndActive).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CBT_CREATEWNDA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CBT_CREATEWNDA {
    fn eq(&self, other: &Self) -> bool {
        self.lpcs == other.lpcs && self.hwndInsertAfter == other.hwndInsertAfter
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CBT_CREATEWNDA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CBT_CREATEWNDA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CBT_CREATEWNDA").field("lpcs", &self.lpcs).field("hwndInsertAfter", &self.hwndInsertAfter).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CBT_CREATEWNDW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CBT_CREATEWNDW {
    fn eq(&self, other: &Self) -> bool {
        self.lpcs == other.lpcs && self.hwndInsertAfter == other.hwndInsertAfter
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CBT_CREATEWNDW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CBT_CREATEWNDW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CBT_CREATEWNDW").field("lpcs", &self.lpcs).field("hwndInsertAfter", &self.hwndInsertAfter).finish()
    }
}
impl ::core::default::Default for CHANGEFILTERSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CHANGEFILTERSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.ExtStatus == other.ExtStatus
    }
}
impl ::core::cmp::Eq for CHANGEFILTERSTRUCT {}
impl ::core::fmt::Debug for CHANGEFILTERSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGEFILTERSTRUCT").field("cbSize", &self.cbSize).field("ExtStatus", &self.ExtStatus).finish()
    }
}
impl ::core::default::Default for CHANGE_WINDOW_MESSAGE_FILTER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CHANGE_WINDOW_MESSAGE_FILTER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHANGE_WINDOW_MESSAGE_FILTER_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CLIENTCREATESTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CLIENTCREATESTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.hWindowMenu == other.hWindowMenu && self.idFirstChild == other.idFirstChild
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CLIENTCREATESTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CLIENTCREATESTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLIENTCREATESTRUCT").field("hWindowMenu", &self.hWindowMenu).field("idFirstChild", &self.idFirstChild).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CREATESTRUCTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CREATESTRUCTA {
    fn eq(&self, other: &Self) -> bool {
        self.lpCreateParams == other.lpCreateParams && self.hInstance == other.hInstance && self.hMenu == other.hMenu && self.hwndParent == other.hwndParent && self.cy == other.cy && self.cx == other.cx && self.y == other.y && self.x == other.x && self.style == other.style && self.lpszName == other.lpszName && self.lpszClass == other.lpszClass && self.dwExStyle == other.dwExStyle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CREATESTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CREATESTRUCTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREATESTRUCTA").field("lpCreateParams", &self.lpCreateParams).field("hInstance", &self.hInstance).field("hMenu", &self.hMenu).field("hwndParent", &self.hwndParent).field("cy", &self.cy).field("cx", &self.cx).field("y", &self.y).field("x", &self.x).field("style", &self.style).field("lpszName", &self.lpszName).field("lpszClass", &self.lpszClass).field("dwExStyle", &self.dwExStyle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CREATESTRUCTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CREATESTRUCTW {
    fn eq(&self, other: &Self) -> bool {
        self.lpCreateParams == other.lpCreateParams && self.hInstance == other.hInstance && self.hMenu == other.hMenu && self.hwndParent == other.hwndParent && self.cy == other.cy && self.cx == other.cx && self.y == other.y && self.x == other.x && self.style == other.style && self.lpszName == other.lpszName && self.lpszClass == other.lpszClass && self.dwExStyle == other.dwExStyle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CREATESTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CREATESTRUCTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREATESTRUCTW").field("lpCreateParams", &self.lpCreateParams).field("hInstance", &self.hInstance).field("hMenu", &self.hMenu).field("hwndParent", &self.hwndParent).field("cy", &self.cy).field("cx", &self.cx).field("y", &self.y).field("x", &self.x).field("style", &self.style).field("lpszName", &self.lpszName).field("lpszClass", &self.lpszClass).field("dwExStyle", &self.dwExStyle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CURSORINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CURSORINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.flags == other.flags && self.hCursor == other.hCursor && self.ptScreenPos == other.ptScreenPos
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CURSORINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CURSORINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CURSORINFO").field("cbSize", &self.cbSize).field("flags", &self.flags).field("hCursor", &self.hCursor).field("ptScreenPos", &self.ptScreenPos).finish()
    }
}
impl ::core::default::Default for CURSORINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CURSORINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CURSORINFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CURSORSHAPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CURSORSHAPE {
    fn eq(&self, other: &Self) -> bool {
        self.xHotSpot == other.xHotSpot && self.yHotSpot == other.yHotSpot && self.cx == other.cx && self.cy == other.cy && self.cbWidth == other.cbWidth && self.Planes == other.Planes && self.BitsPixel == other.BitsPixel
    }
}
impl ::core::cmp::Eq for CURSORSHAPE {}
impl ::core::fmt::Debug for CURSORSHAPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CURSORSHAPE").field("xHotSpot", &self.xHotSpot).field("yHotSpot", &self.yHotSpot).field("cx", &self.cx).field("cy", &self.cy).field("cbWidth", &self.cbWidth).field("Planes", &self.Planes).field("BitsPixel", &self.BitsPixel).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CWPRETSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CWPRETSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.lResult == other.lResult && self.lParam == other.lParam && self.wParam == other.wParam && self.message == other.message && self.hwnd == other.hwnd
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CWPRETSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CWPRETSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CWPRETSTRUCT").field("lResult", &self.lResult).field("lParam", &self.lParam).field("wParam", &self.wParam).field("message", &self.message).field("hwnd", &self.hwnd).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CWPSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CWPSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.lParam == other.lParam && self.wParam == other.wParam && self.message == other.message && self.hwnd == other.hwnd
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CWPSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CWPSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CWPSTRUCT").field("lParam", &self.lParam).field("wParam", &self.wParam).field("message", &self.message).field("hwnd", &self.hwnd).finish()
    }
}
impl ::core::default::Default for CWP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEBUGHOOKINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEBUGHOOKINFO {
    fn eq(&self, other: &Self) -> bool {
        self.idThread == other.idThread && self.idThreadInstaller == other.idThreadInstaller && self.lParam == other.lParam && self.wParam == other.wParam && self.code == other.code
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEBUGHOOKINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEBUGHOOKINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEBUGHOOKINFO").field("idThread", &self.idThread).field("idThreadInstaller", &self.idThreadInstaller).field("lParam", &self.lParam).field("wParam", &self.wParam).field("code", &self.code).finish()
    }
}
impl ::core::default::Default for DI_FLAGS {
    fn default() -> Self {
        Self(0)
    }
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
impl ::core::default::Default for DLGITEMTEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DLGTEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DROPSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DROPSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.hwndSource == other.hwndSource && self.hwndSink == other.hwndSink && self.wFmt == other.wFmt && self.dwData == other.dwData && self.ptDrop == other.ptDrop && self.dwControlData == other.dwControlData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DROPSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DROPSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DROPSTRUCT").field("hwndSource", &self.hwndSource).field("hwndSink", &self.hwndSink).field("wFmt", &self.wFmt).field("dwData", &self.dwData).field("ptDrop", &self.ptDrop).field("dwControlData", &self.dwControlData).finish()
    }
}
impl ::core::default::Default for EDIT_CONTROL_FEATURE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EDIT_CONTROL_FEATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EDIT_CONTROL_FEATURE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EVENTMSG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EVENTMSG {
    fn eq(&self, other: &Self) -> bool {
        self.message == other.message && self.paramL == other.paramL && self.paramH == other.paramH && self.time == other.time && self.hwnd == other.hwnd
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EVENTMSG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EVENTMSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EVENTMSG").field("message", &self.message).field("paramL", &self.paramL).field("paramH", &self.paramH).field("time", &self.time).field("hwnd", &self.hwnd).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FLASHWINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FLASHWINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.hwnd == other.hwnd && self.dwFlags == other.dwFlags && self.uCount == other.uCount && self.dwTimeout == other.dwTimeout
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FLASHWINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FLASHWINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLASHWINFO").field("cbSize", &self.cbSize).field("hwnd", &self.hwnd).field("dwFlags", &self.dwFlags).field("uCount", &self.uCount).field("dwTimeout", &self.dwTimeout).finish()
    }
}
impl ::core::default::Default for FLASHWINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
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
impl ::core::default::Default for FOREGROUND_WINDOW_LOCK_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FOREGROUND_WINDOW_LOCK_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FOREGROUND_WINDOW_LOCK_CODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for GDI_IMAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GDI_IMAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GDI_IMAGE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for GET_ANCESTOR_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_ANCESTOR_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_ANCESTOR_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for GET_CLASS_LONG_INDEX {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_CLASS_LONG_INDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_CLASS_LONG_INDEX").field(&self.0).finish()
    }
}
impl ::core::default::Default for GET_MENU_DEFAULT_ITEM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
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
impl ::core::default::Default for GET_WINDOW_CMD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_WINDOW_CMD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_WINDOW_CMD").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GUITHREADINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GUITHREADINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.flags == other.flags && self.hwndActive == other.hwndActive && self.hwndFocus == other.hwndFocus && self.hwndCapture == other.hwndCapture && self.hwndMenuOwner == other.hwndMenuOwner && self.hwndMoveSize == other.hwndMoveSize && self.hwndCaret == other.hwndCaret && self.rcCaret == other.rcCaret
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GUITHREADINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GUITHREADINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GUITHREADINFO").field("cbSize", &self.cbSize).field("flags", &self.flags).field("hwndActive", &self.hwndActive).field("hwndFocus", &self.hwndFocus).field("hwndCapture", &self.hwndCapture).field("hwndMenuOwner", &self.hwndMenuOwner).field("hwndMoveSize", &self.hwndMoveSize).field("hwndCaret", &self.hwndCaret).field("rcCaret", &self.rcCaret).finish()
    }
}
impl ::core::default::Default for GUITHREADINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
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
impl ::core::default::Default for HANDEDNESS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HANDEDNESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HANDEDNESS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HARDWAREHOOKSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HARDWAREHOOKSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.hwnd == other.hwnd && self.message == other.message && self.wParam == other.wParam && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HARDWAREHOOKSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HARDWAREHOOKSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HARDWAREHOOKSTRUCT").field("hwnd", &self.hwnd).field("message", &self.message).field("wParam", &self.wParam).field("lParam", &self.lParam).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for ICONINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for ICONINFO {
    fn eq(&self, other: &Self) -> bool {
        self.fIcon == other.fIcon && self.xHotspot == other.xHotspot && self.yHotspot == other.yHotspot && self.hbmMask == other.hbmMask && self.hbmColor == other.hbmColor
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for ICONINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for ICONINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICONINFO").field("fIcon", &self.fIcon).field("xHotspot", &self.xHotspot).field("yHotspot", &self.yHotspot).field("hbmMask", &self.hbmMask).field("hbmColor", &self.hbmColor).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for ICONINFOEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for ICONINFOEXA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.fIcon == other.fIcon && self.xHotspot == other.xHotspot && self.yHotspot == other.yHotspot && self.hbmMask == other.hbmMask && self.hbmColor == other.hbmColor && self.wResID == other.wResID && self.szModName == other.szModName && self.szResName == other.szResName
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for ICONINFOEXA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for ICONINFOEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICONINFOEXA").field("cbSize", &self.cbSize).field("fIcon", &self.fIcon).field("xHotspot", &self.xHotspot).field("yHotspot", &self.yHotspot).field("hbmMask", &self.hbmMask).field("hbmColor", &self.hbmColor).field("wResID", &self.wResID).field("szModName", &self.szModName).field("szResName", &self.szResName).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for ICONINFOEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for ICONINFOEXW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.fIcon == other.fIcon && self.xHotspot == other.xHotspot && self.yHotspot == other.yHotspot && self.hbmMask == other.hbmMask && self.hbmColor == other.hbmColor && self.wResID == other.wResID && self.szModName == other.szModName && self.szResName == other.szResName
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for ICONINFOEXW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for ICONINFOEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICONINFOEXW").field("cbSize", &self.cbSize).field("fIcon", &self.fIcon).field("xHotspot", &self.xHotspot).field("yHotspot", &self.yHotspot).field("hbmMask", &self.hbmMask).field("hbmColor", &self.hbmColor).field("wResID", &self.wResID).field("szModName", &self.szModName).field("szResName", &self.szResName).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for ICONMETRICSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for ICONMETRICSA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.iHorzSpacing == other.iHorzSpacing && self.iVertSpacing == other.iVertSpacing && self.iTitleWrap == other.iTitleWrap && self.lfFont == other.lfFont
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for ICONMETRICSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for ICONMETRICSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICONMETRICSA").field("cbSize", &self.cbSize).field("iHorzSpacing", &self.iHorzSpacing).field("iVertSpacing", &self.iVertSpacing).field("iTitleWrap", &self.iTitleWrap).field("lfFont", &self.lfFont).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for ICONMETRICSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for ICONMETRICSW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.iHorzSpacing == other.iHorzSpacing && self.iVertSpacing == other.iVertSpacing && self.iTitleWrap == other.iTitleWrap && self.lfFont == other.lfFont
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for ICONMETRICSW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for ICONMETRICSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ICONMETRICSW").field("cbSize", &self.cbSize).field("iHorzSpacing", &self.iHorzSpacing).field("iVertSpacing", &self.iVertSpacing).field("iTitleWrap", &self.iTitleWrap).field("lfFont", &self.lfFont).finish()
    }
}
impl ::core::default::Default for IMAGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
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
impl ::core::default::Default for IndexedResourceQualifier {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IndexedResourceQualifier {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value
    }
}
impl ::core::cmp::Eq for IndexedResourceQualifier {}
impl ::core::fmt::Debug for IndexedResourceQualifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IndexedResourceQualifier").field("name", &self.name).field("value", &self.value).finish()
    }
}
impl ::core::default::Default for KBDLLHOOKSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KBDLLHOOKSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.vkCode == other.vkCode && self.scanCode == other.scanCode && self.flags == other.flags && self.time == other.time && self.dwExtraInfo == other.dwExtraInfo
    }
}
impl ::core::cmp::Eq for KBDLLHOOKSTRUCT {}
impl ::core::fmt::Debug for KBDLLHOOKSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KBDLLHOOKSTRUCT").field("vkCode", &self.vkCode).field("scanCode", &self.scanCode).field("flags", &self.flags).field("time", &self.time).field("dwExtraInfo", &self.dwExtraInfo).finish()
    }
}
impl ::core::default::Default for KBDLLHOOKSTRUCT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
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
impl ::core::default::Default for LAYERED_WINDOW_ATTRIBUTES_FLAGS {
    fn default() -> Self {
        Self(0)
    }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MDICREATESTRUCTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MDICREATESTRUCTA {
    fn eq(&self, other: &Self) -> bool {
        self.szClass == other.szClass && self.szTitle == other.szTitle && self.hOwner == other.hOwner && self.x == other.x && self.y == other.y && self.cx == other.cx && self.cy == other.cy && self.style == other.style && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MDICREATESTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MDICREATESTRUCTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MDICREATESTRUCTA").field("szClass", &self.szClass).field("szTitle", &self.szTitle).field("hOwner", &self.hOwner).field("x", &self.x).field("y", &self.y).field("cx", &self.cx).field("cy", &self.cy).field("style", &self.style).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MDICREATESTRUCTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MDICREATESTRUCTW {
    fn eq(&self, other: &Self) -> bool {
        self.szClass == other.szClass && self.szTitle == other.szTitle && self.hOwner == other.hOwner && self.x == other.x && self.y == other.y && self.cx == other.cx && self.cy == other.cy && self.style == other.style && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MDICREATESTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MDICREATESTRUCTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MDICREATESTRUCTW").field("szClass", &self.szClass).field("szTitle", &self.szTitle).field("hOwner", &self.hOwner).field("x", &self.x).field("y", &self.y).field("cx", &self.cx).field("cy", &self.cy).field("style", &self.style).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MDINEXTMENU {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MDINEXTMENU {
    fn eq(&self, other: &Self) -> bool {
        self.hmenuIn == other.hmenuIn && self.hmenuNext == other.hmenuNext && self.hwndNext == other.hwndNext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MDINEXTMENU {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MDINEXTMENU {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MDINEXTMENU").field("hmenuIn", &self.hmenuIn).field("hmenuNext", &self.hmenuNext).field("hwndNext", &self.hwndNext).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MENUBARINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MENUBARINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.rcBar == other.rcBar && self.hMenu == other.hMenu && self.hwndMenu == other.hwndMenu && self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MENUBARINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MENUBARINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MENUBARINFO").field("cbSize", &self.cbSize).field("rcBar", &self.rcBar).field("hMenu", &self.hMenu).field("hwndMenu", &self.hwndMenu).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for MENUGETOBJECTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MENUGETOBJECTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.uPos == other.uPos && self.hmenu == other.hmenu && self.riid == other.riid && self.pvObj == other.pvObj
    }
}
impl ::core::cmp::Eq for MENUGETOBJECTINFO {}
impl ::core::fmt::Debug for MENUGETOBJECTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MENUGETOBJECTINFO").field("dwFlags", &self.dwFlags).field("uPos", &self.uPos).field("hmenu", &self.hmenu).field("riid", &self.riid).field("pvObj", &self.pvObj).finish()
    }
}
impl ::core::default::Default for MENUGETOBJECTINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MENUGETOBJECTINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MENUGETOBJECTINFO_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for MENUINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for MENUINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.fMask == other.fMask && self.dwStyle == other.dwStyle && self.cyMax == other.cyMax && self.hbrBack == other.hbrBack && self.dwContextHelpID == other.dwContextHelpID && self.dwMenuData == other.dwMenuData
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for MENUINFO {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for MENUINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MENUINFO").field("cbSize", &self.cbSize).field("fMask", &self.fMask).field("dwStyle", &self.dwStyle).field("cyMax", &self.cyMax).field("hbrBack", &self.hbrBack).field("dwContextHelpID", &self.dwContextHelpID).field("dwMenuData", &self.dwMenuData).finish()
    }
}
impl ::core::default::Default for MENUINFO_MASK {
    fn default() -> Self {
        Self(0)
    }
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
impl ::core::default::Default for MENUINFO_STYLE {
    fn default() -> Self {
        Self(0)
    }
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
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for MENUITEMINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for MENUITEMINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.fMask == other.fMask && self.fType == other.fType && self.fState == other.fState && self.wID == other.wID && self.hSubMenu == other.hSubMenu && self.hbmpChecked == other.hbmpChecked && self.hbmpUnchecked == other.hbmpUnchecked && self.dwItemData == other.dwItemData && self.dwTypeData == other.dwTypeData && self.cch == other.cch && self.hbmpItem == other.hbmpItem
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for MENUITEMINFOA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for MENUITEMINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MENUITEMINFOA").field("cbSize", &self.cbSize).field("fMask", &self.fMask).field("fType", &self.fType).field("fState", &self.fState).field("wID", &self.wID).field("hSubMenu", &self.hSubMenu).field("hbmpChecked", &self.hbmpChecked).field("hbmpUnchecked", &self.hbmpUnchecked).field("dwItemData", &self.dwItemData).field("dwTypeData", &self.dwTypeData).field("cch", &self.cch).field("hbmpItem", &self.hbmpItem).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for MENUITEMINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for MENUITEMINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.fMask == other.fMask && self.fType == other.fType && self.fState == other.fState && self.wID == other.wID && self.hSubMenu == other.hSubMenu && self.hbmpChecked == other.hbmpChecked && self.hbmpUnchecked == other.hbmpUnchecked && self.dwItemData == other.dwItemData && self.dwTypeData == other.dwTypeData && self.cch == other.cch && self.hbmpItem == other.hbmpItem
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for MENUITEMINFOW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for MENUITEMINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MENUITEMINFOW").field("cbSize", &self.cbSize).field("fMask", &self.fMask).field("fType", &self.fType).field("fState", &self.fState).field("wID", &self.wID).field("hSubMenu", &self.hSubMenu).field("hbmpChecked", &self.hbmpChecked).field("hbmpUnchecked", &self.hbmpUnchecked).field("dwItemData", &self.dwItemData).field("dwTypeData", &self.dwTypeData).field("cch", &self.cch).field("hbmpItem", &self.hbmpItem).finish()
    }
}
impl ::core::default::Default for MENUITEMTEMPLATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MENUITEMTEMPLATE {
    fn eq(&self, other: &Self) -> bool {
        self.mtOption == other.mtOption && self.mtID == other.mtID && self.mtString == other.mtString
    }
}
impl ::core::cmp::Eq for MENUITEMTEMPLATE {}
impl ::core::fmt::Debug for MENUITEMTEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MENUITEMTEMPLATE").field("mtOption", &self.mtOption).field("mtID", &self.mtID).field("mtString", &self.mtString).finish()
    }
}
impl ::core::default::Default for MENUITEMTEMPLATEHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MENUITEMTEMPLATEHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.versionNumber == other.versionNumber && self.offset == other.offset
    }
}
impl ::core::cmp::Eq for MENUITEMTEMPLATEHEADER {}
impl ::core::fmt::Debug for MENUITEMTEMPLATEHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MENUITEMTEMPLATEHEADER").field("versionNumber", &self.versionNumber).field("offset", &self.offset).finish()
    }
}
impl ::core::default::Default for MENU_ITEM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
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
impl ::core::default::Default for MENU_ITEM_MASK {
    fn default() -> Self {
        Self(0)
    }
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
impl ::core::default::Default for MENU_ITEM_STATE {
    fn default() -> Self {
        Self(0)
    }
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
impl ::core::default::Default for MENU_ITEM_TYPE {
    fn default() -> Self {
        Self(0)
    }
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
impl ::core::default::Default for MESSAGEBOX_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MESSAGEBOX_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MESSAGEBOX_RESULT").field(&self.0).finish()
    }
}
impl ::core::default::Default for MESSAGEBOX_STYLE {
    fn default() -> Self {
        Self(0)
    }
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
impl ::core::default::Default for MESSAGE_RESOURCE_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MESSAGE_RESOURCE_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.LowId == other.LowId && self.HighId == other.HighId && self.OffsetToEntries == other.OffsetToEntries
    }
}
impl ::core::cmp::Eq for MESSAGE_RESOURCE_BLOCK {}
impl ::core::fmt::Debug for MESSAGE_RESOURCE_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MESSAGE_RESOURCE_BLOCK").field("LowId", &self.LowId).field("HighId", &self.HighId).field("OffsetToEntries", &self.OffsetToEntries).finish()
    }
}
impl ::core::default::Default for MESSAGE_RESOURCE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MESSAGE_RESOURCE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfBlocks == other.NumberOfBlocks && self.Blocks == other.Blocks
    }
}
impl ::core::cmp::Eq for MESSAGE_RESOURCE_DATA {}
impl ::core::fmt::Debug for MESSAGE_RESOURCE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MESSAGE_RESOURCE_DATA").field("NumberOfBlocks", &self.NumberOfBlocks).field("Blocks", &self.Blocks).finish()
    }
}
impl ::core::default::Default for MESSAGE_RESOURCE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MESSAGE_RESOURCE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.Flags == other.Flags && self.Text == other.Text
    }
}
impl ::core::cmp::Eq for MESSAGE_RESOURCE_ENTRY {}
impl ::core::fmt::Debug for MESSAGE_RESOURCE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MESSAGE_RESOURCE_ENTRY").field("Length", &self.Length).field("Flags", &self.Flags).field("Text", &self.Text).finish()
    }
}
impl ::core::default::Default for MINIMIZEDMETRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MINIMIZEDMETRICS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.iWidth == other.iWidth && self.iHorzGap == other.iHorzGap && self.iVertGap == other.iVertGap && self.iArrange == other.iArrange
    }
}
impl ::core::cmp::Eq for MINIMIZEDMETRICS {}
impl ::core::fmt::Debug for MINIMIZEDMETRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MINIMIZEDMETRICS").field("cbSize", &self.cbSize).field("iWidth", &self.iWidth).field("iHorzGap", &self.iHorzGap).field("iVertGap", &self.iVertGap).field("iArrange", &self.iArrange).finish()
    }
}
impl ::core::default::Default for MINIMIZEDMETRICS_ARRANGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MINIMIZEDMETRICS_ARRANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MINIMIZEDMETRICS_ARRANGE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MINMAXINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MINMAXINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ptReserved == other.ptReserved && self.ptMaxSize == other.ptMaxSize && self.ptMaxPosition == other.ptMaxPosition && self.ptMinTrackSize == other.ptMinTrackSize && self.ptMaxTrackSize == other.ptMaxTrackSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MINMAXINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MINMAXINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MINMAXINFO").field("ptReserved", &self.ptReserved).field("ptMaxSize", &self.ptMaxSize).field("ptMaxPosition", &self.ptMaxPosition).field("ptMinTrackSize", &self.ptMinTrackSize).field("ptMaxTrackSize", &self.ptMaxTrackSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MOUSEHOOKSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MOUSEHOOKSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.pt == other.pt && self.hwnd == other.hwnd && self.wHitTestCode == other.wHitTestCode && self.dwExtraInfo == other.dwExtraInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MOUSEHOOKSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MOUSEHOOKSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOUSEHOOKSTRUCT").field("pt", &self.pt).field("hwnd", &self.hwnd).field("wHitTestCode", &self.wHitTestCode).field("dwExtraInfo", &self.dwExtraInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MOUSEHOOKSTRUCTEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MOUSEHOOKSTRUCTEX {
    fn eq(&self, other: &Self) -> bool {
        self.Base == other.Base && self.mouseData == other.mouseData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MOUSEHOOKSTRUCTEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MOUSEHOOKSTRUCTEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOUSEHOOKSTRUCTEX").field("Base", &self.Base).field("mouseData", &self.mouseData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MSG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MSG {
    fn eq(&self, other: &Self) -> bool {
        self.hwnd == other.hwnd && self.message == other.message && self.wParam == other.wParam && self.lParam == other.lParam && self.time == other.time && self.pt == other.pt
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MSG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSG").field("hwnd", &self.hwnd).field("message", &self.message).field("wParam", &self.wParam).field("lParam", &self.lParam).field("time", &self.time).field("pt", &self.pt).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::default::Default for MSGBOXPARAMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
impl ::core::default::Default for MSGBOXPARAMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MSGFLTINFO_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MSGFLTINFO_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSGFLTINFO_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MSG_WAIT_FOR_MULTIPLE_OBJECTS_EX_FLAGS {
    fn default() -> Self {
        Self(0)
    }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MSLLHOOKSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MSLLHOOKSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.pt == other.pt && self.mouseData == other.mouseData && self.flags == other.flags && self.time == other.time && self.dwExtraInfo == other.dwExtraInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MSLLHOOKSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MSLLHOOKSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MSLLHOOKSTRUCT").field("pt", &self.pt).field("mouseData", &self.mouseData).field("flags", &self.flags).field("time", &self.time).field("dwExtraInfo", &self.dwExtraInfo).finish()
    }
}
impl ::core::default::Default for MrmDumpType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MrmDumpType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MrmDumpType").field(&self.0).finish()
    }
}
impl ::core::default::Default for MrmIndexerFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MrmIndexerFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MrmIndexerFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for MrmPackagingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MrmPackagingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MrmPackagingMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for MrmPackagingOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MrmPackagingOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MrmPackagingOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for MrmPlatformVersion {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MrmPlatformVersion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MrmPlatformVersion").field(&self.0).finish()
    }
}
impl ::core::default::Default for MrmResourceIndexerHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MrmResourceIndexerHandle {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}
impl ::core::cmp::Eq for MrmResourceIndexerHandle {}
impl ::core::fmt::Debug for MrmResourceIndexerHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MrmResourceIndexerHandle").field("handle", &self.handle).finish()
    }
}
impl ::core::default::Default for MrmResourceIndexerMessage {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MrmResourceIndexerMessage {
    fn eq(&self, other: &Self) -> bool {
        self.severity == other.severity && self.id == other.id && self.text == other.text
    }
}
impl ::core::cmp::Eq for MrmResourceIndexerMessage {}
impl ::core::fmt::Debug for MrmResourceIndexerMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MrmResourceIndexerMessage").field("severity", &self.severity).field("id", &self.id).field("text", &self.text).finish()
    }
}
impl ::core::default::Default for MrmResourceIndexerMessageSeverity {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MrmResourceIndexerMessageSeverity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MrmResourceIndexerMessageSeverity").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NCCALCSIZE_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NCCALCSIZE_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.rgrc == other.rgrc && self.lppos == other.lppos
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NCCALCSIZE_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NCCALCSIZE_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NCCALCSIZE_PARAMS").field("rgrc", &self.rgrc).field("lppos", &self.lppos).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for NONCLIENTMETRICSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for NONCLIENTMETRICSA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.iBorderWidth == other.iBorderWidth && self.iScrollWidth == other.iScrollWidth && self.iScrollHeight == other.iScrollHeight && self.iCaptionWidth == other.iCaptionWidth && self.iCaptionHeight == other.iCaptionHeight && self.lfCaptionFont == other.lfCaptionFont && self.iSmCaptionWidth == other.iSmCaptionWidth && self.iSmCaptionHeight == other.iSmCaptionHeight && self.lfSmCaptionFont == other.lfSmCaptionFont && self.iMenuWidth == other.iMenuWidth && self.iMenuHeight == other.iMenuHeight && self.lfMenuFont == other.lfMenuFont && self.lfStatusFont == other.lfStatusFont && self.lfMessageFont == other.lfMessageFont && self.iPaddedBorderWidth == other.iPaddedBorderWidth
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for NONCLIENTMETRICSA {}
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
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for NONCLIENTMETRICSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for NONCLIENTMETRICSW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.iBorderWidth == other.iBorderWidth && self.iScrollWidth == other.iScrollWidth && self.iScrollHeight == other.iScrollHeight && self.iCaptionWidth == other.iCaptionWidth && self.iCaptionHeight == other.iCaptionHeight && self.lfCaptionFont == other.lfCaptionFont && self.iSmCaptionWidth == other.iSmCaptionWidth && self.iSmCaptionHeight == other.iSmCaptionHeight && self.lfSmCaptionFont == other.lfSmCaptionFont && self.iMenuWidth == other.iMenuWidth && self.iMenuHeight == other.iMenuHeight && self.lfMenuFont == other.lfMenuFont && self.lfStatusFont == other.lfStatusFont && self.lfMessageFont == other.lfMessageFont && self.iPaddedBorderWidth == other.iPaddedBorderWidth
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for NONCLIENTMETRICSW {}
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
impl ::core::default::Default for OBJECT_IDENTIFIER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OBJECT_IDENTIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OBJECT_IDENTIFIER").field(&self.0).finish()
    }
}
impl ::core::default::Default for PEEK_MESSAGE_REMOVE_TYPE {
    fn default() -> Self {
        Self(0)
    }
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
impl ::core::default::Default for POINTER_INPUT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POINTER_INPUT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POINTER_INPUT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for QUEUE_STATUS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SCROLLBARINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SCROLLBARINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.rcScrollBar == other.rcScrollBar && self.dxyLineButton == other.dxyLineButton && self.xyThumbTop == other.xyThumbTop && self.xyThumbBottom == other.xyThumbBottom && self.reserved == other.reserved && self.rgstate == other.rgstate
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SCROLLBARINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SCROLLBARINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCROLLBARINFO").field("cbSize", &self.cbSize).field("rcScrollBar", &self.rcScrollBar).field("dxyLineButton", &self.dxyLineButton).field("xyThumbTop", &self.xyThumbTop).field("xyThumbBottom", &self.xyThumbBottom).field("reserved", &self.reserved).field("rgstate", &self.rgstate).finish()
    }
}
impl ::core::default::Default for SCROLLBAR_COMMAND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCROLLBAR_COMMAND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCROLLBAR_COMMAND").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCROLLBAR_CONSTANTS {
    fn default() -> Self {
        Self(0)
    }
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
impl ::core::default::Default for SCROLLINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCROLLINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.fMask == other.fMask && self.nMin == other.nMin && self.nMax == other.nMax && self.nPage == other.nPage && self.nPos == other.nPos && self.nTrackPos == other.nTrackPos
    }
}
impl ::core::cmp::Eq for SCROLLINFO {}
impl ::core::fmt::Debug for SCROLLINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCROLLINFO").field("cbSize", &self.cbSize).field("fMask", &self.fMask).field("nMin", &self.nMin).field("nMax", &self.nMax).field("nPage", &self.nPage).field("nPos", &self.nPos).field("nTrackPos", &self.nTrackPos).finish()
    }
}
impl ::core::default::Default for SCROLLINFO_MASK {
    fn default() -> Self {
        Self(0)
    }
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
impl ::core::default::Default for SEND_MESSAGE_TIMEOUT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
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
impl ::core::default::Default for SET_WINDOW_POS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SHELLHOOKINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SHELLHOOKINFO {
    fn eq(&self, other: &Self) -> bool {
        self.hwnd == other.hwnd && self.rc == other.rc
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SHELLHOOKINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SHELLHOOKINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHELLHOOKINFO").field("hwnd", &self.hwnd).field("rc", &self.rc).finish()
    }
}
impl ::core::default::Default for SHOW_WINDOW_CMD {
    fn default() -> Self {
        Self(0)
    }
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
impl ::core::default::Default for STYLESTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STYLESTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.styleOld == other.styleOld && self.styleNew == other.styleNew
    }
}
impl ::core::cmp::Eq for STYLESTRUCT {}
impl ::core::fmt::Debug for STYLESTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STYLESTRUCT").field("styleOld", &self.styleOld).field("styleNew", &self.styleNew).finish()
    }
}
impl ::core::default::Default for SYSTEM_CURSOR_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYSTEM_CURSOR_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEM_CURSOR_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYSTEM_METRICS_INDEX {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYSTEM_METRICS_INDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEM_METRICS_INDEX").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYSTEM_PARAMETERS_INFO_ACTION {
    fn default() -> Self {
        Self(0)
    }
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
impl ::core::default::Default for SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
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
impl ::core::default::Default for TILE_WINDOWS_HOW {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TILE_WINDOWS_HOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TILE_WINDOWS_HOW").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TITLEBARINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TITLEBARINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.rcTitleBar == other.rcTitleBar && self.rgstate == other.rgstate
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TITLEBARINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TITLEBARINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TITLEBARINFO").field("cbSize", &self.cbSize).field("rcTitleBar", &self.rcTitleBar).field("rgstate", &self.rgstate).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TITLEBARINFOEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TITLEBARINFOEX {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.rcTitleBar == other.rcTitleBar && self.rgstate == other.rgstate && self.rgrect == other.rgrect
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TITLEBARINFOEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TITLEBARINFOEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TITLEBARINFOEX").field("cbSize", &self.cbSize).field("rcTitleBar", &self.rcTitleBar).field("rgstate", &self.rgstate).field("rgrect", &self.rgrect).finish()
    }
}
impl ::core::default::Default for TOUCHPREDICTIONPARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TOUCHPREDICTIONPARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwLatency == other.dwLatency && self.dwSampleTime == other.dwSampleTime && self.bUseHWTimeStamp == other.bUseHWTimeStamp
    }
}
impl ::core::cmp::Eq for TOUCHPREDICTIONPARAMETERS {}
impl ::core::fmt::Debug for TOUCHPREDICTIONPARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOUCHPREDICTIONPARAMETERS").field("cbSize", &self.cbSize).field("dwLatency", &self.dwLatency).field("dwSampleTime", &self.dwSampleTime).field("bUseHWTimeStamp", &self.bUseHWTimeStamp).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TPMPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TPMPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.rcExclude == other.rcExclude
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TPMPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TPMPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TPMPARAMS").field("cbSize", &self.cbSize).field("rcExclude", &self.rcExclude).finish()
    }
}
impl ::core::default::Default for TRACK_POPUP_MENU_FLAGS {
    fn default() -> Self {
        Self(0)
    }
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for UPDATELAYEREDWINDOWINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for UPDATELAYEREDWINDOWINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.hdcDst == other.hdcDst && self.pptDst == other.pptDst && self.psize == other.psize && self.hdcSrc == other.hdcSrc && self.pptSrc == other.pptSrc && self.crKey == other.crKey && self.pblend == other.pblend && self.dwFlags == other.dwFlags && self.prcDirty == other.prcDirty
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for UPDATELAYEREDWINDOWINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for UPDATELAYEREDWINDOWINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UPDATELAYEREDWINDOWINFO").field("cbSize", &self.cbSize).field("hdcDst", &self.hdcDst).field("pptDst", &self.pptDst).field("psize", &self.psize).field("hdcSrc", &self.hdcSrc).field("pptSrc", &self.pptSrc).field("crKey", &self.crKey).field("pblend", &self.pblend).field("dwFlags", &self.dwFlags).field("prcDirty", &self.prcDirty).finish()
    }
}
impl ::core::default::Default for UPDATE_LAYERED_WINDOW_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UPDATE_LAYERED_WINDOW_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UPDATE_LAYERED_WINDOW_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINDOWINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINDOWINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.rcWindow == other.rcWindow && self.rcClient == other.rcClient && self.dwStyle == other.dwStyle && self.dwExStyle == other.dwExStyle && self.dwWindowStatus == other.dwWindowStatus && self.cxWindowBorders == other.cxWindowBorders && self.cyWindowBorders == other.cyWindowBorders && self.atomWindowType == other.atomWindowType && self.wCreatorVersion == other.wCreatorVersion
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINDOWINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINDOWINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINDOWINFO").field("cbSize", &self.cbSize).field("rcWindow", &self.rcWindow).field("rcClient", &self.rcClient).field("dwStyle", &self.dwStyle).field("dwExStyle", &self.dwExStyle).field("dwWindowStatus", &self.dwWindowStatus).field("cxWindowBorders", &self.cxWindowBorders).field("cyWindowBorders", &self.cyWindowBorders).field("atomWindowType", &self.atomWindowType).field("wCreatorVersion", &self.wCreatorVersion).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINDOWPLACEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINDOWPLACEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.flags == other.flags && self.showCmd == other.showCmd && self.ptMinPosition == other.ptMinPosition && self.ptMaxPosition == other.ptMaxPosition && self.rcNormalPosition == other.rcNormalPosition
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINDOWPLACEMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINDOWPLACEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINDOWPLACEMENT").field("length", &self.length).field("flags", &self.flags).field("showCmd", &self.showCmd).field("ptMinPosition", &self.ptMinPosition).field("ptMaxPosition", &self.ptMaxPosition).field("rcNormalPosition", &self.rcNormalPosition).finish()
    }
}
impl ::core::default::Default for WINDOWPLACEMENT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WINDOWPOS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WINDOWPOS {
    fn eq(&self, other: &Self) -> bool {
        self.hwnd == other.hwnd && self.hwndInsertAfter == other.hwndInsertAfter && self.x == other.x && self.y == other.y && self.cx == other.cx && self.cy == other.cy && self.flags == other.flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WINDOWPOS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WINDOWPOS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINDOWPOS").field("hwnd", &self.hwnd).field("hwndInsertAfter", &self.hwndInsertAfter).field("x", &self.x).field("y", &self.y).field("cx", &self.cx).field("cy", &self.cy).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for WINDOWS_HOOK_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINDOWS_HOOK_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINDOWS_HOOK_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINDOW_DISPLAY_AFFINITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINDOW_DISPLAY_AFFINITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINDOW_DISPLAY_AFFINITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINDOW_EX_STYLE {
    fn default() -> Self {
        Self(0)
    }
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
impl ::core::default::Default for WINDOW_LONG_PTR_INDEX {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINDOW_LONG_PTR_INDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINDOW_LONG_PTR_INDEX").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINDOW_MESSAGE_FILTER_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WINDOW_MESSAGE_FILTER_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WINDOW_MESSAGE_FILTER_ACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINDOW_STYLE {
    fn default() -> Self {
        Self(0)
    }
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for WNDCLASSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for WNDCLASSEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for WNDCLASSEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for WNDCLASSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WNDCLASS_STYLES {
    fn default() -> Self {
        Self(0)
    }
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
