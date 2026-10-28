#[repr(C, packed(8))]
#[cfg(all(feature = "gdipluspixelformats", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct BitmapData {
    pub Width: super::UINT,
    pub Height: super::UINT,
    pub Stride: super::INT,
    pub PixelFormat: super::PixelFormat,
    pub Scan0: *mut core::ffi::c_void,
    pub Reserved: super::UINT_PTR,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EncoderParameter {
    pub Guid: windows_sys::core::GUID,
    pub NumberOfValues: super::ULONG,
    pub Type: super::ULONG,
    pub Value: *mut core::ffi::c_void,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct EncoderParameters {
    pub Count: super::UINT,
    pub Parameter: [EncoderParameter; 1],
}
#[cfg(feature = "shared")]
impl Default for EncoderParameters {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct ImageCodecInfo {
    pub Clsid: super::CLSID,
    pub FormatID: windows_sys::core::GUID,
    pub CodecName: *const super::WCHAR,
    pub DllName: *const super::WCHAR,
    pub FormatDescription: *const super::WCHAR,
    pub FilenameExtension: *const super::WCHAR,
    pub MimeType: *const super::WCHAR,
    pub Flags: super::DWORD,
    pub Version: super::DWORD,
    pub SigCount: super::DWORD,
    pub SigSize: super::DWORD,
    pub SigPattern: *const super::BYTE,
    pub SigMask: *const super::BYTE,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ImageItemData {
    pub Size: super::UINT,
    pub Position: super::UINT,
    pub Desc: *mut core::ffi::c_void,
    pub DescSize: super::UINT,
    pub Data: *mut core::ffi::c_void,
    pub DataSize: super::UINT,
    pub Cookie: super::UINT,
}
#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "wtypes"))]
#[derive(Clone, Copy, Default)]
pub struct PropertyItem {
    pub id: super::PROPID,
    pub length: super::ULONG,
    pub r#type: super::WORD,
    pub value: *mut core::ffi::c_void,
}
pub const PropertyTagArtist: i32 = 315;
pub const PropertyTagBitsPerSample: i32 = 258;
pub const PropertyTagCellHeight: i32 = 265;
pub const PropertyTagCellWidth: i32 = 264;
pub const PropertyTagChrominanceTable: i32 = 20625;
pub const PropertyTagColorMap: i32 = 320;
pub const PropertyTagColorTransferFunction: i32 = 20506;
pub const PropertyTagCompression: i32 = 259;
pub const PropertyTagCopyright: i32 = 33432;
pub const PropertyTagDateTime: i32 = 306;
pub const PropertyTagDocumentName: i32 = 269;
pub const PropertyTagDotRange: i32 = 336;
pub const PropertyTagEquipMake: i32 = 271;
pub const PropertyTagEquipModel: i32 = 272;
pub const PropertyTagExifAperture: i32 = 37378;
pub const PropertyTagExifBrightness: i32 = 37379;
pub const PropertyTagExifCfaPattern: i32 = 41730;
pub const PropertyTagExifColorSpace: i32 = 40961;
pub const PropertyTagExifCompBPP: i32 = 37122;
pub const PropertyTagExifCompConfig: i32 = 37121;
pub const PropertyTagExifContrast: i32 = 41992;
pub const PropertyTagExifCustomRendered: i32 = 41985;
pub const PropertyTagExifDTDigSS: i32 = 37522;
pub const PropertyTagExifDTDigitized: i32 = 36868;
pub const PropertyTagExifDTOrig: i32 = 36867;
pub const PropertyTagExifDTOrigSS: i32 = 37521;
pub const PropertyTagExifDTSubsec: i32 = 37520;
pub const PropertyTagExifDeviceSettingDesc: i32 = 41995;
pub const PropertyTagExifDigitalZoomRatio: i32 = 41988;
pub const PropertyTagExifExposureBias: i32 = 37380;
pub const PropertyTagExifExposureIndex: i32 = 41493;
pub const PropertyTagExifExposureMode: i32 = 41986;
pub const PropertyTagExifExposureProg: i32 = 34850;
pub const PropertyTagExifExposureTime: i32 = 33434;
pub const PropertyTagExifFNumber: i32 = 33437;
pub const PropertyTagExifFPXVer: i32 = 40960;
pub const PropertyTagExifFileSource: i32 = 41728;
pub const PropertyTagExifFlash: i32 = 37385;
pub const PropertyTagExifFlashEnergy: i32 = 41483;
pub const PropertyTagExifFocalLength: i32 = 37386;
pub const PropertyTagExifFocalLengthIn35mmFilm: i32 = 41989;
pub const PropertyTagExifFocalResUnit: i32 = 41488;
pub const PropertyTagExifFocalXRes: i32 = 41486;
pub const PropertyTagExifFocalYRes: i32 = 41487;
pub const PropertyTagExifGainControl: i32 = 41991;
pub const PropertyTagExifIFD: i32 = 34665;
pub const PropertyTagExifISOSpeed: i32 = 34855;
pub const PropertyTagExifInterop: i32 = 40965;
pub const PropertyTagExifLightSource: i32 = 37384;
pub const PropertyTagExifMakerNote: i32 = 37500;
pub const PropertyTagExifMaxAperture: i32 = 37381;
pub const PropertyTagExifMeteringMode: i32 = 37383;
pub const PropertyTagExifOECF: i32 = 34856;
pub const PropertyTagExifPixXDim: i32 = 40962;
pub const PropertyTagExifPixYDim: i32 = 40963;
pub const PropertyTagExifRelatedWav: i32 = 40964;
pub const PropertyTagExifSaturation: i32 = 41993;
pub const PropertyTagExifSceneCaptureType: i32 = 41990;
pub const PropertyTagExifSceneType: i32 = 41729;
pub const PropertyTagExifSensingMethod: i32 = 41495;
pub const PropertyTagExifSharpness: i32 = 41994;
pub const PropertyTagExifShutterSpeed: i32 = 37377;
pub const PropertyTagExifSpatialFR: i32 = 41484;
pub const PropertyTagExifSpectralSense: i32 = 34852;
pub const PropertyTagExifSubjectArea: i32 = 37396;
pub const PropertyTagExifSubjectDist: i32 = 37382;
pub const PropertyTagExifSubjectDistanceRange: i32 = 41996;
pub const PropertyTagExifSubjectLoc: i32 = 41492;
pub const PropertyTagExifUniqueImageID: i32 = 42016;
pub const PropertyTagExifUserComment: i32 = 37510;
pub const PropertyTagExifVer: i32 = 36864;
pub const PropertyTagExifWhiteBalance: i32 = 41987;
pub const PropertyTagExtraSamples: i32 = 338;
pub const PropertyTagFillOrder: i32 = 266;
pub const PropertyTagFrameDelay: i32 = 20736;
pub const PropertyTagFreeByteCounts: i32 = 289;
pub const PropertyTagFreeOffset: i32 = 288;
pub const PropertyTagGamma: i32 = 769;
pub const PropertyTagGlobalPalette: i32 = 20738;
pub const PropertyTagGpsAltitude: i32 = 6;
pub const PropertyTagGpsAltitudeRef: i32 = 5;
pub const PropertyTagGpsAreaInformation: i32 = 28;
pub const PropertyTagGpsDate: i32 = 29;
pub const PropertyTagGpsDestBear: i32 = 24;
pub const PropertyTagGpsDestBearRef: i32 = 23;
pub const PropertyTagGpsDestDist: i32 = 26;
pub const PropertyTagGpsDestDistRef: i32 = 25;
pub const PropertyTagGpsDestLat: i32 = 20;
pub const PropertyTagGpsDestLatRef: i32 = 19;
pub const PropertyTagGpsDestLong: i32 = 22;
pub const PropertyTagGpsDestLongRef: i32 = 21;
pub const PropertyTagGpsDifferential: i32 = 30;
pub const PropertyTagGpsGpsDop: i32 = 11;
pub const PropertyTagGpsGpsMeasureMode: i32 = 10;
pub const PropertyTagGpsGpsSatellites: i32 = 8;
pub const PropertyTagGpsGpsStatus: i32 = 9;
pub const PropertyTagGpsGpsTime: i32 = 7;
pub const PropertyTagGpsIFD: i32 = 34853;
pub const PropertyTagGpsImgDir: i32 = 17;
pub const PropertyTagGpsImgDirRef: i32 = 16;
pub const PropertyTagGpsLatitude: i32 = 2;
pub const PropertyTagGpsLatitudeRef: i32 = 1;
pub const PropertyTagGpsLongitude: i32 = 4;
pub const PropertyTagGpsLongitudeRef: i32 = 3;
pub const PropertyTagGpsMapDatum: i32 = 18;
pub const PropertyTagGpsProcessingMethod: i32 = 27;
pub const PropertyTagGpsSpeed: i32 = 13;
pub const PropertyTagGpsSpeedRef: i32 = 12;
pub const PropertyTagGpsTrack: i32 = 15;
pub const PropertyTagGpsTrackRef: i32 = 14;
pub const PropertyTagGpsVer: i32 = 0;
pub const PropertyTagGrayResponseCurve: i32 = 291;
pub const PropertyTagGrayResponseUnit: i32 = 290;
pub const PropertyTagGridSize: i32 = 20497;
pub const PropertyTagHalftoneDegree: i32 = 20492;
pub const PropertyTagHalftoneHints: i32 = 321;
pub const PropertyTagHalftoneLPI: i32 = 20490;
pub const PropertyTagHalftoneLPIUnit: i32 = 20491;
pub const PropertyTagHalftoneMisc: i32 = 20494;
pub const PropertyTagHalftoneScreen: i32 = 20495;
pub const PropertyTagHalftoneShape: i32 = 20493;
pub const PropertyTagHostComputer: i32 = 316;
pub const PropertyTagICCProfile: i32 = 34675;
pub const PropertyTagICCProfileDescriptor: i32 = 770;
pub const PropertyTagImageDescription: i32 = 270;
pub const PropertyTagImageHeight: i32 = 257;
pub const PropertyTagImageTitle: i32 = 800;
pub const PropertyTagImageWidth: i32 = 256;
pub const PropertyTagIndexBackground: i32 = 20739;
pub const PropertyTagIndexTransparent: i32 = 20740;
pub const PropertyTagInkNames: i32 = 333;
pub const PropertyTagInkSet: i32 = 332;
pub const PropertyTagJPEGACTables: i32 = 521;
pub const PropertyTagJPEGDCTables: i32 = 520;
pub const PropertyTagJPEGInterFormat: i32 = 513;
pub const PropertyTagJPEGInterLength: i32 = 514;
pub const PropertyTagJPEGLosslessPredictors: i32 = 517;
pub const PropertyTagJPEGPointTransforms: i32 = 518;
pub const PropertyTagJPEGProc: i32 = 512;
pub const PropertyTagJPEGQTables: i32 = 519;
pub const PropertyTagJPEGQuality: i32 = 20496;
pub const PropertyTagJPEGRestartInterval: i32 = 515;
pub const PropertyTagLoopCount: i32 = 20737;
pub const PropertyTagLuminanceTable: i32 = 20624;
pub const PropertyTagMaxSampleValue: i32 = 281;
pub const PropertyTagMinSampleValue: i32 = 280;
pub const PropertyTagNewSubfileType: i32 = 254;
pub const PropertyTagNumberOfInks: i32 = 334;
pub const PropertyTagOrientation: i32 = 274;
pub const PropertyTagPageName: i32 = 285;
pub const PropertyTagPageNumber: i32 = 297;
pub const PropertyTagPaletteHistogram: i32 = 20755;
pub const PropertyTagPhotometricInterp: i32 = 262;
pub const PropertyTagPixelPerUnitX: i32 = 20753;
pub const PropertyTagPixelPerUnitY: i32 = 20754;
pub const PropertyTagPixelUnit: i32 = 20752;
pub const PropertyTagPlanarConfig: i32 = 284;
pub const PropertyTagPredictor: i32 = 317;
pub const PropertyTagPrimaryChromaticities: i32 = 319;
pub const PropertyTagPrintFlags: i32 = 20485;
pub const PropertyTagPrintFlagsBleedWidth: i32 = 20488;
pub const PropertyTagPrintFlagsBleedWidthScale: i32 = 20489;
pub const PropertyTagPrintFlagsCrop: i32 = 20487;
pub const PropertyTagPrintFlagsVersion: i32 = 20486;
pub const PropertyTagREFBlackWhite: i32 = 532;
pub const PropertyTagResolutionUnit: i32 = 296;
pub const PropertyTagResolutionXLengthUnit: i32 = 20483;
pub const PropertyTagResolutionXUnit: i32 = 20481;
pub const PropertyTagResolutionYLengthUnit: i32 = 20484;
pub const PropertyTagResolutionYUnit: i32 = 20482;
pub const PropertyTagRowsPerStrip: i32 = 278;
pub const PropertyTagSMaxSampleValue: i32 = 341;
pub const PropertyTagSMinSampleValue: i32 = 340;
pub const PropertyTagSRGBRenderingIntent: i32 = 771;
pub const PropertyTagSampleFormat: i32 = 339;
pub const PropertyTagSamplesPerPixel: i32 = 277;
pub const PropertyTagSoftwareUsed: i32 = 305;
pub const PropertyTagStripBytesCount: i32 = 279;
pub const PropertyTagStripOffsets: i32 = 273;
pub const PropertyTagSubfileType: i32 = 255;
pub const PropertyTagT4Option: i32 = 292;
pub const PropertyTagT6Option: i32 = 293;
pub const PropertyTagTargetPrinter: i32 = 337;
pub const PropertyTagThreshHolding: i32 = 263;
pub const PropertyTagThumbnailArtist: i32 = 20532;
pub const PropertyTagThumbnailBitsPerSample: i32 = 20514;
pub const PropertyTagThumbnailColorDepth: i32 = 20501;
pub const PropertyTagThumbnailCompressedSize: i32 = 20505;
pub const PropertyTagThumbnailCompression: i32 = 20515;
pub const PropertyTagThumbnailCopyRight: i32 = 20539;
pub const PropertyTagThumbnailData: i32 = 20507;
pub const PropertyTagThumbnailDateTime: i32 = 20531;
pub const PropertyTagThumbnailEquipMake: i32 = 20518;
pub const PropertyTagThumbnailEquipModel: i32 = 20519;
pub const PropertyTagThumbnailFormat: i32 = 20498;
pub const PropertyTagThumbnailHeight: i32 = 20500;
pub const PropertyTagThumbnailImageDescription: i32 = 20517;
pub const PropertyTagThumbnailImageHeight: i32 = 20513;
pub const PropertyTagThumbnailImageWidth: i32 = 20512;
pub const PropertyTagThumbnailOrientation: i32 = 20521;
pub const PropertyTagThumbnailPhotometricInterp: i32 = 20516;
pub const PropertyTagThumbnailPlanarConfig: i32 = 20527;
pub const PropertyTagThumbnailPlanes: i32 = 20502;
pub const PropertyTagThumbnailPrimaryChromaticities: i32 = 20534;
pub const PropertyTagThumbnailRawBytes: i32 = 20503;
pub const PropertyTagThumbnailRefBlackWhite: i32 = 20538;
pub const PropertyTagThumbnailResolutionUnit: i32 = 20528;
pub const PropertyTagThumbnailResolutionX: i32 = 20525;
pub const PropertyTagThumbnailResolutionY: i32 = 20526;
pub const PropertyTagThumbnailRowsPerStrip: i32 = 20523;
pub const PropertyTagThumbnailSamplesPerPixel: i32 = 20522;
pub const PropertyTagThumbnailSize: i32 = 20504;
pub const PropertyTagThumbnailSoftwareUsed: i32 = 20530;
pub const PropertyTagThumbnailStripBytesCount: i32 = 20524;
pub const PropertyTagThumbnailStripOffsets: i32 = 20520;
pub const PropertyTagThumbnailTransferFunction: i32 = 20529;
pub const PropertyTagThumbnailWhitePoint: i32 = 20533;
pub const PropertyTagThumbnailWidth: i32 = 20499;
pub const PropertyTagThumbnailYCbCrCoefficients: i32 = 20535;
pub const PropertyTagThumbnailYCbCrPositioning: i32 = 20537;
pub const PropertyTagThumbnailYCbCrSubsampling: i32 = 20536;
pub const PropertyTagTileByteCounts: i32 = 325;
pub const PropertyTagTileLength: i32 = 323;
pub const PropertyTagTileOffset: i32 = 324;
pub const PropertyTagTileWidth: i32 = 322;
pub const PropertyTagTransferFuncition: i32 = 301;
pub const PropertyTagTransferRange: i32 = 342;
pub const PropertyTagTypeASCII: i32 = 2;
pub const PropertyTagTypeByte: i32 = 1;
pub const PropertyTagTypeLong: i32 = 4;
pub const PropertyTagTypeRational: i32 = 5;
pub const PropertyTagTypeSLONG: i32 = 9;
pub const PropertyTagTypeSRational: i32 = 10;
pub const PropertyTagTypeShort: i32 = 3;
pub const PropertyTagTypeUndefined: i32 = 7;
pub const PropertyTagWhitePoint: i32 = 318;
pub const PropertyTagXPosition: i32 = 286;
pub const PropertyTagXResolution: i32 = 282;
pub const PropertyTagYCbCrCoefficients: i32 = 529;
pub const PropertyTagYCbCrPositioning: i32 = 531;
pub const PropertyTagYCbCrSubsampling: i32 = 530;
pub const PropertyTagYPosition: i32 = 287;
pub const PropertyTagYResolution: i32 = 283;
pub const Rotate180FlipNone: RotateFlipType = 2;
pub const Rotate180FlipX: RotateFlipType = 6;
pub const Rotate180FlipXY: RotateFlipType = 0;
pub const Rotate180FlipY: RotateFlipType = 4;
pub const Rotate270FlipNone: RotateFlipType = 3;
pub const Rotate270FlipX: RotateFlipType = 7;
pub const Rotate270FlipXY: RotateFlipType = 1;
pub const Rotate270FlipY: RotateFlipType = 5;
pub const Rotate90FlipNone: RotateFlipType = 1;
pub const Rotate90FlipX: RotateFlipType = 5;
pub const Rotate90FlipXY: RotateFlipType = 3;
pub const Rotate90FlipY: RotateFlipType = 7;
pub type RotateFlipType = i32;
pub const RotateNoneFlipNone: RotateFlipType = 0;
pub const RotateNoneFlipX: RotateFlipType = 4;
pub const RotateNoneFlipXY: RotateFlipType = 2;
pub const RotateNoneFlipY: RotateFlipType = 6;
