pub trait DataSourceImpl: Sized {
    fn getDataMember();
    fn getDataMemberName();
    fn getDataMemberCount();
    fn addDataSourceListener();
    fn removeDataSourceListener();
}
pub trait DataSourceListenerImpl: Sized {
    fn dataMemberChanged();
    fn dataMemberAdded();
    fn dataMemberRemoved();
}
#[cfg(feature = "Win32_System_Com")]
pub trait DataSourceObjectImpl: Sized + IDispatchImpl {}
pub trait IAccessorImpl: Sized {
    fn AddRefAccessor();
    fn CreateAccessor();
    fn GetBindings();
    fn ReleaseAccessor();
}
pub trait IAlterIndexImpl: Sized {
    fn AlterIndex();
}
pub trait IAlterTableImpl: Sized {
    fn AlterColumn();
    fn AlterTable();
}
pub trait IBindResourceImpl: Sized {
    fn Bind();
}
pub trait IChapteredRowsetImpl: Sized {
    fn AddRefChapter();
    fn ReleaseChapter();
}
pub trait IColumnMapperImpl: Sized {
    fn GetPropInfoFromName();
    fn GetPropInfoFromId();
    fn EnumPropInfo();
    fn IsMapUpToDate();
}
pub trait IColumnMapperCreatorImpl: Sized {
    fn GetColumnMapper();
}
pub trait IColumnsInfoImpl: Sized {
    fn GetColumnInfo();
    fn MapColumnIDs();
}
pub trait IColumnsInfo2Impl: Sized + IColumnsInfoImpl {
    fn GetRestrictedColumnInfo();
}
pub trait IColumnsRowsetImpl: Sized {
    fn GetAvailableColumns();
    fn GetColumnsRowset();
}
pub trait ICommandImpl: Sized {
    fn Cancel();
    fn Execute();
    fn GetDBSession();
}
pub trait ICommandCostImpl: Sized {
    fn GetAccumulatedCost();
    fn GetCostEstimate();
    fn GetCostGoals();
    fn GetCostLimits();
    fn SetCostGoals();
    fn SetCostLimits();
}
pub trait ICommandPersistImpl: Sized {
    fn DeleteCommand();
    fn GetCurrentCommand();
    fn LoadCommand();
    fn SaveCommand();
}
pub trait ICommandPrepareImpl: Sized {
    fn Prepare();
    fn Unprepare();
}
pub trait ICommandPropertiesImpl: Sized {
    fn GetProperties();
    fn SetProperties();
}
pub trait ICommandStreamImpl: Sized {
    fn GetCommandStream();
    fn SetCommandStream();
}
pub trait ICommandTextImpl: Sized + ICommandImpl {
    fn GetCommandText();
    fn SetCommandText();
}
pub trait ICommandValidateImpl: Sized {
    fn ValidateCompletely();
    fn ValidateSyntax();
}
pub trait ICommandWithParametersImpl: Sized {
    fn GetParameterInfo();
    fn MapParameterNames();
    fn SetParameterInfo();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IConditionImpl: Sized + IPersistStreamImpl + IPersistImpl {
    fn GetConditionType();
    fn GetSubConditions();
    fn GetComparisonInfo();
    fn GetValueType();
    fn GetValueNormalization();
    fn GetInputTerms();
    fn Clone();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICondition2Impl: Sized + IConditionImpl + IPersistStreamImpl + IPersistImpl {
    fn GetLocale();
    fn GetLeafConditionInfo();
}
pub trait IConditionFactoryImpl: Sized {
    fn MakeNot();
    fn MakeAndOr();
    fn MakeLeaf();
    fn Resolve();
}
pub trait IConditionFactory2Impl: Sized + IConditionFactoryImpl {
    fn CreateTrueFalse();
    fn CreateNegation();
    fn CreateCompoundFromObjectArray();
    fn CreateCompoundFromArray();
    fn CreateStringLeaf();
    fn CreateIntegerLeaf();
    fn CreateBooleanLeaf();
    fn CreateLeaf();
    fn ResolveCondition();
}
pub trait IConditionGeneratorImpl: Sized {
    fn Initialize();
    fn RecognizeNamedEntities();
    fn GenerateForLeaf();
    fn DefaultPhrase();
}
pub trait IConvertTypeImpl: Sized {
    fn CanConvert();
}
pub trait ICreateRowImpl: Sized {
    fn CreateRow();
}
pub trait IDBAsynchNotifyImpl: Sized {
    fn OnLowResource();
    fn OnProgress();
    fn OnStop();
}
pub trait IDBAsynchStatusImpl: Sized {
    fn Abort();
    fn GetStatus();
}
pub trait IDBBinderPropertiesImpl: Sized + IDBPropertiesImpl {
    fn Reset();
}
pub trait IDBCreateCommandImpl: Sized {
    fn CreateCommand();
}
pub trait IDBCreateSessionImpl: Sized {
    fn CreateSession();
}
pub trait IDBDataSourceAdminImpl: Sized {
    fn CreateDataSource();
    fn DestroyDataSource();
    fn GetCreationProperties();
    fn ModifyDataSource();
}
pub trait IDBInfoImpl: Sized {
    fn GetKeywords();
    fn GetLiteralInfo();
}
pub trait IDBInitializeImpl: Sized {
    fn Initialize();
    fn Uninitialize();
}
pub trait IDBPromptInitializeImpl: Sized {
    fn PromptDataSource();
    fn PromptFileName();
}
pub trait IDBPropertiesImpl: Sized {
    fn GetProperties();
    fn GetPropertyInfo();
    fn SetProperties();
}
pub trait IDBSchemaCommandImpl: Sized {
    fn GetCommand();
    fn GetSchemas();
}
pub trait IDBSchemaRowsetImpl: Sized {
    fn GetRowset();
    fn GetSchemas();
}
pub trait IDCInfoImpl: Sized {
    fn GetInfo();
    fn SetInfo();
}
pub trait IDataConvertImpl: Sized {
    fn DataConvert();
    fn CanConvert();
    fn GetConversionSize();
}
pub trait IDataInitializeImpl: Sized {
    fn GetDataSource();
    fn GetInitializationString();
    fn CreateDBInstance();
    fn CreateDBInstanceEx();
    fn LoadStringFromStorage();
    fn WriteStringToStorage();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDataSourceLocatorImpl: Sized + IDispatchImpl {
    fn hWnd();
    fn SethWnd();
    fn PromptNew();
    fn PromptEdit();
}
pub trait IEntityImpl: Sized {
    fn Name();
    fn Base();
    fn Relationships();
    fn GetRelationship();
    fn MetaData();
    fn NamedEntities();
    fn GetNamedEntity();
    fn DefaultPhrase();
}
pub trait IEnumItemPropertiesImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
pub trait IEnumSearchRootsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumSearchScopeRulesImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumSubscriptionImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
pub trait IErrorLookupImpl: Sized {
    fn GetErrorDescription();
    fn GetHelpInfo();
    fn ReleaseErrors();
}
pub trait IErrorRecordsImpl: Sized {
    fn AddErrorRecord();
    fn GetBasicErrorInfo();
    fn GetCustomErrorObject();
    fn GetErrorInfo();
    fn GetErrorParameters();
    fn GetRecordCount();
}
pub trait IGetDataSourceImpl: Sized {
    fn GetDataSource();
}
pub trait IGetRowImpl: Sized {
    fn GetRowFromHROW();
    fn GetURLFromHROW();
}
pub trait IGetSessionImpl: Sized {
    fn GetSession();
}
pub trait IGetSourceRowImpl: Sized {
    fn GetSourceRow();
}
pub trait IIndexDefinitionImpl: Sized {
    fn CreateIndex();
    fn DropIndex();
}
pub trait IIntervalImpl: Sized {
    fn GetLimits();
}
pub trait ILoadFilterImpl: Sized {
    fn LoadIFilter();
    fn LoadIFilterFromStorage();
    fn LoadIFilterFromStream();
}
pub trait ILoadFilterWithPrivateComActivationImpl: Sized + ILoadFilterImpl {
    fn LoadIFilterWithPrivateComActivation();
}
pub trait IMDDatasetImpl: Sized {
    fn FreeAxisInfo();
    fn GetAxisInfo();
    fn GetAxisRowset();
    fn GetCellData();
    fn GetSpecification();
}
pub trait IMDFindImpl: Sized {
    fn FindCell();
    fn FindTuple();
}
pub trait IMDRangeRowsetImpl: Sized {
    fn GetRangeRowset();
}
pub trait IMetaDataImpl: Sized {
    fn GetData();
}
pub trait IMultipleResultsImpl: Sized {
    fn GetResult();
}
pub trait INamedEntityImpl: Sized {
    fn GetValue();
    fn DefaultPhrase();
}
pub trait INamedEntityCollectorImpl: Sized {
    fn Add();
}
pub trait IObjectAccessControlImpl: Sized {
    fn GetObjectAccessRights();
    fn GetObjectOwner();
    fn IsObjectAccessAllowed();
    fn SetObjectAccessRights();
    fn SetObjectOwner();
}
pub trait IOpLockStatusImpl: Sized {
    fn IsOplockValid();
    fn IsOplockBroken();
    fn GetOplockEventHandle();
}
pub trait IOpenRowsetImpl: Sized {
    fn OpenRowset();
}
pub trait IParentRowsetImpl: Sized {
    fn GetChildRowset();
}
pub trait IProtocolHandlerSiteImpl: Sized {
    fn GetFilter();
}
pub trait IProvideMonikerImpl: Sized {
    fn GetMoniker();
}
pub trait IQueryParserImpl: Sized {
    fn Parse();
    fn SetOption();
    fn GetOption();
    fn SetMultiOption();
    fn GetSchemaProvider();
    fn RestateToString();
    fn ParsePropertyValue();
    fn RestatePropertyValueToString();
}
pub trait IQueryParserManagerImpl: Sized {
    fn CreateLoadedParser();
    fn InitializeOptions();
    fn SetOption();
}
pub trait IQuerySolutionImpl: Sized + IConditionFactoryImpl {
    fn GetQuery();
    fn GetErrors();
    fn GetLexicalData();
}
pub trait IReadDataImpl: Sized {
    fn ReadData();
    fn ReleaseChapter();
}
pub trait IRegisterProviderImpl: Sized {
    fn GetURLMapping();
    fn SetURLMapping();
    fn UnregisterProvider();
}
pub trait IRelationshipImpl: Sized {
    fn Name();
    fn IsReal();
    fn Destination();
    fn MetaData();
    fn DefaultPhrase();
}
pub trait IRichChunkImpl: Sized {
    fn GetData();
}
pub trait IRowImpl: Sized {
    fn GetColumns();
    fn GetSourceRowset();
    fn Open();
}
pub trait IRowChangeImpl: Sized {
    fn SetColumns();
}
pub trait IRowPositionImpl: Sized {
    fn ClearRowPosition();
    fn GetRowPosition();
    fn GetRowset();
    fn Initialize();
    fn SetRowPosition();
}
pub trait IRowPositionChangeImpl: Sized {
    fn OnRowPositionChange();
}
pub trait IRowSchemaChangeImpl: Sized + IRowChangeImpl {
    fn DeleteColumns();
    fn AddColumns();
}
pub trait IRowsetImpl: Sized {
    fn AddRefRows();
    fn GetData();
    fn GetNextRows();
    fn ReleaseRows();
    fn RestartPosition();
}
pub trait IRowsetAsynchImpl: Sized {
    fn RatioFinished();
    fn Stop();
}
pub trait IRowsetBookmarkImpl: Sized {
    fn PositionOnBookmark();
}
pub trait IRowsetChangeImpl: Sized {
    fn DeleteRows();
    fn SetData();
    fn InsertRow();
}
pub trait IRowsetChangeExtInfoImpl: Sized {
    fn GetOriginalRow();
    fn GetPendingColumns();
}
pub trait IRowsetChapterMemberImpl: Sized {
    fn IsRowInChapter();
}
pub trait IRowsetCopyRowsImpl: Sized {
    fn CloseSource();
    fn CopyByHROWS();
    fn CopyRows();
    fn DefineSource();
}
pub trait IRowsetCurrentIndexImpl: Sized + IRowsetIndexImpl {
    fn GetIndex();
    fn SetIndex();
}
pub trait IRowsetEventsImpl: Sized {
    fn OnNewItem();
    fn OnChangedItem();
    fn OnDeletedItem();
    fn OnRowsetEvent();
}
pub trait IRowsetFastLoadImpl: Sized {
    fn InsertRow();
    fn Commit();
}
pub trait IRowsetFindImpl: Sized {
    fn FindNextRow();
}
pub trait IRowsetIdentityImpl: Sized {
    fn IsSameRow();
}
pub trait IRowsetIndexImpl: Sized {
    fn GetIndexInfo();
    fn Seek();
    fn SetRange();
}
pub trait IRowsetInfoImpl: Sized {
    fn GetProperties();
    fn GetReferencedRowset();
    fn GetSpecification();
}
pub trait IRowsetKeysImpl: Sized {
    fn ListKeys();
}
pub trait IRowsetLocateImpl: Sized + IRowsetImpl {
    fn Compare();
    fn GetRowsAt();
    fn GetRowsByBookmark();
    fn Hash();
}
pub trait IRowsetNewRowAfterImpl: Sized {
    fn SetNewDataAfter();
}
pub trait IRowsetNextRowsetImpl: Sized {
    fn GetNextRowset();
}
pub trait IRowsetNotifyImpl: Sized {
    fn OnFieldChange();
    fn OnRowChange();
    fn OnRowsetChange();
}
pub trait IRowsetPrioritizationImpl: Sized {
    fn SetScopePriority();
    fn GetScopePriority();
    fn GetScopeStatistics();
}
pub trait IRowsetQueryStatusImpl: Sized {
    fn GetStatus();
    fn GetStatusEx();
}
pub trait IRowsetRefreshImpl: Sized {
    fn RefreshVisibleData();
    fn GetLastVisibleData();
}
pub trait IRowsetResynchImpl: Sized {
    fn GetVisibleData();
    fn ResynchRows();
}
pub trait IRowsetScrollImpl: Sized + IRowsetLocateImpl + IRowsetImpl {
    fn GetApproximatePosition();
    fn GetRowsAtRatio();
}
pub trait IRowsetUpdateImpl: Sized + IRowsetChangeImpl {
    fn GetOriginalData();
    fn GetPendingRows();
    fn GetRowStatus();
    fn Undo();
    fn Update();
}
pub trait IRowsetViewImpl: Sized {
    fn CreateView();
    fn GetView();
}
pub trait IRowsetWatchAllImpl: Sized {
    fn Acknowledge();
    fn Start();
    fn StopWatching();
}
pub trait IRowsetWatchNotifyImpl: Sized {
    fn OnChange();
}
pub trait IRowsetWatchRegionImpl: Sized + IRowsetWatchAllImpl {
    fn CreateWatchRegion();
    fn ChangeWatchMode();
    fn DeleteWatchRegion();
    fn GetWatchRegionInfo();
    fn Refresh();
    fn ShrinkWatchRegion();
}
pub trait IRowsetWithParametersImpl: Sized {
    fn GetParameterInfo();
    fn Requery();
}
pub trait ISQLErrorInfoImpl: Sized {
    fn GetSQLInfo();
}
pub trait ISQLGetDiagFieldImpl: Sized {
    fn GetDiagField();
}
pub trait ISQLRequestDiagFieldsImpl: Sized {
    fn RequestDiagFields();
}
pub trait ISQLServerErrorInfoImpl: Sized {
    fn GetErrorInfo();
}
pub trait ISchemaLocalizerSupportImpl: Sized {
    fn Localize();
}
pub trait ISchemaLockImpl: Sized {
    fn GetSchemaLock();
    fn ReleaseSchemaLock();
}
pub trait ISchemaProviderImpl: Sized {
    fn Entities();
    fn RootEntity();
    fn GetEntity();
    fn MetaData();
    fn Localize();
    fn SaveBinary();
    fn LookupAuthoredNamedEntity();
}
pub trait IScopedOperationsImpl: Sized + IBindResourceImpl {
    fn Copy();
    fn Move();
    fn Delete();
    fn OpenRowset();
}
pub trait ISearchCatalogManagerImpl: Sized {
    fn Name();
    fn GetParameter();
    fn SetParameter();
    fn GetCatalogStatus();
    fn Reset();
    fn Reindex();
    fn ReindexMatchingURLs();
    fn ReindexSearchRoot();
    fn SetConnectTimeout();
    fn ConnectTimeout();
    fn SetDataTimeout();
    fn DataTimeout();
    fn NumberOfItems();
    fn NumberOfItemsToIndex();
    fn URLBeingIndexed();
    fn GetURLIndexingState();
    fn GetPersistentItemsChangedSink();
    fn RegisterViewForNotification();
    fn GetItemsChangedSink();
    fn UnregisterViewForNotification();
    fn SetExtensionClusion();
    fn EnumerateExcludedExtensions();
    fn GetQueryHelper();
    fn SetDiacriticSensitivity();
    fn DiacriticSensitivity();
    fn GetCrawlScopeManager();
}
pub trait ISearchCatalogManager2Impl: Sized + ISearchCatalogManagerImpl {
    fn PrioritizeMatchingURLs();
}
pub trait ISearchCrawlScopeManagerImpl: Sized {
    fn AddDefaultScopeRule();
    fn AddRoot();
    fn RemoveRoot();
    fn EnumerateRoots();
    fn AddHierarchicalScope();
    fn AddUserScopeRule();
    fn RemoveScopeRule();
    fn EnumerateScopeRules();
    fn HasParentScopeRule();
    fn HasChildScopeRule();
    fn IncludedInCrawlScope();
    fn IncludedInCrawlScopeEx();
    fn RevertToDefaultScopes();
    fn SaveAll();
    fn GetParentScopeVersionId();
    fn RemoveDefaultScopeRule();
}
pub trait ISearchCrawlScopeManager2Impl: Sized + ISearchCrawlScopeManagerImpl {
    fn GetVersion();
}
pub trait ISearchItemsChangedSinkImpl: Sized {
    fn StartedMonitoringScope();
    fn StoppedMonitoringScope();
    fn OnItemsChanged();
}
pub trait ISearchLanguageSupportImpl: Sized {
    fn SetDiacriticSensitivity();
    fn GetDiacriticSensitivity();
    fn LoadWordBreaker();
    fn LoadStemmer();
    fn IsPrefixNormalized();
}
pub trait ISearchManagerImpl: Sized {
    fn GetIndexerVersionStr();
    fn GetIndexerVersion();
    fn GetParameter();
    fn SetParameter();
    fn ProxyName();
    fn BypassList();
    fn SetProxy();
    fn GetCatalog();
    fn UserAgent();
    fn SetUserAgent();
    fn UseProxy();
    fn LocalBypass();
    fn PortNumber();
}
pub trait ISearchManager2Impl: Sized + ISearchManagerImpl {
    fn CreateCatalog();
    fn DeleteCatalog();
}
pub trait ISearchNotifyInlineSiteImpl: Sized {
    fn OnItemIndexedStatusChange();
    fn OnCatalogStatusChange();
}
pub trait ISearchPersistentItemsChangedSinkImpl: Sized {
    fn StartedMonitoringScope();
    fn StoppedMonitoringScope();
    fn OnItemsChanged();
}
pub trait ISearchProtocolImpl: Sized {
    fn Init();
    fn CreateAccessor();
    fn CloseAccessor();
    fn ShutDown();
}
pub trait ISearchProtocol2Impl: Sized + ISearchProtocolImpl {
    fn CreateAccessorEx();
}
pub trait ISearchProtocolThreadContextImpl: Sized {
    fn ThreadInit();
    fn ThreadShutdown();
    fn ThreadIdle();
}
pub trait ISearchQueryHelperImpl: Sized {
    fn ConnectionString();
    fn SetQueryContentLocale();
    fn QueryContentLocale();
    fn SetQueryKeywordLocale();
    fn QueryKeywordLocale();
    fn SetQueryTermExpansion();
    fn QueryTermExpansion();
    fn SetQuerySyntax();
    fn QuerySyntax();
    fn SetQueryContentProperties();
    fn QueryContentProperties();
    fn SetQuerySelectColumns();
    fn QuerySelectColumns();
    fn SetQueryWhereRestrictions();
    fn QueryWhereRestrictions();
    fn SetQuerySorting();
    fn QuerySorting();
    fn GenerateSQLFromUserQuery();
    fn WriteProperties();
    fn SetQueryMaxResults();
    fn QueryMaxResults();
}
pub trait ISearchQueryHitsImpl: Sized {
    fn Init();
    fn NextHitMoniker();
    fn NextHitOffset();
}
pub trait ISearchRootImpl: Sized {
    fn SetSchedule();
    fn Schedule();
    fn SetRootURL();
    fn RootURL();
    fn SetIsHierarchical();
    fn IsHierarchical();
    fn SetProvidesNotifications();
    fn ProvidesNotifications();
    fn SetUseNotificationsOnly();
    fn UseNotificationsOnly();
    fn SetEnumerationDepth();
    fn EnumerationDepth();
    fn SetHostDepth();
    fn HostDepth();
    fn SetFollowDirectories();
    fn FollowDirectories();
    fn SetAuthenticationType();
    fn AuthenticationType();
    fn SetUser();
    fn User();
    fn SetPassword();
    fn Password();
}
pub trait ISearchScopeRuleImpl: Sized {
    fn PatternOrURL();
    fn IsIncluded();
    fn IsDefault();
    fn FollowFlags();
}
pub trait ISearchViewChangedSinkImpl: Sized {
    fn OnChange();
}
pub trait ISecurityInfoImpl: Sized {
    fn GetCurrentTrustee();
    fn GetObjectTypes();
    fn GetPermissions();
}
pub trait IServiceImpl: Sized {
    fn InvokeService();
}
pub trait ISessionPropertiesImpl: Sized {
    fn GetProperties();
    fn SetProperties();
}
pub trait ISimpleCommandCreatorImpl: Sized {
    fn CreateICommand();
    fn VerifyCatalog();
    fn GetDefaultCatalog();
}
pub trait ISourcesRowsetImpl: Sized {
    fn GetSourcesRowset();
}
pub trait IStemmerImpl: Sized {
    fn Init();
    fn GenerateWordForms();
    fn GetLicenseToUse();
}
pub trait ISubscriptionItemImpl: Sized {
    fn GetCookie();
    fn GetSubscriptionItemInfo();
    fn SetSubscriptionItemInfo();
    fn ReadProperties();
    fn WriteProperties();
    fn EnumProperties();
    fn NotifyChanged();
}
pub trait ISubscriptionMgrImpl: Sized {
    fn DeleteSubscription();
    fn UpdateSubscription();
    fn UpdateAll();
    fn IsSubscribed();
    fn GetSubscriptionInfo();
    fn GetDefaultInfo();
    fn ShowSubscriptionProperties();
    fn CreateSubscription();
}
pub trait ISubscriptionMgr2Impl: Sized + ISubscriptionMgrImpl {
    fn GetItemFromURL();
    fn GetItemFromCookie();
    fn GetSubscriptionRunState();
    fn EnumSubscriptions();
    fn UpdateItems();
    fn AbortItems();
    fn AbortAll();
}
pub trait ITableCreationImpl: Sized + ITableDefinitionImpl {
    fn GetTableDefinition();
}
pub trait ITableDefinitionImpl: Sized {
    fn CreateTable();
    fn DropTable();
    fn AddColumn();
    fn DropColumn();
}
pub trait ITableDefinitionWithConstraintsImpl: Sized + ITableCreationImpl + ITableDefinitionImpl {
    fn AddConstraint();
    fn CreateTableWithConstraints();
    fn DropConstraint();
}
pub trait ITableRenameImpl: Sized {
    fn RenameColumn();
    fn RenameTable();
}
pub trait ITokenCollectionImpl: Sized {
    fn NumberOfTokens();
    fn GetToken();
}
pub trait ITransactionJoinImpl: Sized {
    fn GetOptionsObject();
    fn JoinTransaction();
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait ITransactionLocalImpl: Sized + ITransactionImpl {
    fn GetOptionsObject();
    fn StartTransaction();
}
pub trait ITransactionObjectImpl: Sized {
    fn GetTransactionObject();
}
pub trait ITrusteeAdminImpl: Sized {
    fn CompareTrustees();
    fn CreateTrustee();
    fn DeleteTrustee();
    fn SetTrusteeProperties();
    fn GetTrusteeProperties();
}
pub trait ITrusteeGroupAdminImpl: Sized {
    fn AddMember();
    fn DeleteMember();
    fn IsMember();
    fn GetMembers();
    fn GetMemberships();
}
pub trait IUMSImpl: Sized {
    fn SqlUmsSuspend();
    fn SqlUmsYield();
    fn SqlUmsSwitchPremptive();
    fn SqlUmsSwitchNonPremptive();
    fn SqlUmsFIsPremptive();
}
pub trait IUMSInitializeImpl: Sized {
    fn Initialize();
}
pub trait IUrlAccessorImpl: Sized {
    fn AddRequestParameter();
    fn GetDocFormat();
    fn GetCLSID();
    fn GetHost();
    fn IsDirectory();
    fn GetSize();
    fn GetLastModified();
    fn GetFileName();
    fn GetSecurityDescriptor();
    fn GetRedirectedURL();
    fn GetSecurityProvider();
    fn BindToStream();
    fn BindToFilter();
}
pub trait IUrlAccessor2Impl: Sized + IUrlAccessorImpl {
    fn GetDisplayUrl();
    fn IsDocument();
    fn GetCodePage();
}
pub trait IUrlAccessor3Impl: Sized + IUrlAccessor2Impl + IUrlAccessorImpl {
    fn GetImpersonationSidBlobs();
}
pub trait IUrlAccessor4Impl: Sized + IUrlAccessor3Impl + IUrlAccessor2Impl + IUrlAccessorImpl {
    fn ShouldIndexItemContent();
    fn ShouldIndexProperty();
}
pub trait IViewChapterImpl: Sized {
    fn GetSpecification();
    fn OpenViewChapter();
}
pub trait IViewFilterImpl: Sized {
    fn GetFilter();
    fn GetFilterBindings();
    fn SetFilter();
}
pub trait IViewRowsetImpl: Sized {
    fn GetSpecification();
    fn OpenViewRowset();
}
pub trait IViewSortImpl: Sized {
    fn GetSortOrder();
    fn SetSortOrder();
}
pub trait IWordBreakerImpl: Sized {
    fn Init();
    fn BreakText();
    fn ComposePhrase();
    fn GetLicenseToUse();
}
pub trait IWordFormSinkImpl: Sized {
    fn PutAltWord();
    fn PutWord();
}
pub trait IWordSinkImpl: Sized {
    fn PutWord();
    fn PutAltWord();
    fn StartAltPhrase();
    fn EndAltPhrase();
    fn PutBreak();
}
pub trait OLEDBSimpleProviderImpl: Sized {
    fn getRowCount();
    fn getColumnCount();
    fn getRWStatus();
    fn getVariant();
    fn setVariant();
    fn getLocale();
    fn deleteRows();
    fn insertRows();
    fn find();
    fn addOLEDBSimpleProviderListener();
    fn removeOLEDBSimpleProviderListener();
    fn isAsync();
    fn getEstimatedRows();
    fn stopTransfer();
}
pub trait OLEDBSimpleProviderListenerImpl: Sized {
    fn aboutToChangeCell();
    fn cellChanged();
    fn aboutToDeleteRows();
    fn deletedRows();
    fn aboutToInsertRows();
    fn insertedRows();
    fn rowsAvailable();
    fn transferComplete();
}
