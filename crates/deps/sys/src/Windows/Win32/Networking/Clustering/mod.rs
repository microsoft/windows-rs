#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn AddClusterGroupDependency();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn AddClusterGroupSetDependency();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn AddClusterGroupToGroupSetDependency();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddClusterNode();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddClusterNodeEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn AddClusterResourceDependency();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn AddClusterResourceNode();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddClusterStorageNode();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddCrossClusterGroupSetDependency();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn AddResourceToClusterSharedVolumes();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BackupClusterDatabase();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CanResourceBeDependent();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn CancelClusterGroupOperation();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ChangeClusterResourceGroup();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ChangeClusterResourceGroupEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseCluster();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn CloseClusterCryptProvider();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseClusterGroup();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseClusterGroupSet();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseClusterNetInterface();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseClusterNetwork();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseClusterNode();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseClusterNotifyPort();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseClusterResource();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusAddClusterHealthFault();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusGetClusterHealthFaults();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusRemoveClusterHealthFault();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusWorkerCheckTerminate();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusWorkerCreate();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusWorkerTerminate();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusWorkerTerminateEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusWorkersTerminate();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterAddGroupToAffinityRule();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterAddGroupToGroupSet();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterAddGroupToGroupSetWithDomains();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterAffinityRuleControl();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterClearBackupStateForSharedVolume();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterCloseEnum();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterCloseEnumEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterControl();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterCreateAffinityRule();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterDecrypt();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterEncrypt();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterEnum();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterEnumEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterGetEnumCount();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterGetEnumCountEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterGetVolumeNameForVolumeMountPoint();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterGetVolumePathName();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterGroupCloseEnum();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterGroupCloseEnumEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterGroupControl();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterGroupEnum();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterGroupEnumEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterGroupGetEnumCount();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterGroupGetEnumCountEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterGroupOpenEnum();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterGroupOpenEnumEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterGroupSetCloseEnum();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterGroupSetControl();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterGroupSetEnum();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterGroupSetGetEnumCount();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterGroupSetOpenEnum();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterIsPathOnSharedVolume();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterNetInterfaceCloseEnum();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterNetInterfaceControl();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterNetInterfaceEnum();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterNetInterfaceOpenEnum();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterNetworkCloseEnum();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterNetworkControl();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterNetworkEnum();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterNetworkGetEnumCount();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterNetworkOpenEnum();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterNodeCloseEnum();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterNodeCloseEnumEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterNodeControl();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterNodeEnum();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterNodeEnumEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterNodeGetEnumCount();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterNodeGetEnumCountEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterNodeOpenEnum();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterNodeOpenEnumEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterNodeReplacement();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterOpenEnum();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterOpenEnumEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterPrepareSharedVolumeForBackup();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterRegBatchAddCommand();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterRegBatchCloseNotification();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterRegBatchReadCommand();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterRegCloseBatch();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterRegCloseBatchEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterRegCloseBatchNotifyPort();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn ClusterRegCloseKey();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterRegCloseReadBatch();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterRegCloseReadBatchEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterRegCloseReadBatchReply();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn ClusterRegCreateBatch();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn ClusterRegCreateBatchNotifyPort();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_Security`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Registry"))]
    pub fn ClusterRegCreateKey();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn ClusterRegCreateReadBatch();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ClusterRegDeleteKey();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ClusterRegDeleteValue();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ClusterRegEnumKey();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ClusterRegEnumValue();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterRegGetBatchNotification();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_Security`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Registry"))]
    pub fn ClusterRegGetKeySecurity();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ClusterRegOpenKey();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ClusterRegQueryInfoKey();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ClusterRegQueryValue();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterRegReadBatchAddCommand();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterRegReadBatchReplyNextCommand();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_Security`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Registry"))]
    pub fn ClusterRegSetKeySecurity();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ClusterRegSetValue();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterRegSyncDatabase();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterRemoveAffinityRule();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterRemoveGroupFromAffinityRule();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterRemoveGroupFromGroupSet();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterResourceCloseEnum();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterResourceCloseEnumEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterResourceControl();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterResourceControlAsUser();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterResourceEnum();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterResourceEnumEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterResourceGetEnumCount();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterResourceGetEnumCountEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterResourceOpenEnum();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterResourceOpenEnumEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterResourceTypeCloseEnum();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterResourceTypeControl();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterResourceTypeControlAsUser();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterResourceTypeEnum();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterResourceTypeGetEnumCount();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterResourceTypeOpenEnum();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterSetAccountAccess();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterSharedVolumeSetSnapshotState();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterUpgradeFunctionalLevel();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateCluster();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateClusterAvailabilitySet();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateClusterGroup();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateClusterGroupEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateClusterGroupSet();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateClusterNameAccount();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn CreateClusterNotifyPort();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn CreateClusterNotifyPortV2();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateClusterResource();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateClusterResourceType();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn DeleteClusterGroup();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn DeleteClusterGroupSet();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn DeleteClusterResource();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteClusterResourceType();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyCluster();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn DestroyClusterGroup();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn DetermineCNOResTypeFromCluster();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DetermineCNOResTypeFromNodelist();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn DetermineClusterCloudTypeFromCluster();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DetermineClusterCloudTypeFromNodelist();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn EvictClusterNode();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn EvictClusterNodeEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn FailClusterResource();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn FreeClusterCrypt();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeClusterHealthFault();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeClusterHealthFaultArray();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn GetClusterFromGroup();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn GetClusterFromNetInterface();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn GetClusterFromNetwork();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn GetClusterFromNode();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn GetClusterFromResource();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn GetClusterGroupKey();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterGroupState();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterInformation();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn GetClusterKey();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterNetInterface();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn GetClusterNetInterfaceKey();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn GetClusterNetInterfaceState();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterNetworkId();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn GetClusterNetworkKey();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn GetClusterNetworkState();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterNodeId();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn GetClusterNodeKey();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn GetClusterNodeState();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterNotify();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterNotifyV2();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterQuorumResource();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterResourceDependencyExpression();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn GetClusterResourceKey();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterResourceNetworkName();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterResourceState();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn GetClusterResourceTypeKey();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNodeCloudTypeDW();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNodeClusterState();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNotifyEventHandle();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeClusterHealthFault();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeClusterHealthFaultArray();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsFileOnClusterSharedVolume();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn MoveClusterGroup();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn MoveClusterGroupEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn OfflineClusterGroup();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn OfflineClusterGroupEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn OfflineClusterResource();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn OfflineClusterResourceEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn OnlineClusterGroup();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn OnlineClusterGroupEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn OnlineClusterResource();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn OnlineClusterResourceEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenCluster();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterCryptProvider();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterCryptProviderEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterGroup();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterGroupEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterGroupSet();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterNetInterface();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterNetInterfaceEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterNetwork();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterNetworkEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterNode();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn OpenClusterNodeById();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterNodeEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterResource();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterResourceEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn PauseClusterNode();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PauseClusterNodeEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryAppInstanceVersion();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterAppInstance();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn RegisterAppInstanceVersion();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterClusterNotify();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterClusterNotifyV2();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterClusterResourceTypeNotifyV2();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn RemoveClusterGroupDependency();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn RemoveClusterGroupSetDependency();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn RemoveClusterGroupToGroupSetDependency();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveClusterNameAccount();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn RemoveClusterResourceDependency();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn RemoveClusterResourceNode();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveClusterStorageNode();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveCrossClusterGroupSetDependency();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn RemoveResourceFromClusterSharedVolumes();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilAddUnknownProperties();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilCreateDirectoryTree();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilDupGroup();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilDupParameterBlock();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilDupResource();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilDupString();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilEnumGroups();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilEnumGroupsEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilEnumPrivateProperties();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilEnumProperties();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilEnumResources();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilEnumResourcesEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilEnumResourcesEx2();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilExpandEnvironmentStrings();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindBinaryProperty();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindDependentDiskResourceDriveLetter();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindDwordProperty();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindExpandSzProperty();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindExpandedSzProperty();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindFileTimeProperty();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindLongProperty();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindMultiSzProperty();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindSzProperty();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindULargeIntegerProperty();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilFreeEnvironment();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFreeParameterBlock();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilGetAllProperties();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilGetBinaryProperty();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilGetBinaryValue();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilGetClusterGroupType();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilGetClusterId();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilGetClusterRoleState();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilGetCoreClusterResources();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilGetCoreClusterResourcesEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilGetCoreGroup();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilGetDwordProperty();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilGetDwordValue();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilGetEnvironmentWithNetName();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetFileTimeProperty();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilGetLongProperty();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetMultiSzProperty();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn ResUtilGetPrivateProperties();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilGetProperties();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilGetPropertiesToParameterBlock();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilGetProperty();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetPropertyFormats();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilGetPropertySize();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilGetQwordValue();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceDependency();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceDependencyByClass();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceDependencyByClassEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceDependencyByName();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceDependencyByNameEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceDependencyEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceDependentIPAddressProps();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceName();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceNameDependency();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceNameDependencyEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetSzProperty();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilGetSzValue();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGroupsEqual();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilIsPathValid();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilIsResourceClassEqual();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilLeftPaxosIsLessThanRight();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilNodeEnum();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilPaxosComparer();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilPropertyListFromParameterBlock();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilRemoveResourceServiceEnvironment();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilResourceDepEnum();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilResourceTypesEqual();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilResourcesEqual();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetBinaryValue();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetDwordValue();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetExpandSzValue();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetMultiSzValue();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn ResUtilSetPrivatePropertyList();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetPropertyParameterBlock();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetPropertyParameterBlockEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetPropertyTable();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetPropertyTableEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetQwordValue();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilSetResourceServiceEnvironment();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn ResUtilSetResourceServiceStartParameters();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn ResUtilSetResourceServiceStartParametersEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetSzValue();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetUnknownProperties();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetValueEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilStartResourceService();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilStopResourceService();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Security`*"]
    #[cfg(feature = "Win32_Security")]
    pub fn ResUtilStopService();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilTerminateServiceProcessFromResDll();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilVerifyPrivatePropertyList();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilVerifyPropertyTable();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilVerifyResourceService();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Security`*"]
    #[cfg(feature = "Win32_Security")]
    pub fn ResUtilVerifyService();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilVerifyShutdownSafe();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilsDeleteKeyTree();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResetAllAppInstanceVersions();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn RestartClusterResource();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RestoreClusterDatabase();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResumeClusterNode();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResumeClusterNodeEx();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetAppInstanceCsvFlags();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClusterGroupName();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn SetClusterGroupNodeList();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClusterGroupSetDependencyExpression();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClusterName();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClusterNetworkName();
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn SetClusterNetworkPriorityOrder();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClusterQuorumResource();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClusterResourceDependencyExpression();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClusterResourceName();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClusterServiceAccountPassword();
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetGroupDependencyExpression();
}
