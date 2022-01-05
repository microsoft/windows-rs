#[cfg(feature = "Win32_System_Com")]
pub trait IAdviseSinkExImpl: Sized + IAdviseSinkImpl {
    fn OnViewStatusChange();
}
pub trait ICanHandleExceptionImpl: Sized {
    fn CanHandleException();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IClassFactory2Impl: Sized + IClassFactoryImpl {
    fn GetLicInfo();
    fn RequestLicKey();
    fn CreateInstanceLic();
}
pub trait IContinueImpl: Sized {
    fn FContinue();
}
pub trait IContinueCallbackImpl: Sized {
    fn FContinue();
    fn FContinuePrinting();
}
pub trait ICreateErrorInfoImpl: Sized {
    fn SetGUID();
    fn SetSource();
    fn SetDescription();
    fn SetHelpFile();
    fn SetHelpContext();
}
pub trait ICreateTypeInfoImpl: Sized {
    fn SetGuid();
    fn SetTypeFlags();
    fn SetDocString();
    fn SetHelpContext();
    fn SetVersion();
    fn AddRefTypeInfo();
    fn AddFuncDesc();
    fn AddImplType();
    fn SetImplTypeFlags();
    fn SetAlignment();
    fn SetSchema();
    fn AddVarDesc();
    fn SetFuncAndParamNames();
    fn SetVarName();
    fn SetTypeDescAlias();
    fn DefineFuncAsDllEntry();
    fn SetFuncDocString();
    fn SetVarDocString();
    fn SetFuncHelpContext();
    fn SetVarHelpContext();
    fn SetMops();
    fn SetTypeIdldesc();
    fn LayOut();
}
pub trait ICreateTypeInfo2Impl: Sized + ICreateTypeInfoImpl {
    fn DeleteFuncDesc();
    fn DeleteFuncDescByMemId();
    fn DeleteVarDesc();
    fn DeleteVarDescByMemId();
    fn DeleteImplType();
    fn SetCustData();
    fn SetFuncCustData();
    fn SetParamCustData();
    fn SetVarCustData();
    fn SetImplTypeCustData();
    fn SetHelpStringContext();
    fn SetFuncHelpStringContext();
    fn SetVarHelpStringContext();
    fn Invalidate();
    fn SetName();
}
pub trait ICreateTypeLibImpl: Sized {
    fn CreateTypeInfo();
    fn SetName();
    fn SetVersion();
    fn SetGuid();
    fn SetDocString();
    fn SetHelpFileName();
    fn SetHelpContext();
    fn SetLcid();
    fn SetLibFlags();
    fn SaveAllChanges();
}
pub trait ICreateTypeLib2Impl: Sized + ICreateTypeLibImpl {
    fn DeleteTypeInfo();
    fn SetCustData();
    fn SetHelpStringContext();
    fn SetHelpStringDll();
}
pub trait IDispErrorImpl: Sized {
    fn QueryErrorInfo();
    fn GetNext();
    fn GetHresult();
    fn GetSource();
    fn GetHelpInfo();
    fn GetDescription();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDispatchExImpl: Sized + IDispatchImpl {
    fn GetDispID();
    fn InvokeEx();
    fn DeleteMemberByName();
    fn DeleteMemberByDispID();
    fn GetMemberProperties();
    fn GetMemberName();
    fn GetNextDispID();
    fn GetNameSpaceParent();
}
pub trait IDropSourceImpl: Sized {
    fn QueryContinueDrag();
    fn GiveFeedback();
}
pub trait IDropSourceNotifyImpl: Sized {
    fn DragEnterTarget();
    fn DragLeaveTarget();
}
pub trait IDropTargetImpl: Sized {
    fn DragEnter();
    fn DragOver();
    fn DragLeave();
    fn Drop();
}
pub trait IEnterpriseDropTargetImpl: Sized {
    fn SetDropSourceEnterpriseId();
    fn IsEvaluatingEdpPolicy();
}
pub trait IEnumOLEVERBImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumOleDocumentViewsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumOleUndoUnitsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumVARIANTImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IFontImpl: Sized {
    fn Name();
    fn SetName();
    fn Size();
    fn SetSize();
    fn Bold();
    fn SetBold();
    fn Italic();
    fn SetItalic();
    fn Underline();
    fn SetUnderline();
    fn Strikethrough();
    fn SetStrikethrough();
    fn Weight();
    fn SetWeight();
    fn Charset();
    fn SetCharset();
    fn hFont();
    fn Clone();
    fn IsEqual();
    fn SetRatio();
    fn QueryTextMetrics();
    fn AddRefHfont();
    fn ReleaseHfont();
    fn SetHdc();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFontDispImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait IFontEventsDispImpl: Sized + IDispatchImpl {}
pub trait IGetOleObjectImpl: Sized {
    fn GetOleObject();
}
pub trait IGetVBAObjectImpl: Sized {
    fn GetObject();
}
pub trait IObjectIdentityImpl: Sized {
    fn IsEqualObject();
}
pub trait IObjectWithSiteImpl: Sized {
    fn SetSite();
    fn GetSite();
}
pub trait IOleAdviseHolderImpl: Sized {
    fn Advise();
    fn Unadvise();
    fn EnumAdvise();
    fn SendOnRename();
    fn SendOnSave();
    fn SendOnClose();
}
pub trait IOleCacheImpl: Sized {
    fn Cache();
    fn Uncache();
    fn EnumCache();
    fn InitCache();
    fn SetData();
}
pub trait IOleCache2Impl: Sized + IOleCacheImpl {
    fn UpdateCache();
    fn DiscardCache();
}
pub trait IOleCacheControlImpl: Sized {
    fn OnRun();
    fn OnStop();
}
pub trait IOleClientSiteImpl: Sized {
    fn SaveObject();
    fn GetMoniker();
    fn GetContainer();
    fn ShowObject();
    fn OnShowWindow();
    fn RequestNewObjectLayout();
}
pub trait IOleCommandTargetImpl: Sized {
    fn QueryStatus();
    fn Exec();
}
pub trait IOleContainerImpl: Sized + IParseDisplayNameImpl {
    fn EnumObjects();
    fn LockContainer();
}
pub trait IOleControlImpl: Sized {
    fn GetControlInfo();
    fn OnMnemonic();
    fn OnAmbientPropertyChange();
    fn FreezeEvents();
}
pub trait IOleControlSiteImpl: Sized {
    fn OnControlInfoChanged();
    fn LockInPlaceActive();
    fn GetExtendedControl();
    fn TransformCoords();
    fn TranslateAccelerator();
    fn OnFocus();
    fn ShowPropertyFrame();
}
pub trait IOleDocumentImpl: Sized {
    fn CreateView();
    fn GetDocMiscStatus();
    fn EnumViews();
}
pub trait IOleDocumentSiteImpl: Sized {
    fn ActivateMe();
}
pub trait IOleDocumentViewImpl: Sized {
    fn SetInPlaceSite();
    fn GetInPlaceSite();
    fn GetDocument();
    fn SetRect();
    fn GetRect();
    fn SetRectComplex();
    fn Show();
    fn UIActivate();
    fn Open();
    fn CloseView();
    fn SaveViewState();
    fn ApplyViewState();
    fn Clone();
}
pub trait IOleInPlaceActiveObjectImpl: Sized + IOleWindowImpl {
    fn TranslateAccelerator();
    fn OnFrameWindowActivate();
    fn OnDocWindowActivate();
    fn ResizeBorder();
    fn EnableModeless();
}
pub trait IOleInPlaceFrameImpl: Sized + IOleInPlaceUIWindowImpl + IOleWindowImpl {
    fn InsertMenus();
    fn SetMenu();
    fn RemoveMenus();
    fn SetStatusText();
    fn EnableModeless();
    fn TranslateAccelerator();
}
pub trait IOleInPlaceObjectImpl: Sized + IOleWindowImpl {
    fn InPlaceDeactivate();
    fn UIDeactivate();
    fn SetObjectRects();
    fn ReactivateAndUndo();
}
pub trait IOleInPlaceObjectWindowlessImpl: Sized + IOleInPlaceObjectImpl + IOleWindowImpl {
    fn OnWindowMessage();
    fn GetDropTarget();
}
pub trait IOleInPlaceSiteImpl: Sized + IOleWindowImpl {
    fn CanInPlaceActivate();
    fn OnInPlaceActivate();
    fn OnUIActivate();
    fn GetWindowContext();
    fn Scroll();
    fn OnUIDeactivate();
    fn OnInPlaceDeactivate();
    fn DiscardUndoState();
    fn DeactivateAndUndo();
    fn OnPosRectChange();
}
pub trait IOleInPlaceSiteExImpl: Sized + IOleInPlaceSiteImpl + IOleWindowImpl {
    fn OnInPlaceActivateEx();
    fn OnInPlaceDeactivateEx();
    fn RequestUIActivate();
}
pub trait IOleInPlaceSiteWindowlessImpl: Sized + IOleInPlaceSiteExImpl + IOleInPlaceSiteImpl + IOleWindowImpl {
    fn CanWindowlessActivate();
    fn GetCapture();
    fn SetCapture();
    fn GetFocus();
    fn SetFocus();
    fn GetDC();
    fn ReleaseDC();
    fn InvalidateRect();
    fn InvalidateRgn();
    fn ScrollRect();
    fn AdjustRect();
    fn OnDefWindowMessage();
}
pub trait IOleInPlaceUIWindowImpl: Sized + IOleWindowImpl {
    fn GetBorder();
    fn RequestBorderSpace();
    fn SetBorderSpace();
    fn SetActiveObject();
}
pub trait IOleItemContainerImpl: Sized + IOleContainerImpl + IParseDisplayNameImpl {
    fn GetObject();
    fn GetObjectStorage();
    fn IsRunning();
}
pub trait IOleLinkImpl: Sized {
    fn SetUpdateOptions();
    fn GetUpdateOptions();
    fn SetSourceMoniker();
    fn GetSourceMoniker();
    fn SetSourceDisplayName();
    fn GetSourceDisplayName();
    fn BindToSource();
    fn BindIfRunning();
    fn GetBoundSource();
    fn UnbindSource();
    fn Update();
}
pub trait IOleObjectImpl: Sized {
    fn SetClientSite();
    fn GetClientSite();
    fn SetHostNames();
    fn Close();
    fn SetMoniker();
    fn GetMoniker();
    fn InitFromData();
    fn GetClipboardData();
    fn DoVerb();
    fn EnumVerbs();
    fn Update();
    fn IsUpToDate();
    fn GetUserClassID();
    fn GetUserType();
    fn SetExtent();
    fn GetExtent();
    fn Advise();
    fn Unadvise();
    fn EnumAdvise();
    fn GetMiscStatus();
    fn SetColorScheme();
}
pub trait IOleParentUndoUnitImpl: Sized + IOleUndoUnitImpl {
    fn Open();
    fn Close();
    fn Add();
    fn FindUnit();
    fn GetParentState();
}
pub trait IOleUILinkContainerAImpl: Sized {
    fn GetNextLink();
    fn SetLinkUpdateOptions();
    fn GetLinkUpdateOptions();
    fn SetLinkSource();
    fn GetLinkSource();
    fn OpenLinkSource();
    fn UpdateLink();
    fn CancelLink();
}
pub trait IOleUILinkContainerWImpl: Sized {
    fn GetNextLink();
    fn SetLinkUpdateOptions();
    fn GetLinkUpdateOptions();
    fn SetLinkSource();
    fn GetLinkSource();
    fn OpenLinkSource();
    fn UpdateLink();
    fn CancelLink();
}
pub trait IOleUILinkInfoAImpl: Sized + IOleUILinkContainerAImpl {
    fn GetLastUpdate();
}
pub trait IOleUILinkInfoWImpl: Sized + IOleUILinkContainerWImpl {
    fn GetLastUpdate();
}
pub trait IOleUIObjInfoAImpl: Sized {
    fn GetObjectInfo();
    fn GetConvertInfo();
    fn ConvertObject();
    fn GetViewInfo();
    fn SetViewInfo();
}
pub trait IOleUIObjInfoWImpl: Sized {
    fn GetObjectInfo();
    fn GetConvertInfo();
    fn ConvertObject();
    fn GetViewInfo();
    fn SetViewInfo();
}
pub trait IOleUndoManagerImpl: Sized {
    fn Open();
    fn Close();
    fn Add();
    fn GetOpenParentState();
    fn DiscardFrom();
    fn UndoTo();
    fn RedoTo();
    fn EnumUndoable();
    fn EnumRedoable();
    fn GetLastUndoDescription();
    fn GetLastRedoDescription();
    fn Enable();
}
pub trait IOleUndoUnitImpl: Sized {
    fn Do();
    fn GetDescription();
    fn GetUnitType();
    fn OnNextAdd();
}
pub trait IOleWindowImpl: Sized {
    fn GetWindow();
    fn ContextSensitiveHelp();
}
pub trait IParseDisplayNameImpl: Sized {
    fn ParseDisplayName();
}
pub trait IPerPropertyBrowsingImpl: Sized {
    fn GetDisplayString();
    fn MapPropertyToPage();
    fn GetPredefinedStrings();
    fn GetPredefinedValue();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPersistPropertyBagImpl: Sized + IPersistImpl {
    fn InitNew();
    fn Load();
    fn Save();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPersistPropertyBag2Impl: Sized + IPersistImpl {
    fn InitNew();
    fn Load();
    fn Save();
    fn IsDirty();
}
pub trait IPictureImpl: Sized {
    fn Handle();
    fn hPal();
    fn Type();
    fn Width();
    fn Height();
    fn Render();
    fn set_hPal();
    fn CurDC();
    fn SelectPicture();
    fn KeepOriginalFormat();
    fn SetKeepOriginalFormat();
    fn PictureChanged();
    fn SaveAsFile();
    fn Attributes();
}
pub trait IPicture2Impl: Sized {
    fn Handle();
    fn hPal();
    fn Type();
    fn Width();
    fn Height();
    fn Render();
    fn set_hPal();
    fn CurDC();
    fn SelectPicture();
    fn KeepOriginalFormat();
    fn SetKeepOriginalFormat();
    fn PictureChanged();
    fn SaveAsFile();
    fn Attributes();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPictureDispImpl: Sized + IDispatchImpl {}
pub trait IPointerInactiveImpl: Sized {
    fn GetActivationPolicy();
    fn OnInactiveMouseMove();
    fn OnInactiveSetCursor();
}
pub trait IPrintImpl: Sized {
    fn SetInitialPageNum();
    fn GetPageInfo();
    fn Print();
}
pub trait IPropertyNotifySinkImpl: Sized {
    fn OnChanged();
    fn OnRequestEdit();
}
pub trait IPropertyPageImpl: Sized {
    fn SetPageSite();
    fn Activate();
    fn Deactivate();
    fn GetPageInfo();
    fn SetObjects();
    fn Show();
    fn Move();
    fn IsPageDirty();
    fn Apply();
    fn Help();
    fn TranslateAccelerator();
}
pub trait IPropertyPage2Impl: Sized + IPropertyPageImpl {
    fn EditProperty();
}
pub trait IPropertyPageSiteImpl: Sized {
    fn OnStatusChange();
    fn GetLocaleID();
    fn GetPageContainer();
    fn TranslateAccelerator();
}
pub trait IProtectFocusImpl: Sized {
    fn AllowFocusChange();
}
pub trait IProtectedModeMenuServicesImpl: Sized {
    fn CreateMenu();
    fn LoadMenu();
    fn LoadMenuID();
}
pub trait IProvideClassInfoImpl: Sized {
    fn GetClassInfo();
}
pub trait IProvideClassInfo2Impl: Sized + IProvideClassInfoImpl {
    fn GetGUID();
}
pub trait IProvideMultipleClassInfoImpl: Sized + IProvideClassInfo2Impl + IProvideClassInfoImpl {
    fn GetMultiTypeInfoCount();
    fn GetInfoOfIndex();
}
pub trait IProvideRuntimeContextImpl: Sized {
    fn GetCurrentSourceContext();
}
pub trait IQuickActivateImpl: Sized {
    fn QuickActivate();
    fn SetContentExtent();
    fn GetContentExtent();
}
pub trait IRecordInfoImpl: Sized {
    fn RecordInit();
    fn RecordClear();
    fn RecordCopy();
    fn GetGuid();
    fn GetName();
    fn GetSize();
    fn GetTypeInfo();
    fn GetField();
    fn GetFieldNoCopy();
    fn PutField();
    fn PutFieldNoCopy();
    fn GetFieldNames();
    fn IsMatchingType();
    fn RecordCreate();
    fn RecordCreateCopy();
    fn RecordDestroy();
}
pub trait ISimpleFrameSiteImpl: Sized {
    fn PreMessageFilter();
    fn PostMessageFilter();
}
pub trait ISpecifyPropertyPagesImpl: Sized {
    fn GetPages();
}
pub trait ITypeChangeEventsImpl: Sized {
    fn RequestTypeChange();
    fn AfterTypeChange();
}
pub trait ITypeFactoryImpl: Sized {
    fn CreateFromTypeInfo();
}
pub trait ITypeMarshalImpl: Sized {
    fn Size();
    fn Marshal();
    fn Unmarshal();
    fn Free();
}
pub trait IVBFormatImpl: Sized {
    fn Format();
}
pub trait IVBGetControlImpl: Sized {
    fn EnumControls();
}
pub trait IVariantChangeTypeImpl: Sized {
    fn ChangeType();
}
pub trait IViewObjectImpl: Sized {
    fn Draw();
    fn GetColorSet();
    fn Freeze();
    fn Unfreeze();
    fn SetAdvise();
    fn GetAdvise();
}
pub trait IViewObject2Impl: Sized + IViewObjectImpl {
    fn GetExtent();
}
pub trait IViewObjectExImpl: Sized + IViewObject2Impl + IViewObjectImpl {
    fn GetRect();
    fn GetViewStatus();
    fn QueryHitPoint();
    fn QueryHitRect();
    fn GetNaturalExtent();
}
pub trait IZoomEventsImpl: Sized {
    fn OnZoomPercentChanged();
}
