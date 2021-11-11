#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AbortPath();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddFontMemResourceEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddFontResourceA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddFontResourceExA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddFontResourceExW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddFontResourceW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AlphaBlend();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AngleArc();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AnimatePalette();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Arc();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ArcTo();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BeginPaint();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BeginPath();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BitBlt();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CancelDC();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeDisplaySettingsA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeDisplaySettingsExA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeDisplaySettingsExW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeDisplaySettingsW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Chord();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClientToScreen();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CloseEnhMetaFile();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseFigure();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CloseMetaFile();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CombineRgn();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CombineTransform();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyEnhMetaFileA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyEnhMetaFileW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyMetaFileA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyMetaFileW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyRect();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateBitmap();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateBitmapIndirect();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateBrushIndirect();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateCompatibleBitmap();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateCompatibleDC();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDCA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDCW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateDIBPatternBrush();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateDIBPatternBrushPt();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDIBSection();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateDIBitmap();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateDiscardableBitmap();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateEllipticRgn();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateEllipticRgnIndirect();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateEnhMetaFileA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateEnhMetaFileW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateFontA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateFontIndirectA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateFontIndirectExA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateFontIndirectExW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateFontIndirectW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateFontPackage();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateFontW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateHalftonePalette();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateHatchBrush();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateICA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateICW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateMetaFileA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateMetaFileW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreatePalette();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreatePatternBrush();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreatePen();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePenIndirect();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePolyPolygonRgn();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePolygonRgn();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateRectRgn();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateRectRgnIndirect();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateRoundRectRgn();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateScalableFontResourceA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateScalableFontResourceW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn CreateSolidBrush();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPtoLP();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteDC();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteEnhMetaFile();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteMetaFile();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteObject();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawAnimatedRects();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawCaption();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawEdge();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawEscape();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawFocusRect();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawFrameControl();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawStateA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawStateW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawTextA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawTextExA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawTextExW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawTextW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Ellipse();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndPaint();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndPath();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplayDevicesA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplayDevicesW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplayMonitors();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplaySettingsA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplaySettingsExA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplaySettingsExW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDisplaySettingsW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumEnhMetaFile();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFontFamiliesA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFontFamiliesExA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFontFamiliesExW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFontFamiliesW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFontsA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumFontsW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumMetaFile();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumObjects();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EqualRect();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EqualRgn();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn ExcludeClipRect();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExcludeUpdateRgn();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn ExtCreatePen();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtCreateRegion();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtFloodFill();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn ExtSelectClipRgn();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtTextOutA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtTextOutW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FillPath();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FillRect();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FillRgn();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FixBrushOrgEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlattenPath();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FloodFill();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FrameRect();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FrameRgn();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiAlphaBlend();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiComment();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiFlush();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GdiGetBatchLimit();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiGradientFill();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GdiSetBatchLimit();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GdiTransparentBlt();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetArcDirection();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAspectRatioFilterEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetBitmapBits();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetBitmapDimensionEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetBkColor();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetBkMode();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetBoundsRect();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetBrushOrgEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharABCWidthsA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharABCWidthsFloatA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharABCWidthsFloatW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharABCWidthsI();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharABCWidthsW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidth32A();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidth32W();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidthA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidthFloatA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidthFloatW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidthI();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharWidthW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharacterPlacementA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCharacterPlacementW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClipBox();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetClipRgn();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetColorAdjustment();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetCurrentObject();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentPositionEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDC();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetDCBrushColor();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDCEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDCOrgEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetDCPenColor();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetDIBColorTable();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetDIBits();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetDeviceCaps();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnhMetaFileA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetEnhMetaFileBits();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnhMetaFileDescriptionA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnhMetaFileDescriptionW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnhMetaFileHeader();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetEnhMetaFilePaletteEntries();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEnhMetaFileW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetFontData();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetFontLanguageInfo();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetFontUnicodeRanges();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGlyphIndicesA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGlyphIndicesW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGlyphOutlineA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGlyphOutlineW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetGraphicsMode();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetKerningPairsA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetKerningPairsW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetLayout();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetMapMode();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMetaFileA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetMetaFileBitsEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMetaFileW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetMetaRgn();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMiterLimit();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorInfoA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMonitorInfoW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetNearestColor();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetNearestPaletteIndex();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetObjectA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetObjectType();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetObjectW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOutlineTextMetricsA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOutlineTextMetricsW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetPaletteEntries();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPath();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetPixel();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetPolyFillMode();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetROP2();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetRandomRgn();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRasterizerCaps();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRegionData();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRgnBox();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetStockObject();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetStretchBltMode();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetSysColorBrush();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetSystemPaletteEntries();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetSystemPaletteUse();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTabbedTextExtentA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTabbedTextExtentW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetTextAlign();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetTextCharacterExtra();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetTextColor();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentExPointA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentExPointI();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentExPointW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentPoint32A();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentPoint32W();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentPointA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentPointI();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextExtentPointW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextFaceA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextFaceW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextMetricsA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTextMetricsW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUpdateRect();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUpdateRgn();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetViewportExtEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetViewportOrgEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn GetWinMetaFileBits();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowDC();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowExtEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowOrgEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowRgn();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowRgnBox();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWorldTransform();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GradientFill();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GrayStringA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GrayStringW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InflateRect();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn IntersectClipRect();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IntersectRect();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InvalidateRect();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InvalidateRgn();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InvertRect();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InvertRgn();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsRectEmpty();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LPtoDP();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LineDDA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LineTo();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadBitmapA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadBitmapW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LockWindowUpdate();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapWindowPoints();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MaskBlt();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn MergeFontPackage();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ModifyWorldTransform();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MonitorFromPoint();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MonitorFromRect();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MonitorFromWindow();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveToEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn OffsetClipRgn();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OffsetRect();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn OffsetRgn();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OffsetViewportOrgEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OffsetWindowOrgEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PaintDesktop();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PaintRgn();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PatBlt();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn PathToRegion();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Pie();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlayEnhMetaFile();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlayEnhMetaFileRecord();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlayMetaFile();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlayMetaFileRecord();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlgBlt();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyBezier();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyBezierTo();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyDraw();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyPolygon();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyPolyline();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyTextOutA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolyTextOutW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Polygon();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Polyline();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PolylineTo();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PtInRect();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PtInRegion();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PtVisible();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn RealizePalette();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RectInRegion();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RectVisible();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Rectangle();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RedrawWindow();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReleaseDC();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveFontMemResourceEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveFontResourceA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveFontResourceExA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveFontResourceExW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveFontResourceW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResetDCA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResetDCW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResizePalette();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RestoreDC();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RoundRect();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SaveDC();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScaleViewportExtEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScaleWindowExtEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScreenToClient();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SelectClipPath();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SelectClipRgn();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SelectObject();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SelectPalette();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetArcDirection();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetBitmapBits();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetBitmapDimensionEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetBkColor();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetBkMode();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetBoundsRect();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetBrushOrgEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetColorAdjustment();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetDCBrushColor();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetDCPenColor();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetDIBColorTable();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetDIBits();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetDIBitsToDevice();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetEnhMetaFileBits();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetGraphicsMode();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetLayout();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetMapMode();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetMapperFlags();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetMetaFileBitsEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetMetaRgn();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMiterLimit();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetPaletteEntries();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetPixel();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPixelV();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetPolyFillMode();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetROP2();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetRect();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetRectEmpty();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetRectRgn();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetStretchBltMode();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetSystemPaletteUse();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetTextAlign();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetTextCharacterExtra();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn SetTextColor();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTextJustification();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetViewportExtEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetViewportOrgEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowExtEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowOrgEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowRgn();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWorldTransform();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StretchBlt();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn StretchDIBits();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrokeAndFillPath();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrokePath();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SubtractRect();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn TTCharToUnicode();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TTDeleteEmbeddedFont();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn TTEmbedFont();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn TTEmbedFontEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TTEmbedFontFromFileA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TTEnableEmbeddingForFacename();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn TTGetEmbeddedFontInfo();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn TTGetEmbeddingType();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TTGetNewFontName();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TTIsEmbeddingEnabled();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TTIsEmbeddingEnabledForFacename();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TTLoadEmbeddedFont();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn TTRunValidationTests();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn TTRunValidationTestsEx();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TabbedTextOutA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TabbedTextOutW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TextOutA();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TextOutW();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TransparentBlt();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnionRect();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnrealizeObject();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateColors();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateWindow();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ValidateRect();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ValidateRgn();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WidenPath();
    #[doc = "*Required features: `Win32_Graphics_Gdi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowFromDC();
    #[doc = "*Required features: `Win32_Graphics_Gdi`*"]
    pub fn wglSwapMultipleBuffers();
}
