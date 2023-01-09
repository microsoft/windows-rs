impl ::core::default::Default for ACTIVATE_KEYBOARD_LAYOUT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ACTIVATE_KEYBOARD_LAYOUT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACTIVATE_KEYBOARD_LAYOUT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for DEADKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEADKEY {
    fn eq(&self, other: &Self) -> bool {
        self.dwBoth == other.dwBoth && self.wchComposed == other.wchComposed && self.uFlags == other.uFlags
    }
}
impl ::core::cmp::Eq for DEADKEY {}
impl ::core::fmt::Debug for DEADKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEADKEY").field("dwBoth", &self.dwBoth).field("wchComposed", &self.wchComposed).field("uFlags", &self.uFlags).finish()
    }
}
impl ::core::default::Default for GET_MOUSE_MOVE_POINTS_EX_RESOLUTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_MOUSE_MOVE_POINTS_EX_RESOLUTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_MOUSE_MOVE_POINTS_EX_RESOLUTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for HARDWAREINPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HARDWAREINPUT {
    fn eq(&self, other: &Self) -> bool {
        self.uMsg == other.uMsg && self.wParamL == other.wParamL && self.wParamH == other.wParamH
    }
}
impl ::core::cmp::Eq for HARDWAREINPUT {}
impl ::core::fmt::Debug for HARDWAREINPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HARDWAREINPUT").field("uMsg", &self.uMsg).field("wParamL", &self.wParamL).field("wParamH", &self.wParamH).finish()
    }
}
impl ::core::default::Default for HOT_KEY_MODIFIERS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HOT_KEY_MODIFIERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HOT_KEY_MODIFIERS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for HOT_KEY_MODIFIERS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for HOT_KEY_MODIFIERS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for HOT_KEY_MODIFIERS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for HOT_KEY_MODIFIERS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for HOT_KEY_MODIFIERS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for INPUT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for INPUT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INPUT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KBDNLSTABLES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KBDNLSTABLES {
    fn eq(&self, other: &Self) -> bool {
        self.OEMIdentifier == other.OEMIdentifier && self.LayoutInformation == other.LayoutInformation && self.NumOfVkToF == other.NumOfVkToF && self.pVkToF == other.pVkToF && self.NumOfMouseVKey == other.NumOfMouseVKey && self.pusMouseVKey == other.pusMouseVKey
    }
}
impl ::core::cmp::Eq for KBDNLSTABLES {}
impl ::core::fmt::Debug for KBDNLSTABLES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KBDNLSTABLES").field("OEMIdentifier", &self.OEMIdentifier).field("LayoutInformation", &self.LayoutInformation).field("NumOfVkToF", &self.NumOfVkToF).field("pVkToF", &self.pVkToF).field("NumOfMouseVKey", &self.NumOfMouseVKey).field("pusMouseVKey", &self.pusMouseVKey).finish()
    }
}
impl ::core::default::Default for KBDTABLES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KBDTABLES {
    fn eq(&self, other: &Self) -> bool {
        self.pCharModifiers == other.pCharModifiers && self.pVkToWcharTable == other.pVkToWcharTable && self.pDeadKey == other.pDeadKey && self.pKeyNames == other.pKeyNames && self.pKeyNamesExt == other.pKeyNamesExt && self.pKeyNamesDead == other.pKeyNamesDead && self.pusVSCtoVK == other.pusVSCtoVK && self.bMaxVSCtoVK == other.bMaxVSCtoVK && self.pVSCtoVK_E0 == other.pVSCtoVK_E0 && self.pVSCtoVK_E1 == other.pVSCtoVK_E1 && self.fLocaleFlags == other.fLocaleFlags && self.nLgMax == other.nLgMax && self.cbLgEntry == other.cbLgEntry && self.pLigature == other.pLigature && self.dwType == other.dwType && self.dwSubType == other.dwSubType
    }
}
impl ::core::cmp::Eq for KBDTABLES {}
impl ::core::fmt::Debug for KBDTABLES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KBDTABLES")
            .field("pCharModifiers", &self.pCharModifiers)
            .field("pVkToWcharTable", &self.pVkToWcharTable)
            .field("pDeadKey", &self.pDeadKey)
            .field("pKeyNames", &self.pKeyNames)
            .field("pKeyNamesExt", &self.pKeyNamesExt)
            .field("pKeyNamesDead", &self.pKeyNamesDead)
            .field("pusVSCtoVK", &self.pusVSCtoVK)
            .field("bMaxVSCtoVK", &self.bMaxVSCtoVK)
            .field("pVSCtoVK_E0", &self.pVSCtoVK_E0)
            .field("pVSCtoVK_E1", &self.pVSCtoVK_E1)
            .field("fLocaleFlags", &self.fLocaleFlags)
            .field("nLgMax", &self.nLgMax)
            .field("cbLgEntry", &self.cbLgEntry)
            .field("pLigature", &self.pLigature)
            .field("dwType", &self.dwType)
            .field("dwSubType", &self.dwSubType)
            .finish()
    }
}
impl ::core::default::Default for KBDTABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KBDTABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.wszDllName == other.wszDllName && self.dwType == other.dwType && self.dwSubType == other.dwSubType
    }
}
impl ::core::cmp::Eq for KBDTABLE_DESC {}
impl ::core::fmt::Debug for KBDTABLE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KBDTABLE_DESC").field("wszDllName", &self.wszDllName).field("dwType", &self.dwType).field("dwSubType", &self.dwSubType).finish()
    }
}
impl ::core::default::Default for KBDTABLE_MULTI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KBDTABLE_MULTI {
    fn eq(&self, other: &Self) -> bool {
        self.nTables == other.nTables && self.aKbdTables == other.aKbdTables
    }
}
impl ::core::cmp::Eq for KBDTABLE_MULTI {}
impl ::core::fmt::Debug for KBDTABLE_MULTI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KBDTABLE_MULTI").field("nTables", &self.nTables).field("aKbdTables", &self.aKbdTables).finish()
    }
}
impl ::core::default::Default for KBD_TYPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KBD_TYPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwType == other.dwType && self.dwSubType == other.dwSubType
    }
}
impl ::core::cmp::Eq for KBD_TYPE_INFO {}
impl ::core::fmt::Debug for KBD_TYPE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KBD_TYPE_INFO").field("dwVersion", &self.dwVersion).field("dwType", &self.dwType).field("dwSubType", &self.dwSubType).finish()
    }
}
impl ::core::default::Default for KEYBDINPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KEYBDINPUT {
    fn eq(&self, other: &Self) -> bool {
        self.wVk == other.wVk && self.wScan == other.wScan && self.dwFlags == other.dwFlags && self.time == other.time && self.dwExtraInfo == other.dwExtraInfo
    }
}
impl ::core::cmp::Eq for KEYBDINPUT {}
impl ::core::fmt::Debug for KEYBDINPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KEYBDINPUT").field("wVk", &self.wVk).field("wScan", &self.wScan).field("dwFlags", &self.dwFlags).field("time", &self.time).field("dwExtraInfo", &self.dwExtraInfo).finish()
    }
}
impl ::core::default::Default for KEYBD_EVENT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KEYBD_EVENT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KEYBD_EVENT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for KEYBD_EVENT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for KEYBD_EVENT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for KEYBD_EVENT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for KEYBD_EVENT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for KEYBD_EVENT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for LASTINPUTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LASTINPUTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwTime == other.dwTime
    }
}
impl ::core::cmp::Eq for LASTINPUTINFO {}
impl ::core::fmt::Debug for LASTINPUTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LASTINPUTINFO").field("cbSize", &self.cbSize).field("dwTime", &self.dwTime).finish()
    }
}
impl ::core::default::Default for LIGATURE1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LIGATURE1 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey && self.ModificationNumber == other.ModificationNumber && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for LIGATURE1 {}
impl ::core::fmt::Debug for LIGATURE1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LIGATURE1").field("VirtualKey", &self.VirtualKey).field("ModificationNumber", &self.ModificationNumber).field("wch", &self.wch).finish()
    }
}
impl ::core::default::Default for LIGATURE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LIGATURE2 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey && self.ModificationNumber == other.ModificationNumber && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for LIGATURE2 {}
impl ::core::fmt::Debug for LIGATURE2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LIGATURE2").field("VirtualKey", &self.VirtualKey).field("ModificationNumber", &self.ModificationNumber).field("wch", &self.wch).finish()
    }
}
impl ::core::default::Default for LIGATURE3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LIGATURE3 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey && self.ModificationNumber == other.ModificationNumber && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for LIGATURE3 {}
impl ::core::fmt::Debug for LIGATURE3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LIGATURE3").field("VirtualKey", &self.VirtualKey).field("ModificationNumber", &self.ModificationNumber).field("wch", &self.wch).finish()
    }
}
impl ::core::default::Default for LIGATURE4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LIGATURE4 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey && self.ModificationNumber == other.ModificationNumber && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for LIGATURE4 {}
impl ::core::fmt::Debug for LIGATURE4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LIGATURE4").field("VirtualKey", &self.VirtualKey).field("ModificationNumber", &self.ModificationNumber).field("wch", &self.wch).finish()
    }
}
impl ::core::default::Default for LIGATURE5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LIGATURE5 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey && self.ModificationNumber == other.ModificationNumber && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for LIGATURE5 {}
impl ::core::fmt::Debug for LIGATURE5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LIGATURE5").field("VirtualKey", &self.VirtualKey).field("ModificationNumber", &self.ModificationNumber).field("wch", &self.wch).finish()
    }
}
impl ::core::default::Default for MAP_VIRTUAL_KEY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MAP_VIRTUAL_KEY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MAP_VIRTUAL_KEY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MODIFIERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MODIFIERS {
    fn eq(&self, other: &Self) -> bool {
        self.pVkToBit == other.pVkToBit && self.wMaxModBits == other.wMaxModBits && self.ModNumber == other.ModNumber
    }
}
impl ::core::cmp::Eq for MODIFIERS {}
impl ::core::fmt::Debug for MODIFIERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MODIFIERS").field("pVkToBit", &self.pVkToBit).field("wMaxModBits", &self.wMaxModBits).field("ModNumber", &self.ModNumber).finish()
    }
}
impl ::core::default::Default for MOUSEINPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MOUSEINPUT {
    fn eq(&self, other: &Self) -> bool {
        self.dx == other.dx && self.dy == other.dy && self.mouseData == other.mouseData && self.dwFlags == other.dwFlags && self.time == other.time && self.dwExtraInfo == other.dwExtraInfo
    }
}
impl ::core::cmp::Eq for MOUSEINPUT {}
impl ::core::fmt::Debug for MOUSEINPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOUSEINPUT").field("dx", &self.dx).field("dy", &self.dy).field("mouseData", &self.mouseData).field("dwFlags", &self.dwFlags).field("time", &self.time).field("dwExtraInfo", &self.dwExtraInfo).finish()
    }
}
impl ::core::default::Default for MOUSEMOVEPOINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MOUSEMOVEPOINT {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.time == other.time && self.dwExtraInfo == other.dwExtraInfo
    }
}
impl ::core::cmp::Eq for MOUSEMOVEPOINT {}
impl ::core::fmt::Debug for MOUSEMOVEPOINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOUSEMOVEPOINT").field("x", &self.x).field("y", &self.y).field("time", &self.time).field("dwExtraInfo", &self.dwExtraInfo).finish()
    }
}
impl ::core::default::Default for MOUSE_EVENT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MOUSE_EVENT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MOUSE_EVENT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MOUSE_EVENT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MOUSE_EVENT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MOUSE_EVENT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MOUSE_EVENT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MOUSE_EVENT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TRACKMOUSEEVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TRACKMOUSEEVENT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.hwndTrack == other.hwndTrack && self.dwHoverTime == other.dwHoverTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TRACKMOUSEEVENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TRACKMOUSEEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRACKMOUSEEVENT").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("hwndTrack", &self.hwndTrack).field("dwHoverTime", &self.dwHoverTime).finish()
    }
}
impl ::core::default::Default for TRACKMOUSEEVENT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TRACKMOUSEEVENT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TRACKMOUSEEVENT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TRACKMOUSEEVENT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TRACKMOUSEEVENT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TRACKMOUSEEVENT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TRACKMOUSEEVENT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TRACKMOUSEEVENT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for VIRTUAL_KEY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VIRTUAL_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VIRTUAL_KEY").field(&self.0).finish()
    }
}
impl ::core::default::Default for VK_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VK_F {
    fn eq(&self, other: &Self) -> bool {
        self.Vk == other.Vk && self.NLSFEProcType == other.NLSFEProcType && self.NLSFEProcCurrent == other.NLSFEProcCurrent && self.NLSFEProcSwitch == other.NLSFEProcSwitch && self.NLSFEProc == other.NLSFEProc && self.NLSFEProcAlt == other.NLSFEProcAlt
    }
}
impl ::core::cmp::Eq for VK_F {}
impl ::core::fmt::Debug for VK_F {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_F").field("Vk", &self.Vk).field("NLSFEProcType", &self.NLSFEProcType).field("NLSFEProcCurrent", &self.NLSFEProcCurrent).field("NLSFEProcSwitch", &self.NLSFEProcSwitch).field("NLSFEProc", &self.NLSFEProc).field("NLSFEProcAlt", &self.NLSFEProcAlt).finish()
    }
}
impl ::core::default::Default for VK_FPARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VK_FPARAM {
    fn eq(&self, other: &Self) -> bool {
        self.NLSFEProcIndex == other.NLSFEProcIndex && self.NLSFEProcParam == other.NLSFEProcParam
    }
}
impl ::core::cmp::Eq for VK_FPARAM {}
impl ::core::fmt::Debug for VK_FPARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_FPARAM").field("NLSFEProcIndex", &self.NLSFEProcIndex).field("NLSFEProcParam", &self.NLSFEProcParam).finish()
    }
}
impl ::core::default::Default for VK_TO_BIT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VK_TO_BIT {
    fn eq(&self, other: &Self) -> bool {
        self.Vk == other.Vk && self.ModBits == other.ModBits
    }
}
impl ::core::cmp::Eq for VK_TO_BIT {}
impl ::core::fmt::Debug for VK_TO_BIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_TO_BIT").field("Vk", &self.Vk).field("ModBits", &self.ModBits).finish()
    }
}
impl ::core::default::Default for VK_TO_WCHARS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VK_TO_WCHARS1 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey && self.Attributes == other.Attributes && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for VK_TO_WCHARS1 {}
impl ::core::fmt::Debug for VK_TO_WCHARS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_TO_WCHARS1").field("VirtualKey", &self.VirtualKey).field("Attributes", &self.Attributes).field("wch", &self.wch).finish()
    }
}
impl ::core::default::Default for VK_TO_WCHARS10 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VK_TO_WCHARS10 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey && self.Attributes == other.Attributes && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for VK_TO_WCHARS10 {}
impl ::core::fmt::Debug for VK_TO_WCHARS10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_TO_WCHARS10").field("VirtualKey", &self.VirtualKey).field("Attributes", &self.Attributes).field("wch", &self.wch).finish()
    }
}
impl ::core::default::Default for VK_TO_WCHARS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VK_TO_WCHARS2 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey && self.Attributes == other.Attributes && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for VK_TO_WCHARS2 {}
impl ::core::fmt::Debug for VK_TO_WCHARS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_TO_WCHARS2").field("VirtualKey", &self.VirtualKey).field("Attributes", &self.Attributes).field("wch", &self.wch).finish()
    }
}
impl ::core::default::Default for VK_TO_WCHARS3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VK_TO_WCHARS3 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey && self.Attributes == other.Attributes && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for VK_TO_WCHARS3 {}
impl ::core::fmt::Debug for VK_TO_WCHARS3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_TO_WCHARS3").field("VirtualKey", &self.VirtualKey).field("Attributes", &self.Attributes).field("wch", &self.wch).finish()
    }
}
impl ::core::default::Default for VK_TO_WCHARS4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VK_TO_WCHARS4 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey && self.Attributes == other.Attributes && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for VK_TO_WCHARS4 {}
impl ::core::fmt::Debug for VK_TO_WCHARS4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_TO_WCHARS4").field("VirtualKey", &self.VirtualKey).field("Attributes", &self.Attributes).field("wch", &self.wch).finish()
    }
}
impl ::core::default::Default for VK_TO_WCHARS5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VK_TO_WCHARS5 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey && self.Attributes == other.Attributes && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for VK_TO_WCHARS5 {}
impl ::core::fmt::Debug for VK_TO_WCHARS5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_TO_WCHARS5").field("VirtualKey", &self.VirtualKey).field("Attributes", &self.Attributes).field("wch", &self.wch).finish()
    }
}
impl ::core::default::Default for VK_TO_WCHARS6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VK_TO_WCHARS6 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey && self.Attributes == other.Attributes && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for VK_TO_WCHARS6 {}
impl ::core::fmt::Debug for VK_TO_WCHARS6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_TO_WCHARS6").field("VirtualKey", &self.VirtualKey).field("Attributes", &self.Attributes).field("wch", &self.wch).finish()
    }
}
impl ::core::default::Default for VK_TO_WCHARS7 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VK_TO_WCHARS7 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey && self.Attributes == other.Attributes && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for VK_TO_WCHARS7 {}
impl ::core::fmt::Debug for VK_TO_WCHARS7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_TO_WCHARS7").field("VirtualKey", &self.VirtualKey).field("Attributes", &self.Attributes).field("wch", &self.wch).finish()
    }
}
impl ::core::default::Default for VK_TO_WCHARS8 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VK_TO_WCHARS8 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey && self.Attributes == other.Attributes && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for VK_TO_WCHARS8 {}
impl ::core::fmt::Debug for VK_TO_WCHARS8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_TO_WCHARS8").field("VirtualKey", &self.VirtualKey).field("Attributes", &self.Attributes).field("wch", &self.wch).finish()
    }
}
impl ::core::default::Default for VK_TO_WCHARS9 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VK_TO_WCHARS9 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey && self.Attributes == other.Attributes && self.wch == other.wch
    }
}
impl ::core::cmp::Eq for VK_TO_WCHARS9 {}
impl ::core::fmt::Debug for VK_TO_WCHARS9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_TO_WCHARS9").field("VirtualKey", &self.VirtualKey).field("Attributes", &self.Attributes).field("wch", &self.wch).finish()
    }
}
impl ::core::default::Default for VK_TO_WCHAR_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VK_TO_WCHAR_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.pVkToWchars == other.pVkToWchars && self.nModifications == other.nModifications && self.cbSize == other.cbSize
    }
}
impl ::core::cmp::Eq for VK_TO_WCHAR_TABLE {}
impl ::core::fmt::Debug for VK_TO_WCHAR_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_TO_WCHAR_TABLE").field("pVkToWchars", &self.pVkToWchars).field("nModifications", &self.nModifications).field("cbSize", &self.cbSize).finish()
    }
}
impl ::core::default::Default for VK_VSC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VK_VSC {
    fn eq(&self, other: &Self) -> bool {
        self.Vk == other.Vk && self.Vsc == other.Vsc
    }
}
impl ::core::cmp::Eq for VK_VSC {}
impl ::core::fmt::Debug for VK_VSC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VK_VSC").field("Vk", &self.Vk).field("Vsc", &self.Vsc).finish()
    }
}
impl ::core::default::Default for VSC_LPWSTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VSC_LPWSTR {
    fn eq(&self, other: &Self) -> bool {
        self.vsc == other.vsc && self.pwsz == other.pwsz
    }
}
impl ::core::cmp::Eq for VSC_LPWSTR {}
impl ::core::fmt::Debug for VSC_LPWSTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VSC_LPWSTR").field("vsc", &self.vsc).field("pwsz", &self.pwsz).finish()
    }
}
impl ::core::default::Default for VSC_VK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VSC_VK {
    fn eq(&self, other: &Self) -> bool {
        self.Vsc == other.Vsc && self.Vk == other.Vk
    }
}
impl ::core::cmp::Eq for VSC_VK {}
impl ::core::fmt::Debug for VSC_VK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VSC_VK").field("Vsc", &self.Vsc).field("Vk", &self.Vk).finish()
    }
}
