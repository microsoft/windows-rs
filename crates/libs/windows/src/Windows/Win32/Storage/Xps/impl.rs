pub trait IXpsDocumentPackageTargetImpl: Sized {
    fn GetXpsOMPackageWriter();
    fn GetXpsOMFactory();
    fn GetXpsType();
}
pub trait IXpsDocumentPackageTarget3DImpl: Sized {
    fn GetXpsOMPackageWriter3D();
    fn GetXpsOMFactory();
}
pub trait IXpsOMBrushImpl: Sized + IXpsOMShareableImpl {
    fn GetOpacity();
    fn SetOpacity();
}
pub trait IXpsOMCanvasImpl: Sized + IXpsOMVisualImpl + IXpsOMShareableImpl {
    fn GetVisuals();
    fn GetUseAliasedEdgeMode();
    fn SetUseAliasedEdgeMode();
    fn GetAccessibilityShortDescription();
    fn SetAccessibilityShortDescription();
    fn GetAccessibilityLongDescription();
    fn SetAccessibilityLongDescription();
    fn GetDictionary();
    fn GetDictionaryLocal();
    fn SetDictionaryLocal();
    fn GetDictionaryResource();
    fn SetDictionaryResource();
    fn Clone();
}
pub trait IXpsOMColorProfileResourceImpl: Sized + IXpsOMResourceImpl + IXpsOMPartImpl {
    fn GetStream();
    fn SetContent();
}
pub trait IXpsOMColorProfileResourceCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Append();
    fn GetByPartName();
}
pub trait IXpsOMCorePropertiesImpl: Sized + IXpsOMPartImpl {
    fn GetOwner();
    fn GetCategory();
    fn SetCategory();
    fn GetContentStatus();
    fn SetContentStatus();
    fn GetContentType();
    fn SetContentType();
    fn GetCreated();
    fn SetCreated();
    fn GetCreator();
    fn SetCreator();
    fn GetDescription();
    fn SetDescription();
    fn GetIdentifier();
    fn SetIdentifier();
    fn GetKeywords();
    fn SetKeywords();
    fn GetLanguage();
    fn SetLanguage();
    fn GetLastModifiedBy();
    fn SetLastModifiedBy();
    fn GetLastPrinted();
    fn SetLastPrinted();
    fn GetModified();
    fn SetModified();
    fn GetRevision();
    fn SetRevision();
    fn GetSubject();
    fn SetSubject();
    fn GetTitle();
    fn SetTitle();
    fn GetVersion();
    fn SetVersion();
    fn Clone();
}
pub trait IXpsOMDashCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Append();
}
pub trait IXpsOMDictionaryImpl: Sized {
    fn GetOwner();
    fn GetCount();
    fn GetAt();
    fn GetByKey();
    fn GetIndex();
    fn Append();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Clone();
}
pub trait IXpsOMDocumentImpl: Sized + IXpsOMPartImpl {
    fn GetOwner();
    fn GetPageReferences();
    fn GetPrintTicketResource();
    fn SetPrintTicketResource();
    fn GetDocumentStructureResource();
    fn SetDocumentStructureResource();
    fn GetSignatureBlockResources();
    fn Clone();
}
pub trait IXpsOMDocumentCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Append();
}
pub trait IXpsOMDocumentSequenceImpl: Sized + IXpsOMPartImpl {
    fn GetOwner();
    fn GetDocuments();
    fn GetPrintTicketResource();
    fn SetPrintTicketResource();
}
pub trait IXpsOMDocumentStructureResourceImpl: Sized + IXpsOMResourceImpl + IXpsOMPartImpl {
    fn GetOwner();
    fn GetStream();
    fn SetContent();
}
pub trait IXpsOMFontResourceImpl: Sized + IXpsOMResourceImpl + IXpsOMPartImpl {
    fn GetStream();
    fn SetContent();
    fn GetEmbeddingOption();
}
pub trait IXpsOMFontResourceCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn SetAt();
    fn InsertAt();
    fn Append();
    fn RemoveAt();
    fn GetByPartName();
}
pub trait IXpsOMGeometryImpl: Sized + IXpsOMShareableImpl {
    fn GetFigures();
    fn GetFillRule();
    fn SetFillRule();
    fn GetTransform();
    fn GetTransformLocal();
    fn SetTransformLocal();
    fn GetTransformLookup();
    fn SetTransformLookup();
    fn Clone();
}
pub trait IXpsOMGeometryFigureImpl: Sized {
    fn GetOwner();
    fn GetSegmentData();
    fn GetSegmentTypes();
    fn GetSegmentStrokes();
    fn SetSegments();
    fn GetStartPoint();
    fn SetStartPoint();
    fn GetIsClosed();
    fn SetIsClosed();
    fn GetIsFilled();
    fn SetIsFilled();
    fn GetSegmentCount();
    fn GetSegmentDataCount();
    fn GetSegmentStrokePattern();
    fn Clone();
}
pub trait IXpsOMGeometryFigureCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Append();
}
pub trait IXpsOMGlyphsImpl: Sized + IXpsOMVisualImpl + IXpsOMShareableImpl {
    fn GetUnicodeString();
    fn GetGlyphIndexCount();
    fn GetGlyphIndices();
    fn GetGlyphMappingCount();
    fn GetGlyphMappings();
    fn GetProhibitedCaretStopCount();
    fn GetProhibitedCaretStops();
    fn GetBidiLevel();
    fn GetIsSideways();
    fn GetDeviceFontName();
    fn GetStyleSimulations();
    fn SetStyleSimulations();
    fn GetOrigin();
    fn SetOrigin();
    fn GetFontRenderingEmSize();
    fn SetFontRenderingEmSize();
    fn GetFontResource();
    fn SetFontResource();
    fn GetFontFaceIndex();
    fn SetFontFaceIndex();
    fn GetFillBrush();
    fn GetFillBrushLocal();
    fn SetFillBrushLocal();
    fn GetFillBrushLookup();
    fn SetFillBrushLookup();
    fn GetGlyphsEditor();
    fn Clone();
}
pub trait IXpsOMGlyphsEditorImpl: Sized {
    fn ApplyEdits();
    fn GetUnicodeString();
    fn SetUnicodeString();
    fn GetGlyphIndexCount();
    fn GetGlyphIndices();
    fn SetGlyphIndices();
    fn GetGlyphMappingCount();
    fn GetGlyphMappings();
    fn SetGlyphMappings();
    fn GetProhibitedCaretStopCount();
    fn GetProhibitedCaretStops();
    fn SetProhibitedCaretStops();
    fn GetBidiLevel();
    fn SetBidiLevel();
    fn GetIsSideways();
    fn SetIsSideways();
    fn GetDeviceFontName();
    fn SetDeviceFontName();
}
pub trait IXpsOMGradientBrushImpl: Sized + IXpsOMBrushImpl + IXpsOMShareableImpl {
    fn GetGradientStops();
    fn GetTransform();
    fn GetTransformLocal();
    fn SetTransformLocal();
    fn GetTransformLookup();
    fn SetTransformLookup();
    fn GetSpreadMethod();
    fn SetSpreadMethod();
    fn GetColorInterpolationMode();
    fn SetColorInterpolationMode();
}
pub trait IXpsOMGradientStopImpl: Sized {
    fn GetOwner();
    fn GetOffset();
    fn SetOffset();
    fn GetColor();
    fn SetColor();
    fn Clone();
}
pub trait IXpsOMGradientStopCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Append();
}
pub trait IXpsOMImageBrushImpl: Sized + IXpsOMTileBrushImpl + IXpsOMBrushImpl + IXpsOMShareableImpl {
    fn GetImageResource();
    fn SetImageResource();
    fn GetColorProfileResource();
    fn SetColorProfileResource();
    fn Clone();
}
pub trait IXpsOMImageResourceImpl: Sized + IXpsOMResourceImpl + IXpsOMPartImpl {
    fn GetStream();
    fn SetContent();
    fn GetImageType();
}
pub trait IXpsOMImageResourceCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Append();
    fn GetByPartName();
}
pub trait IXpsOMLinearGradientBrushImpl: Sized + IXpsOMGradientBrushImpl + IXpsOMBrushImpl + IXpsOMShareableImpl {
    fn GetStartPoint();
    fn SetStartPoint();
    fn GetEndPoint();
    fn SetEndPoint();
    fn Clone();
}
pub trait IXpsOMMatrixTransformImpl: Sized + IXpsOMShareableImpl {
    fn GetMatrix();
    fn SetMatrix();
    fn Clone();
}
pub trait IXpsOMNameCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
}
pub trait IXpsOMObjectFactoryImpl: Sized {
    fn CreatePackage();
    fn CreatePackageFromFile();
    fn CreatePackageFromStream();
    fn CreateStoryFragmentsResource();
    fn CreateDocumentStructureResource();
    fn CreateSignatureBlockResource();
    fn CreateRemoteDictionaryResource();
    fn CreateRemoteDictionaryResourceFromStream();
    fn CreatePartResources();
    fn CreateDocumentSequence();
    fn CreateDocument();
    fn CreatePageReference();
    fn CreatePage();
    fn CreatePageFromStream();
    fn CreateCanvas();
    fn CreateGlyphs();
    fn CreatePath();
    fn CreateGeometry();
    fn CreateGeometryFigure();
    fn CreateMatrixTransform();
    fn CreateSolidColorBrush();
    fn CreateColorProfileResource();
    fn CreateImageBrush();
    fn CreateVisualBrush();
    fn CreateImageResource();
    fn CreatePrintTicketResource();
    fn CreateFontResource();
    fn CreateGradientStop();
    fn CreateLinearGradientBrush();
    fn CreateRadialGradientBrush();
    fn CreateCoreProperties();
    fn CreateDictionary();
    fn CreatePartUriCollection();
    fn CreatePackageWriterOnFile();
    fn CreatePackageWriterOnStream();
    fn CreatePartUri();
    fn CreateReadOnlyStreamOnFile();
}
pub trait IXpsOMObjectFactory1Impl: Sized + IXpsOMObjectFactoryImpl {
    fn GetDocumentTypeFromFile();
    fn GetDocumentTypeFromStream();
    fn ConvertHDPhotoToJpegXR();
    fn ConvertJpegXRToHDPhoto();
    fn CreatePackageWriterOnFile1();
    fn CreatePackageWriterOnStream1();
    fn CreatePackage1();
    fn CreatePackageFromStream1();
    fn CreatePackageFromFile1();
    fn CreatePage1();
    fn CreatePageFromStream1();
    fn CreateRemoteDictionaryResourceFromStream1();
}
pub trait IXpsOMPackageImpl: Sized {
    fn GetDocumentSequence();
    fn SetDocumentSequence();
    fn GetCoreProperties();
    fn SetCoreProperties();
    fn GetDiscardControlPartName();
    fn SetDiscardControlPartName();
    fn GetThumbnailResource();
    fn SetThumbnailResource();
    fn WriteToFile();
    fn WriteToStream();
}
pub trait IXpsOMPackage1Impl: Sized + IXpsOMPackageImpl {
    fn GetDocumentType();
    fn WriteToFile1();
    fn WriteToStream1();
}
pub trait IXpsOMPackageTargetImpl: Sized {
    fn CreateXpsOMPackageWriter();
}
pub trait IXpsOMPackageWriterImpl: Sized {
    fn StartNewDocument();
    fn AddPage();
    fn AddResource();
    fn Close();
    fn IsClosed();
}
pub trait IXpsOMPackageWriter3DImpl: Sized + IXpsOMPackageWriterImpl {
    fn AddModelTexture();
    fn SetModelPrintTicket();
}
pub trait IXpsOMPageImpl: Sized + IXpsOMPartImpl {
    fn GetOwner();
    fn GetVisuals();
    fn GetPageDimensions();
    fn SetPageDimensions();
    fn GetContentBox();
    fn SetContentBox();
    fn GetBleedBox();
    fn SetBleedBox();
    fn GetLanguage();
    fn SetLanguage();
    fn GetName();
    fn SetName();
    fn GetIsHyperlinkTarget();
    fn SetIsHyperlinkTarget();
    fn GetDictionary();
    fn GetDictionaryLocal();
    fn SetDictionaryLocal();
    fn GetDictionaryResource();
    fn SetDictionaryResource();
    fn Write();
    fn GenerateUnusedLookupKey();
    fn Clone();
}
pub trait IXpsOMPage1Impl: Sized + IXpsOMPageImpl + IXpsOMPartImpl {
    fn GetDocumentType();
    fn Write1();
}
pub trait IXpsOMPageReferenceImpl: Sized {
    fn GetOwner();
    fn GetPage();
    fn SetPage();
    fn DiscardPage();
    fn IsPageLoaded();
    fn GetAdvisoryPageDimensions();
    fn SetAdvisoryPageDimensions();
    fn GetStoryFragmentsResource();
    fn SetStoryFragmentsResource();
    fn GetPrintTicketResource();
    fn SetPrintTicketResource();
    fn GetThumbnailResource();
    fn SetThumbnailResource();
    fn CollectLinkTargets();
    fn CollectPartResources();
    fn HasRestrictedFonts();
    fn Clone();
}
pub trait IXpsOMPageReferenceCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Append();
}
pub trait IXpsOMPartImpl: Sized {
    fn GetPartName();
    fn SetPartName();
}
pub trait IXpsOMPartResourcesImpl: Sized {
    fn GetFontResources();
    fn GetImageResources();
    fn GetColorProfileResources();
    fn GetRemoteDictionaryResources();
}
pub trait IXpsOMPartUriCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Append();
}
pub trait IXpsOMPathImpl: Sized + IXpsOMVisualImpl + IXpsOMShareableImpl {
    fn GetGeometry();
    fn GetGeometryLocal();
    fn SetGeometryLocal();
    fn GetGeometryLookup();
    fn SetGeometryLookup();
    fn GetAccessibilityShortDescription();
    fn SetAccessibilityShortDescription();
    fn GetAccessibilityLongDescription();
    fn SetAccessibilityLongDescription();
    fn GetSnapsToPixels();
    fn SetSnapsToPixels();
    fn GetStrokeBrush();
    fn GetStrokeBrushLocal();
    fn SetStrokeBrushLocal();
    fn GetStrokeBrushLookup();
    fn SetStrokeBrushLookup();
    fn GetStrokeDashes();
    fn GetStrokeDashCap();
    fn SetStrokeDashCap();
    fn GetStrokeDashOffset();
    fn SetStrokeDashOffset();
    fn GetStrokeStartLineCap();
    fn SetStrokeStartLineCap();
    fn GetStrokeEndLineCap();
    fn SetStrokeEndLineCap();
    fn GetStrokeLineJoin();
    fn SetStrokeLineJoin();
    fn GetStrokeMiterLimit();
    fn SetStrokeMiterLimit();
    fn GetStrokeThickness();
    fn SetStrokeThickness();
    fn GetFillBrush();
    fn GetFillBrushLocal();
    fn SetFillBrushLocal();
    fn GetFillBrushLookup();
    fn SetFillBrushLookup();
    fn Clone();
}
pub trait IXpsOMPrintTicketResourceImpl: Sized + IXpsOMResourceImpl + IXpsOMPartImpl {
    fn GetStream();
    fn SetContent();
}
pub trait IXpsOMRadialGradientBrushImpl: Sized + IXpsOMGradientBrushImpl + IXpsOMBrushImpl + IXpsOMShareableImpl {
    fn GetCenter();
    fn SetCenter();
    fn GetRadiiSizes();
    fn SetRadiiSizes();
    fn GetGradientOrigin();
    fn SetGradientOrigin();
    fn Clone();
}
pub trait IXpsOMRemoteDictionaryResourceImpl: Sized + IXpsOMResourceImpl + IXpsOMPartImpl {
    fn GetDictionary();
    fn SetDictionary();
}
pub trait IXpsOMRemoteDictionaryResource1Impl: Sized + IXpsOMRemoteDictionaryResourceImpl + IXpsOMResourceImpl + IXpsOMPartImpl {
    fn GetDocumentType();
    fn Write1();
}
pub trait IXpsOMRemoteDictionaryResourceCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Append();
    fn GetByPartName();
}
pub trait IXpsOMResourceImpl: Sized + IXpsOMPartImpl {}
pub trait IXpsOMShareableImpl: Sized {
    fn GetOwner();
    fn GetType();
}
pub trait IXpsOMSignatureBlockResourceImpl: Sized + IXpsOMResourceImpl + IXpsOMPartImpl {
    fn GetOwner();
    fn GetStream();
    fn SetContent();
}
pub trait IXpsOMSignatureBlockResourceCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Append();
    fn GetByPartName();
}
pub trait IXpsOMSolidColorBrushImpl: Sized + IXpsOMBrushImpl + IXpsOMShareableImpl {
    fn GetColor();
    fn SetColor();
    fn Clone();
}
pub trait IXpsOMStoryFragmentsResourceImpl: Sized + IXpsOMResourceImpl + IXpsOMPartImpl {
    fn GetOwner();
    fn GetStream();
    fn SetContent();
}
pub trait IXpsOMThumbnailGeneratorImpl: Sized {
    fn GenerateThumbnail();
}
pub trait IXpsOMTileBrushImpl: Sized + IXpsOMBrushImpl + IXpsOMShareableImpl {
    fn GetTransform();
    fn GetTransformLocal();
    fn SetTransformLocal();
    fn GetTransformLookup();
    fn SetTransformLookup();
    fn GetViewbox();
    fn SetViewbox();
    fn GetViewport();
    fn SetViewport();
    fn GetTileMode();
    fn SetTileMode();
}
pub trait IXpsOMVisualImpl: Sized + IXpsOMShareableImpl {
    fn GetTransform();
    fn GetTransformLocal();
    fn SetTransformLocal();
    fn GetTransformLookup();
    fn SetTransformLookup();
    fn GetClipGeometry();
    fn GetClipGeometryLocal();
    fn SetClipGeometryLocal();
    fn GetClipGeometryLookup();
    fn SetClipGeometryLookup();
    fn GetOpacity();
    fn SetOpacity();
    fn GetOpacityMaskBrush();
    fn GetOpacityMaskBrushLocal();
    fn SetOpacityMaskBrushLocal();
    fn GetOpacityMaskBrushLookup();
    fn SetOpacityMaskBrushLookup();
    fn GetName();
    fn SetName();
    fn GetIsHyperlinkTarget();
    fn SetIsHyperlinkTarget();
    fn GetHyperlinkNavigateUri();
    fn SetHyperlinkNavigateUri();
    fn GetLanguage();
    fn SetLanguage();
}
pub trait IXpsOMVisualBrushImpl: Sized + IXpsOMTileBrushImpl + IXpsOMBrushImpl + IXpsOMShareableImpl {
    fn GetVisual();
    fn GetVisualLocal();
    fn SetVisualLocal();
    fn GetVisualLookup();
    fn SetVisualLookup();
    fn Clone();
}
pub trait IXpsOMVisualCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn InsertAt();
    fn RemoveAt();
    fn SetAt();
    fn Append();
}
pub trait IXpsSignatureImpl: Sized {
    fn GetSignatureId();
    fn GetSignatureValue();
    fn GetCertificateEnumerator();
    fn GetSigningTime();
    fn GetSigningTimeFormat();
    fn GetSignaturePartName();
    fn Verify();
    fn GetPolicy();
    fn GetCustomObjectEnumerator();
    fn GetCustomReferenceEnumerator();
    fn GetSignatureXml();
    fn SetSignatureXml();
}
pub trait IXpsSignatureBlockImpl: Sized {
    fn GetRequests();
    fn GetPartName();
    fn GetDocumentIndex();
    fn GetDocumentName();
    fn CreateRequest();
}
pub trait IXpsSignatureBlockCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn RemoveAt();
}
pub trait IXpsSignatureCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn RemoveAt();
}
pub trait IXpsSignatureManagerImpl: Sized {
    fn LoadPackageFile();
    fn LoadPackageStream();
    fn Sign();
    fn GetSignatureOriginPartName();
    fn SetSignatureOriginPartName();
    fn GetSignatures();
    fn AddSignatureBlock();
    fn GetSignatureBlocks();
    fn CreateSigningOptions();
    fn SavePackageToFile();
    fn SavePackageToStream();
}
pub trait IXpsSignatureRequestImpl: Sized {
    fn GetIntent();
    fn SetIntent();
    fn GetRequestedSigner();
    fn SetRequestedSigner();
    fn GetRequestSignByDate();
    fn SetRequestSignByDate();
    fn GetSigningLocale();
    fn SetSigningLocale();
    fn GetSpotLocation();
    fn SetSpotLocation();
    fn GetRequestId();
    fn GetSignature();
}
pub trait IXpsSignatureRequestCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn RemoveAt();
}
pub trait IXpsSigningOptionsImpl: Sized {
    fn GetSignatureId();
    fn SetSignatureId();
    fn GetSignatureMethod();
    fn SetSignatureMethod();
    fn GetDigestMethod();
    fn SetDigestMethod();
    fn GetSignaturePartName();
    fn SetSignaturePartName();
    fn GetPolicy();
    fn SetPolicy();
    fn GetSigningTimeFormat();
    fn SetSigningTimeFormat();
    fn GetCustomObjects();
    fn GetCustomReferences();
    fn GetCertificateSet();
    fn GetFlags();
    fn SetFlags();
}
