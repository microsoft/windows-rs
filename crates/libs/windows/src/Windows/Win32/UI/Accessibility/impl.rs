pub trait IAccIdentityImpl: Sized {
    fn GetIdentityString();
}
pub trait IAccPropServerImpl: Sized {
    fn GetPropValue();
}
pub trait IAccPropServicesImpl: Sized {
    fn SetPropValue();
    fn SetPropServer();
    fn ClearProps();
    fn SetHwndProp();
    fn SetHwndPropStr();
    fn SetHwndPropServer();
    fn ClearHwndProps();
    fn ComposeHwndIdentityString();
    fn DecomposeHwndIdentityString();
    fn SetHmenuProp();
    fn SetHmenuPropStr();
    fn SetHmenuPropServer();
    fn ClearHmenuProps();
    fn ComposeHmenuIdentityString();
    fn DecomposeHmenuIdentityString();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAccessibleImpl: Sized + IDispatchImpl {
    fn accParent();
    fn accChildCount();
    fn accChild();
    fn accName();
    fn accValue();
    fn accDescription();
    fn accRole();
    fn accState();
    fn accHelp();
    fn accHelpTopic();
    fn accKeyboardShortcut();
    fn accFocus();
    fn accSelection();
    fn accDefaultAction();
    fn accSelect();
    fn accLocation();
    fn accNavigate();
    fn accHitTest();
    fn accDoDefaultAction();
    fn SetaccName();
    fn SetaccValue();
}
pub trait IAccessibleExImpl: Sized {
    fn GetObjectForChild();
    fn GetIAccessiblePair();
    fn GetRuntimeId();
    fn ConvertReturnedElement();
}
pub trait IAccessibleHandlerImpl: Sized {
    fn AccessibleObjectFromID();
}
pub trait IAccessibleHostingElementProvidersImpl: Sized {
    fn GetEmbeddedFragmentRoots();
    fn GetObjectIdForProvider();
}
pub trait IAccessibleWindowlessSiteImpl: Sized {
    fn AcquireObjectIdRange();
    fn ReleaseObjectIdRange();
    fn QueryObjectIdRanges();
    fn GetParentAccessible();
}
pub trait IAnnotationProviderImpl: Sized {
    fn AnnotationTypeId();
    fn AnnotationTypeName();
    fn Author();
    fn DateTime();
    fn Target();
}
pub trait ICustomNavigationProviderImpl: Sized {
    fn Navigate();
}
pub trait IDockProviderImpl: Sized {
    fn SetDockPosition();
    fn DockPosition();
}
pub trait IDragProviderImpl: Sized {
    fn IsGrabbed();
    fn DropEffect();
    fn DropEffects();
    fn GetGrabbedItems();
}
pub trait IDropTargetProviderImpl: Sized {
    fn DropTargetEffect();
    fn DropTargetEffects();
}
pub trait IExpandCollapseProviderImpl: Sized {
    fn Expand();
    fn Collapse();
    fn ExpandCollapseState();
}
pub trait IGridItemProviderImpl: Sized {
    fn Row();
    fn Column();
    fn RowSpan();
    fn ColumnSpan();
    fn ContainingGrid();
}
pub trait IGridProviderImpl: Sized {
    fn GetItem();
    fn RowCount();
    fn ColumnCount();
}
pub trait IInvokeProviderImpl: Sized {
    fn Invoke();
}
pub trait IItemContainerProviderImpl: Sized {
    fn FindItemByProperty();
}
pub trait ILegacyIAccessibleProviderImpl: Sized {
    fn Select();
    fn DoDefaultAction();
    fn SetValue();
    fn GetIAccessible();
    fn ChildId();
    fn Name();
    fn Value();
    fn Description();
    fn Role();
    fn State();
    fn Help();
    fn KeyboardShortcut();
    fn GetSelection();
    fn DefaultAction();
}
pub trait IMultipleViewProviderImpl: Sized {
    fn GetViewName();
    fn SetCurrentView();
    fn CurrentView();
    fn GetSupportedViews();
}
pub trait IObjectModelProviderImpl: Sized {
    fn GetUnderlyingObjectModel();
}
pub trait IProxyProviderWinEventHandlerImpl: Sized {
    fn RespondToWinEvent();
}
pub trait IProxyProviderWinEventSinkImpl: Sized {
    fn AddAutomationPropertyChangedEvent();
    fn AddAutomationEvent();
    fn AddStructureChangedEvent();
}
pub trait IRangeValueProviderImpl: Sized {
    fn SetValue();
    fn Value();
    fn IsReadOnly();
    fn Maximum();
    fn Minimum();
    fn LargeChange();
    fn SmallChange();
}
pub trait IRawElementProviderAdviseEventsImpl: Sized {
    fn AdviseEventAdded();
    fn AdviseEventRemoved();
}
pub trait IRawElementProviderFragmentImpl: Sized {
    fn Navigate();
    fn GetRuntimeId();
    fn BoundingRectangle();
    fn GetEmbeddedFragmentRoots();
    fn SetFocus();
    fn FragmentRoot();
}
pub trait IRawElementProviderFragmentRootImpl: Sized {
    fn ElementProviderFromPoint();
    fn GetFocus();
}
pub trait IRawElementProviderHostingAccessiblesImpl: Sized {
    fn GetEmbeddedAccessibles();
}
pub trait IRawElementProviderHwndOverrideImpl: Sized {
    fn GetOverrideProviderForHwnd();
}
pub trait IRawElementProviderSimpleImpl: Sized {
    fn ProviderOptions();
    fn GetPatternProvider();
    fn GetPropertyValue();
    fn HostRawElementProvider();
}
pub trait IRawElementProviderSimple2Impl: Sized + IRawElementProviderSimpleImpl {
    fn ShowContextMenu();
}
pub trait IRawElementProviderSimple3Impl: Sized + IRawElementProviderSimple2Impl + IRawElementProviderSimpleImpl {
    fn GetMetadataValue();
}
pub trait IRawElementProviderWindowlessSiteImpl: Sized {
    fn GetAdjacentFragment();
    fn GetRuntimeIdPrefix();
}
pub trait IRichEditUiaInformationImpl: Sized {
    fn GetBoundaryRectangle();
    fn IsVisible();
}
pub trait IRicheditWindowlessAccessibilityImpl: Sized {
    fn CreateProvider();
}
pub trait IScrollItemProviderImpl: Sized {
    fn ScrollIntoView();
}
pub trait IScrollProviderImpl: Sized {
    fn Scroll();
    fn SetScrollPercent();
    fn HorizontalScrollPercent();
    fn VerticalScrollPercent();
    fn HorizontalViewSize();
    fn VerticalViewSize();
    fn HorizontallyScrollable();
    fn VerticallyScrollable();
}
pub trait ISelectionItemProviderImpl: Sized {
    fn Select();
    fn AddToSelection();
    fn RemoveFromSelection();
    fn IsSelected();
    fn SelectionContainer();
}
pub trait ISelectionProviderImpl: Sized {
    fn GetSelection();
    fn CanSelectMultiple();
    fn IsSelectionRequired();
}
pub trait ISelectionProvider2Impl: Sized + ISelectionProviderImpl {
    fn FirstSelectedItem();
    fn LastSelectedItem();
    fn CurrentSelectedItem();
    fn ItemCount();
}
pub trait ISpreadsheetItemProviderImpl: Sized {
    fn Formula();
    fn GetAnnotationObjects();
    fn GetAnnotationTypes();
}
pub trait ISpreadsheetProviderImpl: Sized {
    fn GetItemByName();
}
pub trait IStylesProviderImpl: Sized {
    fn StyleId();
    fn StyleName();
    fn FillColor();
    fn FillPatternStyle();
    fn Shape();
    fn FillPatternColor();
    fn ExtendedProperties();
}
pub trait ISynchronizedInputProviderImpl: Sized {
    fn StartListening();
    fn Cancel();
}
pub trait ITableItemProviderImpl: Sized {
    fn GetRowHeaderItems();
    fn GetColumnHeaderItems();
}
pub trait ITableProviderImpl: Sized {
    fn GetRowHeaders();
    fn GetColumnHeaders();
    fn RowOrColumnMajor();
}
pub trait ITextChildProviderImpl: Sized {
    fn TextContainer();
    fn TextRange();
}
pub trait ITextEditProviderImpl: Sized + ITextProviderImpl {
    fn GetActiveComposition();
    fn GetConversionTarget();
}
pub trait ITextProviderImpl: Sized {
    fn GetSelection();
    fn GetVisibleRanges();
    fn RangeFromChild();
    fn RangeFromPoint();
    fn DocumentRange();
    fn SupportedTextSelection();
}
pub trait ITextProvider2Impl: Sized + ITextProviderImpl {
    fn RangeFromAnnotation();
    fn GetCaretRange();
}
pub trait ITextRangeProviderImpl: Sized {
    fn Clone();
    fn Compare();
    fn CompareEndpoints();
    fn ExpandToEnclosingUnit();
    fn FindAttribute();
    fn FindText();
    fn GetAttributeValue();
    fn GetBoundingRectangles();
    fn GetEnclosingElement();
    fn GetText();
    fn Move();
    fn MoveEndpointByUnit();
    fn MoveEndpointByRange();
    fn Select();
    fn AddToSelection();
    fn RemoveFromSelection();
    fn ScrollIntoView();
    fn GetChildren();
}
pub trait ITextRangeProvider2Impl: Sized + ITextRangeProviderImpl {
    fn ShowContextMenu();
}
pub trait IToggleProviderImpl: Sized {
    fn Toggle();
    fn ToggleState();
}
pub trait ITransformProviderImpl: Sized {
    fn Move();
    fn Resize();
    fn Rotate();
    fn CanMove();
    fn CanResize();
    fn CanRotate();
}
pub trait ITransformProvider2Impl: Sized + ITransformProviderImpl {
    fn Zoom();
    fn CanZoom();
    fn ZoomLevel();
    fn ZoomMinimum();
    fn ZoomMaximum();
    fn ZoomByUnit();
}
pub trait IUIAutomationImpl: Sized {
    fn CompareElements();
    fn CompareRuntimeIds();
    fn GetRootElement();
    fn ElementFromHandle();
    fn ElementFromPoint();
    fn GetFocusedElement();
    fn GetRootElementBuildCache();
    fn ElementFromHandleBuildCache();
    fn ElementFromPointBuildCache();
    fn GetFocusedElementBuildCache();
    fn CreateTreeWalker();
    fn ControlViewWalker();
    fn ContentViewWalker();
    fn RawViewWalker();
    fn RawViewCondition();
    fn ControlViewCondition();
    fn ContentViewCondition();
    fn CreateCacheRequest();
    fn CreateTrueCondition();
    fn CreateFalseCondition();
    fn CreatePropertyCondition();
    fn CreatePropertyConditionEx();
    fn CreateAndCondition();
    fn CreateAndConditionFromArray();
    fn CreateAndConditionFromNativeArray();
    fn CreateOrCondition();
    fn CreateOrConditionFromArray();
    fn CreateOrConditionFromNativeArray();
    fn CreateNotCondition();
    fn AddAutomationEventHandler();
    fn RemoveAutomationEventHandler();
    fn AddPropertyChangedEventHandlerNativeArray();
    fn AddPropertyChangedEventHandler();
    fn RemovePropertyChangedEventHandler();
    fn AddStructureChangedEventHandler();
    fn RemoveStructureChangedEventHandler();
    fn AddFocusChangedEventHandler();
    fn RemoveFocusChangedEventHandler();
    fn RemoveAllEventHandlers();
    fn IntNativeArrayToSafeArray();
    fn IntSafeArrayToNativeArray();
    fn RectToVariant();
    fn VariantToRect();
    fn SafeArrayToRectNativeArray();
    fn CreateProxyFactoryEntry();
    fn ProxyFactoryMapping();
    fn GetPropertyProgrammaticName();
    fn GetPatternProgrammaticName();
    fn PollForPotentialSupportedPatterns();
    fn PollForPotentialSupportedProperties();
    fn CheckNotSupported();
    fn ReservedNotSupportedValue();
    fn ReservedMixedAttributeValue();
    fn ElementFromIAccessible();
    fn ElementFromIAccessibleBuildCache();
}
pub trait IUIAutomation2Impl: Sized + IUIAutomationImpl {
    fn AutoSetFocus();
    fn SetAutoSetFocus();
    fn ConnectionTimeout();
    fn SetConnectionTimeout();
    fn TransactionTimeout();
    fn SetTransactionTimeout();
}
pub trait IUIAutomation3Impl: Sized + IUIAutomation2Impl + IUIAutomationImpl {
    fn AddTextEditTextChangedEventHandler();
    fn RemoveTextEditTextChangedEventHandler();
}
pub trait IUIAutomation4Impl: Sized + IUIAutomation3Impl + IUIAutomation2Impl + IUIAutomationImpl {
    fn AddChangesEventHandler();
    fn RemoveChangesEventHandler();
}
pub trait IUIAutomation5Impl: Sized + IUIAutomation4Impl + IUIAutomation3Impl + IUIAutomation2Impl + IUIAutomationImpl {
    fn AddNotificationEventHandler();
    fn RemoveNotificationEventHandler();
}
pub trait IUIAutomation6Impl: Sized + IUIAutomation5Impl + IUIAutomation4Impl + IUIAutomation3Impl + IUIAutomation2Impl + IUIAutomationImpl {
    fn CreateEventHandlerGroup();
    fn AddEventHandlerGroup();
    fn RemoveEventHandlerGroup();
    fn ConnectionRecoveryBehavior();
    fn SetConnectionRecoveryBehavior();
    fn CoalesceEvents();
    fn SetCoalesceEvents();
    fn AddActiveTextPositionChangedEventHandler();
    fn RemoveActiveTextPositionChangedEventHandler();
}
pub trait IUIAutomationActiveTextPositionChangedEventHandlerImpl: Sized {
    fn HandleActiveTextPositionChangedEvent();
}
pub trait IUIAutomationAndConditionImpl: Sized + IUIAutomationConditionImpl {
    fn ChildCount();
    fn GetChildrenAsNativeArray();
    fn GetChildren();
}
pub trait IUIAutomationAnnotationPatternImpl: Sized {
    fn CurrentAnnotationTypeId();
    fn CurrentAnnotationTypeName();
    fn CurrentAuthor();
    fn CurrentDateTime();
    fn CurrentTarget();
    fn CachedAnnotationTypeId();
    fn CachedAnnotationTypeName();
    fn CachedAuthor();
    fn CachedDateTime();
    fn CachedTarget();
}
pub trait IUIAutomationBoolConditionImpl: Sized + IUIAutomationConditionImpl {
    fn BooleanValue();
}
pub trait IUIAutomationCacheRequestImpl: Sized {
    fn AddProperty();
    fn AddPattern();
    fn Clone();
    fn TreeScope();
    fn SetTreeScope();
    fn TreeFilter();
    fn SetTreeFilter();
    fn AutomationElementMode();
    fn SetAutomationElementMode();
}
pub trait IUIAutomationChangesEventHandlerImpl: Sized {
    fn HandleChangesEvent();
}
pub trait IUIAutomationConditionImpl: Sized {}
pub trait IUIAutomationCustomNavigationPatternImpl: Sized {
    fn Navigate();
}
pub trait IUIAutomationDockPatternImpl: Sized {
    fn SetDockPosition();
    fn CurrentDockPosition();
    fn CachedDockPosition();
}
pub trait IUIAutomationDragPatternImpl: Sized {
    fn CurrentIsGrabbed();
    fn CachedIsGrabbed();
    fn CurrentDropEffect();
    fn CachedDropEffect();
    fn CurrentDropEffects();
    fn CachedDropEffects();
    fn GetCurrentGrabbedItems();
    fn GetCachedGrabbedItems();
}
pub trait IUIAutomationDropTargetPatternImpl: Sized {
    fn CurrentDropTargetEffect();
    fn CachedDropTargetEffect();
    fn CurrentDropTargetEffects();
    fn CachedDropTargetEffects();
}
pub trait IUIAutomationElementImpl: Sized {
    fn SetFocus();
    fn GetRuntimeId();
    fn FindFirst();
    fn FindAll();
    fn FindFirstBuildCache();
    fn FindAllBuildCache();
    fn BuildUpdatedCache();
    fn GetCurrentPropertyValue();
    fn GetCurrentPropertyValueEx();
    fn GetCachedPropertyValue();
    fn GetCachedPropertyValueEx();
    fn GetCurrentPatternAs();
    fn GetCachedPatternAs();
    fn GetCurrentPattern();
    fn GetCachedPattern();
    fn GetCachedParent();
    fn GetCachedChildren();
    fn CurrentProcessId();
    fn CurrentControlType();
    fn CurrentLocalizedControlType();
    fn CurrentName();
    fn CurrentAcceleratorKey();
    fn CurrentAccessKey();
    fn CurrentHasKeyboardFocus();
    fn CurrentIsKeyboardFocusable();
    fn CurrentIsEnabled();
    fn CurrentAutomationId();
    fn CurrentClassName();
    fn CurrentHelpText();
    fn CurrentCulture();
    fn CurrentIsControlElement();
    fn CurrentIsContentElement();
    fn CurrentIsPassword();
    fn CurrentNativeWindowHandle();
    fn CurrentItemType();
    fn CurrentIsOffscreen();
    fn CurrentOrientation();
    fn CurrentFrameworkId();
    fn CurrentIsRequiredForForm();
    fn CurrentItemStatus();
    fn CurrentBoundingRectangle();
    fn CurrentLabeledBy();
    fn CurrentAriaRole();
    fn CurrentAriaProperties();
    fn CurrentIsDataValidForForm();
    fn CurrentControllerFor();
    fn CurrentDescribedBy();
    fn CurrentFlowsTo();
    fn CurrentProviderDescription();
    fn CachedProcessId();
    fn CachedControlType();
    fn CachedLocalizedControlType();
    fn CachedName();
    fn CachedAcceleratorKey();
    fn CachedAccessKey();
    fn CachedHasKeyboardFocus();
    fn CachedIsKeyboardFocusable();
    fn CachedIsEnabled();
    fn CachedAutomationId();
    fn CachedClassName();
    fn CachedHelpText();
    fn CachedCulture();
    fn CachedIsControlElement();
    fn CachedIsContentElement();
    fn CachedIsPassword();
    fn CachedNativeWindowHandle();
    fn CachedItemType();
    fn CachedIsOffscreen();
    fn CachedOrientation();
    fn CachedFrameworkId();
    fn CachedIsRequiredForForm();
    fn CachedItemStatus();
    fn CachedBoundingRectangle();
    fn CachedLabeledBy();
    fn CachedAriaRole();
    fn CachedAriaProperties();
    fn CachedIsDataValidForForm();
    fn CachedControllerFor();
    fn CachedDescribedBy();
    fn CachedFlowsTo();
    fn CachedProviderDescription();
    fn GetClickablePoint();
}
pub trait IUIAutomationElement2Impl: Sized + IUIAutomationElementImpl {
    fn CurrentOptimizeForVisualContent();
    fn CachedOptimizeForVisualContent();
    fn CurrentLiveSetting();
    fn CachedLiveSetting();
    fn CurrentFlowsFrom();
    fn CachedFlowsFrom();
}
pub trait IUIAutomationElement3Impl: Sized + IUIAutomationElement2Impl + IUIAutomationElementImpl {
    fn ShowContextMenu();
    fn CurrentIsPeripheral();
    fn CachedIsPeripheral();
}
pub trait IUIAutomationElement4Impl: Sized + IUIAutomationElement3Impl + IUIAutomationElement2Impl + IUIAutomationElementImpl {
    fn CurrentPositionInSet();
    fn CurrentSizeOfSet();
    fn CurrentLevel();
    fn CurrentAnnotationTypes();
    fn CurrentAnnotationObjects();
    fn CachedPositionInSet();
    fn CachedSizeOfSet();
    fn CachedLevel();
    fn CachedAnnotationTypes();
    fn CachedAnnotationObjects();
}
pub trait IUIAutomationElement5Impl: Sized + IUIAutomationElement4Impl + IUIAutomationElement3Impl + IUIAutomationElement2Impl + IUIAutomationElementImpl {
    fn CurrentLandmarkType();
    fn CurrentLocalizedLandmarkType();
    fn CachedLandmarkType();
    fn CachedLocalizedLandmarkType();
}
pub trait IUIAutomationElement6Impl: Sized + IUIAutomationElement5Impl + IUIAutomationElement4Impl + IUIAutomationElement3Impl + IUIAutomationElement2Impl + IUIAutomationElementImpl {
    fn CurrentFullDescription();
    fn CachedFullDescription();
}
pub trait IUIAutomationElement7Impl: Sized + IUIAutomationElement6Impl + IUIAutomationElement5Impl + IUIAutomationElement4Impl + IUIAutomationElement3Impl + IUIAutomationElement2Impl + IUIAutomationElementImpl {
    fn FindFirstWithOptions();
    fn FindAllWithOptions();
    fn FindFirstWithOptionsBuildCache();
    fn FindAllWithOptionsBuildCache();
    fn GetCurrentMetadataValue();
}
pub trait IUIAutomationElement8Impl: Sized + IUIAutomationElement7Impl + IUIAutomationElement6Impl + IUIAutomationElement5Impl + IUIAutomationElement4Impl + IUIAutomationElement3Impl + IUIAutomationElement2Impl + IUIAutomationElementImpl {
    fn CurrentHeadingLevel();
    fn CachedHeadingLevel();
}
pub trait IUIAutomationElement9Impl: Sized + IUIAutomationElement8Impl + IUIAutomationElement7Impl + IUIAutomationElement6Impl + IUIAutomationElement5Impl + IUIAutomationElement4Impl + IUIAutomationElement3Impl + IUIAutomationElement2Impl + IUIAutomationElementImpl {
    fn CurrentIsDialog();
    fn CachedIsDialog();
}
pub trait IUIAutomationElementArrayImpl: Sized {
    fn Length();
    fn GetElement();
}
pub trait IUIAutomationEventHandlerImpl: Sized {
    fn HandleAutomationEvent();
}
pub trait IUIAutomationEventHandlerGroupImpl: Sized {
    fn AddActiveTextPositionChangedEventHandler();
    fn AddAutomationEventHandler();
    fn AddChangesEventHandler();
    fn AddNotificationEventHandler();
    fn AddPropertyChangedEventHandler();
    fn AddStructureChangedEventHandler();
    fn AddTextEditTextChangedEventHandler();
}
pub trait IUIAutomationExpandCollapsePatternImpl: Sized {
    fn Expand();
    fn Collapse();
    fn CurrentExpandCollapseState();
    fn CachedExpandCollapseState();
}
pub trait IUIAutomationFocusChangedEventHandlerImpl: Sized {
    fn HandleFocusChangedEvent();
}
pub trait IUIAutomationGridItemPatternImpl: Sized {
    fn CurrentContainingGrid();
    fn CurrentRow();
    fn CurrentColumn();
    fn CurrentRowSpan();
    fn CurrentColumnSpan();
    fn CachedContainingGrid();
    fn CachedRow();
    fn CachedColumn();
    fn CachedRowSpan();
    fn CachedColumnSpan();
}
pub trait IUIAutomationGridPatternImpl: Sized {
    fn GetItem();
    fn CurrentRowCount();
    fn CurrentColumnCount();
    fn CachedRowCount();
    fn CachedColumnCount();
}
pub trait IUIAutomationInvokePatternImpl: Sized {
    fn Invoke();
}
pub trait IUIAutomationItemContainerPatternImpl: Sized {
    fn FindItemByProperty();
}
pub trait IUIAutomationLegacyIAccessiblePatternImpl: Sized {
    fn Select();
    fn DoDefaultAction();
    fn SetValue();
    fn CurrentChildId();
    fn CurrentName();
    fn CurrentValue();
    fn CurrentDescription();
    fn CurrentRole();
    fn CurrentState();
    fn CurrentHelp();
    fn CurrentKeyboardShortcut();
    fn GetCurrentSelection();
    fn CurrentDefaultAction();
    fn CachedChildId();
    fn CachedName();
    fn CachedValue();
    fn CachedDescription();
    fn CachedRole();
    fn CachedState();
    fn CachedHelp();
    fn CachedKeyboardShortcut();
    fn GetCachedSelection();
    fn CachedDefaultAction();
    fn GetIAccessible();
}
pub trait IUIAutomationMultipleViewPatternImpl: Sized {
    fn GetViewName();
    fn SetCurrentView();
    fn CurrentCurrentView();
    fn GetCurrentSupportedViews();
    fn CachedCurrentView();
    fn GetCachedSupportedViews();
}
pub trait IUIAutomationNotConditionImpl: Sized + IUIAutomationConditionImpl {
    fn GetChild();
}
pub trait IUIAutomationNotificationEventHandlerImpl: Sized {
    fn HandleNotificationEvent();
}
pub trait IUIAutomationObjectModelPatternImpl: Sized {
    fn GetUnderlyingObjectModel();
}
pub trait IUIAutomationOrConditionImpl: Sized + IUIAutomationConditionImpl {
    fn ChildCount();
    fn GetChildrenAsNativeArray();
    fn GetChildren();
}
pub trait IUIAutomationPatternHandlerImpl: Sized {
    fn CreateClientWrapper();
    fn Dispatch();
}
pub trait IUIAutomationPatternInstanceImpl: Sized {
    fn GetProperty();
    fn CallMethod();
}
pub trait IUIAutomationPropertyChangedEventHandlerImpl: Sized {
    fn HandlePropertyChangedEvent();
}
pub trait IUIAutomationPropertyConditionImpl: Sized + IUIAutomationConditionImpl {
    fn PropertyId();
    fn PropertyValue();
    fn PropertyConditionFlags();
}
pub trait IUIAutomationProxyFactoryImpl: Sized {
    fn CreateProvider();
    fn ProxyFactoryId();
}
pub trait IUIAutomationProxyFactoryEntryImpl: Sized {
    fn ProxyFactory();
    fn ClassName();
    fn ImageName();
    fn AllowSubstringMatch();
    fn CanCheckBaseClass();
    fn NeedsAdviseEvents();
    fn SetClassName();
    fn SetImageName();
    fn SetAllowSubstringMatch();
    fn SetCanCheckBaseClass();
    fn SetNeedsAdviseEvents();
    fn SetWinEventsForAutomationEvent();
    fn GetWinEventsForAutomationEvent();
}
pub trait IUIAutomationProxyFactoryMappingImpl: Sized {
    fn Count();
    fn GetTable();
    fn GetEntry();
    fn SetTable();
    fn InsertEntries();
    fn InsertEntry();
    fn RemoveEntry();
    fn ClearTable();
    fn RestoreDefaultTable();
}
pub trait IUIAutomationRangeValuePatternImpl: Sized {
    fn SetValue();
    fn CurrentValue();
    fn CurrentIsReadOnly();
    fn CurrentMaximum();
    fn CurrentMinimum();
    fn CurrentLargeChange();
    fn CurrentSmallChange();
    fn CachedValue();
    fn CachedIsReadOnly();
    fn CachedMaximum();
    fn CachedMinimum();
    fn CachedLargeChange();
    fn CachedSmallChange();
}
pub trait IUIAutomationRegistrarImpl: Sized {
    fn RegisterProperty();
    fn RegisterEvent();
    fn RegisterPattern();
}
pub trait IUIAutomationScrollItemPatternImpl: Sized {
    fn ScrollIntoView();
}
pub trait IUIAutomationScrollPatternImpl: Sized {
    fn Scroll();
    fn SetScrollPercent();
    fn CurrentHorizontalScrollPercent();
    fn CurrentVerticalScrollPercent();
    fn CurrentHorizontalViewSize();
    fn CurrentVerticalViewSize();
    fn CurrentHorizontallyScrollable();
    fn CurrentVerticallyScrollable();
    fn CachedHorizontalScrollPercent();
    fn CachedVerticalScrollPercent();
    fn CachedHorizontalViewSize();
    fn CachedVerticalViewSize();
    fn CachedHorizontallyScrollable();
    fn CachedVerticallyScrollable();
}
pub trait IUIAutomationSelectionItemPatternImpl: Sized {
    fn Select();
    fn AddToSelection();
    fn RemoveFromSelection();
    fn CurrentIsSelected();
    fn CurrentSelectionContainer();
    fn CachedIsSelected();
    fn CachedSelectionContainer();
}
pub trait IUIAutomationSelectionPatternImpl: Sized {
    fn GetCurrentSelection();
    fn CurrentCanSelectMultiple();
    fn CurrentIsSelectionRequired();
    fn GetCachedSelection();
    fn CachedCanSelectMultiple();
    fn CachedIsSelectionRequired();
}
pub trait IUIAutomationSelectionPattern2Impl: Sized + IUIAutomationSelectionPatternImpl {
    fn CurrentFirstSelectedItem();
    fn CurrentLastSelectedItem();
    fn CurrentCurrentSelectedItem();
    fn CurrentItemCount();
    fn CachedFirstSelectedItem();
    fn CachedLastSelectedItem();
    fn CachedCurrentSelectedItem();
    fn CachedItemCount();
}
pub trait IUIAutomationSpreadsheetItemPatternImpl: Sized {
    fn CurrentFormula();
    fn GetCurrentAnnotationObjects();
    fn GetCurrentAnnotationTypes();
    fn CachedFormula();
    fn GetCachedAnnotationObjects();
    fn GetCachedAnnotationTypes();
}
pub trait IUIAutomationSpreadsheetPatternImpl: Sized {
    fn GetItemByName();
}
pub trait IUIAutomationStructureChangedEventHandlerImpl: Sized {
    fn HandleStructureChangedEvent();
}
pub trait IUIAutomationStylesPatternImpl: Sized {
    fn CurrentStyleId();
    fn CurrentStyleName();
    fn CurrentFillColor();
    fn CurrentFillPatternStyle();
    fn CurrentShape();
    fn CurrentFillPatternColor();
    fn CurrentExtendedProperties();
    fn GetCurrentExtendedPropertiesAsArray();
    fn CachedStyleId();
    fn CachedStyleName();
    fn CachedFillColor();
    fn CachedFillPatternStyle();
    fn CachedShape();
    fn CachedFillPatternColor();
    fn CachedExtendedProperties();
    fn GetCachedExtendedPropertiesAsArray();
}
pub trait IUIAutomationSynchronizedInputPatternImpl: Sized {
    fn StartListening();
    fn Cancel();
}
pub trait IUIAutomationTableItemPatternImpl: Sized {
    fn GetCurrentRowHeaderItems();
    fn GetCurrentColumnHeaderItems();
    fn GetCachedRowHeaderItems();
    fn GetCachedColumnHeaderItems();
}
pub trait IUIAutomationTablePatternImpl: Sized {
    fn GetCurrentRowHeaders();
    fn GetCurrentColumnHeaders();
    fn CurrentRowOrColumnMajor();
    fn GetCachedRowHeaders();
    fn GetCachedColumnHeaders();
    fn CachedRowOrColumnMajor();
}
pub trait IUIAutomationTextChildPatternImpl: Sized {
    fn TextContainer();
    fn TextRange();
}
pub trait IUIAutomationTextEditPatternImpl: Sized + IUIAutomationTextPatternImpl {
    fn GetActiveComposition();
    fn GetConversionTarget();
}
pub trait IUIAutomationTextEditTextChangedEventHandlerImpl: Sized {
    fn HandleTextEditTextChangedEvent();
}
pub trait IUIAutomationTextPatternImpl: Sized {
    fn RangeFromPoint();
    fn RangeFromChild();
    fn GetSelection();
    fn GetVisibleRanges();
    fn DocumentRange();
    fn SupportedTextSelection();
}
pub trait IUIAutomationTextPattern2Impl: Sized + IUIAutomationTextPatternImpl {
    fn RangeFromAnnotation();
    fn GetCaretRange();
}
pub trait IUIAutomationTextRangeImpl: Sized {
    fn Clone();
    fn Compare();
    fn CompareEndpoints();
    fn ExpandToEnclosingUnit();
    fn FindAttribute();
    fn FindText();
    fn GetAttributeValue();
    fn GetBoundingRectangles();
    fn GetEnclosingElement();
    fn GetText();
    fn Move();
    fn MoveEndpointByUnit();
    fn MoveEndpointByRange();
    fn Select();
    fn AddToSelection();
    fn RemoveFromSelection();
    fn ScrollIntoView();
    fn GetChildren();
}
pub trait IUIAutomationTextRange2Impl: Sized + IUIAutomationTextRangeImpl {
    fn ShowContextMenu();
}
pub trait IUIAutomationTextRange3Impl: Sized + IUIAutomationTextRange2Impl + IUIAutomationTextRangeImpl {
    fn GetEnclosingElementBuildCache();
    fn GetChildrenBuildCache();
    fn GetAttributeValues();
}
pub trait IUIAutomationTextRangeArrayImpl: Sized {
    fn Length();
    fn GetElement();
}
pub trait IUIAutomationTogglePatternImpl: Sized {
    fn Toggle();
    fn CurrentToggleState();
    fn CachedToggleState();
}
pub trait IUIAutomationTransformPatternImpl: Sized {
    fn Move();
    fn Resize();
    fn Rotate();
    fn CurrentCanMove();
    fn CurrentCanResize();
    fn CurrentCanRotate();
    fn CachedCanMove();
    fn CachedCanResize();
    fn CachedCanRotate();
}
pub trait IUIAutomationTransformPattern2Impl: Sized + IUIAutomationTransformPatternImpl {
    fn Zoom();
    fn ZoomByUnit();
    fn CurrentCanZoom();
    fn CachedCanZoom();
    fn CurrentZoomLevel();
    fn CachedZoomLevel();
    fn CurrentZoomMinimum();
    fn CachedZoomMinimum();
    fn CurrentZoomMaximum();
    fn CachedZoomMaximum();
}
pub trait IUIAutomationTreeWalkerImpl: Sized {
    fn GetParentElement();
    fn GetFirstChildElement();
    fn GetLastChildElement();
    fn GetNextSiblingElement();
    fn GetPreviousSiblingElement();
    fn NormalizeElement();
    fn GetParentElementBuildCache();
    fn GetFirstChildElementBuildCache();
    fn GetLastChildElementBuildCache();
    fn GetNextSiblingElementBuildCache();
    fn GetPreviousSiblingElementBuildCache();
    fn NormalizeElementBuildCache();
    fn Condition();
}
pub trait IUIAutomationValuePatternImpl: Sized {
    fn SetValue();
    fn CurrentValue();
    fn CurrentIsReadOnly();
    fn CachedValue();
    fn CachedIsReadOnly();
}
pub trait IUIAutomationVirtualizedItemPatternImpl: Sized {
    fn Realize();
}
pub trait IUIAutomationWindowPatternImpl: Sized {
    fn Close();
    fn WaitForInputIdle();
    fn SetWindowVisualState();
    fn CurrentCanMaximize();
    fn CurrentCanMinimize();
    fn CurrentIsModal();
    fn CurrentIsTopmost();
    fn CurrentWindowVisualState();
    fn CurrentWindowInteractionState();
    fn CachedCanMaximize();
    fn CachedCanMinimize();
    fn CachedIsModal();
    fn CachedIsTopmost();
    fn CachedWindowVisualState();
    fn CachedWindowInteractionState();
}
pub trait IValueProviderImpl: Sized {
    fn SetValue();
    fn Value();
    fn IsReadOnly();
}
pub trait IVirtualizedItemProviderImpl: Sized {
    fn Realize();
}
pub trait IWindowProviderImpl: Sized {
    fn SetVisualState();
    fn Close();
    fn WaitForInputIdle();
    fn CanMaximize();
    fn CanMinimize();
    fn IsModal();
    fn WindowVisualState();
    fn WindowInteractionState();
    fn IsTopmost();
}
