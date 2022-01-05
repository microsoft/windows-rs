pub trait IAsynchronousDataRetrieverImpl: Sized {
    fn GetIdParameters();
    fn RegisterCallback();
    fn RevokeCallback();
    fn LoadChangeData();
}
pub trait IChangeConflictImpl: Sized {
    fn GetDestinationProviderConflictingChange();
    fn GetSourceProviderConflictingChange();
    fn GetDestinationProviderConflictingData();
    fn GetSourceProviderConflictingData();
    fn GetResolveActionForChange();
    fn SetResolveActionForChange();
    fn GetResolveActionForChangeUnit();
    fn SetResolveActionForChangeUnit();
}
pub trait IChangeUnitExceptionImpl: Sized {
    fn GetItemId();
    fn GetChangeUnitId();
    fn GetClockVector();
}
pub trait IChangeUnitListFilterInfoImpl: Sized + ISyncFilterInfoImpl {
    fn Initialize();
    fn GetChangeUnitIdCount();
    fn GetChangeUnitId();
}
pub trait IClockVectorImpl: Sized {
    fn GetClockVectorElements();
    fn GetClockVectorElementCount();
}
pub trait IClockVectorElementImpl: Sized {
    fn GetReplicaKey();
    fn GetTickCount();
}
pub trait ICombinedFilterInfoImpl: Sized + ISyncFilterInfoImpl {
    fn GetFilterCount();
    fn GetFilterInfo();
    fn GetFilterCombinationType();
}
pub trait IConstraintConflictImpl: Sized {
    fn GetDestinationProviderConflictingChange();
    fn GetSourceProviderConflictingChange();
    fn GetDestinationProviderOriginalChange();
    fn GetDestinationProviderConflictingData();
    fn GetSourceProviderConflictingData();
    fn GetDestinationProviderOriginalData();
    fn GetConstraintResolveActionForChange();
    fn SetConstraintResolveActionForChange();
    fn GetConstraintResolveActionForChangeUnit();
    fn SetConstraintResolveActionForChangeUnit();
    fn GetConstraintConflictReason();
    fn IsTemporary();
}
pub trait IConstructReplicaKeyMapImpl: Sized {
    fn FindOrAddReplica();
}
pub trait ICoreFragmentImpl: Sized {
    fn NextColumn();
    fn NextRange();
    fn Reset();
    fn GetColumnCount();
    fn GetRangeCount();
}
pub trait ICoreFragmentInspectorImpl: Sized {
    fn NextCoreFragments();
    fn Reset();
}
pub trait ICustomFilterInfoImpl: Sized + ISyncFilterInfoImpl {
    fn GetSyncFilter();
}
pub trait IDataRetrieverCallbackImpl: Sized {
    fn LoadChangeDataComplete();
    fn LoadChangeDataError();
}
pub trait IEnumChangeUnitExceptionsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumClockVectorImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumFeedClockVectorImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumItemIdsImpl: Sized {
    fn Next();
}
pub trait IEnumRangeExceptionsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumSingleItemExceptionsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumSyncChangeUnitsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumSyncChangesImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumSyncProviderConfigUIInfosImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumSyncProviderInfosImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IFeedClockVectorImpl: Sized + IClockVectorImpl {
    fn GetUpdateCount();
    fn IsNoConflictsSpecified();
}
pub trait IFeedClockVectorElementImpl: Sized + IClockVectorElementImpl {
    fn GetSyncTime();
    fn GetFlags();
}
pub trait IFilterKeyMapImpl: Sized {
    fn GetCount();
    fn AddFilter();
    fn GetFilter();
    fn Serialize();
}
pub trait IFilterRequestCallbackImpl: Sized {
    fn RequestFilter();
}
pub trait IFilterTrackingProviderImpl: Sized {
    fn SpecifyTrackedFilters();
    fn AddTrackedFilter();
}
pub trait IFilterTrackingRequestCallbackImpl: Sized {
    fn RequestTrackedFilter();
}
pub trait IFilterTrackingSyncChangeBuilderImpl: Sized {
    fn AddFilterChange();
    fn SetAllChangeUnitsPresentFlag();
}
pub trait IForgottenKnowledgeImpl: Sized + ISyncKnowledgeImpl {
    fn ForgetToVersion();
}
pub trait IKnowledgeSyncProviderImpl: Sized + ISyncProviderImpl {
    fn BeginSession();
    fn GetSyncBatchParameters();
    fn GetChangeBatch();
    fn GetFullEnumerationChangeBatch();
    fn ProcessChangeBatch();
    fn ProcessFullEnumerationChangeBatch();
    fn EndSession();
}
pub trait ILoadChangeContextImpl: Sized {
    fn GetSyncChange();
    fn SetRecoverableErrorOnChange();
    fn SetRecoverableErrorOnChangeUnit();
}
pub trait IProviderConverterImpl: Sized {
    fn Initialize();
}
pub trait IRangeExceptionImpl: Sized {
    fn GetClosedRangeStart();
    fn GetClosedRangeEnd();
    fn GetClockVector();
}
pub trait IRecoverableErrorImpl: Sized {
    fn GetStage();
    fn GetProvider();
    fn GetChangeWithRecoverableError();
    fn GetRecoverableErrorDataForChange();
    fn GetRecoverableErrorDataForChangeUnit();
}
pub trait IRecoverableErrorDataImpl: Sized {
    fn Initialize();
    fn GetItemDisplayName();
    fn GetErrorDescription();
}
pub trait IRegisteredSyncProviderImpl: Sized {
    fn Init();
    fn GetInstanceId();
    fn Reset();
}
pub trait IReplicaKeyMapImpl: Sized {
    fn LookupReplicaKey();
    fn LookupReplicaId();
    fn Serialize();
}
pub trait IRequestFilteredSyncImpl: Sized {
    fn SpecifyFilter();
}
pub trait ISingleItemExceptionImpl: Sized {
    fn GetItemId();
    fn GetClockVector();
}
pub trait ISupportFilteredSyncImpl: Sized {
    fn AddFilter();
}
pub trait ISupportLastWriteTimeImpl: Sized {
    fn GetItemChangeTime();
    fn GetChangeUnitChangeTime();
}
pub trait ISyncCallbackImpl: Sized {
    fn OnProgress();
    fn OnChange();
    fn OnConflict();
    fn OnFullEnumerationNeeded();
    fn OnRecoverableError();
}
pub trait ISyncCallback2Impl: Sized + ISyncCallbackImpl {
    fn OnChangeApplied();
    fn OnChangeFailed();
}
pub trait ISyncChangeImpl: Sized {
    fn GetOwnerReplicaId();
    fn GetRootItemId();
    fn GetChangeVersion();
    fn GetCreationVersion();
    fn GetFlags();
    fn GetWorkEstimate();
    fn GetChangeUnits();
    fn GetMadeWithKnowledge();
    fn GetLearnedKnowledge();
    fn SetWorkEstimate();
}
pub trait ISyncChangeBatchImpl: Sized + ISyncChangeBatchBaseImpl {
    fn BeginUnorderedGroup();
    fn EndUnorderedGroup();
    fn AddLoggedConflict();
}
pub trait ISyncChangeBatch2Impl: Sized + ISyncChangeBatchImpl + ISyncChangeBatchBaseImpl {
    fn AddMergeTombstoneMetadataToGroup();
    fn AddMergeTombstoneLoggedConflict();
}
pub trait ISyncChangeBatchAdvancedImpl: Sized {
    fn GetFilterInfo();
    fn ConvertFullEnumerationChangeBatchToRegularChangeBatch();
    fn GetUpperBoundItemId();
    fn GetBatchLevelKnowledgeShouldBeApplied();
}
pub trait ISyncChangeBatchBaseImpl: Sized {
    fn GetChangeEnumerator();
    fn GetIsLastBatch();
    fn GetWorkEstimateForBatch();
    fn GetRemainingWorkEstimateForSession();
    fn BeginOrderedGroup();
    fn EndOrderedGroup();
    fn AddItemMetadataToGroup();
    fn GetLearnedKnowledge();
    fn GetPrerequisiteKnowledge();
    fn GetSourceForgottenKnowledge();
    fn SetLastBatch();
    fn SetWorkEstimateForBatch();
    fn SetRemainingWorkEstimateForSession();
    fn Serialize();
}
pub trait ISyncChangeBatchBase2Impl: Sized + ISyncChangeBatchBaseImpl {
    fn SerializeWithOptions();
}
pub trait ISyncChangeBatchWithFilterKeyMapImpl: Sized {
    fn GetFilterKeyMap();
    fn SetFilterKeyMap();
    fn SetFilterForgottenKnowledge();
    fn GetFilteredReplicaLearnedKnowledge();
    fn GetLearnedFilterForgottenKnowledge();
    fn GetFilteredReplicaLearnedForgottenKnowledge();
    fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete();
    fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete();
}
pub trait ISyncChangeBatchWithPrerequisiteImpl: Sized + ISyncChangeBatchBaseImpl {
    fn SetPrerequisiteKnowledge();
    fn GetLearnedKnowledgeWithPrerequisite();
    fn GetLearnedForgottenKnowledge();
}
pub trait ISyncChangeBuilderImpl: Sized {
    fn AddChangeUnitMetadata();
}
pub trait ISyncChangeUnitImpl: Sized {
    fn GetItemChange();
    fn GetChangeUnitId();
    fn GetChangeUnitVersion();
}
pub trait ISyncChangeWithFilterKeyMapImpl: Sized {
    fn GetFilterCount();
    fn GetFilterChange();
    fn GetAllChangeUnitsPresentFlag();
    fn GetFilterForgottenKnowledge();
    fn GetFilteredReplicaLearnedKnowledge();
    fn GetLearnedFilterForgottenKnowledge();
    fn GetFilteredReplicaLearnedForgottenKnowledge();
    fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete();
    fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete();
}
pub trait ISyncChangeWithPrerequisiteImpl: Sized {
    fn GetPrerequisiteKnowledge();
    fn GetLearnedKnowledgeWithPrerequisite();
}
pub trait ISyncConstraintCallbackImpl: Sized {
    fn OnConstraintConflict();
}
pub trait ISyncDataConverterImpl: Sized {
    fn ConvertDataRetrieverFromProviderFormat();
    fn ConvertDataRetrieverToProviderFormat();
    fn ConvertDataFromProviderFormat();
    fn ConvertDataToProviderFormat();
}
pub trait ISyncFilterImpl: Sized {
    fn IsIdentical();
    fn Serialize();
}
pub trait ISyncFilterDeserializerImpl: Sized {
    fn DeserializeSyncFilter();
}
pub trait ISyncFilterInfoImpl: Sized {
    fn Serialize();
}
pub trait ISyncFilterInfo2Impl: Sized + ISyncFilterInfoImpl {
    fn GetFlags();
}
pub trait ISyncFullEnumerationChangeImpl: Sized {
    fn GetLearnedKnowledgeAfterRecoveryComplete();
    fn GetLearnedForgottenKnowledge();
}
pub trait ISyncFullEnumerationChangeBatchImpl: Sized + ISyncChangeBatchBaseImpl {
    fn GetLearnedKnowledgeAfterRecoveryComplete();
    fn GetClosedLowerBoundItemId();
    fn GetClosedUpperBoundItemId();
}
pub trait ISyncFullEnumerationChangeBatch2Impl: Sized + ISyncFullEnumerationChangeBatchImpl + ISyncChangeBatchBaseImpl {
    fn AddMergeTombstoneMetadataToGroup();
}
pub trait ISyncKnowledgeImpl: Sized {
    fn GetOwnerReplicaId();
    fn Serialize();
    fn SetLocalTickCount();
    fn ContainsChange();
    fn ContainsChangeUnit();
    fn GetScopeVector();
    fn GetReplicaKeyMap();
    fn Clone();
    fn ConvertVersion();
    fn MapRemoteToLocal();
    fn Union();
    fn ProjectOntoItem();
    fn ProjectOntoChangeUnit();
    fn ProjectOntoRange();
    fn ExcludeItem();
    fn ExcludeChangeUnit();
    fn ContainsKnowledge();
    fn FindMinTickCountForReplica();
    fn GetRangeExceptions();
    fn GetSingleItemExceptions();
    fn GetChangeUnitExceptions();
    fn FindClockVectorForItem();
    fn FindClockVectorForChangeUnit();
    fn GetVersion();
}
pub trait ISyncKnowledge2Impl: Sized + ISyncKnowledgeImpl {
    fn GetIdParameters();
    fn ProjectOntoColumnSet();
    fn SerializeWithOptions();
    fn GetLowestUncontainedId();
    fn GetInspector();
    fn GetMinimumSupportedVersion();
    fn GetStatistics();
    fn ContainsKnowledgeForItem();
    fn ContainsKnowledgeForChangeUnit();
    fn ProjectOntoKnowledgeWithPrerequisite();
    fn Complement();
    fn IntersectsWithKnowledge();
    fn GetKnowledgeCookie();
    fn CompareToKnowledgeCookie();
}
pub trait ISyncMergeTombstoneChangeImpl: Sized {
    fn GetWinnerItemId();
}
pub trait ISyncProviderImpl: Sized {
    fn GetIdParameters();
}
pub trait ISyncProviderConfigUIImpl: Sized {
    fn Init();
    fn GetRegisteredProperties();
    fn CreateAndRegisterNewSyncProvider();
    fn ModifySyncProvider();
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ISyncProviderConfigUIInfoImpl: Sized + IPropertyStoreImpl {
    fn GetSyncProviderConfigUI();
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ISyncProviderInfoImpl: Sized + IPropertyStoreImpl {
    fn GetSyncProvider();
}
pub trait ISyncProviderRegistrationImpl: Sized {
    fn CreateSyncProviderConfigUIRegistrationInstance();
    fn UnregisterSyncProviderConfigUI();
    fn EnumerateSyncProviderConfigUIs();
    fn CreateSyncProviderRegistrationInstance();
    fn UnregisterSyncProvider();
    fn GetSyncProviderConfigUIInfoforProvider();
    fn EnumerateSyncProviders();
    fn GetSyncProviderInfo();
    fn GetSyncProviderFromInstanceId();
    fn GetSyncProviderConfigUIInfo();
    fn GetSyncProviderConfigUIFromInstanceId();
    fn GetSyncProviderState();
    fn SetSyncProviderState();
    fn RegisterForEvent();
    fn RevokeEvent();
    fn GetChange();
}
pub trait ISyncRegistrationChangeImpl: Sized {
    fn GetEvent();
    fn GetInstanceId();
}
pub trait ISyncSessionExtendedErrorInfoImpl: Sized {
    fn GetSyncProviderWithError();
}
pub trait ISyncSessionStateImpl: Sized {
    fn IsCanceled();
    fn GetInfoForChangeApplication();
    fn LoadInfoFromChangeApplication();
    fn GetForgottenKnowledgeRecoveryRangeStart();
    fn GetForgottenKnowledgeRecoveryRangeEnd();
    fn SetForgottenKnowledgeRecoveryRange();
    fn OnProgress();
}
pub trait ISyncSessionState2Impl: Sized + ISyncSessionStateImpl {
    fn SetProviderWithError();
    fn GetSessionErrorStatus();
}
pub trait ISynchronousDataRetrieverImpl: Sized {
    fn GetIdParameters();
    fn LoadChangeData();
}
