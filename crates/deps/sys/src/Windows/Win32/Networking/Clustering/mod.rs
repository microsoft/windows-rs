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
    pub fn AddClusterNode(hcluster: *const _HCLUSTER, lpsznodename: super::super::Foundation::PWSTR, pfnprogresscallback: ::windows::runtime::RawPtr, pvcallbackarg: *const ::core::ffi::c_void) -> *mut _HNODE;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddClusterNodeEx(hcluster: *const _HCLUSTER, lpsznodename: super::super::Foundation::PWSTR, dwflags: u32, pfnprogresscallback: ::windows::runtime::RawPtr, pvcallbackarg: *const ::core::ffi::c_void) -> *mut _HNODE;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn AddClusterResourceDependency(hresource: *const _HRESOURCE, hdependson: *const _HRESOURCE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn AddClusterResourceNode(hresource: *const _HRESOURCE, hnode: *const _HNODE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddClusterStorageNode(hcluster: *const _HCLUSTER, lpsznodename: super::super::Foundation::PWSTR, pfnprogresscallback: ::windows::runtime::RawPtr, pvcallbackarg: *const ::core::ffi::c_void, lpszclusterstoragenodedescription: super::super::Foundation::PWSTR, lpszclusterstoragenodelocation: super::super::Foundation::PWSTR) -> u32;
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
    pub fn ClusWorkerCreate(lpworker: *mut CLUS_WORKER, lpstartaddress: ::windows::runtime::RawPtr, lpparameter: *mut ::core::ffi::c_void) -> u32;
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
    pub fn ClusterSharedVolumeSetSnapshotState(guidsnapshotset: ::windows::runtime::GUID, lpszvolumename: super::super::Foundation::PWSTR, state: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterUpgradeFunctionalLevel(hcluster: *const _HCLUSTER, perform: super::super::Foundation::BOOL, pfnprogresscallback: ::windows::runtime::RawPtr, pvcallbackarg: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateCluster(pconfig: *const CREATE_CLUSTER_CONFIG, pfnprogresscallback: ::windows::runtime::RawPtr, pvcallbackarg: *const ::core::ffi::c_void) -> *mut _HCLUSTER;
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
    pub fn CreateClusterNameAccount(hcluster: *const _HCLUSTER, pconfig: *const CREATE_CLUSTER_NAME_ACCOUNT, pfnprogresscallback: ::windows::runtime::RawPtr, pvcallbackarg: *const ::core::ffi::c_void) -> u32;
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
    pub fn DestroyCluster(hcluster: *const _HCLUSTER, pfnprogresscallback: ::windows::runtime::RawPtr, pvcallbackarg: *const ::core::ffi::c_void, fdeletevirtualcomputerobjects: super::super::Foundation::BOOL) -> u32;
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
    pub fn EvictClusterNodeEx(hnode: *const _HNODE, dwtimeout: u32, phrcleanupstatus: *mut ::windows::runtime::HRESULT) -> u32;
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
    pub fn QueryAppInstanceVersion(appinstanceid: *const ::windows::runtime::GUID, instanceversionhigh: *mut u64, instanceversionlow: *mut u64, versionstatus: *mut super::super::Foundation::NTSTATUS) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterAppInstance(processhandle: super::super::Foundation::HANDLE, appinstanceid: *const ::windows::runtime::GUID, childreninheritappinstance: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn RegisterAppInstanceVersion(appinstanceid: *const ::windows::runtime::GUID, instanceversionhigh: u64, instanceversionlow: u64) -> u32;
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
    pub fn ResUtilEnumGroups(hcluster: *mut _HCLUSTER, hself: *mut _HGROUP, prescallback: ::windows::runtime::RawPtr, pparameter: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilEnumGroupsEx(hcluster: *mut _HCLUSTER, hself: *mut _HGROUP, grouptype: CLUSGROUP_TYPE, prescallback: ::windows::runtime::RawPtr, pparameter: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilEnumPrivateProperties(hkeyclusterkey: super::super::System::Registry::HKEY, pszoutproperties: super::super::Foundation::PWSTR, cboutpropertiessize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilEnumProperties(ppropertytable: *const RESUTIL_PROPERTY_ITEM, pszoutproperties: super::super::Foundation::PWSTR, cboutpropertiessize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilEnumResources(hself: *mut _HRESOURCE, lpszrestypename: super::super::Foundation::PWSTR, prescallback: ::windows::runtime::RawPtr, pparameter: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilEnumResourcesEx(hcluster: *mut _HCLUSTER, hself: *mut _HRESOURCE, lpszrestypename: super::super::Foundation::PWSTR, prescallback: ::windows::runtime::RawPtr, pparameter: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilEnumResourcesEx2(hcluster: *mut _HCLUSTER, hself: *mut _HRESOURCE, lpszrestypename: super::super::Foundation::PWSTR, prescallback: ::windows::runtime::RawPtr, pparameter: *mut ::core::ffi::c_void, dwdesiredaccess: u32) -> u32;
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
    pub fn ResUtilGetClusterId(hcluster: *mut _HCLUSTER, guid: *mut ::windows::runtime::GUID) -> u32;
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
    pub fn ResUtilNodeEnum(hcluster: *mut _HCLUSTER, pnodecallback: ::windows::runtime::RawPtr, pparameter: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilPaxosComparer(left: *const PaxosTagCStruct, right: *const PaxosTagCStruct) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilPropertyListFromParameterBlock(ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut ::core::ffi::c_void, pcboutpropertylistsize: *mut u32, pinparams: *const u8, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilRemoveResourceServiceEnvironment(pszservicename: super::super::Foundation::PWSTR, pfnlogevent: ::windows::runtime::RawPtr, hresourcehandle: isize) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`*"]
    pub fn ResUtilResourceDepEnum(hself: *mut _HRESOURCE, enumtype: u32, prescallback: ::windows::runtime::RawPtr, pparameter: *mut ::core::ffi::c_void) -> u32;
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
    pub fn ResUtilSetResourceServiceEnvironment(pszservicename: super::super::Foundation::PWSTR, hresource: *mut _HRESOURCE, pfnlogevent: ::windows::runtime::RawPtr, hresourcehandle: isize) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn ResUtilSetResourceServiceStartParameters(pszservicename: super::super::Foundation::PWSTR, schscmhandle: super::super::Security::SC_HANDLE, phservice: *mut isize, pfnlogevent: ::windows::runtime::RawPtr, hresourcehandle: isize) -> u32;
    #[doc = "*Required features: `Win32_Networking_Clustering`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn ResUtilSetResourceServiceStartParametersEx(pszservicename: super::super::Foundation::PWSTR, schscmhandle: super::super::Security::SC_HANDLE, phservice: *mut isize, dwdesiredaccess: u32, pfnlogevent: ::windows::runtime::RawPtr, hresourcehandle: isize) -> u32;
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
    pub fn ResUtilTerminateServiceProcessFromResDll(dwservicepid: u32, boffline: super::super::Foundation::BOOL, pdwresourcestate: *mut u32, pfnlogevent: ::windows::runtime::RawPtr, hresourcehandle: isize) -> u32;
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
