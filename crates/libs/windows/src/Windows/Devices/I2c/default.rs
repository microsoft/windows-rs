impl ::core::default::Default for I2cBusSpeed {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for I2cBusSpeed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("I2cBusSpeed").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for I2cConnectionSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for I2cConnectionSettings {}
impl ::core::fmt::Debug for I2cConnectionSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("I2cConnectionSettings").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for I2cController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for I2cController {}
impl ::core::fmt::Debug for I2cController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("I2cController").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for I2cDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for I2cDevice {}
impl ::core::fmt::Debug for I2cDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("I2cDevice").field(&self.0).finish()
    }
}
impl ::core::default::Default for I2cSharingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for I2cSharingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("I2cSharingMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for I2cTransferResult {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for I2cTransferResult {
    fn eq(&self, other: &Self) -> bool {
        self.Status == other.Status && self.BytesTransferred == other.BytesTransferred
    }
}
impl ::core::cmp::Eq for I2cTransferResult {}
impl ::core::fmt::Debug for I2cTransferResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("I2cTransferResult").field("Status", &self.Status).field("BytesTransferred", &self.BytesTransferred).finish()
    }
}
impl ::core::default::Default for I2cTransferStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for I2cTransferStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("I2cTransferStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for II2cDeviceStatics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for II2cDeviceStatics {}
impl ::core::fmt::Debug for II2cDeviceStatics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("II2cDeviceStatics").field(&self.0).finish()
    }
}
