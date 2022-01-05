pub trait IDynamicRendererImpl: Sized {
    fn Enabled();
    fn SetEnabled();
    fn HWND();
    fn SetHWND();
    fn ClipRectangle();
    fn SetClipRectangle();
    fn ClipRegion();
    fn SetClipRegion();
    fn DrawingAttributes();
    fn putref_DrawingAttributes();
    fn DataCacheEnabled();
    fn SetDataCacheEnabled();
    fn ReleaseCachedData();
    fn Refresh();
    fn Draw();
}
pub trait IGestureRecognizerImpl: Sized {
    fn Enabled();
    fn SetEnabled();
    fn MaxStrokeCount();
    fn SetMaxStrokeCount();
    fn EnableGestures();
    fn Reset();
}
pub trait IHandwrittenTextInsertionImpl: Sized {
    fn InsertRecognitionResultsArray();
    fn InsertInkRecognitionResult();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkCollectorImpl: Sized + IDispatchImpl {
    fn hWnd();
    fn SethWnd();
    fn Enabled();
    fn SetEnabled();
    fn DefaultDrawingAttributes();
    fn putref_DefaultDrawingAttributes();
    fn Renderer();
    fn putref_Renderer();
    fn Ink();
    fn putref_Ink();
    fn AutoRedraw();
    fn SetAutoRedraw();
    fn CollectingInk();
    fn CollectionMode();
    fn SetCollectionMode();
    fn DynamicRendering();
    fn SetDynamicRendering();
    fn DesiredPacketDescription();
    fn SetDesiredPacketDescription();
    fn MouseIcon();
    fn SetMouseIcon();
    fn putref_MouseIcon();
    fn MousePointer();
    fn SetMousePointer();
    fn Cursors();
    fn MarginX();
    fn SetMarginX();
    fn MarginY();
    fn SetMarginY();
    fn Tablet();
    fn SupportHighContrastInk();
    fn SetSupportHighContrastInk();
    fn SetGestureStatus();
    fn GetGestureStatus();
    fn GetWindowInputRectangle();
    fn SetWindowInputRectangle();
    fn SetAllTabletsMode();
    fn SetSingleTabletIntegratedMode();
    fn GetEventInterest();
    fn SetEventInterest();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkCursorImpl: Sized + IDispatchImpl {
    fn Name();
    fn Id();
    fn Inverted();
    fn DrawingAttributes();
    fn putref_DrawingAttributes();
    fn Tablet();
    fn Buttons();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkCursorButtonImpl: Sized + IDispatchImpl {
    fn Name();
    fn Id();
    fn State();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkCursorButtonsImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkCursorsImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkCustomStrokesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
    fn Add();
    fn Remove();
    fn Clear();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkDispImpl: Sized + IDispatchImpl {
    fn Strokes();
    fn ExtendedProperties();
    fn Dirty();
    fn SetDirty();
    fn CustomStrokes();
    fn GetBoundingBox();
    fn DeleteStrokes();
    fn DeleteStroke();
    fn ExtractStrokes();
    fn ExtractWithRectangle();
    fn Clip();
    fn Clone();
    fn HitTestCircle();
    fn HitTestWithRectangle();
    fn HitTestWithLasso();
    fn NearestPoint();
    fn CreateStrokes();
    fn AddStrokesAtRectangle();
    fn Save();
    fn Load();
    fn CreateStroke();
    fn ClipboardCopyWithRectangle();
    fn ClipboardCopy();
    fn CanPaste();
    fn ClipboardPaste();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkDividerImpl: Sized + IDispatchImpl {
    fn Strokes();
    fn putref_Strokes();
    fn RecognizerContext();
    fn putref_RecognizerContext();
    fn LineHeight();
    fn SetLineHeight();
    fn Divide();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkDivisionResultImpl: Sized + IDispatchImpl {
    fn Strokes();
    fn ResultByType();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkDivisionUnitImpl: Sized + IDispatchImpl {
    fn Strokes();
    fn DivisionType();
    fn RecognizedString();
    fn RotationTransform();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkDivisionUnitsImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkDrawingAttributesImpl: Sized + IDispatchImpl {
    fn Color();
    fn SetColor();
    fn Width();
    fn SetWidth();
    fn Height();
    fn SetHeight();
    fn FitToCurve();
    fn SetFitToCurve();
    fn IgnorePressure();
    fn SetIgnorePressure();
    fn AntiAliased();
    fn SetAntiAliased();
    fn Transparency();
    fn SetTransparency();
    fn RasterOperation();
    fn SetRasterOperation();
    fn PenTip();
    fn SetPenTip();
    fn ExtendedProperties();
    fn Clone();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkEditImpl: Sized + IDispatchImpl {
    fn Status();
    fn UseMouseForInput();
    fn SetUseMouseForInput();
    fn InkMode();
    fn SetInkMode();
    fn InkInsertMode();
    fn SetInkInsertMode();
    fn DrawingAttributes();
    fn putref_DrawingAttributes();
    fn RecognitionTimeout();
    fn SetRecognitionTimeout();
    fn Recognizer();
    fn putref_Recognizer();
    fn Factoid();
    fn SetFactoid();
    fn SelInks();
    fn SetSelInks();
    fn SelInksDisplayMode();
    fn SetSelInksDisplayMode();
    fn Recognize();
    fn GetGestureStatus();
    fn SetGestureStatus();
    fn SetBackColor();
    fn BackColor();
    fn Appearance();
    fn SetAppearance();
    fn BorderStyle();
    fn SetBorderStyle();
    fn Hwnd();
    fn Font();
    fn putref_Font();
    fn Text();
    fn SetText();
    fn MouseIcon();
    fn SetMouseIcon();
    fn putref_MouseIcon();
    fn MousePointer();
    fn SetMousePointer();
    fn Locked();
    fn SetLocked();
    fn Enabled();
    fn SetEnabled();
    fn MaxLength();
    fn SetMaxLength();
    fn MultiLine();
    fn SetMultiLine();
    fn ScrollBars();
    fn SetScrollBars();
    fn DisableNoScroll();
    fn SetDisableNoScroll();
    fn SelAlignment();
    fn SetSelAlignment();
    fn SelBold();
    fn SetSelBold();
    fn SelItalic();
    fn SetSelItalic();
    fn SelUnderline();
    fn SetSelUnderline();
    fn SelColor();
    fn SetSelColor();
    fn SelFontName();
    fn SetSelFontName();
    fn SelFontSize();
    fn SetSelFontSize();
    fn SelCharOffset();
    fn SetSelCharOffset();
    fn TextRTF();
    fn SetTextRTF();
    fn SelStart();
    fn SetSelStart();
    fn SelLength();
    fn SetSelLength();
    fn SelText();
    fn SetSelText();
    fn SelRTF();
    fn SetSelRTF();
    fn Refresh();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkExtendedPropertiesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Item();
    fn Add();
    fn Remove();
    fn Clear();
    fn DoesPropertyExist();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkExtendedPropertyImpl: Sized + IDispatchImpl {
    fn Guid();
    fn Data();
    fn SetData();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkGestureImpl: Sized + IDispatchImpl {
    fn Confidence();
    fn Id();
    fn GetHotPoint();
}
pub trait IInkLineInfoImpl: Sized {
    fn SetFormat();
    fn GetFormat();
    fn GetInkExtent();
    fn GetCandidate();
    fn SetCandidate();
    fn Recognize();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkOverlayImpl: Sized + IDispatchImpl {
    fn hWnd();
    fn SethWnd();
    fn Enabled();
    fn SetEnabled();
    fn DefaultDrawingAttributes();
    fn putref_DefaultDrawingAttributes();
    fn Renderer();
    fn putref_Renderer();
    fn Ink();
    fn putref_Ink();
    fn AutoRedraw();
    fn SetAutoRedraw();
    fn CollectingInk();
    fn CollectionMode();
    fn SetCollectionMode();
    fn DynamicRendering();
    fn SetDynamicRendering();
    fn DesiredPacketDescription();
    fn SetDesiredPacketDescription();
    fn MouseIcon();
    fn SetMouseIcon();
    fn putref_MouseIcon();
    fn MousePointer();
    fn SetMousePointer();
    fn EditingMode();
    fn SetEditingMode();
    fn Selection();
    fn SetSelection();
    fn EraserMode();
    fn SetEraserMode();
    fn EraserWidth();
    fn SetEraserWidth();
    fn AttachMode();
    fn SetAttachMode();
    fn Cursors();
    fn MarginX();
    fn SetMarginX();
    fn MarginY();
    fn SetMarginY();
    fn Tablet();
    fn SupportHighContrastInk();
    fn SetSupportHighContrastInk();
    fn SupportHighContrastSelectionUI();
    fn SetSupportHighContrastSelectionUI();
    fn HitTestSelection();
    fn Draw();
    fn SetGestureStatus();
    fn GetGestureStatus();
    fn GetWindowInputRectangle();
    fn SetWindowInputRectangle();
    fn SetAllTabletsMode();
    fn SetSingleTabletIntegratedMode();
    fn GetEventInterest();
    fn SetEventInterest();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkPictureImpl: Sized + IDispatchImpl {
    fn hWnd();
    fn DefaultDrawingAttributes();
    fn putref_DefaultDrawingAttributes();
    fn Renderer();
    fn putref_Renderer();
    fn Ink();
    fn putref_Ink();
    fn AutoRedraw();
    fn SetAutoRedraw();
    fn CollectingInk();
    fn CollectionMode();
    fn SetCollectionMode();
    fn DynamicRendering();
    fn SetDynamicRendering();
    fn DesiredPacketDescription();
    fn SetDesiredPacketDescription();
    fn MouseIcon();
    fn SetMouseIcon();
    fn putref_MouseIcon();
    fn MousePointer();
    fn SetMousePointer();
    fn EditingMode();
    fn SetEditingMode();
    fn Selection();
    fn SetSelection();
    fn EraserMode();
    fn SetEraserMode();
    fn EraserWidth();
    fn SetEraserWidth();
    fn putref_Picture();
    fn SetPicture();
    fn Picture();
    fn SetSizeMode();
    fn SizeMode();
    fn SetBackColor();
    fn BackColor();
    fn Cursors();
    fn MarginX();
    fn SetMarginX();
    fn MarginY();
    fn SetMarginY();
    fn Tablet();
    fn SupportHighContrastInk();
    fn SetSupportHighContrastInk();
    fn SupportHighContrastSelectionUI();
    fn SetSupportHighContrastSelectionUI();
    fn HitTestSelection();
    fn SetGestureStatus();
    fn GetGestureStatus();
    fn GetWindowInputRectangle();
    fn SetWindowInputRectangle();
    fn SetAllTabletsMode();
    fn SetSingleTabletIntegratedMode();
    fn GetEventInterest();
    fn SetEventInterest();
    fn InkEnabled();
    fn SetInkEnabled();
    fn Enabled();
    fn SetEnabled();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRecognitionAlternateImpl: Sized + IDispatchImpl {
    fn String();
    fn Confidence();
    fn Baseline();
    fn Midline();
    fn Ascender();
    fn Descender();
    fn LineNumber();
    fn Strokes();
    fn LineAlternates();
    fn ConfidenceAlternates();
    fn GetStrokesFromStrokeRanges();
    fn GetStrokesFromTextRange();
    fn GetTextRangeFromStrokes();
    fn AlternatesWithConstantPropertyValues();
    fn GetPropertyValue();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRecognitionAlternatesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Strokes();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRecognitionResultImpl: Sized + IDispatchImpl {
    fn TopString();
    fn TopAlternate();
    fn TopConfidence();
    fn Strokes();
    fn AlternatesFromSelection();
    fn ModifyTopAlternate();
    fn SetResultOnStrokes();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRecognizerImpl: Sized + IDispatchImpl {
    fn Name();
    fn Vendor();
    fn Capabilities();
    fn Languages();
    fn SupportedProperties();
    fn PreferredPacketDescription();
    fn CreateRecognizerContext();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRecognizer2Impl: Sized + IDispatchImpl {
    fn Id();
    fn UnicodeRanges();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRecognizerContextImpl: Sized + IDispatchImpl {
    fn Strokes();
    fn putref_Strokes();
    fn CharacterAutoCompletionMode();
    fn SetCharacterAutoCompletionMode();
    fn Factoid();
    fn SetFactoid();
    fn Guide();
    fn putref_Guide();
    fn PrefixText();
    fn SetPrefixText();
    fn SuffixText();
    fn SetSuffixText();
    fn RecognitionFlags();
    fn SetRecognitionFlags();
    fn WordList();
    fn putref_WordList();
    fn Recognizer();
    fn Recognize();
    fn StopBackgroundRecognition();
    fn EndInkInput();
    fn BackgroundRecognize();
    fn BackgroundRecognizeWithAlternates();
    fn Clone();
    fn IsStringSupported();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRecognizerContext2Impl: Sized + IDispatchImpl {
    fn EnabledUnicodeRanges();
    fn SetEnabledUnicodeRanges();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRecognizerGuideImpl: Sized + IDispatchImpl {
    fn WritingBox();
    fn SetWritingBox();
    fn DrawnBox();
    fn SetDrawnBox();
    fn Rows();
    fn SetRows();
    fn Columns();
    fn SetColumns();
    fn Midline();
    fn SetMidline();
    fn GuideData();
    fn SetGuideData();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRecognizersImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn GetDefaultRecognizer();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRectangleImpl: Sized + IDispatchImpl {
    fn Top();
    fn SetTop();
    fn Left();
    fn SetLeft();
    fn Bottom();
    fn SetBottom();
    fn Right();
    fn SetRight();
    fn Data();
    fn SetData();
    fn GetRectangle();
    fn SetRectangle();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkRendererImpl: Sized + IDispatchImpl {
    fn GetViewTransform();
    fn SetViewTransform();
    fn GetObjectTransform();
    fn SetObjectTransform();
    fn Draw();
    fn DrawStroke();
    fn PixelToInkSpace();
    fn InkSpaceToPixel();
    fn PixelToInkSpaceFromPoints();
    fn InkSpaceToPixelFromPoints();
    fn Measure();
    fn MeasureStroke();
    fn Move();
    fn Rotate();
    fn ScaleTransform();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkStrokeDispImpl: Sized + IDispatchImpl {
    fn ID();
    fn BezierPoints();
    fn DrawingAttributes();
    fn putref_DrawingAttributes();
    fn Ink();
    fn ExtendedProperties();
    fn PolylineCusps();
    fn BezierCusps();
    fn SelfIntersections();
    fn PacketCount();
    fn PacketSize();
    fn PacketDescription();
    fn Deleted();
    fn GetBoundingBox();
    fn FindIntersections();
    fn GetRectangleIntersections();
    fn Clip();
    fn HitTestCircle();
    fn NearestPoint();
    fn Split();
    fn GetPacketDescriptionPropertyMetrics();
    fn GetPoints();
    fn SetPoints();
    fn GetPacketData();
    fn GetPacketValuesByProperty();
    fn SetPacketValuesByProperty();
    fn GetFlattenedBezierPoints();
    fn Transform();
    fn ScaleToRectangle();
    fn Move();
    fn Rotate();
    fn Shear();
    fn ScaleTransform();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkStrokesImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Ink();
    fn RecognitionResult();
    fn ToString();
    fn Item();
    fn Add();
    fn AddStrokes();
    fn Remove();
    fn RemoveStrokes();
    fn ModifyDrawingAttributes();
    fn GetBoundingBox();
    fn Transform();
    fn ScaleToRectangle();
    fn Move();
    fn Rotate();
    fn Shear();
    fn ScaleTransform();
    fn Clip();
    fn RemoveRecognitionResult();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkTabletImpl: Sized + IDispatchImpl {
    fn Name();
    fn PlugAndPlayId();
    fn MaximumInputRectangle();
    fn HardwareCapabilities();
    fn IsPacketPropertySupported();
    fn GetPropertyMetrics();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkTablet2Impl: Sized + IDispatchImpl {
    fn DeviceKind();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkTablet3Impl: Sized + IDispatchImpl {
    fn IsMultiTouch();
    fn MaximumCursors();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkTabletsImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn DefaultTablet();
    fn Item();
    fn IsPacketPropertySupported();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkTransformImpl: Sized + IDispatchImpl {
    fn Reset();
    fn Translate();
    fn Rotate();
    fn Reflect();
    fn Shear();
    fn ScaleTransform();
    fn GetTransform();
    fn SetTransform();
    fn eM11();
    fn SeteM11();
    fn eM12();
    fn SeteM12();
    fn eM21();
    fn SeteM21();
    fn eM22();
    fn SeteM22();
    fn eDx();
    fn SeteDx();
    fn eDy();
    fn SeteDy();
    fn Data();
    fn SetData();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkWordListImpl: Sized + IDispatchImpl {
    fn AddWord();
    fn RemoveWord();
    fn Merge();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IInkWordList2Impl: Sized + IDispatchImpl {
    fn AddWords();
}
pub trait IInputPanelWindowHandleImpl: Sized {
    fn AttachedEditWindow32();
    fn SetAttachedEditWindow32();
    fn AttachedEditWindow64();
    fn SetAttachedEditWindow64();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMathInputControlImpl: Sized + IDispatchImpl {
    fn Show();
    fn Hide();
    fn IsVisible();
    fn GetPosition();
    fn SetPosition();
    fn Clear();
    fn SetCustomPaint();
    fn SetCaptionText();
    fn LoadInk();
    fn SetOwnerWindow();
    fn EnableExtendedButtons();
    fn GetPreviewHeight();
    fn SetPreviewHeight();
    fn EnableAutoGrow();
    fn AddFunctionName();
    fn RemoveFunctionName();
    fn GetHoverIcon();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPenInputPanelImpl: Sized + IDispatchImpl {
    fn Busy();
    fn Factoid();
    fn SetFactoid();
    fn AttachedEditWindow();
    fn SetAttachedEditWindow();
    fn CurrentPanel();
    fn SetCurrentPanel();
    fn DefaultPanel();
    fn SetDefaultPanel();
    fn Visible();
    fn SetVisible();
    fn Top();
    fn Left();
    fn Width();
    fn Height();
    fn VerticalOffset();
    fn SetVerticalOffset();
    fn HorizontalOffset();
    fn SetHorizontalOffset();
    fn AutoShow();
    fn SetAutoShow();
    fn MoveTo();
    fn CommitPendingInput();
    fn Refresh();
    fn EnableTsf();
}
pub trait IRealTimeStylusImpl: Sized {
    fn Enabled();
    fn SetEnabled();
    fn HWND();
    fn SetHWND();
    fn WindowInputRectangle();
    fn SetWindowInputRectangle();
    fn AddStylusSyncPlugin();
    fn RemoveStylusSyncPlugin();
    fn RemoveAllStylusSyncPlugins();
    fn GetStylusSyncPlugin();
    fn GetStylusSyncPluginCount();
    fn AddStylusAsyncPlugin();
    fn RemoveStylusAsyncPlugin();
    fn RemoveAllStylusAsyncPlugins();
    fn GetStylusAsyncPlugin();
    fn GetStylusAsyncPluginCount();
    fn ChildRealTimeStylusPlugin();
    fn putref_ChildRealTimeStylusPlugin();
    fn AddCustomStylusDataToQueue();
    fn ClearStylusQueues();
    fn SetAllTabletsMode();
    fn SetSingleTabletMode();
    fn GetTablet();
    fn GetTabletContextIdFromTablet();
    fn GetTabletFromTabletContextId();
    fn GetAllTabletContextIds();
    fn GetStyluses();
    fn GetStylusForId();
    fn SetDesiredPacketDescription();
    fn GetDesiredPacketDescription();
    fn GetPacketDescriptionData();
}
pub trait IRealTimeStylus2Impl: Sized {
    fn FlicksEnabled();
    fn SetFlicksEnabled();
}
pub trait IRealTimeStylus3Impl: Sized {
    fn MultiTouchEnabled();
    fn SetMultiTouchEnabled();
}
pub trait IRealTimeStylusSynchronizationImpl: Sized {
    fn AcquireLock();
    fn ReleaseLock();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISketchInkImpl: Sized + IDispatchImpl {}
pub trait IStrokeBuilderImpl: Sized {
    fn CreateStroke();
    fn BeginStroke();
    fn AppendPackets();
    fn EndStroke();
    fn Ink();
    fn putref_Ink();
}
pub trait IStylusAsyncPluginImpl: Sized + IStylusPluginImpl {}
pub trait IStylusPluginImpl: Sized {
    fn RealTimeStylusEnabled();
    fn RealTimeStylusDisabled();
    fn StylusInRange();
    fn StylusOutOfRange();
    fn StylusDown();
    fn StylusUp();
    fn StylusButtonDown();
    fn StylusButtonUp();
    fn InAirPackets();
    fn Packets();
    fn CustomStylusDataAdded();
    fn SystemEvent();
    fn TabletAdded();
    fn TabletRemoved();
    fn Error();
    fn UpdateMapping();
    fn DataInterest();
}
pub trait IStylusSyncPluginImpl: Sized + IStylusPluginImpl {}
pub trait ITextInputPanelImpl: Sized {
    fn AttachedEditWindow();
    fn SetAttachedEditWindow();
    fn CurrentInteractionMode();
    fn DefaultInPlaceState();
    fn SetDefaultInPlaceState();
    fn CurrentInPlaceState();
    fn DefaultInputArea();
    fn SetDefaultInputArea();
    fn CurrentInputArea();
    fn CurrentCorrectionMode();
    fn PreferredInPlaceDirection();
    fn SetPreferredInPlaceDirection();
    fn ExpandPostInsertionCorrection();
    fn SetExpandPostInsertionCorrection();
    fn InPlaceVisibleOnFocus();
    fn SetInPlaceVisibleOnFocus();
    fn InPlaceBoundingRectangle();
    fn PopUpCorrectionHeight();
    fn PopDownCorrectionHeight();
    fn CommitPendingInput();
    fn SetInPlaceVisibility();
    fn SetInPlacePosition();
    fn SetInPlaceHoverTargetPosition();
    fn Advise();
    fn Unadvise();
}
pub trait ITextInputPanelEventSinkImpl: Sized {
    fn InPlaceStateChanging();
    fn InPlaceStateChanged();
    fn InPlaceSizeChanging();
    fn InPlaceSizeChanged();
    fn InputAreaChanging();
    fn InputAreaChanged();
    fn CorrectionModeChanging();
    fn CorrectionModeChanged();
    fn InPlaceVisibilityChanging();
    fn InPlaceVisibilityChanged();
    fn TextInserting();
    fn TextInserted();
}
pub trait ITextInputPanelRunInfoImpl: Sized {
    fn IsTipRunning();
}
pub trait ITipAutoCompleteClientImpl: Sized {
    fn AdviseProvider();
    fn UnadviseProvider();
    fn UserSelection();
    fn PreferredRects();
    fn RequestShowUI();
}
pub trait ITipAutoCompleteProviderImpl: Sized {
    fn UpdatePendingText();
    fn Show();
}
#[cfg(feature = "Win32_System_Com")]
pub trait _IInkCollectorEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait _IInkEditEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait _IInkEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait _IInkOverlayEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait _IInkPictureEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait _IInkRecognitionEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait _IInkStrokesEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait _IMathInputControlEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait _IPenInputPanelEventsImpl: Sized + IDispatchImpl {}
