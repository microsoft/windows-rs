pub const DIALOPTION_BILLING: u32 = 64;
pub const DIALOPTION_DIALTONE: u32 = 256;
pub const DIALOPTION_QUIET: u32 = 128;
pub type LPMODEMDEVCAPS = *mut MODEMDEVCAPS;
pub type LPMODEMSETTINGS = *mut MODEMSETTINGS;
pub const MDMSPKRFLAG_CALLSETUP: u32 = 8;
pub const MDMSPKRFLAG_DIAL: u32 = 2;
pub const MDMSPKRFLAG_OFF: u32 = 1;
pub const MDMSPKRFLAG_ON: u32 = 4;
pub const MDMSPKR_CALLSETUP: u32 = 3;
pub const MDMSPKR_DIAL: u32 = 1;
pub const MDMSPKR_OFF: u32 = 0;
pub const MDMSPKR_ON: u32 = 2;
pub const MDMVOLFLAG_HIGH: u32 = 4;
pub const MDMVOLFLAG_LOW: u32 = 1;
pub const MDMVOLFLAG_MEDIUM: u32 = 2;
pub const MDMVOL_HIGH: u32 = 2;
pub const MDMVOL_LOW: u32 = 0;
pub const MDMVOL_MEDIUM: u32 = 1;
pub const MDM_ANALOG_RLP_OFF: u32 = 1;
pub const MDM_ANALOG_RLP_ON: u32 = 0;
pub const MDM_ANALOG_V34: u32 = 2;
pub const MDM_AUTO_ML_2: u32 = 2;
pub const MDM_AUTO_ML_DEFAULT: u32 = 0;
pub const MDM_AUTO_ML_NONE: u32 = 1;
pub const MDM_AUTO_SPEED_DEFAULT: u32 = 0;
pub const MDM_BEARERMODE_ANALOG: u32 = 0;
pub const MDM_BEARERMODE_GSM: u32 = 2;
pub const MDM_BEARERMODE_ISDN: u32 = 1;
pub const MDM_BLIND_DIAL: u32 = 512;
pub const MDM_CCITT_OVERRIDE: u32 = 64;
pub const MDM_CELLULAR: u32 = 8;
pub const MDM_COMPRESSION: u32 = 1;
pub const MDM_DIAGNOSTICS: u32 = 2048;
pub const MDM_ERROR_CONTROL: u32 = 2;
pub const MDM_FLOWCONTROL_HARD: u32 = 16;
pub const MDM_FLOWCONTROL_SOFT: u32 = 32;
pub const MDM_FORCED_EC: u32 = 4;
pub const MDM_HDLCPPP_AUTH_CHAP: u32 = 3;
pub const MDM_HDLCPPP_AUTH_DEFAULT: u32 = 0;
pub const MDM_HDLCPPP_AUTH_MSCHAP: u32 = 4;
pub const MDM_HDLCPPP_AUTH_NONE: u32 = 1;
pub const MDM_HDLCPPP_AUTH_PAP: u32 = 2;
pub const MDM_HDLCPPP_ML_2: u32 = 2;
pub const MDM_HDLCPPP_ML_DEFAULT: u32 = 0;
pub const MDM_HDLCPPP_ML_NONE: u32 = 1;
pub const MDM_HDLCPPP_SPEED_56K: u32 = 2;
pub const MDM_HDLCPPP_SPEED_64K: u32 = 1;
pub const MDM_HDLCPPP_SPEED_DEFAULT: u32 = 0;
pub const MDM_MASK_AUTO_ML: u32 = 192;
pub const MDM_MASK_AUTO_SPEED: u32 = 7;
pub const MDM_MASK_BEARERMODE: u32 = 61440;
pub const MDM_MASK_EXTENDEDINFO: u32 = 268431360;
pub const MDM_MASK_HDLCPPP_AUTH: u32 = 56;
pub const MDM_MASK_HDLCPPP_ML: u32 = 192;
pub const MDM_MASK_HDLCPPP_SPEED: u32 = 7;
pub const MDM_MASK_PROTOCOLDATA: u32 = 267386880;
pub const MDM_MASK_PROTOCOLID: u32 = 983040;
pub const MDM_MASK_PROTOCOLINFO: u32 = 268369920;
pub const MDM_MASK_V110_SPEED: u32 = 15;
pub const MDM_MASK_V120_ML: u32 = 192;
pub const MDM_MASK_V120_SPEED: u32 = 7;
pub const MDM_MASK_X75_DATA: u32 = 7;
pub const MDM_PIAFS_INCOMING: u32 = 0;
pub const MDM_PIAFS_OUTGOING: u32 = 1;
pub const MDM_PROTOCOLID_ANALOG: u32 = 7;
pub const MDM_PROTOCOLID_AUTO: u32 = 6;
pub const MDM_PROTOCOLID_DEFAULT: u32 = 0;
pub const MDM_PROTOCOLID_GPRS: u32 = 8;
pub const MDM_PROTOCOLID_HDLCPPP: u32 = 1;
pub const MDM_PROTOCOLID_PIAFS: u32 = 9;
pub const MDM_PROTOCOLID_V110: u32 = 4;
pub const MDM_PROTOCOLID_V120: u32 = 5;
pub const MDM_PROTOCOLID_V128: u32 = 2;
pub const MDM_PROTOCOLID_X75: u32 = 3;
pub const MDM_PROTOCOL_ANALOG_NRLP: u32 = 1507328;
pub const MDM_PROTOCOL_ANALOG_RLP: u32 = 458752;
pub const MDM_PROTOCOL_ANALOG_V34: u32 = 2555904;
pub const MDM_PROTOCOL_AUTO_1CH: u32 = 67502080;
pub const MDM_PROTOCOL_AUTO_2CH: u32 = 134610944;
pub const MDM_PROTOCOL_GPRS: u32 = 524288;
pub const MDM_PROTOCOL_HDLCPPP_112K: u32 = 136380416;
pub const MDM_PROTOCOL_HDLCPPP_112K_CHAP: u32 = 161546240;
pub const MDM_PROTOCOL_HDLCPPP_112K_MSCHAP: u32 = 169934848;
pub const MDM_PROTOCOL_HDLCPPP_112K_PAP: u32 = 153157632;
pub const MDM_PROTOCOL_HDLCPPP_128K: u32 = 135331840;
pub const MDM_PROTOCOL_HDLCPPP_128K_CHAP: u32 = 160497664;
pub const MDM_PROTOCOL_HDLCPPP_128K_MSCHAP: u32 = 168886272;
pub const MDM_PROTOCOL_HDLCPPP_128K_PAP: u32 = 152109056;
pub const MDM_PROTOCOL_HDLCPPP_56K: u32 = 2162688;
pub const MDM_PROTOCOL_HDLCPPP_64K: u32 = 1114112;
pub const MDM_PROTOCOL_PIAFS_INCOMING: u32 = 589824;
pub const MDM_PROTOCOL_PIAFS_OUTGOING: u32 = 1638400;
pub const MDM_PROTOCOL_V110_12DOT0K: u32 = 5505024;
pub const MDM_PROTOCOL_V110_14DOT4K: u32 = 6553600;
pub const MDM_PROTOCOL_V110_19DOT2K: u32 = 7602176;
pub const MDM_PROTOCOL_V110_1DOT2K: u32 = 1310720;
pub const MDM_PROTOCOL_V110_28DOT8K: u32 = 8650752;
pub const MDM_PROTOCOL_V110_2DOT4K: u32 = 2359296;
pub const MDM_PROTOCOL_V110_38DOT4K: u32 = 9699328;
pub const MDM_PROTOCOL_V110_4DOT8K: u32 = 3407872;
pub const MDM_PROTOCOL_V110_57DOT6K: u32 = 10747904;
pub const MDM_PROTOCOL_V110_9DOT6K: u32 = 4456448;
pub const MDM_PROTOCOL_V120_112K: u32 = 136642560;
pub const MDM_PROTOCOL_V120_128K: u32 = 135593984;
pub const MDM_PROTOCOL_V120_56K: u32 = 69533696;
pub const MDM_PROTOCOL_V120_64K: u32 = 68485120;
pub const MDM_PROTOCOL_X75_128K: u32 = 2293760;
pub const MDM_PROTOCOL_X75_64K: u32 = 1245184;
pub const MDM_PROTOCOL_X75_BTX: u32 = 4390912;
pub const MDM_PROTOCOL_X75_T_70: u32 = 3342336;
pub const MDM_SHIFT_AUTO_ML: u32 = 6;
pub const MDM_SHIFT_AUTO_SPEED: u32 = 0;
pub const MDM_SHIFT_BEARERMODE: u32 = 12;
pub const MDM_SHIFT_EXTENDEDINFO: u32 = 12;
pub const MDM_SHIFT_HDLCPPP_AUTH: u32 = 3;
pub const MDM_SHIFT_HDLCPPP_ML: u32 = 6;
pub const MDM_SHIFT_HDLCPPP_SPEED: u32 = 0;
pub const MDM_SHIFT_PROTOCOLDATA: u32 = 20;
pub const MDM_SHIFT_PROTOCOLID: u32 = 16;
pub const MDM_SHIFT_PROTOCOLINFO: u32 = 16;
pub const MDM_SHIFT_V110_SPEED: u32 = 0;
pub const MDM_SHIFT_V120_ML: u32 = 6;
pub const MDM_SHIFT_V120_SPEED: u32 = 0;
pub const MDM_SHIFT_X75_DATA: u32 = 0;
pub const MDM_SPEED_ADJUST: u32 = 128;
pub const MDM_TONE_DIAL: u32 = 256;
pub const MDM_V110_SPEED_12DOT0K: u32 = 5;
pub const MDM_V110_SPEED_14DOT4K: u32 = 6;
pub const MDM_V110_SPEED_19DOT2K: u32 = 7;
pub const MDM_V110_SPEED_1DOT2K: u32 = 1;
pub const MDM_V110_SPEED_28DOT8K: u32 = 8;
pub const MDM_V110_SPEED_2DOT4K: u32 = 2;
pub const MDM_V110_SPEED_38DOT4K: u32 = 9;
pub const MDM_V110_SPEED_4DOT8K: u32 = 3;
pub const MDM_V110_SPEED_57DOT6K: u32 = 10;
pub const MDM_V110_SPEED_9DOT6K: u32 = 4;
pub const MDM_V110_SPEED_DEFAULT: u32 = 0;
pub const MDM_V120_ML_2: u32 = 2;
pub const MDM_V120_ML_DEFAULT: u32 = 0;
pub const MDM_V120_ML_NONE: u32 = 1;
pub const MDM_V120_SPEED_56K: u32 = 2;
pub const MDM_V120_SPEED_64K: u32 = 1;
pub const MDM_V120_SPEED_DEFAULT: u32 = 0;
pub const MDM_V23_OVERRIDE: u32 = 1024;
pub const MDM_X75_DATA_128K: u32 = 2;
pub const MDM_X75_DATA_64K: u32 = 1;
pub const MDM_X75_DATA_BTX: u32 = 4;
pub const MDM_X75_DATA_DEFAULT: u32 = 0;
pub const MDM_X75_DATA_T_70: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MODEMDEVCAPS {
    pub dwActualSize: u32,
    pub dwRequiredSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwModemProviderVersion: u32,
    pub dwModemManufacturerOffset: u32,
    pub dwModemManufacturerSize: u32,
    pub dwModemModelOffset: u32,
    pub dwModemModelSize: u32,
    pub dwModemVersionOffset: u32,
    pub dwModemVersionSize: u32,
    pub dwDialOptions: u32,
    pub dwCallSetupFailTimer: u32,
    pub dwInactivityTimeout: u32,
    pub dwSpeakerVolume: u32,
    pub dwSpeakerMode: u32,
    pub dwModemOptions: u32,
    pub dwMaxDTERate: u32,
    pub dwMaxDCERate: u32,
    pub abVariablePortion: [u8; 1],
}
impl Default for MODEMDEVCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MODEMSETTINGS {
    pub dwActualSize: u32,
    pub dwRequiredSize: u32,
    pub dwDevSpecificOffset: u32,
    pub dwDevSpecificSize: u32,
    pub dwCallSetupFailTimer: u32,
    pub dwInactivityTimeout: u32,
    pub dwSpeakerVolume: u32,
    pub dwSpeakerMode: u32,
    pub dwPreferredModemOptions: u32,
    pub dwNegotiatedModemOptions: u32,
    pub dwNegotiatedDCERate: u32,
    pub abVariablePortion: [u8; 1],
}
impl Default for MODEMSETTINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PMODEMDEVCAPS = *mut MODEMDEVCAPS;
pub type PMODEMSETTINGS = *mut MODEMSETTINGS;
