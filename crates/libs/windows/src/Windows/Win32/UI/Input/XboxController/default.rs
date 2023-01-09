impl ::core::default::Default for BATTERY_DEVTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BATTERY_DEVTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BATTERY_DEVTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for BATTERY_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BATTERY_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BATTERY_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for BATTERY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BATTERY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BATTERY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for XINPUT_BATTERY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for XINPUT_BATTERY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.BatteryType == other.BatteryType && self.BatteryLevel == other.BatteryLevel
    }
}
impl ::core::cmp::Eq for XINPUT_BATTERY_INFORMATION {}
impl ::core::fmt::Debug for XINPUT_BATTERY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XINPUT_BATTERY_INFORMATION").field("BatteryType", &self.BatteryType).field("BatteryLevel", &self.BatteryLevel).finish()
    }
}
impl ::core::default::Default for XINPUT_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for XINPUT_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.SubType == other.SubType && self.Flags == other.Flags && self.Gamepad == other.Gamepad && self.Vibration == other.Vibration
    }
}
impl ::core::cmp::Eq for XINPUT_CAPABILITIES {}
impl ::core::fmt::Debug for XINPUT_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XINPUT_CAPABILITIES").field("Type", &self.Type).field("SubType", &self.SubType).field("Flags", &self.Flags).field("Gamepad", &self.Gamepad).field("Vibration", &self.Vibration).finish()
    }
}
impl ::core::default::Default for XINPUT_CAPABILITIES_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XINPUT_CAPABILITIES_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XINPUT_CAPABILITIES_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for XINPUT_CAPABILITIES_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for XINPUT_CAPABILITIES_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for XINPUT_CAPABILITIES_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for XINPUT_CAPABILITIES_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for XINPUT_CAPABILITIES_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for XINPUT_DEVSUBTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XINPUT_DEVSUBTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XINPUT_DEVSUBTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for XINPUT_DEVTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XINPUT_DEVTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XINPUT_DEVTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for XINPUT_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XINPUT_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XINPUT_FLAG").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for XINPUT_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for XINPUT_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for XINPUT_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for XINPUT_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for XINPUT_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for XINPUT_GAMEPAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for XINPUT_GAMEPAD {
    fn eq(&self, other: &Self) -> bool {
        self.wButtons == other.wButtons && self.bLeftTrigger == other.bLeftTrigger && self.bRightTrigger == other.bRightTrigger && self.sThumbLX == other.sThumbLX && self.sThumbLY == other.sThumbLY && self.sThumbRX == other.sThumbRX && self.sThumbRY == other.sThumbRY
    }
}
impl ::core::cmp::Eq for XINPUT_GAMEPAD {}
impl ::core::fmt::Debug for XINPUT_GAMEPAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XINPUT_GAMEPAD").field("wButtons", &self.wButtons).field("bLeftTrigger", &self.bLeftTrigger).field("bRightTrigger", &self.bRightTrigger).field("sThumbLX", &self.sThumbLX).field("sThumbLY", &self.sThumbLY).field("sThumbRX", &self.sThumbRX).field("sThumbRY", &self.sThumbRY).finish()
    }
}
impl ::core::default::Default for XINPUT_GAMEPAD_BUTTON_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XINPUT_GAMEPAD_BUTTON_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XINPUT_GAMEPAD_BUTTON_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for XINPUT_GAMEPAD_BUTTON_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for XINPUT_GAMEPAD_BUTTON_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for XINPUT_GAMEPAD_BUTTON_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for XINPUT_GAMEPAD_BUTTON_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for XINPUT_GAMEPAD_BUTTON_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for XINPUT_KEYSTROKE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for XINPUT_KEYSTROKE {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey && self.Unicode == other.Unicode && self.Flags == other.Flags && self.UserIndex == other.UserIndex && self.HidCode == other.HidCode
    }
}
impl ::core::cmp::Eq for XINPUT_KEYSTROKE {}
impl ::core::fmt::Debug for XINPUT_KEYSTROKE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XINPUT_KEYSTROKE").field("VirtualKey", &self.VirtualKey).field("Unicode", &self.Unicode).field("Flags", &self.Flags).field("UserIndex", &self.UserIndex).field("HidCode", &self.HidCode).finish()
    }
}
impl ::core::default::Default for XINPUT_KEYSTROKE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XINPUT_KEYSTROKE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XINPUT_KEYSTROKE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for XINPUT_KEYSTROKE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for XINPUT_KEYSTROKE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for XINPUT_KEYSTROKE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for XINPUT_KEYSTROKE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for XINPUT_KEYSTROKE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for XINPUT_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for XINPUT_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.dwPacketNumber == other.dwPacketNumber && self.Gamepad == other.Gamepad
    }
}
impl ::core::cmp::Eq for XINPUT_STATE {}
impl ::core::fmt::Debug for XINPUT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XINPUT_STATE").field("dwPacketNumber", &self.dwPacketNumber).field("Gamepad", &self.Gamepad).finish()
    }
}
impl ::core::default::Default for XINPUT_VIBRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for XINPUT_VIBRATION {
    fn eq(&self, other: &Self) -> bool {
        self.wLeftMotorSpeed == other.wLeftMotorSpeed && self.wRightMotorSpeed == other.wRightMotorSpeed
    }
}
impl ::core::cmp::Eq for XINPUT_VIBRATION {}
impl ::core::fmt::Debug for XINPUT_VIBRATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XINPUT_VIBRATION").field("wLeftMotorSpeed", &self.wLeftMotorSpeed).field("wRightMotorSpeed", &self.wRightMotorSpeed).finish()
    }
}
impl ::core::default::Default for XINPUT_VIRTUAL_KEY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XINPUT_VIRTUAL_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XINPUT_VIRTUAL_KEY").field(&self.0).finish()
    }
}
