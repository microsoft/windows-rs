pub const DIALOPTION_BILLING: i32 = 64;
pub const DIALOPTION_DIALTONE: i32 = 256;
pub const DIALOPTION_QUIET: i32 = 128;
pub type LPMODEMDEVCAPS = *mut MODEMDEVCAPS;
pub type LPMODEMSETTINGS = *mut MODEMSETTINGS;
pub const MDMSPKRFLAG_CALLSETUP: i32 = 8;
pub const MDMSPKRFLAG_DIAL: i32 = 2;
pub const MDMSPKRFLAG_OFF: i32 = 1;
pub const MDMSPKRFLAG_ON: i32 = 4;
pub const MDMSPKR_CALLSETUP: i32 = 3;
pub const MDMSPKR_DIAL: i32 = 1;
pub const MDMSPKR_OFF: i32 = 0;
pub const MDMSPKR_ON: i32 = 2;
pub const MDMVOLFLAG_HIGH: i32 = 4;
pub const MDMVOLFLAG_LOW: i32 = 1;
pub const MDMVOLFLAG_MEDIUM: i32 = 2;
pub const MDMVOL_HIGH: i32 = 2;
pub const MDMVOL_LOW: i32 = 0;
pub const MDMVOL_MEDIUM: i32 = 1;
pub const MDM_ANALOG_RLP_OFF: i32 = 1;
pub const MDM_ANALOG_RLP_ON: i32 = 0;
pub const MDM_ANALOG_V34: i32 = 2;
pub const MDM_AUTO_ML_2: i32 = 2;
pub const MDM_AUTO_ML_DEFAULT: i32 = 0;
pub const MDM_AUTO_ML_NONE: i32 = 1;
pub const MDM_AUTO_SPEED_DEFAULT: i32 = 0;
pub const MDM_BEARERMODE_ANALOG: i32 = 0;
pub const MDM_BEARERMODE_GSM: i32 = 2;
pub const MDM_BEARERMODE_ISDN: i32 = 1;
pub const MDM_BLIND_DIAL: i32 = 512;
pub const MDM_CCITT_OVERRIDE: i32 = 64;
pub const MDM_CELLULAR: i32 = 8;
pub const MDM_COMPRESSION: i32 = 1;
pub const MDM_DIAGNOSTICS: i32 = 2048;
pub const MDM_ERROR_CONTROL: i32 = 2;
pub const MDM_FLOWCONTROL_HARD: i32 = 16;
pub const MDM_FLOWCONTROL_SOFT: i32 = 32;
pub const MDM_FORCED_EC: i32 = 4;
pub const MDM_HDLCPPP_AUTH_CHAP: i32 = 3;
pub const MDM_HDLCPPP_AUTH_DEFAULT: i32 = 0;
pub const MDM_HDLCPPP_AUTH_MSCHAP: i32 = 4;
pub const MDM_HDLCPPP_AUTH_NONE: i32 = 1;
pub const MDM_HDLCPPP_AUTH_PAP: i32 = 2;
pub const MDM_HDLCPPP_ML_2: i32 = 2;
pub const MDM_HDLCPPP_ML_DEFAULT: i32 = 0;
pub const MDM_HDLCPPP_ML_NONE: i32 = 1;
pub const MDM_HDLCPPP_SPEED_56K: i32 = 2;
pub const MDM_HDLCPPP_SPEED_64K: i32 = 1;
pub const MDM_HDLCPPP_SPEED_DEFAULT: i32 = 0;
pub const MDM_MASK_AUTO_ML: i32 = 192;
pub const MDM_MASK_AUTO_SPEED: i32 = 7;
pub const MDM_MASK_BEARERMODE: i32 = 61440;
pub const MDM_MASK_EXTENDEDINFO: i32 = 268431360;
pub const MDM_MASK_HDLCPPP_AUTH: i32 = 56;
pub const MDM_MASK_HDLCPPP_ML: i32 = 192;
pub const MDM_MASK_HDLCPPP_SPEED: i32 = 7;
pub const MDM_MASK_PROTOCOLDATA: i32 = 267386880;
pub const MDM_MASK_PROTOCOLID: i32 = 983040;
pub const MDM_MASK_PROTOCOLINFO: i32 = 268369920;
pub const MDM_MASK_V110_SPEED: i32 = 15;
pub const MDM_MASK_V120_ML: i32 = 192;
pub const MDM_MASK_V120_SPEED: i32 = 7;
pub const MDM_MASK_X75_DATA: i32 = 7;
pub const MDM_PIAFS_INCOMING: i32 = 0;
pub const MDM_PIAFS_OUTGOING: i32 = 1;
pub const MDM_PROTOCOLID_ANALOG: i32 = 7;
pub const MDM_PROTOCOLID_AUTO: i32 = 6;
pub const MDM_PROTOCOLID_DEFAULT: i32 = 0;
pub const MDM_PROTOCOLID_GPRS: i32 = 8;
pub const MDM_PROTOCOLID_HDLCPPP: i32 = 1;
pub const MDM_PROTOCOLID_PIAFS: i32 = 9;
pub const MDM_PROTOCOLID_V110: i32 = 4;
pub const MDM_PROTOCOLID_V120: i32 = 5;
pub const MDM_PROTOCOLID_V128: i32 = 2;
pub const MDM_PROTOCOLID_X75: i32 = 3;
pub const MDM_PROTOCOL_ANALOG_NRLP: i32 = 1507328;
pub const MDM_PROTOCOL_ANALOG_RLP: i32 = 458752;
pub const MDM_PROTOCOL_ANALOG_V34: i32 = 2555904;
pub const MDM_PROTOCOL_AUTO_1CH: i32 = 67502080;
pub const MDM_PROTOCOL_AUTO_2CH: i32 = 134610944;
pub const MDM_PROTOCOL_GPRS: i32 = 524288;
pub const MDM_PROTOCOL_HDLCPPP_112K: i32 = 136380416;
pub const MDM_PROTOCOL_HDLCPPP_112K_CHAP: i32 = 161546240;
pub const MDM_PROTOCOL_HDLCPPP_112K_MSCHAP: i32 = 169934848;
pub const MDM_PROTOCOL_HDLCPPP_112K_PAP: i32 = 153157632;
pub const MDM_PROTOCOL_HDLCPPP_128K: i32 = 135331840;
pub const MDM_PROTOCOL_HDLCPPP_128K_CHAP: i32 = 160497664;
pub const MDM_PROTOCOL_HDLCPPP_128K_MSCHAP: i32 = 168886272;
pub const MDM_PROTOCOL_HDLCPPP_128K_PAP: i32 = 152109056;
pub const MDM_PROTOCOL_HDLCPPP_56K: i32 = 2162688;
pub const MDM_PROTOCOL_HDLCPPP_64K: i32 = 1114112;
pub const MDM_PROTOCOL_PIAFS_INCOMING: i32 = 589824;
pub const MDM_PROTOCOL_PIAFS_OUTGOING: i32 = 1638400;
pub const MDM_PROTOCOL_V110_12DOT0K: i32 = 5505024;
pub const MDM_PROTOCOL_V110_14DOT4K: i32 = 6553600;
pub const MDM_PROTOCOL_V110_19DOT2K: i32 = 7602176;
pub const MDM_PROTOCOL_V110_1DOT2K: i32 = 1310720;
pub const MDM_PROTOCOL_V110_28DOT8K: i32 = 8650752;
pub const MDM_PROTOCOL_V110_2DOT4K: i32 = 2359296;
pub const MDM_PROTOCOL_V110_38DOT4K: i32 = 9699328;
pub const MDM_PROTOCOL_V110_4DOT8K: i32 = 3407872;
pub const MDM_PROTOCOL_V110_57DOT6K: i32 = 10747904;
pub const MDM_PROTOCOL_V110_9DOT6K: i32 = 4456448;
pub const MDM_PROTOCOL_V120_112K: i32 = 136642560;
pub const MDM_PROTOCOL_V120_128K: i32 = 135593984;
pub const MDM_PROTOCOL_V120_56K: i32 = 69533696;
pub const MDM_PROTOCOL_V120_64K: i32 = 68485120;
pub const MDM_PROTOCOL_X75_128K: i32 = 2293760;
pub const MDM_PROTOCOL_X75_64K: i32 = 1245184;
pub const MDM_PROTOCOL_X75_BTX: i32 = 4390912;
pub const MDM_PROTOCOL_X75_T_70: i32 = 3342336;
pub const MDM_SHIFT_AUTO_ML: i32 = 6;
pub const MDM_SHIFT_AUTO_SPEED: i32 = 0;
pub const MDM_SHIFT_BEARERMODE: i32 = 12;
pub const MDM_SHIFT_EXTENDEDINFO: i32 = 12;
pub const MDM_SHIFT_HDLCPPP_AUTH: i32 = 3;
pub const MDM_SHIFT_HDLCPPP_ML: i32 = 6;
pub const MDM_SHIFT_HDLCPPP_SPEED: i32 = 0;
pub const MDM_SHIFT_PROTOCOLDATA: i32 = 20;
pub const MDM_SHIFT_PROTOCOLID: i32 = 16;
pub const MDM_SHIFT_PROTOCOLINFO: i32 = 16;
pub const MDM_SHIFT_V110_SPEED: i32 = 0;
pub const MDM_SHIFT_V120_ML: i32 = 6;
pub const MDM_SHIFT_V120_SPEED: i32 = 0;
pub const MDM_SHIFT_X75_DATA: i32 = 0;
pub const MDM_SPEED_ADJUST: i32 = 128;
pub const MDM_TONE_DIAL: i32 = 256;
pub const MDM_V110_SPEED_12DOT0K: i32 = 5;
pub const MDM_V110_SPEED_14DOT4K: i32 = 6;
pub const MDM_V110_SPEED_19DOT2K: i32 = 7;
pub const MDM_V110_SPEED_1DOT2K: i32 = 1;
pub const MDM_V110_SPEED_28DOT8K: i32 = 8;
pub const MDM_V110_SPEED_2DOT4K: i32 = 2;
pub const MDM_V110_SPEED_38DOT4K: i32 = 9;
pub const MDM_V110_SPEED_4DOT8K: i32 = 3;
pub const MDM_V110_SPEED_57DOT6K: i32 = 10;
pub const MDM_V110_SPEED_9DOT6K: i32 = 4;
pub const MDM_V110_SPEED_DEFAULT: i32 = 0;
pub const MDM_V120_ML_2: i32 = 2;
pub const MDM_V120_ML_DEFAULT: i32 = 0;
pub const MDM_V120_ML_NONE: i32 = 1;
pub const MDM_V120_SPEED_56K: i32 = 2;
pub const MDM_V120_SPEED_64K: i32 = 1;
pub const MDM_V120_SPEED_DEFAULT: i32 = 0;
pub const MDM_V23_OVERRIDE: i32 = 1024;
pub const MDM_X75_DATA_128K: i32 = 2;
pub const MDM_X75_DATA_64K: i32 = 1;
pub const MDM_X75_DATA_BTX: i32 = 4;
pub const MDM_X75_DATA_DEFAULT: i32 = 0;
pub const MDM_X75_DATA_T_70: i32 = 3;
#[repr(C)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
