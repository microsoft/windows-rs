pub const ALPHA_SHIFT: u32 = 24u32;
pub const Aborted: Status = 9i32;
pub const AccessDenied: Status = 12i32;
pub const AdjustBlackSaturation: CurveAdjustments = 7i32;
pub const AdjustContrast: CurveAdjustments = 2i32;
pub const AdjustDensity: CurveAdjustments = 1i32;
pub const AdjustExposure: CurveAdjustments = 0i32;
pub const AdjustHighlight: CurveAdjustments = 3i32;
pub const AdjustMidtone: CurveAdjustments = 5i32;
pub const AdjustShadow: CurveAdjustments = 4i32;
pub const AdjustWhiteSaturation: CurveAdjustments = 6i32;
pub const BLUE_SHIFT: u32 = 0u32;
pub const BlurEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0x633c80a4_1843_482b_9ef2_be2834c5fdd4);
pub const BrightnessContrastEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0xd3a1dbe1_8ec4_4c17_9f4c_ea97ad1c343d);
pub const BrushTypeHatchFill: BrushType = 1i32;
pub const BrushTypeLinearGradient: BrushType = 4i32;
pub const BrushTypePathGradient: BrushType = 3i32;
pub const BrushTypeSolidColor: BrushType = 0i32;
pub const BrushTypeTextureFill: BrushType = 2i32;
pub const CodecIImageBytes: windows_core::GUID = windows_core::GUID::from_u128(0x025d1823_6c7d_447b_bbdb_a3cbc3dfa2fc);
pub const ColorAdjustTypeAny: ColorAdjustType = 6i32;
pub const ColorAdjustTypeBitmap: ColorAdjustType = 1i32;
pub const ColorAdjustTypeBrush: ColorAdjustType = 2i32;
pub const ColorAdjustTypeCount: ColorAdjustType = 5i32;
pub const ColorAdjustTypeDefault: ColorAdjustType = 0i32;
pub const ColorAdjustTypePen: ColorAdjustType = 3i32;
pub const ColorAdjustTypeText: ColorAdjustType = 4i32;
pub const ColorBalanceEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0x537e597d_251e_48da_9664_29ca496b70f8);
pub const ColorChannelFlagsC: ColorChannelFlags = 0i32;
pub const ColorChannelFlagsK: ColorChannelFlags = 3i32;
pub const ColorChannelFlagsLast: ColorChannelFlags = 4i32;
pub const ColorChannelFlagsM: ColorChannelFlags = 1i32;
pub const ColorChannelFlagsY: ColorChannelFlags = 2i32;
pub const ColorCurveEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0xdd6a0022_58e4_4a67_9d9b_d48eb881a53d);
pub const ColorLUTEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0xa7ce72a9_0f7f_40d7_b3cc_d0c02d5c3212);
pub const ColorMatrixEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0x718f2615_7933_40e3_a511_5f68fe14dd74);
pub const ColorMatrixFlagsAltGray: ColorMatrixFlags = 2i32;
pub const ColorMatrixFlagsDefault: ColorMatrixFlags = 0i32;
pub const ColorMatrixFlagsSkipGrays: ColorMatrixFlags = 1i32;
pub const ColorModeARGB32: ColorMode = 0i32;
pub const ColorModeARGB64: ColorMode = 1i32;
pub const CombineModeComplement: CombineMode = 5i32;
pub const CombineModeExclude: CombineMode = 4i32;
pub const CombineModeIntersect: CombineMode = 1i32;
pub const CombineModeReplace: CombineMode = 0i32;
pub const CombineModeUnion: CombineMode = 2i32;
pub const CombineModeXor: CombineMode = 3i32;
pub const CompositingModeSourceCopy: CompositingMode = 1i32;
pub const CompositingModeSourceOver: CompositingMode = 0i32;
pub const CompositingQualityAssumeLinear: CompositingQuality = 4i32;
pub const CompositingQualityDefault: CompositingQuality = 0i32;
pub const CompositingQualityGammaCorrected: CompositingQuality = 3i32;
pub const CompositingQualityHighQuality: CompositingQuality = 2i32;
pub const CompositingQualityHighSpeed: CompositingQuality = 1i32;
pub const CompositingQualityInvalid: CompositingQuality = -1i32;
pub const ConvertToEmfPlusFlagsDefault: ConvertToEmfPlusFlags = 0i32;
pub const ConvertToEmfPlusFlagsInvalidRecord: ConvertToEmfPlusFlags = 4i32;
pub const ConvertToEmfPlusFlagsRopUsed: ConvertToEmfPlusFlags = 1i32;
pub const ConvertToEmfPlusFlagsText: ConvertToEmfPlusFlags = 2i32;
pub const CoordinateSpaceDevice: CoordinateSpace = 2i32;
pub const CoordinateSpacePage: CoordinateSpace = 1i32;
pub const CoordinateSpaceWorld: CoordinateSpace = 0i32;
pub const CurveChannelAll: CurveChannel = 0i32;
pub const CurveChannelBlue: CurveChannel = 3i32;
pub const CurveChannelGreen: CurveChannel = 2i32;
pub const CurveChannelRed: CurveChannel = 1i32;
pub const CustomLineCapTypeAdjustableArrow: CustomLineCapType = 1i32;
pub const CustomLineCapTypeDefault: CustomLineCapType = 0i32;
pub const DashCapFlat: DashCap = 0i32;
pub const DashCapRound: DashCap = 2i32;
pub const DashCapTriangle: DashCap = 3i32;
pub const DashStyleCustom: DashStyle = 5i32;
pub const DashStyleDash: DashStyle = 1i32;
pub const DashStyleDashDot: DashStyle = 3i32;
pub const DashStyleDashDotDot: DashStyle = 4i32;
pub const DashStyleDot: DashStyle = 2i32;
pub const DashStyleSolid: DashStyle = 0i32;
pub const DebugEventLevelFatal: DebugEventLevel = 0i32;
pub const DebugEventLevelWarning: DebugEventLevel = 1i32;
pub const DitherTypeDualSpiral4x4: DitherType = 7i32;
pub const DitherTypeDualSpiral8x8: DitherType = 8i32;
pub const DitherTypeErrorDiffusion: DitherType = 9i32;
pub const DitherTypeMax: DitherType = 10i32;
pub const DitherTypeNone: DitherType = 0i32;
pub const DitherTypeOrdered16x16: DitherType = 4i32;
pub const DitherTypeOrdered4x4: DitherType = 2i32;
pub const DitherTypeOrdered8x8: DitherType = 3i32;
pub const DitherTypeSolid: DitherType = 1i32;
pub const DitherTypeSpiral4x4: DitherType = 5i32;
pub const DitherTypeSpiral8x8: DitherType = 6i32;
pub const DriverStringOptionsCmapLookup: DriverStringOptions = 1i32;
pub const DriverStringOptionsLimitSubpixel: DriverStringOptions = 8i32;
pub const DriverStringOptionsRealizedAdvance: DriverStringOptions = 4i32;
pub const DriverStringOptionsVertical: DriverStringOptions = 2i32;
pub const EmfPlusRecordTotal: EmfPlusRecordType = 16443i32;
pub const EmfPlusRecordTypeBeginContainer: EmfPlusRecordType = 16423i32;
pub const EmfPlusRecordTypeBeginContainerNoParams: EmfPlusRecordType = 16424i32;
pub const EmfPlusRecordTypeClear: EmfPlusRecordType = 16393i32;
pub const EmfPlusRecordTypeComment: EmfPlusRecordType = 16387i32;
pub const EmfPlusRecordTypeDrawArc: EmfPlusRecordType = 16402i32;
pub const EmfPlusRecordTypeDrawBeziers: EmfPlusRecordType = 16409i32;
pub const EmfPlusRecordTypeDrawClosedCurve: EmfPlusRecordType = 16407i32;
pub const EmfPlusRecordTypeDrawCurve: EmfPlusRecordType = 16408i32;
pub const EmfPlusRecordTypeDrawDriverString: EmfPlusRecordType = 16438i32;
pub const EmfPlusRecordTypeDrawEllipse: EmfPlusRecordType = 16399i32;
pub const EmfPlusRecordTypeDrawImage: EmfPlusRecordType = 16410i32;
pub const EmfPlusRecordTypeDrawImagePoints: EmfPlusRecordType = 16411i32;
pub const EmfPlusRecordTypeDrawLines: EmfPlusRecordType = 16397i32;
pub const EmfPlusRecordTypeDrawPath: EmfPlusRecordType = 16405i32;
pub const EmfPlusRecordTypeDrawPie: EmfPlusRecordType = 16401i32;
pub const EmfPlusRecordTypeDrawRects: EmfPlusRecordType = 16395i32;
pub const EmfPlusRecordTypeDrawString: EmfPlusRecordType = 16412i32;
pub const EmfPlusRecordTypeEndContainer: EmfPlusRecordType = 16425i32;
pub const EmfPlusRecordTypeEndOfFile: EmfPlusRecordType = 16386i32;
pub const EmfPlusRecordTypeFillClosedCurve: EmfPlusRecordType = 16406i32;
pub const EmfPlusRecordTypeFillEllipse: EmfPlusRecordType = 16398i32;
pub const EmfPlusRecordTypeFillPath: EmfPlusRecordType = 16404i32;
pub const EmfPlusRecordTypeFillPie: EmfPlusRecordType = 16400i32;
pub const EmfPlusRecordTypeFillPolygon: EmfPlusRecordType = 16396i32;
pub const EmfPlusRecordTypeFillRects: EmfPlusRecordType = 16394i32;
pub const EmfPlusRecordTypeFillRegion: EmfPlusRecordType = 16403i32;
pub const EmfPlusRecordTypeGetDC: EmfPlusRecordType = 16388i32;
pub const EmfPlusRecordTypeHeader: EmfPlusRecordType = 16385i32;
pub const EmfPlusRecordTypeInvalid: EmfPlusRecordType = 16384i32;
pub const EmfPlusRecordTypeMax: EmfPlusRecordType = 16442i32;
pub const EmfPlusRecordTypeMin: EmfPlusRecordType = 16385i32;
pub const EmfPlusRecordTypeMultiFormatEnd: EmfPlusRecordType = 16391i32;
pub const EmfPlusRecordTypeMultiFormatSection: EmfPlusRecordType = 16390i32;
pub const EmfPlusRecordTypeMultiFormatStart: EmfPlusRecordType = 16389i32;
pub const EmfPlusRecordTypeMultiplyWorldTransform: EmfPlusRecordType = 16428i32;
pub const EmfPlusRecordTypeObject: EmfPlusRecordType = 16392i32;
pub const EmfPlusRecordTypeOffsetClip: EmfPlusRecordType = 16437i32;
pub const EmfPlusRecordTypeResetClip: EmfPlusRecordType = 16433i32;
pub const EmfPlusRecordTypeResetWorldTransform: EmfPlusRecordType = 16427i32;
pub const EmfPlusRecordTypeRestore: EmfPlusRecordType = 16422i32;
pub const EmfPlusRecordTypeRotateWorldTransform: EmfPlusRecordType = 16431i32;
pub const EmfPlusRecordTypeSave: EmfPlusRecordType = 16421i32;
pub const EmfPlusRecordTypeScaleWorldTransform: EmfPlusRecordType = 16430i32;
pub const EmfPlusRecordTypeSerializableObject: EmfPlusRecordType = 16440i32;
pub const EmfPlusRecordTypeSetAntiAliasMode: EmfPlusRecordType = 16414i32;
pub const EmfPlusRecordTypeSetClipPath: EmfPlusRecordType = 16435i32;
pub const EmfPlusRecordTypeSetClipRect: EmfPlusRecordType = 16434i32;
pub const EmfPlusRecordTypeSetClipRegion: EmfPlusRecordType = 16436i32;
pub const EmfPlusRecordTypeSetCompositingMode: EmfPlusRecordType = 16419i32;
pub const EmfPlusRecordTypeSetCompositingQuality: EmfPlusRecordType = 16420i32;
pub const EmfPlusRecordTypeSetInterpolationMode: EmfPlusRecordType = 16417i32;
pub const EmfPlusRecordTypeSetPageTransform: EmfPlusRecordType = 16432i32;
pub const EmfPlusRecordTypeSetPixelOffsetMode: EmfPlusRecordType = 16418i32;
pub const EmfPlusRecordTypeSetRenderingOrigin: EmfPlusRecordType = 16413i32;
pub const EmfPlusRecordTypeSetTSClip: EmfPlusRecordType = 16442i32;
pub const EmfPlusRecordTypeSetTSGraphics: EmfPlusRecordType = 16441i32;
pub const EmfPlusRecordTypeSetTextContrast: EmfPlusRecordType = 16416i32;
pub const EmfPlusRecordTypeSetTextRenderingHint: EmfPlusRecordType = 16415i32;
pub const EmfPlusRecordTypeSetWorldTransform: EmfPlusRecordType = 16426i32;
pub const EmfPlusRecordTypeStrokeFillPath: EmfPlusRecordType = 16439i32;
pub const EmfPlusRecordTypeTranslateWorldTransform: EmfPlusRecordType = 16429i32;
pub const EmfRecordTypeAbortPath: EmfPlusRecordType = 68i32;
pub const EmfRecordTypeAlphaBlend: EmfPlusRecordType = 114i32;
pub const EmfRecordTypeAngleArc: EmfPlusRecordType = 41i32;
pub const EmfRecordTypeArc: EmfPlusRecordType = 45i32;
pub const EmfRecordTypeArcTo: EmfPlusRecordType = 55i32;
pub const EmfRecordTypeBeginPath: EmfPlusRecordType = 59i32;
pub const EmfRecordTypeBitBlt: EmfPlusRecordType = 76i32;
pub const EmfRecordTypeChord: EmfPlusRecordType = 46i32;
pub const EmfRecordTypeCloseFigure: EmfPlusRecordType = 61i32;
pub const EmfRecordTypeColorCorrectPalette: EmfPlusRecordType = 111i32;
pub const EmfRecordTypeColorMatchToTargetW: EmfPlusRecordType = 121i32;
pub const EmfRecordTypeCreateBrushIndirect: EmfPlusRecordType = 39i32;
pub const EmfRecordTypeCreateColorSpace: EmfPlusRecordType = 99i32;
pub const EmfRecordTypeCreateColorSpaceW: EmfPlusRecordType = 122i32;
pub const EmfRecordTypeCreateDIBPatternBrushPt: EmfPlusRecordType = 94i32;
pub const EmfRecordTypeCreateMonoBrush: EmfPlusRecordType = 93i32;
pub const EmfRecordTypeCreatePalette: EmfPlusRecordType = 49i32;
pub const EmfRecordTypeCreatePen: EmfPlusRecordType = 38i32;
pub const EmfRecordTypeDeleteColorSpace: EmfPlusRecordType = 101i32;
pub const EmfRecordTypeDeleteObject: EmfPlusRecordType = 40i32;
pub const EmfRecordTypeDrawEscape: EmfPlusRecordType = 105i32;
pub const EmfRecordTypeEOF: EmfPlusRecordType = 14i32;
pub const EmfRecordTypeEllipse: EmfPlusRecordType = 42i32;
pub const EmfRecordTypeEndPath: EmfPlusRecordType = 60i32;
pub const EmfRecordTypeExcludeClipRect: EmfPlusRecordType = 29i32;
pub const EmfRecordTypeExtCreateFontIndirect: EmfPlusRecordType = 82i32;
pub const EmfRecordTypeExtCreatePen: EmfPlusRecordType = 95i32;
pub const EmfRecordTypeExtEscape: EmfPlusRecordType = 106i32;
pub const EmfRecordTypeExtFloodFill: EmfPlusRecordType = 53i32;
pub const EmfRecordTypeExtSelectClipRgn: EmfPlusRecordType = 75i32;
pub const EmfRecordTypeExtTextOutA: EmfPlusRecordType = 83i32;
pub const EmfRecordTypeExtTextOutW: EmfPlusRecordType = 84i32;
pub const EmfRecordTypeFillPath: EmfPlusRecordType = 62i32;
pub const EmfRecordTypeFillRgn: EmfPlusRecordType = 71i32;
pub const EmfRecordTypeFlattenPath: EmfPlusRecordType = 65i32;
pub const EmfRecordTypeForceUFIMapping: EmfPlusRecordType = 109i32;
pub const EmfRecordTypeFrameRgn: EmfPlusRecordType = 72i32;
pub const EmfRecordTypeGLSBoundedRecord: EmfPlusRecordType = 103i32;
pub const EmfRecordTypeGLSRecord: EmfPlusRecordType = 102i32;
pub const EmfRecordTypeGdiComment: EmfPlusRecordType = 70i32;
pub const EmfRecordTypeGradientFill: EmfPlusRecordType = 118i32;
pub const EmfRecordTypeHeader: EmfPlusRecordType = 1i32;
pub const EmfRecordTypeIntersectClipRect: EmfPlusRecordType = 30i32;
pub const EmfRecordTypeInvertRgn: EmfPlusRecordType = 73i32;
pub const EmfRecordTypeLineTo: EmfPlusRecordType = 54i32;
pub const EmfRecordTypeMaskBlt: EmfPlusRecordType = 78i32;
pub const EmfRecordTypeMax: EmfPlusRecordType = 122i32;
pub const EmfRecordTypeMin: EmfPlusRecordType = 1i32;
pub const EmfRecordTypeModifyWorldTransform: EmfPlusRecordType = 36i32;
pub const EmfRecordTypeMoveToEx: EmfPlusRecordType = 27i32;
pub const EmfRecordTypeNamedEscape: EmfPlusRecordType = 110i32;
pub const EmfRecordTypeOffsetClipRgn: EmfPlusRecordType = 26i32;
pub const EmfRecordTypePaintRgn: EmfPlusRecordType = 74i32;
pub const EmfRecordTypePie: EmfPlusRecordType = 47i32;
pub const EmfRecordTypePixelFormat: EmfPlusRecordType = 104i32;
pub const EmfRecordTypePlgBlt: EmfPlusRecordType = 79i32;
pub const EmfRecordTypePolyBezier: EmfPlusRecordType = 2i32;
pub const EmfRecordTypePolyBezier16: EmfPlusRecordType = 85i32;
pub const EmfRecordTypePolyBezierTo: EmfPlusRecordType = 5i32;
pub const EmfRecordTypePolyBezierTo16: EmfPlusRecordType = 88i32;
pub const EmfRecordTypePolyDraw: EmfPlusRecordType = 56i32;
pub const EmfRecordTypePolyDraw16: EmfPlusRecordType = 92i32;
pub const EmfRecordTypePolyLineTo: EmfPlusRecordType = 6i32;
pub const EmfRecordTypePolyPolygon: EmfPlusRecordType = 8i32;
pub const EmfRecordTypePolyPolygon16: EmfPlusRecordType = 91i32;
pub const EmfRecordTypePolyPolyline: EmfPlusRecordType = 7i32;
pub const EmfRecordTypePolyPolyline16: EmfPlusRecordType = 90i32;
pub const EmfRecordTypePolyTextOutA: EmfPlusRecordType = 96i32;
pub const EmfRecordTypePolyTextOutW: EmfPlusRecordType = 97i32;
pub const EmfRecordTypePolygon: EmfPlusRecordType = 3i32;
pub const EmfRecordTypePolygon16: EmfPlusRecordType = 86i32;
pub const EmfRecordTypePolyline: EmfPlusRecordType = 4i32;
pub const EmfRecordTypePolyline16: EmfPlusRecordType = 87i32;
pub const EmfRecordTypePolylineTo16: EmfPlusRecordType = 89i32;
pub const EmfRecordTypeRealizePalette: EmfPlusRecordType = 52i32;
pub const EmfRecordTypeRectangle: EmfPlusRecordType = 43i32;
pub const EmfRecordTypeReserved_069: EmfPlusRecordType = 69i32;
pub const EmfRecordTypeReserved_117: EmfPlusRecordType = 117i32;
pub const EmfRecordTypeResizePalette: EmfPlusRecordType = 51i32;
pub const EmfRecordTypeRestoreDC: EmfPlusRecordType = 34i32;
pub const EmfRecordTypeRoundRect: EmfPlusRecordType = 44i32;
pub const EmfRecordTypeSaveDC: EmfPlusRecordType = 33i32;
pub const EmfRecordTypeScaleViewportExtEx: EmfPlusRecordType = 31i32;
pub const EmfRecordTypeScaleWindowExtEx: EmfPlusRecordType = 32i32;
pub const EmfRecordTypeSelectClipPath: EmfPlusRecordType = 67i32;
pub const EmfRecordTypeSelectObject: EmfPlusRecordType = 37i32;
pub const EmfRecordTypeSelectPalette: EmfPlusRecordType = 48i32;
pub const EmfRecordTypeSetArcDirection: EmfPlusRecordType = 57i32;
pub const EmfRecordTypeSetBkColor: EmfPlusRecordType = 25i32;
pub const EmfRecordTypeSetBkMode: EmfPlusRecordType = 18i32;
pub const EmfRecordTypeSetBrushOrgEx: EmfPlusRecordType = 13i32;
pub const EmfRecordTypeSetColorAdjustment: EmfPlusRecordType = 23i32;
pub const EmfRecordTypeSetColorSpace: EmfPlusRecordType = 100i32;
pub const EmfRecordTypeSetDIBitsToDevice: EmfPlusRecordType = 80i32;
pub const EmfRecordTypeSetICMMode: EmfPlusRecordType = 98i32;
pub const EmfRecordTypeSetICMProfileA: EmfPlusRecordType = 112i32;
pub const EmfRecordTypeSetICMProfileW: EmfPlusRecordType = 113i32;
pub const EmfRecordTypeSetLayout: EmfPlusRecordType = 115i32;
pub const EmfRecordTypeSetLinkedUFIs: EmfPlusRecordType = 119i32;
pub const EmfRecordTypeSetMapMode: EmfPlusRecordType = 17i32;
pub const EmfRecordTypeSetMapperFlags: EmfPlusRecordType = 16i32;
pub const EmfRecordTypeSetMetaRgn: EmfPlusRecordType = 28i32;
pub const EmfRecordTypeSetMiterLimit: EmfPlusRecordType = 58i32;
pub const EmfRecordTypeSetPaletteEntries: EmfPlusRecordType = 50i32;
pub const EmfRecordTypeSetPixelV: EmfPlusRecordType = 15i32;
pub const EmfRecordTypeSetPolyFillMode: EmfPlusRecordType = 19i32;
pub const EmfRecordTypeSetROP2: EmfPlusRecordType = 20i32;
pub const EmfRecordTypeSetStretchBltMode: EmfPlusRecordType = 21i32;
pub const EmfRecordTypeSetTextAlign: EmfPlusRecordType = 22i32;
pub const EmfRecordTypeSetTextColor: EmfPlusRecordType = 24i32;
pub const EmfRecordTypeSetTextJustification: EmfPlusRecordType = 120i32;
pub const EmfRecordTypeSetViewportExtEx: EmfPlusRecordType = 11i32;
pub const EmfRecordTypeSetViewportOrgEx: EmfPlusRecordType = 12i32;
pub const EmfRecordTypeSetWindowExtEx: EmfPlusRecordType = 9i32;
pub const EmfRecordTypeSetWindowOrgEx: EmfPlusRecordType = 10i32;
pub const EmfRecordTypeSetWorldTransform: EmfPlusRecordType = 35i32;
pub const EmfRecordTypeSmallTextOut: EmfPlusRecordType = 108i32;
pub const EmfRecordTypeStartDoc: EmfPlusRecordType = 107i32;
pub const EmfRecordTypeStretchBlt: EmfPlusRecordType = 77i32;
pub const EmfRecordTypeStretchDIBits: EmfPlusRecordType = 81i32;
pub const EmfRecordTypeStrokeAndFillPath: EmfPlusRecordType = 63i32;
pub const EmfRecordTypeStrokePath: EmfPlusRecordType = 64i32;
pub const EmfRecordTypeTransparentBlt: EmfPlusRecordType = 116i32;
pub const EmfRecordTypeWidenPath: EmfPlusRecordType = 66i32;
pub const EmfToWmfBitsFlagsDefault: EmfToWmfBitsFlags = 0i32;
pub const EmfToWmfBitsFlagsEmbedEmf: EmfToWmfBitsFlags = 1i32;
pub const EmfToWmfBitsFlagsIncludePlaceable: EmfToWmfBitsFlags = 2i32;
pub const EmfToWmfBitsFlagsNoXORClip: EmfToWmfBitsFlags = 4i32;
pub const EmfTypeEmfOnly: EmfType = 3i32;
pub const EmfTypeEmfPlusDual: EmfType = 5i32;
pub const EmfTypeEmfPlusOnly: EmfType = 4i32;
pub const EncoderChrominanceTable: windows_core::GUID = windows_core::GUID::from_u128(0xf2e455dc_09b3_4316_8260_676ada32481c);
pub const EncoderColorDepth: windows_core::GUID = windows_core::GUID::from_u128(0x66087055_ad66_4c7c_9a18_38a2310b8337);
pub const EncoderColorSpace: windows_core::GUID = windows_core::GUID::from_u128(0xae7a62a0_ee2c_49d8_9d07_1ba8a927596e);
pub const EncoderCompression: windows_core::GUID = windows_core::GUID::from_u128(0xe09d739d_ccd4_44ee_8eba_3fbf8be4fc58);
pub const EncoderImageItems: windows_core::GUID = windows_core::GUID::from_u128(0x63875e13_1f1d_45ab_9195_a29b6066a650);
pub const EncoderLuminanceTable: windows_core::GUID = windows_core::GUID::from_u128(0xedb33bce_0266_4a77_b904_27216099e717);
pub const EncoderParameterValueTypeASCII: EncoderParameterValueType = 2i32;
pub const EncoderParameterValueTypeByte: EncoderParameterValueType = 1i32;
pub const EncoderParameterValueTypeLong: EncoderParameterValueType = 4i32;
pub const EncoderParameterValueTypeLongRange: EncoderParameterValueType = 6i32;
pub const EncoderParameterValueTypePointer: EncoderParameterValueType = 9i32;
pub const EncoderParameterValueTypeRational: EncoderParameterValueType = 5i32;
pub const EncoderParameterValueTypeRationalRange: EncoderParameterValueType = 8i32;
pub const EncoderParameterValueTypeShort: EncoderParameterValueType = 3i32;
pub const EncoderParameterValueTypeUndefined: EncoderParameterValueType = 7i32;
pub const EncoderQuality: windows_core::GUID = windows_core::GUID::from_u128(0x1d5be4b5_fa4a_452d_9cdd_5db35105e7eb);
pub const EncoderRenderMethod: windows_core::GUID = windows_core::GUID::from_u128(0x6d42c53a_229a_4825_8bb7_5c99e2b9a8b8);
pub const EncoderSaveAsCMYK: windows_core::GUID = windows_core::GUID::from_u128(0xa219bbc9_0a9d_4005_a3ee_3a421b8bb06c);
pub const EncoderSaveFlag: windows_core::GUID = windows_core::GUID::from_u128(0x292266fc_ac40_47bf_8cfc_a85b89a655de);
pub const EncoderScanMethod: windows_core::GUID = windows_core::GUID::from_u128(0x3a4e2661_3109_4e56_8536_42c156e7dcfa);
pub const EncoderTransformation: windows_core::GUID = windows_core::GUID::from_u128(0x8d0eb2d1_a58e_4ea8_aa14_108074b7b6f9);
pub const EncoderValueColorTypeCMYK: EncoderValue = 0i32;
pub const EncoderValueColorTypeGray: EncoderValue = 24i32;
pub const EncoderValueColorTypeRGB: EncoderValue = 25i32;
pub const EncoderValueColorTypeYCCK: EncoderValue = 1i32;
pub const EncoderValueCompressionCCITT3: EncoderValue = 3i32;
pub const EncoderValueCompressionCCITT4: EncoderValue = 4i32;
pub const EncoderValueCompressionLZW: EncoderValue = 2i32;
pub const EncoderValueCompressionNone: EncoderValue = 6i32;
pub const EncoderValueCompressionRle: EncoderValue = 5i32;
pub const EncoderValueFlush: EncoderValue = 20i32;
pub const EncoderValueFrameDimensionPage: EncoderValue = 23i32;
pub const EncoderValueFrameDimensionResolution: EncoderValue = 22i32;
pub const EncoderValueFrameDimensionTime: EncoderValue = 21i32;
pub const EncoderValueLastFrame: EncoderValue = 19i32;
pub const EncoderValueMultiFrame: EncoderValue = 18i32;
pub const EncoderValueRenderNonProgressive: EncoderValue = 12i32;
pub const EncoderValueRenderProgressive: EncoderValue = 11i32;
pub const EncoderValueScanMethodInterlaced: EncoderValue = 7i32;
pub const EncoderValueScanMethodNonInterlaced: EncoderValue = 8i32;
pub const EncoderValueTransformFlipHorizontal: EncoderValue = 16i32;
pub const EncoderValueTransformFlipVertical: EncoderValue = 17i32;
pub const EncoderValueTransformRotate180: EncoderValue = 14i32;
pub const EncoderValueTransformRotate270: EncoderValue = 15i32;
pub const EncoderValueTransformRotate90: EncoderValue = 13i32;
pub const EncoderValueVersionGif87: EncoderValue = 9i32;
pub const EncoderValueVersionGif89: EncoderValue = 10i32;
pub const EncoderVersion: windows_core::GUID = windows_core::GUID::from_u128(0x24d18c76_814a_41a4_bf53_1c219cccf797);
pub const FileNotFound: Status = 10i32;
pub const FillModeAlternate: FillMode = 0i32;
pub const FillModeWinding: FillMode = 1i32;
pub const FlatnessDefault: f32 = 0.25f32;
pub const FlushIntentionFlush: FlushIntention = 0i32;
pub const FlushIntentionSync: FlushIntention = 1i32;
pub const FontFamilyNotFound: Status = 14i32;
pub const FontStyleBold: FontStyle = 1i32;
pub const FontStyleBoldItalic: FontStyle = 3i32;
pub const FontStyleItalic: FontStyle = 2i32;
pub const FontStyleNotFound: Status = 15i32;
pub const FontStyleRegular: FontStyle = 0i32;
pub const FontStyleStrikeout: FontStyle = 8i32;
pub const FontStyleUnderline: FontStyle = 4i32;
pub const FormatIDImageInformation: windows_core::GUID = windows_core::GUID::from_u128(0xe5836cbe_5eef_4f1d_acde_ae4c43b608ce);
pub const FormatIDJpegAppHeaders: windows_core::GUID = windows_core::GUID::from_u128(0x1c4afdcd_6177_43cf_abc7_5f51af39ee85);
pub const FrameDimensionPage: windows_core::GUID = windows_core::GUID::from_u128(0x7462dc86_6180_4c7e_8e3f_ee7333a7a483);
pub const FrameDimensionResolution: windows_core::GUID = windows_core::GUID::from_u128(0x84236f7b_3bd3_428f_8dab_4ea1439ca315);
pub const FrameDimensionTime: windows_core::GUID = windows_core::GUID::from_u128(0x6aedbd6d_3fb5_418a_83a6_7f45229dc872);
pub const GDIP_EMFPLUSFLAGS_DISPLAY: u32 = 1u32;
pub const GDIP_EMFPLUS_RECORD_BASE: u32 = 16384u32;
pub const GDIP_WMF_RECORD_BASE: u32 = 65536u32;
pub const GREEN_SHIFT: u32 = 8u32;
pub const GdiplusNotInitialized: Status = 18i32;
pub const GdiplusStartupDefault: GdiplusStartupParams = 0i32;
pub const GdiplusStartupNoSetRound: GdiplusStartupParams = 1i32;
pub const GdiplusStartupSetPSValue: GdiplusStartupParams = 2i32;
pub const GdiplusStartupTransparencyMask: GdiplusStartupParams = -16777216i32;
pub const GenericError: Status = 1i32;
pub const GenericFontFamilyMonospace: GenericFontFamily = 2i32;
pub const GenericFontFamilySansSerif: GenericFontFamily = 1i32;
pub const GenericFontFamilySerif: GenericFontFamily = 0i32;
pub const HatchStyle05Percent: HatchStyle = 6i32;
pub const HatchStyle10Percent: HatchStyle = 7i32;
pub const HatchStyle20Percent: HatchStyle = 8i32;
pub const HatchStyle25Percent: HatchStyle = 9i32;
pub const HatchStyle30Percent: HatchStyle = 10i32;
pub const HatchStyle40Percent: HatchStyle = 11i32;
pub const HatchStyle50Percent: HatchStyle = 12i32;
pub const HatchStyle60Percent: HatchStyle = 13i32;
pub const HatchStyle70Percent: HatchStyle = 14i32;
pub const HatchStyle75Percent: HatchStyle = 15i32;
pub const HatchStyle80Percent: HatchStyle = 16i32;
pub const HatchStyle90Percent: HatchStyle = 17i32;
pub const HatchStyleBackwardDiagonal: HatchStyle = 3i32;
pub const HatchStyleCross: HatchStyle = 4i32;
pub const HatchStyleDarkDownwardDiagonal: HatchStyle = 20i32;
pub const HatchStyleDarkHorizontal: HatchStyle = 29i32;
pub const HatchStyleDarkUpwardDiagonal: HatchStyle = 21i32;
pub const HatchStyleDarkVertical: HatchStyle = 28i32;
pub const HatchStyleDashedDownwardDiagonal: HatchStyle = 30i32;
pub const HatchStyleDashedHorizontal: HatchStyle = 32i32;
pub const HatchStyleDashedUpwardDiagonal: HatchStyle = 31i32;
pub const HatchStyleDashedVertical: HatchStyle = 33i32;
pub const HatchStyleDiagonalBrick: HatchStyle = 38i32;
pub const HatchStyleDiagonalCross: HatchStyle = 5i32;
pub const HatchStyleDivot: HatchStyle = 42i32;
pub const HatchStyleDottedDiamond: HatchStyle = 44i32;
pub const HatchStyleDottedGrid: HatchStyle = 43i32;
pub const HatchStyleForwardDiagonal: HatchStyle = 2i32;
pub const HatchStyleHorizontal: HatchStyle = 0i32;
pub const HatchStyleHorizontalBrick: HatchStyle = 39i32;
pub const HatchStyleLargeCheckerBoard: HatchStyle = 50i32;
pub const HatchStyleLargeConfetti: HatchStyle = 35i32;
pub const HatchStyleLargeGrid: HatchStyle = 4i32;
pub const HatchStyleLightDownwardDiagonal: HatchStyle = 18i32;
pub const HatchStyleLightHorizontal: HatchStyle = 25i32;
pub const HatchStyleLightUpwardDiagonal: HatchStyle = 19i32;
pub const HatchStyleLightVertical: HatchStyle = 24i32;
pub const HatchStyleMax: HatchStyle = 52i32;
pub const HatchStyleMin: HatchStyle = 0i32;
pub const HatchStyleNarrowHorizontal: HatchStyle = 27i32;
pub const HatchStyleNarrowVertical: HatchStyle = 26i32;
pub const HatchStyleOutlinedDiamond: HatchStyle = 51i32;
pub const HatchStylePlaid: HatchStyle = 41i32;
pub const HatchStyleShingle: HatchStyle = 45i32;
pub const HatchStyleSmallCheckerBoard: HatchStyle = 49i32;
pub const HatchStyleSmallConfetti: HatchStyle = 34i32;
pub const HatchStyleSmallGrid: HatchStyle = 48i32;
pub const HatchStyleSolidDiamond: HatchStyle = 52i32;
pub const HatchStyleSphere: HatchStyle = 47i32;
pub const HatchStyleTotal: HatchStyle = 53i32;
pub const HatchStyleTrellis: HatchStyle = 46i32;
pub const HatchStyleVertical: HatchStyle = 1i32;
pub const HatchStyleWave: HatchStyle = 37i32;
pub const HatchStyleWeave: HatchStyle = 40i32;
pub const HatchStyleWideDownwardDiagonal: HatchStyle = 22i32;
pub const HatchStyleWideUpwardDiagonal: HatchStyle = 23i32;
pub const HatchStyleZigZag: HatchStyle = 36i32;
pub const HistogramFormatA: HistogramFormat = 7i32;
pub const HistogramFormatARGB: HistogramFormat = 0i32;
pub const HistogramFormatB: HistogramFormat = 4i32;
pub const HistogramFormatG: HistogramFormat = 5i32;
pub const HistogramFormatGray: HistogramFormat = 3i32;
pub const HistogramFormatPARGB: HistogramFormat = 1i32;
pub const HistogramFormatR: HistogramFormat = 6i32;
pub const HistogramFormatRGB: HistogramFormat = 2i32;
pub const HotkeyPrefixHide: HotkeyPrefix = 2i32;
pub const HotkeyPrefixNone: HotkeyPrefix = 0i32;
pub const HotkeyPrefixShow: HotkeyPrefix = 1i32;
pub const HueSaturationLightnessEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0x8b2dd6c3_eb07_4d87_a5f0_7108e26a9c5f);
pub const ImageCodecFlagsBlockingDecode: ImageCodecFlags = 32i32;
pub const ImageCodecFlagsBuiltin: ImageCodecFlags = 65536i32;
pub const ImageCodecFlagsDecoder: ImageCodecFlags = 2i32;
pub const ImageCodecFlagsEncoder: ImageCodecFlags = 1i32;
pub const ImageCodecFlagsSeekableEncode: ImageCodecFlags = 16i32;
pub const ImageCodecFlagsSupportBitmap: ImageCodecFlags = 4i32;
pub const ImageCodecFlagsSupportVector: ImageCodecFlags = 8i32;
pub const ImageCodecFlagsSystem: ImageCodecFlags = 131072i32;
pub const ImageCodecFlagsUser: ImageCodecFlags = 262144i32;
pub const ImageFlagsCaching: ImageFlags = 131072i32;
pub const ImageFlagsColorSpaceCMYK: ImageFlags = 32i32;
pub const ImageFlagsColorSpaceGRAY: ImageFlags = 64i32;
pub const ImageFlagsColorSpaceRGB: ImageFlags = 16i32;
pub const ImageFlagsColorSpaceYCBCR: ImageFlags = 128i32;
pub const ImageFlagsColorSpaceYCCK: ImageFlags = 256i32;
pub const ImageFlagsHasAlpha: ImageFlags = 2i32;
pub const ImageFlagsHasRealDPI: ImageFlags = 4096i32;
pub const ImageFlagsHasRealPixelSize: ImageFlags = 8192i32;
pub const ImageFlagsHasTranslucent: ImageFlags = 4i32;
pub const ImageFlagsNone: ImageFlags = 0i32;
pub const ImageFlagsPartiallyScalable: ImageFlags = 8i32;
pub const ImageFlagsReadOnly: ImageFlags = 65536i32;
pub const ImageFlagsScalable: ImageFlags = 1i32;
pub const ImageFormatBMP: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cab_0728_11d3_9d7b_0000f81ef32e);
pub const ImageFormatEMF: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cac_0728_11d3_9d7b_0000f81ef32e);
pub const ImageFormatEXIF: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cb2_0728_11d3_9d7b_0000f81ef32e);
pub const ImageFormatGIF: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cb0_0728_11d3_9d7b_0000f81ef32e);
pub const ImageFormatHEIF: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cb6_0728_11d3_9d7b_0000f81ef32e);
pub const ImageFormatIcon: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cb5_0728_11d3_9d7b_0000f81ef32e);
pub const ImageFormatJPEG: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cae_0728_11d3_9d7b_0000f81ef32e);
pub const ImageFormatMemoryBMP: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3caa_0728_11d3_9d7b_0000f81ef32e);
pub const ImageFormatPNG: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3caf_0728_11d3_9d7b_0000f81ef32e);
pub const ImageFormatTIFF: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cb1_0728_11d3_9d7b_0000f81ef32e);
pub const ImageFormatUndefined: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3ca9_0728_11d3_9d7b_0000f81ef32e);
pub const ImageFormatWEBP: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cb7_0728_11d3_9d7b_0000f81ef32e);
pub const ImageFormatWMF: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cad_0728_11d3_9d7b_0000f81ef32e);
pub const ImageLockModeRead: ImageLockMode = 1i32;
pub const ImageLockModeUserInputBuf: ImageLockMode = 4i32;
pub const ImageLockModeWrite: ImageLockMode = 2i32;
pub const ImageTypeBitmap: ImageType = 1i32;
pub const ImageTypeMetafile: ImageType = 2i32;
pub const ImageTypeUnknown: ImageType = 0i32;
pub const InsufficientBuffer: Status = 5i32;
pub const InterpolationModeBicubic: InterpolationMode = 4i32;
pub const InterpolationModeBilinear: InterpolationMode = 3i32;
pub const InterpolationModeDefault: InterpolationMode = 0i32;
pub const InterpolationModeHighQuality: InterpolationMode = 2i32;
pub const InterpolationModeHighQualityBicubic: InterpolationMode = 7i32;
pub const InterpolationModeHighQualityBilinear: InterpolationMode = 6i32;
pub const InterpolationModeInvalid: InterpolationMode = -1i32;
pub const InterpolationModeLowQuality: InterpolationMode = 1i32;
pub const InterpolationModeNearestNeighbor: InterpolationMode = 5i32;
pub const InvalidParameter: Status = 2i32;
pub const ItemDataPositionAfterBits: ItemDataPosition = 2i32;
pub const ItemDataPositionAfterHeader: ItemDataPosition = 0i32;
pub const ItemDataPositionAfterPalette: ItemDataPosition = 1i32;
pub const LevelsEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0x99c354ec_2a31_4f3a_8c34_17a803b33a25);
pub const LineCapAnchorMask: LineCap = 240i32;
pub const LineCapArrowAnchor: LineCap = 20i32;
pub const LineCapCustom: LineCap = 255i32;
pub const LineCapDiamondAnchor: LineCap = 19i32;
pub const LineCapFlat: LineCap = 0i32;
pub const LineCapNoAnchor: LineCap = 16i32;
pub const LineCapRound: LineCap = 2i32;
pub const LineCapRoundAnchor: LineCap = 18i32;
pub const LineCapSquare: LineCap = 1i32;
pub const LineCapSquareAnchor: LineCap = 17i32;
pub const LineCapTriangle: LineCap = 3i32;
pub const LineJoinBevel: LineJoin = 1i32;
pub const LineJoinMiter: LineJoin = 0i32;
pub const LineJoinMiterClipped: LineJoin = 3i32;
pub const LineJoinRound: LineJoin = 2i32;
pub const LinearGradientModeBackwardDiagonal: LinearGradientMode = 3i32;
pub const LinearGradientModeForwardDiagonal: LinearGradientMode = 2i32;
pub const LinearGradientModeHorizontal: LinearGradientMode = 0i32;
pub const LinearGradientModeVertical: LinearGradientMode = 1i32;
pub const MatrixOrderAppend: MatrixOrder = 1i32;
pub const MatrixOrderPrepend: MatrixOrder = 0i32;
pub const MetafileFrameUnitDocument: MetafileFrameUnit = 5i32;
pub const MetafileFrameUnitGdi: MetafileFrameUnit = 7i32;
pub const MetafileFrameUnitInch: MetafileFrameUnit = 4i32;
pub const MetafileFrameUnitMillimeter: MetafileFrameUnit = 6i32;
pub const MetafileFrameUnitPixel: MetafileFrameUnit = 2i32;
pub const MetafileFrameUnitPoint: MetafileFrameUnit = 3i32;
pub const MetafileTypeEmf: MetafileType = 3i32;
pub const MetafileTypeEmfPlusDual: MetafileType = 5i32;
pub const MetafileTypeEmfPlusOnly: MetafileType = 4i32;
pub const MetafileTypeInvalid: MetafileType = 0i32;
pub const MetafileTypeWmf: MetafileType = 1i32;
pub const MetafileTypeWmfPlaceable: MetafileType = 2i32;
pub const NotImplemented: Status = 6i32;
pub const NotTrueTypeFont: Status = 16i32;
pub const ObjectBusy: Status = 4i32;
pub const ObjectTypeBrush: ObjectType = 1i32;
pub const ObjectTypeCustomLineCap: ObjectType = 9i32;
pub const ObjectTypeFont: ObjectType = 6i32;
pub const ObjectTypeGraphics: ObjectType = 10i32;
pub const ObjectTypeImage: ObjectType = 5i32;
pub const ObjectTypeImageAttributes: ObjectType = 8i32;
pub const ObjectTypeInvalid: ObjectType = 0i32;
pub const ObjectTypeMax: ObjectType = 10i32;
pub const ObjectTypeMin: ObjectType = 1i32;
pub const ObjectTypePath: ObjectType = 3i32;
pub const ObjectTypePen: ObjectType = 2i32;
pub const ObjectTypeRegion: ObjectType = 4i32;
pub const ObjectTypeStringFormat: ObjectType = 7i32;
pub const Ok: Status = 0i32;
pub const OutOfMemory: Status = 3i32;
pub const PaletteFlagsGrayScale: PaletteFlags = 2i32;
pub const PaletteFlagsHalftone: PaletteFlags = 4i32;
pub const PaletteFlagsHasAlpha: PaletteFlags = 1i32;
pub const PaletteTypeCustom: PaletteType = 0i32;
pub const PaletteTypeFixedBW: PaletteType = 2i32;
pub const PaletteTypeFixedHalftone125: PaletteType = 6i32;
pub const PaletteTypeFixedHalftone216: PaletteType = 7i32;
pub const PaletteTypeFixedHalftone252: PaletteType = 8i32;
pub const PaletteTypeFixedHalftone256: PaletteType = 9i32;
pub const PaletteTypeFixedHalftone27: PaletteType = 4i32;
pub const PaletteTypeFixedHalftone64: PaletteType = 5i32;
pub const PaletteTypeFixedHalftone8: PaletteType = 3i32;
pub const PaletteTypeOptimal: PaletteType = 1i32;
pub const PathPointTypeBezier: PathPointType = 3i32;
pub const PathPointTypeBezier3: PathPointType = 3i32;
pub const PathPointTypeCloseSubpath: PathPointType = 128i32;
pub const PathPointTypeDashMode: PathPointType = 16i32;
pub const PathPointTypeLine: PathPointType = 1i32;
pub const PathPointTypePathMarker: PathPointType = 32i32;
pub const PathPointTypePathTypeMask: PathPointType = 7i32;
pub const PathPointTypeStart: PathPointType = 0i32;
pub const PenAlignmentCenter: PenAlignment = 0i32;
pub const PenAlignmentInset: PenAlignment = 1i32;
pub const PenTypeHatchFill: PenType = 1i32;
pub const PenTypeLinearGradient: PenType = 4i32;
pub const PenTypePathGradient: PenType = 3i32;
pub const PenTypeSolidColor: PenType = 0i32;
pub const PenTypeTextureFill: PenType = 2i32;
pub const PenTypeUnknown: PenType = -1i32;
pub const PixelFormatAlpha: u32 = 262144u32;
pub const PixelFormatCanonical: u32 = 2097152u32;
pub const PixelFormatDontCare: u32 = 0u32;
pub const PixelFormatExtended: u32 = 1048576u32;
pub const PixelFormatGDI: u32 = 131072u32;
pub const PixelFormatIndexed: u32 = 65536u32;
pub const PixelFormatMax: u32 = 16u32;
pub const PixelFormatPAlpha: u32 = 524288u32;
pub const PixelFormatUndefined: u32 = 0u32;
pub const PixelOffsetModeDefault: PixelOffsetMode = 0i32;
pub const PixelOffsetModeHalf: PixelOffsetMode = 4i32;
pub const PixelOffsetModeHighQuality: PixelOffsetMode = 2i32;
pub const PixelOffsetModeHighSpeed: PixelOffsetMode = 1i32;
pub const PixelOffsetModeInvalid: PixelOffsetMode = -1i32;
pub const PixelOffsetModeNone: PixelOffsetMode = 3i32;
pub const ProfileNotFound: Status = 21i32;
pub const PropertyNotFound: Status = 19i32;
pub const PropertyNotSupported: Status = 20i32;
pub const PropertyTagArtist: u32 = 315u32;
pub const PropertyTagBitsPerSample: u32 = 258u32;
pub const PropertyTagCellHeight: u32 = 265u32;
pub const PropertyTagCellWidth: u32 = 264u32;
pub const PropertyTagChrominanceTable: u32 = 20625u32;
pub const PropertyTagColorMap: u32 = 320u32;
pub const PropertyTagColorTransferFunction: u32 = 20506u32;
pub const PropertyTagCompression: u32 = 259u32;
pub const PropertyTagCopyright: u32 = 33432u32;
pub const PropertyTagDateTime: u32 = 306u32;
pub const PropertyTagDocumentName: u32 = 269u32;
pub const PropertyTagDotRange: u32 = 336u32;
pub const PropertyTagEquipMake: u32 = 271u32;
pub const PropertyTagEquipModel: u32 = 272u32;
pub const PropertyTagExifAperture: u32 = 37378u32;
pub const PropertyTagExifBrightness: u32 = 37379u32;
pub const PropertyTagExifCfaPattern: u32 = 41730u32;
pub const PropertyTagExifColorSpace: u32 = 40961u32;
pub const PropertyTagExifCompBPP: u32 = 37122u32;
pub const PropertyTagExifCompConfig: u32 = 37121u32;
pub const PropertyTagExifContrast: u32 = 41992u32;
pub const PropertyTagExifCustomRendered: u32 = 41985u32;
pub const PropertyTagExifDTDigSS: u32 = 37522u32;
pub const PropertyTagExifDTDigitized: u32 = 36868u32;
pub const PropertyTagExifDTOrig: u32 = 36867u32;
pub const PropertyTagExifDTOrigSS: u32 = 37521u32;
pub const PropertyTagExifDTSubsec: u32 = 37520u32;
pub const PropertyTagExifDeviceSettingDesc: u32 = 41995u32;
pub const PropertyTagExifDigitalZoomRatio: u32 = 41988u32;
pub const PropertyTagExifExposureBias: u32 = 37380u32;
pub const PropertyTagExifExposureIndex: u32 = 41493u32;
pub const PropertyTagExifExposureMode: u32 = 41986u32;
pub const PropertyTagExifExposureProg: u32 = 34850u32;
pub const PropertyTagExifExposureTime: u32 = 33434u32;
pub const PropertyTagExifFNumber: u32 = 33437u32;
pub const PropertyTagExifFPXVer: u32 = 40960u32;
pub const PropertyTagExifFileSource: u32 = 41728u32;
pub const PropertyTagExifFlash: u32 = 37385u32;
pub const PropertyTagExifFlashEnergy: u32 = 41483u32;
pub const PropertyTagExifFocalLength: u32 = 37386u32;
pub const PropertyTagExifFocalLengthIn35mmFilm: u32 = 41989u32;
pub const PropertyTagExifFocalResUnit: u32 = 41488u32;
pub const PropertyTagExifFocalXRes: u32 = 41486u32;
pub const PropertyTagExifFocalYRes: u32 = 41487u32;
pub const PropertyTagExifGainControl: u32 = 41991u32;
pub const PropertyTagExifIFD: u32 = 34665u32;
pub const PropertyTagExifISOSpeed: u32 = 34855u32;
pub const PropertyTagExifInterop: u32 = 40965u32;
pub const PropertyTagExifLightSource: u32 = 37384u32;
pub const PropertyTagExifMakerNote: u32 = 37500u32;
pub const PropertyTagExifMaxAperture: u32 = 37381u32;
pub const PropertyTagExifMeteringMode: u32 = 37383u32;
pub const PropertyTagExifOECF: u32 = 34856u32;
pub const PropertyTagExifPixXDim: u32 = 40962u32;
pub const PropertyTagExifPixYDim: u32 = 40963u32;
pub const PropertyTagExifRelatedWav: u32 = 40964u32;
pub const PropertyTagExifSaturation: u32 = 41993u32;
pub const PropertyTagExifSceneCaptureType: u32 = 41990u32;
pub const PropertyTagExifSceneType: u32 = 41729u32;
pub const PropertyTagExifSensingMethod: u32 = 41495u32;
pub const PropertyTagExifSharpness: u32 = 41994u32;
pub const PropertyTagExifShutterSpeed: u32 = 37377u32;
pub const PropertyTagExifSpatialFR: u32 = 41484u32;
pub const PropertyTagExifSpectralSense: u32 = 34852u32;
pub const PropertyTagExifSubjectArea: u32 = 37396u32;
pub const PropertyTagExifSubjectDist: u32 = 37382u32;
pub const PropertyTagExifSubjectDistanceRange: u32 = 41996u32;
pub const PropertyTagExifSubjectLoc: u32 = 41492u32;
pub const PropertyTagExifUniqueImageID: u32 = 42016u32;
pub const PropertyTagExifUserComment: u32 = 37510u32;
pub const PropertyTagExifVer: u32 = 36864u32;
pub const PropertyTagExifWhiteBalance: u32 = 41987u32;
pub const PropertyTagExtraSamples: u32 = 338u32;
pub const PropertyTagFillOrder: u32 = 266u32;
pub const PropertyTagFrameDelay: u32 = 20736u32;
pub const PropertyTagFreeByteCounts: u32 = 289u32;
pub const PropertyTagFreeOffset: u32 = 288u32;
pub const PropertyTagGamma: u32 = 769u32;
pub const PropertyTagGlobalPalette: u32 = 20738u32;
pub const PropertyTagGpsAltitude: u32 = 6u32;
pub const PropertyTagGpsAltitudeRef: u32 = 5u32;
pub const PropertyTagGpsAreaInformation: u32 = 28u32;
pub const PropertyTagGpsDate: u32 = 29u32;
pub const PropertyTagGpsDestBear: u32 = 24u32;
pub const PropertyTagGpsDestBearRef: u32 = 23u32;
pub const PropertyTagGpsDestDist: u32 = 26u32;
pub const PropertyTagGpsDestDistRef: u32 = 25u32;
pub const PropertyTagGpsDestLat: u32 = 20u32;
pub const PropertyTagGpsDestLatRef: u32 = 19u32;
pub const PropertyTagGpsDestLong: u32 = 22u32;
pub const PropertyTagGpsDestLongRef: u32 = 21u32;
pub const PropertyTagGpsDifferential: u32 = 30u32;
pub const PropertyTagGpsGpsDop: u32 = 11u32;
pub const PropertyTagGpsGpsMeasureMode: u32 = 10u32;
pub const PropertyTagGpsGpsSatellites: u32 = 8u32;
pub const PropertyTagGpsGpsStatus: u32 = 9u32;
pub const PropertyTagGpsGpsTime: u32 = 7u32;
pub const PropertyTagGpsIFD: u32 = 34853u32;
pub const PropertyTagGpsImgDir: u32 = 17u32;
pub const PropertyTagGpsImgDirRef: u32 = 16u32;
pub const PropertyTagGpsLatitude: u32 = 2u32;
pub const PropertyTagGpsLatitudeRef: u32 = 1u32;
pub const PropertyTagGpsLongitude: u32 = 4u32;
pub const PropertyTagGpsLongitudeRef: u32 = 3u32;
pub const PropertyTagGpsMapDatum: u32 = 18u32;
pub const PropertyTagGpsProcessingMethod: u32 = 27u32;
pub const PropertyTagGpsSpeed: u32 = 13u32;
pub const PropertyTagGpsSpeedRef: u32 = 12u32;
pub const PropertyTagGpsTrack: u32 = 15u32;
pub const PropertyTagGpsTrackRef: u32 = 14u32;
pub const PropertyTagGpsVer: u32 = 0u32;
pub const PropertyTagGrayResponseCurve: u32 = 291u32;
pub const PropertyTagGrayResponseUnit: u32 = 290u32;
pub const PropertyTagGridSize: u32 = 20497u32;
pub const PropertyTagHalftoneDegree: u32 = 20492u32;
pub const PropertyTagHalftoneHints: u32 = 321u32;
pub const PropertyTagHalftoneLPI: u32 = 20490u32;
pub const PropertyTagHalftoneLPIUnit: u32 = 20491u32;
pub const PropertyTagHalftoneMisc: u32 = 20494u32;
pub const PropertyTagHalftoneScreen: u32 = 20495u32;
pub const PropertyTagHalftoneShape: u32 = 20493u32;
pub const PropertyTagHostComputer: u32 = 316u32;
pub const PropertyTagICCProfile: u32 = 34675u32;
pub const PropertyTagICCProfileDescriptor: u32 = 770u32;
pub const PropertyTagImageDescription: u32 = 270u32;
pub const PropertyTagImageHeight: u32 = 257u32;
pub const PropertyTagImageTitle: u32 = 800u32;
pub const PropertyTagImageWidth: u32 = 256u32;
pub const PropertyTagIndexBackground: u32 = 20739u32;
pub const PropertyTagIndexTransparent: u32 = 20740u32;
pub const PropertyTagInkNames: u32 = 333u32;
pub const PropertyTagInkSet: u32 = 332u32;
pub const PropertyTagJPEGACTables: u32 = 521u32;
pub const PropertyTagJPEGDCTables: u32 = 520u32;
pub const PropertyTagJPEGInterFormat: u32 = 513u32;
pub const PropertyTagJPEGInterLength: u32 = 514u32;
pub const PropertyTagJPEGLosslessPredictors: u32 = 517u32;
pub const PropertyTagJPEGPointTransforms: u32 = 518u32;
pub const PropertyTagJPEGProc: u32 = 512u32;
pub const PropertyTagJPEGQTables: u32 = 519u32;
pub const PropertyTagJPEGQuality: u32 = 20496u32;
pub const PropertyTagJPEGRestartInterval: u32 = 515u32;
pub const PropertyTagLoopCount: u32 = 20737u32;
pub const PropertyTagLuminanceTable: u32 = 20624u32;
pub const PropertyTagMaxSampleValue: u32 = 281u32;
pub const PropertyTagMinSampleValue: u32 = 280u32;
pub const PropertyTagNewSubfileType: u32 = 254u32;
pub const PropertyTagNumberOfInks: u32 = 334u32;
pub const PropertyTagOrientation: u32 = 274u32;
pub const PropertyTagPageName: u32 = 285u32;
pub const PropertyTagPageNumber: u32 = 297u32;
pub const PropertyTagPaletteHistogram: u32 = 20755u32;
pub const PropertyTagPhotometricInterp: u32 = 262u32;
pub const PropertyTagPixelPerUnitX: u32 = 20753u32;
pub const PropertyTagPixelPerUnitY: u32 = 20754u32;
pub const PropertyTagPixelUnit: u32 = 20752u32;
pub const PropertyTagPlanarConfig: u32 = 284u32;
pub const PropertyTagPredictor: u32 = 317u32;
pub const PropertyTagPrimaryChromaticities: u32 = 319u32;
pub const PropertyTagPrintFlags: u32 = 20485u32;
pub const PropertyTagPrintFlagsBleedWidth: u32 = 20488u32;
pub const PropertyTagPrintFlagsBleedWidthScale: u32 = 20489u32;
pub const PropertyTagPrintFlagsCrop: u32 = 20487u32;
pub const PropertyTagPrintFlagsVersion: u32 = 20486u32;
pub const PropertyTagREFBlackWhite: u32 = 532u32;
pub const PropertyTagResolutionUnit: u32 = 296u32;
pub const PropertyTagResolutionXLengthUnit: u32 = 20483u32;
pub const PropertyTagResolutionXUnit: u32 = 20481u32;
pub const PropertyTagResolutionYLengthUnit: u32 = 20484u32;
pub const PropertyTagResolutionYUnit: u32 = 20482u32;
pub const PropertyTagRowsPerStrip: u32 = 278u32;
pub const PropertyTagSMaxSampleValue: u32 = 341u32;
pub const PropertyTagSMinSampleValue: u32 = 340u32;
pub const PropertyTagSRGBRenderingIntent: u32 = 771u32;
pub const PropertyTagSampleFormat: u32 = 339u32;
pub const PropertyTagSamplesPerPixel: u32 = 277u32;
pub const PropertyTagSoftwareUsed: u32 = 305u32;
pub const PropertyTagStripBytesCount: u32 = 279u32;
pub const PropertyTagStripOffsets: u32 = 273u32;
pub const PropertyTagSubfileType: u32 = 255u32;
pub const PropertyTagT4Option: u32 = 292u32;
pub const PropertyTagT6Option: u32 = 293u32;
pub const PropertyTagTargetPrinter: u32 = 337u32;
pub const PropertyTagThreshHolding: u32 = 263u32;
pub const PropertyTagThumbnailArtist: u32 = 20532u32;
pub const PropertyTagThumbnailBitsPerSample: u32 = 20514u32;
pub const PropertyTagThumbnailColorDepth: u32 = 20501u32;
pub const PropertyTagThumbnailCompressedSize: u32 = 20505u32;
pub const PropertyTagThumbnailCompression: u32 = 20515u32;
pub const PropertyTagThumbnailCopyRight: u32 = 20539u32;
pub const PropertyTagThumbnailData: u32 = 20507u32;
pub const PropertyTagThumbnailDateTime: u32 = 20531u32;
pub const PropertyTagThumbnailEquipMake: u32 = 20518u32;
pub const PropertyTagThumbnailEquipModel: u32 = 20519u32;
pub const PropertyTagThumbnailFormat: u32 = 20498u32;
pub const PropertyTagThumbnailHeight: u32 = 20500u32;
pub const PropertyTagThumbnailImageDescription: u32 = 20517u32;
pub const PropertyTagThumbnailImageHeight: u32 = 20513u32;
pub const PropertyTagThumbnailImageWidth: u32 = 20512u32;
pub const PropertyTagThumbnailOrientation: u32 = 20521u32;
pub const PropertyTagThumbnailPhotometricInterp: u32 = 20516u32;
pub const PropertyTagThumbnailPlanarConfig: u32 = 20527u32;
pub const PropertyTagThumbnailPlanes: u32 = 20502u32;
pub const PropertyTagThumbnailPrimaryChromaticities: u32 = 20534u32;
pub const PropertyTagThumbnailRawBytes: u32 = 20503u32;
pub const PropertyTagThumbnailRefBlackWhite: u32 = 20538u32;
pub const PropertyTagThumbnailResolutionUnit: u32 = 20528u32;
pub const PropertyTagThumbnailResolutionX: u32 = 20525u32;
pub const PropertyTagThumbnailResolutionY: u32 = 20526u32;
pub const PropertyTagThumbnailRowsPerStrip: u32 = 20523u32;
pub const PropertyTagThumbnailSamplesPerPixel: u32 = 20522u32;
pub const PropertyTagThumbnailSize: u32 = 20504u32;
pub const PropertyTagThumbnailSoftwareUsed: u32 = 20530u32;
pub const PropertyTagThumbnailStripBytesCount: u32 = 20524u32;
pub const PropertyTagThumbnailStripOffsets: u32 = 20520u32;
pub const PropertyTagThumbnailTransferFunction: u32 = 20529u32;
pub const PropertyTagThumbnailWhitePoint: u32 = 20533u32;
pub const PropertyTagThumbnailWidth: u32 = 20499u32;
pub const PropertyTagThumbnailYCbCrCoefficients: u32 = 20535u32;
pub const PropertyTagThumbnailYCbCrPositioning: u32 = 20537u32;
pub const PropertyTagThumbnailYCbCrSubsampling: u32 = 20536u32;
pub const PropertyTagTileByteCounts: u32 = 325u32;
pub const PropertyTagTileLength: u32 = 323u32;
pub const PropertyTagTileOffset: u32 = 324u32;
pub const PropertyTagTileWidth: u32 = 322u32;
pub const PropertyTagTransferFuncition: u32 = 301u32;
pub const PropertyTagTransferRange: u32 = 342u32;
pub const PropertyTagTypeASCII: u32 = 2u32;
pub const PropertyTagTypeByte: u32 = 1u32;
pub const PropertyTagTypeLong: u32 = 4u32;
pub const PropertyTagTypeRational: u32 = 5u32;
pub const PropertyTagTypeSLONG: u32 = 9u32;
pub const PropertyTagTypeSRational: u32 = 10u32;
pub const PropertyTagTypeShort: u32 = 3u32;
pub const PropertyTagTypeUndefined: u32 = 7u32;
pub const PropertyTagWhitePoint: u32 = 318u32;
pub const PropertyTagXPosition: u32 = 286u32;
pub const PropertyTagXResolution: u32 = 282u32;
pub const PropertyTagYCbCrCoefficients: u32 = 529u32;
pub const PropertyTagYCbCrPositioning: u32 = 531u32;
pub const PropertyTagYCbCrSubsampling: u32 = 530u32;
pub const PropertyTagYPosition: u32 = 287u32;
pub const PropertyTagYResolution: u32 = 283u32;
pub const QualityModeDefault: QualityMode = 0i32;
pub const QualityModeHigh: QualityMode = 2i32;
pub const QualityModeInvalid: QualityMode = -1i32;
pub const QualityModeLow: QualityMode = 1i32;
pub const RED_SHIFT: u32 = 16u32;
pub const RedEyeCorrectionEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0x74d29d05_69a4_4266_9549_3cc52836b632);
pub const Rotate180FlipNone: RotateFlipType = 2i32;
pub const Rotate180FlipX: RotateFlipType = 6i32;
pub const Rotate180FlipXY: RotateFlipType = 0i32;
pub const Rotate180FlipY: RotateFlipType = 4i32;
pub const Rotate270FlipNone: RotateFlipType = 3i32;
pub const Rotate270FlipX: RotateFlipType = 7i32;
pub const Rotate270FlipXY: RotateFlipType = 1i32;
pub const Rotate270FlipY: RotateFlipType = 5i32;
pub const Rotate90FlipNone: RotateFlipType = 1i32;
pub const Rotate90FlipX: RotateFlipType = 5i32;
pub const Rotate90FlipXY: RotateFlipType = 3i32;
pub const Rotate90FlipY: RotateFlipType = 7i32;
pub const RotateNoneFlipNone: RotateFlipType = 0i32;
pub const RotateNoneFlipX: RotateFlipType = 4i32;
pub const RotateNoneFlipXY: RotateFlipType = 2i32;
pub const RotateNoneFlipY: RotateFlipType = 6i32;
pub const SharpenEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0x63cbf3ee_c526_402c_8f71_62c540bf5142);
pub const SmoothingModeAntiAlias: SmoothingMode = 4i32;
pub const SmoothingModeAntiAlias8x4: SmoothingMode = 4i32;
pub const SmoothingModeAntiAlias8x8: SmoothingMode = 5i32;
pub const SmoothingModeDefault: SmoothingMode = 0i32;
pub const SmoothingModeHighQuality: SmoothingMode = 2i32;
pub const SmoothingModeHighSpeed: SmoothingMode = 1i32;
pub const SmoothingModeInvalid: SmoothingMode = -1i32;
pub const SmoothingModeNone: SmoothingMode = 3i32;
pub const StringAlignmentCenter: StringAlignment = 1i32;
pub const StringAlignmentFar: StringAlignment = 2i32;
pub const StringAlignmentNear: StringAlignment = 0i32;
pub const StringDigitSubstituteNational: StringDigitSubstitute = 2i32;
pub const StringDigitSubstituteNone: StringDigitSubstitute = 1i32;
pub const StringDigitSubstituteTraditional: StringDigitSubstitute = 3i32;
pub const StringDigitSubstituteUser: StringDigitSubstitute = 0i32;
pub const StringFormatFlagsBypassGDI: StringFormatFlags = -2147483648i32;
pub const StringFormatFlagsDirectionRightToLeft: StringFormatFlags = 1i32;
pub const StringFormatFlagsDirectionVertical: StringFormatFlags = 2i32;
pub const StringFormatFlagsDisplayFormatControl: StringFormatFlags = 32i32;
pub const StringFormatFlagsLineLimit: StringFormatFlags = 8192i32;
pub const StringFormatFlagsMeasureTrailingSpaces: StringFormatFlags = 2048i32;
pub const StringFormatFlagsNoClip: StringFormatFlags = 16384i32;
pub const StringFormatFlagsNoFitBlackBox: StringFormatFlags = 4i32;
pub const StringFormatFlagsNoFontFallback: StringFormatFlags = 1024i32;
pub const StringFormatFlagsNoWrap: StringFormatFlags = 4096i32;
pub const StringTrimmingCharacter: StringTrimming = 1i32;
pub const StringTrimmingEllipsisCharacter: StringTrimming = 3i32;
pub const StringTrimmingEllipsisPath: StringTrimming = 5i32;
pub const StringTrimmingEllipsisWord: StringTrimming = 4i32;
pub const StringTrimmingNone: StringTrimming = 0i32;
pub const StringTrimmingWord: StringTrimming = 2i32;
pub const TestControlForceBilinear: GpTestControlEnum = 0i32;
pub const TestControlGetBuildNumber: GpTestControlEnum = 2i32;
pub const TestControlNoICM: GpTestControlEnum = 1i32;
pub const TextRenderingHintAntiAlias: TextRenderingHint = 4i32;
pub const TextRenderingHintAntiAliasGridFit: TextRenderingHint = 3i32;
pub const TextRenderingHintClearTypeGridFit: TextRenderingHint = 5i32;
pub const TextRenderingHintSingleBitPerPixel: TextRenderingHint = 2i32;
pub const TextRenderingHintSingleBitPerPixelGridFit: TextRenderingHint = 1i32;
pub const TextRenderingHintSystemDefault: TextRenderingHint = 0i32;
pub const TintEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0x1077af00_2848_4441_9489_44ad4c2d7a2c);
pub const UnitDisplay: Unit = 1i32;
pub const UnitDocument: Unit = 5i32;
pub const UnitInch: Unit = 4i32;
pub const UnitMillimeter: Unit = 6i32;
pub const UnitPixel: Unit = 2i32;
pub const UnitPoint: Unit = 3i32;
pub const UnitWorld: Unit = 0i32;
pub const UnknownImageFormat: Status = 13i32;
pub const UnsupportedGdiplusVersion: Status = 17i32;
pub const ValueOverflow: Status = 11i32;
pub const WarpModeBilinear: WarpMode = 1i32;
pub const WarpModePerspective: WarpMode = 0i32;
pub const Win32Error: Status = 7i32;
pub const WmfRecordTypeAbortDoc: EmfPlusRecordType = 65618i32;
pub const WmfRecordTypeAnimatePalette: EmfPlusRecordType = 66614i32;
pub const WmfRecordTypeArc: EmfPlusRecordType = 67607i32;
pub const WmfRecordTypeBitBlt: EmfPlusRecordType = 67874i32;
pub const WmfRecordTypeChord: EmfPlusRecordType = 67632i32;
pub const WmfRecordTypeCreateBitmap: EmfPlusRecordType = 67326i32;
pub const WmfRecordTypeCreateBitmapIndirect: EmfPlusRecordType = 66301i32;
pub const WmfRecordTypeCreateBrush: EmfPlusRecordType = 65784i32;
pub const WmfRecordTypeCreateBrushIndirect: EmfPlusRecordType = 66300i32;
pub const WmfRecordTypeCreateFontIndirect: EmfPlusRecordType = 66299i32;
pub const WmfRecordTypeCreatePalette: EmfPlusRecordType = 65783i32;
pub const WmfRecordTypeCreatePatternBrush: EmfPlusRecordType = 66041i32;
pub const WmfRecordTypeCreatePenIndirect: EmfPlusRecordType = 66298i32;
pub const WmfRecordTypeCreateRegion: EmfPlusRecordType = 67327i32;
pub const WmfRecordTypeDIBBitBlt: EmfPlusRecordType = 67904i32;
pub const WmfRecordTypeDIBCreatePatternBrush: EmfPlusRecordType = 65858i32;
pub const WmfRecordTypeDIBStretchBlt: EmfPlusRecordType = 68417i32;
pub const WmfRecordTypeDeleteObject: EmfPlusRecordType = 66032i32;
pub const WmfRecordTypeDrawText: EmfPlusRecordType = 67119i32;
pub const WmfRecordTypeEllipse: EmfPlusRecordType = 66584i32;
pub const WmfRecordTypeEndDoc: EmfPlusRecordType = 65630i32;
pub const WmfRecordTypeEndPage: EmfPlusRecordType = 65616i32;
pub const WmfRecordTypeEscape: EmfPlusRecordType = 67110i32;
pub const WmfRecordTypeExcludeClipRect: EmfPlusRecordType = 66581i32;
pub const WmfRecordTypeExtFloodFill: EmfPlusRecordType = 66888i32;
pub const WmfRecordTypeExtTextOut: EmfPlusRecordType = 68146i32;
pub const WmfRecordTypeFillRegion: EmfPlusRecordType = 66088i32;
pub const WmfRecordTypeFloodFill: EmfPlusRecordType = 66585i32;
pub const WmfRecordTypeFrameRegion: EmfPlusRecordType = 66601i32;
pub const WmfRecordTypeIntersectClipRect: EmfPlusRecordType = 66582i32;
pub const WmfRecordTypeInvertRegion: EmfPlusRecordType = 65834i32;
pub const WmfRecordTypeLineTo: EmfPlusRecordType = 66067i32;
pub const WmfRecordTypeMoveTo: EmfPlusRecordType = 66068i32;
pub const WmfRecordTypeOffsetClipRgn: EmfPlusRecordType = 66080i32;
pub const WmfRecordTypeOffsetViewportOrg: EmfPlusRecordType = 66065i32;
pub const WmfRecordTypeOffsetWindowOrg: EmfPlusRecordType = 66063i32;
pub const WmfRecordTypePaintRegion: EmfPlusRecordType = 65835i32;
pub const WmfRecordTypePatBlt: EmfPlusRecordType = 67101i32;
pub const WmfRecordTypePie: EmfPlusRecordType = 67610i32;
pub const WmfRecordTypePolyPolygon: EmfPlusRecordType = 66872i32;
pub const WmfRecordTypePolygon: EmfPlusRecordType = 66340i32;
pub const WmfRecordTypePolyline: EmfPlusRecordType = 66341i32;
pub const WmfRecordTypeRealizePalette: EmfPlusRecordType = 65589i32;
pub const WmfRecordTypeRectangle: EmfPlusRecordType = 66587i32;
pub const WmfRecordTypeResetDC: EmfPlusRecordType = 65868i32;
pub const WmfRecordTypeResizePalette: EmfPlusRecordType = 65849i32;
pub const WmfRecordTypeRestoreDC: EmfPlusRecordType = 65831i32;
pub const WmfRecordTypeRoundRect: EmfPlusRecordType = 67100i32;
pub const WmfRecordTypeSaveDC: EmfPlusRecordType = 65566i32;
pub const WmfRecordTypeScaleViewportExt: EmfPlusRecordType = 66578i32;
pub const WmfRecordTypeScaleWindowExt: EmfPlusRecordType = 66576i32;
pub const WmfRecordTypeSelectClipRegion: EmfPlusRecordType = 65836i32;
pub const WmfRecordTypeSelectObject: EmfPlusRecordType = 65837i32;
pub const WmfRecordTypeSelectPalette: EmfPlusRecordType = 66100i32;
pub const WmfRecordTypeSetBkColor: EmfPlusRecordType = 66049i32;
pub const WmfRecordTypeSetBkMode: EmfPlusRecordType = 65794i32;
pub const WmfRecordTypeSetDIBToDev: EmfPlusRecordType = 68915i32;
pub const WmfRecordTypeSetLayout: EmfPlusRecordType = 65865i32;
pub const WmfRecordTypeSetMapMode: EmfPlusRecordType = 65795i32;
pub const WmfRecordTypeSetMapperFlags: EmfPlusRecordType = 66097i32;
pub const WmfRecordTypeSetPalEntries: EmfPlusRecordType = 65591i32;
pub const WmfRecordTypeSetPixel: EmfPlusRecordType = 66591i32;
pub const WmfRecordTypeSetPolyFillMode: EmfPlusRecordType = 65798i32;
pub const WmfRecordTypeSetROP2: EmfPlusRecordType = 65796i32;
pub const WmfRecordTypeSetRelAbs: EmfPlusRecordType = 65797i32;
pub const WmfRecordTypeSetStretchBltMode: EmfPlusRecordType = 65799i32;
pub const WmfRecordTypeSetTextAlign: EmfPlusRecordType = 65838i32;
pub const WmfRecordTypeSetTextCharExtra: EmfPlusRecordType = 65800i32;
pub const WmfRecordTypeSetTextColor: EmfPlusRecordType = 66057i32;
pub const WmfRecordTypeSetTextJustification: EmfPlusRecordType = 66058i32;
pub const WmfRecordTypeSetViewportExt: EmfPlusRecordType = 66062i32;
pub const WmfRecordTypeSetViewportOrg: EmfPlusRecordType = 66061i32;
pub const WmfRecordTypeSetWindowExt: EmfPlusRecordType = 66060i32;
pub const WmfRecordTypeSetWindowOrg: EmfPlusRecordType = 66059i32;
pub const WmfRecordTypeStartDoc: EmfPlusRecordType = 65869i32;
pub const WmfRecordTypeStartPage: EmfPlusRecordType = 65615i32;
pub const WmfRecordTypeStretchBlt: EmfPlusRecordType = 68387i32;
pub const WmfRecordTypeStretchDIB: EmfPlusRecordType = 69443i32;
pub const WmfRecordTypeTextOut: EmfPlusRecordType = 66849i32;
pub const WrapModeClamp: WrapMode = 4i32;
pub const WrapModeTile: WrapMode = 0i32;
pub const WrapModeTileFlipX: WrapMode = 1i32;
pub const WrapModeTileFlipXY: WrapMode = 3i32;
pub const WrapModeTileFlipY: WrapMode = 2i32;
pub const WrongState: Status = 8i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BrushType(pub i32);
impl windows_core::TypeKind for BrushType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ColorAdjustType(pub i32);
impl windows_core::TypeKind for ColorAdjustType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ColorChannelFlags(pub i32);
impl windows_core::TypeKind for ColorChannelFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ColorMatrixFlags(pub i32);
impl windows_core::TypeKind for ColorMatrixFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ColorMode(pub i32);
impl windows_core::TypeKind for ColorMode {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CombineMode(pub i32);
impl windows_core::TypeKind for CombineMode {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CompositingMode(pub i32);
impl windows_core::TypeKind for CompositingMode {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CompositingQuality(pub i32);
impl windows_core::TypeKind for CompositingQuality {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ConvertToEmfPlusFlags(pub i32);
impl windows_core::TypeKind for ConvertToEmfPlusFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CoordinateSpace(pub i32);
impl windows_core::TypeKind for CoordinateSpace {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CurveAdjustments(pub i32);
impl windows_core::TypeKind for CurveAdjustments {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CurveChannel(pub i32);
impl windows_core::TypeKind for CurveChannel {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CustomLineCapType(pub i32);
impl windows_core::TypeKind for CustomLineCapType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DashCap(pub i32);
impl windows_core::TypeKind for DashCap {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DashStyle(pub i32);
impl windows_core::TypeKind for DashStyle {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DebugEventLevel(pub i32);
impl windows_core::TypeKind for DebugEventLevel {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DitherType(pub i32);
impl windows_core::TypeKind for DitherType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DriverStringOptions(pub i32);
impl windows_core::TypeKind for DriverStringOptions {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EmfPlusRecordType(pub i32);
impl windows_core::TypeKind for EmfPlusRecordType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EmfToWmfBitsFlags(pub i32);
impl windows_core::TypeKind for EmfToWmfBitsFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EmfType(pub i32);
impl windows_core::TypeKind for EmfType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EncoderParameterValueType(pub i32);
impl windows_core::TypeKind for EncoderParameterValueType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EncoderValue(pub i32);
impl windows_core::TypeKind for EncoderValue {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FillMode(pub i32);
impl windows_core::TypeKind for FillMode {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FlushIntention(pub i32);
impl windows_core::TypeKind for FlushIntention {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FontStyle(pub i32);
impl windows_core::TypeKind for FontStyle {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GdiplusStartupParams(pub i32);
impl windows_core::TypeKind for GdiplusStartupParams {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GenericFontFamily(pub i32);
impl windows_core::TypeKind for GenericFontFamily {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GpTestControlEnum(pub i32);
impl windows_core::TypeKind for GpTestControlEnum {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HatchStyle(pub i32);
impl windows_core::TypeKind for HatchStyle {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HistogramFormat(pub i32);
impl windows_core::TypeKind for HistogramFormat {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HotkeyPrefix(pub i32);
impl windows_core::TypeKind for HotkeyPrefix {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ImageCodecFlags(pub i32);
impl windows_core::TypeKind for ImageCodecFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ImageFlags(pub i32);
impl windows_core::TypeKind for ImageFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ImageLockMode(pub i32);
impl windows_core::TypeKind for ImageLockMode {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ImageType(pub i32);
impl windows_core::TypeKind for ImageType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InterpolationMode(pub i32);
impl windows_core::TypeKind for InterpolationMode {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ItemDataPosition(pub i32);
impl windows_core::TypeKind for ItemDataPosition {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LineCap(pub i32);
impl windows_core::TypeKind for LineCap {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LineJoin(pub i32);
impl windows_core::TypeKind for LineJoin {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LinearGradientMode(pub i32);
impl windows_core::TypeKind for LinearGradientMode {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MatrixOrder(pub i32);
impl windows_core::TypeKind for MatrixOrder {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MetafileFrameUnit(pub i32);
impl windows_core::TypeKind for MetafileFrameUnit {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MetafileType(pub i32);
impl windows_core::TypeKind for MetafileType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ObjectType(pub i32);
impl windows_core::TypeKind for ObjectType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PaletteFlags(pub i32);
impl windows_core::TypeKind for PaletteFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PaletteType(pub i32);
impl windows_core::TypeKind for PaletteType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PathPointType(pub i32);
impl windows_core::TypeKind for PathPointType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PenAlignment(pub i32);
impl windows_core::TypeKind for PenAlignment {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PenType(pub i32);
impl windows_core::TypeKind for PenType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PixelOffsetMode(pub i32);
impl windows_core::TypeKind for PixelOffsetMode {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct QualityMode(pub i32);
impl windows_core::TypeKind for QualityMode {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RotateFlipType(pub i32);
impl windows_core::TypeKind for RotateFlipType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SmoothingMode(pub i32);
impl windows_core::TypeKind for SmoothingMode {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Status(pub i32);
impl windows_core::TypeKind for Status {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StringAlignment(pub i32);
impl windows_core::TypeKind for StringAlignment {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StringDigitSubstitute(pub i32);
impl windows_core::TypeKind for StringDigitSubstitute {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StringFormatFlags(pub i32);
impl windows_core::TypeKind for StringFormatFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StringTrimming(pub i32);
impl windows_core::TypeKind for StringTrimming {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TextRenderingHint(pub i32);
impl windows_core::TypeKind for TextRenderingHint {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Unit(pub i32);
impl windows_core::TypeKind for Unit {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WarpMode(pub i32);
impl windows_core::TypeKind for WarpMode {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WrapMode(pub i32);
impl windows_core::TypeKind for WrapMode {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BitmapData {
    pub Width: u32,
    pub Height: u32,
    pub Stride: i32,
    pub PixelFormat: i32,
    pub Scan0: *mut core::ffi::c_void,
    pub Reserved: usize,
}
impl Default for BitmapData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BitmapData {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Blur {
    pub Base: Effect,
}
impl Default for Blur {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for Blur {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BlurParams {
    pub radius: f32,
    pub expandEdge: super::super::Foundation::BOOL,
}
impl Default for BlurParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BlurParams {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BrightnessContrast {
    pub Base: Effect,
}
impl Default for BrightnessContrast {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BrightnessContrast {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BrightnessContrastParams {
    pub brightnessLevel: i32,
    pub contrastLevel: i32,
}
impl Default for BrightnessContrastParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BrightnessContrastParams {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CharacterRange {
    pub First: i32,
    pub Length: i32,
}
impl Default for CharacterRange {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CharacterRange {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub Argb: u32,
}
impl Color {
    pub const AliceBlue: i32 = -984833i32;
    pub const AntiqueWhite: i32 = -332841i32;
    pub const Aqua: i32 = -16711681i32;
    pub const Aquamarine: i32 = -8388652i32;
    pub const Azure: i32 = -983041i32;
    pub const Beige: i32 = -657956i32;
    pub const Bisque: i32 = -6972i32;
    pub const Black: i32 = -16777216i32;
    pub const BlanchedAlmond: i32 = -5171i32;
    pub const Blue: i32 = -16776961i32;
    pub const BlueViolet: i32 = -7722014i32;
    pub const Brown: i32 = -5952982i32;
    pub const BurlyWood: i32 = -2180985i32;
    pub const CadetBlue: i32 = -10510688i32;
    pub const Chartreuse: i32 = -8388864i32;
    pub const Chocolate: i32 = -2987746i32;
    pub const Coral: i32 = -32944i32;
    pub const CornflowerBlue: i32 = -10185235i32;
    pub const Cornsilk: i32 = -1828i32;
    pub const Crimson: i32 = -2354116i32;
    pub const Cyan: i32 = -16711681i32;
    pub const DarkBlue: i32 = -16777077i32;
    pub const DarkCyan: i32 = -16741493i32;
    pub const DarkGoldenrod: i32 = -4684277i32;
    pub const DarkGray: i32 = -5658199i32;
    pub const DarkGreen: i32 = -16751616i32;
    pub const DarkKhaki: i32 = -4343957i32;
    pub const DarkMagenta: i32 = -7667573i32;
    pub const DarkOliveGreen: i32 = -11179217i32;
    pub const DarkOrange: i32 = -29696i32;
    pub const DarkOrchid: i32 = -6737204i32;
    pub const DarkRed: i32 = -7667712i32;
    pub const DarkSalmon: i32 = -1468806i32;
    pub const DarkSeaGreen: i32 = -7357301i32;
    pub const DarkSlateBlue: i32 = -12042869i32;
    pub const DarkSlateGray: i32 = -13676721i32;
    pub const DarkTurquoise: i32 = -16724271i32;
    pub const DarkViolet: i32 = -7077677i32;
    pub const DeepPink: i32 = -60269i32;
    pub const DeepSkyBlue: i32 = -16728065i32;
    pub const DimGray: i32 = -9868951i32;
    pub const DodgerBlue: i32 = -14774017i32;
    pub const Firebrick: i32 = -5103070i32;
    pub const FloralWhite: i32 = -1296i32;
    pub const ForestGreen: i32 = -14513374i32;
    pub const Fuchsia: i32 = -65281i32;
    pub const Gainsboro: i32 = -2302756i32;
    pub const GhostWhite: i32 = -460545i32;
    pub const Gold: i32 = -10496i32;
    pub const Goldenrod: i32 = -2448096i32;
    pub const Gray: i32 = -8355712i32;
    pub const Green: i32 = -16744448i32;
    pub const GreenYellow: i32 = -5374161i32;
    pub const Honeydew: i32 = -983056i32;
    pub const HotPink: i32 = -38476i32;
    pub const IndianRed: i32 = -3318692i32;
    pub const Indigo: i32 = -11861886i32;
    pub const Ivory: i32 = -16i32;
    pub const Khaki: i32 = -989556i32;
    pub const Lavender: i32 = -1644806i32;
    pub const LavenderBlush: i32 = -3851i32;
    pub const LawnGreen: i32 = -8586240i32;
    pub const LemonChiffon: i32 = -1331i32;
    pub const LightBlue: i32 = -5383962i32;
    pub const LightCoral: i32 = -1015680i32;
    pub const LightCyan: i32 = -2031617i32;
    pub const LightGoldenrodYellow: i32 = -329006i32;
    pub const LightGray: i32 = -2894893i32;
    pub const LightGreen: i32 = -7278960i32;
    pub const LightPink: i32 = -18751i32;
    pub const LightSalmon: i32 = -24454i32;
    pub const LightSeaGreen: i32 = -14634326i32;
    pub const LightSkyBlue: i32 = -7876870i32;
    pub const LightSlateGray: i32 = -8943463i32;
    pub const LightSteelBlue: i32 = -5192482i32;
    pub const LightYellow: i32 = -32i32;
    pub const Lime: i32 = -16711936i32;
    pub const LimeGreen: i32 = -13447886i32;
    pub const Linen: i32 = -331546i32;
    pub const Magenta: i32 = -65281i32;
    pub const Maroon: i32 = -8388608i32;
    pub const MediumAquamarine: i32 = -10039894i32;
    pub const MediumBlue: i32 = -16777011i32;
    pub const MediumOrchid: i32 = -4565549i32;
    pub const MediumPurple: i32 = -7114533i32;
    pub const MediumSeaGreen: i32 = -12799119i32;
    pub const MediumSlateBlue: i32 = -8689426i32;
    pub const MediumSpringGreen: i32 = -16713062i32;
    pub const MediumTurquoise: i32 = -12004916i32;
    pub const MediumVioletRed: i32 = -3730043i32;
    pub const MidnightBlue: i32 = -15132304i32;
    pub const MintCream: i32 = -655366i32;
    pub const MistyRose: i32 = -6943i32;
    pub const Moccasin: i32 = -6987i32;
    pub const NavajoWhite: i32 = -8531i32;
    pub const Navy: i32 = -16777088i32;
    pub const OldLace: i32 = -133658i32;
    pub const Olive: i32 = -8355840i32;
    pub const OliveDrab: i32 = -9728477i32;
    pub const Orange: i32 = -23296i32;
    pub const OrangeRed: i32 = -47872i32;
    pub const Orchid: i32 = -2461482i32;
    pub const PaleGoldenrod: i32 = -1120086i32;
    pub const PaleGreen: i32 = -6751336i32;
    pub const PaleTurquoise: i32 = -5247250i32;
    pub const PaleVioletRed: i32 = -2396013i32;
    pub const PapayaWhip: i32 = -4139i32;
    pub const PeachPuff: i32 = -9543i32;
    pub const Peru: i32 = -3308225i32;
    pub const Pink: i32 = -16181i32;
    pub const Plum: i32 = -2252579i32;
    pub const PowderBlue: i32 = -5185306i32;
    pub const Purple: i32 = -8388480i32;
    pub const Red: i32 = -65536i32;
    pub const RosyBrown: i32 = -4419697i32;
    pub const RoyalBlue: i32 = -12490271i32;
    pub const SaddleBrown: i32 = -7650029i32;
    pub const Salmon: i32 = -360334i32;
    pub const SandyBrown: i32 = -744352i32;
    pub const SeaGreen: i32 = -13726889i32;
    pub const SeaShell: i32 = -2578i32;
    pub const Sienna: i32 = -6270419i32;
    pub const Silver: i32 = -4144960i32;
    pub const SkyBlue: i32 = -7876885i32;
    pub const SlateBlue: i32 = -9807155i32;
    pub const SlateGray: i32 = -9404272i32;
    pub const Snow: i32 = -1286i32;
    pub const SpringGreen: i32 = -16711809i32;
    pub const SteelBlue: i32 = -12156236i32;
    pub const Tan: i32 = -2968436i32;
    pub const Teal: i32 = -16744320i32;
    pub const Thistle: i32 = -2572328i32;
    pub const Tomato: i32 = -40121i32;
    pub const Transparent: i32 = 16777215i32;
    pub const Turquoise: i32 = -12525360i32;
    pub const Violet: i32 = -1146130i32;
    pub const Wheat: i32 = -663885i32;
    pub const White: i32 = -1i32;
    pub const WhiteSmoke: i32 = -657931i32;
    pub const Yellow: i32 = -256i32;
    pub const YellowGreen: i32 = -6632142i32;
    pub const AlphaShift: i32 = 24i32;
    pub const RedShift: i32 = 16i32;
    pub const GreenShift: i32 = 8i32;
    pub const BlueShift: i32 = 0i32;
    pub const AlphaMask: i32 = -16777216i32;
    pub const RedMask: i32 = 16711680i32;
    pub const GreenMask: i32 = 65280i32;
    pub const BlueMask: i32 = 255i32;
}
impl Default for Color {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for Color {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorBalance {
    pub Base: Effect,
}
impl Default for ColorBalance {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ColorBalance {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorBalanceParams {
    pub cyanRed: i32,
    pub magentaGreen: i32,
    pub yellowBlue: i32,
}
impl Default for ColorBalanceParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ColorBalanceParams {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorCurve {
    pub Base: Effect,
}
impl Default for ColorCurve {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ColorCurve {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorCurveParams {
    pub adjustment: CurveAdjustments,
    pub channel: CurveChannel,
    pub adjustValue: i32,
}
impl Default for ColorCurveParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ColorCurveParams {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorLUT {
    pub Base: Effect,
}
impl Default for ColorLUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ColorLUT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorLUTParams {
    pub lutB: [u8; 256],
    pub lutG: [u8; 256],
    pub lutR: [u8; 256],
    pub lutA: [u8; 256],
}
impl Default for ColorLUTParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ColorLUTParams {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorMap {
    pub oldColor: Color,
    pub newColor: Color,
}
impl Default for ColorMap {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ColorMap {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorMatrix {
    pub m: [f32; 25],
}
impl Default for ColorMatrix {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ColorMatrix {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorMatrixEffect {
    pub Base: Effect,
}
impl Default for ColorMatrixEffect {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ColorMatrixEffect {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorPalette {
    pub Flags: u32,
    pub Count: u32,
    pub Entries: [u32; 1],
}
impl Default for ColorPalette {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ColorPalette {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENHMETAHEADER3 {
    pub iType: u32,
    pub nSize: u32,
    pub rclBounds: super::super::Foundation::RECTL,
    pub rclFrame: super::super::Foundation::RECTL,
    pub dSignature: u32,
    pub nVersion: u32,
    pub nBytes: u32,
    pub nRecords: u32,
    pub nHandles: u16,
    pub sReserved: u16,
    pub nDescription: u32,
    pub offDescription: u32,
    pub nPalEntries: u32,
    pub szlDevice: super::super::Foundation::SIZE,
    pub szlMillimeters: super::super::Foundation::SIZE,
}
impl Default for ENHMETAHEADER3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ENHMETAHEADER3 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Effect {
    pub lpVtbl: *mut *mut core::ffi::c_void,
    pub nativeEffect: *mut CGpEffect,
    pub auxDataSize: i32,
    pub auxData: *mut core::ffi::c_void,
    pub useAuxData: super::super::Foundation::BOOL,
}
impl Default for Effect {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for Effect {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EncoderParameter {
    pub Guid: windows_core::GUID,
    pub NumberOfValues: u32,
    pub Type: u32,
    pub Value: *mut core::ffi::c_void,
}
impl Default for EncoderParameter {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for EncoderParameter {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EncoderParameters {
    pub Count: u32,
    pub Parameter: [EncoderParameter; 1],
}
impl Default for EncoderParameters {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for EncoderParameters {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GdiplusStartupInput {
    pub GdiplusVersion: u32,
    pub DebugEventCallback: isize,
    pub SuppressBackgroundThread: super::super::Foundation::BOOL,
    pub SuppressExternalCodecs: super::super::Foundation::BOOL,
}
impl Default for GdiplusStartupInput {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GdiplusStartupInput {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GdiplusStartupInputEx {
    pub Base: GdiplusStartupInput,
    pub StartupParameters: i32,
}
impl Default for GdiplusStartupInputEx {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GdiplusStartupInputEx {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GdiplusStartupOutput {
    pub NotificationHook: isize,
    pub NotificationUnhook: isize,
}
impl Default for GdiplusStartupOutput {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GdiplusStartupOutput {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpAdjustableArrowCap(pub u8);
impl Default for GpAdjustableArrowCap {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpAdjustableArrowCap {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpBitmap(pub u8);
impl Default for GpBitmap {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpBitmap {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpBrush(pub u8);
impl Default for GpBrush {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpBrush {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpCachedBitmap(pub u8);
impl Default for GpCachedBitmap {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpCachedBitmap {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpCustomLineCap(pub u8);
impl Default for GpCustomLineCap {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpCustomLineCap {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpFont(pub u8);
impl Default for GpFont {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpFont {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpFontCollection(pub u8);
impl Default for GpFontCollection {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpFontCollection {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpFontFamily(pub u8);
impl Default for GpFontFamily {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpFontFamily {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpGraphics(pub u8);
impl Default for GpGraphics {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpGraphics {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpHatch(pub u8);
impl Default for GpHatch {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpHatch {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpImage(pub u8);
impl Default for GpImage {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpImage {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpImageAttributes(pub u8);
impl Default for GpImageAttributes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpImageAttributes {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpInstalledFontCollection(pub u8);
impl Default for GpInstalledFontCollection {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpInstalledFontCollection {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpLineGradient(pub u8);
impl Default for GpLineGradient {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpLineGradient {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpMetafile(pub u8);
impl Default for GpMetafile {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpMetafile {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpPath(pub u8);
impl Default for GpPath {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpPath {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpPathGradient(pub u8);
impl Default for GpPathGradient {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpPathGradient {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpPathIterator(pub u8);
impl Default for GpPathIterator {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpPathIterator {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpPen(pub u8);
impl Default for GpPen {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpPen {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpPrivateFontCollection(pub u8);
impl Default for GpPrivateFontCollection {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpPrivateFontCollection {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpRegion(pub u8);
impl Default for GpRegion {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpRegion {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpSolidFill(pub u8);
impl Default for GpSolidFill {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpSolidFill {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpStringFormat(pub u8);
impl Default for GpStringFormat {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpStringFormat {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpTexture(pub u8);
impl Default for GpTexture {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpTexture {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HueSaturationLightness {
    pub Base: Effect,
}
impl Default for HueSaturationLightness {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HueSaturationLightness {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HueSaturationLightnessParams {
    pub hueLevel: i32,
    pub saturationLevel: i32,
    pub lightnessLevel: i32,
}
impl Default for HueSaturationLightnessParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HueSaturationLightnessParams {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImageCodecInfo {
    pub Clsid: windows_core::GUID,
    pub FormatID: windows_core::GUID,
    pub CodecName: windows_core::PCWSTR,
    pub DllName: windows_core::PCWSTR,
    pub FormatDescription: windows_core::PCWSTR,
    pub FilenameExtension: windows_core::PCWSTR,
    pub MimeType: windows_core::PCWSTR,
    pub Flags: u32,
    pub Version: u32,
    pub SigCount: u32,
    pub SigSize: u32,
    pub SigPattern: *const u8,
    pub SigMask: *const u8,
}
impl Default for ImageCodecInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ImageCodecInfo {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImageItemData {
    pub Size: u32,
    pub Position: u32,
    pub Desc: *mut core::ffi::c_void,
    pub DescSize: u32,
    pub Data: *mut core::ffi::c_void,
    pub DataSize: u32,
    pub Cookie: u32,
}
impl Default for ImageItemData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ImageItemData {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Levels {
    pub Base: Effect,
}
impl Default for Levels {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for Levels {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LevelsParams {
    pub highlight: i32,
    pub midtone: i32,
    pub shadow: i32,
}
impl Default for LevelsParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for LevelsParams {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MetafileHeader {
    pub Type: MetafileType,
    pub Size: u32,
    pub Version: u32,
    pub EmfPlusFlags: u32,
    pub DpiX: f32,
    pub DpiY: f32,
    pub X: i32,
    pub Y: i32,
    pub Width: i32,
    pub Height: i32,
    pub Anonymous: MetafileHeader_0,
    pub EmfPlusHeaderSize: i32,
    pub LogicalDpiX: i32,
    pub LogicalDpiY: i32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for MetafileHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for MetafileHeader {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union MetafileHeader_0 {
    pub WmfHeader: super::Gdi::METAHEADER,
    pub EmfHeader: ENHMETAHEADER3,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for MetafileHeader_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for MetafileHeader_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PWMFRect16 {
    pub Left: i16,
    pub Top: i16,
    pub Right: i16,
    pub Bottom: i16,
}
impl Default for PWMFRect16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PWMFRect16 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub X: i32,
    pub Y: i32,
}
impl Default for Point {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for Point {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointF {
    pub X: f32,
    pub Y: f32,
}
impl Default for PointF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PointF {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PropertyItem {
    pub id: u32,
    pub length: u32,
    pub r#type: u16,
    pub value: *mut core::ffi::c_void,
}
impl Default for PropertyItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PropertyItem {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub X: i32,
    pub Y: i32,
    pub Width: i32,
    pub Height: i32,
}
impl Default for Rect {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for Rect {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RectF {
    pub X: f32,
    pub Y: f32,
    pub Width: f32,
    pub Height: f32,
}
impl Default for RectF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RectF {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RedEyeCorrection {
    pub Base: Effect,
}
impl Default for RedEyeCorrection {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RedEyeCorrection {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RedEyeCorrectionParams {
    pub numberOfAreas: u32,
    pub areas: *mut super::super::Foundation::RECT,
}
impl Default for RedEyeCorrectionParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RedEyeCorrectionParams {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sharpen {
    pub Base: Effect,
}
impl Default for Sharpen {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for Sharpen {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SharpenParams {
    pub radius: f32,
    pub amount: f32,
}
impl Default for SharpenParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SharpenParams {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size {
    pub Width: i32,
    pub Height: i32,
}
impl Default for Size {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for Size {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SizeF {
    pub Width: f32,
    pub Height: f32,
}
impl Default for SizeF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SizeF {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tint {
    pub Base: Effect,
}
impl Default for Tint {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for Tint {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TintParams {
    pub hue: i32,
    pub amount: i32,
}
impl Default for TintParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for TintParams {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WmfPlaceableFileHeader {
    pub Key: u32,
    pub Hmf: i16,
    pub BoundingBox: PWMFRect16,
    pub Inch: i16,
    pub Reserved: u32,
    pub Checksum: i16,
}
impl Default for WmfPlaceableFileHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for WmfPlaceableFileHeader {
    type TypeKind = windows_core::CloneType;
}
pub type DebugEventProc = Option<unsafe extern "system" fn(level: DebugEventLevel, message: windows_core::PCSTR)>;
pub type DrawImageAbort = Option<unsafe extern "system" fn() -> super::super::Foundation::BOOL>;
pub type EnumerateMetafileProc = Option<unsafe extern "system" fn(param0: EmfPlusRecordType, param1: u32, param2: u32, param3: *const u8, param4: *mut core::ffi::c_void) -> super::super::Foundation::BOOL>;
pub type GetThumbnailImageAbort = Option<unsafe extern "system" fn() -> super::super::Foundation::BOOL>;
pub type ImageAbort = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void) -> super::super::Foundation::BOOL>;
pub type NotificationHookProc = Option<unsafe extern "system" fn(token: *mut usize) -> Status>;
pub type NotificationUnhookProc = Option<unsafe extern "system" fn(token: usize)>;
