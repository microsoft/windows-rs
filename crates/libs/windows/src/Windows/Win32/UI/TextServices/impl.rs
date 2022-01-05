pub trait IAccClientDocMgrImpl: Sized {
    fn GetDocuments();
    fn LookupByHWND();
    fn LookupByPoint();
    fn GetFocused();
}
pub trait IAccDictionaryImpl: Sized {
    fn GetLocalizedString();
    fn GetParentTerm();
    fn GetMnemonicString();
    fn LookupMnemonicTerm();
    fn ConvertValueToString();
}
pub trait IAccServerDocMgrImpl: Sized {
    fn NewDocument();
    fn RevokeDocument();
    fn OnDocumentFocus();
}
pub trait IAccStoreImpl: Sized {
    fn Register();
    fn Unregister();
    fn GetDocuments();
    fn LookupByHWND();
    fn LookupByPoint();
    fn OnDocumentFocus();
    fn GetFocused();
}
pub trait IAnchorImpl: Sized {
    fn SetGravity();
    fn GetGravity();
    fn IsEqual();
    fn Compare();
    fn Shift();
    fn ShiftTo();
    fn ShiftRegion();
    fn SetChangeHistoryMask();
    fn GetChangeHistory();
    fn ClearChangeHistory();
    fn Clone();
}
pub trait IClonableWrapperImpl: Sized {
    fn CloneNewWrapper();
}
pub trait ICoCreateLocallyImpl: Sized {
    fn CoCreateLocally();
}
pub trait ICoCreatedLocallyImpl: Sized {
    fn LocalInit();
}
pub trait IDocWrapImpl: Sized {
    fn SetDoc();
    fn GetWrappedDoc();
}
pub trait IEnumITfCompositionViewImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
pub trait IEnumSpeechCommandsImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
pub trait IEnumTfCandidatesImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
pub trait IEnumTfContextViewsImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
pub trait IEnumTfContextsImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
pub trait IEnumTfDisplayAttributeInfoImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
pub trait IEnumTfDocumentMgrsImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
pub trait IEnumTfFunctionProvidersImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
pub trait IEnumTfInputProcessorProfilesImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
pub trait IEnumTfLangBarItemsImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
pub trait IEnumTfLanguageProfilesImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
pub trait IEnumTfLatticeElementsImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
pub trait IEnumTfPropertiesImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
pub trait IEnumTfPropertyValueImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
pub trait IEnumTfRangesImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
pub trait IEnumTfUIElementsImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
pub trait IInternalDocWrapImpl: Sized {
    fn NotifyRevoke();
}
pub trait ISpeechCommandProviderImpl: Sized {
    fn EnumSpeechCommands();
    fn ProcessCommand();
}
pub trait ITextStoreACPImpl: Sized {
    fn AdviseSink();
    fn UnadviseSink();
    fn RequestLock();
    fn GetStatus();
    fn QueryInsert();
    fn GetSelection();
    fn SetSelection();
    fn GetText();
    fn SetText();
    fn GetFormattedText();
    fn GetEmbedded();
    fn QueryInsertEmbedded();
    fn InsertEmbedded();
    fn InsertTextAtSelection();
    fn InsertEmbeddedAtSelection();
    fn RequestSupportedAttrs();
    fn RequestAttrsAtPosition();
    fn RequestAttrsTransitioningAtPosition();
    fn FindNextAttrTransition();
    fn RetrieveRequestedAttrs();
    fn GetEndACP();
    fn GetActiveView();
    fn GetACPFromPoint();
    fn GetTextExt();
    fn GetScreenExt();
    fn GetWnd();
}
pub trait ITextStoreACP2Impl: Sized {
    fn AdviseSink();
    fn UnadviseSink();
    fn RequestLock();
    fn GetStatus();
    fn QueryInsert();
    fn GetSelection();
    fn SetSelection();
    fn GetText();
    fn SetText();
    fn GetFormattedText();
    fn GetEmbedded();
    fn QueryInsertEmbedded();
    fn InsertEmbedded();
    fn InsertTextAtSelection();
    fn InsertEmbeddedAtSelection();
    fn RequestSupportedAttrs();
    fn RequestAttrsAtPosition();
    fn RequestAttrsTransitioningAtPosition();
    fn FindNextAttrTransition();
    fn RetrieveRequestedAttrs();
    fn GetEndACP();
    fn GetActiveView();
    fn GetACPFromPoint();
    fn GetTextExt();
    fn GetScreenExt();
}
pub trait ITextStoreACPExImpl: Sized {
    fn ScrollToRect();
}
pub trait ITextStoreACPServicesImpl: Sized {
    fn Serialize();
    fn Unserialize();
    fn ForceLoadProperty();
    fn CreateRange();
}
pub trait ITextStoreACPSinkImpl: Sized {
    fn OnTextChange();
    fn OnSelectionChange();
    fn OnLayoutChange();
    fn OnStatusChange();
    fn OnAttrsChange();
    fn OnLockGranted();
    fn OnStartEditTransaction();
    fn OnEndEditTransaction();
}
pub trait ITextStoreACPSinkExImpl: Sized + ITextStoreACPSinkImpl {
    fn OnDisconnect();
}
pub trait ITextStoreAnchorImpl: Sized {
    fn AdviseSink();
    fn UnadviseSink();
    fn RequestLock();
    fn GetStatus();
    fn QueryInsert();
    fn GetSelection();
    fn SetSelection();
    fn GetText();
    fn SetText();
    fn GetFormattedText();
    fn GetEmbedded();
    fn InsertEmbedded();
    fn RequestSupportedAttrs();
    fn RequestAttrsAtPosition();
    fn RequestAttrsTransitioningAtPosition();
    fn FindNextAttrTransition();
    fn RetrieveRequestedAttrs();
    fn GetStart();
    fn GetEnd();
    fn GetActiveView();
    fn GetAnchorFromPoint();
    fn GetTextExt();
    fn GetScreenExt();
    fn GetWnd();
    fn QueryInsertEmbedded();
    fn InsertTextAtSelection();
    fn InsertEmbeddedAtSelection();
}
pub trait ITextStoreAnchorExImpl: Sized {
    fn ScrollToRect();
}
pub trait ITextStoreAnchorSinkImpl: Sized {
    fn OnTextChange();
    fn OnSelectionChange();
    fn OnLayoutChange();
    fn OnStatusChange();
    fn OnAttrsChange();
    fn OnLockGranted();
    fn OnStartEditTransaction();
    fn OnEndEditTransaction();
}
pub trait ITextStoreSinkAnchorExImpl: Sized + ITextStoreAnchorSinkImpl {
    fn OnDisconnect();
}
pub trait ITfActiveLanguageProfileNotifySinkImpl: Sized {
    fn OnActivated();
}
pub trait ITfCandidateListImpl: Sized {
    fn EnumCandidates();
    fn GetCandidate();
    fn GetCandidateNum();
    fn SetResult();
}
pub trait ITfCandidateListUIElementImpl: Sized + ITfUIElementImpl {
    fn GetUpdatedFlags();
    fn GetDocumentMgr();
    fn GetCount();
    fn GetSelection();
    fn GetString();
    fn GetPageIndex();
    fn SetPageIndex();
    fn GetCurrentPage();
}
pub trait ITfCandidateListUIElementBehaviorImpl: Sized + ITfCandidateListUIElementImpl + ITfUIElementImpl {
    fn SetSelection();
    fn Finalize();
    fn Abort();
}
pub trait ITfCandidateStringImpl: Sized {
    fn GetString();
    fn GetIndex();
}
pub trait ITfCategoryMgrImpl: Sized {
    fn RegisterCategory();
    fn UnregisterCategory();
    fn EnumCategoriesInItem();
    fn EnumItemsInCategory();
    fn FindClosestCategory();
    fn RegisterGUIDDescription();
    fn UnregisterGUIDDescription();
    fn GetGUIDDescription();
    fn RegisterGUIDDWORD();
    fn UnregisterGUIDDWORD();
    fn GetGUIDDWORD();
    fn RegisterGUID();
    fn GetGUID();
    fn IsEqualTfGuidAtom();
}
pub trait ITfCleanupContextDurationSinkImpl: Sized {
    fn OnStartCleanupContext();
    fn OnEndCleanupContext();
}
pub trait ITfCleanupContextSinkImpl: Sized {
    fn OnCleanupContext();
}
pub trait ITfClientIdImpl: Sized {
    fn GetClientId();
}
pub trait ITfCompartmentImpl: Sized {
    fn SetValue();
    fn GetValue();
}
pub trait ITfCompartmentEventSinkImpl: Sized {
    fn OnChange();
}
pub trait ITfCompartmentMgrImpl: Sized {
    fn GetCompartment();
    fn ClearCompartment();
    fn EnumCompartments();
}
pub trait ITfCompositionImpl: Sized {
    fn GetRange();
    fn ShiftStart();
    fn ShiftEnd();
    fn EndComposition();
}
pub trait ITfCompositionSinkImpl: Sized {
    fn OnCompositionTerminated();
}
pub trait ITfCompositionViewImpl: Sized {
    fn GetOwnerClsid();
    fn GetRange();
}
pub trait ITfConfigureSystemKeystrokeFeedImpl: Sized {
    fn DisableSystemKeystrokeFeed();
    fn EnableSystemKeystrokeFeed();
}
pub trait ITfContextImpl: Sized {
    fn RequestEditSession();
    fn InWriteSession();
    fn GetSelection();
    fn SetSelection();
    fn GetStart();
    fn GetEnd();
    fn GetActiveView();
    fn EnumViews();
    fn GetStatus();
    fn GetProperty();
    fn GetAppProperty();
    fn TrackProperties();
    fn EnumProperties();
    fn GetDocumentMgr();
    fn CreateRangeBackup();
}
pub trait ITfContextCompositionImpl: Sized {
    fn StartComposition();
    fn EnumCompositions();
    fn FindComposition();
    fn TakeOwnership();
}
pub trait ITfContextKeyEventSinkImpl: Sized {
    fn OnKeyDown();
    fn OnKeyUp();
    fn OnTestKeyDown();
    fn OnTestKeyUp();
}
pub trait ITfContextOwnerImpl: Sized {
    fn GetACPFromPoint();
    fn GetTextExt();
    fn GetScreenExt();
    fn GetStatus();
    fn GetWnd();
    fn GetAttribute();
}
pub trait ITfContextOwnerCompositionServicesImpl: Sized + ITfContextCompositionImpl {
    fn TerminateComposition();
}
pub trait ITfContextOwnerCompositionSinkImpl: Sized {
    fn OnStartComposition();
    fn OnUpdateComposition();
    fn OnEndComposition();
}
pub trait ITfContextOwnerServicesImpl: Sized {
    fn OnLayoutChange();
    fn OnStatusChange();
    fn OnAttributeChange();
    fn Serialize();
    fn Unserialize();
    fn ForceLoadProperty();
    fn CreateRange();
}
pub trait ITfContextViewImpl: Sized {
    fn GetRangeFromPoint();
    fn GetTextExt();
    fn GetScreenExt();
    fn GetWnd();
}
pub trait ITfCreatePropertyStoreImpl: Sized {
    fn IsStoreSerializable();
    fn CreatePropertyStore();
}
pub trait ITfDisplayAttributeInfoImpl: Sized {
    fn GetGUID();
    fn GetDescription();
    fn GetAttributeInfo();
    fn SetAttributeInfo();
    fn Reset();
}
pub trait ITfDisplayAttributeMgrImpl: Sized {
    fn OnUpdateInfo();
    fn EnumDisplayAttributeInfo();
    fn GetDisplayAttributeInfo();
}
pub trait ITfDisplayAttributeNotifySinkImpl: Sized {
    fn OnUpdateInfo();
}
pub trait ITfDisplayAttributeProviderImpl: Sized {
    fn EnumDisplayAttributeInfo();
    fn GetDisplayAttributeInfo();
}
pub trait ITfDocumentMgrImpl: Sized {
    fn CreateContext();
    fn Push();
    fn Pop();
    fn GetTop();
    fn GetBase();
    fn EnumContexts();
}
pub trait ITfEditRecordImpl: Sized {
    fn GetSelectionStatus();
    fn GetTextAndPropertyUpdates();
}
pub trait ITfEditSessionImpl: Sized {
    fn DoEditSession();
}
pub trait ITfEditTransactionSinkImpl: Sized {
    fn OnStartEditTransaction();
    fn OnEndEditTransaction();
}
pub trait ITfFnAdviseTextImpl: Sized + ITfFunctionImpl {
    fn OnTextUpdate();
    fn OnLatticeUpdate();
}
pub trait ITfFnBalloonImpl: Sized {
    fn UpdateBalloon();
}
pub trait ITfFnConfigureImpl: Sized + ITfFunctionImpl {
    fn Show();
}
pub trait ITfFnConfigureRegisterEudcImpl: Sized + ITfFunctionImpl {
    fn Show();
}
pub trait ITfFnConfigureRegisterWordImpl: Sized + ITfFunctionImpl {
    fn Show();
}
pub trait ITfFnCustomSpeechCommandImpl: Sized + ITfFunctionImpl {
    fn SetSpeechCommandProvider();
}
pub trait ITfFnGetLinguisticAlternatesImpl: Sized + ITfFunctionImpl {
    fn GetAlternates();
}
pub trait ITfFnGetPreferredTouchKeyboardLayoutImpl: Sized + ITfFunctionImpl {
    fn GetLayout();
}
pub trait ITfFnGetSAPIObjectImpl: Sized + ITfFunctionImpl {
    fn Get();
}
pub trait ITfFnLMInternalImpl: Sized + ITfFnLMProcessorImpl + ITfFunctionImpl {
    fn ProcessLattice();
}
pub trait ITfFnLMProcessorImpl: Sized + ITfFunctionImpl {
    fn QueryRange();
    fn QueryLangID();
    fn GetReconversion();
    fn Reconvert();
    fn QueryKey();
    fn InvokeKey();
    fn InvokeFunc();
}
pub trait ITfFnLangProfileUtilImpl: Sized + ITfFunctionImpl {
    fn RegisterActiveProfiles();
    fn IsProfileAvailableForLang();
}
pub trait ITfFnPlayBackImpl: Sized + ITfFunctionImpl {
    fn QueryRange();
    fn Play();
}
pub trait ITfFnPropertyUIStatusImpl: Sized + ITfFunctionImpl {
    fn GetStatus();
    fn SetStatus();
}
pub trait ITfFnReconversionImpl: Sized + ITfFunctionImpl {
    fn QueryRange();
    fn GetReconversion();
    fn Reconvert();
}
pub trait ITfFnSearchCandidateProviderImpl: Sized + ITfFunctionImpl {
    fn GetSearchCandidates();
    fn SetResult();
}
pub trait ITfFnShowHelpImpl: Sized + ITfFunctionImpl {
    fn Show();
}
pub trait ITfFunctionImpl: Sized {
    fn GetDisplayName();
}
pub trait ITfFunctionProviderImpl: Sized {
    fn GetType();
    fn GetDescription();
    fn GetFunction();
}
pub trait ITfInputProcessorProfileActivationSinkImpl: Sized {
    fn OnActivated();
}
pub trait ITfInputProcessorProfileMgrImpl: Sized {
    fn ActivateProfile();
    fn DeactivateProfile();
    fn GetProfile();
    fn EnumProfiles();
    fn ReleaseInputProcessor();
    fn RegisterProfile();
    fn UnregisterProfile();
    fn GetActiveProfile();
}
pub trait ITfInputProcessorProfileSubstituteLayoutImpl: Sized {
    fn GetSubstituteKeyboardLayout();
}
pub trait ITfInputProcessorProfilesImpl: Sized {
    fn Register();
    fn Unregister();
    fn AddLanguageProfile();
    fn RemoveLanguageProfile();
    fn EnumInputProcessorInfo();
    fn GetDefaultLanguageProfile();
    fn SetDefaultLanguageProfile();
    fn ActivateLanguageProfile();
    fn GetActiveLanguageProfile();
    fn GetLanguageProfileDescription();
    fn GetCurrentLanguage();
    fn ChangeCurrentLanguage();
    fn GetLanguageList();
    fn EnumLanguageProfiles();
    fn EnableLanguageProfile();
    fn IsEnabledLanguageProfile();
    fn EnableLanguageProfileByDefault();
    fn SubstituteKeyboardLayout();
}
pub trait ITfInputProcessorProfilesExImpl: Sized + ITfInputProcessorProfilesImpl {
    fn SetLanguageProfileDisplayName();
}
pub trait ITfInputScopeImpl: Sized {
    fn GetInputScopes();
    fn GetPhrase();
    fn GetRegularExpression();
    fn GetSRGS();
    fn GetXML();
}
pub trait ITfInputScope2Impl: Sized + ITfInputScopeImpl {
    fn EnumWordList();
}
pub trait ITfInsertAtSelectionImpl: Sized {
    fn InsertTextAtSelection();
    fn InsertEmbeddedAtSelection();
}
pub trait ITfIntegratableCandidateListUIElementImpl: Sized {
    fn SetIntegrationStyle();
    fn GetSelectionStyle();
    fn OnKeyDown();
    fn ShowCandidateNumbers();
    fn FinalizeExactCompositionString();
}
pub trait ITfKeyEventSinkImpl: Sized {
    fn OnSetFocus();
    fn OnTestKeyDown();
    fn OnTestKeyUp();
    fn OnKeyDown();
    fn OnKeyUp();
    fn OnPreservedKey();
}
pub trait ITfKeyTraceEventSinkImpl: Sized {
    fn OnKeyTraceDown();
    fn OnKeyTraceUp();
}
pub trait ITfKeystrokeMgrImpl: Sized {
    fn AdviseKeyEventSink();
    fn UnadviseKeyEventSink();
    fn GetForeground();
    fn TestKeyDown();
    fn TestKeyUp();
    fn KeyDown();
    fn KeyUp();
    fn GetPreservedKey();
    fn IsPreservedKey();
    fn PreserveKey();
    fn UnpreserveKey();
    fn SetPreservedKeyDescription();
    fn GetPreservedKeyDescription();
    fn SimulatePreservedKey();
}
pub trait ITfLMLatticeImpl: Sized {
    fn QueryType();
    fn EnumLatticeElements();
}
pub trait ITfLangBarEventSinkImpl: Sized {
    fn OnSetFocus();
    fn OnThreadTerminate();
    fn OnThreadItemChange();
    fn OnModalInput();
    fn ShowFloating();
    fn GetItemFloatingRect();
}
pub trait ITfLangBarItemImpl: Sized {
    fn GetInfo();
    fn GetStatus();
    fn Show();
    fn GetTooltipString();
}
pub trait ITfLangBarItemBalloonImpl: Sized + ITfLangBarItemImpl {
    fn OnClick();
    fn GetPreferredSize();
    fn GetBalloonInfo();
}
pub trait ITfLangBarItemBitmapImpl: Sized + ITfLangBarItemImpl {
    fn OnClick();
    fn GetPreferredSize();
    fn DrawBitmap();
}
pub trait ITfLangBarItemBitmapButtonImpl: Sized + ITfLangBarItemImpl {
    fn OnClick();
    fn InitMenu();
    fn OnMenuSelect();
    fn GetPreferredSize();
    fn DrawBitmap();
    fn GetText();
}
pub trait ITfLangBarItemButtonImpl: Sized + ITfLangBarItemImpl {
    fn OnClick();
    fn InitMenu();
    fn OnMenuSelect();
    fn GetIcon();
    fn GetText();
}
pub trait ITfLangBarItemMgrImpl: Sized {
    fn EnumItems();
    fn GetItem();
    fn AddItem();
    fn RemoveItem();
    fn AdviseItemSink();
    fn UnadviseItemSink();
    fn GetItemFloatingRect();
    fn GetItemsStatus();
    fn GetItemNum();
    fn GetItems();
    fn AdviseItemsSink();
    fn UnadviseItemsSink();
}
pub trait ITfLangBarItemSinkImpl: Sized {
    fn OnUpdate();
}
pub trait ITfLangBarMgrImpl: Sized {
    fn AdviseEventSink();
    fn UnadviseEventSink();
    fn GetThreadMarshalInterface();
    fn GetThreadLangBarItemMgr();
    fn GetInputProcessorProfiles();
    fn RestoreLastFocus();
    fn SetModalInput();
    fn ShowFloating();
    fn GetShowFloatingStatus();
}
pub trait ITfLanguageProfileNotifySinkImpl: Sized {
    fn OnLanguageChange();
    fn OnLanguageChanged();
}
pub trait ITfMSAAControlImpl: Sized {
    fn SystemEnableMSAA();
    fn SystemDisableMSAA();
}
pub trait ITfMenuImpl: Sized {
    fn AddMenuItem();
}
pub trait ITfMessagePumpImpl: Sized {
    fn PeekMessageA();
    fn GetMessageA();
    fn PeekMessageW();
    fn GetMessageW();
}
pub trait ITfMouseSinkImpl: Sized {
    fn OnMouseEvent();
}
pub trait ITfMouseTrackerImpl: Sized {
    fn AdviseMouseSink();
    fn UnadviseMouseSink();
}
pub trait ITfMouseTrackerACPImpl: Sized {
    fn AdviseMouseSink();
    fn UnadviseMouseSink();
}
pub trait ITfPersistentPropertyLoaderACPImpl: Sized {
    fn LoadProperty();
}
pub trait ITfPreservedKeyNotifySinkImpl: Sized {
    fn OnUpdated();
}
pub trait ITfPropertyImpl: Sized + ITfReadOnlyPropertyImpl {
    fn FindRange();
    fn SetValueStore();
    fn SetValue();
    fn Clear();
}
pub trait ITfPropertyStoreImpl: Sized {
    fn GetType();
    fn GetDataType();
    fn GetData();
    fn OnTextUpdated();
    fn Shrink();
    fn Divide();
    fn Clone();
    fn GetPropertyRangeCreator();
    fn Serialize();
}
pub trait ITfQueryEmbeddedImpl: Sized {
    fn QueryInsertEmbedded();
}
pub trait ITfRangeImpl: Sized {
    fn GetText();
    fn SetText();
    fn GetFormattedText();
    fn GetEmbedded();
    fn InsertEmbedded();
    fn ShiftStart();
    fn ShiftEnd();
    fn ShiftStartToRange();
    fn ShiftEndToRange();
    fn ShiftStartRegion();
    fn ShiftEndRegion();
    fn IsEmpty();
    fn Collapse();
    fn IsEqualStart();
    fn IsEqualEnd();
    fn CompareStart();
    fn CompareEnd();
    fn AdjustForInsert();
    fn GetGravity();
    fn SetGravity();
    fn Clone();
    fn GetContext();
}
pub trait ITfRangeACPImpl: Sized + ITfRangeImpl {
    fn GetExtent();
    fn SetExtent();
}
pub trait ITfRangeBackupImpl: Sized {
    fn Restore();
}
pub trait ITfReadOnlyPropertyImpl: Sized {
    fn GetType();
    fn EnumRanges();
    fn GetValue();
    fn GetContext();
}
pub trait ITfReadingInformationUIElementImpl: Sized + ITfUIElementImpl {
    fn GetUpdatedFlags();
    fn GetContext();
    fn GetString();
    fn GetMaxReadingStringLength();
    fn GetErrorIndex();
    fn IsVerticalOrderPreferred();
}
pub trait ITfReverseConversionImpl: Sized {
    fn DoReverseConversion();
}
pub trait ITfReverseConversionListImpl: Sized {
    fn GetLength();
    fn GetString();
}
pub trait ITfReverseConversionMgrImpl: Sized {
    fn GetReverseConversion();
}
pub trait ITfSourceImpl: Sized {
    fn AdviseSink();
    fn UnadviseSink();
}
pub trait ITfSourceSingleImpl: Sized {
    fn AdviseSingleSink();
    fn UnadviseSingleSink();
}
pub trait ITfSpeechUIServerImpl: Sized {
    fn Initialize();
    fn ShowUI();
    fn UpdateBalloon();
}
pub trait ITfStatusSinkImpl: Sized {
    fn OnStatusChange();
}
pub trait ITfSystemDeviceTypeLangBarItemImpl: Sized {
    fn SetIconMode();
    fn GetIconMode();
}
pub trait ITfSystemLangBarItemImpl: Sized {
    fn SetIcon();
    fn SetTooltipString();
}
pub trait ITfSystemLangBarItemSinkImpl: Sized {
    fn InitMenu();
    fn OnMenuSelect();
}
pub trait ITfSystemLangBarItemTextImpl: Sized {
    fn SetItemText();
    fn GetItemText();
}
pub trait ITfTextEditSinkImpl: Sized {
    fn OnEndEdit();
}
pub trait ITfTextInputProcessorImpl: Sized {
    fn Activate();
    fn Deactivate();
}
pub trait ITfTextInputProcessorExImpl: Sized + ITfTextInputProcessorImpl {
    fn ActivateEx();
}
pub trait ITfTextLayoutSinkImpl: Sized {
    fn OnLayoutChange();
}
pub trait ITfThreadFocusSinkImpl: Sized {
    fn OnSetThreadFocus();
    fn OnKillThreadFocus();
}
pub trait ITfThreadMgrImpl: Sized {
    fn Activate();
    fn Deactivate();
    fn CreateDocumentMgr();
    fn EnumDocumentMgrs();
    fn GetFocus();
    fn SetFocus();
    fn AssociateFocus();
    fn IsThreadFocus();
    fn GetFunctionProvider();
    fn EnumFunctionProviders();
    fn GetGlobalCompartment();
}
pub trait ITfThreadMgr2Impl: Sized {
    fn Activate();
    fn Deactivate();
    fn CreateDocumentMgr();
    fn EnumDocumentMgrs();
    fn GetFocus();
    fn SetFocus();
    fn IsThreadFocus();
    fn GetFunctionProvider();
    fn EnumFunctionProviders();
    fn GetGlobalCompartment();
    fn ActivateEx();
    fn GetActiveFlags();
    fn SuspendKeystrokeHandling();
    fn ResumeKeystrokeHandling();
}
pub trait ITfThreadMgrEventSinkImpl: Sized {
    fn OnInitDocumentMgr();
    fn OnUninitDocumentMgr();
    fn OnSetFocus();
    fn OnPushContext();
    fn OnPopContext();
}
pub trait ITfThreadMgrExImpl: Sized + ITfThreadMgrImpl {
    fn ActivateEx();
    fn GetActiveFlags();
}
pub trait ITfToolTipUIElementImpl: Sized + ITfUIElementImpl {
    fn GetString();
}
pub trait ITfTransitoryExtensionSinkImpl: Sized {
    fn OnTransitoryExtensionUpdated();
}
pub trait ITfTransitoryExtensionUIElementImpl: Sized + ITfUIElementImpl {
    fn GetDocumentMgr();
}
pub trait ITfUIElementImpl: Sized {
    fn GetDescription();
    fn GetGUID();
    fn Show();
    fn IsShown();
}
pub trait ITfUIElementMgrImpl: Sized {
    fn BeginUIElement();
    fn UpdateUIElement();
    fn EndUIElement();
    fn GetUIElement();
    fn EnumUIElements();
}
pub trait ITfUIElementSinkImpl: Sized {
    fn BeginUIElement();
    fn UpdateUIElement();
    fn EndUIElement();
}
pub trait IUIManagerEventSinkImpl: Sized {
    fn OnWindowOpening();
    fn OnWindowOpened();
    fn OnWindowUpdating();
    fn OnWindowUpdated();
    fn OnWindowClosing();
    fn OnWindowClosed();
}
pub trait IVersionInfoImpl: Sized {
    fn GetSubcomponentCount();
    fn GetImplementationID();
    fn GetBuildVersion();
    fn GetComponentDescription();
    fn GetInstanceDescription();
}
