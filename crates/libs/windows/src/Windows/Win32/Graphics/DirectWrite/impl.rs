pub trait IDWriteAsyncResultImpl: Sized {
    fn GetWaitHandle();
    fn GetResult();
}
pub trait IDWriteBitmapRenderTargetImpl: Sized {
    fn DrawGlyphRun();
    fn GetMemoryDC();
    fn GetPixelsPerDip();
    fn SetPixelsPerDip();
    fn GetCurrentTransform();
    fn SetCurrentTransform();
    fn GetSize();
    fn Resize();
}
pub trait IDWriteBitmapRenderTarget1Impl: Sized + IDWriteBitmapRenderTargetImpl {
    fn GetTextAntialiasMode();
    fn SetTextAntialiasMode();
}
pub trait IDWriteColorGlyphRunEnumeratorImpl: Sized {
    fn MoveNext();
    fn GetCurrentRun();
}
pub trait IDWriteColorGlyphRunEnumerator1Impl: Sized + IDWriteColorGlyphRunEnumeratorImpl {
    fn GetCurrentRun();
}
pub trait IDWriteFactoryImpl: Sized {
    fn GetSystemFontCollection();
    fn CreateCustomFontCollection();
    fn RegisterFontCollectionLoader();
    fn UnregisterFontCollectionLoader();
    fn CreateFontFileReference();
    fn CreateCustomFontFileReference();
    fn CreateFontFace();
    fn CreateRenderingParams();
    fn CreateMonitorRenderingParams();
    fn CreateCustomRenderingParams();
    fn RegisterFontFileLoader();
    fn UnregisterFontFileLoader();
    fn CreateTextFormat();
    fn CreateTypography();
    fn GetGdiInterop();
    fn CreateTextLayout();
    fn CreateGdiCompatibleTextLayout();
    fn CreateEllipsisTrimmingSign();
    fn CreateTextAnalyzer();
    fn CreateNumberSubstitution();
    fn CreateGlyphRunAnalysis();
}
pub trait IDWriteFactory1Impl: Sized + IDWriteFactoryImpl {
    fn GetEudcFontCollection();
    fn CreateCustomRenderingParams();
}
pub trait IDWriteFactory2Impl: Sized + IDWriteFactory1Impl + IDWriteFactoryImpl {
    fn GetSystemFontFallback();
    fn CreateFontFallbackBuilder();
    fn TranslateColorGlyphRun();
    fn CreateCustomRenderingParams();
    fn CreateGlyphRunAnalysis();
}
pub trait IDWriteFactory3Impl: Sized + IDWriteFactory2Impl + IDWriteFactory1Impl + IDWriteFactoryImpl {
    fn CreateGlyphRunAnalysis();
    fn CreateCustomRenderingParams();
    fn CreateFontFaceReference();
    fn CreateFontFaceReference();
    fn GetSystemFontSet();
    fn CreateFontSetBuilder();
    fn CreateFontCollectionFromFontSet();
    fn GetSystemFontCollection();
    fn GetFontDownloadQueue();
}
pub trait IDWriteFactory4Impl: Sized + IDWriteFactory3Impl + IDWriteFactory2Impl + IDWriteFactory1Impl + IDWriteFactoryImpl {
    fn TranslateColorGlyphRun();
    fn ComputeGlyphOrigins();
    fn ComputeGlyphOrigins();
}
pub trait IDWriteFactory5Impl: Sized + IDWriteFactory4Impl + IDWriteFactory3Impl + IDWriteFactory2Impl + IDWriteFactory1Impl + IDWriteFactoryImpl {
    fn CreateFontSetBuilder();
    fn CreateInMemoryFontFileLoader();
    fn CreateHttpFontFileLoader();
    fn AnalyzeContainerType();
    fn UnpackFontFile();
}
pub trait IDWriteFactory6Impl: Sized + IDWriteFactory5Impl + IDWriteFactory4Impl + IDWriteFactory3Impl + IDWriteFactory2Impl + IDWriteFactory1Impl + IDWriteFactoryImpl {
    fn CreateFontFaceReference();
    fn CreateFontResource();
    fn GetSystemFontSet();
    fn GetSystemFontCollection();
    fn CreateFontCollectionFromFontSet();
    fn CreateFontSetBuilder();
    fn CreateTextFormat();
}
pub trait IDWriteFactory7Impl: Sized + IDWriteFactory6Impl + IDWriteFactory5Impl + IDWriteFactory4Impl + IDWriteFactory3Impl + IDWriteFactory2Impl + IDWriteFactory1Impl + IDWriteFactoryImpl {
    fn GetSystemFontSet();
    fn GetSystemFontCollection();
}
pub trait IDWriteFontImpl: Sized {
    fn GetFontFamily();
    fn GetWeight();
    fn GetStretch();
    fn GetStyle();
    fn IsSymbolFont();
    fn GetFaceNames();
    fn GetInformationalStrings();
    fn GetSimulations();
    fn GetMetrics();
    fn HasCharacter();
    fn CreateFontFace();
}
pub trait IDWriteFont1Impl: Sized + IDWriteFontImpl {
    fn GetMetrics();
    fn GetPanose();
    fn GetUnicodeRanges();
    fn IsMonospacedFont();
}
pub trait IDWriteFont2Impl: Sized + IDWriteFont1Impl + IDWriteFontImpl {
    fn IsColorFont();
}
pub trait IDWriteFont3Impl: Sized + IDWriteFont2Impl + IDWriteFont1Impl + IDWriteFontImpl {
    fn CreateFontFace();
    fn Equals();
    fn GetFontFaceReference();
    fn HasCharacter();
    fn GetLocality();
}
pub trait IDWriteFontCollectionImpl: Sized {
    fn GetFontFamilyCount();
    fn GetFontFamily();
    fn FindFamilyName();
    fn GetFontFromFontFace();
}
pub trait IDWriteFontCollection1Impl: Sized + IDWriteFontCollectionImpl {
    fn GetFontSet();
    fn GetFontFamily();
}
pub trait IDWriteFontCollection2Impl: Sized + IDWriteFontCollection1Impl + IDWriteFontCollectionImpl {
    fn GetFontFamily();
    fn GetMatchingFonts();
    fn GetFontFamilyModel();
    fn GetFontSet();
}
pub trait IDWriteFontCollection3Impl: Sized + IDWriteFontCollection2Impl + IDWriteFontCollection1Impl + IDWriteFontCollectionImpl {
    fn GetExpirationEvent();
}
pub trait IDWriteFontCollectionLoaderImpl: Sized {
    fn CreateEnumeratorFromKey();
}
pub trait IDWriteFontDownloadListenerImpl: Sized {
    fn DownloadCompleted();
}
pub trait IDWriteFontDownloadQueueImpl: Sized {
    fn AddListener();
    fn RemoveListener();
    fn IsEmpty();
    fn BeginDownload();
    fn CancelDownload();
    fn GetGenerationCount();
}
pub trait IDWriteFontFaceImpl: Sized {
    fn GetType();
    fn GetFiles();
    fn GetIndex();
    fn GetSimulations();
    fn IsSymbolFont();
    fn GetMetrics();
    fn GetGlyphCount();
    fn GetDesignGlyphMetrics();
    fn GetGlyphIndices();
    fn TryGetFontTable();
    fn ReleaseFontTable();
    fn GetGlyphRunOutline();
    fn GetRecommendedRenderingMode();
    fn GetGdiCompatibleMetrics();
    fn GetGdiCompatibleGlyphMetrics();
}
pub trait IDWriteFontFace1Impl: Sized + IDWriteFontFaceImpl {
    fn GetMetrics();
    fn GetGdiCompatibleMetrics();
    fn GetCaretMetrics();
    fn GetUnicodeRanges();
    fn IsMonospacedFont();
    fn GetDesignGlyphAdvances();
    fn GetGdiCompatibleGlyphAdvances();
    fn GetKerningPairAdjustments();
    fn HasKerningPairs();
    fn GetRecommendedRenderingMode();
    fn GetVerticalGlyphVariants();
    fn HasVerticalGlyphVariants();
}
pub trait IDWriteFontFace2Impl: Sized + IDWriteFontFace1Impl + IDWriteFontFaceImpl {
    fn IsColorFont();
    fn GetColorPaletteCount();
    fn GetPaletteEntryCount();
    fn GetPaletteEntries();
    fn GetRecommendedRenderingMode();
}
pub trait IDWriteFontFace3Impl: Sized + IDWriteFontFace2Impl + IDWriteFontFace1Impl + IDWriteFontFaceImpl {
    fn GetFontFaceReference();
    fn GetPanose();
    fn GetWeight();
    fn GetStretch();
    fn GetStyle();
    fn GetFamilyNames();
    fn GetFaceNames();
    fn GetInformationalStrings();
    fn HasCharacter();
    fn GetRecommendedRenderingMode();
    fn IsCharacterLocal();
    fn IsGlyphLocal();
    fn AreCharactersLocal();
    fn AreGlyphsLocal();
}
pub trait IDWriteFontFace4Impl: Sized + IDWriteFontFace3Impl + IDWriteFontFace2Impl + IDWriteFontFace1Impl + IDWriteFontFaceImpl {
    fn GetGlyphImageFormats();
    fn GetGlyphImageFormats();
    fn GetGlyphImageData();
    fn ReleaseGlyphImageData();
}
pub trait IDWriteFontFace5Impl: Sized + IDWriteFontFace4Impl + IDWriteFontFace3Impl + IDWriteFontFace2Impl + IDWriteFontFace1Impl + IDWriteFontFaceImpl {
    fn GetFontAxisValueCount();
    fn GetFontAxisValues();
    fn HasVariations();
    fn GetFontResource();
    fn Equals();
}
pub trait IDWriteFontFace6Impl: Sized + IDWriteFontFace5Impl + IDWriteFontFace4Impl + IDWriteFontFace3Impl + IDWriteFontFace2Impl + IDWriteFontFace1Impl + IDWriteFontFaceImpl {
    fn GetFamilyNames();
    fn GetFaceNames();
}
pub trait IDWriteFontFaceReferenceImpl: Sized {
    fn CreateFontFace();
    fn CreateFontFaceWithSimulations();
    fn Equals();
    fn GetFontFaceIndex();
    fn GetSimulations();
    fn GetFontFile();
    fn GetLocalFileSize();
    fn GetFileSize();
    fn GetFileTime();
    fn GetLocality();
    fn EnqueueFontDownloadRequest();
    fn EnqueueCharacterDownloadRequest();
    fn EnqueueGlyphDownloadRequest();
    fn EnqueueFileFragmentDownloadRequest();
}
pub trait IDWriteFontFaceReference1Impl: Sized + IDWriteFontFaceReferenceImpl {
    fn CreateFontFace();
    fn GetFontAxisValueCount();
    fn GetFontAxisValues();
}
pub trait IDWriteFontFallbackImpl: Sized {
    fn MapCharacters();
}
pub trait IDWriteFontFallback1Impl: Sized + IDWriteFontFallbackImpl {
    fn MapCharacters();
}
pub trait IDWriteFontFallbackBuilderImpl: Sized {
    fn AddMapping();
    fn AddMappings();
    fn CreateFontFallback();
}
pub trait IDWriteFontFamilyImpl: Sized + IDWriteFontListImpl {
    fn GetFamilyNames();
    fn GetFirstMatchingFont();
    fn GetMatchingFonts();
}
pub trait IDWriteFontFamily1Impl: Sized + IDWriteFontFamilyImpl + IDWriteFontListImpl {
    fn GetFontLocality();
    fn GetFont();
    fn GetFontFaceReference();
}
pub trait IDWriteFontFamily2Impl: Sized + IDWriteFontFamily1Impl + IDWriteFontFamilyImpl + IDWriteFontListImpl {
    fn GetMatchingFonts();
    fn GetFontSet();
}
pub trait IDWriteFontFileImpl: Sized {
    fn GetReferenceKey();
    fn GetLoader();
    fn Analyze();
}
pub trait IDWriteFontFileEnumeratorImpl: Sized {
    fn MoveNext();
    fn GetCurrentFontFile();
}
pub trait IDWriteFontFileLoaderImpl: Sized {
    fn CreateStreamFromKey();
}
pub trait IDWriteFontFileStreamImpl: Sized {
    fn ReadFileFragment();
    fn ReleaseFileFragment();
    fn GetFileSize();
    fn GetLastWriteTime();
}
pub trait IDWriteFontListImpl: Sized {
    fn GetFontCollection();
    fn GetFontCount();
    fn GetFont();
}
pub trait IDWriteFontList1Impl: Sized + IDWriteFontListImpl {
    fn GetFontLocality();
    fn GetFont();
    fn GetFontFaceReference();
}
pub trait IDWriteFontList2Impl: Sized + IDWriteFontList1Impl + IDWriteFontListImpl {
    fn GetFontSet();
}
pub trait IDWriteFontResourceImpl: Sized {
    fn GetFontFile();
    fn GetFontFaceIndex();
    fn GetFontAxisCount();
    fn GetDefaultFontAxisValues();
    fn GetFontAxisRanges();
    fn GetFontAxisAttributes();
    fn GetAxisNames();
    fn GetAxisValueNameCount();
    fn GetAxisValueNames();
    fn HasVariations();
    fn CreateFontFace();
    fn CreateFontFaceReference();
}
pub trait IDWriteFontSetImpl: Sized {
    fn GetFontCount();
    fn GetFontFaceReference();
    fn FindFontFaceReference();
    fn FindFontFace();
    fn GetPropertyValues();
    fn GetPropertyValues();
    fn GetPropertyValues();
    fn GetPropertyOccurrenceCount();
    fn GetMatchingFonts();
    fn GetMatchingFonts();
}
pub trait IDWriteFontSet1Impl: Sized + IDWriteFontSetImpl {
    fn GetMatchingFonts();
    fn GetFirstFontResources();
    fn GetFilteredFonts();
    fn GetFilteredFonts();
    fn GetFilteredFonts();
    fn GetFilteredFontIndices();
    fn GetFilteredFontIndices();
    fn GetFontAxisRanges();
    fn GetFontAxisRanges();
    fn GetFontFaceReference();
    fn CreateFontResource();
    fn CreateFontFace();
    fn GetFontLocality();
}
pub trait IDWriteFontSet2Impl: Sized + IDWriteFontSet1Impl + IDWriteFontSetImpl {
    fn GetExpirationEvent();
}
pub trait IDWriteFontSet3Impl: Sized + IDWriteFontSet2Impl + IDWriteFontSet1Impl + IDWriteFontSetImpl {
    fn GetFontSourceType();
    fn GetFontSourceNameLength();
    fn GetFontSourceName();
}
pub trait IDWriteFontSetBuilderImpl: Sized {
    fn AddFontFaceReference();
    fn AddFontFaceReference();
    fn AddFontSet();
    fn CreateFontSet();
}
pub trait IDWriteFontSetBuilder1Impl: Sized + IDWriteFontSetBuilderImpl {
    fn AddFontFile();
}
pub trait IDWriteFontSetBuilder2Impl: Sized + IDWriteFontSetBuilder1Impl + IDWriteFontSetBuilderImpl {
    fn AddFont();
    fn AddFontFile();
}
pub trait IDWriteGdiInteropImpl: Sized {
    fn CreateFontFromLOGFONT();
    fn ConvertFontToLOGFONT();
    fn ConvertFontFaceToLOGFONT();
    fn CreateFontFaceFromHdc();
    fn CreateBitmapRenderTarget();
}
pub trait IDWriteGdiInterop1Impl: Sized + IDWriteGdiInteropImpl {
    fn CreateFontFromLOGFONT();
    fn GetFontSignature();
    fn GetFontSignature();
    fn GetMatchingFontsByLOGFONT();
}
pub trait IDWriteGlyphRunAnalysisImpl: Sized {
    fn GetAlphaTextureBounds();
    fn CreateAlphaTexture();
    fn GetAlphaBlendParams();
}
pub trait IDWriteInMemoryFontFileLoaderImpl: Sized + IDWriteFontFileLoaderImpl {
    fn CreateInMemoryFontFileReference();
    fn GetFileCount();
}
pub trait IDWriteInlineObjectImpl: Sized {
    fn Draw();
    fn GetMetrics();
    fn GetOverhangMetrics();
    fn GetBreakConditions();
}
pub trait IDWriteLocalFontFileLoaderImpl: Sized + IDWriteFontFileLoaderImpl {
    fn GetFilePathLengthFromKey();
    fn GetFilePathFromKey();
    fn GetLastWriteTimeFromKey();
}
pub trait IDWriteLocalizedStringsImpl: Sized {
    fn GetCount();
    fn FindLocaleName();
    fn GetLocaleNameLength();
    fn GetLocaleName();
    fn GetStringLength();
    fn GetString();
}
pub trait IDWriteNumberSubstitutionImpl: Sized {}
pub trait IDWritePixelSnappingImpl: Sized {
    fn IsPixelSnappingDisabled();
    fn GetCurrentTransform();
    fn GetPixelsPerDip();
}
pub trait IDWriteRemoteFontFileLoaderImpl: Sized + IDWriteFontFileLoaderImpl {
    fn CreateRemoteStreamFromKey();
    fn GetLocalityFromKey();
    fn CreateFontFileReferenceFromUrl();
}
pub trait IDWriteRemoteFontFileStreamImpl: Sized + IDWriteFontFileStreamImpl {
    fn GetLocalFileSize();
    fn GetFileFragmentLocality();
    fn GetLocality();
    fn BeginDownload();
}
pub trait IDWriteRenderingParamsImpl: Sized {
    fn GetGamma();
    fn GetEnhancedContrast();
    fn GetClearTypeLevel();
    fn GetPixelGeometry();
    fn GetRenderingMode();
}
pub trait IDWriteRenderingParams1Impl: Sized + IDWriteRenderingParamsImpl {
    fn GetGrayscaleEnhancedContrast();
}
pub trait IDWriteRenderingParams2Impl: Sized + IDWriteRenderingParams1Impl + IDWriteRenderingParamsImpl {
    fn GetGridFitMode();
}
pub trait IDWriteRenderingParams3Impl: Sized + IDWriteRenderingParams2Impl + IDWriteRenderingParams1Impl + IDWriteRenderingParamsImpl {
    fn GetRenderingMode1();
}
pub trait IDWriteStringListImpl: Sized {
    fn GetCount();
    fn GetLocaleNameLength();
    fn GetLocaleName();
    fn GetStringLength();
    fn GetString();
}
pub trait IDWriteTextAnalysisSinkImpl: Sized {
    fn SetScriptAnalysis();
    fn SetLineBreakpoints();
    fn SetBidiLevel();
    fn SetNumberSubstitution();
}
pub trait IDWriteTextAnalysisSink1Impl: Sized + IDWriteTextAnalysisSinkImpl {
    fn SetGlyphOrientation();
}
pub trait IDWriteTextAnalysisSourceImpl: Sized {
    fn GetTextAtPosition();
    fn GetTextBeforePosition();
    fn GetParagraphReadingDirection();
    fn GetLocaleName();
    fn GetNumberSubstitution();
}
pub trait IDWriteTextAnalysisSource1Impl: Sized + IDWriteTextAnalysisSourceImpl {
    fn GetVerticalGlyphOrientation();
}
pub trait IDWriteTextAnalyzerImpl: Sized {
    fn AnalyzeScript();
    fn AnalyzeBidi();
    fn AnalyzeNumberSubstitution();
    fn AnalyzeLineBreakpoints();
    fn GetGlyphs();
    fn GetGlyphPlacements();
    fn GetGdiCompatibleGlyphPlacements();
}
pub trait IDWriteTextAnalyzer1Impl: Sized + IDWriteTextAnalyzerImpl {
    fn ApplyCharacterSpacing();
    fn GetBaseline();
    fn AnalyzeVerticalGlyphOrientation();
    fn GetGlyphOrientationTransform();
    fn GetScriptProperties();
    fn GetTextComplexity();
    fn GetJustificationOpportunities();
    fn JustifyGlyphAdvances();
    fn GetJustifiedGlyphs();
}
pub trait IDWriteTextAnalyzer2Impl: Sized + IDWriteTextAnalyzer1Impl + IDWriteTextAnalyzerImpl {
    fn GetGlyphOrientationTransform();
    fn GetTypographicFeatures();
    fn CheckTypographicFeature();
}
pub trait IDWriteTextFormatImpl: Sized {
    fn SetTextAlignment();
    fn SetParagraphAlignment();
    fn SetWordWrapping();
    fn SetReadingDirection();
    fn SetFlowDirection();
    fn SetIncrementalTabStop();
    fn SetTrimming();
    fn SetLineSpacing();
    fn GetTextAlignment();
    fn GetParagraphAlignment();
    fn GetWordWrapping();
    fn GetReadingDirection();
    fn GetFlowDirection();
    fn GetIncrementalTabStop();
    fn GetTrimming();
    fn GetLineSpacing();
    fn GetFontCollection();
    fn GetFontFamilyNameLength();
    fn GetFontFamilyName();
    fn GetFontWeight();
    fn GetFontStyle();
    fn GetFontStretch();
    fn GetFontSize();
    fn GetLocaleNameLength();
    fn GetLocaleName();
}
pub trait IDWriteTextFormat1Impl: Sized + IDWriteTextFormatImpl {
    fn SetVerticalGlyphOrientation();
    fn GetVerticalGlyphOrientation();
    fn SetLastLineWrapping();
    fn GetLastLineWrapping();
    fn SetOpticalAlignment();
    fn GetOpticalAlignment();
    fn SetFontFallback();
    fn GetFontFallback();
}
pub trait IDWriteTextFormat2Impl: Sized + IDWriteTextFormat1Impl + IDWriteTextFormatImpl {
    fn SetLineSpacing();
    fn GetLineSpacing();
}
pub trait IDWriteTextFormat3Impl: Sized + IDWriteTextFormat2Impl + IDWriteTextFormat1Impl + IDWriteTextFormatImpl {
    fn SetFontAxisValues();
    fn GetFontAxisValueCount();
    fn GetFontAxisValues();
    fn GetAutomaticFontAxes();
    fn SetAutomaticFontAxes();
}
pub trait IDWriteTextLayoutImpl: Sized + IDWriteTextFormatImpl {
    fn SetMaxWidth();
    fn SetMaxHeight();
    fn SetFontCollection();
    fn SetFontFamilyName();
    fn SetFontWeight();
    fn SetFontStyle();
    fn SetFontStretch();
    fn SetFontSize();
    fn SetUnderline();
    fn SetStrikethrough();
    fn SetDrawingEffect();
    fn SetInlineObject();
    fn SetTypography();
    fn SetLocaleName();
    fn GetMaxWidth();
    fn GetMaxHeight();
    fn GetFontCollection();
    fn GetFontFamilyNameLength();
    fn GetFontFamilyName();
    fn GetFontWeight();
    fn GetFontStyle();
    fn GetFontStretch();
    fn GetFontSize();
    fn GetUnderline();
    fn GetStrikethrough();
    fn GetDrawingEffect();
    fn GetInlineObject();
    fn GetTypography();
    fn GetLocaleNameLength();
    fn GetLocaleName();
    fn Draw();
    fn GetLineMetrics();
    fn GetMetrics();
    fn GetOverhangMetrics();
    fn GetClusterMetrics();
    fn DetermineMinWidth();
    fn HitTestPoint();
    fn HitTestTextPosition();
    fn HitTestTextRange();
}
pub trait IDWriteTextLayout1Impl: Sized + IDWriteTextLayoutImpl + IDWriteTextFormatImpl {
    fn SetPairKerning();
    fn GetPairKerning();
    fn SetCharacterSpacing();
    fn GetCharacterSpacing();
}
pub trait IDWriteTextLayout2Impl: Sized + IDWriteTextLayout1Impl + IDWriteTextLayoutImpl + IDWriteTextFormatImpl {
    fn GetMetrics();
    fn SetVerticalGlyphOrientation();
    fn GetVerticalGlyphOrientation();
    fn SetLastLineWrapping();
    fn GetLastLineWrapping();
    fn SetOpticalAlignment();
    fn GetOpticalAlignment();
    fn SetFontFallback();
    fn GetFontFallback();
}
pub trait IDWriteTextLayout3Impl: Sized + IDWriteTextLayout2Impl + IDWriteTextLayout1Impl + IDWriteTextLayoutImpl + IDWriteTextFormatImpl {
    fn InvalidateLayout();
    fn SetLineSpacing();
    fn GetLineSpacing();
    fn GetLineMetrics();
}
pub trait IDWriteTextLayout4Impl: Sized + IDWriteTextLayout3Impl + IDWriteTextLayout2Impl + IDWriteTextLayout1Impl + IDWriteTextLayoutImpl + IDWriteTextFormatImpl {
    fn SetFontAxisValues();
    fn GetFontAxisValueCount();
    fn GetFontAxisValues();
    fn GetAutomaticFontAxes();
    fn SetAutomaticFontAxes();
}
pub trait IDWriteTextRendererImpl: Sized + IDWritePixelSnappingImpl {
    fn DrawGlyphRun();
    fn DrawUnderline();
    fn DrawStrikethrough();
    fn DrawInlineObject();
}
pub trait IDWriteTextRenderer1Impl: Sized + IDWriteTextRendererImpl + IDWritePixelSnappingImpl {
    fn DrawGlyphRun();
    fn DrawUnderline();
    fn DrawStrikethrough();
    fn DrawInlineObject();
}
pub trait IDWriteTypographyImpl: Sized {
    fn AddFontFeature();
    fn GetFontFeatureCount();
    fn GetFontFeature();
}
