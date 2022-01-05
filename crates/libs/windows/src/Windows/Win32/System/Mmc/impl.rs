#[cfg(feature = "Win32_System_Com")]
pub trait AppEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait ColumnImpl: Sized + IDispatchImpl {
    fn Name();
    fn Width();
    fn SetWidth();
    fn DisplayPosition();
    fn SetDisplayPosition();
    fn Hidden();
    fn SetHidden();
    fn SetAsSortColumn();
    fn IsSortColumn();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ColumnsImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ContextMenuImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
pub trait DocumentImpl: Sized + IDispatchImpl {
    fn Save();
    fn SaveAs();
    fn Close();
    fn Views();
    fn SnapIns();
    fn ActiveView();
    fn Name();
    fn SetName();
    fn Location();
    fn IsSaved();
    fn Mode();
    fn SetMode();
    fn RootNode();
    fn ScopeNamespace();
    fn CreateProperties();
    fn Application();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ExtensionImpl: Sized + IDispatchImpl {
    fn Name();
    fn Vendor();
    fn Version();
    fn Extensions();
    fn SnapinCLSID();
    fn EnableAllExtensions();
    fn Enable();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ExtensionsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
pub trait FrameImpl: Sized + IDispatchImpl {
    fn Maximize();
    fn Minimize();
    fn Restore();
    fn Top();
    fn SetTop();
    fn Bottom();
    fn SetBottom();
    fn Left();
    fn SetLeft();
    fn Right();
    fn SetRight();
}
pub trait IColumnDataImpl: Sized {
    fn SetColumnConfigData();
    fn GetColumnConfigData();
    fn SetColumnSortData();
    fn GetColumnSortData();
}
pub trait IComponentImpl: Sized {
    fn Initialize();
    fn Notify();
    fn Destroy();
    fn QueryDataObject();
    fn GetResultViewType();
    fn GetDisplayInfo();
    fn CompareObjects();
}
pub trait IComponent2Impl: Sized + IComponentImpl {
    fn QueryDispatch();
    fn GetResultViewType2();
    fn RestoreResultView();
}
pub trait IComponentDataImpl: Sized {
    fn Initialize();
    fn CreateComponent();
    fn Notify();
    fn Destroy();
    fn QueryDataObject();
    fn GetDisplayInfo();
    fn CompareObjects();
}
pub trait IComponentData2Impl: Sized + IComponentDataImpl {
    fn QueryDispatch();
}
pub trait IConsoleImpl: Sized {
    fn SetHeader();
    fn SetToolbar();
    fn QueryResultView();
    fn QueryScopeImageList();
    fn QueryResultImageList();
    fn UpdateAllViews();
    fn MessageBox();
    fn QueryConsoleVerb();
    fn SelectScopeItem();
    fn GetMainWindow();
    fn NewWindow();
}
pub trait IConsole2Impl: Sized + IConsoleImpl {
    fn Expand();
    fn IsTaskpadViewPreferred();
    fn SetStatusText();
}
pub trait IConsole3Impl: Sized + IConsole2Impl + IConsoleImpl {
    fn RenameScopeItem();
}
pub trait IConsoleNameSpaceImpl: Sized {
    fn InsertItem();
    fn DeleteItem();
    fn SetItem();
    fn GetItem();
    fn GetChildItem();
    fn GetNextItem();
    fn GetParentItem();
}
pub trait IConsoleNameSpace2Impl: Sized + IConsoleNameSpaceImpl {
    fn Expand();
    fn AddExtension();
}
pub trait IConsolePowerImpl: Sized {
    fn SetExecutionState();
    fn ResetIdleTimer();
}
pub trait IConsolePowerSinkImpl: Sized {
    fn OnPowerBroadcast();
}
pub trait IConsoleVerbImpl: Sized {
    fn GetVerbState();
    fn SetVerbState();
    fn SetDefaultVerb();
    fn GetDefaultVerb();
}
pub trait IContextMenuCallbackImpl: Sized {
    fn AddItem();
}
pub trait IContextMenuCallback2Impl: Sized {
    fn AddItem();
}
pub trait IContextMenuProviderImpl: Sized + IContextMenuCallbackImpl {
    fn EmptyMenuList();
    fn AddPrimaryExtensionItems();
    fn AddThirdPartyExtensionItems();
    fn ShowContextMenu();
}
pub trait IControlbarImpl: Sized {
    fn Create();
    fn Attach();
    fn Detach();
}
pub trait IDisplayHelpImpl: Sized {
    fn ShowTopic();
}
pub trait IEnumTASKImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IExtendContextMenuImpl: Sized {
    fn AddMenuItems();
    fn Command();
}
pub trait IExtendControlbarImpl: Sized {
    fn SetControlbar();
    fn ControlbarNotify();
}
pub trait IExtendPropertySheetImpl: Sized {
    fn CreatePropertyPages();
    fn QueryPagesFor();
}
pub trait IExtendPropertySheet2Impl: Sized + IExtendPropertySheetImpl {
    fn GetWatermarks();
}
pub trait IExtendTaskPadImpl: Sized {
    fn TaskNotify();
    fn EnumTasks();
    fn GetTitle();
    fn GetDescriptiveText();
    fn GetBackground();
    fn GetListPadInfo();
}
pub trait IExtendViewImpl: Sized {
    fn GetViews();
}
pub trait IHeaderCtrlImpl: Sized {
    fn InsertColumn();
    fn DeleteColumn();
    fn SetColumnText();
    fn GetColumnText();
    fn SetColumnWidth();
    fn GetColumnWidth();
}
pub trait IHeaderCtrl2Impl: Sized + IHeaderCtrlImpl {
    fn SetChangeTimeOut();
    fn SetColumnFilter();
    fn GetColumnFilter();
}
pub trait IImageListImpl: Sized {
    fn ImageListSetIcon();
    fn ImageListSetStrip();
}
pub trait IMMCVersionInfoImpl: Sized {
    fn GetMMCVersion();
}
pub trait IMenuButtonImpl: Sized {
    fn AddButton();
    fn SetButton();
    fn SetButtonState();
}
pub trait IMessageViewImpl: Sized {
    fn SetTitleText();
    fn SetBodyText();
    fn SetIcon();
    fn Clear();
}
pub trait INodePropertiesImpl: Sized {
    fn GetProperty();
}
pub trait IPropertySheetCallbackImpl: Sized {
    fn AddPage();
    fn RemovePage();
}
pub trait IPropertySheetProviderImpl: Sized {
    fn CreatePropertySheet();
    fn FindPropertySheet();
    fn AddPrimaryPages();
    fn AddExtensionPages();
    fn Show();
}
pub trait IRequiredExtensionsImpl: Sized {
    fn EnableAllExtensions();
    fn GetFirstExtension();
    fn GetNextExtension();
}
pub trait IResultDataImpl: Sized {
    fn InsertItem();
    fn DeleteItem();
    fn FindItemByLParam();
    fn DeleteAllRsltItems();
    fn SetItem();
    fn GetItem();
    fn GetNextItem();
    fn ModifyItemState();
    fn ModifyViewStyle();
    fn SetViewMode();
    fn GetViewMode();
    fn UpdateItem();
    fn Sort();
    fn SetDescBarText();
    fn SetItemCount();
}
pub trait IResultData2Impl: Sized + IResultDataImpl {
    fn RenameResultItem();
}
pub trait IResultDataCompareImpl: Sized {
    fn Compare();
}
pub trait IResultDataCompareExImpl: Sized {
    fn Compare();
}
pub trait IResultOwnerDataImpl: Sized {
    fn FindItem();
    fn CacheHint();
    fn SortItems();
}
pub trait ISnapinAboutImpl: Sized {
    fn GetSnapinDescription();
    fn GetProvider();
    fn GetSnapinVersion();
    fn GetSnapinImage();
    fn GetStaticFolderImage();
}
pub trait ISnapinHelpImpl: Sized {
    fn GetHelpTopic();
}
pub trait ISnapinHelp2Impl: Sized + ISnapinHelpImpl {
    fn GetLinkedTopics();
}
pub trait ISnapinPropertiesImpl: Sized {
    fn Initialize();
    fn QueryPropertyNames();
    fn PropertiesChanged();
}
pub trait ISnapinPropertiesCallbackImpl: Sized {
    fn AddPropertyName();
}
pub trait IStringTableImpl: Sized {
    fn AddString();
    fn GetString();
    fn GetStringLength();
    fn DeleteString();
    fn DeleteAllStrings();
    fn FindString();
    fn Enumerate();
}
pub trait IToolbarImpl: Sized {
    fn AddBitmap();
    fn AddButtons();
    fn InsertButton();
    fn DeleteButton();
    fn GetButtonState();
    fn SetButtonState();
}
pub trait IViewExtensionCallbackImpl: Sized {
    fn AddView();
}
#[cfg(feature = "Win32_System_Com")]
pub trait MenuItemImpl: Sized + IDispatchImpl {
    fn DisplayName();
    fn LanguageIndependentName();
    fn Path();
    fn LanguageIndependentPath();
    fn Execute();
    fn Enabled();
}
#[cfg(feature = "Win32_System_Com")]
pub trait NodeImpl: Sized + IDispatchImpl {
    fn Name();
    fn Property();
    fn Bookmark();
    fn IsScopeNode();
    fn Nodetype();
}
#[cfg(feature = "Win32_System_Com")]
pub trait NodesImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
pub trait PropertiesImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Remove();
}
#[cfg(feature = "Win32_System_Com")]
pub trait PropertyImpl: Sized + IDispatchImpl {
    fn Value();
    fn SetValue();
    fn Name();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ScopeNamespaceImpl: Sized + IDispatchImpl {
    fn GetParent();
    fn GetChild();
    fn GetNext();
    fn GetRoot();
    fn Expand();
}
#[cfg(feature = "Win32_System_Com")]
pub trait SnapInImpl: Sized + IDispatchImpl {
    fn Name();
    fn Vendor();
    fn Version();
    fn Extensions();
    fn SnapinCLSID();
    fn Properties();
    fn EnableAllExtensions();
}
#[cfg(feature = "Win32_System_Com")]
pub trait SnapInsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Add();
    fn Remove();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ViewImpl: Sized + IDispatchImpl {
    fn ActiveScopeNode();
    fn SetActiveScopeNode();
    fn Selection();
    fn ListItems();
    fn SnapinScopeObject();
    fn SnapinSelectionObject();
    fn Is();
    fn Document();
    fn SelectAll();
    fn Select();
    fn Deselect();
    fn IsSelected();
    fn DisplayScopeNodePropertySheet();
    fn DisplaySelectionPropertySheet();
    fn CopyScopeNode();
    fn CopySelection();
    fn DeleteScopeNode();
    fn DeleteSelection();
    fn RenameScopeNode();
    fn RenameSelectedItem();
    fn ScopeNodeContextMenu();
    fn SelectionContextMenu();
    fn RefreshScopeNode();
    fn RefreshSelection();
    fn ExecuteSelectionMenuItem();
    fn ExecuteScopeNodeMenuItem();
    fn ExecuteShellCommand();
    fn Frame();
    fn Close();
    fn ScopeTreeVisible();
    fn SetScopeTreeVisible();
    fn Back();
    fn Forward();
    fn SetStatusBarText();
    fn Memento();
    fn ViewMemento();
    fn Columns();
    fn CellContents();
    fn ExportList();
    fn ListViewMode();
    fn SetListViewMode();
    fn ControlObject();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ViewsImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn Add();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait _AppEventsImpl: Sized + IDispatchImpl {
    fn OnQuit();
    fn OnDocumentOpen();
    fn OnDocumentClose();
    fn OnSnapInAdded();
    fn OnSnapInRemoved();
    fn OnNewView();
    fn OnViewClose();
    fn OnViewChange();
    fn OnSelectionChange();
    fn OnContextMenuExecuted();
    fn OnToolbarButtonClicked();
    fn OnListUpdated();
}
#[cfg(feature = "Win32_System_Com")]
pub trait _ApplicationImpl: Sized + IDispatchImpl {
    fn Help();
    fn Quit();
    fn Document();
    fn Load();
    fn Frame();
    fn Visible();
    fn Show();
    fn Hide();
    fn UserControl();
    fn SetUserControl();
    fn VersionMajor();
    fn VersionMinor();
}
#[cfg(feature = "Win32_System_Com")]
pub trait _EventConnectorImpl: Sized + IDispatchImpl {
    fn ConnectTo();
    fn Disconnect();
}
