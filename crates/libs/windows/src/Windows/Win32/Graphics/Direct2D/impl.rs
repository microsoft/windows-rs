pub trait ID2D1AnalysisTransformImpl: Sized {
    fn ProcessAnalysisResults();
}
pub trait ID2D1BitmapImpl: Sized + ID2D1ImageImpl + ID2D1ResourceImpl {
    fn GetSize();
    fn GetPixelSize();
    fn GetPixelFormat();
    fn GetDpi();
    fn CopyFromBitmap();
    fn CopyFromRenderTarget();
    fn CopyFromMemory();
}
pub trait ID2D1Bitmap1Impl: Sized + ID2D1BitmapImpl + ID2D1ImageImpl + ID2D1ResourceImpl {
    fn GetColorContext();
    fn GetOptions();
    fn GetSurface();
    fn Map();
    fn Unmap();
}
pub trait ID2D1BitmapBrushImpl: Sized + ID2D1BrushImpl + ID2D1ResourceImpl {
    fn SetExtendModeX();
    fn SetExtendModeY();
    fn SetInterpolationMode();
    fn SetBitmap();
    fn GetExtendModeX();
    fn GetExtendModeY();
    fn GetInterpolationMode();
    fn GetBitmap();
}
pub trait ID2D1BitmapBrush1Impl: Sized + ID2D1BitmapBrushImpl + ID2D1BrushImpl + ID2D1ResourceImpl {
    fn SetInterpolationMode1();
    fn GetInterpolationMode1();
}
pub trait ID2D1BitmapRenderTargetImpl: Sized + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn GetBitmap();
}
pub trait ID2D1BlendTransformImpl: Sized + ID2D1ConcreteTransformImpl + ID2D1TransformNodeImpl {
    fn SetDescription();
    fn GetDescription();
}
pub trait ID2D1BorderTransformImpl: Sized + ID2D1ConcreteTransformImpl + ID2D1TransformNodeImpl {
    fn SetExtendModeX();
    fn SetExtendModeY();
    fn GetExtendModeX();
    fn GetExtendModeY();
}
pub trait ID2D1BoundsAdjustmentTransformImpl: Sized + ID2D1TransformNodeImpl {
    fn SetOutputBounds();
    fn GetOutputBounds();
}
pub trait ID2D1BrushImpl: Sized + ID2D1ResourceImpl {
    fn SetOpacity();
    fn SetTransform();
    fn GetOpacity();
    fn GetTransform();
}
pub trait ID2D1ColorContextImpl: Sized + ID2D1ResourceImpl {
    fn GetColorSpace();
    fn GetProfileSize();
    fn GetProfile();
}
pub trait ID2D1ColorContext1Impl: Sized + ID2D1ColorContextImpl + ID2D1ResourceImpl {
    fn GetColorContextType();
    fn GetDXGIColorSpace();
    fn GetSimpleColorProfile();
}
pub trait ID2D1CommandListImpl: Sized + ID2D1ImageImpl + ID2D1ResourceImpl {
    fn Stream();
    fn Close();
}
pub trait ID2D1CommandSinkImpl: Sized {
    fn BeginDraw();
    fn EndDraw();
    fn SetAntialiasMode();
    fn SetTags();
    fn SetTextAntialiasMode();
    fn SetTextRenderingParams();
    fn SetTransform();
    fn SetPrimitiveBlend();
    fn SetUnitMode();
    fn Clear();
    fn DrawGlyphRun();
    fn DrawLine();
    fn DrawGeometry();
    fn DrawRectangle();
    fn DrawBitmap();
    fn DrawImage();
    fn DrawGdiMetafile();
    fn FillMesh();
    fn FillOpacityMask();
    fn FillGeometry();
    fn FillRectangle();
    fn PushAxisAlignedClip();
    fn PushLayer();
    fn PopAxisAlignedClip();
    fn PopLayer();
}
pub trait ID2D1CommandSink1Impl: Sized + ID2D1CommandSinkImpl {
    fn SetPrimitiveBlend1();
}
pub trait ID2D1CommandSink2Impl: Sized + ID2D1CommandSink1Impl + ID2D1CommandSinkImpl {
    fn DrawInk();
    fn DrawGradientMesh();
    fn DrawGdiMetafile();
}
pub trait ID2D1CommandSink3Impl: Sized + ID2D1CommandSink2Impl + ID2D1CommandSink1Impl + ID2D1CommandSinkImpl {
    fn DrawSpriteBatch();
}
pub trait ID2D1CommandSink4Impl: Sized + ID2D1CommandSink3Impl + ID2D1CommandSink2Impl + ID2D1CommandSink1Impl + ID2D1CommandSinkImpl {
    fn SetPrimitiveBlend2();
}
pub trait ID2D1CommandSink5Impl: Sized + ID2D1CommandSink4Impl + ID2D1CommandSink3Impl + ID2D1CommandSink2Impl + ID2D1CommandSink1Impl + ID2D1CommandSinkImpl {
    fn BlendImage();
}
pub trait ID2D1ComputeInfoImpl: Sized + ID2D1RenderInfoImpl {
    fn SetComputeShaderConstantBuffer();
    fn SetComputeShader();
    fn SetResourceTexture();
}
pub trait ID2D1ComputeTransformImpl: Sized + ID2D1TransformImpl + ID2D1TransformNodeImpl {
    fn SetComputeInfo();
    fn CalculateThreadgroups();
}
pub trait ID2D1ConcreteTransformImpl: Sized + ID2D1TransformNodeImpl {
    fn SetOutputBuffer();
    fn SetCached();
}
pub trait ID2D1DCRenderTargetImpl: Sized + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn BindDC();
}
pub trait ID2D1DeviceImpl: Sized + ID2D1ResourceImpl {
    fn CreateDeviceContext();
    fn CreatePrintControl();
    fn SetMaximumTextureMemory();
    fn GetMaximumTextureMemory();
    fn ClearResources();
}
pub trait ID2D1Device1Impl: Sized + ID2D1DeviceImpl + ID2D1ResourceImpl {
    fn GetRenderingPriority();
    fn SetRenderingPriority();
    fn CreateDeviceContext();
}
pub trait ID2D1Device2Impl: Sized + ID2D1Device1Impl + ID2D1DeviceImpl + ID2D1ResourceImpl {
    fn CreateDeviceContext();
    fn FlushDeviceContexts();
    fn GetDxgiDevice();
}
pub trait ID2D1Device3Impl: Sized + ID2D1Device2Impl + ID2D1Device1Impl + ID2D1DeviceImpl + ID2D1ResourceImpl {
    fn CreateDeviceContext();
}
pub trait ID2D1Device4Impl: Sized + ID2D1Device3Impl + ID2D1Device2Impl + ID2D1Device1Impl + ID2D1DeviceImpl + ID2D1ResourceImpl {
    fn CreateDeviceContext();
    fn SetMaximumColorGlyphCacheMemory();
    fn GetMaximumColorGlyphCacheMemory();
}
pub trait ID2D1Device5Impl: Sized + ID2D1Device4Impl + ID2D1Device3Impl + ID2D1Device2Impl + ID2D1Device1Impl + ID2D1DeviceImpl + ID2D1ResourceImpl {
    fn CreateDeviceContext();
}
pub trait ID2D1Device6Impl: Sized + ID2D1Device5Impl + ID2D1Device4Impl + ID2D1Device3Impl + ID2D1Device2Impl + ID2D1Device1Impl + ID2D1DeviceImpl + ID2D1ResourceImpl {
    fn CreateDeviceContext();
}
pub trait ID2D1DeviceContextImpl: Sized + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn CreateBitmap();
    fn CreateBitmapFromWicBitmap();
    fn CreateColorContext();
    fn CreateColorContextFromFilename();
    fn CreateColorContextFromWicColorContext();
    fn CreateBitmapFromDxgiSurface();
    fn CreateEffect();
    fn CreateGradientStopCollection();
    fn CreateImageBrush();
    fn CreateBitmapBrush();
    fn CreateCommandList();
    fn IsDxgiFormatSupported();
    fn IsBufferPrecisionSupported();
    fn GetImageLocalBounds();
    fn GetImageWorldBounds();
    fn GetGlyphRunWorldBounds();
    fn GetDevice();
    fn SetTarget();
    fn GetTarget();
    fn SetRenderingControls();
    fn GetRenderingControls();
    fn SetPrimitiveBlend();
    fn GetPrimitiveBlend();
    fn SetUnitMode();
    fn GetUnitMode();
    fn DrawGlyphRun();
    fn DrawImage();
    fn DrawGdiMetafile();
    fn DrawBitmap();
    fn PushLayer();
    fn InvalidateEffectInputRectangle();
    fn GetEffectInvalidRectangleCount();
    fn GetEffectInvalidRectangles();
    fn GetEffectRequiredInputRectangles();
    fn FillOpacityMask();
}
pub trait ID2D1DeviceContext1Impl: Sized + ID2D1DeviceContextImpl + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn CreateFilledGeometryRealization();
    fn CreateStrokedGeometryRealization();
    fn DrawGeometryRealization();
}
pub trait ID2D1DeviceContext2Impl: Sized + ID2D1DeviceContext1Impl + ID2D1DeviceContextImpl + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn CreateInk();
    fn CreateInkStyle();
    fn CreateGradientMesh();
    fn CreateImageSourceFromWic();
    fn CreateLookupTable3D();
    fn CreateImageSourceFromDxgi();
    fn GetGradientMeshWorldBounds();
    fn DrawInk();
    fn DrawGradientMesh();
    fn DrawGdiMetafile();
    fn CreateTransformedImageSource();
}
pub trait ID2D1DeviceContext3Impl: Sized + ID2D1DeviceContext2Impl + ID2D1DeviceContext1Impl + ID2D1DeviceContextImpl + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn CreateSpriteBatch();
    fn DrawSpriteBatch();
}
pub trait ID2D1DeviceContext4Impl: Sized + ID2D1DeviceContext3Impl + ID2D1DeviceContext2Impl + ID2D1DeviceContext1Impl + ID2D1DeviceContextImpl + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn CreateSvgGlyphStyle();
    fn DrawText();
    fn DrawTextLayout();
    fn DrawColorBitmapGlyphRun();
    fn DrawSvgGlyphRun();
    fn GetColorBitmapGlyphImage();
    fn GetSvgGlyphImage();
}
pub trait ID2D1DeviceContext5Impl: Sized + ID2D1DeviceContext4Impl + ID2D1DeviceContext3Impl + ID2D1DeviceContext2Impl + ID2D1DeviceContext1Impl + ID2D1DeviceContextImpl + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn CreateSvgDocument();
    fn DrawSvgDocument();
    fn CreateColorContextFromDxgiColorSpace();
    fn CreateColorContextFromSimpleColorProfile();
}
pub trait ID2D1DeviceContext6Impl: Sized + ID2D1DeviceContext5Impl + ID2D1DeviceContext4Impl + ID2D1DeviceContext3Impl + ID2D1DeviceContext2Impl + ID2D1DeviceContext1Impl + ID2D1DeviceContextImpl + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn BlendImage();
}
pub trait ID2D1DrawInfoImpl: Sized + ID2D1RenderInfoImpl {
    fn SetPixelShaderConstantBuffer();
    fn SetResourceTexture();
    fn SetVertexShaderConstantBuffer();
    fn SetPixelShader();
    fn SetVertexProcessing();
}
pub trait ID2D1DrawTransformImpl: Sized + ID2D1TransformImpl + ID2D1TransformNodeImpl {
    fn SetDrawInfo();
}
pub trait ID2D1DrawingStateBlockImpl: Sized + ID2D1ResourceImpl {
    fn GetDescription();
    fn SetDescription();
    fn SetTextRenderingParams();
    fn GetTextRenderingParams();
}
pub trait ID2D1DrawingStateBlock1Impl: Sized + ID2D1DrawingStateBlockImpl + ID2D1ResourceImpl {
    fn GetDescription();
    fn SetDescription();
}
pub trait ID2D1EffectImpl: Sized + ID2D1PropertiesImpl {
    fn SetInput();
    fn SetInputCount();
    fn GetInput();
    fn GetInputCount();
    fn GetOutput();
}
pub trait ID2D1EffectContextImpl: Sized {
    fn GetDpi();
    fn CreateEffect();
    fn GetMaximumSupportedFeatureLevel();
    fn CreateTransformNodeFromEffect();
    fn CreateBlendTransform();
    fn CreateBorderTransform();
    fn CreateOffsetTransform();
    fn CreateBoundsAdjustmentTransform();
    fn LoadPixelShader();
    fn LoadVertexShader();
    fn LoadComputeShader();
    fn IsShaderLoaded();
    fn CreateResourceTexture();
    fn FindResourceTexture();
    fn CreateVertexBuffer();
    fn FindVertexBuffer();
    fn CreateColorContext();
    fn CreateColorContextFromFilename();
    fn CreateColorContextFromWicColorContext();
    fn CheckFeatureSupport();
    fn IsBufferPrecisionSupported();
}
pub trait ID2D1EffectContext1Impl: Sized + ID2D1EffectContextImpl {
    fn CreateLookupTable3D();
}
pub trait ID2D1EffectContext2Impl: Sized + ID2D1EffectContext1Impl + ID2D1EffectContextImpl {
    fn CreateColorContextFromDxgiColorSpace();
    fn CreateColorContextFromSimpleColorProfile();
}
pub trait ID2D1EffectImplImpl: Sized {
    fn Initialize();
    fn PrepareForRender();
    fn SetGraph();
}
pub trait ID2D1EllipseGeometryImpl: Sized + ID2D1GeometryImpl + ID2D1ResourceImpl {
    fn GetEllipse();
}
pub trait ID2D1FactoryImpl: Sized {
    fn ReloadSystemMetrics();
    fn GetDesktopDpi();
    fn CreateRectangleGeometry();
    fn CreateRoundedRectangleGeometry();
    fn CreateEllipseGeometry();
    fn CreateGeometryGroup();
    fn CreateTransformedGeometry();
    fn CreatePathGeometry();
    fn CreateStrokeStyle();
    fn CreateDrawingStateBlock();
    fn CreateWicBitmapRenderTarget();
    fn CreateHwndRenderTarget();
    fn CreateDxgiSurfaceRenderTarget();
    fn CreateDCRenderTarget();
}
pub trait ID2D1Factory1Impl: Sized + ID2D1FactoryImpl {
    fn CreateDevice();
    fn CreateStrokeStyle();
    fn CreatePathGeometry();
    fn CreateDrawingStateBlock();
    fn CreateGdiMetafile();
    fn RegisterEffectFromStream();
    fn RegisterEffectFromString();
    fn UnregisterEffect();
    fn GetRegisteredEffects();
    fn GetEffectProperties();
}
pub trait ID2D1Factory2Impl: Sized + ID2D1Factory1Impl + ID2D1FactoryImpl {
    fn CreateDevice();
}
pub trait ID2D1Factory3Impl: Sized + ID2D1Factory2Impl + ID2D1Factory1Impl + ID2D1FactoryImpl {
    fn CreateDevice();
}
pub trait ID2D1Factory4Impl: Sized + ID2D1Factory3Impl + ID2D1Factory2Impl + ID2D1Factory1Impl + ID2D1FactoryImpl {
    fn CreateDevice();
}
pub trait ID2D1Factory5Impl: Sized + ID2D1Factory4Impl + ID2D1Factory3Impl + ID2D1Factory2Impl + ID2D1Factory1Impl + ID2D1FactoryImpl {
    fn CreateDevice();
}
pub trait ID2D1Factory6Impl: Sized + ID2D1Factory5Impl + ID2D1Factory4Impl + ID2D1Factory3Impl + ID2D1Factory2Impl + ID2D1Factory1Impl + ID2D1FactoryImpl {
    fn CreateDevice();
}
pub trait ID2D1Factory7Impl: Sized + ID2D1Factory6Impl + ID2D1Factory5Impl + ID2D1Factory4Impl + ID2D1Factory3Impl + ID2D1Factory2Impl + ID2D1Factory1Impl + ID2D1FactoryImpl {
    fn CreateDevice();
}
pub trait ID2D1GdiInteropRenderTargetImpl: Sized {
    fn GetDC();
    fn ReleaseDC();
}
pub trait ID2D1GdiMetafileImpl: Sized + ID2D1ResourceImpl {
    fn Stream();
    fn GetBounds();
}
pub trait ID2D1GdiMetafile1Impl: Sized + ID2D1GdiMetafileImpl + ID2D1ResourceImpl {
    fn GetDpi();
    fn GetSourceBounds();
}
pub trait ID2D1GdiMetafileSinkImpl: Sized {
    fn ProcessRecord();
}
pub trait ID2D1GdiMetafileSink1Impl: Sized + ID2D1GdiMetafileSinkImpl {
    fn ProcessRecord();
}
pub trait ID2D1GeometryImpl: Sized + ID2D1ResourceImpl {
    fn GetBounds();
    fn GetWidenedBounds();
    fn StrokeContainsPoint();
    fn FillContainsPoint();
    fn CompareWithGeometry();
    fn Simplify();
    fn Tessellate();
    fn CombineWithGeometry();
    fn Outline();
    fn ComputeArea();
    fn ComputeLength();
    fn ComputePointAtLength();
    fn Widen();
}
pub trait ID2D1GeometryGroupImpl: Sized + ID2D1GeometryImpl + ID2D1ResourceImpl {
    fn GetFillMode();
    fn GetSourceGeometryCount();
    fn GetSourceGeometries();
}
pub trait ID2D1GeometryRealizationImpl: Sized + ID2D1ResourceImpl {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1GeometrySinkImpl: Sized + ID2D1SimplifiedGeometrySinkImpl {
    fn AddLine();
    fn AddBezier();
    fn AddQuadraticBezier();
    fn AddQuadraticBeziers();
    fn AddArc();
}
pub trait ID2D1GradientMeshImpl: Sized + ID2D1ResourceImpl {
    fn GetPatchCount();
    fn GetPatches();
}
pub trait ID2D1GradientStopCollectionImpl: Sized + ID2D1ResourceImpl {
    fn GetGradientStopCount();
    fn GetGradientStops();
    fn GetColorInterpolationGamma();
    fn GetExtendMode();
}
pub trait ID2D1GradientStopCollection1Impl: Sized + ID2D1GradientStopCollectionImpl + ID2D1ResourceImpl {
    fn GetGradientStops1();
    fn GetPreInterpolationSpace();
    fn GetPostInterpolationSpace();
    fn GetBufferPrecision();
    fn GetColorInterpolationMode();
}
pub trait ID2D1HwndRenderTargetImpl: Sized + ID2D1RenderTargetImpl + ID2D1ResourceImpl {
    fn CheckWindowState();
    fn Resize();
    fn GetHwnd();
}
pub trait ID2D1ImageImpl: Sized + ID2D1ResourceImpl {}
pub trait ID2D1ImageBrushImpl: Sized + ID2D1BrushImpl + ID2D1ResourceImpl {
    fn SetImage();
    fn SetExtendModeX();
    fn SetExtendModeY();
    fn SetInterpolationMode();
    fn SetSourceRectangle();
    fn GetImage();
    fn GetExtendModeX();
    fn GetExtendModeY();
    fn GetInterpolationMode();
    fn GetSourceRectangle();
}
pub trait ID2D1ImageSourceImpl: Sized + ID2D1ImageImpl + ID2D1ResourceImpl {
    fn OfferResources();
    fn TryReclaimResources();
}
pub trait ID2D1ImageSourceFromWicImpl: Sized + ID2D1ImageSourceImpl + ID2D1ImageImpl + ID2D1ResourceImpl {
    fn EnsureCached();
    fn TrimCache();
    fn GetSource();
}
pub trait ID2D1InkImpl: Sized + ID2D1ResourceImpl {
    fn SetStartPoint();
    fn GetStartPoint();
    fn AddSegments();
    fn RemoveSegmentsAtEnd();
    fn SetSegments();
    fn SetSegmentAtEnd();
    fn GetSegmentCount();
    fn GetSegments();
    fn StreamAsGeometry();
    fn GetBounds();
}
pub trait ID2D1InkStyleImpl: Sized + ID2D1ResourceImpl {
    fn SetNibTransform();
    fn GetNibTransform();
    fn SetNibShape();
    fn GetNibShape();
}
pub trait ID2D1LayerImpl: Sized + ID2D1ResourceImpl {
    fn GetSize();
}
pub trait ID2D1LinearGradientBrushImpl: Sized + ID2D1BrushImpl + ID2D1ResourceImpl {
    fn SetStartPoint();
    fn SetEndPoint();
    fn GetStartPoint();
    fn GetEndPoint();
    fn GetGradientStopCollection();
}
pub trait ID2D1LookupTable3DImpl: Sized + ID2D1ResourceImpl {}
pub trait ID2D1MeshImpl: Sized + ID2D1ResourceImpl {
    fn Open();
}
pub trait ID2D1MultithreadImpl: Sized {
    fn GetMultithreadProtected();
    fn Enter();
    fn Leave();
}
pub trait ID2D1OffsetTransformImpl: Sized + ID2D1TransformNodeImpl {
    fn SetOffset();
    fn GetOffset();
}
pub trait ID2D1PathGeometryImpl: Sized + ID2D1GeometryImpl + ID2D1ResourceImpl {
    fn Open();
    fn Stream();
    fn GetSegmentCount();
    fn GetFigureCount();
}
pub trait ID2D1PathGeometry1Impl: Sized + ID2D1PathGeometryImpl + ID2D1GeometryImpl + ID2D1ResourceImpl {
    fn ComputePointAndSegmentAtLength();
}
pub trait ID2D1PrintControlImpl: Sized {
    fn AddPage();
    fn Close();
}
pub trait ID2D1PropertiesImpl: Sized {
    fn GetPropertyCount();
    fn GetPropertyName();
    fn GetPropertyNameLength();
    fn GetType();
    fn GetPropertyIndex();
    fn SetValueByName();
    fn SetValue();
    fn GetValueByName();
    fn GetValue();
    fn GetValueSize();
    fn GetSubProperties();
}
pub trait ID2D1RadialGradientBrushImpl: Sized + ID2D1BrushImpl + ID2D1ResourceImpl {
    fn SetCenter();
    fn SetGradientOriginOffset();
    fn SetRadiusX();
    fn SetRadiusY();
    fn GetCenter();
    fn GetGradientOriginOffset();
    fn GetRadiusX();
    fn GetRadiusY();
    fn GetGradientStopCollection();
}
pub trait ID2D1RectangleGeometryImpl: Sized + ID2D1GeometryImpl + ID2D1ResourceImpl {
    fn GetRect();
}
pub trait ID2D1RenderInfoImpl: Sized {
    fn SetInputDescription();
    fn SetOutputBuffer();
    fn SetCached();
    fn SetInstructionCountHint();
}
pub trait ID2D1RenderTargetImpl: Sized + ID2D1ResourceImpl {
    fn CreateBitmap();
    fn CreateBitmapFromWicBitmap();
    fn CreateSharedBitmap();
    fn CreateBitmapBrush();
    fn CreateSolidColorBrush();
    fn CreateGradientStopCollection();
    fn CreateLinearGradientBrush();
    fn CreateRadialGradientBrush();
    fn CreateCompatibleRenderTarget();
    fn CreateLayer();
    fn CreateMesh();
    fn DrawLine();
    fn DrawRectangle();
    fn FillRectangle();
    fn DrawRoundedRectangle();
    fn FillRoundedRectangle();
    fn DrawEllipse();
    fn FillEllipse();
    fn DrawGeometry();
    fn FillGeometry();
    fn FillMesh();
    fn FillOpacityMask();
    fn DrawBitmap();
    fn DrawText();
    fn DrawTextLayout();
    fn DrawGlyphRun();
    fn SetTransform();
    fn GetTransform();
    fn SetAntialiasMode();
    fn GetAntialiasMode();
    fn SetTextAntialiasMode();
    fn GetTextAntialiasMode();
    fn SetTextRenderingParams();
    fn GetTextRenderingParams();
    fn SetTags();
    fn GetTags();
    fn PushLayer();
    fn PopLayer();
    fn Flush();
    fn SaveDrawingState();
    fn RestoreDrawingState();
    fn PushAxisAlignedClip();
    fn PopAxisAlignedClip();
    fn Clear();
    fn BeginDraw();
    fn EndDraw();
    fn GetPixelFormat();
    fn SetDpi();
    fn GetDpi();
    fn GetSize();
    fn GetPixelSize();
    fn GetMaximumBitmapSize();
    fn IsSupported();
}
pub trait ID2D1ResourceImpl: Sized {
    fn GetFactory();
}
pub trait ID2D1ResourceTextureImpl: Sized {
    fn Update();
}
pub trait ID2D1RoundedRectangleGeometryImpl: Sized + ID2D1GeometryImpl + ID2D1ResourceImpl {
    fn GetRoundedRect();
}
pub trait ID2D1SolidColorBrushImpl: Sized + ID2D1BrushImpl + ID2D1ResourceImpl {
    fn SetColor();
    fn GetColor();
}
pub trait ID2D1SourceTransformImpl: Sized + ID2D1TransformImpl + ID2D1TransformNodeImpl {
    fn SetRenderInfo();
    fn Draw();
}
pub trait ID2D1SpriteBatchImpl: Sized + ID2D1ResourceImpl {
    fn AddSprites();
    fn SetSprites();
    fn GetSprites();
    fn GetSpriteCount();
    fn Clear();
}
pub trait ID2D1StrokeStyleImpl: Sized + ID2D1ResourceImpl {
    fn GetStartCap();
    fn GetEndCap();
    fn GetDashCap();
    fn GetMiterLimit();
    fn GetLineJoin();
    fn GetDashOffset();
    fn GetDashStyle();
    fn GetDashesCount();
    fn GetDashes();
}
pub trait ID2D1StrokeStyle1Impl: Sized + ID2D1StrokeStyleImpl + ID2D1ResourceImpl {
    fn GetStrokeTransformType();
}
pub trait ID2D1SvgAttributeImpl: Sized + ID2D1ResourceImpl {
    fn GetElement();
    fn Clone();
}
pub trait ID2D1SvgDocumentImpl: Sized + ID2D1ResourceImpl {
    fn SetViewportSize();
    fn GetViewportSize();
    fn SetRoot();
    fn GetRoot();
    fn FindElementById();
    fn Serialize();
    fn Deserialize();
    fn CreatePaint();
    fn CreateStrokeDashArray();
    fn CreatePointCollection();
    fn CreatePathData();
}
pub trait ID2D1SvgElementImpl: Sized + ID2D1ResourceImpl {
    fn GetDocument();
    fn GetTagName();
    fn GetTagNameLength();
    fn IsTextContent();
    fn GetParent();
    fn HasChildren();
    fn GetFirstChild();
    fn GetLastChild();
    fn GetPreviousChild();
    fn GetNextChild();
    fn InsertChildBefore();
    fn AppendChild();
    fn ReplaceChild();
    fn RemoveChild();
    fn CreateChild();
    fn IsAttributeSpecified();
    fn GetSpecifiedAttributeCount();
    fn GetSpecifiedAttributeName();
    fn GetSpecifiedAttributeNameLength();
    fn RemoveAttribute();
    fn SetTextValue();
    fn GetTextValue();
    fn GetTextValueLength();
    fn SetAttributeValue();
    fn SetAttributeValue();
    fn SetAttributeValue();
    fn GetAttributeValue();
    fn GetAttributeValue();
    fn GetAttributeValue();
    fn GetAttributeValueLength();
}
pub trait ID2D1SvgGlyphStyleImpl: Sized + ID2D1ResourceImpl {
    fn SetFill();
    fn GetFill();
    fn SetStroke();
    fn GetStrokeDashesCount();
    fn GetStroke();
}
pub trait ID2D1SvgPaintImpl: Sized + ID2D1SvgAttributeImpl + ID2D1ResourceImpl {
    fn SetPaintType();
    fn GetPaintType();
    fn SetColor();
    fn GetColor();
    fn SetId();
    fn GetId();
    fn GetIdLength();
}
pub trait ID2D1SvgPathDataImpl: Sized + ID2D1SvgAttributeImpl + ID2D1ResourceImpl {
    fn RemoveSegmentDataAtEnd();
    fn UpdateSegmentData();
    fn GetSegmentData();
    fn GetSegmentDataCount();
    fn RemoveCommandsAtEnd();
    fn UpdateCommands();
    fn GetCommands();
    fn GetCommandsCount();
    fn CreatePathGeometry();
}
pub trait ID2D1SvgPointCollectionImpl: Sized + ID2D1SvgAttributeImpl + ID2D1ResourceImpl {
    fn RemovePointsAtEnd();
    fn UpdatePoints();
    fn GetPoints();
    fn GetPointsCount();
}
pub trait ID2D1SvgStrokeDashArrayImpl: Sized + ID2D1SvgAttributeImpl + ID2D1ResourceImpl {
    fn RemoveDashesAtEnd();
    fn UpdateDashes();
    fn UpdateDashes();
    fn GetDashes();
    fn GetDashes();
    fn GetDashesCount();
}
pub trait ID2D1TessellationSinkImpl: Sized {
    fn AddTriangles();
    fn Close();
}
pub trait ID2D1TransformImpl: Sized + ID2D1TransformNodeImpl {
    fn MapOutputRectToInputRects();
    fn MapInputRectsToOutputRect();
    fn MapInvalidRect();
}
pub trait ID2D1TransformGraphImpl: Sized {
    fn GetInputCount();
    fn SetSingleTransformNode();
    fn AddNode();
    fn RemoveNode();
    fn SetOutputNode();
    fn ConnectNode();
    fn ConnectToEffectInput();
    fn Clear();
    fn SetPassthroughGraph();
}
pub trait ID2D1TransformNodeImpl: Sized {
    fn GetInputCount();
}
pub trait ID2D1TransformedGeometryImpl: Sized + ID2D1GeometryImpl + ID2D1ResourceImpl {
    fn GetSourceGeometry();
    fn GetTransform();
}
pub trait ID2D1TransformedImageSourceImpl: Sized + ID2D1ImageImpl + ID2D1ResourceImpl {
    fn GetSource();
    fn GetProperties();
}
pub trait ID2D1VertexBufferImpl: Sized {
    fn Map();
    fn Unmap();
}
