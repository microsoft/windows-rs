impl ::core::default::Default for GameControllerVersionInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GameControllerVersionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.Major == other.Major && self.Minor == other.Minor && self.Build == other.Build && self.Revision == other.Revision
    }
}
impl ::core::cmp::Eq for GameControllerVersionInfo {}
impl ::core::fmt::Debug for GameControllerVersionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GameControllerVersionInfo").field("Major", &self.Major).field("Minor", &self.Minor).field("Build", &self.Build).field("Revision", &self.Revision).finish()
    }
}
impl ::core::default::Default for GipFirmwareUpdateProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GipFirmwareUpdateProgress {
    fn eq(&self, other: &Self) -> bool {
        self.PercentCompleted == other.PercentCompleted && self.CurrentComponentId == other.CurrentComponentId
    }
}
impl ::core::cmp::Eq for GipFirmwareUpdateProgress {}
impl ::core::fmt::Debug for GipFirmwareUpdateProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GipFirmwareUpdateProgress").field("PercentCompleted", &self.PercentCompleted).field("CurrentComponentId", &self.CurrentComponentId).finish()
    }
}
impl ::core::cmp::PartialEq for GipFirmwareUpdateResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GipFirmwareUpdateResult {}
impl ::core::fmt::Debug for GipFirmwareUpdateResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GipFirmwareUpdateResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for GipFirmwareUpdateStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GipFirmwareUpdateStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GipFirmwareUpdateStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GipGameControllerProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GipGameControllerProvider {}
impl ::core::fmt::Debug for GipGameControllerProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GipGameControllerProvider").field(&self.0).finish()
    }
}
impl ::core::default::Default for GipMessageClass {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GipMessageClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GipMessageClass").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for HidGameControllerProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidGameControllerProvider {}
impl ::core::fmt::Debug for HidGameControllerProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidGameControllerProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICustomGameControllerFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICustomGameControllerFactory {}
impl ::core::fmt::Debug for ICustomGameControllerFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICustomGameControllerFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGameControllerInputSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGameControllerInputSink {}
impl ::core::fmt::Debug for IGameControllerInputSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGameControllerInputSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGameControllerProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGameControllerProvider {}
impl ::core::fmt::Debug for IGameControllerProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGameControllerProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGipGameControllerInputSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGipGameControllerInputSink {}
impl ::core::fmt::Debug for IGipGameControllerInputSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGipGameControllerInputSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IHidGameControllerInputSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHidGameControllerInputSink {}
impl ::core::fmt::Debug for IHidGameControllerInputSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHidGameControllerInputSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXusbGameControllerInputSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXusbGameControllerInputSink {}
impl ::core::fmt::Debug for IXusbGameControllerInputSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXusbGameControllerInputSink").field(&self.0).finish()
    }
}
impl ::core::default::Default for XusbDeviceSubtype {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XusbDeviceSubtype {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XusbDeviceSubtype").field(&self.0).finish()
    }
}
impl ::core::default::Default for XusbDeviceType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XusbDeviceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XusbDeviceType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for XusbGameControllerProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for XusbGameControllerProvider {}
impl ::core::fmt::Debug for XusbGameControllerProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XusbGameControllerProvider").field(&self.0).finish()
    }
}
