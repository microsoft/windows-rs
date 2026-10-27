#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BD_CLASS_SUPPORT_BITMAP {
    pub _bitfield1: u8,
    pub _bitfield2: u8,
}
#[cfg(feature = "shared")]
impl BD_CLASS_SUPPORT_BITMAP {
    pub fn Version8(&self) -> super::UCHAR {
        (self._bitfield1) & 1
    }
    pub fn set_Version8(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !1) | (value & 1);
    }
    pub fn Version9(&self) -> super::UCHAR {
        (self._bitfield1 >> 1) & 1
    }
    pub fn set_Version9(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Version10(&self) -> super::UCHAR {
        (self._bitfield1 >> 2) & 1
    }
    pub fn set_Version10(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn Version11(&self) -> super::UCHAR {
        (self._bitfield1 >> 3) & 1
    }
    pub fn set_Version11(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn Version12(&self) -> super::UCHAR {
        (self._bitfield1 >> 4) & 1
    }
    pub fn set_Version12(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn Version13(&self) -> super::UCHAR {
        (self._bitfield1 >> 5) & 1
    }
    pub fn set_Version13(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn Version14(&self) -> super::UCHAR {
        (self._bitfield1 >> 6) & 1
    }
    pub fn set_Version14(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn Version15(&self) -> super::UCHAR {
        self._bitfield1 >> 7
    }
    pub fn set_Version15(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !(1 << 7)) | ((value & 1) << 7);
    }
    pub fn Version0(&self) -> super::UCHAR {
        (self._bitfield2) & 1
    }
    pub fn set_Version0(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !1) | (value & 1);
    }
    pub fn Version1(&self) -> super::UCHAR {
        (self._bitfield2 >> 1) & 1
    }
    pub fn set_Version1(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Version2(&self) -> super::UCHAR {
        (self._bitfield2 >> 2) & 1
    }
    pub fn set_Version2(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn Version3(&self) -> super::UCHAR {
        (self._bitfield2 >> 3) & 1
    }
    pub fn set_Version3(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn Version4(&self) -> super::UCHAR {
        (self._bitfield2 >> 4) & 1
    }
    pub fn set_Version4(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn Version5(&self) -> super::UCHAR {
        (self._bitfield2 >> 5) & 1
    }
    pub fn set_Version5(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn Version6(&self) -> super::UCHAR {
        (self._bitfield2 >> 6) & 1
    }
    pub fn set_Version6(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn Version7(&self) -> super::UCHAR {
        self._bitfield2 >> 7
    }
    pub fn set_Version7(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !(1 << 7)) | ((value & 1) << 7);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_BD_READ {
    pub Header: FEATURE_HEADER,
    pub Reserved: [super::UCHAR; 4],
    pub Class0BitmapBDREReadSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class1BitmapBDREReadSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class2BitmapBDREReadSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class3BitmapBDREReadSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class0BitmapBDRReadSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class1BitmapBDRReadSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class2BitmapBDRReadSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class3BitmapBDRReadSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class0BitmapBDROMReadSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class1BitmapBDROMReadSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class2BitmapBDROMReadSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class3BitmapBDROMReadSupport: BD_CLASS_SUPPORT_BITMAP,
}
#[cfg(feature = "shared")]
impl Default for FEATURE_BD_READ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_BD_R_PSEUDO_OVERWRITE {
    pub Header: FEATURE_HEADER,
    pub Reserved: [super::UCHAR; 4],
}
#[cfg(feature = "shared")]
impl Default for FEATURE_BD_R_PSEUDO_OVERWRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_BD_WRITE {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: [super::UCHAR; 3],
    pub Class0BitmapBDREWriteSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class1BitmapBDREWriteSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class2BitmapBDREWriteSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class3BitmapBDREWriteSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class0BitmapBDRWriteSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class1BitmapBDRWriteSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class2BitmapBDRWriteSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class3BitmapBDRWriteSupport: BD_CLASS_SUPPORT_BITMAP,
}
#[cfg(feature = "shared")]
impl FEATURE_BD_WRITE {
    pub fn SupportsVerifyNotRequired(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_SupportsVerifyNotRequired(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved1(&self) -> super::UCHAR {
        self._bitfield >> 1
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_BD_WRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_CD_RW_MEDIA_WRITE_SUPPORT {
    pub Header: FEATURE_HEADER,
    pub Reserved1: super::UCHAR,
    pub CDRWMediaSubtypeSupport: FEATURE_CD_RW_MEDIA_WRITE_SUPPORT_0,
    pub Reserved2: [super::UCHAR; 2],
}
#[cfg(feature = "shared")]
impl Default for FEATURE_CD_RW_MEDIA_WRITE_SUPPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FEATURE_CD_RW_MEDIA_WRITE_SUPPORT_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl FEATURE_CD_RW_MEDIA_WRITE_SUPPORT_0 {
    pub fn Subtype0(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_Subtype0(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Subtype1(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_Subtype1(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Subtype2(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_Subtype2(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn Subtype3(&self) -> super::UCHAR {
        (self._bitfield >> 3) & 1
    }
    pub fn set_Subtype3(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn Subtype4(&self) -> super::UCHAR {
        (self._bitfield >> 4) & 1
    }
    pub fn set_Subtype4(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn Subtype5(&self) -> super::UCHAR {
        (self._bitfield >> 5) & 1
    }
    pub fn set_Subtype5(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn Subtype6(&self) -> super::UCHAR {
        (self._bitfield >> 6) & 1
    }
    pub fn set_Subtype6(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn Subtype7(&self) -> super::UCHAR {
        self._bitfield >> 7
    }
    pub fn set_Subtype7(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FEATURE_DATA_AACS {
    pub Header: FEATURE_HEADER,
    pub _bitfield1: u8,
    pub BindingNonceBlockCount: super::UCHAR,
    pub _bitfield2: u8,
    pub AACSVersion: super::UCHAR,
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_AACS {
    pub fn BindingNonceGeneration(&self) -> super::UCHAR {
        (self._bitfield1) & 1
    }
    pub fn set_BindingNonceGeneration(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !1) | (value & 1);
    }
    pub fn Reserved0(&self) -> super::UCHAR {
        self._bitfield1 >> 1
    }
    pub fn set_Reserved0(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !(127 << 1)) | ((value & 127) << 1);
    }
    pub fn NumberOfAGIDs(&self) -> super::UCHAR {
        (self._bitfield2) & 15
    }
    pub fn set_NumberOfAGIDs(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !15) | (value & 15);
    }
    pub fn Reserved1(&self) -> super::UCHAR {
        self._bitfield2 >> 4
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_CDRW_CAV_WRITE {
    pub Header: FEATURE_HEADER,
    pub Reserved1: [super::UCHAR; 4],
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_CDRW_CAV_WRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_CD_AUDIO_ANALOG_PLAY {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: super::UCHAR,
    pub NumerOfVolumeLevels: [super::UCHAR; 2],
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_CD_AUDIO_ANALOG_PLAY {
    pub fn SeperateVolume(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_SeperateVolume(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn SeperateChannelMute(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_SeperateChannelMute(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn ScanSupported(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_ScanSupported(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn Reserved1(&self) -> super::UCHAR {
        self._bitfield >> 3
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(31 << 3)) | ((value & 31) << 3);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_CD_AUDIO_ANALOG_PLAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_CD_MASTERING {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub MaximumCueSheetLength: [super::UCHAR; 3],
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_CD_MASTERING {
    pub fn RWSubchannelsRecordable(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_RWSubchannelsRecordable(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn CdRewritable(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_CdRewritable(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn TestWriteOk(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_TestWriteOk(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn RawRecordingOk(&self) -> super::UCHAR {
        (self._bitfield >> 3) & 1
    }
    pub fn set_RawRecordingOk(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn RawMultiSessionOk(&self) -> super::UCHAR {
        (self._bitfield >> 4) & 1
    }
    pub fn set_RawMultiSessionOk(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn SessionAtOnceOk(&self) -> super::UCHAR {
        (self._bitfield >> 5) & 1
    }
    pub fn set_SessionAtOnceOk(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn BufferUnderrunFree(&self) -> super::UCHAR {
        (self._bitfield >> 6) & 1
    }
    pub fn set_BufferUnderrunFree(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn Reserved1(&self) -> super::UCHAR {
        self._bitfield >> 7
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_CD_MASTERING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_CD_READ {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: [super::UCHAR; 3],
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_CD_READ {
    pub fn CDText(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_CDText(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn C2ErrorData(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_C2ErrorData(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Reserved01(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 31
    }
    pub fn set_Reserved01(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(31 << 2)) | ((value & 31) << 2);
    }
    pub fn DigitalAudioPlay(&self) -> super::UCHAR {
        self._bitfield >> 7
    }
    pub fn set_DigitalAudioPlay(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_CD_READ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_CD_TRACK_AT_ONCE {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: super::UCHAR,
    pub DataTypeSupported: [super::UCHAR; 2],
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_CD_TRACK_AT_ONCE {
    pub fn RWSubchannelsRecordable(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_RWSubchannelsRecordable(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn CdRewritable(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_CdRewritable(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn TestWriteOk(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_TestWriteOk(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn RWSubchannelPackedOk(&self) -> super::UCHAR {
        (self._bitfield >> 3) & 1
    }
    pub fn set_RWSubchannelPackedOk(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn RWSubchannelRawOk(&self) -> super::UCHAR {
        (self._bitfield >> 4) & 1
    }
    pub fn set_RWSubchannelRawOk(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn Reserved1(&self) -> super::UCHAR {
        (self._bitfield >> 5) & 1
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn BufferUnderrunFree(&self) -> super::UCHAR {
        (self._bitfield >> 6) & 1
    }
    pub fn set_BufferUnderrunFree(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn Reserved3(&self) -> super::UCHAR {
        self._bitfield >> 7
    }
    pub fn set_Reserved3(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_CD_TRACK_AT_ONCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_CORE {
    pub Header: FEATURE_HEADER,
    pub PhysicalInterface: [super::UCHAR; 4],
    pub _bitfield: u8,
    pub Reserved2: [super::UCHAR; 3],
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_CORE {
    pub fn DeviceBusyEvent(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_DeviceBusyEvent(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn INQUIRY2(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_INQUIRY2(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Reserved1(&self) -> super::UCHAR {
        self._bitfield >> 2
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(63 << 2)) | ((value & 63) << 2);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_CORE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FEATURE_DATA_DDCD_READ {
    pub Header: FEATURE_HEADER,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_DDCD_RW_WRITE {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: [super::UCHAR; 3],
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_DDCD_RW_WRITE {
    pub fn Blank(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_Blank(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Intermediate(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_Intermediate(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Reserved1(&self) -> super::UCHAR {
        self._bitfield >> 2
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(63 << 2)) | ((value & 63) << 2);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_DDCD_RW_WRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_DDCD_R_WRITE {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved3: [super::UCHAR; 3],
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_DDCD_R_WRITE {
    pub fn Reserved1(&self) -> super::UCHAR {
        (self._bitfield) & 3
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !3) | (value & 3);
    }
    pub fn TestWrite(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_TestWrite(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn Reserved2(&self) -> super::UCHAR {
        self._bitfield >> 3
    }
    pub fn set_Reserved2(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(31 << 3)) | ((value & 31) << 3);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_DDCD_R_WRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_DEFECT_MANAGEMENT {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: [super::UCHAR; 3],
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_DEFECT_MANAGEMENT {
    pub fn Reserved1(&self) -> super::UCHAR {
        (self._bitfield) & 127
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !127) | (value & 127);
    }
    pub fn SupplimentalSpareArea(&self) -> super::UCHAR {
        self._bitfield >> 7
    }
    pub fn set_SupplimentalSpareArea(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_DEFECT_MANAGEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_DISC_CONTROL_BLOCKS {
    pub Header: FEATURE_HEADER,
    pub Data: [FEATURE_DATA_DISC_CONTROL_BLOCKS_EX; 0],
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_DISC_CONTROL_BLOCKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_DISC_CONTROL_BLOCKS_EX {
    pub ContentDescriptor: [super::UCHAR; 4],
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_DISC_CONTROL_BLOCKS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_DVD_CPRM {
    pub Header: FEATURE_HEADER,
    pub Reserved0: [super::UCHAR; 3],
    pub CPRMVersion: super::UCHAR,
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_DVD_CPRM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_DVD_CSS {
    pub Header: FEATURE_HEADER,
    pub Reserved1: [super::UCHAR; 3],
    pub CssVersion: super::UCHAR,
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_DVD_CSS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_DVD_PLUS_R {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: [super::UCHAR; 3],
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_DVD_PLUS_R {
    pub fn Write(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_Write(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved1(&self) -> super::UCHAR {
        self._bitfield >> 1
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_DVD_PLUS_R {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_DVD_PLUS_RW {
    pub Header: FEATURE_HEADER,
    pub _bitfield1: u8,
    pub _bitfield2: u8,
    pub Reserved03: [super::UCHAR; 2],
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_DVD_PLUS_RW {
    pub fn Write(&self) -> super::UCHAR {
        (self._bitfield1) & 1
    }
    pub fn set_Write(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !1) | (value & 1);
    }
    pub fn Reserved1(&self) -> super::UCHAR {
        self._bitfield1 >> 1
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !(127 << 1)) | ((value & 127) << 1);
    }
    pub fn CloseOnly(&self) -> super::UCHAR {
        (self._bitfield2) & 1
    }
    pub fn set_CloseOnly(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !1) | (value & 1);
    }
    pub fn QuickStart(&self) -> super::UCHAR {
        (self._bitfield2 >> 1) & 1
    }
    pub fn set_QuickStart(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Reserved02(&self) -> super::UCHAR {
        self._bitfield2 >> 2
    }
    pub fn set_Reserved02(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !(63 << 2)) | ((value & 63) << 2);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_DVD_PLUS_RW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_DVD_PLUS_RW_DUAL_LAYER {
    pub Header: FEATURE_HEADER,
    pub _bitfield1: u8,
    pub _bitfield2: u8,
    pub Reserved3: [super::UCHAR; 2],
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_DVD_PLUS_RW_DUAL_LAYER {
    pub fn Write(&self) -> super::UCHAR {
        (self._bitfield1) & 1
    }
    pub fn set_Write(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !1) | (value & 1);
    }
    pub fn Reserved1(&self) -> super::UCHAR {
        self._bitfield1 >> 1
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !(127 << 1)) | ((value & 127) << 1);
    }
    pub fn CloseOnly(&self) -> super::UCHAR {
        (self._bitfield2) & 1
    }
    pub fn set_CloseOnly(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !1) | (value & 1);
    }
    pub fn QuickStart(&self) -> super::UCHAR {
        (self._bitfield2 >> 1) & 1
    }
    pub fn set_QuickStart(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Reserved2(&self) -> super::UCHAR {
        self._bitfield2 >> 2
    }
    pub fn set_Reserved2(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !(63 << 2)) | ((value & 63) << 2);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_DVD_PLUS_RW_DUAL_LAYER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_DVD_PLUS_R_DUAL_LAYER {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: [super::UCHAR; 3],
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_DVD_PLUS_R_DUAL_LAYER {
    pub fn Write(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_Write(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved1(&self) -> super::UCHAR {
        self._bitfield >> 1
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_DVD_PLUS_R_DUAL_LAYER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FEATURE_DATA_DVD_READ {
    pub Header: FEATURE_HEADER,
    pub _bitfield1: u8,
    pub Reserved2: super::UCHAR,
    pub _bitfield2: u8,
    pub Reserved4: super::UCHAR,
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_DVD_READ {
    pub fn Multi110(&self) -> super::UCHAR {
        (self._bitfield1) & 1
    }
    pub fn set_Multi110(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !1) | (value & 1);
    }
    pub fn Reserved1(&self) -> super::UCHAR {
        self._bitfield1 >> 1
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !(127 << 1)) | ((value & 127) << 1);
    }
    pub fn DualDashR(&self) -> super::UCHAR {
        (self._bitfield2) & 1
    }
    pub fn set_DualDashR(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !1) | (value & 1);
    }
    pub fn Reserved3(&self) -> super::UCHAR {
        self._bitfield2 >> 1
    }
    pub fn set_Reserved3(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_DVD_RECORDABLE_WRITE {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved4: [super::UCHAR; 3],
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_DVD_RECORDABLE_WRITE {
    pub fn Reserved1(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn DVD_RW(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_DVD_RW(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn TestWrite(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_TestWrite(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn RDualLayer(&self) -> super::UCHAR {
        (self._bitfield >> 3) & 1
    }
    pub fn set_RDualLayer(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn Reserved02(&self) -> super::UCHAR {
        (self._bitfield >> 4) & 3
    }
    pub fn set_Reserved02(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(3 << 4)) | ((value & 3) << 4);
    }
    pub fn BufferUnderrunFree(&self) -> super::UCHAR {
        (self._bitfield >> 6) & 1
    }
    pub fn set_BufferUnderrunFree(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn Reserved3(&self) -> super::UCHAR {
        self._bitfield >> 7
    }
    pub fn set_Reserved3(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_DVD_RECORDABLE_WRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_DVD_RW_RESTRICTED_OVERWRITE {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved1: [super::UCHAR; 3],
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_DVD_RW_RESTRICTED_OVERWRITE {
    pub fn Blank(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_Blank(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Intermediate(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_Intermediate(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn DefectStatusDataRead(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_DefectStatusDataRead(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn DefectStatusDataGenerate(&self) -> super::UCHAR {
        (self._bitfield >> 3) & 1
    }
    pub fn set_DefectStatusDataGenerate(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn Reserved0(&self) -> super::UCHAR {
        self._bitfield >> 4
    }
    pub fn set_Reserved0(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_DVD_RW_RESTRICTED_OVERWRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_EMBEDDED_CHANGER {
    pub Header: FEATURE_HEADER,
    pub _bitfield1: u8,
    pub Reserved4: [super::UCHAR; 2],
    pub _bitfield2: u8,
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_EMBEDDED_CHANGER {
    pub fn Reserved1(&self) -> super::UCHAR {
        (self._bitfield1) & 3
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !3) | (value & 3);
    }
    pub fn SupportsDiscPresent(&self) -> super::UCHAR {
        (self._bitfield1 >> 2) & 1
    }
    pub fn set_SupportsDiscPresent(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn Reserved2(&self) -> super::UCHAR {
        (self._bitfield1 >> 3) & 1
    }
    pub fn set_Reserved2(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn SideChangeCapable(&self) -> super::UCHAR {
        (self._bitfield1 >> 4) & 1
    }
    pub fn set_SideChangeCapable(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn Reserved3(&self) -> super::UCHAR {
        self._bitfield1 >> 5
    }
    pub fn set_Reserved3(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !(7 << 5)) | ((value & 7) << 5);
    }
    pub fn HighestSlotNumber(&self) -> super::UCHAR {
        (self._bitfield2) & 31
    }
    pub fn set_HighestSlotNumber(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !31) | (value & 31);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield2 >> 5
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !(7 << 5)) | ((value & 7) << 5);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_EMBEDDED_CHANGER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_FIRMWARE_DATE {
    pub Header: FEATURE_HEADER,
    pub Year: [super::UCHAR; 4],
    pub Month: [super::UCHAR; 2],
    pub Day: [super::UCHAR; 2],
    pub Hour: [super::UCHAR; 2],
    pub Minute: [super::UCHAR; 2],
    pub Seconds: [super::UCHAR; 2],
    pub Reserved: [super::UCHAR; 2],
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_FIRMWARE_DATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_FORMATTABLE {
    pub Header: FEATURE_HEADER,
    pub _bitfield1: u8,
    pub Reserved2: [super::UCHAR; 3],
    pub _bitfield2: u8,
    pub Reserved4: [super::UCHAR; 3],
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_FORMATTABLE {
    pub fn FullCertification(&self) -> super::UCHAR {
        (self._bitfield1) & 1
    }
    pub fn set_FullCertification(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !1) | (value & 1);
    }
    pub fn QuickCertification(&self) -> super::UCHAR {
        (self._bitfield1 >> 1) & 1
    }
    pub fn set_QuickCertification(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn SpareAreaExpansion(&self) -> super::UCHAR {
        (self._bitfield1 >> 2) & 1
    }
    pub fn set_SpareAreaExpansion(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn RENoSpareAllocated(&self) -> super::UCHAR {
        (self._bitfield1 >> 3) & 1
    }
    pub fn set_RENoSpareAllocated(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn Reserved1(&self) -> super::UCHAR {
        self._bitfield1 >> 4
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !(15 << 4)) | ((value & 15) << 4);
    }
    pub fn RRandomWritable(&self) -> super::UCHAR {
        (self._bitfield2) & 1
    }
    pub fn set_RRandomWritable(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !1) | (value & 1);
    }
    pub fn Reserved3(&self) -> super::UCHAR {
        self._bitfield2 >> 1
    }
    pub fn set_Reserved3(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_FORMATTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FEATURE_DATA_HDDVD_READ {
    pub Header: FEATURE_HEADER,
    pub _bitfield1: u8,
    pub Reserved1: super::UCHAR,
    pub _bitfield2: u8,
    pub Reserved3: super::UCHAR,
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_HDDVD_READ {
    pub fn Recordable(&self) -> super::UCHAR {
        (self._bitfield1) & 1
    }
    pub fn set_Recordable(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !1) | (value & 1);
    }
    pub fn Reserved0(&self) -> super::UCHAR {
        self._bitfield1 >> 1
    }
    pub fn set_Reserved0(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !(127 << 1)) | ((value & 127) << 1);
    }
    pub fn Rewritable(&self) -> super::UCHAR {
        (self._bitfield2) & 1
    }
    pub fn set_Rewritable(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !1) | (value & 1);
    }
    pub fn Reserved2(&self) -> super::UCHAR {
        self._bitfield2 >> 1
    }
    pub fn set_Reserved2(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FEATURE_DATA_HDDVD_WRITE {
    pub Header: FEATURE_HEADER,
    pub _bitfield1: u8,
    pub Reserved1: super::UCHAR,
    pub _bitfield2: u8,
    pub Reserved3: super::UCHAR,
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_HDDVD_WRITE {
    pub fn Recordable(&self) -> super::UCHAR {
        (self._bitfield1) & 1
    }
    pub fn set_Recordable(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !1) | (value & 1);
    }
    pub fn Reserved0(&self) -> super::UCHAR {
        self._bitfield1 >> 1
    }
    pub fn set_Reserved0(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !(127 << 1)) | ((value & 127) << 1);
    }
    pub fn Rewritable(&self) -> super::UCHAR {
        (self._bitfield2) & 1
    }
    pub fn set_Rewritable(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !1) | (value & 1);
    }
    pub fn Reserved2(&self) -> super::UCHAR {
        self._bitfield2 >> 1
    }
    pub fn set_Reserved2(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_INCREMENTAL_STREAMING_WRITABLE {
    pub Header: FEATURE_HEADER,
    pub DataTypeSupported: [super::UCHAR; 2],
    pub _bitfield: u8,
    pub NumberOfLinkSizes: super::UCHAR,
    pub LinkSize: [super::UCHAR; 0],
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_INCREMENTAL_STREAMING_WRITABLE {
    pub fn BufferUnderrunFree(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_BufferUnderrunFree(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn AddressModeReservation(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_AddressModeReservation(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn TrackRessourceInformation(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_TrackRessourceInformation(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn Reserved01(&self) -> super::UCHAR {
        self._bitfield >> 3
    }
    pub fn set_Reserved01(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(31 << 3)) | ((value & 31) << 3);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_INCREMENTAL_STREAMING_WRITABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_LAYER_JUMP_RECORDING {
    pub Header: FEATURE_HEADER,
    pub Reserved0: [super::UCHAR; 3],
    pub NumberOfLinkSizes: super::UCHAR,
    pub LinkSizes: [super::UCHAR; 0],
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_LAYER_JUMP_RECORDING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_LOGICAL_UNIT_SERIAL_NUMBER {
    pub Header: FEATURE_HEADER,
    pub SerialNumber: [super::UCHAR; 0],
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_LOGICAL_UNIT_SERIAL_NUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_MICROCODE_UPDATE {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: [super::UCHAR; 3],
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_MICROCODE_UPDATE {
    pub fn M5(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_M5(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved1(&self) -> super::UCHAR {
        self._bitfield >> 1
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_MICROCODE_UPDATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_MORPHING {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: [super::UCHAR; 3],
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_MORPHING {
    pub fn Asynchronous(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_Asynchronous(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn OCEvent(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_OCEvent(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Reserved01(&self) -> super::UCHAR {
        self._bitfield >> 2
    }
    pub fn set_Reserved01(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(63 << 2)) | ((value & 63) << 2);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_MORPHING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_MRW {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: [super::UCHAR; 3],
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_MRW {
    pub fn Write(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_Write(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn DvdPlusRead(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_DvdPlusRead(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn DvdPlusWrite(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_DvdPlusWrite(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn Reserved01(&self) -> super::UCHAR {
        self._bitfield >> 3
    }
    pub fn set_Reserved01(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(31 << 3)) | ((value & 31) << 3);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_MRW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FEATURE_DATA_MULTI_READ {
    pub Header: FEATURE_HEADER,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FEATURE_DATA_POWER_MANAGEMENT {
    pub Header: FEATURE_HEADER,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_PROFILE_LIST {
    pub Header: FEATURE_HEADER,
    pub Profiles: [FEATURE_DATA_PROFILE_LIST_EX; 0],
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_PROFILE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_PROFILE_LIST_EX {
    pub ProfileNumber: [super::UCHAR; 2],
    pub _bitfield: u8,
    pub Reserved2: super::UCHAR,
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_PROFILE_LIST_EX {
    pub fn Current(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_Current(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved1(&self) -> super::UCHAR {
        self._bitfield >> 1
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_PROFILE_LIST_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_RANDOM_READABLE {
    pub Header: FEATURE_HEADER,
    pub LogicalBlockSize: [super::UCHAR; 4],
    pub Blocking: [super::UCHAR; 2],
    pub _bitfield: u8,
    pub Reserved2: super::UCHAR,
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_RANDOM_READABLE {
    pub fn ErrorRecoveryPagePresent(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_ErrorRecoveryPagePresent(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved1(&self) -> super::UCHAR {
        self._bitfield >> 1
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_RANDOM_READABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_RANDOM_WRITABLE {
    pub Header: FEATURE_HEADER,
    pub LastLBA: [super::UCHAR; 4],
    pub LogicalBlockSize: [super::UCHAR; 4],
    pub Blocking: [super::UCHAR; 2],
    pub _bitfield: u8,
    pub Reserved2: super::UCHAR,
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_RANDOM_WRITABLE {
    pub fn ErrorRecoveryPagePresent(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_ErrorRecoveryPagePresent(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved1(&self) -> super::UCHAR {
        self._bitfield >> 1
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_RANDOM_WRITABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_REAL_TIME_STREAMING {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: [super::UCHAR; 3],
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_REAL_TIME_STREAMING {
    pub fn StreamRecording(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_StreamRecording(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn WriteSpeedInGetPerf(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_WriteSpeedInGetPerf(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn WriteSpeedInMP2A(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_WriteSpeedInMP2A(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn SetCDSpeed(&self) -> super::UCHAR {
        (self._bitfield >> 3) & 1
    }
    pub fn set_SetCDSpeed(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn ReadBufferCapacityBlock(&self) -> super::UCHAR {
        (self._bitfield >> 4) & 1
    }
    pub fn set_ReadBufferCapacityBlock(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn Reserved1(&self) -> super::UCHAR {
        self._bitfield >> 5
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(7 << 5)) | ((value & 7) << 5);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_REAL_TIME_STREAMING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_REMOVABLE_MEDIUM {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved3: [super::UCHAR; 3],
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_REMOVABLE_MEDIUM {
    pub fn Lockable(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_Lockable(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn DBML(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_DBML(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn DefaultToPrevent(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_DefaultToPrevent(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn Eject(&self) -> super::UCHAR {
        (self._bitfield >> 3) & 1
    }
    pub fn set_Eject(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn Load(&self) -> super::UCHAR {
        (self._bitfield >> 4) & 1
    }
    pub fn set_Load(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn LoadingMechanism(&self) -> super::UCHAR {
        self._bitfield >> 5
    }
    pub fn set_LoadingMechanism(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(7 << 5)) | ((value & 7) << 5);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_REMOVABLE_MEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_RESERVED {
    pub Header: FEATURE_HEADER,
    pub Data: [super::UCHAR; 0],
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_RESERVED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FEATURE_DATA_RESTRICTED_OVERWRITE {
    pub Header: FEATURE_HEADER,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FEATURE_DATA_SECTOR_ERASABLE {
    pub Header: FEATURE_HEADER,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_SMART {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved02: [super::UCHAR; 3],
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_SMART {
    pub fn FaultFailureReportingPagePresent(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_FaultFailureReportingPagePresent(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved1(&self) -> super::UCHAR {
        self._bitfield >> 1
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_SMART {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_TIMEOUT {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: super::UCHAR,
    pub UnitLength: [super::UCHAR; 2],
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_TIMEOUT {
    pub fn Group3(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_Group3(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved1(&self) -> super::UCHAR {
        self._bitfield >> 1
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_TIMEOUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_VENDOR_SPECIFIC {
    pub Header: FEATURE_HEADER,
    pub VendorSpecificData: [super::UCHAR; 0],
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_VENDOR_SPECIFIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_WRITE_ONCE {
    pub Header: FEATURE_HEADER,
    pub LogicalBlockSize: [super::UCHAR; 4],
    pub Blocking: [super::UCHAR; 2],
    pub _bitfield: u8,
    pub Reserved2: super::UCHAR,
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_WRITE_ONCE {
    pub fn ErrorRecoveryPagePresent(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_ErrorRecoveryPagePresent(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved1(&self) -> super::UCHAR {
        self._bitfield >> 1
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_WRITE_ONCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_DATA_WRITE_PROTECT {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: [super::UCHAR; 3],
}
#[cfg(feature = "shared")]
impl FEATURE_DATA_WRITE_PROTECT {
    pub fn SupportsSWPPBit(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_SupportsSWPPBit(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn SupportsPersistentWriteProtect(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_SupportsPersistentWriteProtect(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn WriteInhibitDCB(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_WriteInhibitDCB(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn DiscWriteProtectPAC(&self) -> super::UCHAR {
        (self._bitfield >> 3) & 1
    }
    pub fn set_DiscWriteProtectPAC(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn Reserved01(&self) -> super::UCHAR {
        self._bitfield >> 4
    }
    pub fn set_Reserved01(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_DATA_WRITE_PROTECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_ENHANCED_DEFECT_REPORTING {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub NumberOfDBICacheZones: super::UCHAR,
    pub NumberOfEntries: [super::UCHAR; 2],
}
#[cfg(feature = "shared")]
impl FEATURE_ENHANCED_DEFECT_REPORTING {
    pub fn DRTDMSupported(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_DRTDMSupported(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved0(&self) -> super::UCHAR {
        self._bitfield >> 1
    }
    pub fn set_Reserved0(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_ENHANCED_DEFECT_REPORTING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_HEADER {
    pub FeatureCode: [super::UCHAR; 2],
    pub _bitfield: u8,
    pub AdditionalLength: super::UCHAR,
}
#[cfg(feature = "shared")]
impl FEATURE_HEADER {
    pub fn Current(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_Current(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Persistent(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_Persistent(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Version(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 15
    }
    pub fn set_Version(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(15 << 2)) | ((value & 15) << 2);
    }
    pub fn Reserved0(&self) -> super::UCHAR {
        self._bitfield >> 6
    }
    pub fn set_Reserved0(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(3 << 6)) | ((value & 3) << 6);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_HYBRID_DISC {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: [super::UCHAR; 3],
}
#[cfg(feature = "shared")]
impl FEATURE_HYBRID_DISC {
    pub fn ResetImmunity(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_ResetImmunity(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved1(&self) -> super::UCHAR {
        self._bitfield >> 1
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[cfg(feature = "shared")]
impl Default for FEATURE_HYBRID_DISC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FEATURE_MEDIA_SERIAL_NUMBER {
    pub Header: FEATURE_HEADER,
}
pub type FEATURE_NUMBER = i32;
pub type FEATURE_PROFILE_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FEATURE_TSR {
    pub Header: FEATURE_HEADER,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FEATURE_VCPS {
    pub Header: FEATURE_HEADER,
    pub Reserved: [super::UCHAR; 4],
}
#[cfg(feature = "shared")]
impl Default for FEATURE_VCPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FeatureAACS: FEATURE_NUMBER = 269;
pub const FeatureBDRPseudoOverwrite: FEATURE_NUMBER = 56;
pub const FeatureBDRead: FEATURE_NUMBER = 64;
pub const FeatureBDWrite: FEATURE_NUMBER = 65;
pub const FeatureCDAudioAnalogPlay: FEATURE_NUMBER = 259;
pub const FeatureCDRWMediaWriteSupport: FEATURE_NUMBER = 55;
pub const FeatureCdMastering: FEATURE_NUMBER = 46;
pub const FeatureCdRead: FEATURE_NUMBER = 30;
pub const FeatureCdTrackAtOnce: FEATURE_NUMBER = 45;
pub const FeatureCdrwCAVWrite: FEATURE_NUMBER = 39;
pub const FeatureCore: FEATURE_NUMBER = 1;
pub const FeatureDDCDRWWrite: FEATURE_NUMBER = 50;
pub const FeatureDDCDRWrite: FEATURE_NUMBER = 49;
pub const FeatureDDCDRead: FEATURE_NUMBER = 48;
pub const FeatureDefectManagement: FEATURE_NUMBER = 36;
pub const FeatureDiscControlBlocks: FEATURE_NUMBER = 266;
pub const FeatureDvdCPRM: FEATURE_NUMBER = 267;
pub const FeatureDvdCSS: FEATURE_NUMBER = 262;
pub const FeatureDvdPlusR: FEATURE_NUMBER = 43;
pub const FeatureDvdPlusRDualLayer: FEATURE_NUMBER = 59;
pub const FeatureDvdPlusRW: FEATURE_NUMBER = 42;
pub const FeatureDvdPlusRWDualLayer: FEATURE_NUMBER = 58;
pub const FeatureDvdRead: FEATURE_NUMBER = 31;
pub const FeatureDvdRecordableWrite: FEATURE_NUMBER = 47;
pub const FeatureEmbeddedChanger: FEATURE_NUMBER = 258;
pub const FeatureEnhancedDefectReporting: FEATURE_NUMBER = 41;
pub const FeatureFirmwareDate: FEATURE_NUMBER = 268;
pub const FeatureFormattable: FEATURE_NUMBER = 35;
pub const FeatureHDDVDRead: FEATURE_NUMBER = 80;
pub const FeatureHDDVDWrite: FEATURE_NUMBER = 81;
pub const FeatureHybridDisc: FEATURE_NUMBER = 128;
pub const FeatureIncrementalStreamingWritable: FEATURE_NUMBER = 33;
pub const FeatureLayerJumpRecording: FEATURE_NUMBER = 51;
pub const FeatureLogicalUnitSerialNumber: FEATURE_NUMBER = 264;
pub const FeatureMediaSerialNumber: FEATURE_NUMBER = 265;
pub const FeatureMicrocodeUpgrade: FEATURE_NUMBER = 260;
pub const FeatureMorphing: FEATURE_NUMBER = 2;
pub const FeatureMrw: FEATURE_NUMBER = 40;
pub const FeatureMultiRead: FEATURE_NUMBER = 29;
pub const FeaturePowerManagement: FEATURE_NUMBER = 256;
pub const FeatureProfileList: FEATURE_NUMBER = 0;
pub const FeatureRandomReadable: FEATURE_NUMBER = 16;
pub const FeatureRandomWritable: FEATURE_NUMBER = 32;
pub const FeatureRealTimeStreaming: FEATURE_NUMBER = 263;
pub const FeatureRemovableMedium: FEATURE_NUMBER = 3;
pub const FeatureRestrictedOverwrite: FEATURE_NUMBER = 38;
pub const FeatureRigidRestrictedOverwrite: FEATURE_NUMBER = 44;
pub const FeatureSMART: FEATURE_NUMBER = 257;
pub const FeatureSectorErasable: FEATURE_NUMBER = 34;
pub const FeatureTSR: FEATURE_NUMBER = 66;
pub const FeatureTimeout: FEATURE_NUMBER = 261;
pub const FeatureVCPS: FEATURE_NUMBER = 272;
pub const FeatureWriteOnce: FEATURE_NUMBER = 37;
pub const FeatureWriteProtect: FEATURE_NUMBER = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GET_CONFIGURATION_HEADER {
    pub DataLength: [super::UCHAR; 4],
    pub Reserved: [super::UCHAR; 2],
    pub CurrentProfile: [super::UCHAR; 2],
    pub Data: [super::UCHAR; 0],
}
#[cfg(feature = "shared")]
impl Default for GET_CONFIGURATION_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GET_CONFIGURATION_IOCTL_INPUT {
    pub Feature: FEATURE_NUMBER,
    pub RequestType: super::ULONG,
    pub Reserved: [super::PVOID; 2],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for GET_CONFIGURATION_IOCTL_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type PBD_CLASS_SUPPORT_BITMAP = *mut BD_CLASS_SUPPORT_BITMAP;
#[cfg(feature = "shared")]
pub type PFEATURE_BD_READ = *mut FEATURE_BD_READ;
#[cfg(feature = "shared")]
pub type PFEATURE_BD_R_PSEUDO_OVERWRITE = *mut FEATURE_BD_R_PSEUDO_OVERWRITE;
#[cfg(feature = "shared")]
pub type PFEATURE_BD_WRITE = *mut FEATURE_BD_WRITE;
#[cfg(feature = "shared")]
pub type PFEATURE_CD_RW_MEDIA_WRITE_SUPPORT = *mut FEATURE_CD_RW_MEDIA_WRITE_SUPPORT;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_AACS = *mut FEATURE_DATA_AACS;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_CDRW_CAV_WRITE = *mut FEATURE_DATA_CDRW_CAV_WRITE;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_CD_AUDIO_ANALOG_PLAY = *mut FEATURE_DATA_CD_AUDIO_ANALOG_PLAY;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_CD_MASTERING = *mut FEATURE_DATA_CD_MASTERING;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_CD_READ = *mut FEATURE_DATA_CD_READ;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_CD_TRACK_AT_ONCE = *mut FEATURE_DATA_CD_TRACK_AT_ONCE;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_CORE = *mut FEATURE_DATA_CORE;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_DDCD_READ = *mut FEATURE_DATA_DDCD_READ;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_DDCD_RW_WRITE = *mut FEATURE_DATA_DDCD_RW_WRITE;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_DDCD_R_WRITE = *mut FEATURE_DATA_DDCD_R_WRITE;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_DEFECT_MANAGEMENT = *mut FEATURE_DATA_DEFECT_MANAGEMENT;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_DISC_CONTROL_BLOCKS = *mut FEATURE_DATA_DISC_CONTROL_BLOCKS;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_DISC_CONTROL_BLOCKS_EX = *mut FEATURE_DATA_DISC_CONTROL_BLOCKS_EX;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_DVD_CPRM = *mut FEATURE_DATA_DVD_CPRM;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_DVD_CSS = *mut FEATURE_DATA_DVD_CSS;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_DVD_PLUS_R = *mut FEATURE_DATA_DVD_PLUS_R;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_DVD_PLUS_RW = *mut FEATURE_DATA_DVD_PLUS_RW;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_DVD_PLUS_RW_DUAL_LAYER = *mut FEATURE_DATA_DVD_PLUS_RW_DUAL_LAYER;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_DVD_PLUS_R_DUAL_LAYER = *mut FEATURE_DATA_DVD_PLUS_R_DUAL_LAYER;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_DVD_READ = *mut FEATURE_DATA_DVD_READ;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_DVD_RECORDABLE_WRITE = *mut FEATURE_DATA_DVD_RECORDABLE_WRITE;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_DVD_RW_RESTRICTED_OVERWRITE = *mut FEATURE_DATA_DVD_RW_RESTRICTED_OVERWRITE;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_EMBEDDED_CHANGER = *mut FEATURE_DATA_EMBEDDED_CHANGER;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_FIRMWARE_DATE = *mut FEATURE_DATA_FIRMWARE_DATE;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_FORMATTABLE = *mut FEATURE_DATA_FORMATTABLE;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_HDDVD_READ = *mut FEATURE_DATA_HDDVD_READ;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_HDDVD_WRITE = *mut FEATURE_DATA_HDDVD_WRITE;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_INCREMENTAL_STREAMING_WRITABLE = *mut FEATURE_DATA_INCREMENTAL_STREAMING_WRITABLE;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_LAYER_JUMP_RECORDING = *mut FEATURE_DATA_LAYER_JUMP_RECORDING;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_LOGICAL_UNIT_SERIAL_NUMBER = *mut FEATURE_DATA_LOGICAL_UNIT_SERIAL_NUMBER;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_MICROCODE_UPDATE = *mut FEATURE_DATA_MICROCODE_UPDATE;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_MORPHING = *mut FEATURE_DATA_MORPHING;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_MRW = *mut FEATURE_DATA_MRW;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_MULTI_READ = *mut FEATURE_DATA_MULTI_READ;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_POWER_MANAGEMENT = *mut FEATURE_DATA_POWER_MANAGEMENT;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_PROFILE_LIST = *mut FEATURE_DATA_PROFILE_LIST;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_PROFILE_LIST_EX = *mut FEATURE_DATA_PROFILE_LIST_EX;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_RANDOM_READABLE = *mut FEATURE_DATA_RANDOM_READABLE;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_RANDOM_WRITABLE = *mut FEATURE_DATA_RANDOM_WRITABLE;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_REAL_TIME_STREAMING = *mut FEATURE_DATA_REAL_TIME_STREAMING;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_REMOVABLE_MEDIUM = *mut FEATURE_DATA_REMOVABLE_MEDIUM;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_RESERVED = *mut FEATURE_DATA_RESERVED;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_RESTRICTED_OVERWRITE = *mut FEATURE_DATA_RESTRICTED_OVERWRITE;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_SECTOR_ERASABLE = *mut FEATURE_DATA_SECTOR_ERASABLE;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_SMART = *mut FEATURE_DATA_SMART;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_TIMEOUT = *mut FEATURE_DATA_TIMEOUT;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_VENDOR_SPECIFIC = *mut FEATURE_DATA_VENDOR_SPECIFIC;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_WRITE_ONCE = *mut FEATURE_DATA_WRITE_ONCE;
#[cfg(feature = "shared")]
pub type PFEATURE_DATA_WRITE_PROTECT = *mut FEATURE_DATA_WRITE_PROTECT;
#[cfg(feature = "shared")]
pub type PFEATURE_ENHANCED_DEFECT_REPORTING = *mut FEATURE_ENHANCED_DEFECT_REPORTING;
#[cfg(feature = "shared")]
pub type PFEATURE_HEADER = *mut FEATURE_HEADER;
#[cfg(feature = "shared")]
pub type PFEATURE_HYBRID_DISC = *mut FEATURE_HYBRID_DISC;
#[cfg(feature = "shared")]
pub type PFEATURE_MEDIA_SERIAL_NUMBER = *mut FEATURE_MEDIA_SERIAL_NUMBER;
pub type PFEATURE_NUMBER = *mut FEATURE_NUMBER;
pub type PFEATURE_PROFILE_TYPE = *mut FEATURE_PROFILE_TYPE;
#[cfg(feature = "shared")]
pub type PFEATURE_TSR = *mut FEATURE_TSR;
#[cfg(feature = "shared")]
pub type PFEATURE_VCPS = *mut FEATURE_VCPS;
#[cfg(feature = "shared")]
pub type PGET_CONFIGURATION_HEADER = *mut GET_CONFIGURATION_HEADER;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PGET_CONFIGURATION_IOCTL_INPUT = *mut GET_CONFIGURATION_IOCTL_INPUT;
pub const ProfileAS_MO: FEATURE_PROFILE_TYPE = 5;
pub const ProfileBDRRandomWritable: FEATURE_PROFILE_TYPE = 66;
pub const ProfileBDRSequentialWritable: FEATURE_PROFILE_TYPE = 65;
pub const ProfileBDRewritable: FEATURE_PROFILE_TYPE = 67;
pub const ProfileBDRom: FEATURE_PROFILE_TYPE = 64;
pub const ProfileCdRecordable: FEATURE_PROFILE_TYPE = 9;
pub const ProfileCdRewritable: FEATURE_PROFILE_TYPE = 10;
pub const ProfileCdrom: FEATURE_PROFILE_TYPE = 8;
pub const ProfileDDCdRecordable: FEATURE_PROFILE_TYPE = 33;
pub const ProfileDDCdRewritable: FEATURE_PROFILE_TYPE = 34;
pub const ProfileDDCdrom: FEATURE_PROFILE_TYPE = 32;
pub const ProfileDvdDashRDualLayer: FEATURE_PROFILE_TYPE = 21;
pub const ProfileDvdDashRLayerJump: FEATURE_PROFILE_TYPE = 22;
pub const ProfileDvdPlusR: FEATURE_PROFILE_TYPE = 27;
pub const ProfileDvdPlusRDualLayer: FEATURE_PROFILE_TYPE = 43;
pub const ProfileDvdPlusRW: FEATURE_PROFILE_TYPE = 26;
pub const ProfileDvdPlusRWDualLayer: FEATURE_PROFILE_TYPE = 42;
pub const ProfileDvdRWSequential: FEATURE_PROFILE_TYPE = 20;
pub const ProfileDvdRam: FEATURE_PROFILE_TYPE = 18;
pub const ProfileDvdRecordable: FEATURE_PROFILE_TYPE = 17;
pub const ProfileDvdRewritable: FEATURE_PROFILE_TYPE = 19;
pub const ProfileDvdRom: FEATURE_PROFILE_TYPE = 16;
pub const ProfileHDDVDRDualLayer: FEATURE_PROFILE_TYPE = 88;
pub const ProfileHDDVDRWDualLayer: FEATURE_PROFILE_TYPE = 90;
pub const ProfileHDDVDRam: FEATURE_PROFILE_TYPE = 82;
pub const ProfileHDDVDRecordable: FEATURE_PROFILE_TYPE = 81;
pub const ProfileHDDVDRewritable: FEATURE_PROFILE_TYPE = 83;
pub const ProfileHDDVDRom: FEATURE_PROFILE_TYPE = 80;
pub const ProfileInvalid: FEATURE_PROFILE_TYPE = 0;
pub const ProfileMOErasable: FEATURE_PROFILE_TYPE = 3;
pub const ProfileMOWriteOnce: FEATURE_PROFILE_TYPE = 4;
pub const ProfileNonRemovableDisk: FEATURE_PROFILE_TYPE = 1;
pub const ProfileNonStandard: FEATURE_PROFILE_TYPE = 65535;
pub const ProfileRemovableDisk: FEATURE_PROFILE_TYPE = 2;
pub const SCSI_GET_CONFIGURATION_REQUEST_TYPE_ALL: i32 = 0;
pub const SCSI_GET_CONFIGURATION_REQUEST_TYPE_CURRENT: i32 = 1;
pub const SCSI_GET_CONFIGURATION_REQUEST_TYPE_ONE: i32 = 2;
