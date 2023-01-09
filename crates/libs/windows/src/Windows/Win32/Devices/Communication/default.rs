impl ::core::default::Default for CLEAR_COMM_ERROR_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLEAR_COMM_ERROR_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLEAR_COMM_ERROR_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CLEAR_COMM_ERROR_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CLEAR_COMM_ERROR_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CLEAR_COMM_ERROR_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CLEAR_COMM_ERROR_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CLEAR_COMM_ERROR_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COMMCONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COMMCONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.wVersion == other.wVersion && self.wReserved == other.wReserved && self.dcb == other.dcb && self.dwProviderSubType == other.dwProviderSubType && self.dwProviderOffset == other.dwProviderOffset && self.dwProviderSize == other.dwProviderSize && self.wcProviderData == other.wcProviderData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COMMCONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COMMCONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMMCONFIG").field("dwSize", &self.dwSize).field("wVersion", &self.wVersion).field("wReserved", &self.wReserved).field("dcb", &self.dcb).field("dwProviderSubType", &self.dwProviderSubType).field("dwProviderOffset", &self.dwProviderOffset).field("dwProviderSize", &self.dwProviderSize).field("wcProviderData", &self.wcProviderData).finish()
    }
}
impl ::core::default::Default for COMMPROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for COMMPROP {
    fn eq(&self, other: &Self) -> bool {
        self.wPacketLength == other.wPacketLength
            && self.wPacketVersion == other.wPacketVersion
            && self.dwServiceMask == other.dwServiceMask
            && self.dwReserved1 == other.dwReserved1
            && self.dwMaxTxQueue == other.dwMaxTxQueue
            && self.dwMaxRxQueue == other.dwMaxRxQueue
            && self.dwMaxBaud == other.dwMaxBaud
            && self.dwProvSubType == other.dwProvSubType
            && self.dwProvCapabilities == other.dwProvCapabilities
            && self.dwSettableParams == other.dwSettableParams
            && self.dwSettableBaud == other.dwSettableBaud
            && self.wSettableData == other.wSettableData
            && self.wSettableStopParity == other.wSettableStopParity
            && self.dwCurrentTxQueue == other.dwCurrentTxQueue
            && self.dwCurrentRxQueue == other.dwCurrentRxQueue
            && self.dwProvSpec1 == other.dwProvSpec1
            && self.dwProvSpec2 == other.dwProvSpec2
            && self.wcProvChar == other.wcProvChar
    }
}
impl ::core::cmp::Eq for COMMPROP {}
impl ::core::fmt::Debug for COMMPROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMMPROP")
            .field("wPacketLength", &self.wPacketLength)
            .field("wPacketVersion", &self.wPacketVersion)
            .field("dwServiceMask", &self.dwServiceMask)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwMaxTxQueue", &self.dwMaxTxQueue)
            .field("dwMaxRxQueue", &self.dwMaxRxQueue)
            .field("dwMaxBaud", &self.dwMaxBaud)
            .field("dwProvSubType", &self.dwProvSubType)
            .field("dwProvCapabilities", &self.dwProvCapabilities)
            .field("dwSettableParams", &self.dwSettableParams)
            .field("dwSettableBaud", &self.dwSettableBaud)
            .field("wSettableData", &self.wSettableData)
            .field("wSettableStopParity", &self.wSettableStopParity)
            .field("dwCurrentTxQueue", &self.dwCurrentTxQueue)
            .field("dwCurrentRxQueue", &self.dwCurrentRxQueue)
            .field("dwProvSpec1", &self.dwProvSpec1)
            .field("dwProvSpec2", &self.dwProvSpec2)
            .field("wcProvChar", &self.wcProvChar)
            .finish()
    }
}
impl ::core::default::Default for COMMPROP_STOP_PARITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COMMPROP_STOP_PARITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMMPROP_STOP_PARITY").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for COMMPROP_STOP_PARITY {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for COMMPROP_STOP_PARITY {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for COMMPROP_STOP_PARITY {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for COMMPROP_STOP_PARITY {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for COMMPROP_STOP_PARITY {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for COMMTIMEOUTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for COMMTIMEOUTS {
    fn eq(&self, other: &Self) -> bool {
        self.ReadIntervalTimeout == other.ReadIntervalTimeout && self.ReadTotalTimeoutMultiplier == other.ReadTotalTimeoutMultiplier && self.ReadTotalTimeoutConstant == other.ReadTotalTimeoutConstant && self.WriteTotalTimeoutMultiplier == other.WriteTotalTimeoutMultiplier && self.WriteTotalTimeoutConstant == other.WriteTotalTimeoutConstant
    }
}
impl ::core::cmp::Eq for COMMTIMEOUTS {}
impl ::core::fmt::Debug for COMMTIMEOUTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMMTIMEOUTS").field("ReadIntervalTimeout", &self.ReadIntervalTimeout).field("ReadTotalTimeoutMultiplier", &self.ReadTotalTimeoutMultiplier).field("ReadTotalTimeoutConstant", &self.ReadTotalTimeoutConstant).field("WriteTotalTimeoutMultiplier", &self.WriteTotalTimeoutMultiplier).field("WriteTotalTimeoutConstant", &self.WriteTotalTimeoutConstant).finish()
    }
}
impl ::core::default::Default for COMM_EVENT_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COMM_EVENT_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMM_EVENT_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for COMM_EVENT_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for COMM_EVENT_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for COMM_EVENT_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for COMM_EVENT_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for COMM_EVENT_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for COMSTAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for COMSTAT {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield && self.cbInQue == other.cbInQue && self.cbOutQue == other.cbOutQue
    }
}
impl ::core::cmp::Eq for COMSTAT {}
impl ::core::fmt::Debug for COMSTAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMSTAT").field("_bitfield", &self._bitfield).field("cbInQue", &self.cbInQue).field("cbOutQue", &self.cbOutQue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DCB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DCB {
    fn eq(&self, other: &Self) -> bool {
        self.DCBlength == other.DCBlength && self.BaudRate == other.BaudRate && self._bitfield == other._bitfield && self.wReserved == other.wReserved && self.XonLim == other.XonLim && self.XoffLim == other.XoffLim && self.ByteSize == other.ByteSize && self.Parity == other.Parity && self.StopBits == other.StopBits && self.XonChar == other.XonChar && self.XoffChar == other.XoffChar && self.ErrorChar == other.ErrorChar && self.EofChar == other.EofChar && self.EvtChar == other.EvtChar && self.wReserved1 == other.wReserved1
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DCB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DCB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCB")
            .field("DCBlength", &self.DCBlength)
            .field("BaudRate", &self.BaudRate)
            .field("_bitfield", &self._bitfield)
            .field("wReserved", &self.wReserved)
            .field("XonLim", &self.XonLim)
            .field("XoffLim", &self.XoffLim)
            .field("ByteSize", &self.ByteSize)
            .field("Parity", &self.Parity)
            .field("StopBits", &self.StopBits)
            .field("XonChar", &self.XonChar)
            .field("XoffChar", &self.XoffChar)
            .field("ErrorChar", &self.ErrorChar)
            .field("EofChar", &self.EofChar)
            .field("EvtChar", &self.EvtChar)
            .field("wReserved1", &self.wReserved1)
            .finish()
    }
}
impl ::core::default::Default for DCB_PARITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DCB_PARITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCB_PARITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for DCB_STOP_BITS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DCB_STOP_BITS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCB_STOP_BITS").field(&self.0).finish()
    }
}
impl ::core::default::Default for ESCAPE_COMM_FUNCTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ESCAPE_COMM_FUNCTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESCAPE_COMM_FUNCTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for MODEMDEVCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MODEMDEVCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwActualSize == other.dwActualSize
            && self.dwRequiredSize == other.dwRequiredSize
            && self.dwDevSpecificOffset == other.dwDevSpecificOffset
            && self.dwDevSpecificSize == other.dwDevSpecificSize
            && self.dwModemProviderVersion == other.dwModemProviderVersion
            && self.dwModemManufacturerOffset == other.dwModemManufacturerOffset
            && self.dwModemManufacturerSize == other.dwModemManufacturerSize
            && self.dwModemModelOffset == other.dwModemModelOffset
            && self.dwModemModelSize == other.dwModemModelSize
            && self.dwModemVersionOffset == other.dwModemVersionOffset
            && self.dwModemVersionSize == other.dwModemVersionSize
            && self.dwDialOptions == other.dwDialOptions
            && self.dwCallSetupFailTimer == other.dwCallSetupFailTimer
            && self.dwInactivityTimeout == other.dwInactivityTimeout
            && self.dwSpeakerVolume == other.dwSpeakerVolume
            && self.dwSpeakerMode == other.dwSpeakerMode
            && self.dwModemOptions == other.dwModemOptions
            && self.dwMaxDTERate == other.dwMaxDTERate
            && self.dwMaxDCERate == other.dwMaxDCERate
            && self.abVariablePortion == other.abVariablePortion
    }
}
impl ::core::cmp::Eq for MODEMDEVCAPS {}
impl ::core::fmt::Debug for MODEMDEVCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MODEMDEVCAPS")
            .field("dwActualSize", &self.dwActualSize)
            .field("dwRequiredSize", &self.dwRequiredSize)
            .field("dwDevSpecificOffset", &self.dwDevSpecificOffset)
            .field("dwDevSpecificSize", &self.dwDevSpecificSize)
            .field("dwModemProviderVersion", &self.dwModemProviderVersion)
            .field("dwModemManufacturerOffset", &self.dwModemManufacturerOffset)
            .field("dwModemManufacturerSize", &self.dwModemManufacturerSize)
            .field("dwModemModelOffset", &self.dwModemModelOffset)
            .field("dwModemModelSize", &self.dwModemModelSize)
            .field("dwModemVersionOffset", &self.dwModemVersionOffset)
            .field("dwModemVersionSize", &self.dwModemVersionSize)
            .field("dwDialOptions", &self.dwDialOptions)
            .field("dwCallSetupFailTimer", &self.dwCallSetupFailTimer)
            .field("dwInactivityTimeout", &self.dwInactivityTimeout)
            .field("dwSpeakerVolume", &self.dwSpeakerVolume)
            .field("dwSpeakerMode", &self.dwSpeakerMode)
            .field("dwModemOptions", &self.dwModemOptions)
            .field("dwMaxDTERate", &self.dwMaxDTERate)
            .field("dwMaxDCERate", &self.dwMaxDCERate)
            .field("abVariablePortion", &self.abVariablePortion)
            .finish()
    }
}
impl ::core::default::Default for MODEMDEVCAPS_DIAL_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MODEMDEVCAPS_DIAL_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MODEMDEVCAPS_DIAL_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MODEMDEVCAPS_DIAL_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MODEMDEVCAPS_DIAL_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MODEMDEVCAPS_DIAL_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MODEMDEVCAPS_DIAL_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MODEMDEVCAPS_DIAL_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for MODEMDEVCAPS_SPEAKER_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MODEMDEVCAPS_SPEAKER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MODEMDEVCAPS_SPEAKER_MODE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MODEMDEVCAPS_SPEAKER_MODE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MODEMDEVCAPS_SPEAKER_MODE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MODEMDEVCAPS_SPEAKER_MODE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MODEMDEVCAPS_SPEAKER_MODE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MODEMDEVCAPS_SPEAKER_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for MODEMDEVCAPS_SPEAKER_VOLUME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MODEMDEVCAPS_SPEAKER_VOLUME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MODEMDEVCAPS_SPEAKER_VOLUME").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MODEMDEVCAPS_SPEAKER_VOLUME {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MODEMDEVCAPS_SPEAKER_VOLUME {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MODEMDEVCAPS_SPEAKER_VOLUME {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MODEMDEVCAPS_SPEAKER_VOLUME {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MODEMDEVCAPS_SPEAKER_VOLUME {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for MODEMSETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MODEMSETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.dwActualSize == other.dwActualSize && self.dwRequiredSize == other.dwRequiredSize && self.dwDevSpecificOffset == other.dwDevSpecificOffset && self.dwDevSpecificSize == other.dwDevSpecificSize && self.dwCallSetupFailTimer == other.dwCallSetupFailTimer && self.dwInactivityTimeout == other.dwInactivityTimeout && self.dwSpeakerVolume == other.dwSpeakerVolume && self.dwSpeakerMode == other.dwSpeakerMode && self.dwPreferredModemOptions == other.dwPreferredModemOptions && self.dwNegotiatedModemOptions == other.dwNegotiatedModemOptions && self.dwNegotiatedDCERate == other.dwNegotiatedDCERate && self.abVariablePortion == other.abVariablePortion
    }
}
impl ::core::cmp::Eq for MODEMSETTINGS {}
impl ::core::fmt::Debug for MODEMSETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MODEMSETTINGS")
            .field("dwActualSize", &self.dwActualSize)
            .field("dwRequiredSize", &self.dwRequiredSize)
            .field("dwDevSpecificOffset", &self.dwDevSpecificOffset)
            .field("dwDevSpecificSize", &self.dwDevSpecificSize)
            .field("dwCallSetupFailTimer", &self.dwCallSetupFailTimer)
            .field("dwInactivityTimeout", &self.dwInactivityTimeout)
            .field("dwSpeakerVolume", &self.dwSpeakerVolume)
            .field("dwSpeakerMode", &self.dwSpeakerMode)
            .field("dwPreferredModemOptions", &self.dwPreferredModemOptions)
            .field("dwNegotiatedModemOptions", &self.dwNegotiatedModemOptions)
            .field("dwNegotiatedDCERate", &self.dwNegotiatedDCERate)
            .field("abVariablePortion", &self.abVariablePortion)
            .finish()
    }
}
impl ::core::default::Default for MODEMSETTINGS_SPEAKER_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MODEMSETTINGS_SPEAKER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MODEMSETTINGS_SPEAKER_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MODEM_SPEAKER_VOLUME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MODEM_SPEAKER_VOLUME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MODEM_SPEAKER_VOLUME").field(&self.0).finish()
    }
}
impl ::core::default::Default for MODEM_STATUS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MODEM_STATUS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MODEM_STATUS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MODEM_STATUS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MODEM_STATUS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MODEM_STATUS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MODEM_STATUS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MODEM_STATUS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for PURGE_COMM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PURGE_COMM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PURGE_COMM_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PURGE_COMM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PURGE_COMM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PURGE_COMM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PURGE_COMM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PURGE_COMM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
