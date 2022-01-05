#[cfg(feature = "Win32_System_Com")]
pub trait CIE4ConnectionPointImpl: Sized + IConnectionPointImpl {
    fn DoInvokeIE4();
    fn DoInvokePIDLIE4();
}
#[cfg(feature = "Win32_System_Com")]
pub trait DFConstraintImpl: Sized + IDispatchImpl {
    fn Name();
    fn Value();
}
#[cfg(feature = "Win32_System_Com")]
pub trait DShellFolderViewEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait DShellNameSpaceEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait DShellWindowsEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait DWebBrowserEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait DWebBrowserEvents2Impl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait FolderImpl: Sized + IDispatchImpl {
    fn Title();
    fn Application();
    fn Parent();
    fn ParentFolder();
    fn Items();
    fn ParseName();
    fn NewFolder();
    fn MoveHere();
    fn CopyHere();
    fn GetDetailsOf();
}
#[cfg(feature = "Win32_System_Com")]
pub trait Folder2Impl: Sized + FolderImpl + IDispatchImpl {
    fn Self_();
    fn OfflineStatus();
    fn Synchronize();
    fn HaveToShowWebViewBarricade();
    fn DismissedWebViewBarricade();
}
#[cfg(feature = "Win32_System_Com")]
pub trait Folder3Impl: Sized + Folder2Impl + FolderImpl + IDispatchImpl {
    fn ShowWebViewBarricade();
    fn SetShowWebViewBarricade();
}
#[cfg(feature = "Win32_System_Com")]
pub trait FolderItemImpl: Sized + IDispatchImpl {
    fn Application();
    fn Parent();
    fn Name();
    fn SetName();
    fn Path();
    fn GetLink();
    fn GetFolder();
    fn IsLink();
    fn IsFolder();
    fn IsFileSystem();
    fn IsBrowsable();
    fn ModifyDate();
    fn SetModifyDate();
    fn Size();
    fn Type();
    fn Verbs();
    fn InvokeVerb();
}
#[cfg(feature = "Win32_System_Com")]
pub trait FolderItem2Impl: Sized + FolderItemImpl + IDispatchImpl {
    fn InvokeVerbEx();
    fn ExtendedProperty();
}
#[cfg(feature = "Win32_System_Com")]
pub trait FolderItemVerbImpl: Sized + IDispatchImpl {
    fn Application();
    fn Parent();
    fn Name();
    fn DoIt();
}
#[cfg(feature = "Win32_System_Com")]
pub trait FolderItemVerbsImpl: Sized + IDispatchImpl {
    fn Count();
    fn Application();
    fn Parent();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait FolderItemsImpl: Sized + IDispatchImpl {
    fn Count();
    fn Application();
    fn Parent();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait FolderItems2Impl: Sized + FolderItemsImpl + IDispatchImpl {
    fn InvokeVerbEx();
}
#[cfg(feature = "Win32_System_Com")]
pub trait FolderItems3Impl: Sized + FolderItems2Impl + FolderItemsImpl + IDispatchImpl {
    fn Filter();
    fn Verbs();
}
pub trait IACListImpl: Sized {
    fn Expand();
}
pub trait IACList2Impl: Sized + IACListImpl {
    fn SetOptions();
    fn GetOptions();
}
pub trait IAccessibilityDockingServiceImpl: Sized {
    fn GetAvailableSize();
    fn DockWindow();
    fn UndockWindow();
}
pub trait IAccessibilityDockingServiceCallbackImpl: Sized {
    fn Undocked();
}
pub trait IAccessibleObjectImpl: Sized {
    fn SetAccessibleName();
}
pub trait IActionProgressImpl: Sized {
    fn Begin();
    fn UpdateProgress();
    fn UpdateText();
    fn QueryCancel();
    fn ResetCancel();
    fn End();
}
pub trait IActionProgressDialogImpl: Sized {
    fn Initialize();
    fn Stop();
}
pub trait IAppActivationUIInfoImpl: Sized {
    fn GetMonitor();
    fn GetInvokePoint();
    fn GetShowCommand();
    fn GetShowUI();
    fn GetKeyState();
}
pub trait IAppPublisherImpl: Sized {
    fn GetNumberOfCategories();
    fn GetCategories();
    fn GetNumberOfApps();
    fn EnumApps();
}
pub trait IAppVisibilityImpl: Sized {
    fn GetAppVisibilityOnMonitor();
    fn IsLauncherVisible();
    fn Advise();
    fn Unadvise();
}
pub trait IAppVisibilityEventsImpl: Sized {
    fn AppVisibilityOnMonitorChanged();
    fn LauncherVisibilityChange();
}
pub trait IApplicationActivationManagerImpl: Sized {
    fn ActivateApplication();
    fn ActivateForFile();
    fn ActivateForProtocol();
}
pub trait IApplicationAssociationRegistrationImpl: Sized {
    fn QueryCurrentDefault();
    fn QueryAppIsDefault();
    fn QueryAppIsDefaultAll();
    fn SetAppAsDefault();
    fn SetAppAsDefaultAll();
    fn ClearUserAssociations();
}
pub trait IApplicationAssociationRegistrationUIImpl: Sized {
    fn LaunchAdvancedAssociationUI();
}
pub trait IApplicationDesignModeSettingsImpl: Sized {
    fn SetNativeDisplaySize();
    fn SetScaleFactor();
    fn SetApplicationViewState();
    fn ComputeApplicationSize();
    fn IsApplicationViewStateSupported();
    fn TriggerEdgeGesture();
}
pub trait IApplicationDesignModeSettings2Impl: Sized + IApplicationDesignModeSettingsImpl {
    fn SetNativeDisplayOrientation();
    fn SetApplicationViewOrientation();
    fn SetAdjacentDisplayEdges();
    fn SetIsOnLockScreen();
    fn SetApplicationViewMinWidth();
    fn GetApplicationSizeBounds();
    fn GetApplicationViewOrientation();
}
pub trait IApplicationDestinationsImpl: Sized {
    fn SetAppID();
    fn RemoveDestination();
    fn RemoveAllDestinations();
}
pub trait IApplicationDocumentListsImpl: Sized {
    fn SetAppID();
    fn GetList();
}
pub trait IAssocHandlerImpl: Sized {
    fn GetName();
    fn GetUIName();
    fn GetIconLocation();
    fn IsRecommended();
    fn MakeDefault();
    fn Invoke();
    fn CreateInvoker();
}
pub trait IAssocHandlerInvokerImpl: Sized {
    fn SupportsSelection();
    fn Invoke();
}
pub trait IAttachmentExecuteImpl: Sized {
    fn SetClientTitle();
    fn SetClientGuid();
    fn SetLocalPath();
    fn SetFileName();
    fn SetSource();
    fn SetReferrer();
    fn CheckPolicy();
    fn Prompt();
    fn Save();
    fn Execute();
    fn SaveWithUI();
    fn ClearClientState();
}
pub trait IAutoCompleteImpl: Sized {
    fn Init();
    fn Enable();
}
pub trait IAutoComplete2Impl: Sized + IAutoCompleteImpl {
    fn SetOptions();
    fn GetOptions();
}
pub trait IAutoCompleteDropDownImpl: Sized {
    fn GetDropDownStatus();
    fn ResetEnumerator();
}
pub trait IBandHostImpl: Sized {
    fn CreateBand();
    fn SetBandAvailability();
    fn DestroyBand();
}
pub trait IBandSiteImpl: Sized {
    fn AddBand();
    fn EnumBands();
    fn QueryBand();
    fn SetBandState();
    fn RemoveBand();
    fn GetBandObject();
    fn SetBandSiteInfo();
    fn GetBandSiteInfo();
}
pub trait IBannerNotificationHandlerImpl: Sized {
    fn OnBannerEvent();
}
pub trait IBanneredBarImpl: Sized {
    fn SetIconSize();
    fn GetIconSize();
    fn SetBitmap();
    fn GetBitmap();
}
pub trait IBrowserFrameOptionsImpl: Sized {
    fn GetFrameOptions();
}
pub trait IBrowserServiceImpl: Sized {
    fn GetParentSite();
    fn SetTitle();
    fn GetTitle();
    fn GetOleObject();
    fn GetTravelLog();
    fn ShowControlWindow();
    fn IsControlWindowShown();
    fn IEGetDisplayName();
    fn IEParseDisplayName();
    fn DisplayParseError();
    fn NavigateToPidl();
    fn SetNavigateState();
    fn GetNavigateState();
    fn NotifyRedirect();
    fn UpdateWindowList();
    fn UpdateBackForwardState();
    fn SetFlags();
    fn GetFlags();
    fn CanNavigateNow();
    fn GetPidl();
    fn SetReferrer();
    fn GetBrowserIndex();
    fn GetBrowserByIndex();
    fn GetHistoryObject();
    fn SetHistoryObject();
    fn CacheOLEServer();
    fn GetSetCodePage();
    fn OnHttpEquiv();
    fn GetPalette();
    fn RegisterWindow();
}
pub trait IBrowserService2Impl: Sized + IBrowserServiceImpl {
    fn WndProcBS();
    fn SetAsDefFolderSettings();
    fn GetViewRect();
    fn OnSize();
    fn OnCreate();
    fn OnCommand();
    fn OnDestroy();
    fn OnNotify();
    fn OnSetFocus();
    fn OnFrameWindowActivateBS();
    fn ReleaseShellView();
    fn ActivatePendingView();
    fn CreateViewWindow();
    fn CreateBrowserPropSheetExt();
    fn GetViewWindow();
    fn GetBaseBrowserData();
    fn PutBaseBrowserData();
    fn InitializeTravelLog();
    fn SetTopBrowser();
    fn Offline();
    fn AllowViewResize();
    fn SetActivateState();
    fn UpdateSecureLockIcon();
    fn InitializeDownloadManager();
    fn InitializeTransitionSite();
    fn _Initialize();
    fn _CancelPendingNavigationAsync();
    fn _CancelPendingView();
    fn _MaySaveChanges();
    fn _PauseOrResumeView();
    fn _DisableModeless();
    fn _NavigateToPidl2();
    fn _TryShell2Rename();
    fn _SwitchActivationNow();
    fn _ExecChildren();
    fn _SendChildren();
    fn GetFolderSetData();
    fn _OnFocusChange();
    fn v_ShowHideChildWindows();
    fn _get_itbLastFocus();
    fn _put_itbLastFocus();
    fn _UIActivateView();
    fn _GetViewBorderRect();
    fn _UpdateViewRectSize();
    fn _ResizeNextBorder();
    fn _ResizeView();
    fn _GetEffectiveClientArea();
    fn v_GetViewStream();
    fn ForwardViewMsg();
    fn SetAcceleratorMenu();
    fn _GetToolbarCount();
    fn _GetToolbarItem();
    fn _SaveToolbars();
    fn _LoadToolbars();
    fn _CloseAndReleaseToolbars();
    fn v_MayGetNextToolbarFocus();
    fn _ResizeNextBorderHelper();
    fn _FindTBar();
    fn _SetFocus();
    fn v_MayTranslateAccelerator();
    fn _GetBorderDWHelper();
    fn v_CheckZoneCrossing();
}
pub trait IBrowserService3Impl: Sized + IBrowserService2Impl + IBrowserServiceImpl {
    fn _PositionViewWindow();
    fn IEParseDisplayNameEx();
}
pub trait IBrowserService4Impl: Sized + IBrowserService3Impl + IBrowserService2Impl + IBrowserServiceImpl {
    fn ActivateView();
    fn SaveViewState();
    fn _ResizeAllBorders();
}
pub trait ICDBurnImpl: Sized {
    fn GetRecorderDriveLetter();
    fn Burn();
    fn HasRecordableDrive();
}
pub trait ICDBurnExtImpl: Sized {
    fn GetSupportedActionTypes();
}
pub trait ICategorizerImpl: Sized {
    fn GetDescription();
    fn GetCategory();
    fn GetCategoryInfo();
    fn CompareCategory();
}
pub trait ICategoryProviderImpl: Sized {
    fn CanCategorizeOnSCID();
    fn GetDefaultCategory();
    fn GetCategoryForSCID();
    fn EnumCategories();
    fn GetCategoryName();
    fn CreateCategory();
}
pub trait IColumnManagerImpl: Sized {
    fn SetColumnInfo();
    fn GetColumnInfo();
    fn GetColumnCount();
    fn GetColumns();
    fn SetColumns();
}
pub trait IColumnProviderImpl: Sized {
    fn Initialize();
    fn GetColumnInfo();
    fn GetItemData();
}
pub trait ICommDlgBrowserImpl: Sized {
    fn OnDefaultCommand();
    fn OnStateChange();
    fn IncludeObject();
}
pub trait ICommDlgBrowser2Impl: Sized + ICommDlgBrowserImpl {
    fn Notify();
    fn GetDefaultMenuText();
    fn GetViewFlags();
}
pub trait ICommDlgBrowser3Impl: Sized + ICommDlgBrowser2Impl + ICommDlgBrowserImpl {
    fn OnColumnClicked();
    fn GetCurrentFilter();
    fn OnPreViewCreated();
}
pub trait IComputerInfoChangeNotifyImpl: Sized {
    fn ComputerInfoChanged();
}
pub trait IConnectableCredentialProviderCredentialImpl: Sized + ICredentialProviderCredentialImpl {
    fn Connect();
    fn Disconnect();
}
pub trait IContactManagerInteropImpl: Sized {
    fn ShowContactCardForWindow();
}
pub trait IContextMenuImpl: Sized {
    fn QueryContextMenu();
    fn InvokeCommand();
    fn GetCommandString();
}
pub trait IContextMenu2Impl: Sized + IContextMenuImpl {
    fn HandleMenuMsg();
}
pub trait IContextMenu3Impl: Sized + IContextMenu2Impl + IContextMenuImpl {
    fn HandleMenuMsg2();
}
pub trait IContextMenuCBImpl: Sized {
    fn CallBack();
}
pub trait IContextMenuSiteImpl: Sized {
    fn DoContextMenuPopup();
}
pub trait ICopyHookAImpl: Sized {
    fn CopyCallback();
}
pub trait ICopyHookWImpl: Sized {
    fn CopyCallback();
}
pub trait ICreateProcessInputsImpl: Sized {
    fn GetCreateFlags();
    fn SetCreateFlags();
    fn AddCreateFlags();
    fn SetHotKey();
    fn AddStartupFlags();
    fn SetTitle();
    fn SetEnvironmentVariable();
}
pub trait ICreatingProcessImpl: Sized {
    fn OnCreating();
}
pub trait ICredentialProviderImpl: Sized {
    fn SetUsageScenario();
    fn SetSerialization();
    fn Advise();
    fn UnAdvise();
    fn GetFieldDescriptorCount();
    fn GetFieldDescriptorAt();
    fn GetCredentialCount();
    fn GetCredentialAt();
}
pub trait ICredentialProviderCredentialImpl: Sized {
    fn Advise();
    fn UnAdvise();
    fn SetSelected();
    fn SetDeselected();
    fn GetFieldState();
    fn GetStringValue();
    fn GetBitmapValue();
    fn GetCheckboxValue();
    fn GetSubmitButtonValue();
    fn GetComboBoxValueCount();
    fn GetComboBoxValueAt();
    fn SetStringValue();
    fn SetCheckboxValue();
    fn SetComboBoxSelectedValue();
    fn CommandLinkClicked();
    fn GetSerialization();
    fn ReportResult();
}
pub trait ICredentialProviderCredential2Impl: Sized + ICredentialProviderCredentialImpl {
    fn GetUserSid();
}
pub trait ICredentialProviderCredentialEventsImpl: Sized {
    fn SetFieldState();
    fn SetFieldInteractiveState();
    fn SetFieldString();
    fn SetFieldCheckbox();
    fn SetFieldBitmap();
    fn SetFieldComboBoxSelectedItem();
    fn DeleteFieldComboBoxItem();
    fn AppendFieldComboBoxItem();
    fn SetFieldSubmitButton();
    fn OnCreatingWindow();
}
pub trait ICredentialProviderCredentialEvents2Impl: Sized + ICredentialProviderCredentialEventsImpl {
    fn BeginFieldUpdates();
    fn EndFieldUpdates();
    fn SetFieldOptions();
}
pub trait ICredentialProviderCredentialWithFieldOptionsImpl: Sized {
    fn GetFieldOptions();
}
pub trait ICredentialProviderEventsImpl: Sized {
    fn CredentialsChanged();
}
pub trait ICredentialProviderFilterImpl: Sized {
    fn Filter();
    fn UpdateRemoteCredential();
}
pub trait ICredentialProviderSetUserArrayImpl: Sized {
    fn SetUserArray();
}
pub trait ICredentialProviderUserImpl: Sized {
    fn GetSid();
    fn GetProviderID();
    fn GetStringValue();
    fn GetValue();
}
pub trait ICredentialProviderUserArrayImpl: Sized {
    fn SetProviderFilter();
    fn GetAccountOptions();
    fn GetCount();
    fn GetAt();
}
pub trait ICurrentItemImpl: Sized + IRelatedItemImpl {}
pub trait ICurrentWorkingDirectoryImpl: Sized {
    fn GetDirectory();
    fn SetDirectory();
}
pub trait ICustomDestinationListImpl: Sized {
    fn SetAppID();
    fn BeginList();
    fn AppendCategory();
    fn AppendKnownCategory();
    fn AddUserTasks();
    fn CommitList();
    fn GetRemovedDestinations();
    fn DeleteList();
    fn AbortList();
}
pub trait IDataObjectAsyncCapabilityImpl: Sized {
    fn SetAsyncMode();
    fn GetAsyncMode();
    fn StartOperation();
    fn InOperation();
    fn EndOperation();
}
pub trait IDataObjectProviderImpl: Sized {
    fn GetDataObject();
    fn SetDataObject();
}
pub trait IDataTransferManagerInteropImpl: Sized {
    fn GetForWindow();
    fn ShowShareUIForWindow();
}
pub trait IDefaultExtractIconInitImpl: Sized {
    fn SetFlags();
    fn SetKey();
    fn SetNormalIcon();
    fn SetOpenIcon();
    fn SetShortcutIcon();
    fn SetDefaultIcon();
}
pub trait IDefaultFolderMenuInitializeImpl: Sized {
    fn Initialize();
    fn SetMenuRestrictions();
    fn GetMenuRestrictions();
    fn SetHandlerClsid();
}
pub trait IDelegateFolderImpl: Sized {
    fn SetItemAlloc();
}
pub trait IDelegateItemImpl: Sized + IRelatedItemImpl {}
#[cfg(feature = "Win32_System_Ole")]
pub trait IDeskBandImpl: Sized + IDockingWindowImpl + IOleWindowImpl {
    fn GetBandInfo();
}
#[cfg(feature = "Win32_System_Ole")]
pub trait IDeskBand2Impl: Sized + IDeskBandImpl + IDockingWindowImpl + IOleWindowImpl {
    fn CanRenderComposited();
    fn SetCompositionState();
    fn GetCompositionState();
}
pub trait IDeskBandInfoImpl: Sized {
    fn GetDefaultBandWidth();
}
#[cfg(feature = "Win32_System_Ole")]
pub trait IDeskBarImpl: Sized + IOleWindowImpl {
    fn SetClient();
    fn GetClient();
    fn OnPosRectChangeDB();
}
#[cfg(feature = "Win32_System_Ole")]
pub trait IDeskBarClientImpl: Sized + IOleWindowImpl {
    fn SetDeskBarSite();
    fn SetModeDBC();
    fn UIActivateDBC();
    fn GetSize();
}
pub trait IDesktopGadgetImpl: Sized {
    fn RunGadget();
}
pub trait IDesktopWallpaperImpl: Sized {
    fn SetWallpaper();
    fn GetWallpaper();
    fn GetMonitorDevicePathAt();
    fn GetMonitorDevicePathCount();
    fn GetMonitorRECT();
    fn SetBackgroundColor();
    fn GetBackgroundColor();
    fn SetPosition();
    fn GetPosition();
    fn SetSlideshow();
    fn GetSlideshow();
    fn SetSlideshowOptions();
    fn GetSlideshowOptions();
    fn AdvanceSlideshow();
    fn GetStatus();
    fn Enable();
}
pub trait IDestinationStreamFactoryImpl: Sized {
    fn GetDestinationStream();
}
pub trait IDisplayItemImpl: Sized + IRelatedItemImpl {}
pub trait IDocViewSiteImpl: Sized {
    fn OnSetTitle();
}
#[cfg(feature = "Win32_System_Ole")]
pub trait IDockingWindowImpl: Sized + IOleWindowImpl {
    fn ShowDW();
    fn CloseDW();
    fn ResizeBorderDW();
}
#[cfg(feature = "Win32_System_Ole")]
pub trait IDockingWindowFrameImpl: Sized + IOleWindowImpl {
    fn AddToolbar();
    fn RemoveToolbar();
    fn FindToolbar();
}
#[cfg(feature = "Win32_System_Ole")]
pub trait IDockingWindowSiteImpl: Sized + IOleWindowImpl {
    fn GetBorderDW();
    fn RequestBorderSpaceDW();
    fn SetBorderSpaceDW();
}
pub trait IDragSourceHelperImpl: Sized {
    fn InitializeFromBitmap();
    fn InitializeFromWindow();
}
pub trait IDragSourceHelper2Impl: Sized + IDragSourceHelperImpl {
    fn SetFlags();
}
pub trait IDropTargetHelperImpl: Sized {
    fn DragEnter();
    fn DragLeave();
    fn DragOver();
    fn Drop();
    fn Show();
}
pub trait IDynamicHWHandlerImpl: Sized {
    fn GetDynamicInfo();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumACStringImpl: Sized + IEnumStringImpl {
    fn NextItem();
    fn SetEnumOptions();
    fn GetEnumOptions();
}
pub trait IEnumAssocHandlersImpl: Sized {
    fn Next();
}
pub trait IEnumExplorerCommandImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumExtraSearchImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumFullIDListImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumHLITEMImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumIDListImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumObjectsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumPublishedAppsImpl: Sized {
    fn Next();
    fn Reset();
}
pub trait IEnumReadyCallbackImpl: Sized {
    fn EnumReady();
}
pub trait IEnumResourcesImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumShellItemsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumSyncMgrConflictImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumSyncMgrEventsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumSyncMgrSyncItemsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumTravelLogEntryImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumerableViewImpl: Sized {
    fn SetEnumReadyCallback();
    fn CreateEnumIDListFromContents();
}
pub trait IExecuteCommandImpl: Sized {
    fn SetKeyState();
    fn SetParameters();
    fn SetPosition();
    fn SetShowWindow();
    fn SetNoShowUI();
    fn SetDirectory();
    fn Execute();
}
pub trait IExecuteCommandApplicationHostEnvironmentImpl: Sized {
    fn GetValue();
}
pub trait IExecuteCommandHostImpl: Sized {
    fn GetUIMode();
}
pub trait IExpDispSupportImpl: Sized {
    fn FindConnectionPoint();
    fn OnTranslateAccelerator();
    fn OnInvoke();
}
pub trait IExpDispSupportXPImpl: Sized {
    fn FindCIE4ConnectionPoint();
    fn OnTranslateAccelerator();
    fn OnInvoke();
}
pub trait IExplorerBrowserImpl: Sized {
    fn Initialize();
    fn Destroy();
    fn SetRect();
    fn SetPropertyBag();
    fn SetEmptyText();
    fn SetFolderSettings();
    fn Advise();
    fn Unadvise();
    fn SetOptions();
    fn GetOptions();
    fn BrowseToIDList();
    fn BrowseToObject();
    fn FillFromObject();
    fn RemoveAll();
    fn GetCurrentView();
}
pub trait IExplorerBrowserEventsImpl: Sized {
    fn OnNavigationPending();
    fn OnViewCreated();
    fn OnNavigationComplete();
    fn OnNavigationFailed();
}
pub trait IExplorerCommandImpl: Sized {
    fn GetTitle();
    fn GetIcon();
    fn GetToolTip();
    fn GetCanonicalName();
    fn GetState();
    fn Invoke();
    fn GetFlags();
    fn EnumSubCommands();
}
pub trait IExplorerCommandProviderImpl: Sized {
    fn GetCommands();
    fn GetCommand();
}
pub trait IExplorerCommandStateImpl: Sized {
    fn GetState();
}
pub trait IExplorerPaneVisibilityImpl: Sized {
    fn GetPaneState();
}
pub trait IExtensionServicesImpl: Sized {
    fn SetAdditionalHeaders();
    fn SetAuthenticateData();
}
pub trait IExtractIconAImpl: Sized {
    fn GetIconLocation();
    fn Extract();
}
pub trait IExtractIconWImpl: Sized {
    fn GetIconLocation();
    fn Extract();
}
pub trait IExtractImageImpl: Sized {
    fn GetLocation();
    fn Extract();
}
pub trait IExtractImage2Impl: Sized + IExtractImageImpl {
    fn GetDateStamp();
}
pub trait IFileDialogImpl: Sized + IModalWindowImpl {
    fn SetFileTypes();
    fn SetFileTypeIndex();
    fn GetFileTypeIndex();
    fn Advise();
    fn Unadvise();
    fn SetOptions();
    fn GetOptions();
    fn SetDefaultFolder();
    fn SetFolder();
    fn GetFolder();
    fn GetCurrentSelection();
    fn SetFileName();
    fn GetFileName();
    fn SetTitle();
    fn SetOkButtonLabel();
    fn SetFileNameLabel();
    fn GetResult();
    fn AddPlace();
    fn SetDefaultExtension();
    fn Close();
    fn SetClientGuid();
    fn ClearClientData();
    fn SetFilter();
}
pub trait IFileDialog2Impl: Sized + IFileDialogImpl + IModalWindowImpl {
    fn SetCancelButtonLabel();
    fn SetNavigationRoot();
}
pub trait IFileDialogControlEventsImpl: Sized {
    fn OnItemSelected();
    fn OnButtonClicked();
    fn OnCheckButtonToggled();
    fn OnControlActivating();
}
pub trait IFileDialogCustomizeImpl: Sized {
    fn EnableOpenDropDown();
    fn AddMenu();
    fn AddPushButton();
    fn AddComboBox();
    fn AddRadioButtonList();
    fn AddCheckButton();
    fn AddEditBox();
    fn AddSeparator();
    fn AddText();
    fn SetControlLabel();
    fn GetControlState();
    fn SetControlState();
    fn GetEditBoxText();
    fn SetEditBoxText();
    fn GetCheckButtonState();
    fn SetCheckButtonState();
    fn AddControlItem();
    fn RemoveControlItem();
    fn RemoveAllControlItems();
    fn GetControlItemState();
    fn SetControlItemState();
    fn GetSelectedControlItem();
    fn SetSelectedControlItem();
    fn StartVisualGroup();
    fn EndVisualGroup();
    fn MakeProminent();
    fn SetControlItemText();
}
pub trait IFileDialogEventsImpl: Sized {
    fn OnFileOk();
    fn OnFolderChanging();
    fn OnFolderChange();
    fn OnSelectionChange();
    fn OnShareViolation();
    fn OnTypeChange();
    fn OnOverwrite();
}
pub trait IFileIsInUseImpl: Sized {
    fn GetAppName();
    fn GetUsage();
    fn GetCapabilities();
    fn GetSwitchToHWND();
    fn CloseFile();
}
pub trait IFileOpenDialogImpl: Sized + IFileDialogImpl + IModalWindowImpl {
    fn GetResults();
    fn GetSelectedItems();
}
pub trait IFileOperationImpl: Sized {
    fn Advise();
    fn Unadvise();
    fn SetOperationFlags();
    fn SetProgressMessage();
    fn SetProgressDialog();
    fn SetProperties();
    fn SetOwnerWindow();
    fn ApplyPropertiesToItem();
    fn ApplyPropertiesToItems();
    fn RenameItem();
    fn RenameItems();
    fn MoveItem();
    fn MoveItems();
    fn CopyItem();
    fn CopyItems();
    fn DeleteItem();
    fn DeleteItems();
    fn NewItem();
    fn PerformOperations();
    fn GetAnyOperationsAborted();
}
pub trait IFileOperation2Impl: Sized + IFileOperationImpl {
    fn SetOperationFlags2();
}
pub trait IFileOperationProgressSinkImpl: Sized {
    fn StartOperations();
    fn FinishOperations();
    fn PreRenameItem();
    fn PostRenameItem();
    fn PreMoveItem();
    fn PostMoveItem();
    fn PreCopyItem();
    fn PostCopyItem();
    fn PreDeleteItem();
    fn PostDeleteItem();
    fn PreNewItem();
    fn PostNewItem();
    fn UpdateProgress();
    fn ResetTimer();
    fn PauseTimer();
    fn ResumeTimer();
}
pub trait IFileSaveDialogImpl: Sized + IFileDialogImpl + IModalWindowImpl {
    fn SetSaveAsItem();
    fn SetProperties();
    fn SetCollectedProperties();
    fn GetProperties();
    fn ApplyProperties();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFileSearchBandImpl: Sized + IDispatchImpl {
    fn SetFocus();
    fn SetSearchParameters();
    fn SearchID();
    fn Scope();
    fn QueryFile();
}
pub trait IFileSyncMergeHandlerImpl: Sized {
    fn Merge();
    fn ShowResolveConflictUIAsync();
}
pub trait IFileSystemBindDataImpl: Sized {
    fn SetFindData();
    fn GetFindData();
}
pub trait IFileSystemBindData2Impl: Sized + IFileSystemBindDataImpl {
    fn SetFileID();
    fn GetFileID();
    fn SetJunctionCLSID();
    fn GetJunctionCLSID();
}
pub trait IFolderBandPrivImpl: Sized {
    fn SetCascade();
    fn SetAccelerators();
    fn SetNoIcons();
    fn SetNoText();
}
pub trait IFolderFilterImpl: Sized {
    fn ShouldShow();
    fn GetEnumFlags();
}
pub trait IFolderFilterSiteImpl: Sized {
    fn SetFilter();
}
pub trait IFolderViewImpl: Sized {
    fn GetCurrentViewMode();
    fn SetCurrentViewMode();
    fn GetFolder();
    fn Item();
    fn ItemCount();
    fn Items();
    fn GetSelectionMarkedItem();
    fn GetFocusedItem();
    fn GetItemPosition();
    fn GetSpacing();
    fn GetDefaultSpacing();
    fn GetAutoArrange();
    fn SelectItem();
    fn SelectAndPositionItems();
}
pub trait IFolderView2Impl: Sized + IFolderViewImpl {
    fn SetGroupBy();
    fn GetGroupBy();
    fn SetViewProperty();
    fn GetViewProperty();
    fn SetTileViewProperties();
    fn SetExtendedTileViewProperties();
    fn SetText();
    fn SetCurrentFolderFlags();
    fn GetCurrentFolderFlags();
    fn GetSortColumnCount();
    fn SetSortColumns();
    fn GetSortColumns();
    fn GetItem();
    fn GetVisibleItem();
    fn GetSelectedItem();
    fn GetSelection();
    fn GetSelectionState();
    fn InvokeVerbOnSelection();
    fn SetViewModeAndIconSize();
    fn GetViewModeAndIconSize();
    fn SetGroupSubsetCount();
    fn GetGroupSubsetCount();
    fn SetRedraw();
    fn IsMoveInSameFolder();
    fn DoRename();
}
pub trait IFolderViewHostImpl: Sized {
    fn Initialize();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFolderViewOCImpl: Sized + IDispatchImpl {
    fn SetFolderView();
}
pub trait IFolderViewOptionsImpl: Sized {
    fn SetFolderViewOptions();
    fn GetFolderViewOptions();
}
pub trait IFolderViewSettingsImpl: Sized {
    fn GetColumnPropertyList();
    fn GetGroupByProperty();
    fn GetViewMode();
    fn GetIconSize();
    fn GetFolderFlags();
    fn GetSortColumns();
    fn GetGroupSubsetCount();
}
pub trait IFrameworkInputPaneImpl: Sized {
    fn Advise();
    fn AdviseWithHWND();
    fn Unadvise();
    fn Location();
}
pub trait IFrameworkInputPaneHandlerImpl: Sized {
    fn Showing();
    fn Hiding();
}
pub trait IGetServiceIdsImpl: Sized {
    fn GetServiceIds();
}
pub trait IHWEventHandlerImpl: Sized {
    fn Initialize();
    fn HandleEvent();
    fn HandleEventWithContent();
}
pub trait IHWEventHandler2Impl: Sized + IHWEventHandlerImpl {
    fn HandleEventWithHWND();
}
pub trait IHandlerActivationHostImpl: Sized {
    fn BeforeCoCreateInstance();
    fn BeforeCreateProcess();
}
pub trait IHandlerInfoImpl: Sized {
    fn GetApplicationDisplayName();
    fn GetApplicationPublisher();
    fn GetApplicationIconReference();
}
pub trait IHandlerInfo2Impl: Sized + IHandlerInfoImpl {
    fn GetApplicationId();
}
pub trait IHlinkImpl: Sized {
    fn SetHlinkSite();
    fn GetHlinkSite();
    fn SetMonikerReference();
    fn GetMonikerReference();
    fn SetStringReference();
    fn GetStringReference();
    fn SetFriendlyName();
    fn GetFriendlyName();
    fn SetTargetFrameName();
    fn GetTargetFrameName();
    fn GetMiscStatus();
    fn Navigate();
    fn SetAdditionalParams();
    fn GetAdditionalParams();
}
pub trait IHlinkBrowseContextImpl: Sized {
    fn Register();
    fn GetObject();
    fn Revoke();
    fn SetBrowseWindowInfo();
    fn GetBrowseWindowInfo();
    fn SetInitialHlink();
    fn OnNavigateHlink();
    fn UpdateHlink();
    fn EnumNavigationStack();
    fn QueryHlink();
    fn GetHlink();
    fn SetCurrentHlink();
    fn Clone();
    fn Close();
}
pub trait IHlinkFrameImpl: Sized {
    fn SetBrowseContext();
    fn GetBrowseContext();
    fn Navigate();
    fn OnNavigate();
    fn UpdateHlink();
}
pub trait IHlinkSiteImpl: Sized {
    fn QueryService();
    fn GetMoniker();
    fn ReadyToNavigate();
    fn OnNavigationComplete();
}
pub trait IHlinkTargetImpl: Sized {
    fn SetBrowseContext();
    fn GetBrowseContext();
    fn Navigate();
    fn GetMoniker();
    fn GetFriendlyName();
}
pub trait IHomeGroupImpl: Sized {
    fn IsMember();
    fn ShowSharingWizard();
}
pub trait IIOCancelInformationImpl: Sized {
    fn SetCancelInformation();
    fn GetCancelInformation();
}
pub trait IIdentityNameImpl: Sized + IRelatedItemImpl {}
pub trait IImageRecompressImpl: Sized {
    fn RecompressImage();
}
pub trait IInitializeCommandImpl: Sized {
    fn Initialize();
}
pub trait IInitializeNetworkFolderImpl: Sized {
    fn Initialize();
}
pub trait IInitializeObjectImpl: Sized {
    fn Initialize();
}
pub trait IInitializeWithBindCtxImpl: Sized {
    fn Initialize();
}
pub trait IInitializeWithItemImpl: Sized {
    fn Initialize();
}
pub trait IInitializeWithPropertyStoreImpl: Sized {
    fn Initialize();
}
pub trait IInitializeWithWindowImpl: Sized {
    fn Initialize();
}
pub trait IInputObjectImpl: Sized {
    fn UIActivateIO();
    fn HasFocusIO();
    fn TranslateAcceleratorIO();
}
pub trait IInputObject2Impl: Sized + IInputObjectImpl {
    fn TranslateAcceleratorGlobal();
}
pub trait IInputObjectSiteImpl: Sized {
    fn OnFocusChangeIS();
}
pub trait IInputPaneAnimationCoordinatorImpl: Sized {
    fn AddAnimation();
}
pub trait IInputPanelConfigurationImpl: Sized {
    fn EnableFocusTracking();
}
pub trait IInputPanelInvocationConfigurationImpl: Sized {
    fn RequireTouchInEditControl();
}
pub trait IInsertItemImpl: Sized {
    fn InsertItem();
}
pub trait IItemNameLimitsImpl: Sized {
    fn GetValidCharacters();
    fn GetMaxLength();
}
pub trait IKnownFolderImpl: Sized {
    fn GetId();
    fn GetCategory();
    fn GetShellItem();
    fn GetPath();
    fn SetPath();
    fn GetIDList();
    fn GetFolderType();
    fn GetRedirectionCapabilities();
    fn GetFolderDefinition();
}
pub trait IKnownFolderManagerImpl: Sized {
    fn FolderIdFromCsidl();
    fn FolderIdToCsidl();
    fn GetFolderIds();
    fn GetFolder();
    fn GetFolderByName();
    fn RegisterFolder();
    fn UnregisterFolder();
    fn FindFolderFromPath();
    fn FindFolderFromIDList();
    fn Redirect();
}
pub trait ILaunchSourceAppUserModelIdImpl: Sized {
    fn GetAppUserModelId();
}
pub trait ILaunchSourceViewSizePreferenceImpl: Sized {
    fn GetSourceViewToPosition();
    fn GetSourceViewSizePreference();
}
pub trait ILaunchTargetMonitorImpl: Sized {
    fn GetMonitor();
}
pub trait ILaunchTargetViewSizePreferenceImpl: Sized {
    fn GetTargetViewSizePreference();
}
pub trait ILaunchUIContextImpl: Sized {
    fn SetAssociatedWindow();
    fn SetTabGroupingPreference();
}
pub trait ILaunchUIContextProviderImpl: Sized {
    fn UpdateContext();
}
pub trait IMenuBandImpl: Sized {
    fn IsMenuMessage();
    fn TranslateMenuMessage();
}
#[cfg(feature = "Win32_System_Ole")]
pub trait IMenuPopupImpl: Sized + IDeskBarImpl + IOleWindowImpl {
    fn Popup();
    fn OnSelect();
    fn SetSubMenu();
}
pub trait IModalWindowImpl: Sized {
    fn Show();
}
pub trait INameSpaceTreeAccessibleImpl: Sized {
    fn OnGetDefaultAccessibilityAction();
    fn OnDoDefaultAccessibilityAction();
    fn OnGetAccessibilityRole();
}
pub trait INameSpaceTreeControlImpl: Sized {
    fn Initialize();
    fn TreeAdvise();
    fn TreeUnadvise();
    fn AppendRoot();
    fn InsertRoot();
    fn RemoveRoot();
    fn RemoveAllRoots();
    fn GetRootItems();
    fn SetItemState();
    fn GetItemState();
    fn GetSelectedItems();
    fn GetItemCustomState();
    fn SetItemCustomState();
    fn EnsureItemVisible();
    fn SetTheme();
    fn GetNextItem();
    fn HitTest();
    fn GetItemRect();
    fn CollapseAll();
}
pub trait INameSpaceTreeControl2Impl: Sized + INameSpaceTreeControlImpl {
    fn SetControlStyle();
    fn GetControlStyle();
    fn SetControlStyle2();
    fn GetControlStyle2();
}
pub trait INameSpaceTreeControlCustomDrawImpl: Sized {
    fn PrePaint();
    fn PostPaint();
    fn ItemPrePaint();
    fn ItemPostPaint();
}
pub trait INameSpaceTreeControlDropHandlerImpl: Sized {
    fn OnDragEnter();
    fn OnDragOver();
    fn OnDragPosition();
    fn OnDrop();
    fn OnDropPosition();
    fn OnDragLeave();
}
pub trait INameSpaceTreeControlEventsImpl: Sized {
    fn OnItemClick();
    fn OnPropertyItemCommit();
    fn OnItemStateChanging();
    fn OnItemStateChanged();
    fn OnSelectionChanged();
    fn OnKeyboardInput();
    fn OnBeforeExpand();
    fn OnAfterExpand();
    fn OnBeginLabelEdit();
    fn OnEndLabelEdit();
    fn OnGetToolTip();
    fn OnBeforeItemDelete();
    fn OnItemAdded();
    fn OnItemDeleted();
    fn OnBeforeContextMenu();
    fn OnAfterContextMenu();
    fn OnBeforeStateImageChange();
    fn OnGetDefaultIconIndex();
}
pub trait INameSpaceTreeControlFolderCapabilitiesImpl: Sized {
    fn GetFolderCapabilities();
}
pub trait INamedPropertyBagImpl: Sized {
    fn ReadPropertyNPB();
    fn WritePropertyNPB();
    fn RemovePropertyNPB();
}
pub trait INamespaceWalkImpl: Sized {
    fn Walk();
    fn GetIDArrayResult();
}
pub trait INamespaceWalkCBImpl: Sized {
    fn FoundItem();
    fn EnterFolder();
    fn LeaveFolder();
    fn InitializeProgressDialog();
}
pub trait INamespaceWalkCB2Impl: Sized + INamespaceWalkCBImpl {
    fn WalkComplete();
}
pub trait INetworkFolderInternalImpl: Sized {
    fn GetResourceDisplayType();
    fn GetIDList();
    fn GetProvider();
}
pub trait INewMenuClientImpl: Sized {
    fn IncludeItems();
    fn SelectAndEditItem();
}
pub trait INewShortcutHookAImpl: Sized {
    fn SetReferent();
    fn GetReferent();
    fn SetFolder();
    fn GetFolder();
    fn GetName();
    fn GetExtension();
}
pub trait INewShortcutHookWImpl: Sized {
    fn SetReferent();
    fn GetReferent();
    fn SetFolder();
    fn GetFolder();
    fn GetName();
    fn GetExtension();
}
#[cfg(feature = "Win32_System_Com")]
pub trait INewWDEventsImpl: Sized + IWebWizardHostImpl + IDispatchImpl {
    fn PassportAuthenticate();
}
pub trait INewWindowManagerImpl: Sized {
    fn EvaluateNewWindow();
}
pub trait INotifyReplicaImpl: Sized {
    fn YouAreAReplica();
}
pub trait IObjMgrImpl: Sized {
    fn Append();
    fn Remove();
}
pub trait IObjectProviderImpl: Sized {
    fn QueryObject();
}
pub trait IObjectWithAppUserModelIDImpl: Sized {
    fn SetAppID();
    fn GetAppID();
}
pub trait IObjectWithBackReferencesImpl: Sized {
    fn RemoveBackReferences();
}
pub trait IObjectWithCancelEventImpl: Sized {
    fn GetCancelEvent();
}
pub trait IObjectWithFolderEnumModeImpl: Sized {
    fn SetMode();
    fn GetMode();
}
pub trait IObjectWithProgIDImpl: Sized {
    fn SetProgID();
    fn GetProgID();
}
pub trait IObjectWithSelectionImpl: Sized {
    fn SetSelection();
    fn GetSelection();
}
pub trait IOpenControlPanelImpl: Sized {
    fn Open();
    fn GetPath();
    fn GetCurrentView();
}
pub trait IOpenSearchSourceImpl: Sized {
    fn GetResults();
}
pub trait IOperationsProgressDialogImpl: Sized {
    fn StartProgressDialog();
    fn StopProgressDialog();
    fn SetOperation();
    fn SetMode();
    fn UpdateProgress();
    fn UpdateLocations();
    fn ResetTimer();
    fn PauseTimer();
    fn ResumeTimer();
    fn GetMilliseconds();
    fn GetOperationStatus();
}
pub trait IPackageDebugSettingsImpl: Sized {
    fn EnableDebugging();
    fn DisableDebugging();
    fn Suspend();
    fn Resume();
    fn TerminateAllProcesses();
    fn SetTargetSessionId();
    fn EnumerateBackgroundTasks();
    fn ActivateBackgroundTask();
    fn StartServicing();
    fn StopServicing();
    fn StartSessionRedirection();
    fn StopSessionRedirection();
    fn GetPackageExecutionState();
    fn RegisterForPackageStateChanges();
    fn UnregisterForPackageStateChanges();
}
pub trait IPackageDebugSettings2Impl: Sized + IPackageDebugSettingsImpl {
    fn EnumerateApps();
}
pub trait IPackageExecutionStateChangeNotificationImpl: Sized {
    fn OnStateChanged();
}
pub trait IParentAndItemImpl: Sized {
    fn SetParentAndItem();
    fn GetParentAndItem();
}
pub trait IParseAndCreateItemImpl: Sized {
    fn SetItem();
    fn GetItem();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPersistFolderImpl: Sized + IPersistImpl {
    fn Initialize();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPersistFolder2Impl: Sized + IPersistFolderImpl + IPersistImpl {
    fn GetCurFolder();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPersistFolder3Impl: Sized + IPersistFolder2Impl + IPersistFolderImpl + IPersistImpl {
    fn InitializeEx();
    fn GetFolderTargetInfo();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPersistIDListImpl: Sized + IPersistImpl {
    fn SetIDList();
    fn GetIDList();
}
pub trait IPreviewHandlerImpl: Sized {
    fn SetWindow();
    fn SetRect();
    fn DoPreview();
    fn Unload();
    fn SetFocus();
    fn QueryFocus();
    fn TranslateAccelerator();
}
pub trait IPreviewHandlerFrameImpl: Sized {
    fn GetWindowContext();
    fn TranslateAccelerator();
}
pub trait IPreviewHandlerVisualsImpl: Sized {
    fn SetBackgroundColor();
    fn SetFont();
    fn SetTextColor();
}
pub trait IPreviewItemImpl: Sized + IRelatedItemImpl {}
pub trait IPreviousVersionsInfoImpl: Sized {
    fn AreSnapshotsAvailable();
}
pub trait IProfferServiceImpl: Sized {
    fn ProfferService();
    fn RevokeService();
}
pub trait IProgressDialogImpl: Sized {
    fn StartProgressDialog();
    fn StopProgressDialog();
    fn SetTitle();
    fn SetAnimation();
    fn HasUserCancelled();
    fn SetProgress();
    fn SetProgress64();
    fn SetLine();
    fn SetCancelMsg();
    fn Timer();
}
pub trait IPropertyKeyStoreImpl: Sized {
    fn GetKeyCount();
    fn GetKeyAt();
    fn AppendKey();
    fn DeleteKey();
    fn IsKeyInStore();
    fn RemoveKey();
}
pub trait IPublishedAppImpl: Sized + IShellAppImpl {
    fn Install();
    fn GetPublishedAppInfo();
    fn Unschedule();
}
pub trait IPublishedApp2Impl: Sized + IPublishedAppImpl + IShellAppImpl {
    fn Install2();
}
pub trait IPublishingWizardImpl: Sized + IWizardExtensionImpl {
    fn Initialize();
    fn GetTransferManifest();
}
pub trait IQueryAssociationsImpl: Sized {
    fn Init();
    fn GetString();
    fn GetKey();
    fn GetData();
    fn GetEnum();
}
pub trait IQueryCancelAutoPlayImpl: Sized {
    fn AllowAutoPlay();
}
pub trait IQueryCodePageImpl: Sized {
    fn GetCodePage();
    fn SetCodePage();
}
pub trait IQueryContinueImpl: Sized {
    fn QueryContinue();
}
pub trait IQueryContinueWithStatusImpl: Sized + IQueryContinueImpl {
    fn SetStatusMessage();
}
pub trait IQueryInfoImpl: Sized {
    fn GetInfoTip();
    fn GetInfoFlags();
}
pub trait IRegTreeItemImpl: Sized {
    fn GetCheckState();
    fn SetCheckState();
}
pub trait IRelatedItemImpl: Sized {
    fn GetItemIDList();
    fn GetItem();
}
pub trait IRemoteComputerImpl: Sized {
    fn Initialize();
}
pub trait IResolveShellLinkImpl: Sized {
    fn ResolveShellLink();
}
pub trait IResultsFolderImpl: Sized {
    fn AddItem();
    fn AddIDList();
    fn RemoveItem();
    fn RemoveIDList();
    fn RemoveAll();
}
pub trait IRunnableTaskImpl: Sized {
    fn Run();
    fn Kill();
    fn Suspend();
    fn Resume();
    fn IsRunning();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IScriptErrorListImpl: Sized + IDispatchImpl {
    fn advanceError();
    fn retreatError();
    fn canAdvanceError();
    fn canRetreatError();
    fn getErrorLine();
    fn getErrorChar();
    fn getErrorCode();
    fn getErrorMsg();
    fn getErrorUrl();
    fn getAlwaysShowLockState();
    fn getDetailsPaneOpen();
    fn setDetailsPaneOpen();
    fn getPerErrorDisplay();
    fn setPerErrorDisplay();
}
pub trait ISearchBoxInfoImpl: Sized {
    fn GetCondition();
    fn GetText();
}
pub trait ISearchContextImpl: Sized {
    fn GetSearchUrl();
    fn GetSearchText();
    fn GetSearchStyle();
}
pub trait ISearchFolderItemFactoryImpl: Sized {
    fn SetDisplayName();
    fn SetFolderTypeID();
    fn SetFolderLogicalViewMode();
    fn SetIconSize();
    fn SetVisibleColumns();
    fn SetSortColumns();
    fn SetGroupColumn();
    fn SetStacks();
    fn SetScope();
    fn SetCondition();
    fn GetShellItem();
    fn GetIDList();
}
pub trait ISharedBitmapImpl: Sized {
    fn GetSharedBitmap();
    fn GetSize();
    fn GetFormat();
    fn InitializeBitmap();
    fn Detach();
}
pub trait ISharingConfigurationManagerImpl: Sized {
    fn CreateShare();
    fn DeleteShare();
    fn ShareExists();
    fn GetSharePermissions();
    fn SharePrinters();
    fn StopSharingPrinters();
    fn ArePrintersShared();
}
pub trait IShellAppImpl: Sized {
    fn GetAppInfo();
    fn GetPossibleActions();
    fn GetSlowAppInfo();
    fn GetCachedSlowAppInfo();
    fn IsInstalled();
}
#[cfg(feature = "Win32_System_Ole")]
pub trait IShellBrowserImpl: Sized + IOleWindowImpl {
    fn InsertMenusSB();
    fn SetMenuSB();
    fn RemoveMenusSB();
    fn SetStatusTextSB();
    fn EnableModelessSB();
    fn TranslateAcceleratorSB();
    fn BrowseObject();
    fn GetViewStateStream();
    fn GetControlWindow();
    fn SendControlMsg();
    fn QueryActiveShellView();
    fn OnViewWindowActive();
    fn SetToolbarItems();
}
pub trait IShellChangeNotifyImpl: Sized {
    fn OnChange();
}
pub trait IShellDetailsImpl: Sized {
    fn GetDetailsOf();
    fn ColumnClick();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IShellDispatchImpl: Sized + IDispatchImpl {
    fn Application();
    fn Parent();
    fn NameSpace();
    fn BrowseForFolder();
    fn Windows();
    fn Open();
    fn Explore();
    fn MinimizeAll();
    fn UndoMinimizeALL();
    fn FileRun();
    fn CascadeWindows();
    fn TileVertically();
    fn TileHorizontally();
    fn ShutdownWindows();
    fn Suspend();
    fn EjectPC();
    fn SetTime();
    fn TrayProperties();
    fn Help();
    fn FindFiles();
    fn FindComputer();
    fn RefreshMenu();
    fn ControlPanelItem();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IShellDispatch2Impl: Sized + IShellDispatchImpl + IDispatchImpl {
    fn IsRestricted();
    fn ShellExecute();
    fn FindPrinter();
    fn GetSystemInformation();
    fn ServiceStart();
    fn ServiceStop();
    fn IsServiceRunning();
    fn CanStartStopService();
    fn ShowBrowserBar();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IShellDispatch3Impl: Sized + IShellDispatch2Impl + IShellDispatchImpl + IDispatchImpl {
    fn AddToRecent();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IShellDispatch4Impl: Sized + IShellDispatch3Impl + IShellDispatch2Impl + IShellDispatchImpl + IDispatchImpl {
    fn WindowsSecurity();
    fn ToggleDesktop();
    fn ExplorerPolicy();
    fn GetSetting();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IShellDispatch5Impl: Sized + IShellDispatch4Impl + IShellDispatch3Impl + IShellDispatch2Impl + IShellDispatchImpl + IDispatchImpl {
    fn WindowSwitcher();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IShellDispatch6Impl: Sized + IShellDispatch5Impl + IShellDispatch4Impl + IShellDispatch3Impl + IShellDispatch2Impl + IShellDispatchImpl + IDispatchImpl {
    fn SearchCommand();
}
pub trait IShellExtInitImpl: Sized {
    fn Initialize();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IShellFavoritesNameSpaceImpl: Sized + IDispatchImpl {
    fn MoveSelectionUp();
    fn MoveSelectionDown();
    fn ResetSort();
    fn NewFolder();
    fn Synchronize();
    fn Import();
    fn Export();
    fn InvokeContextMenuCommand();
    fn MoveSelectionTo();
    fn SubscriptionsEnabled();
    fn CreateSubscriptionForSelection();
    fn DeleteSubscriptionForSelection();
    fn SetRoot();
}
pub trait IShellFolderImpl: Sized {
    fn ParseDisplayName();
    fn EnumObjects();
    fn BindToObject();
    fn BindToStorage();
    fn CompareIDs();
    fn CreateViewObject();
    fn GetAttributesOf();
    fn GetUIObjectOf();
    fn GetDisplayNameOf();
    fn SetNameOf();
}
pub trait IShellFolder2Impl: Sized + IShellFolderImpl {
    fn GetDefaultSearchGUID();
    fn EnumSearches();
    fn GetDefaultColumn();
    fn GetDefaultColumnState();
    fn GetDetailsEx();
    fn GetDetailsOf();
    fn MapColumnToSCID();
}
pub trait IShellFolderBandImpl: Sized {
    fn InitializeSFB();
    fn SetBandInfoSFB();
    fn GetBandInfoSFB();
}
pub trait IShellFolderViewImpl: Sized {
    fn Rearrange();
    fn GetArrangeParam();
    fn ArrangeGrid();
    fn AutoArrange();
    fn GetAutoArrange();
    fn AddObject();
    fn GetObject();
    fn RemoveObject();
    fn GetObjectCount();
    fn SetObjectCount();
    fn UpdateObject();
    fn RefreshObject();
    fn SetRedraw();
    fn GetSelectedCount();
    fn GetSelectedObjects();
    fn IsDropOnSource();
    fn GetDragPoint();
    fn GetDropPoint();
    fn MoveIcons();
    fn SetItemPos();
    fn IsBkDropTarget();
    fn SetClipboard();
    fn SetPoints();
    fn GetItemSpacing();
    fn SetCallback();
    fn Select();
    fn QuerySupport();
    fn SetAutomationObject();
}
pub trait IShellFolderViewCBImpl: Sized {
    fn MessageSFVCB();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IShellFolderViewDualImpl: Sized + IDispatchImpl {
    fn Application();
    fn Parent();
    fn Folder();
    fn SelectedItems();
    fn FocusedItem();
    fn SelectItem();
    fn PopupItemMenu();
    fn Script();
    fn ViewOptions();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IShellFolderViewDual2Impl: Sized + IShellFolderViewDualImpl + IDispatchImpl {
    fn CurrentViewMode();
    fn SetCurrentViewMode();
    fn SelectItemRelative();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IShellFolderViewDual3Impl: Sized + IShellFolderViewDual2Impl + IShellFolderViewDualImpl + IDispatchImpl {
    fn GroupBy();
    fn SetGroupBy();
    fn FolderFlags();
    fn SetFolderFlags();
    fn SortColumns();
    fn SetSortColumns();
    fn SetIconSize();
    fn IconSize();
    fn FilterView();
}
pub trait IShellIconImpl: Sized {
    fn GetIconOf();
}
pub trait IShellIconOverlayImpl: Sized {
    fn GetOverlayIndex();
    fn GetOverlayIconIndex();
}
pub trait IShellIconOverlayIdentifierImpl: Sized {
    fn IsMemberOf();
    fn GetOverlayInfo();
    fn GetPriority();
}
pub trait IShellIconOverlayManagerImpl: Sized {
    fn GetFileOverlayInfo();
    fn GetReservedOverlayInfo();
    fn RefreshOverlayImages();
    fn LoadNonloadedOverlayIdentifiers();
    fn OverlayIndexFromImageIndex();
}
pub trait IShellImageDataImpl: Sized {
    fn Decode();
    fn Draw();
    fn NextFrame();
    fn NextPage();
    fn PrevPage();
    fn IsTransparent();
    fn IsAnimated();
    fn IsVector();
    fn IsMultipage();
    fn IsEditable();
    fn IsPrintable();
    fn IsDecoded();
    fn GetCurrentPage();
    fn GetPageCount();
    fn SelectPage();
    fn GetSize();
    fn GetRawDataFormat();
    fn GetPixelFormat();
    fn GetDelay();
    fn GetProperties();
    fn Rotate();
    fn Scale();
    fn DiscardEdit();
    fn SetEncoderParams();
    fn DisplayName();
    fn GetResolution();
    fn GetEncoderParams();
    fn RegisterAbort();
    fn CloneFrame();
    fn ReplaceFrame();
}
pub trait IShellImageDataAbortImpl: Sized {
    fn QueryAbort();
}
pub trait IShellImageDataFactoryImpl: Sized {
    fn CreateIShellImageData();
    fn CreateImageFromFile();
    fn CreateImageFromStream();
    fn GetDataFormatFromPath();
}
pub trait IShellItemImpl: Sized {
    fn BindToHandler();
    fn GetParent();
    fn GetDisplayName();
    fn GetAttributes();
    fn Compare();
}
pub trait IShellItem2Impl: Sized + IShellItemImpl {
    fn GetPropertyStore();
    fn GetPropertyStoreWithCreateObject();
    fn GetPropertyStoreForKeys();
    fn GetPropertyDescriptionList();
    fn Update();
    fn GetProperty();
    fn GetCLSID();
    fn GetFileTime();
    fn GetInt32();
    fn GetString();
    fn GetUInt32();
    fn GetUInt64();
    fn GetBool();
}
pub trait IShellItemArrayImpl: Sized {
    fn BindToHandler();
    fn GetPropertyStore();
    fn GetPropertyDescriptionList();
    fn GetAttributes();
    fn GetCount();
    fn GetItemAt();
    fn EnumItems();
}
pub trait IShellItemFilterImpl: Sized {
    fn IncludeItem();
    fn GetEnumFlagsForItem();
}
pub trait IShellItemImageFactoryImpl: Sized {
    fn GetImage();
}
pub trait IShellItemResourcesImpl: Sized {
    fn GetAttributes();
    fn GetSize();
    fn GetTimes();
    fn SetTimes();
    fn GetResourceDescription();
    fn EnumResources();
    fn SupportsResource();
    fn OpenResource();
    fn CreateResource();
    fn MarkForDelete();
}
pub trait IShellLibraryImpl: Sized {
    fn LoadLibraryFromItem();
    fn LoadLibraryFromKnownFolder();
    fn AddFolder();
    fn RemoveFolder();
    fn GetFolders();
    fn ResolveFolder();
    fn GetDefaultSaveFolder();
    fn SetDefaultSaveFolder();
    fn GetOptions();
    fn SetOptions();
    fn GetFolderType();
    fn SetFolderType();
    fn GetIcon();
    fn SetIcon();
    fn Commit();
    fn Save();
    fn SaveInKnownFolder();
}
pub trait IShellLinkAImpl: Sized {
    fn GetPath();
    fn GetIDList();
    fn SetIDList();
    fn GetDescription();
    fn SetDescription();
    fn GetWorkingDirectory();
    fn SetWorkingDirectory();
    fn GetArguments();
    fn SetArguments();
    fn GetHotkey();
    fn SetHotkey();
    fn GetShowCmd();
    fn SetShowCmd();
    fn GetIconLocation();
    fn SetIconLocation();
    fn SetRelativePath();
    fn Resolve();
    fn SetPath();
}
pub trait IShellLinkDataListImpl: Sized {
    fn AddDataBlock();
    fn CopyDataBlock();
    fn RemoveDataBlock();
    fn GetFlags();
    fn SetFlags();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IShellLinkDualImpl: Sized + IDispatchImpl {
    fn Path();
    fn SetPath();
    fn Description();
    fn SetDescription();
    fn WorkingDirectory();
    fn SetWorkingDirectory();
    fn Arguments();
    fn SetArguments();
    fn Hotkey();
    fn SetHotkey();
    fn ShowCommand();
    fn SetShowCommand();
    fn Resolve();
    fn GetIconLocation();
    fn SetIconLocation();
    fn Save();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IShellLinkDual2Impl: Sized + IShellLinkDualImpl + IDispatchImpl {
    fn Target();
}
pub trait IShellLinkWImpl: Sized {
    fn GetPath();
    fn GetIDList();
    fn SetIDList();
    fn GetDescription();
    fn SetDescription();
    fn GetWorkingDirectory();
    fn SetWorkingDirectory();
    fn GetArguments();
    fn SetArguments();
    fn GetHotkey();
    fn SetHotkey();
    fn GetShowCmd();
    fn SetShowCmd();
    fn GetIconLocation();
    fn SetIconLocation();
    fn SetRelativePath();
    fn Resolve();
    fn SetPath();
}
pub trait IShellMenuImpl: Sized {
    fn Initialize();
    fn GetMenuInfo();
    fn SetShellFolder();
    fn GetShellFolder();
    fn SetMenu();
    fn GetMenu();
    fn InvalidateItem();
    fn GetState();
    fn SetMenuToolbar();
}
pub trait IShellMenuCallbackImpl: Sized {
    fn CallbackSM();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IShellNameSpaceImpl: Sized + IShellFavoritesNameSpaceImpl + IDispatchImpl {
    fn EnumOptions();
    fn SetEnumOptions();
    fn SelectedItem();
    fn SetSelectedItem();
    fn Root();
    fn SetRoot();
    fn Depth();
    fn SetDepth();
    fn Mode();
    fn SetMode();
    fn Flags();
    fn SetFlags();
    fn SetTVFlags();
    fn TVFlags();
    fn Columns();
    fn SetColumns();
    fn CountViewTypes();
    fn SetViewType();
    fn SelectedItems();
    fn Expand();
    fn UnselectAll();
}
pub trait IShellPropSheetExtImpl: Sized {
    fn AddPages();
    fn ReplacePage();
}
pub trait IShellRunDllImpl: Sized {
    fn Run();
}
pub trait IShellServiceImpl: Sized {
    fn SetOwner();
}
pub trait IShellTaskSchedulerImpl: Sized {
    fn AddTask();
    fn RemoveTasks();
    fn CountTasks();
    fn Status();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IShellUIHelperImpl: Sized + IDispatchImpl {
    fn ResetFirstBootMode();
    fn ResetSafeMode();
    fn RefreshOfflineDesktop();
    fn AddFavorite();
    fn AddChannel();
    fn AddDesktopComponent();
    fn IsSubscribed();
    fn NavigateAndFind();
    fn ImportExportFavorites();
    fn AutoCompleteSaveForm();
    fn AutoScan();
    fn AutoCompleteAttach();
    fn ShowBrowserUI();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IShellUIHelper2Impl: Sized + IShellUIHelperImpl + IDispatchImpl {
    fn AddSearchProvider();
    fn RunOnceShown();
    fn SkipRunOnce();
    fn CustomizeSettings();
    fn SqmEnabled();
    fn PhishingEnabled();
    fn BrandImageUri();
    fn SkipTabsWelcome();
    fn DiagnoseConnection();
    fn CustomizeClearType();
    fn IsSearchProviderInstalled();
    fn IsSearchMigrated();
    fn DefaultSearchProvider();
    fn RunOnceRequiredSettingsComplete();
    fn RunOnceHasShown();
    fn SearchGuideUrl();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IShellUIHelper3Impl: Sized + IShellUIHelper2Impl + IShellUIHelperImpl + IDispatchImpl {
    fn AddService();
    fn IsServiceInstalled();
    fn InPrivateFilteringEnabled();
    fn AddToFavoritesBar();
    fn BuildNewTabPage();
    fn SetRecentlyClosedVisible();
    fn SetActivitiesVisible();
    fn ContentDiscoveryReset();
    fn IsSuggestedSitesEnabled();
    fn EnableSuggestedSites();
    fn NavigateToSuggestedSites();
    fn ShowTabsHelp();
    fn ShowInPrivateHelp();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IShellUIHelper4Impl: Sized + IShellUIHelper3Impl + IShellUIHelper2Impl + IShellUIHelperImpl + IDispatchImpl {
    fn msIsSiteMode();
    fn msSiteModeShowThumbBar();
    fn msSiteModeAddThumbBarButton();
    fn msSiteModeUpdateThumbBarButton();
    fn msSiteModeSetIconOverlay();
    fn msSiteModeClearIconOverlay();
    fn msAddSiteMode();
    fn msSiteModeCreateJumpList();
    fn msSiteModeAddJumpListItem();
    fn msSiteModeClearJumpList();
    fn msSiteModeShowJumpList();
    fn msSiteModeAddButtonStyle();
    fn msSiteModeShowButtonStyle();
    fn msSiteModeActivate();
    fn msIsSiteModeFirstRun();
    fn msAddTrackingProtectionList();
    fn msTrackingProtectionEnabled();
    fn msActiveXFilteringEnabled();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IShellUIHelper5Impl: Sized + IShellUIHelper4Impl + IShellUIHelper3Impl + IShellUIHelper2Impl + IShellUIHelperImpl + IDispatchImpl {
    fn msProvisionNetworks();
    fn msReportSafeUrl();
    fn msSiteModeRefreshBadge();
    fn msSiteModeClearBadge();
    fn msDiagnoseConnectionUILess();
    fn msLaunchNetworkClientHelp();
    fn msChangeDefaultBrowser();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IShellUIHelper6Impl: Sized + IShellUIHelper5Impl + IShellUIHelper4Impl + IShellUIHelper3Impl + IShellUIHelper2Impl + IShellUIHelperImpl + IDispatchImpl {
    fn msStopPeriodicTileUpdate();
    fn msStartPeriodicTileUpdate();
    fn msStartPeriodicTileUpdateBatch();
    fn msClearTile();
    fn msEnableTileNotificationQueue();
    fn msPinnedSiteState();
    fn msEnableTileNotificationQueueForSquare150x150();
    fn msEnableTileNotificationQueueForWide310x150();
    fn msEnableTileNotificationQueueForSquare310x310();
    fn msScheduledTileNotification();
    fn msRemoveScheduledTileNotification();
    fn msStartPeriodicBadgeUpdate();
    fn msStopPeriodicBadgeUpdate();
    fn msLaunchInternetOptions();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IShellUIHelper7Impl: Sized + IShellUIHelper6Impl + IShellUIHelper5Impl + IShellUIHelper4Impl + IShellUIHelper3Impl + IShellUIHelper2Impl + IShellUIHelperImpl + IDispatchImpl {
    fn SetExperimentalFlag();
    fn GetExperimentalFlag();
    fn SetExperimentalValue();
    fn GetExperimentalValue();
    fn ResetAllExperimentalFlagsAndValues();
    fn GetNeedIEAutoLaunchFlag();
    fn SetNeedIEAutoLaunchFlag();
    fn HasNeedIEAutoLaunchFlag();
    fn LaunchIE();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IShellUIHelper8Impl: Sized + IShellUIHelper7Impl + IShellUIHelper6Impl + IShellUIHelper5Impl + IShellUIHelper4Impl + IShellUIHelper3Impl + IShellUIHelper2Impl + IShellUIHelperImpl + IDispatchImpl {
    fn GetCVListData();
    fn GetCVListLocalData();
    fn GetEMIEListData();
    fn GetEMIEListLocalData();
    fn OpenFavoritesPane();
    fn OpenFavoritesSettings();
    fn LaunchInHVSI();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IShellUIHelper9Impl: Sized + IShellUIHelper8Impl + IShellUIHelper7Impl + IShellUIHelper6Impl + IShellUIHelper5Impl + IShellUIHelper4Impl + IShellUIHelper3Impl + IShellUIHelper2Impl + IShellUIHelperImpl + IDispatchImpl {
    fn GetOSSku();
}
#[cfg(feature = "Win32_System_Ole")]
pub trait IShellViewImpl: Sized + IOleWindowImpl {
    fn TranslateAccelerator();
    fn EnableModeless();
    fn UIActivate();
    fn Refresh();
    fn CreateViewWindow();
    fn DestroyViewWindow();
    fn GetCurrentInfo();
    fn AddPropertySheetPages();
    fn SaveViewState();
    fn SelectItem();
    fn GetItemObject();
}
#[cfg(feature = "Win32_System_Ole")]
pub trait IShellView2Impl: Sized + IShellViewImpl + IOleWindowImpl {
    fn GetView();
    fn CreateViewWindow2();
    fn HandleRename();
    fn SelectAndPositionItem();
}
#[cfg(feature = "Win32_System_Ole")]
pub trait IShellView3Impl: Sized + IShellView2Impl + IShellViewImpl + IOleWindowImpl {
    fn CreateViewWindow3();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IShellWindowsImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
    fn Register();
    fn RegisterPending();
    fn Revoke();
    fn OnNavigate();
    fn OnActivated();
    fn FindWindowSW();
    fn OnCreated();
    fn ProcessAttachDetach();
}
pub trait ISortColumnArrayImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn GetSortType();
}
pub trait IStartMenuPinnedListImpl: Sized {
    fn RemoveFromList();
}
pub trait IStorageProviderBannersImpl: Sized {
    fn SetBanner();
    fn ClearBanner();
    fn ClearAllBanners();
    fn GetBanner();
}
pub trait IStorageProviderCopyHookImpl: Sized {
    fn CopyCallback();
}
pub trait IStorageProviderHandlerImpl: Sized {
    fn GetPropertyHandlerFromPath();
    fn GetPropertyHandlerFromUri();
    fn GetPropertyHandlerFromFileId();
}
pub trait IStorageProviderPropertyHandlerImpl: Sized {
    fn RetrieveProperties();
    fn SaveProperties();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStreamAsyncImpl: Sized + IStreamImpl + ISequentialStreamImpl {
    fn ReadAsync();
    fn WriteAsync();
    fn OverlappedResult();
    fn CancelIo();
}
pub trait IStreamUnbufferedInfoImpl: Sized {
    fn GetSectorSize();
}
pub trait ISuspensionDependencyManagerImpl: Sized {
    fn RegisterAsChild();
    fn GroupChildWithParent();
    fn UngroupChildFromParent();
}
pub trait ISyncMgrConflictImpl: Sized {
    fn GetProperty();
    fn GetConflictIdInfo();
    fn GetItemsArray();
    fn Resolve();
    fn GetResolutionHandler();
}
pub trait ISyncMgrConflictFolderImpl: Sized {
    fn GetConflictIDList();
}
pub trait ISyncMgrConflictItemsImpl: Sized {
    fn GetCount();
    fn GetItem();
}
pub trait ISyncMgrConflictPresenterImpl: Sized {
    fn PresentConflict();
}
pub trait ISyncMgrConflictResolutionItemsImpl: Sized {
    fn GetCount();
    fn GetItem();
}
pub trait ISyncMgrConflictResolveInfoImpl: Sized {
    fn GetIterationInfo();
    fn GetPresenterNextStep();
    fn GetPresenterChoice();
    fn GetItemChoiceCount();
    fn GetItemChoice();
    fn SetPresenterNextStep();
    fn SetPresenterChoice();
    fn SetItemChoices();
}
pub trait ISyncMgrConflictStoreImpl: Sized {
    fn EnumConflicts();
    fn BindToConflict();
    fn RemoveConflicts();
    fn GetCount();
}
pub trait ISyncMgrControlImpl: Sized {
    fn StartHandlerSync();
    fn StartItemSync();
    fn StartSyncAll();
    fn StopHandlerSync();
    fn StopItemSync();
    fn StopSyncAll();
    fn UpdateHandlerCollection();
    fn UpdateHandler();
    fn UpdateItem();
    fn UpdateEvents();
    fn UpdateConflict();
    fn UpdateConflicts();
    fn ActivateHandler();
    fn EnableHandler();
    fn EnableItem();
}
pub trait ISyncMgrEnumItemsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait ISyncMgrEventImpl: Sized {
    fn GetEventID();
    fn GetHandlerID();
    fn GetItemID();
    fn GetLevel();
    fn GetFlags();
    fn GetTime();
    fn GetName();
    fn GetDescription();
    fn GetLinkText();
    fn GetLinkReference();
    fn GetContext();
}
pub trait ISyncMgrEventLinkUIOperationImpl: Sized + ISyncMgrUIOperationImpl {
    fn Init();
}
pub trait ISyncMgrEventStoreImpl: Sized {
    fn GetEventEnumerator();
    fn GetEventCount();
    fn GetEvent();
    fn RemoveEvent();
}
pub trait ISyncMgrHandlerImpl: Sized {
    fn GetName();
    fn GetHandlerInfo();
    fn GetObject();
    fn GetCapabilities();
    fn GetPolicies();
    fn Activate();
    fn Enable();
    fn Synchronize();
}
pub trait ISyncMgrHandlerCollectionImpl: Sized {
    fn GetHandlerEnumerator();
    fn BindToHandler();
}
pub trait ISyncMgrHandlerInfoImpl: Sized {
    fn GetType();
    fn GetTypeLabel();
    fn GetComment();
    fn GetLastSyncTime();
    fn IsActive();
    fn IsEnabled();
    fn IsConnected();
}
pub trait ISyncMgrRegisterImpl: Sized {
    fn RegisterSyncMgrHandler();
    fn UnregisterSyncMgrHandler();
    fn GetHandlerRegistrationInfo();
}
pub trait ISyncMgrResolutionHandlerImpl: Sized {
    fn QueryAbilities();
    fn KeepOther();
    fn KeepRecent();
    fn RemoveFromSyncSet();
    fn KeepItems();
}
pub trait ISyncMgrScheduleWizardUIOperationImpl: Sized + ISyncMgrUIOperationImpl {
    fn InitWizard();
}
pub trait ISyncMgrSessionCreatorImpl: Sized {
    fn CreateSession();
}
pub trait ISyncMgrSyncCallbackImpl: Sized {
    fn ReportProgress();
    fn SetHandlerProgressText();
    fn ReportEvent();
    fn CanContinue();
    fn QueryForAdditionalItems();
    fn AddItemToSession();
    fn AddIUnknownToSession();
    fn ProposeItem();
    fn CommitItem();
    fn ReportManualSync();
}
pub trait ISyncMgrSyncItemImpl: Sized {
    fn GetItemID();
    fn GetName();
    fn GetItemInfo();
    fn GetObject();
    fn GetCapabilities();
    fn GetPolicies();
    fn Enable();
    fn Delete();
}
pub trait ISyncMgrSyncItemContainerImpl: Sized {
    fn GetSyncItem();
    fn GetSyncItemEnumerator();
    fn GetSyncItemCount();
}
pub trait ISyncMgrSyncItemInfoImpl: Sized {
    fn GetTypeLabel();
    fn GetComment();
    fn GetLastSyncTime();
    fn IsEnabled();
    fn IsConnected();
}
pub trait ISyncMgrSyncResultImpl: Sized {
    fn Result();
}
pub trait ISyncMgrSynchronizeImpl: Sized {
    fn Initialize();
    fn GetHandlerInfo();
    fn EnumSyncMgrItems();
    fn GetItemObject();
    fn ShowProperties();
    fn SetProgressCallback();
    fn PrepareForSync();
    fn Synchronize();
    fn SetItemStatus();
    fn ShowError();
}
pub trait ISyncMgrSynchronizeCallbackImpl: Sized {
    fn ShowPropertiesCompleted();
    fn PrepareForSyncCompleted();
    fn SynchronizeCompleted();
    fn ShowErrorCompleted();
    fn EnableModeless();
    fn Progress();
    fn LogError();
    fn DeleteLogError();
    fn EstablishConnection();
}
pub trait ISyncMgrSynchronizeInvokeImpl: Sized {
    fn UpdateItems();
    fn UpdateAll();
}
pub trait ISyncMgrUIOperationImpl: Sized {
    fn Run();
}
pub trait ITaskbarListImpl: Sized {
    fn HrInit();
    fn AddTab();
    fn DeleteTab();
    fn ActivateTab();
    fn SetActiveAlt();
}
pub trait ITaskbarList2Impl: Sized + ITaskbarListImpl {
    fn MarkFullscreenWindow();
}
pub trait ITaskbarList3Impl: Sized + ITaskbarList2Impl + ITaskbarListImpl {
    fn SetProgressValue();
    fn SetProgressState();
    fn RegisterTab();
    fn UnregisterTab();
    fn SetTabOrder();
    fn SetTabActive();
    fn ThumbBarAddButtons();
    fn ThumbBarUpdateButtons();
    fn ThumbBarSetImageList();
    fn SetOverlayIcon();
    fn SetThumbnailTooltip();
    fn SetThumbnailClip();
}
pub trait ITaskbarList4Impl: Sized + ITaskbarList3Impl + ITaskbarList2Impl + ITaskbarListImpl {
    fn SetTabProperties();
}
pub trait IThumbnailCacheImpl: Sized {
    fn GetThumbnail();
    fn GetThumbnailByID();
}
pub trait IThumbnailCachePrimerImpl: Sized {
    fn PageInThumbnail();
}
pub trait IThumbnailCaptureImpl: Sized {
    fn CaptureThumbnail();
}
pub trait IThumbnailHandlerFactoryImpl: Sized {
    fn GetThumbnailHandler();
}
pub trait IThumbnailProviderImpl: Sized {
    fn GetThumbnail();
}
pub trait IThumbnailSettingsImpl: Sized {
    fn SetContext();
}
pub trait IThumbnailStreamCacheImpl: Sized {
    fn GetThumbnailStream();
    fn SetThumbnailStream();
}
pub trait ITrackShellMenuImpl: Sized + IShellMenuImpl {
    fn SetObscured();
    fn Popup();
}
pub trait ITranscodeImageImpl: Sized {
    fn TranscodeImage();
}
pub trait ITransferAdviseSinkImpl: Sized {
    fn UpdateProgress();
    fn UpdateTransferState();
    fn ConfirmOverwrite();
    fn ConfirmEncryptionLoss();
    fn FileFailure();
    fn SubStreamFailure();
    fn PropertyFailure();
}
pub trait ITransferDestinationImpl: Sized {
    fn Advise();
    fn Unadvise();
    fn CreateItem();
}
pub trait ITransferMediumItemImpl: Sized + IRelatedItemImpl {}
pub trait ITransferSourceImpl: Sized {
    fn Advise();
    fn Unadvise();
    fn SetProperties();
    fn OpenItem();
    fn MoveItem();
    fn RecycleItem();
    fn RemoveItem();
    fn RenameItem();
    fn LinkItem();
    fn ApplyPropertiesToItem();
    fn GetDefaultDestinationName();
    fn EnterFolder();
    fn LeaveFolder();
}
pub trait ITravelEntryImpl: Sized {
    fn Invoke();
    fn Update();
    fn GetPidl();
}
pub trait ITravelLogImpl: Sized {
    fn AddEntry();
    fn UpdateEntry();
    fn UpdateExternal();
    fn Travel();
    fn GetTravelEntry();
    fn FindTravelEntry();
    fn GetToolTipText();
    fn InsertMenuEntries();
    fn Clone();
    fn CountEntries();
    fn Revert();
}
pub trait ITravelLogClientImpl: Sized {
    fn FindWindowByIndex();
    fn GetWindowData();
    fn LoadHistoryPosition();
}
pub trait ITravelLogEntryImpl: Sized {
    fn GetTitle();
    fn GetURL();
}
pub trait ITravelLogStgImpl: Sized {
    fn CreateEntry();
    fn TravelTo();
    fn EnumEntries();
    fn FindEntries();
    fn GetCount();
    fn RemoveEntry();
    fn GetRelativeEntry();
}
pub trait ITrayDeskBandImpl: Sized {
    fn ShowDeskBand();
    fn HideDeskBand();
    fn IsDeskBandShown();
    fn DeskBandRegistrationChanged();
}
pub trait IURLSearchHookImpl: Sized {
    fn Translate();
}
pub trait IURLSearchHook2Impl: Sized + IURLSearchHookImpl {
    fn TranslateWithSearchContext();
}
pub trait IUniformResourceLocatorAImpl: Sized {
    fn SetURL();
    fn GetURL();
    fn InvokeCommand();
}
pub trait IUniformResourceLocatorWImpl: Sized {
    fn SetURL();
    fn GetURL();
    fn InvokeCommand();
}
pub trait IUpdateIDListImpl: Sized {
    fn Update();
}
pub trait IUseToBrowseItemImpl: Sized + IRelatedItemImpl {}
pub trait IUserAccountChangeCallbackImpl: Sized {
    fn OnPictureChange();
}
pub trait IUserNotificationImpl: Sized {
    fn SetBalloonInfo();
    fn SetBalloonRetry();
    fn SetIconInfo();
    fn Show();
    fn PlaySound();
}
pub trait IUserNotification2Impl: Sized {
    fn SetBalloonInfo();
    fn SetBalloonRetry();
    fn SetIconInfo();
    fn Show();
    fn PlaySound();
}
pub trait IUserNotificationCallbackImpl: Sized {
    fn OnBalloonUserClick();
    fn OnLeftClick();
    fn OnContextMenu();
}
pub trait IViewStateIdentityItemImpl: Sized + IRelatedItemImpl {}
pub trait IVirtualDesktopManagerImpl: Sized {
    fn IsWindowOnCurrentVirtualDesktop();
    fn GetWindowDesktopId();
    fn MoveWindowToDesktop();
}
pub trait IVisualPropertiesImpl: Sized {
    fn SetWatermark();
    fn SetColor();
    fn GetColor();
    fn SetItemHeight();
    fn GetItemHeight();
    fn SetFont();
    fn GetFont();
    fn SetTheme();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWebBrowserImpl: Sized + IDispatchImpl {
    fn GoBack();
    fn GoForward();
    fn GoHome();
    fn GoSearch();
    fn Navigate();
    fn Refresh();
    fn Refresh2();
    fn Stop();
    fn Application();
    fn Parent();
    fn Container();
    fn Document();
    fn TopLevelContainer();
    fn Type();
    fn Left();
    fn SetLeft();
    fn Top();
    fn SetTop();
    fn Width();
    fn SetWidth();
    fn Height();
    fn SetHeight();
    fn LocationName();
    fn LocationURL();
    fn Busy();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWebBrowser2Impl: Sized + IWebBrowserAppImpl + IWebBrowserImpl + IDispatchImpl {
    fn Navigate2();
    fn QueryStatusWB();
    fn ExecWB();
    fn ShowBrowserBar();
    fn ReadyState();
    fn Offline();
    fn SetOffline();
    fn Silent();
    fn SetSilent();
    fn RegisterAsBrowser();
    fn SetRegisterAsBrowser();
    fn RegisterAsDropTarget();
    fn SetRegisterAsDropTarget();
    fn TheaterMode();
    fn SetTheaterMode();
    fn AddressBar();
    fn SetAddressBar();
    fn Resizable();
    fn SetResizable();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWebBrowserAppImpl: Sized + IWebBrowserImpl + IDispatchImpl {
    fn Quit();
    fn ClientToWindow();
    fn PutProperty();
    fn GetProperty();
    fn Name();
    fn HWND();
    fn FullName();
    fn Path();
    fn Visible();
    fn SetVisible();
    fn StatusBar();
    fn SetStatusBar();
    fn StatusText();
    fn SetStatusText();
    fn ToolBar();
    fn SetToolBar();
    fn MenuBar();
    fn SetMenuBar();
    fn FullScreen();
    fn SetFullScreen();
}
pub trait IWebWizardExtensionImpl: Sized + IWizardExtensionImpl {
    fn SetInitialURL();
    fn SetErrorURL();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWebWizardHostImpl: Sized + IDispatchImpl {
    fn FinalBack();
    fn FinalNext();
    fn Cancel();
    fn SetCaption();
    fn Caption();
    fn SetProperty();
    fn Property();
    fn SetWizardButtons();
    fn SetHeaderText();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWebWizardHost2Impl: Sized + IWebWizardHostImpl + IDispatchImpl {
    fn SignString();
}
pub trait IWizardExtensionImpl: Sized {
    fn AddPages();
    fn GetFirstPage();
    fn GetLastPage();
}
pub trait IWizardSiteImpl: Sized {
    fn GetPreviousPage();
    fn GetNextPage();
    fn GetCancelledPage();
}
