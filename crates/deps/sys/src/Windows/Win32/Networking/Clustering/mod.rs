#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn AddClusterGroupDependency(hdependentgroup: *const _HGROUP, hprovidergroup: *const _HGROUP) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn AddClusterGroupSetDependency(hdependentgroupset: *const _HGROUPSET, hprovidergroupset: *const _HGROUPSET) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn AddClusterGroupToGroupSetDependency(hdependentgroup: *const _HGROUP, hprovidergroupset: *const _HGROUPSET) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddClusterNode(hcluster: *const _HCLUSTER, lpsznodename: super::super::Foundation::PWSTR, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> *mut _HNODE;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddClusterNodeEx(hcluster: *const _HCLUSTER, lpsznodename: super::super::Foundation::PWSTR, dwflags: u32, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> *mut _HNODE;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn AddClusterResourceDependency(hresource: *const _HRESOURCE, hdependson: *const _HRESOURCE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn AddClusterResourceNode(hresource: *const _HRESOURCE, hnode: *const _HNODE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddClusterStorageNode(hcluster: *const _HCLUSTER, lpsznodename: super::super::Foundation::PWSTR, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void, lpszclusterstoragenodedescription: super::super::Foundation::PWSTR, lpszclusterstoragenodelocation: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddCrossClusterGroupSetDependency(hdependentgroupset: *const _HGROUPSET, lpremoteclustername: super::super::Foundation::PWSTR, lpremotegroupsetname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn AddResourceToClusterSharedVolumes(hresource: *const _HRESOURCE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BackupClusterDatabase(hcluster: *const _HCLUSTER, lpszpathname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CanResourceBeDependent(hresource: *const _HRESOURCE, hresourcedependent: *const _HRESOURCE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn CancelClusterGroupOperation(hgroup: *const _HGROUP, dwcancelflags_reserved: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ChangeClusterResourceGroup(hresource: *const _HRESOURCE, hgroup: *const _HGROUP) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ChangeClusterResourceGroupEx(hresource: *const _HRESOURCE, hgroup: *const _HGROUP, flags: u64) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseCluster(hcluster: *const _HCLUSTER) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn CloseClusterCryptProvider(hcluscryptprovider: *const _HCLUSCRYPTPROVIDER) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseClusterGroup(hgroup: *const _HGROUP) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseClusterGroupSet(hgroupset: *const _HGROUPSET) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseClusterNetInterface(hnetinterface: *const _HNETINTERFACE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseClusterNetwork(hnetwork: *const _HNETWORK) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseClusterNode(hnode: *const _HNODE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseClusterNotifyPort(hchange: *const _HCHANGE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseClusterResource(hresource: *const _HRESOURCE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusAddClusterHealthFault(hcluster: *const _HCLUSTER, failure: *const CLUSTER_HEALTH_FAULT, param2: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusGetClusterHealthFaults(hcluster: *const _HCLUSTER, objects: *mut CLUSTER_HEALTH_FAULT_ARRAY, flags: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusRemoveClusterHealthFault(hcluster: *const _HCLUSTER, id: super::super::Foundation::PWSTR, flags: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusWorkerCheckTerminate(lpworker: *mut CLUS_WORKER) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusWorkerCreate(lpworker: *mut CLUS_WORKER, lpstartaddress: PWORKER_START_ROUTINE, lpparameter: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusWorkerTerminate(lpworker: *const CLUS_WORKER);
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusWorkerTerminateEx(clusworker: *mut CLUS_WORKER, timeoutinmilliseconds: u32, waitonly: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusWorkersTerminate(clusworkers: *mut *mut CLUS_WORKER, clusworkerscount: usize, timeoutinmilliseconds: u32, waitonly: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterAddGroupToAffinityRule(hcluster: *const _HCLUSTER, rulename: super::super::Foundation::PWSTR, hgroup: *const _HGROUP) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterAddGroupToGroupSet(hgroupset: *const _HGROUPSET, hgroup: *const _HGROUP) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterAddGroupToGroupSetWithDomains(hgroupset: *const _HGROUPSET, hgroup: *const _HGROUP, faultdomain: u32, updatedomain: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterAffinityRuleControl(hcluster: *const _HCLUSTER, affinityrulename: super::super::Foundation::PWSTR, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterClearBackupStateForSharedVolume(lpszvolumepathname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterCloseEnum(henum: *const _HCLUSENUM) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterCloseEnumEx(hclusterenum: *const _HCLUSENUMEX) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterControl(hcluster: *const _HCLUSTER, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterCreateAffinityRule(hcluster: *const _HCLUSTER, rulename: super::super::Foundation::PWSTR, ruletype: CLUS_AFFINITY_RULE_TYPE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterDecrypt(hcluscryptprovider: *const _HCLUSCRYPTPROVIDER, pcryptinput: *const u8, cbcryptinput: u32, ppcryptoutput: *mut *mut u8, pcbcryptoutput: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterEncrypt(hcluscryptprovider: *const _HCLUSCRYPTPROVIDER, pdata: *const u8, cbdata: u32, ppdata: *mut *mut u8, pcbdata: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterEnum(henum: *const _HCLUSENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterEnumEx(hclusterenum: *const _HCLUSENUMEX, dwindex: u32, pitem: *mut CLUSTER_ENUM_ITEM, cbitem: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterGetEnumCount(henum: *const _HCLUSENUM) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterGetEnumCountEx(hclusterenum: *const _HCLUSENUMEX) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterGetVolumeNameForVolumeMountPoint(lpszvolumemountpoint: super::super::Foundation::PWSTR, lpszvolumename: super::super::Foundation::PWSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterGetVolumePathName(lpszfilename: super::super::Foundation::PWSTR, lpszvolumepathname: super::super::Foundation::PWSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterGroupCloseEnum(hgroupenum: *const _HGROUPENUM) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterGroupCloseEnumEx(hgroupenumex: *const _HGROUPENUMEX) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterGroupControl(hgroup: *const _HGROUP, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterGroupEnum(hgroupenum: *const _HGROUPENUM, dwindex: u32, lpdwtype: *mut u32, lpszresourcename: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterGroupEnumEx(hgroupenumex: *const _HGROUPENUMEX, dwindex: u32, pitem: *mut CLUSTER_GROUP_ENUM_ITEM, cbitem: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterGroupGetEnumCount(hgroupenum: *const _HGROUPENUM) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterGroupGetEnumCountEx(hgroupenumex: *const _HGROUPENUMEX) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterGroupOpenEnum(hgroup: *const _HGROUP, dwtype: u32) -> *mut _HGROUPENUM;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterGroupOpenEnumEx(hcluster: *const _HCLUSTER, lpszproperties: super::super::Foundation::PWSTR, cbproperties: u32, lpszroproperties: super::super::Foundation::PWSTR, cbroproperties: u32, dwflags: u32) -> *mut _HGROUPENUMEX;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterGroupSetCloseEnum(hgroupsetenum: *mut _HGROUPSETENUM) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterGroupSetControl(hgroupset: *const _HGROUPSET, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterGroupSetEnum(hgroupsetenum: *const _HGROUPSETENUM, dwindex: u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterGroupSetGetEnumCount(hgroupsetenum: *mut _HGROUPSETENUM) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterGroupSetOpenEnum(hcluster: *mut _HCLUSTER) -> *mut _HGROUPSETENUM;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterIsPathOnSharedVolume(lpszpathname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterNetInterfaceCloseEnum(hnetinterfaceenum: *const _HNETINTERFACEENUM) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterNetInterfaceControl(hnetinterface: *const _HNETINTERFACE, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterNetInterfaceEnum(hnetinterfaceenum: *const _HNETINTERFACEENUM, dwindex: u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterNetInterfaceOpenEnum(hcluster: *const _HCLUSTER, lpsznodename: super::super::Foundation::PWSTR, lpsznetworkname: super::super::Foundation::PWSTR) -> *mut _HNETINTERFACEENUM;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterNetworkCloseEnum(hnetworkenum: *const _HNETWORKENUM) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterNetworkControl(hnetwork: *const _HNETWORK, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterNetworkEnum(hnetworkenum: *const _HNETWORKENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterNetworkGetEnumCount(hnetworkenum: *const _HNETWORKENUM) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterNetworkOpenEnum(hnetwork: *const _HNETWORK, dwtype: u32) -> *mut _HNETWORKENUM;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterNodeCloseEnum(hnodeenum: *const _HNODEENUM) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterNodeCloseEnumEx(hnodeenum: *const _HNODEENUMEX) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterNodeControl(hnode: *const _HNODE, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterNodeEnum(hnodeenum: *const _HNODEENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterNodeEnumEx(hnodeenum: *const _HNODEENUMEX, dwindex: u32, pitem: *mut CLUSTER_ENUM_ITEM, cbitem: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterNodeGetEnumCount(hnodeenum: *const _HNODEENUM) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterNodeGetEnumCountEx(hnodeenum: *const _HNODEENUMEX) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterNodeOpenEnum(hnode: *const _HNODE, dwtype: u32) -> *mut _HNODEENUM;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterNodeOpenEnumEx(hnode: *const _HNODE, dwtype: u32, poptions: *const ::core::ffi::c_void) -> *mut _HNODEENUMEX;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterNodeReplacement(hcluster: *const _HCLUSTER, lpsznodenamecurrent: super::super::Foundation::PWSTR, lpsznodenamenew: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterOpenEnum(hcluster: *const _HCLUSTER, dwtype: u32) -> *mut _HCLUSENUM;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterOpenEnumEx(hcluster: *const _HCLUSTER, dwtype: u32, poptions: *const ::core::ffi::c_void) -> *mut _HCLUSENUMEX;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterPrepareSharedVolumeForBackup(lpszfilename: super::super::Foundation::PWSTR, lpszvolumepathname: super::super::Foundation::PWSTR, lpcchvolumepathname: *mut u32, lpszvolumename: super::super::Foundation::PWSTR, lpcchvolumename: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterRegBatchAddCommand(hregbatch: *const _HREGBATCH, dwcommand: CLUSTER_REG_COMMAND, wzname: super::super::Foundation::PWSTR, dwoptions: u32, lpdata: *const ::core::ffi::c_void, cbdata: u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterRegBatchCloseNotification(hbatchnotification: *const _HREGBATCHNOTIFICATION) -> i32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterRegBatchReadCommand(hbatchnotification: *const _HREGBATCHNOTIFICATION, pbatchcommand: *mut CLUSTER_BATCH_COMMAND) -> i32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterRegCloseBatch(hregbatch: *const _HREGBATCH, bcommit: super::super::Foundation::BOOL, failedcommandnumber: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterRegCloseBatchEx(hregbatch: *const _HREGBATCH, flags: u32, failedcommandnumber: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterRegCloseBatchNotifyPort(hbatchnotifyport: *const _HREGBATCHPORT) -> i32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn ClusterRegCloseKey(hkey: super::super::System::Registry::HKEY) -> i32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterRegCloseReadBatch(hregreadbatch: *const _HREGREADBATCH, phregreadbatchreply: *mut *mut _HREGREADBATCHREPLY) -> i32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterRegCloseReadBatchEx(hregreadbatch: *const _HREGREADBATCH, flags: u32, phregreadbatchreply: *mut *mut _HREGREADBATCHREPLY) -> i32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterRegCloseReadBatchReply(hregreadbatchreply: *const _HREGREADBATCHREPLY) -> i32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn ClusterRegCreateBatch(hkey: super::super::System::Registry::HKEY, phregbatch: *mut *mut _HREGBATCH) -> i32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn ClusterRegCreateBatchNotifyPort(hkey: super::super::System::Registry::HKEY, phbatchnotifyport: *mut *mut _HREGBATCHPORT) -> i32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_Security`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Registry"))]
    pub fn ClusterRegCreateKey(hkey: super::super::System::Registry::HKEY, lpszsubkey: super::super::Foundation::PWSTR, dwoptions: u32, samdesired: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, phkresult: *mut super::super::System::Registry::HKEY, lpdwdisposition: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn ClusterRegCreateReadBatch(hkey: super::super::System::Registry::HKEY, phregreadbatch: *mut *mut _HREGREADBATCH) -> i32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ClusterRegDeleteKey(hkey: super::super::System::Registry::HKEY, lpszsubkey: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ClusterRegDeleteValue(hkey: super::super::System::Registry::HKEY, lpszvaluename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ClusterRegEnumKey(hkey: super::super::System::Registry::HKEY, dwindex: u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32, lpftlastwritetime: *mut super::super::Foundation::FILETIME) -> i32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ClusterRegEnumValue(hkey: super::super::System::Registry::HKEY, dwindex: u32, lpszvaluename: super::super::Foundation::PWSTR, lpcchvaluename: *mut u32, lpdwtype: *mut u32, lpdata: *mut u8, lpcbdata: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterRegGetBatchNotification(hbatchnotify: *const _HREGBATCHPORT, phbatchnotification: *mut *mut _HREGBATCHNOTIFICATION) -> i32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_Security`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Registry"))]
    pub fn ClusterRegGetKeySecurity(hkey: super::super::System::Registry::HKEY, requestedinformation: u32, psecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ClusterRegOpenKey(hkey: super::super::System::Registry::HKEY, lpszsubkey: super::super::Foundation::PWSTR, samdesired: u32, phkresult: *mut super::super::System::Registry::HKEY) -> i32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ClusterRegQueryInfoKey(hkey: super::super::System::Registry::HKEY, lpcsubkeys: *const u32, lpcchmaxsubkeylen: *const u32, lpcvalues: *const u32, lpcchmaxvaluenamelen: *const u32, lpcbmaxvaluelen: *const u32, lpcbsecuritydescriptor: *const u32, lpftlastwritetime: *const super::super::Foundation::FILETIME) -> i32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ClusterRegQueryValue(hkey: super::super::System::Registry::HKEY, lpszvaluename: super::super::Foundation::PWSTR, lpdwvaluetype: *mut u32, lpdata: *mut u8, lpcbdata: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterRegReadBatchAddCommand(hregreadbatch: *const _HREGREADBATCH, wzsubkeyname: super::super::Foundation::PWSTR, wzvaluename: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterRegReadBatchReplyNextCommand(hregreadbatchreply: *const _HREGREADBATCHREPLY, pbatchcommand: *mut CLUSTER_READ_BATCH_COMMAND) -> i32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_Security`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Registry"))]
    pub fn ClusterRegSetKeySecurity(hkey: super::super::System::Registry::HKEY, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> i32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ClusterRegSetValue(hkey: super::super::System::Registry::HKEY, lpszvaluename: super::super::Foundation::PWSTR, dwtype: u32, lpdata: *const u8, cbdata: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterRegSyncDatabase(hcluster: *const _HCLUSTER, flags: u32) -> i32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterRemoveAffinityRule(hcluster: *const _HCLUSTER, rulename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterRemoveGroupFromAffinityRule(hcluster: *const _HCLUSTER, rulename: super::super::Foundation::PWSTR, hgroup: *const _HGROUP) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterRemoveGroupFromGroupSet(hgroup: *const _HGROUP) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterResourceCloseEnum(hresenum: *const _HRESENUM) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterResourceCloseEnumEx(hresourceenumex: *const _HRESENUMEX) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterResourceControl(hresource: *const _HRESOURCE, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterResourceControlAsUser(hresource: *const _HRESOURCE, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterResourceEnum(hresenum: *const _HRESENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterResourceEnumEx(hresourceenumex: *const _HRESENUMEX, dwindex: u32, pitem: *mut CLUSTER_RESOURCE_ENUM_ITEM, cbitem: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterResourceGetEnumCount(hresenum: *const _HRESENUM) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterResourceGetEnumCountEx(hresourceenumex: *const _HRESENUMEX) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterResourceOpenEnum(hresource: *const _HRESOURCE, dwtype: u32) -> *mut _HRESENUM;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterResourceOpenEnumEx(hcluster: *const _HCLUSTER, lpszproperties: super::super::Foundation::PWSTR, cbproperties: u32, lpszroproperties: super::super::Foundation::PWSTR, cbroproperties: u32, dwflags: u32) -> *mut _HRESENUMEX;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterResourceTypeCloseEnum(hrestypeenum: *const _HRESTYPEENUM) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterResourceTypeControl(hcluster: *const _HCLUSTER, lpszresourcetypename: super::super::Foundation::PWSTR, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterResourceTypeControlAsUser(hcluster: *const _HCLUSTER, lpszresourcetypename: super::super::Foundation::PWSTR, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterResourceTypeEnum(hrestypeenum: *const _HRESTYPEENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ClusterResourceTypeGetEnumCount(hrestypeenum: *const _HRESTYPEENUM) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterResourceTypeOpenEnum(hcluster: *const _HCLUSTER, lpszresourcetypename: super::super::Foundation::PWSTR, dwtype: u32) -> *mut _HRESTYPEENUM;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterSetAccountAccess(hcluster: *const _HCLUSTER, szaccountsid: super::super::Foundation::PWSTR, dwaccess: u32, dwcontroltype: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterSharedVolumeSetSnapshotState(guidsnapshotset: ::windows_sys::core::GUID, lpszvolumename: super::super::Foundation::PWSTR, state: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterUpgradeFunctionalLevel(hcluster: *const _HCLUSTER, perform: super::super::Foundation::BOOL, pfnprogresscallback: PCLUSTER_UPGRADE_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateCluster(pconfig: *const CREATE_CLUSTER_CONFIG, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> *mut _HCLUSTER;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateClusterAvailabilitySet(hcluster: *const _HCLUSTER, lpavailabilitysetname: super::super::Foundation::PWSTR, pavailabilitysetconfig: *const CLUSTER_AVAILABILITY_SET_CONFIG) -> *mut _HGROUPSET;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateClusterGroup(hcluster: *const _HCLUSTER, lpszgroupname: super::super::Foundation::PWSTR) -> *mut _HGROUP;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateClusterGroupEx(hcluster: *const _HCLUSTER, lpszgroupname: super::super::Foundation::PWSTR, pgroupinfo: *const CLUSTER_CREATE_GROUP_INFO) -> *mut _HGROUP;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateClusterGroupSet(hcluster: *const _HCLUSTER, groupsetname: super::super::Foundation::PWSTR) -> *mut _HGROUPSET;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateClusterNameAccount(hcluster: *const _HCLUSTER, pconfig: *const CREATE_CLUSTER_NAME_ACCOUNT, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn CreateClusterNotifyPort(hchange: *const _HCHANGE, hcluster: *const _HCLUSTER, dwfilter: u32, dwnotifykey: usize) -> *mut _HCHANGE;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn CreateClusterNotifyPortV2(hchange: *const _HCHANGE, hcluster: *const _HCLUSTER, filters: *const NOTIFY_FILTER_AND_TYPE, dwfiltercount: u32, dwnotifykey: usize) -> *mut _HCHANGE;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateClusterResource(hgroup: *const _HGROUP, lpszresourcename: super::super::Foundation::PWSTR, lpszresourcetype: super::super::Foundation::PWSTR, dwflags: u32) -> *mut _HRESOURCE;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateClusterResourceType(hcluster: *const _HCLUSTER, lpszresourcetypename: super::super::Foundation::PWSTR, lpszdisplayname: super::super::Foundation::PWSTR, lpszresourcetypedll: super::super::Foundation::PWSTR, dwlooksalivepollinterval: u32, dwisalivepollinterval: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn DeleteClusterGroup(hgroup: *const _HGROUP) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn DeleteClusterGroupSet(hgroupset: *const _HGROUPSET) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn DeleteClusterResource(hresource: *const _HRESOURCE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteClusterResourceType(hcluster: *const _HCLUSTER, lpszresourcetypename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyCluster(hcluster: *const _HCLUSTER, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void, fdeletevirtualcomputerobjects: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn DestroyClusterGroup(hgroup: *const _HGROUP) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn DetermineCNOResTypeFromCluster(hcluster: *const _HCLUSTER, pcnorestype: *mut CLUSTER_MGMT_POINT_RESTYPE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DetermineCNOResTypeFromNodelist(cnodes: u32, ppsznodenames: *const super::super::Foundation::PWSTR, pcnorestype: *mut CLUSTER_MGMT_POINT_RESTYPE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn DetermineClusterCloudTypeFromCluster(hcluster: *const _HCLUSTER, pcloudtype: *mut CLUSTER_CLOUD_TYPE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DetermineClusterCloudTypeFromNodelist(cnodes: u32, ppsznodenames: *const super::super::Foundation::PWSTR, pcloudtype: *mut CLUSTER_CLOUD_TYPE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn EvictClusterNode(hnode: *const _HNODE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn EvictClusterNodeEx(hnode: *const _HNODE, dwtimeout: u32, phrcleanupstatus: *mut ::windows_sys::core::HRESULT) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn FailClusterResource(hresource: *const _HRESOURCE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn FreeClusterCrypt(pcryptinfo: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeClusterHealthFault(clusterhealthfault: *mut CLUSTER_HEALTH_FAULT) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeClusterHealthFaultArray(clusterhealthfaultarray: *mut CLUSTER_HEALTH_FAULT_ARRAY) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn GetClusterFromGroup(hgroup: *const _HGROUP) -> *mut _HCLUSTER;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn GetClusterFromNetInterface(hnetinterface: *const _HNETINTERFACE) -> *mut _HCLUSTER;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn GetClusterFromNetwork(hnetwork: *const _HNETWORK) -> *mut _HCLUSTER;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn GetClusterFromNode(hnode: *const _HNODE) -> *mut _HCLUSTER;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn GetClusterFromResource(hresource: *const _HRESOURCE) -> *mut _HCLUSTER;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn GetClusterGroupKey(hgroup: *const _HGROUP, samdesired: u32) -> super::super::System::Registry::HKEY;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterGroupState(hgroup: *const _HGROUP, lpsznodename: super::super::Foundation::PWSTR, lpcchnodename: *mut u32) -> CLUSTER_GROUP_STATE;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterInformation(hcluster: *const _HCLUSTER, lpszclustername: super::super::Foundation::PWSTR, lpcchclustername: *mut u32, lpclusterinfo: *mut CLUSTERVERSIONINFO) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn GetClusterKey(hcluster: *const _HCLUSTER, samdesired: u32) -> super::super::System::Registry::HKEY;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterNetInterface(hcluster: *const _HCLUSTER, lpsznodename: super::super::Foundation::PWSTR, lpsznetworkname: super::super::Foundation::PWSTR, lpszinterfacename: super::super::Foundation::PWSTR, lpcchinterfacename: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn GetClusterNetInterfaceKey(hnetinterface: *const _HNETINTERFACE, samdesired: u32) -> super::super::System::Registry::HKEY;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn GetClusterNetInterfaceState(hnetinterface: *const _HNETINTERFACE) -> CLUSTER_NETINTERFACE_STATE;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterNetworkId(hnetwork: *const _HNETWORK, lpsznetworkid: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn GetClusterNetworkKey(hnetwork: *const _HNETWORK, samdesired: u32) -> super::super::System::Registry::HKEY;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn GetClusterNetworkState(hnetwork: *const _HNETWORK) -> CLUSTER_NETWORK_STATE;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterNodeId(hnode: *const _HNODE, lpsznodeid: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn GetClusterNodeKey(hnode: *const _HNODE, samdesired: u32) -> super::super::System::Registry::HKEY;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn GetClusterNodeState(hnode: *const _HNODE) -> CLUSTER_NODE_STATE;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterNotify(hchange: *const _HCHANGE, lpdwnotifykey: *mut usize, lpdwfiltertype: *mut u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32, dwmilliseconds: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterNotifyV2(hchange: *const _HCHANGE, lpdwnotifykey: *mut usize, pfilterandtype: *mut NOTIFY_FILTER_AND_TYPE, buffer: *mut u8, lpbbuffersize: *mut u32, lpszobjectid: super::super::Foundation::PWSTR, lpcchobjectid: *mut u32, lpszparentid: super::super::Foundation::PWSTR, lpcchparentid: *mut u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32, lpsztype: super::super::Foundation::PWSTR, lpcchtype: *mut u32, dwmilliseconds: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterQuorumResource(hcluster: *const _HCLUSTER, lpszresourcename: super::super::Foundation::PWSTR, lpcchresourcename: *mut u32, lpszdevicename: super::super::Foundation::PWSTR, lpcchdevicename: *mut u32, lpdwmaxquorumlogsize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterResourceDependencyExpression(hresource: *const _HRESOURCE, lpszdependencyexpression: super::super::Foundation::PWSTR, lpcchdependencyexpression: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn GetClusterResourceKey(hresource: *const _HRESOURCE, samdesired: u32) -> super::super::System::Registry::HKEY;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterResourceNetworkName(hresource: *const _HRESOURCE, lpbuffer: super::super::Foundation::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterResourceState(hresource: *const _HRESOURCE, lpsznodename: super::super::Foundation::PWSTR, lpcchnodename: *mut u32, lpszgroupname: super::super::Foundation::PWSTR, lpcchgroupname: *mut u32) -> CLUSTER_RESOURCE_STATE;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn GetClusterResourceTypeKey(hcluster: *const _HCLUSTER, lpsztypename: super::super::Foundation::PWSTR, samdesired: u32) -> super::super::System::Registry::HKEY;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNodeCloudTypeDW(ppsznodename: super::super::Foundation::PWSTR, nodecloudtype: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNodeClusterState(lpsznodename: super::super::Foundation::PWSTR, pdwclusterstate: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNotifyEventHandle(hchange: *const _HCHANGE, lphtargetevent: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeClusterHealthFault(clusterhealthfault: *mut CLUSTER_HEALTH_FAULT) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeClusterHealthFaultArray(clusterhealthfaultarray: *mut CLUSTER_HEALTH_FAULT_ARRAY) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsFileOnClusterSharedVolume(lpszpathname: super::super::Foundation::PWSTR, pbfileisonsharedvolume: *mut super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn MoveClusterGroup(hgroup: *const _HGROUP, hdestinationnode: *const _HNODE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn MoveClusterGroupEx(hgroup: *const _HGROUP, hdestinationnode: *const _HNODE, dwmoveflags: u32, lpinbuffer: *const u8, cbinbuffersize: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn OfflineClusterGroup(hgroup: *const _HGROUP) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn OfflineClusterGroupEx(hgroup: *const _HGROUP, dwofflineflags: u32, lpinbuffer: *const u8, cbinbuffersize: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn OfflineClusterResource(hresource: *const _HRESOURCE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn OfflineClusterResourceEx(hresource: *const _HRESOURCE, dwofflineflags: u32, lpinbuffer: *const u8, cbinbuffersize: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn OnlineClusterGroup(hgroup: *const _HGROUP, hdestinationnode: *const _HNODE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn OnlineClusterGroupEx(hgroup: *const _HGROUP, hdestinationnode: *const _HNODE, dwonlineflags: u32, lpinbuffer: *const u8, cbinbuffersize: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn OnlineClusterResource(hresource: *const _HRESOURCE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn OnlineClusterResourceEx(hresource: *const _HRESOURCE, dwonlineflags: u32, lpinbuffer: *const u8, cbinbuffersize: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenCluster(lpszclustername: super::super::Foundation::PWSTR) -> *mut _HCLUSTER;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterCryptProvider(lpszresource: super::super::Foundation::PWSTR, lpszprovider: *const i8, dwtype: u32, dwflags: u32) -> *mut _HCLUSCRYPTPROVIDER;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterCryptProviderEx(lpszresource: super::super::Foundation::PWSTR, lpszkeyname: super::super::Foundation::PWSTR, lpszprovider: *const i8, dwtype: u32, dwflags: u32) -> *mut _HCLUSCRYPTPROVIDER;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterEx(lpszclustername: super::super::Foundation::PWSTR, desiredaccess: u32, grantedaccess: *mut u32) -> *mut _HCLUSTER;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterGroup(hcluster: *const _HCLUSTER, lpszgroupname: super::super::Foundation::PWSTR) -> *mut _HGROUP;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterGroupEx(hcluster: *const _HCLUSTER, lpszgroupname: super::super::Foundation::PWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HGROUP;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterGroupSet(hcluster: *const _HCLUSTER, lpszgroupsetname: super::super::Foundation::PWSTR) -> *mut _HGROUPSET;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterNetInterface(hcluster: *const _HCLUSTER, lpszinterfacename: super::super::Foundation::PWSTR) -> *mut _HNETINTERFACE;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterNetInterfaceEx(hcluster: *const _HCLUSTER, lpszinterfacename: super::super::Foundation::PWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HNETINTERFACE;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterNetwork(hcluster: *const _HCLUSTER, lpsznetworkname: super::super::Foundation::PWSTR) -> *mut _HNETWORK;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterNetworkEx(hcluster: *const _HCLUSTER, lpsznetworkname: super::super::Foundation::PWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HNETWORK;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterNode(hcluster: *const _HCLUSTER, lpsznodename: super::super::Foundation::PWSTR) -> *mut _HNODE;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn OpenClusterNodeById(hcluster: *const _HCLUSTER, nodeid: u32) -> *mut _HNODE;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterNodeEx(hcluster: *const _HCLUSTER, lpsznodename: super::super::Foundation::PWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HNODE;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterResource(hcluster: *const _HCLUSTER, lpszresourcename: super::super::Foundation::PWSTR) -> *mut _HRESOURCE;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterResourceEx(hcluster: *const _HCLUSTER, lpszresourcename: super::super::Foundation::PWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HRESOURCE;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn PauseClusterNode(hnode: *const _HNODE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PauseClusterNodeEx(hnode: *const _HNODE, bdrainnode: super::super::Foundation::BOOL, dwpauseflags: u32, hnodedraintarget: *const _HNODE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryAppInstanceVersion(appinstanceid: *const ::windows_sys::core::GUID, instanceversionhigh: *mut u64, instanceversionlow: *mut u64, versionstatus: *mut super::super::Foundation::NTSTATUS) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterAppInstance(processhandle: super::super::Foundation::HANDLE, appinstanceid: *const ::windows_sys::core::GUID, childreninheritappinstance: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn RegisterAppInstanceVersion(appinstanceid: *const ::windows_sys::core::GUID, instanceversionhigh: u64, instanceversionlow: u64) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterClusterNotify(hchange: *const _HCHANGE, dwfiltertype: u32, hobject: super::super::Foundation::HANDLE, dwnotifykey: usize) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterClusterNotifyV2(hchange: *const _HCHANGE, filter: NOTIFY_FILTER_AND_TYPE, hobject: super::super::Foundation::HANDLE, dwnotifykey: usize) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterClusterResourceTypeNotifyV2(hchange: *const _HCHANGE, hcluster: *const _HCLUSTER, flags: i64, restypename: super::super::Foundation::PWSTR, dwnotifykey: usize) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn RemoveClusterGroupDependency(hgroup: *const _HGROUP, hdependson: *const _HGROUP) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn RemoveClusterGroupSetDependency(hgroupset: *const _HGROUPSET, hdependson: *const _HGROUPSET) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn RemoveClusterGroupToGroupSetDependency(hgroup: *const _HGROUP, hdependson: *const _HGROUPSET) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveClusterNameAccount(hcluster: *const _HCLUSTER, bdeletecomputerobjects: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn RemoveClusterResourceDependency(hresource: *const _HRESOURCE, hdependson: *const _HRESOURCE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn RemoveClusterResourceNode(hresource: *const _HRESOURCE, hnode: *const _HNODE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveClusterStorageNode(hcluster: *const _HCLUSTER, lpszclusterstorageenclosurename: super::super::Foundation::PWSTR, dwtimeout: u32, dwflags: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveCrossClusterGroupSetDependency(hdependentgroupset: *const _HGROUPSET, lpremoteclustername: super::super::Foundation::PWSTR, lpremotegroupsetname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn RemoveResourceFromClusterSharedVolumes(hresource: *const _HRESOURCE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilAddUnknownProperties(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut ::core::ffi::c_void, pcboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilCreateDirectoryTree(pszpath: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilDupGroup(group: *mut _HGROUP, copy: *mut *mut _HGROUP) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilDupParameterBlock(poutparams: *mut u8, pinparams: *const u8, ppropertytable: *const RESUTIL_PROPERTY_ITEM) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilDupResource(group: *mut _HRESOURCE, copy: *mut *mut _HRESOURCE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilDupString(pszinstring: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilEnumGroups(hcluster: *mut _HCLUSTER, hself: *mut _HGROUP, prescallback: LPGROUP_CALLBACK_EX, pparameter: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilEnumGroupsEx(hcluster: *mut _HCLUSTER, hself: *mut _HGROUP, grouptype: CLUSGROUP_TYPE, prescallback: LPGROUP_CALLBACK_EX, pparameter: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilEnumPrivateProperties(hkeyclusterkey: super::super::System::Registry::HKEY, pszoutproperties: super::super::Foundation::PWSTR, cboutpropertiessize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilEnumProperties(ppropertytable: *const RESUTIL_PROPERTY_ITEM, pszoutproperties: super::super::Foundation::PWSTR, cboutpropertiessize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilEnumResources(hself: *mut _HRESOURCE, lpszrestypename: super::super::Foundation::PWSTR, prescallback: LPRESOURCE_CALLBACK, pparameter: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilEnumResourcesEx(hcluster: *mut _HCLUSTER, hself: *mut _HRESOURCE, lpszrestypename: super::super::Foundation::PWSTR, prescallback: LPRESOURCE_CALLBACK_EX, pparameter: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilEnumResourcesEx2(hcluster: *mut _HCLUSTER, hself: *mut _HRESOURCE, lpszrestypename: super::super::Foundation::PWSTR, prescallback: LPRESOURCE_CALLBACK_EX, pparameter: *mut ::core::ffi::c_void, dwdesiredaccess: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilExpandEnvironmentStrings(pszsrc: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindBinaryProperty(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: super::super::Foundation::PWSTR, pbpropertyvalue: *mut *mut u8, pcbpropertyvaluesize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindDependentDiskResourceDriveLetter(hcluster: *const _HCLUSTER, hresource: *const _HRESOURCE, pszdriveletter: super::super::Foundation::PWSTR, pcchdriveletter: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindDwordProperty(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: super::super::Foundation::PWSTR, pdwpropertyvalue: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindExpandSzProperty(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: super::super::Foundation::PWSTR, pszpropertyvalue: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindExpandedSzProperty(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: super::super::Foundation::PWSTR, pszpropertyvalue: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindFileTimeProperty(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: super::super::Foundation::PWSTR, pftpropertyvalue: *mut super::super::Foundation::FILETIME) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindLongProperty(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: super::super::Foundation::PWSTR, plpropertyvalue: *mut i32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindMultiSzProperty(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: super::super::Foundation::PWSTR, pszpropertyvalue: *mut super::super::Foundation::PWSTR, pcbpropertyvaluesize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindSzProperty(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: super::super::Foundation::PWSTR, pszpropertyvalue: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindULargeIntegerProperty(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: super::super::Foundation::PWSTR, plpropertyvalue: *mut u64) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilFreeEnvironment(lpenvironment: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFreeParameterBlock(poutparams: *mut u8, pinparams: *const u8, ppropertytable: *const RESUTIL_PROPERTY_ITEM);
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilGetAllProperties(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut ::core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilGetBinaryProperty(ppboutvalue: *mut *mut u8, pcboutvaluesize: *mut u32, pvaluestruct: *const CLUSPROP_BINARY, pboldvalue: *const u8, cboldvaluesize: u32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilGetBinaryValue(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, ppboutvalue: *mut *mut u8, pcboutvaluesize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilGetClusterGroupType(hgroup: *mut _HGROUP, grouptype: *mut CLUSGROUP_TYPE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilGetClusterId(hcluster: *mut _HCLUSTER, guid: *mut ::windows_sys::core::GUID) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilGetClusterRoleState(hcluster: *const _HCLUSTER, eclusterrole: CLUSTER_ROLE) -> CLUSTER_ROLE_STATE;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilGetCoreClusterResources(hcluster: *const _HCLUSTER, phclusternameresource: *mut *mut _HRESOURCE, phclusteripaddressresource: *mut *mut _HRESOURCE, phclusterquorumresource: *mut *mut _HRESOURCE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilGetCoreClusterResourcesEx(hclusterin: *const _HCLUSTER, phclusternameresourceout: *mut *mut _HRESOURCE, phclusterquorumresourceout: *mut *mut _HRESOURCE, dwdesiredaccess: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilGetCoreGroup(hcluster: *mut _HCLUSTER) -> *mut _HGROUP;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilGetDwordProperty(pdwoutvalue: *mut u32, pvaluestruct: *const CLUSPROP_DWORD, dwoldvalue: u32, dwminimum: u32, dwmaximum: u32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilGetDwordValue(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, pdwoutvalue: *mut u32, dwdefaultvalue: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilGetEnvironmentWithNetName(hresource: *const _HRESOURCE) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetFileTimeProperty(pftoutvalue: *mut super::super::Foundation::FILETIME, pvaluestruct: *const CLUSPROP_FILETIME, ftoldvalue: super::super::Foundation::FILETIME, ftminimum: super::super::Foundation::FILETIME, ftmaximum: super::super::Foundation::FILETIME, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilGetLongProperty(ploutvalue: *mut i32, pvaluestruct: *const CLUSPROP_LONG, loldvalue: i32, lminimum: i32, lmaximum: i32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetMultiSzProperty(ppszoutvalue: *mut super::super::Foundation::PWSTR, pcboutvaluesize: *mut u32, pvaluestruct: *const CLUSPROP_SZ, pszoldvalue: super::super::Foundation::PWSTR, cboldvaluesize: u32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn ResUtilGetPrivateProperties(hkeyclusterkey: super::super::System::Registry::HKEY, poutpropertylist: *mut ::core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilGetProperties(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut ::core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilGetPropertiesToParameterBlock(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutparams: *mut u8, bcheckforrequiredproperties: super::super::Foundation::BOOL, psznameofpropinerror: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilGetProperty(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytableitem: *const RESUTIL_PROPERTY_ITEM, poutpropertyitem: *mut *mut ::core::ffi::c_void, pcboutpropertyitemsize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetPropertyFormats(ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertyformatlist: *mut ::core::ffi::c_void, cbpropertyformatlistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilGetPropertySize(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytableitem: *const RESUTIL_PROPERTY_ITEM, pcboutpropertylistsize: *mut u32, pnpropertycount: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilGetQwordValue(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, pqwoutvalue: *mut u64, qwdefaultvalue: u64) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceDependency(hself: super::super::Foundation::HANDLE, lpszresourcetype: super::super::Foundation::PWSTR) -> *mut _HRESOURCE;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceDependencyByClass(hcluster: *mut _HCLUSTER, hself: super::super::Foundation::HANDLE, prci: *mut CLUS_RESOURCE_CLASS_INFO, brecurse: super::super::Foundation::BOOL) -> *mut _HRESOURCE;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceDependencyByClassEx(hcluster: *mut _HCLUSTER, hself: super::super::Foundation::HANDLE, prci: *mut CLUS_RESOURCE_CLASS_INFO, brecurse: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> *mut _HRESOURCE;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceDependencyByName(hcluster: *mut _HCLUSTER, hself: super::super::Foundation::HANDLE, lpszresourcetype: super::super::Foundation::PWSTR, brecurse: super::super::Foundation::BOOL) -> *mut _HRESOURCE;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceDependencyByNameEx(hcluster: *mut _HCLUSTER, hself: super::super::Foundation::HANDLE, lpszresourcetype: super::super::Foundation::PWSTR, brecurse: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> *mut _HRESOURCE;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceDependencyEx(hself: super::super::Foundation::HANDLE, lpszresourcetype: super::super::Foundation::PWSTR, dwdesiredaccess: u32) -> *mut _HRESOURCE;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceDependentIPAddressProps(hresource: *const _HRESOURCE, pszaddress: super::super::Foundation::PWSTR, pcchaddress: *mut u32, pszsubnetmask: super::super::Foundation::PWSTR, pcchsubnetmask: *mut u32, psznetwork: super::super::Foundation::PWSTR, pcchnetwork: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceName(hresource: *const _HRESOURCE, pszresourcename: super::super::Foundation::PWSTR, pcchresourcenameinout: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceNameDependency(lpszresourcename: super::super::Foundation::PWSTR, lpszresourcetype: super::super::Foundation::PWSTR) -> *mut _HRESOURCE;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceNameDependencyEx(lpszresourcename: super::super::Foundation::PWSTR, lpszresourcetype: super::super::Foundation::PWSTR, dwdesiredaccess: u32) -> *mut _HRESOURCE;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetSzProperty(ppszoutvalue: *mut super::super::Foundation::PWSTR, pvaluestruct: *const CLUSPROP_SZ, pszoldvalue: super::super::Foundation::PWSTR, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilGetSzValue(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGroupsEqual(hself: *mut _HGROUP, hgroup: *mut _HGROUP, pequal: *mut super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilIsPathValid(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilIsResourceClassEqual(prci: *mut CLUS_RESOURCE_CLASS_INFO, hresource: *mut _HRESOURCE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilLeftPaxosIsLessThanRight(left: *const PaxosTagCStruct, right: *const PaxosTagCStruct) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilNodeEnum(hcluster: *mut _HCLUSTER, pnodecallback: LPNODE_CALLBACK, pparameter: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilPaxosComparer(left: *const PaxosTagCStruct, right: *const PaxosTagCStruct) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilPropertyListFromParameterBlock(ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut ::core::ffi::c_void, pcboutpropertylistsize: *mut u32, pinparams: *const u8, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilRemoveResourceServiceEnvironment(pszservicename: super::super::Foundation::PWSTR, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilResourceDepEnum(hself: *mut _HRESOURCE, enumtype: u32, prescallback: LPRESOURCE_CALLBACK_EX, pparameter: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilResourceTypesEqual(lpszresourcetypename: super::super::Foundation::PWSTR, hresource: *mut _HRESOURCE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilResourcesEqual(hself: *mut _HRESOURCE, hresource: *mut _HRESOURCE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetBinaryValue(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, pbnewvalue: *const u8, cbnewvaluesize: u32, ppboutvalue: *mut *mut u8, pcboutvaluesize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetDwordValue(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, dwnewvalue: u32, pdwoutvalue: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetExpandSzValue(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, psznewvalue: super::super::Foundation::PWSTR, ppszoutstring: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetMultiSzValue(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, psznewvalue: super::super::Foundation::PWSTR, cbnewvaluesize: u32, ppszoutvalue: *mut super::super::Foundation::PWSTR, pcboutvaluesize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn ResUtilSetPrivatePropertyList(hkeyclusterkey: super::super::System::Registry::HKEY, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetPropertyParameterBlock(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, pinparams: *const u8, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, poutparams: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetPropertyParameterBlockEx(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, pinparams: *const u8, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, bforcewrite: super::super::Foundation::BOOL, poutparams: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetPropertyTable(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, ballowunknownproperties: super::super::Foundation::BOOL, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, poutparams: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetPropertyTableEx(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, ballowunknownproperties: super::super::Foundation::BOOL, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, bforcewrite: super::super::Foundation::BOOL, poutparams: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetQwordValue(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, qwnewvalue: u64, pqwoutvalue: *mut u64) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilSetResourceServiceEnvironment(pszservicename: super::super::Foundation::PWSTR, hresource: *mut _HRESOURCE, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn ResUtilSetResourceServiceStartParameters(pszservicename: super::super::Foundation::PWSTR, schscmhandle: super::super::Security::SC_HANDLE, phservice: *mut isize, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn ResUtilSetResourceServiceStartParametersEx(pszservicename: super::super::Foundation::PWSTR, schscmhandle: super::super::Security::SC_HANDLE, phservice: *mut isize, dwdesiredaccess: u32, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetSzValue(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, psznewvalue: super::super::Foundation::PWSTR, ppszoutstring: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetUnknownProperties(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetValueEx(hkeyclusterkey: super::super::System::Registry::HKEY, valuename: super::super::Foundation::PWSTR, valuetype: u32, valuedata: *const u8, valuesize: u32, flags: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilStartResourceService(pszservicename: super::super::Foundation::PWSTR, phservicehandle: *mut isize) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilStopResourceService(pszservicename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Security`*"]
    #[cfg(feature = "Win32_Security")]
    pub fn ResUtilStopService(hservicehandle: super::super::Security::SC_HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilTerminateServiceProcessFromResDll(dwservicepid: u32, boffline: super::super::Foundation::BOOL, pdwresourcestate: *mut u32, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilVerifyPrivatePropertyList(pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilVerifyPropertyTable(ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, ballowunknownproperties: super::super::Foundation::BOOL, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, poutparams: *mut u8) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilVerifyResourceService(pszservicename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Security`*"]
    #[cfg(feature = "Win32_Security")]
    pub fn ResUtilVerifyService(hservicehandle: super::super::Security::SC_HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilVerifyShutdownSafe(flags: u32, reason: u32, presult: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilsDeleteKeyTree(key: super::super::System::Registry::HKEY, keyname: super::super::Foundation::PWSTR, treatnokeyaserror: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResetAllAppInstanceVersions() -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn RestartClusterResource(hresource: *const _HRESOURCE, dwflags: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RestoreClusterDatabase(lpszpathname: super::super::Foundation::PWSTR, bforce: super::super::Foundation::BOOL, lpszquorumdriveletter: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResumeClusterNode(hnode: *const _HNODE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResumeClusterNodeEx(hnode: *const _HNODE, eresumefailbacktype: CLUSTER_NODE_RESUME_FAILBACK_TYPE, dwresumeflagsreserved: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetAppInstanceCsvFlags(processhandle: super::super::Foundation::HANDLE, mask: u32, flags: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClusterGroupName(hgroup: *const _HGROUP, lpszgroupname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn SetClusterGroupNodeList(hgroup: *const _HGROUP, nodecount: u32, nodelist: *const *const _HNODE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClusterGroupSetDependencyExpression(hgroupset: *const _HGROUPSET, lpszdependencyexprssion: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClusterName(hcluster: *const _HCLUSTER, lpsznewclustername: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClusterNetworkName(hnetwork: *const _HNETWORK, lpszname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn SetClusterNetworkPriorityOrder(hcluster: *const _HCLUSTER, networkcount: u32, networklist: *const *const _HNETWORK) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClusterQuorumResource(hresource: *const _HRESOURCE, lpszdevicename: super::super::Foundation::PWSTR, dwmaxquologsize: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClusterResourceDependencyExpression(hresource: *const _HRESOURCE, lpszdependencyexpression: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClusterResourceName(hresource: *const _HRESOURCE, lpszresourcename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClusterServiceAccountPassword(lpszclustername: super::super::Foundation::PWSTR, lpsznewpassword: super::super::Foundation::PWSTR, dwflags: u32, lpreturnstatusbuffer: *mut CLUSTER_SET_PASSWORD_STATUS, lpcbreturnstatusbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetGroupDependencyExpression(hgroup: *const _HGROUP, lpszdependencyexpression: super::super::Foundation::PWSTR) -> u32;
}
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const BitLockerDecrypted: i32 = 4i32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const BitLockerDecrypting: i32 = 16i32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const BitLockerEnabled: i32 = 1i32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const BitLockerPaused: i32 = 64i32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const BitLockerStopped: i32 = 128i32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const BitlockerEncrypted: i32 = 8i32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const BitlockerEncrypting: i32 = 32i32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CA_UPGRADE_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLCTL_CLUSTER_BASE: u32 = 0u32;
pub struct CLCTL_CODES(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLCTL_GLOBAL_SHIFT: u32 = 23u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLCTL_INTERNAL_SHIFT: u32 = 20u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLCTL_MODIFY_SHIFT: u32 = 22u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLCTL_USER_SHIFT: u32 = 21u32;
pub struct CLRES_CALLBACK_FUNCTION_TABLE(i32);
pub struct CLRES_FUNCTION_TABLE(i32);
pub struct CLRES_V1_FUNCTIONS(i32);
pub struct CLRES_V2_FUNCTIONS(i32);
pub struct CLRES_V3_FUNCTIONS(i32);
pub struct CLRES_V4_FUNCTIONS(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLRES_VERSION_V1_00: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLRES_VERSION_V2_00: u32 = 512u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLRES_VERSION_V3_00: u32 = 768u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLRES_VERSION_V4_00: u32 = 1024u32;
pub struct CLUADMEX_OBJECT_TYPE(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_CHANGE_ACCESS: i32 = 2i32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_CHANGE_RESOURCE_GROUP_FORCE_MOVE_TO_CSV: u64 = 1u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_GROUP_MOVE_FAILBACK: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_GROUP_MOVE_HIGH_PRIORITY_START: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_GROUP_MOVE_IGNORE_AFFINITY_RULE: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_GROUP_MOVE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_GROUP_MOVE_QUEUE_ENABLED: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_GROUP_MOVE_RETURN_TO_SOURCE_NODE_ON_ERROR: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_GROUP_OFFLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_GROUP_ONLINE_BEST_POSSIBLE_NODE: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_GROUP_ONLINE_IGNORE_AFFINITY_RULE: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_GROUP_ONLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_GROUP_ONLINE_SYNCHRONOUS: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_NODE_AVOID_PLACEMENT: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_NODE_PAUSE_REMAIN_ON_PAUSED_NODE_ON_MOVE_ERROR: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_NODE_PAUSE_RETRY_DRAIN_ON_FAILURE: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_NO_ACCESS: i32 = 4i32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_READ_ACCESS: i32 = 1i32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_DO_NOT_UPDATE_PERSISTENT_STATE: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_FORCE_WITH_TERMINATION: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_BEING_DELETED: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_BEING_RESTARTED: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_MOVING: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_PREEMPTED: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_SHUTTING_DOWN: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_UNKNOWN: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_USER_REQUESTED: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_RESOURCE_ONLINE_BEST_POSSIBLE_NODE: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_RESOURCE_ONLINE_DO_NOT_UPDATE_PERSISTENT_STATE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_RESOURCE_ONLINE_IGNORE_AFFINITY_RULE: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_RESOURCE_ONLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_RESOURCE_ONLINE_NECESSARY_FOR_QUORUM: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_VALID_CHANGE_RESOURCE_GROUP_FLAGS: u64 = 1u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_VERSION: u32 = 2560u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_VERSION_RS3: u32 = 2560u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_VERSION_SERVER2008: u32 = 1536u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_VERSION_SERVER2008R2: u32 = 1792u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_VERSION_WINDOWS8: u32 = 1793u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_VERSION_WINDOWSBLUE: u32 = 1794u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSAPI_VERSION_WINTHRESHOLD: u32 = 1795u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSCTL_ACCESS_MODE_MASK: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSCTL_ACCESS_SHIFT: u32 = 0u32;
pub struct CLUSCTL_AFFINITYRULE_CODES(i32);
pub struct CLUSCTL_CLUSTER_CODES(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSCTL_CONTROL_CODE_MASK: u32 = 4194303u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSCTL_FUNCTION_SHIFT: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSCTL_GET_OPERATION_CONTEXT_PARAMS_VERSION_1: u32 = 1u32;
pub struct CLUSCTL_GROUPSET_CODES(i32);
pub struct CLUSCTL_GROUP_CODES(i32);
pub struct CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT(i32);
pub struct CLUSCTL_NETINTERFACE_CODES(i32);
pub struct CLUSCTL_NETWORK_CODES(i32);
pub struct CLUSCTL_NODE_CODES(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSCTL_OBJECT_MASK: u32 = 255u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSCTL_OBJECT_SHIFT: u32 = 24u32;
pub struct CLUSCTL_RESOURCE_CODES(i32);
pub struct CLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSCTL_RESOURCE_STATE_CHANGE_REASON_VERSION_1: u32 = 1u32;
pub struct CLUSCTL_RESOURCE_TYPE_CODES(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_FLAG_ADD_VOLUME_INFO: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_FLAG_FILTER_BY_POOL: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_FLAG_INCLUDE_NON_SHARED_DISKS: u32 = 4u32;
pub struct CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSGROUPSET_STATUS_APPLICATION_READY: u64 = 8u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSGROUPSET_STATUS_GROUPS_ONLINE: u64 = 2u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSGROUPSET_STATUS_GROUPS_PENDING: u64 = 1u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSGROUPSET_STATUS_OS_HEARTBEAT: u64 = 4u64;
pub struct CLUSGROUP_TYPE(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSGRP_STATUS_APPLICATION_READY: u64 = 1024u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSGRP_STATUS_EMBEDDED_FAILURE: u64 = 32u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSGRP_STATUS_LOCKED_MODE: u64 = 1u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSGRP_STATUS_NETWORK_FAILURE: u64 = 128u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSGRP_STATUS_OFFLINE_DUE_TO_ANTIAFFINITY_CONFLICT: u64 = 64u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSGRP_STATUS_OFFLINE_NOT_LOCAL_DISK_OWNER: u64 = 2048u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSGRP_STATUS_OS_HEARTBEAT: u64 = 512u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSGRP_STATUS_PHYSICAL_RESOURCES_LACKING: u64 = 8u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSGRP_STATUS_PREEMPTED: u64 = 2u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSGRP_STATUS_UNMONITORED: u64 = 256u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSGRP_STATUS_WAITING_FOR_DEPENDENCIES: u64 = 4096u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSGRP_STATUS_WAITING_IN_QUEUE_FOR_MOVE: u64 = 4u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSGRP_STATUS_WAITING_TO_START: u64 = 16u64;
pub struct CLUSPROP_BINARY(i32);
pub struct CLUSPROP_BUFFER_HELPER(i32);
pub struct CLUSPROP_DWORD(i32);
pub struct CLUSPROP_FILETIME(i32);
pub struct CLUSPROP_FTSET_INFO(i32);
pub struct CLUSPROP_IPADDR_ENABLENETBIOS(i32);
pub struct CLUSPROP_LARGE_INTEGER(i32);
pub struct CLUSPROP_LIST(i32);
pub struct CLUSPROP_LONG(i32);
pub struct CLUSPROP_PARTITION_INFO(i32);
pub struct CLUSPROP_PARTITION_INFO_EX(i32);
pub struct CLUSPROP_PARTITION_INFO_EX2(i32);
pub struct CLUSPROP_PIFLAGS(i32);
pub struct CLUSPROP_REQUIRED_DEPENDENCY(i32);
pub struct CLUSPROP_RESOURCE_CLASS(i32);
pub struct CLUSPROP_RESOURCE_CLASS_INFO(i32);
pub struct CLUSPROP_SCSI_ADDRESS(i32);
pub struct CLUSPROP_SECURITY_DESCRIPTOR(i32);
pub struct CLUSPROP_SYNTAX(i32);
pub struct CLUSPROP_SZ(i32);
pub struct CLUSPROP_ULARGE_INTEGER(i32);
pub struct CLUSPROP_VALUE(i32);
pub struct CLUSPROP_WORD(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSREG_DATABASE_ISOLATE_READ: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSREG_DATABASE_SYNC_WRITE_TO_ALL_NODES: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSRESDLL_STATUS_DO_NOT_COLLECT_WER_REPORT: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSRESDLL_STATUS_DUMP_NOW: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSRESDLL_STATUS_INSUFFICIENT_MEMORY: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSRESDLL_STATUS_INSUFFICIENT_OTHER_RESOURCES: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSRESDLL_STATUS_INSUFFICIENT_PROCESSOR: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSRESDLL_STATUS_INVALID_PARAMETERS: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSRESDLL_STATUS_NETWORK_NOT_AVAILABLE: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSRESDLL_STATUS_OFFLINE_BUSY: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSRESDLL_STATUS_OFFLINE_DESTINATION_REJECTED: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSRESDLL_STATUS_OFFLINE_DESTINATION_THROTTLED: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSRESDLL_STATUS_OFFLINE_SOURCE_THROTTLED: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSRES_STATUS_APPLICATION_READY: u64 = 256u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSRES_STATUS_EMBEDDED_FAILURE: u64 = 2u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSRES_STATUS_FAILED_DUE_TO_INSUFFICIENT_CPU: u64 = 4u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSRES_STATUS_FAILED_DUE_TO_INSUFFICIENT_GENERIC_RESOURCES: u64 = 16u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSRES_STATUS_FAILED_DUE_TO_INSUFFICIENT_MEMORY: u64 = 8u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSRES_STATUS_LOCKED_MODE: u64 = 1u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSRES_STATUS_NETWORK_FAILURE: u64 = 32u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSRES_STATUS_OFFLINE_NOT_LOCAL_DISK_OWNER: u64 = 512u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSRES_STATUS_OS_HEARTBEAT: u64 = 128u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSRES_STATUS_UNMONITORED: u64 = 64u64;
pub struct CLUSTERSET_OBJECT_TYPE(i32);
pub struct CLUSTERVERSIONINFO(i32);
pub struct CLUSTERVERSIONINFO_NT4(i32);
pub struct CLUSTER_AVAILABILITY_SET_CONFIG(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSTER_AVAILABILITY_SET_CONFIG_V1: u32 = 1u32;
pub struct CLUSTER_BATCH_COMMAND(i32);
pub struct CLUSTER_CHANGE(i32);
pub struct CLUSTER_CHANGE_CLUSTER_V2(i32);
pub struct CLUSTER_CHANGE_GROUPSET_V2(i32);
pub struct CLUSTER_CHANGE_GROUP_V2(i32);
pub struct CLUSTER_CHANGE_NETINTERFACE_V2(i32);
pub struct CLUSTER_CHANGE_NETWORK_V2(i32);
pub struct CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2(i32);
pub struct CLUSTER_CHANGE_NODE_V2(i32);
pub struct CLUSTER_CHANGE_QUORUM_V2(i32);
pub struct CLUSTER_CHANGE_REGISTRY_V2(i32);
pub struct CLUSTER_CHANGE_RESOURCE_TYPE_V2(i32);
pub struct CLUSTER_CHANGE_RESOURCE_V2(i32);
pub struct CLUSTER_CHANGE_SHARED_VOLUME_V2(i32);
pub struct CLUSTER_CHANGE_SPACEPORT_V2(i32);
pub struct CLUSTER_CLOUD_TYPE(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSTER_CONFIGURED: u32 = 2u32;
pub struct CLUSTER_CONTROL_OBJECT(i32);
pub struct CLUSTER_CREATE_GROUP_INFO(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSTER_CREATE_GROUP_INFO_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSTER_CREATE_GROUP_INFO_VERSION_1: u32 = 1u32;
pub struct CLUSTER_CSV_VOLUME_FAULT_STATE(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSTER_DELETE_ACCESS_CONTROL_ENTRY: u32 = 2u32;
pub struct CLUSTER_ENUM(i32);
pub struct CLUSTER_ENUM_ITEM(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSTER_ENUM_ITEM_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSTER_ENUM_ITEM_VERSION_1: u32 = 1u32;
pub struct CLUSTER_GROUP_AUTOFAILBACK_TYPE(i32);
pub struct CLUSTER_GROUP_ENUM(i32);
pub struct CLUSTER_GROUP_ENUM_ITEM(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSTER_GROUP_ENUM_ITEM_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSTER_GROUP_ENUM_ITEM_VERSION_1: u32 = 1u32;
pub struct CLUSTER_GROUP_PRIORITY(i32);
pub struct CLUSTER_GROUP_STATE(i32);
pub struct CLUSTER_HEALTH_FAULT(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSTER_HEALTH_FAULT_ARGS: u32 = 7u32;
pub struct CLUSTER_HEALTH_FAULT_ARRAY(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSTER_HEALTH_FAULT_DESCRIPTION: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSTER_HEALTH_FAULT_ERRORCODE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSTER_HEALTH_FAULT_ERRORTYPE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSTER_HEALTH_FAULT_FLAGS: u32 = 5u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSTER_HEALTH_FAULT_ID: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSTER_HEALTH_FAULT_PROVIDER: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSTER_HEALTH_FAULT_RESERVED: u32 = 6u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSTER_INSTALLED: u32 = 1u32;
pub struct CLUSTER_IP_ENTRY(i32);
pub struct CLUSTER_MEMBERSHIP_INFO(i32);
pub struct CLUSTER_MGMT_POINT_RESTYPE(i32);
pub struct CLUSTER_MGMT_POINT_TYPE(i32);
pub struct CLUSTER_NETINTERFACE_STATE(i32);
pub struct CLUSTER_NETWORK_ENUM(i32);
pub struct CLUSTER_NETWORK_ROLE(i32);
pub struct CLUSTER_NETWORK_STATE(i32);
pub struct CLUSTER_NODE_DRAIN_STATUS(i32);
pub struct CLUSTER_NODE_ENUM(i32);
pub struct CLUSTER_NODE_RESUME_FAILBACK_TYPE(i32);
pub struct CLUSTER_NODE_STATE(i32);
pub struct CLUSTER_NODE_STATUS(i32);
pub struct CLUSTER_NOTIFICATIONS_VERSION(i32);
pub struct CLUSTER_OBJECT_TYPE(i32);
pub struct CLUSTER_PROPERTY_FORMAT(i32);
pub struct CLUSTER_PROPERTY_SYNTAX(i32);
pub struct CLUSTER_PROPERTY_TYPE(i32);
pub struct CLUSTER_QUORUM_TYPE(i32);
pub struct CLUSTER_QUORUM_VALUE(i32);
pub struct CLUSTER_READ_BATCH_COMMAND(i32);
pub struct CLUSTER_REG_COMMAND(i32);
pub struct CLUSTER_RESOURCE_APPLICATION_STATE(i32);
pub struct CLUSTER_RESOURCE_CLASS(i32);
pub struct CLUSTER_RESOURCE_CREATE_FLAGS(i32);
pub struct CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION(i32);
pub struct CLUSTER_RESOURCE_ENUM(i32);
pub struct CLUSTER_RESOURCE_ENUM_ITEM(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSTER_RESOURCE_ENUM_ITEM_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSTER_RESOURCE_ENUM_ITEM_VERSION_1: u32 = 1u32;
pub struct CLUSTER_RESOURCE_RESTART_ACTION(i32);
pub struct CLUSTER_RESOURCE_STATE(i32);
pub struct CLUSTER_RESOURCE_STATE_CHANGE_REASON(i32);
pub struct CLUSTER_RESOURCE_TYPE_ENUM(i32);
pub struct CLUSTER_ROLE(i32);
pub struct CLUSTER_ROLE_STATE(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSTER_RUNNING: u32 = 16u32;
pub struct CLUSTER_SETUP_PHASE(i32);
pub struct CLUSTER_SETUP_PHASE_SEVERITY(i32);
pub struct CLUSTER_SETUP_PHASE_TYPE(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSTER_SET_ACCESS_TYPE_ALLOWED: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSTER_SET_ACCESS_TYPE_DENIED: u32 = 1u32;
pub struct CLUSTER_SET_PASSWORD_STATUS(i32);
pub struct CLUSTER_SHARED_VOLUME_BACKUP_STATE(i32);
pub struct CLUSTER_SHARED_VOLUME_RENAME_GUID_INPUT(i32);
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT(i32);
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME(i32);
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME(i32);
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE(i32);
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME(i32);
pub struct CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE(i32);
pub struct CLUSTER_SHARED_VOLUME_STATE(i32);
pub struct CLUSTER_SHARED_VOLUME_STATE_INFO(i32);
pub struct CLUSTER_SHARED_VOLUME_STATE_INFO_EX(i32);
pub struct CLUSTER_STORAGENODE_STATE(i32);
pub struct CLUSTER_UPGRADE_PHASE(i32);
pub struct CLUSTER_VALIDATE_CSV_FILENAME(i32);
pub struct CLUSTER_VALIDATE_DIRECTORY(i32);
pub struct CLUSTER_VALIDATE_NETNAME(i32);
pub struct CLUSTER_VALIDATE_PATH(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSTER_VERSION_FLAG_MIXED_MODE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUSTER_VERSION_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUS_ACCESS_ANY: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUS_ACCESS_READ: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUS_ACCESS_WRITE: u32 = 2u32;
pub struct CLUS_AFFINITY_RULE_TYPE(i32);
pub struct CLUS_CHARACTERISTICS(i32);
pub struct CLUS_CHKDSK_INFO(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUS_CREATE_CRYPT_CONTAINER_NOT_FOUND: u32 = 1u32;
pub struct CLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT(i32);
pub struct CLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT(i32);
pub struct CLUS_CSV_MAINTENANCE_MODE_INFO(i32);
pub struct CLUS_CSV_VOLUME_INFO(i32);
pub struct CLUS_CSV_VOLUME_NAME(i32);
pub struct CLUS_DISK_NUMBER_INFO(i32);
pub struct CLUS_DNN_LEADER_STATUS(i32);
pub struct CLUS_DNN_SODAFS_CLONE_STATUS(i32);
pub struct CLUS_FLAGS(i32);
pub struct CLUS_FORCE_QUORUM_INFO(i32);
pub struct CLUS_FTSET_INFO(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUS_GLOBAL: u32 = 1u32;
pub struct CLUS_GROUP_START_SETTING(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUS_GRP_MOVE_ALLOWED: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUS_GRP_MOVE_LOCKED: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUS_HYBRID_QUORUM: u32 = 1024u32;
pub struct CLUS_MAINTENANCE_MODE_INFO(i32);
pub struct CLUS_MAINTENANCE_MODE_INFOEX(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUS_MODIFY: u32 = 1u32;
pub struct CLUS_NETNAME_IP_INFO_ENTRY(i32);
pub struct CLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL(i32);
pub struct CLUS_NETNAME_PWD_INFO(i32);
pub struct CLUS_NETNAME_PWD_INFOEX(i32);
pub struct CLUS_NETNAME_VS_TOKEN_INFO(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUS_NODE_MAJORITY_QUORUM: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUS_NOT_GLOBAL: u32 = 0u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUS_NO_MODIFY: u32 = 0u32;
pub struct CLUS_PARTITION_INFO(i32);
pub struct CLUS_PARTITION_INFO_EX(i32);
pub struct CLUS_PARTITION_INFO_EX2(i32);
pub struct CLUS_PROVIDER_STATE_CHANGE_INFO(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUS_RESDLL_OFFLINE_DO_NOT_UPDATE_PERSISTENT_STATE: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUS_RESDLL_OFFLINE_DUE_TO_EMBEDDED_FAILURE: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUS_RESDLL_OFFLINE_IGNORE_NETWORK_CONNECTIVITY: u32 = 32u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUS_RESDLL_OFFLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUS_RESDLL_OFFLINE_QUEUE_ENABLED: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUS_RESDLL_OFFLINE_RETURNING_TO_SOURCE_NODE_BECAUSE_OF_ERROR: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUS_RESDLL_OFFLINE_RETURN_TO_SOURCE_NODE_ON_ERROR: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUS_RESDLL_ONLINE_IGNORE_NETWORK_CONNECTIVITY: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUS_RESDLL_ONLINE_IGNORE_RESOURCE_STATUS: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUS_RESDLL_ONLINE_RECOVER_MONITOR_STATE: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUS_RESDLL_ONLINE_RESTORE_ONLINE_STATE: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUS_RESDLL_ONLINE_RETURN_TO_SOURCE_NODE_ON_ERROR: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUS_RESDLL_OPEN_DONT_DELETE_TEMP_DISK: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CLUS_RESDLL_OPEN_RECOVER_MONITOR_STATE: u32 = 1u32;
pub struct CLUS_RESOURCE_CLASS_INFO(i32);
pub struct CLUS_RESSUBCLASS(i32);
pub struct CLUS_RESSUBCLASS_NETWORK(i32);
pub struct CLUS_RESSUBCLASS_STORAGE(i32);
pub struct CLUS_SCSI_ADDRESS(i32);
pub struct CLUS_SET_MAINTENANCE_MODE_INPUT(i32);
pub struct CLUS_SHARED_VOLUME_BACKUP_MODE(i32);
pub struct CLUS_STARTING_PARAMS(i32);
pub struct CLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS(i32);
pub struct CLUS_STORAGE_REMAP_DRIVELETTER(i32);
pub struct CLUS_STORAGE_SET_DRIVELETTER(i32);
pub struct CLUS_WORKER(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CREATEDC_PRESENT: u32 = 2u32;
pub struct CREATE_CLUSTER_CONFIG(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CREATE_CLUSTER_MAJOR_VERSION_MASK: u32 = 4294967040u32;
pub struct CREATE_CLUSTER_NAME_ACCOUNT(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const CREATE_CLUSTER_VERSION: u32 = 1536u32;
pub struct ClusApplication(i32);
pub struct ClusCryptoKeys(i32);
pub struct ClusDisk(i32);
pub struct ClusDisks(i32);
pub struct ClusNetInterface(i32);
pub struct ClusNetInterfaces(i32);
pub struct ClusNetwork(i32);
pub struct ClusNetworkNetInterfaces(i32);
pub struct ClusNetworks(i32);
pub struct ClusNode(i32);
pub struct ClusNodeNetInterfaces(i32);
pub struct ClusNodes(i32);
pub struct ClusPartition(i32);
pub struct ClusPartitionEx(i32);
pub struct ClusPartitions(i32);
pub struct ClusProperties(i32);
pub struct ClusProperty(i32);
pub struct ClusPropertyValue(i32);
pub struct ClusPropertyValueData(i32);
pub struct ClusPropertyValues(i32);
pub struct ClusRefObject(i32);
pub struct ClusRegistryKeys(i32);
pub struct ClusResDependencies(i32);
pub struct ClusResDependents(i32);
pub struct ClusResGroup(i32);
pub struct ClusResGroupPreferredOwnerNodes(i32);
pub struct ClusResGroupResources(i32);
pub struct ClusResGroups(i32);
pub struct ClusResPossibleOwnerNodes(i32);
pub struct ClusResType(i32);
pub struct ClusResTypePossibleOwnerNodes(i32);
pub struct ClusResTypeResources(i32);
pub struct ClusResTypes(i32);
pub struct ClusResource(i32);
pub struct ClusResources(i32);
pub struct ClusScsiAddress(i32);
pub struct ClusVersion(i32);
pub struct Cluster(i32);
pub struct ClusterNames(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const DNS_LENGTH: u32 = 64u32;
pub struct DomainNames(i32);
pub struct FAILURE_TYPE(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const FE_UPGRADE_VERSION: u32 = 4u32;
pub struct FILESHARE_CHANGE(i32);
pub struct FILESHARE_CHANGE_ENUM(i32);
pub struct FILESHARE_CHANGE_LIST(i32);
pub struct GET_OPERATION_CONTEXT_PARAMS(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const GROUPSET_READY_SETTING_APPLICATION_READY: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const GROUPSET_READY_SETTING_DELAY: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const GROUPSET_READY_SETTING_ONLINE: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const GROUPSET_READY_SETTING_OS_HEARTBEAT: u32 = 3u32;
pub struct GROUP_FAILURE_INFO(i32);
pub struct GROUP_FAILURE_INFO_BUFFER(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const GROUP_FAILURE_INFO_VERSION_1: u32 = 1u32;
pub struct GRP_PLACEMENT_OPTIONS(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const GUID_PRESENT: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const HCI_UPGRADE_BIT: u32 = 32768u32;
pub struct IGetClusterDataInfo(i32);
pub struct IGetClusterGroupInfo(i32);
pub struct IGetClusterNetInterfaceInfo(i32);
pub struct IGetClusterNetworkInfo(i32);
pub struct IGetClusterNodeInfo(i32);
pub struct IGetClusterObjectInfo(i32);
pub struct IGetClusterResourceInfo(i32);
pub struct IGetClusterUIInfo(i32);
pub struct ISClusApplication(i32);
pub struct ISClusCryptoKeys(i32);
pub struct ISClusDisk(i32);
pub struct ISClusDisks(i32);
pub struct ISClusNetInterface(i32);
pub struct ISClusNetInterfaces(i32);
pub struct ISClusNetwork(i32);
pub struct ISClusNetworkNetInterfaces(i32);
pub struct ISClusNetworks(i32);
pub struct ISClusNode(i32);
pub struct ISClusNodeNetInterfaces(i32);
pub struct ISClusNodes(i32);
pub struct ISClusPartition(i32);
pub struct ISClusPartitionEx(i32);
pub struct ISClusPartitions(i32);
pub struct ISClusProperties(i32);
pub struct ISClusProperty(i32);
pub struct ISClusPropertyValue(i32);
pub struct ISClusPropertyValueData(i32);
pub struct ISClusPropertyValues(i32);
pub struct ISClusRefObject(i32);
pub struct ISClusRegistryKeys(i32);
pub struct ISClusResDependencies(i32);
pub struct ISClusResDependents(i32);
pub struct ISClusResGroup(i32);
pub struct ISClusResGroupPreferredOwnerNodes(i32);
pub struct ISClusResGroupResources(i32);
pub struct ISClusResGroups(i32);
pub struct ISClusResPossibleOwnerNodes(i32);
pub struct ISClusResType(i32);
pub struct ISClusResTypePossibleOwnerNodes(i32);
pub struct ISClusResTypeResources(i32);
pub struct ISClusResTypes(i32);
pub struct ISClusResource(i32);
pub struct ISClusResources(i32);
pub struct ISClusScsiAddress(i32);
pub struct ISClusVersion(i32);
pub struct ISCluster(i32);
pub struct ISClusterNames(i32);
pub struct ISDomainNames(i32);
pub struct IWCContextMenuCallback(i32);
pub struct IWCPropertySheetCallback(i32);
pub struct IWCWizard97Callback(i32);
pub struct IWCWizardCallback(i32);
pub struct IWEExtendContextMenu(i32);
pub struct IWEExtendPropertySheet(i32);
pub struct IWEExtendWizard(i32);
pub struct IWEExtendWizard97(i32);
pub struct IWEInvokeCommand(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const LOCKED_MODE_FLAGS_DONT_REMOVE_FROM_MOVE_QUEUE: u32 = 1u32;
pub struct LOG_LEVEL(i32);
pub struct LPGROUP_CALLBACK_EX(i32);
pub struct LPNODE_CALLBACK(i32);
pub struct LPRESOURCE_CALLBACK(i32);
pub struct LPRESOURCE_CALLBACK_EX(i32);
pub struct MAINTENANCE_MODE_TYPE_ENUM(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const MAINTENANCE_MODE_V2_SIG: u32 = 2881155087u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const MAX_CLUSTERNAME_LENGTH: u32 = 63u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const MAX_CO_PASSWORD_LENGTH: u32 = 16u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const MAX_CO_PASSWORD_LENGTHEX: u32 = 127u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const MAX_CO_PASSWORD_STORAGEEX: u32 = 128u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const MAX_CREATINGDC_LENGTH: u32 = 256u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const MAX_OBJECTID: u32 = 64u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const MN_UPGRADE_VERSION: u32 = 3u32;
pub struct MONITOR_STATE(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const NINETEEN_H1_UPGRADE_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const NINETEEN_H2_UPGRADE_VERSION: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const NNLEN: u32 = 80u32;
pub struct NODE_CLUSTER_STATE(i32);
pub struct NOTIFY_FILTER_AND_TYPE(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const NT10_MAJOR_VERSION: u32 = 9u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const NT11_MAJOR_VERSION: u32 = 10u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const NT12_MAJOR_VERSION: u32 = 11u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const NT13_MAJOR_VERSION: u32 = 12u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const NT4SP4_MAJOR_VERSION: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const NT4_MAJOR_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const NT51_MAJOR_VERSION: u32 = 4u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const NT5_MAJOR_VERSION: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const NT6_MAJOR_VERSION: u32 = 5u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const NT7_MAJOR_VERSION: u32 = 6u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const NT8_MAJOR_VERSION: u32 = 7u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const NT9_MAJOR_VERSION: u32 = 8u32;
pub struct NodeUtilizationInfoElement(i32);
pub struct PARBITRATE_ROUTINE(i32);
pub struct PBEGIN_RESCALL_AS_USER_ROUTINE(i32);
pub struct PBEGIN_RESCALL_ROUTINE(i32);
pub struct PBEGIN_RESTYPECALL_AS_USER_ROUTINE(i32);
pub struct PBEGIN_RESTYPECALL_ROUTINE(i32);
pub struct PCANCEL_ROUTINE(i32);
pub struct PCHANGE_RESOURCE_PROCESS_FOR_DUMPS(i32);
pub struct PCHANGE_RES_TYPE_PROCESS_FOR_DUMPS(i32);
pub struct PCLOSE_CLUSTER_CRYPT_PROVIDER(i32);
pub struct PCLOSE_ROUTINE(i32);
pub struct PCLUSAPIClusWorkerCheckTerminate(i32);
pub struct PCLUSAPI_ADD_CLUSTER_GROUP_DEPENDENCY(i32);
pub struct PCLUSAPI_ADD_CLUSTER_GROUP_GROUPSET_DEPENDENCY(i32);
pub struct PCLUSAPI_ADD_CLUSTER_GROUP_TO_GROUP_GROUPSET_DEPENDENCY(i32);
pub struct PCLUSAPI_ADD_CLUSTER_NODE(i32);
pub struct PCLUSAPI_ADD_CLUSTER_NODE_EX(i32);
pub struct PCLUSAPI_ADD_CLUSTER_RESOURCE_DEPENDENCY(i32);
pub struct PCLUSAPI_ADD_CLUSTER_RESOURCE_NODE(i32);
pub struct PCLUSAPI_ADD_CROSS_CLUSTER_GROUPSET_DEPENDENCY(i32);
pub struct PCLUSAPI_ADD_RESOURCE_TO_CLUSTER_SHARED_VOLUMES(i32);
pub struct PCLUSAPI_BACKUP_CLUSTER_DATABASE(i32);
pub struct PCLUSAPI_CAN_RESOURCE_BE_DEPENDENT(i32);
pub struct PCLUSAPI_CHANGE_CLUSTER_RESOURCE_GROUP(i32);
pub struct PCLUSAPI_CHANGE_CLUSTER_RESOURCE_GROUP_EX(i32);
pub struct PCLUSAPI_CLOSE_CLUSTER(i32);
pub struct PCLUSAPI_CLOSE_CLUSTER_GROUP(i32);
pub struct PCLUSAPI_CLOSE_CLUSTER_GROUP_GROUPSET(i32);
pub struct PCLUSAPI_CLOSE_CLUSTER_NETWORK(i32);
pub struct PCLUSAPI_CLOSE_CLUSTER_NET_INTERFACE(i32);
pub struct PCLUSAPI_CLOSE_CLUSTER_NODE(i32);
pub struct PCLUSAPI_CLOSE_CLUSTER_NOTIFY_PORT(i32);
pub struct PCLUSAPI_CLOSE_CLUSTER_RESOURCE(i32);
pub struct PCLUSAPI_CLUSTER_ADD_GROUP_TO_AFFINITY_RULE(i32);
pub struct PCLUSAPI_CLUSTER_ADD_GROUP_TO_GROUP_GROUPSET(i32);
pub struct PCLUSAPI_CLUSTER_AFFINITY_RULE_CONTROL(i32);
pub struct PCLUSAPI_CLUSTER_CLOSE_ENUM(i32);
pub struct PCLUSAPI_CLUSTER_CLOSE_ENUM_EX(i32);
pub struct PCLUSAPI_CLUSTER_CONTROL(i32);
pub struct PCLUSAPI_CLUSTER_CREATE_AFFINITY_RULE(i32);
pub struct PCLUSAPI_CLUSTER_ENUM(i32);
pub struct PCLUSAPI_CLUSTER_ENUM_EX(i32);
pub struct PCLUSAPI_CLUSTER_GET_ENUM_COUNT(i32);
pub struct PCLUSAPI_CLUSTER_GET_ENUM_COUNT_EX(i32);
pub struct PCLUSAPI_CLUSTER_GROUP_CLOSE_ENUM(i32);
pub struct PCLUSAPI_CLUSTER_GROUP_CLOSE_ENUM_EX(i32);
pub struct PCLUSAPI_CLUSTER_GROUP_CONTROL(i32);
pub struct PCLUSAPI_CLUSTER_GROUP_ENUM(i32);
pub struct PCLUSAPI_CLUSTER_GROUP_ENUM_EX(i32);
pub struct PCLUSAPI_CLUSTER_GROUP_GET_ENUM_COUNT(i32);
pub struct PCLUSAPI_CLUSTER_GROUP_GET_ENUM_COUNT_EX(i32);
pub struct PCLUSAPI_CLUSTER_GROUP_GROUPSET_CONTROL(i32);
pub struct PCLUSAPI_CLUSTER_GROUP_OPEN_ENUM(i32);
pub struct PCLUSAPI_CLUSTER_GROUP_OPEN_ENUM_EX(i32);
pub struct PCLUSAPI_CLUSTER_NETWORK_CLOSE_ENUM(i32);
pub struct PCLUSAPI_CLUSTER_NETWORK_CONTROL(i32);
pub struct PCLUSAPI_CLUSTER_NETWORK_ENUM(i32);
pub struct PCLUSAPI_CLUSTER_NETWORK_GET_ENUM_COUNT(i32);
pub struct PCLUSAPI_CLUSTER_NETWORK_OPEN_ENUM(i32);
pub struct PCLUSAPI_CLUSTER_NET_INTERFACE_CONTROL(i32);
pub struct PCLUSAPI_CLUSTER_NODE_CLOSE_ENUM(i32);
pub struct PCLUSAPI_CLUSTER_NODE_CLOSE_ENUM_EX(i32);
pub struct PCLUSAPI_CLUSTER_NODE_CONTROL(i32);
pub struct PCLUSAPI_CLUSTER_NODE_ENUM(i32);
pub struct PCLUSAPI_CLUSTER_NODE_ENUM_EX(i32);
pub struct PCLUSAPI_CLUSTER_NODE_GET_ENUM_COUNT(i32);
pub struct PCLUSAPI_CLUSTER_NODE_GET_ENUM_COUNT_EX(i32);
pub struct PCLUSAPI_CLUSTER_NODE_OPEN_ENUM(i32);
pub struct PCLUSAPI_CLUSTER_NODE_OPEN_ENUM_EX(i32);
pub struct PCLUSAPI_CLUSTER_OPEN_ENUM(i32);
pub struct PCLUSAPI_CLUSTER_OPEN_ENUM_EX(i32);
pub struct PCLUSAPI_CLUSTER_REG_CLOSE_KEY(i32);
pub struct PCLUSAPI_CLUSTER_REG_CREATE_BATCH(i32);
pub struct PCLUSAPI_CLUSTER_REG_CREATE_KEY(i32);
pub struct PCLUSAPI_CLUSTER_REG_DELETE_KEY(i32);
pub struct PCLUSAPI_CLUSTER_REG_DELETE_VALUE(i32);
pub struct PCLUSAPI_CLUSTER_REG_ENUM_KEY(i32);
pub struct PCLUSAPI_CLUSTER_REG_ENUM_VALUE(i32);
pub struct PCLUSAPI_CLUSTER_REG_GET_KEY_SECURITY(i32);
pub struct PCLUSAPI_CLUSTER_REG_OPEN_KEY(i32);
pub struct PCLUSAPI_CLUSTER_REG_QUERY_INFO_KEY(i32);
pub struct PCLUSAPI_CLUSTER_REG_QUERY_VALUE(i32);
pub struct PCLUSAPI_CLUSTER_REG_SET_KEY_SECURITY(i32);
pub struct PCLUSAPI_CLUSTER_REG_SET_VALUE(i32);
pub struct PCLUSAPI_CLUSTER_REG_SYNC_DATABASE(i32);
pub struct PCLUSAPI_CLUSTER_REMOVE_AFFINITY_RULE(i32);
pub struct PCLUSAPI_CLUSTER_REMOVE_GROUP_FROM_AFFINITY_RULE(i32);
pub struct PCLUSAPI_CLUSTER_REMOVE_GROUP_FROM_GROUP_GROUPSET(i32);
pub struct PCLUSAPI_CLUSTER_RESOURCE_CLOSE_ENUM(i32);
pub struct PCLUSAPI_CLUSTER_RESOURCE_CLOSE_ENUM_EX(i32);
pub struct PCLUSAPI_CLUSTER_RESOURCE_CONTROL(i32);
pub struct PCLUSAPI_CLUSTER_RESOURCE_ENUM(i32);
pub struct PCLUSAPI_CLUSTER_RESOURCE_ENUM_EX(i32);
pub struct PCLUSAPI_CLUSTER_RESOURCE_GET_ENUM_COUNT(i32);
pub struct PCLUSAPI_CLUSTER_RESOURCE_GET_ENUM_COUNT_EX(i32);
pub struct PCLUSAPI_CLUSTER_RESOURCE_OPEN_ENUM(i32);
pub struct PCLUSAPI_CLUSTER_RESOURCE_OPEN_ENUM_EX(i32);
pub struct PCLUSAPI_CLUSTER_RESOURCE_TYPE_CLOSE_ENUM(i32);
pub struct PCLUSAPI_CLUSTER_RESOURCE_TYPE_CONTROL(i32);
pub struct PCLUSAPI_CLUSTER_RESOURCE_TYPE_ENUM(i32);
pub struct PCLUSAPI_CLUSTER_RESOURCE_TYPE_GET_ENUM_COUNT(i32);
pub struct PCLUSAPI_CLUSTER_RESOURCE_TYPE_OPEN_ENUM(i32);
pub struct PCLUSAPI_CLUSTER_UPGRADE(i32);
pub struct PCLUSAPI_CLUS_WORKER_CREATE(i32);
pub struct PCLUSAPI_CLUS_WORKER_TERMINATE(i32);
pub struct PCLUSAPI_CREATE_CLUSTER(i32);
pub struct PCLUSAPI_CREATE_CLUSTER_AVAILABILITY_SET(i32);
pub struct PCLUSAPI_CREATE_CLUSTER_CNOLESS(i32);
pub struct PCLUSAPI_CREATE_CLUSTER_GROUP(i32);
pub struct PCLUSAPI_CREATE_CLUSTER_GROUPEX(i32);
pub struct PCLUSAPI_CREATE_CLUSTER_GROUP_GROUPSET(i32);
pub struct PCLUSAPI_CREATE_CLUSTER_NAME_ACCOUNT(i32);
pub struct PCLUSAPI_CREATE_CLUSTER_NOTIFY_PORT(i32);
pub struct PCLUSAPI_CREATE_CLUSTER_NOTIFY_PORT_V2(i32);
pub struct PCLUSAPI_CREATE_CLUSTER_RESOURCE(i32);
pub struct PCLUSAPI_CREATE_CLUSTER_RESOURCE_TYPE(i32);
pub struct PCLUSAPI_DELETE_CLUSTER_GROUP(i32);
pub struct PCLUSAPI_DELETE_CLUSTER_GROUP_GROUPSET(i32);
pub struct PCLUSAPI_DELETE_CLUSTER_RESOURCE(i32);
pub struct PCLUSAPI_DELETE_CLUSTER_RESOURCE_TYPE(i32);
pub struct PCLUSAPI_DESTROY_CLUSTER(i32);
pub struct PCLUSAPI_DESTROY_CLUSTER_GROUP(i32);
pub struct PCLUSAPI_EVICT_CLUSTER_NODE(i32);
pub struct PCLUSAPI_EVICT_CLUSTER_NODE_EX(i32);
pub struct PCLUSAPI_FAIL_CLUSTER_RESOURCE(i32);
pub struct PCLUSAPI_GET_CLUSTER_FROM_GROUP(i32);
pub struct PCLUSAPI_GET_CLUSTER_FROM_GROUP_GROUPSET(i32);
pub struct PCLUSAPI_GET_CLUSTER_FROM_NETWORK(i32);
pub struct PCLUSAPI_GET_CLUSTER_FROM_NET_INTERFACE(i32);
pub struct PCLUSAPI_GET_CLUSTER_FROM_NODE(i32);
pub struct PCLUSAPI_GET_CLUSTER_FROM_RESOURCE(i32);
pub struct PCLUSAPI_GET_CLUSTER_GROUP_KEY(i32);
pub struct PCLUSAPI_GET_CLUSTER_GROUP_STATE(i32);
pub struct PCLUSAPI_GET_CLUSTER_INFORMATION(i32);
pub struct PCLUSAPI_GET_CLUSTER_KEY(i32);
pub struct PCLUSAPI_GET_CLUSTER_NETWORK_ID(i32);
pub struct PCLUSAPI_GET_CLUSTER_NETWORK_KEY(i32);
pub struct PCLUSAPI_GET_CLUSTER_NETWORK_STATE(i32);
pub struct PCLUSAPI_GET_CLUSTER_NET_INTERFACE(i32);
pub struct PCLUSAPI_GET_CLUSTER_NET_INTERFACE_KEY(i32);
pub struct PCLUSAPI_GET_CLUSTER_NET_INTERFACE_STATE(i32);
pub struct PCLUSAPI_GET_CLUSTER_NODE_ID(i32);
pub struct PCLUSAPI_GET_CLUSTER_NODE_KEY(i32);
pub struct PCLUSAPI_GET_CLUSTER_NODE_STATE(i32);
pub struct PCLUSAPI_GET_CLUSTER_NOTIFY(i32);
pub struct PCLUSAPI_GET_CLUSTER_NOTIFY_V2(i32);
pub struct PCLUSAPI_GET_CLUSTER_QUORUM_RESOURCE(i32);
pub struct PCLUSAPI_GET_CLUSTER_RESOURCE_DEPENDENCY_EXPRESSION(i32);
pub struct PCLUSAPI_GET_CLUSTER_RESOURCE_KEY(i32);
pub struct PCLUSAPI_GET_CLUSTER_RESOURCE_NETWORK_NAME(i32);
pub struct PCLUSAPI_GET_CLUSTER_RESOURCE_STATE(i32);
pub struct PCLUSAPI_GET_CLUSTER_RESOURCE_TYPE_KEY(i32);
pub struct PCLUSAPI_GET_NODE_CLUSTER_STATE(i32);
pub struct PCLUSAPI_GET_NOTIFY_EVENT_HANDLE_V2(i32);
pub struct PCLUSAPI_IS_FILE_ON_CLUSTER_SHARED_VOLUME(i32);
pub struct PCLUSAPI_MOVE_CLUSTER_GROUP(i32);
pub struct PCLUSAPI_OFFLINE_CLUSTER_GROUP(i32);
pub struct PCLUSAPI_OFFLINE_CLUSTER_RESOURCE(i32);
pub struct PCLUSAPI_ONLINE_CLUSTER_GROUP(i32);
pub struct PCLUSAPI_ONLINE_CLUSTER_RESOURCE(i32);
pub struct PCLUSAPI_OPEN_CLUSTER(i32);
pub struct PCLUSAPI_OPEN_CLUSTER_EX(i32);
pub struct PCLUSAPI_OPEN_CLUSTER_GROUP(i32);
pub struct PCLUSAPI_OPEN_CLUSTER_GROUP_EX(i32);
pub struct PCLUSAPI_OPEN_CLUSTER_GROUP_GROUPSET(i32);
pub struct PCLUSAPI_OPEN_CLUSTER_NETINTERFACE_EX(i32);
pub struct PCLUSAPI_OPEN_CLUSTER_NETWORK(i32);
pub struct PCLUSAPI_OPEN_CLUSTER_NETWORK_EX(i32);
pub struct PCLUSAPI_OPEN_CLUSTER_NET_INTERFACE(i32);
pub struct PCLUSAPI_OPEN_CLUSTER_NODE(i32);
pub struct PCLUSAPI_OPEN_CLUSTER_NODE_EX(i32);
pub struct PCLUSAPI_OPEN_CLUSTER_RESOURCE(i32);
pub struct PCLUSAPI_OPEN_CLUSTER_RESOURCE_EX(i32);
pub struct PCLUSAPI_OPEN_NODE_BY_ID(i32);
pub struct PCLUSAPI_PAUSE_CLUSTER_NODE(i32);
pub struct PCLUSAPI_PAUSE_CLUSTER_NODE_EX(i32);
pub struct PCLUSAPI_REGISTER_CLUSTER_NOTIFY(i32);
pub struct PCLUSAPI_REGISTER_CLUSTER_NOTIFY_V2(i32);
pub struct PCLUSAPI_REMOVE_CLUSTER_GROUP_DEPENDENCY(i32);
pub struct PCLUSAPI_REMOVE_CLUSTER_GROUP_GROUPSET_DEPENDENCY(i32);
pub struct PCLUSAPI_REMOVE_CLUSTER_GROUP_TO_GROUP_GROUPSET_DEPENDENCY(i32);
pub struct PCLUSAPI_REMOVE_CLUSTER_NAME_ACCOUNT(i32);
pub struct PCLUSAPI_REMOVE_CLUSTER_RESOURCE_DEPENDENCY(i32);
pub struct PCLUSAPI_REMOVE_CLUSTER_RESOURCE_NODE(i32);
pub struct PCLUSAPI_REMOVE_CROSS_CLUSTER_GROUPSET_DEPENDENCY(i32);
pub struct PCLUSAPI_REMOVE_RESOURCE_FROM_CLUSTER_SHARED_VOLUMES(i32);
pub struct PCLUSAPI_RESTART_CLUSTER_RESOURCE(i32);
pub struct PCLUSAPI_RESTORE_CLUSTER_DATABASE(i32);
pub struct PCLUSAPI_RESUME_CLUSTER_NODE(i32);
pub struct PCLUSAPI_RESUME_CLUSTER_NODE_EX(i32);
pub struct PCLUSAPI_SET_CLUSTER_GROUP_GROUPSET_DEPENDENCY_EXPRESSION(i32);
pub struct PCLUSAPI_SET_CLUSTER_GROUP_NAME(i32);
pub struct PCLUSAPI_SET_CLUSTER_GROUP_NODE_LIST(i32);
pub struct PCLUSAPI_SET_CLUSTER_NETWORK_NAME(i32);
pub struct PCLUSAPI_SET_CLUSTER_NETWORK_PRIORITY_ORDER(i32);
pub struct PCLUSAPI_SET_CLUSTER_QUORUM_RESOURCE(i32);
pub struct PCLUSAPI_SET_CLUSTER_RESOURCE_DEPENDENCY_EXPRESSION(i32);
pub struct PCLUSAPI_SET_CLUSTER_RESOURCE_NAME(i32);
pub struct PCLUSAPI_SET_CLUSTER_SERVICE_ACCOUNT_PASSWORD(i32);
pub struct PCLUSAPI_SET_GROUP_DEPENDENCY_EXPRESSION(i32);
pub struct PCLUSAPI_SHARED_VOLUME_SET_SNAPSHOT_STATE(i32);
pub struct PCLUSAPI_SetClusterName(i32);
pub struct PCLUSTER_CLEAR_BACKUP_STATE_FOR_SHARED_VOLUME(i32);
pub struct PCLUSTER_DECRYPT(i32);
pub struct PCLUSTER_ENCRYPT(i32);
pub struct PCLUSTER_GET_VOLUME_NAME_FOR_VOLUME_MOUNT_POINT(i32);
pub struct PCLUSTER_GET_VOLUME_PATH_NAME(i32);
pub struct PCLUSTER_IS_PATH_ON_SHARED_VOLUME(i32);
pub struct PCLUSTER_PREPARE_SHARED_VOLUME_FOR_BACKUP(i32);
pub struct PCLUSTER_REG_BATCH_ADD_COMMAND(i32);
pub struct PCLUSTER_REG_BATCH_CLOSE_NOTIFICATION(i32);
pub struct PCLUSTER_REG_BATCH_READ_COMMAND(i32);
pub struct PCLUSTER_REG_CLOSE_BATCH(i32);
pub struct PCLUSTER_REG_CLOSE_BATCH_NOTIFY_PORT(i32);
pub struct PCLUSTER_REG_CLOSE_READ_BATCH(i32);
pub struct PCLUSTER_REG_CLOSE_READ_BATCH_EX(i32);
pub struct PCLUSTER_REG_CLOSE_READ_BATCH_REPLY(i32);
pub struct PCLUSTER_REG_CREATE_BATCH_NOTIFY_PORT(i32);
pub struct PCLUSTER_REG_CREATE_READ_BATCH(i32);
pub struct PCLUSTER_REG_GET_BATCH_NOTIFICATION(i32);
pub struct PCLUSTER_REG_READ_BATCH_ADD_COMMAND(i32);
pub struct PCLUSTER_REG_READ_BATCH_REPLY_NEXT_COMMAND(i32);
pub struct PCLUSTER_SETUP_PROGRESS_CALLBACK(i32);
pub struct PCLUSTER_SET_ACCOUNT_ACCESS(i32);
pub struct PCLUSTER_UPGRADE_PROGRESS_CALLBACK(i32);
pub struct PEND_CONTROL_CALL(i32);
pub struct PEND_TYPE_CONTROL_CALL(i32);
pub struct PEXTEND_RES_CONTROL_CALL(i32);
pub struct PEXTEND_RES_TYPE_CONTROL_CALL(i32);
pub struct PFREE_CLUSTER_CRYPT(i32);
pub struct PIS_ALIVE_ROUTINE(i32);
pub struct PLACEMENT_OPTIONS(i32);
pub struct PLOG_EVENT_ROUTINE(i32);
pub struct PLOOKS_ALIVE_ROUTINE(i32);
pub struct POFFLINE_ROUTINE(i32);
pub struct POFFLINE_V2_ROUTINE(i32);
pub struct PONLINE_ROUTINE(i32);
pub struct PONLINE_V2_ROUTINE(i32);
pub struct POPEN_CLUSTER_CRYPT_PROVIDER(i32);
pub struct POPEN_CLUSTER_CRYPT_PROVIDEREX(i32);
pub struct POPEN_ROUTINE(i32);
pub struct POPEN_V2_ROUTINE(i32);
pub struct POST_UPGRADE_VERSION_INFO(i32);
pub struct PQUERY_APPINSTANCE_VERSION(i32);
pub struct PQUORUM_RESOURCE_LOST(i32);
pub struct PRAISE_RES_TYPE_NOTIFICATION(i32);
pub struct PREGISTER_APPINSTANCE(i32);
pub struct PREGISTER_APPINSTANCE_VERSION(i32);
pub struct PRELEASE_ROUTINE(i32);
pub struct PREQUEST_DUMP_ROUTINE(i32);
pub struct PRESET_ALL_APPINSTANCE_VERSIONS(i32);
pub struct PRESOURCE_CONTROL_ROUTINE(i32);
pub struct PRESOURCE_TYPE_CONTROL_ROUTINE(i32);
pub struct PRESUTIL_ADD_UNKNOWN_PROPERTIES(i32);
pub struct PRESUTIL_CREATE_DIRECTORY_TREE(i32);
pub struct PRESUTIL_DUP_PARAMETER_BLOCK(i32);
pub struct PRESUTIL_DUP_STRING(i32);
pub struct PRESUTIL_ENUM_PRIVATE_PROPERTIES(i32);
pub struct PRESUTIL_ENUM_PROPERTIES(i32);
pub struct PRESUTIL_ENUM_RESOURCES(i32);
pub struct PRESUTIL_ENUM_RESOURCES_EX(i32);
pub struct PRESUTIL_ENUM_RESOURCES_EX2(i32);
pub struct PRESUTIL_EXPAND_ENVIRONMENT_STRINGS(i32);
pub struct PRESUTIL_FIND_BINARY_PROPERTY(i32);
pub struct PRESUTIL_FIND_DEPENDENT_DISK_RESOURCE_DRIVE_LETTER(i32);
pub struct PRESUTIL_FIND_DWORD_PROPERTY(i32);
pub struct PRESUTIL_FIND_EXPANDED_SZ_PROPERTY(i32);
pub struct PRESUTIL_FIND_EXPAND_SZ_PROPERTY(i32);
pub struct PRESUTIL_FIND_FILETIME_PROPERTY(i32);
pub struct PRESUTIL_FIND_LONG_PROPERTY(i32);
pub struct PRESUTIL_FIND_MULTI_SZ_PROPERTY(i32);
pub struct PRESUTIL_FIND_SZ_PROPERTY(i32);
pub struct PRESUTIL_FIND_ULARGEINTEGER_PROPERTY(i32);
pub struct PRESUTIL_FREE_ENVIRONMENT(i32);
pub struct PRESUTIL_FREE_PARAMETER_BLOCK(i32);
pub struct PRESUTIL_GET_ALL_PROPERTIES(i32);
pub struct PRESUTIL_GET_BINARY_PROPERTY(i32);
pub struct PRESUTIL_GET_BINARY_VALUE(i32);
pub struct PRESUTIL_GET_CORE_CLUSTER_RESOURCES(i32);
pub struct PRESUTIL_GET_CORE_CLUSTER_RESOURCES_EX(i32);
pub struct PRESUTIL_GET_DWORD_PROPERTY(i32);
pub struct PRESUTIL_GET_DWORD_VALUE(i32);
pub struct PRESUTIL_GET_ENVIRONMENT_WITH_NET_NAME(i32);
pub struct PRESUTIL_GET_EXPAND_SZ_VALUE(i32);
pub struct PRESUTIL_GET_FILETIME_PROPERTY(i32);
pub struct PRESUTIL_GET_LONG_PROPERTY(i32);
pub struct PRESUTIL_GET_MULTI_SZ_PROPERTY(i32);
pub struct PRESUTIL_GET_PRIVATE_PROPERTIES(i32);
pub struct PRESUTIL_GET_PROPERTIES(i32);
pub struct PRESUTIL_GET_PROPERTIES_TO_PARAMETER_BLOCK(i32);
pub struct PRESUTIL_GET_PROPERTY(i32);
pub struct PRESUTIL_GET_PROPERTY_FORMATS(i32);
pub struct PRESUTIL_GET_PROPERTY_SIZE(i32);
pub struct PRESUTIL_GET_QWORD_VALUE(i32);
pub struct PRESUTIL_GET_RESOURCE_DEPENDENCY(i32);
pub struct PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_CLASS(i32);
pub struct PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_CLASS_EX(i32);
pub struct PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_NAME(i32);
pub struct PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_NAME_EX(i32);
pub struct PRESUTIL_GET_RESOURCE_DEPENDENCY_EX(i32);
pub struct PRESUTIL_GET_RESOURCE_DEPENDENTIP_ADDRESS_PROPS(i32);
pub struct PRESUTIL_GET_RESOURCE_NAME(i32);
pub struct PRESUTIL_GET_RESOURCE_NAME_DEPENDENCY(i32);
pub struct PRESUTIL_GET_RESOURCE_NAME_DEPENDENCY_EX(i32);
pub struct PRESUTIL_GET_SZ_PROPERTY(i32);
pub struct PRESUTIL_GET_SZ_VALUE(i32);
pub struct PRESUTIL_IS_PATH_VALID(i32);
pub struct PRESUTIL_IS_RESOURCE_CLASS_EQUAL(i32);
pub struct PRESUTIL_PROPERTY_LIST_FROM_PARAMETER_BLOCK(i32);
pub struct PRESUTIL_REMOVE_RESOURCE_SERVICE_ENVIRONMENT(i32);
pub struct PRESUTIL_RESOURCES_EQUAL(i32);
pub struct PRESUTIL_RESOURCE_TYPES_EQUAL(i32);
pub struct PRESUTIL_SET_BINARY_VALUE(i32);
pub struct PRESUTIL_SET_DWORD_VALUE(i32);
pub struct PRESUTIL_SET_EXPAND_SZ_VALUE(i32);
pub struct PRESUTIL_SET_MULTI_SZ_VALUE(i32);
pub struct PRESUTIL_SET_PRIVATE_PROPERTY_LIST(i32);
pub struct PRESUTIL_SET_PROPERTY_PARAMETER_BLOCK(i32);
pub struct PRESUTIL_SET_PROPERTY_PARAMETER_BLOCK_EX(i32);
pub struct PRESUTIL_SET_PROPERTY_TABLE(i32);
pub struct PRESUTIL_SET_PROPERTY_TABLE_EX(i32);
pub struct PRESUTIL_SET_QWORD_VALUE(i32);
pub struct PRESUTIL_SET_RESOURCE_SERVICE_ENVIRONMENT(i32);
pub struct PRESUTIL_SET_RESOURCE_SERVICE_START_PARAMETERS(i32);
pub struct PRESUTIL_SET_RESOURCE_SERVICE_START_PARAMETERS_EX(i32);
pub struct PRESUTIL_SET_SZ_VALUE(i32);
pub struct PRESUTIL_SET_UNKNOWN_PROPERTIES(i32);
pub struct PRESUTIL_START_RESOURCE_SERVICE(i32);
pub struct PRESUTIL_STOP_RESOURCE_SERVICE(i32);
pub struct PRESUTIL_STOP_SERVICE(i32);
pub struct PRESUTIL_TERMINATE_SERVICE_PROCESS_FROM_RES_DLL(i32);
pub struct PRESUTIL_VERIFY_PRIVATE_PROPERTY_LIST(i32);
pub struct PRESUTIL_VERIFY_PROPERTY_TABLE(i32);
pub struct PRESUTIL_VERIFY_RESOURCE_SERVICE(i32);
pub struct PRESUTIL_VERIFY_SERVICE(i32);
pub struct PRES_UTIL_VERIFY_SHUTDOWN_SAFE(i32);
pub struct PSET_INTERNAL_STATE(i32);
pub struct PSET_RESOURCE_INMEMORY_NODELOCAL_PROPERTIES_ROUTINE(i32);
pub struct PSET_RESOURCE_LOCKED_MODE_EX_ROUTINE(i32);
pub struct PSET_RESOURCE_LOCKED_MODE_ROUTINE(i32);
pub struct PSET_RESOURCE_STATUS_ROUTINE(i32);
pub struct PSET_RESOURCE_STATUS_ROUTINE_EX(i32);
pub struct PSIGNAL_FAILURE_ROUTINE(i32);
pub struct PSTARTUP_EX_ROUTINE(i32);
pub struct PSTARTUP_ROUTINE(i32);
pub struct PTERMINATE_ROUTINE(i32);
pub struct PWORKER_START_ROUTINE(i32);
pub struct PaxosTagCStruct(i32);
pub struct RESDLL_CONTEXT_OPERATION_TYPE(i32);
pub struct RESOURCE_EXIT_STATE(i32);
pub struct RESOURCE_FAILURE_INFO(i32);
pub struct RESOURCE_FAILURE_INFO_BUFFER(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const RESOURCE_FAILURE_INFO_VERSION_1: u32 = 1u32;
pub struct RESOURCE_MONITOR_STATE(i32);
pub struct RESOURCE_STATUS(i32);
pub struct RESOURCE_STATUS_EX(i32);
pub struct RESOURCE_TERMINAL_FAILURE_INFO_BUFFER(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const RESTYPE_MONITOR_SHUTTING_DOWN_CLUSSVC_CRASH: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const RESTYPE_MONITOR_SHUTTING_DOWN_NODE_STOP: u32 = 1u32;
pub struct RESUTIL_FILETIME_DATA(i32);
pub struct RESUTIL_LARGEINT_DATA(i32);
pub struct RESUTIL_PROPERTY_ITEM(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const RESUTIL_PROPITEM_IN_MEMORY: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const RESUTIL_PROPITEM_READ_ONLY: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const RESUTIL_PROPITEM_REQUIRED: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const RESUTIL_PROPITEM_SIGNED: u32 = 4u32;
pub struct RESUTIL_ULARGEINT_DATA(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const RS3_UPGRADE_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const RS4_UPGRADE_VERSION: u32 = 2u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const RS5_UPGRADE_VERSION: u32 = 3u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const RedirectedIOReasonBitLockerInitializing: u64 = 16u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const RedirectedIOReasonFileSystemTiering: u64 = 8u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const RedirectedIOReasonMax: u64 = 9223372036854775808u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const RedirectedIOReasonReFs: u64 = 32u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const RedirectedIOReasonUnsafeFileSystemFilter: u64 = 2u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const RedirectedIOReasonUnsafeVolumeFilter: u64 = 4u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const RedirectedIOReasonUserRequest: u64 = 1u64;
pub struct ResourceUtilizationInfoElement(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const SET_APPINSTANCE_CSV_FLAGS_VALID_ONLY_IF_CSV_COORDINATOR: u32 = 1u32;
pub struct SET_APP_INSTANCE_CSV_FLAGS(i32);
pub struct SR_DISK_REPLICATION_ELIGIBLE(i32);
pub struct SR_REPLICATED_DISK_TYPE(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const SR_REPLICATED_PARTITION_DISALLOW_MULTINODE_IO: u32 = 1u32;
pub struct SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP(i32);
pub struct SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT(i32);
pub struct SR_RESOURCE_TYPE_DISK_INFO(i32);
pub struct SR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT(i32);
pub struct SR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS(i32);
pub struct SR_RESOURCE_TYPE_QUERY_ELIGIBLE_SOURCE_DATADISKS(i32);
pub struct SR_RESOURCE_TYPE_QUERY_ELIGIBLE_TARGET_DATADISKS(i32);
pub struct SR_RESOURCE_TYPE_REPLICATED_DISK(i32);
pub struct SR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT(i32);
pub struct SR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY(i32);
pub struct SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO(i32);
pub struct VM_RESDLL_CONTEXT(i32);
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const VolumeRedirectedIOReasonMax: u64 = 9223372036854775808u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const VolumeRedirectedIOReasonNoDiskConnectivity: u64 = 1u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const VolumeRedirectedIOReasonStorageSpaceNotAttached: u64 = 2u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const VolumeRedirectedIOReasonVolumeReplicationEnabled: u64 = 4u64;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const WS2016_RTM_UPGRADE_VERSION: u32 = 8u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const WS2016_TP4_UPGRADE_VERSION: u32 = 6u32;
#[doc = "*Required features: `Win32_Networking_Clustering`*"]
pub const WS2016_TP5_UPGRADE_VERSION: u32 = 7u32;
pub struct WitnessTagHelper(i32);
pub struct WitnessTagUpdateHelper(i32);
pub struct _HCHANGE(i32);
pub struct _HCLUSCRYPTPROVIDER(i32);
pub struct _HCLUSENUM(i32);
pub struct _HCLUSENUMEX(i32);
pub struct _HCLUSTER(i32);
pub struct _HGROUP(i32);
pub struct _HGROUPENUM(i32);
pub struct _HGROUPENUMEX(i32);
pub struct _HGROUPSET(i32);
pub struct _HGROUPSETENUM(i32);
pub struct _HNETINTERFACE(i32);
pub struct _HNETINTERFACEENUM(i32);
pub struct _HNETWORK(i32);
pub struct _HNETWORKENUM(i32);
pub struct _HNODE(i32);
pub struct _HNODEENUM(i32);
pub struct _HNODEENUMEX(i32);
pub struct _HREGBATCH(i32);
pub struct _HREGBATCHNOTIFICATION(i32);
pub struct _HREGBATCHPORT(i32);
pub struct _HREGREADBATCH(i32);
pub struct _HREGREADBATCHREPLY(i32);
pub struct _HRESENUM(i32);
pub struct _HRESENUMEX(i32);
pub struct _HRESOURCE(i32);
pub struct _HRESTYPEENUM(i32);
