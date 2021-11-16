#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    pub fn AddClusterGroupDependency(hdependentgroup: *const _HGROUP, hprovidergroup: *const _HGROUP) -> u32;
    pub fn AddClusterGroupSetDependency(hdependentgroupset: *const _HGROUPSET, hprovidergroupset: *const _HGROUPSET) -> u32;
    pub fn AddClusterGroupToGroupSetDependency(hdependentgroup: *const _HGROUP, hprovidergroupset: *const _HGROUPSET) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddClusterNode(hcluster: *const _HCLUSTER, lpsznodename: super::super::Foundation::PWSTR, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> *mut _HNODE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddClusterNodeEx(hcluster: *const _HCLUSTER, lpsznodename: super::super::Foundation::PWSTR, dwflags: u32, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> *mut _HNODE;
    pub fn AddClusterResourceDependency(hresource: *const _HRESOURCE, hdependson: *const _HRESOURCE) -> u32;
    pub fn AddClusterResourceNode(hresource: *const _HRESOURCE, hnode: *const _HNODE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddClusterStorageNode(hcluster: *const _HCLUSTER, lpsznodename: super::super::Foundation::PWSTR, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void, lpszclusterstoragenodedescription: super::super::Foundation::PWSTR, lpszclusterstoragenodelocation: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddCrossClusterGroupSetDependency(hdependentgroupset: *const _HGROUPSET, lpremoteclustername: super::super::Foundation::PWSTR, lpremotegroupsetname: super::super::Foundation::PWSTR) -> u32;
    pub fn AddResourceToClusterSharedVolumes(hresource: *const _HRESOURCE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BackupClusterDatabase(hcluster: *const _HCLUSTER, lpszpathname: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CanResourceBeDependent(hresource: *const _HRESOURCE, hresourcedependent: *const _HRESOURCE) -> super::super::Foundation::BOOL;
    pub fn CancelClusterGroupOperation(hgroup: *const _HGROUP, dwcancelflags_reserved: u32) -> u32;
    pub fn ChangeClusterResourceGroup(hresource: *const _HRESOURCE, hgroup: *const _HGROUP) -> u32;
    pub fn ChangeClusterResourceGroupEx(hresource: *const _HRESOURCE, hgroup: *const _HGROUP, flags: u64) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseCluster(hcluster: *const _HCLUSTER) -> super::super::Foundation::BOOL;
    pub fn CloseClusterCryptProvider(hcluscryptprovider: *const _HCLUSCRYPTPROVIDER) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseClusterGroup(hgroup: *const _HGROUP) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseClusterGroupSet(hgroupset: *const _HGROUPSET) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseClusterNetInterface(hnetinterface: *const _HNETINTERFACE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseClusterNetwork(hnetwork: *const _HNETWORK) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseClusterNode(hnode: *const _HNODE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseClusterNotifyPort(hchange: *const _HCHANGE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseClusterResource(hresource: *const _HRESOURCE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusAddClusterHealthFault(hcluster: *const _HCLUSTER, failure: *const CLUSTER_HEALTH_FAULT, param2: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusGetClusterHealthFaults(hcluster: *const _HCLUSTER, objects: *mut CLUSTER_HEALTH_FAULT_ARRAY, flags: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusRemoveClusterHealthFault(hcluster: *const _HCLUSTER, id: super::super::Foundation::PWSTR, flags: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusWorkerCheckTerminate(lpworker: *mut CLUS_WORKER) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusWorkerCreate(lpworker: *mut CLUS_WORKER, lpstartaddress: PWORKER_START_ROUTINE, lpparameter: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusWorkerTerminate(lpworker: *const CLUS_WORKER);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusWorkerTerminateEx(clusworker: *mut CLUS_WORKER, timeoutinmilliseconds: u32, waitonly: super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusWorkersTerminate(clusworkers: *mut *mut CLUS_WORKER, clusworkerscount: usize, timeoutinmilliseconds: u32, waitonly: super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterAddGroupToAffinityRule(hcluster: *const _HCLUSTER, rulename: super::super::Foundation::PWSTR, hgroup: *const _HGROUP) -> u32;
    pub fn ClusterAddGroupToGroupSet(hgroupset: *const _HGROUPSET, hgroup: *const _HGROUP) -> u32;
    pub fn ClusterAddGroupToGroupSetWithDomains(hgroupset: *const _HGROUPSET, hgroup: *const _HGROUP, faultdomain: u32, updatedomain: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterAffinityRuleControl(hcluster: *const _HCLUSTER, affinityrulename: super::super::Foundation::PWSTR, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterClearBackupStateForSharedVolume(lpszvolumepathname: super::super::Foundation::PWSTR) -> u32;
    pub fn ClusterCloseEnum(henum: *const _HCLUSENUM) -> u32;
    pub fn ClusterCloseEnumEx(hclusterenum: *const _HCLUSENUMEX) -> u32;
    pub fn ClusterControl(hcluster: *const _HCLUSTER, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterCreateAffinityRule(hcluster: *const _HCLUSTER, rulename: super::super::Foundation::PWSTR, ruletype: CLUS_AFFINITY_RULE_TYPE) -> u32;
    pub fn ClusterDecrypt(hcluscryptprovider: *const _HCLUSCRYPTPROVIDER, pcryptinput: *const u8, cbcryptinput: u32, ppcryptoutput: *mut *mut u8, pcbcryptoutput: *mut u32) -> u32;
    pub fn ClusterEncrypt(hcluscryptprovider: *const _HCLUSCRYPTPROVIDER, pdata: *const u8, cbdata: u32, ppdata: *mut *mut u8, pcbdata: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterEnum(henum: *const _HCLUSENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterEnumEx(hclusterenum: *const _HCLUSENUMEX, dwindex: u32, pitem: *mut CLUSTER_ENUM_ITEM, cbitem: *mut u32) -> u32;
    pub fn ClusterGetEnumCount(henum: *const _HCLUSENUM) -> u32;
    pub fn ClusterGetEnumCountEx(hclusterenum: *const _HCLUSENUMEX) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterGetVolumeNameForVolumeMountPoint(lpszvolumemountpoint: super::super::Foundation::PWSTR, lpszvolumename: super::super::Foundation::PWSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterGetVolumePathName(lpszfilename: super::super::Foundation::PWSTR, lpszvolumepathname: super::super::Foundation::PWSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL;
    pub fn ClusterGroupCloseEnum(hgroupenum: *const _HGROUPENUM) -> u32;
    pub fn ClusterGroupCloseEnumEx(hgroupenumex: *const _HGROUPENUMEX) -> u32;
    pub fn ClusterGroupControl(hgroup: *const _HGROUP, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterGroupEnum(hgroupenum: *const _HGROUPENUM, dwindex: u32, lpdwtype: *mut u32, lpszresourcename: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterGroupEnumEx(hgroupenumex: *const _HGROUPENUMEX, dwindex: u32, pitem: *mut CLUSTER_GROUP_ENUM_ITEM, cbitem: *mut u32) -> u32;
    pub fn ClusterGroupGetEnumCount(hgroupenum: *const _HGROUPENUM) -> u32;
    pub fn ClusterGroupGetEnumCountEx(hgroupenumex: *const _HGROUPENUMEX) -> u32;
    pub fn ClusterGroupOpenEnum(hgroup: *const _HGROUP, dwtype: u32) -> *mut _HGROUPENUM;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterGroupOpenEnumEx(hcluster: *const _HCLUSTER, lpszproperties: super::super::Foundation::PWSTR, cbproperties: u32, lpszroproperties: super::super::Foundation::PWSTR, cbroproperties: u32, dwflags: u32) -> *mut _HGROUPENUMEX;
    pub fn ClusterGroupSetCloseEnum(hgroupsetenum: *mut _HGROUPSETENUM) -> u32;
    pub fn ClusterGroupSetControl(hgroupset: *const _HGROUPSET, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterGroupSetEnum(hgroupsetenum: *const _HGROUPSETENUM, dwindex: u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
    pub fn ClusterGroupSetGetEnumCount(hgroupsetenum: *mut _HGROUPSETENUM) -> u32;
    pub fn ClusterGroupSetOpenEnum(hcluster: *mut _HCLUSTER) -> *mut _HGROUPSETENUM;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterIsPathOnSharedVolume(lpszpathname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    pub fn ClusterNetInterfaceCloseEnum(hnetinterfaceenum: *const _HNETINTERFACEENUM) -> u32;
    pub fn ClusterNetInterfaceControl(hnetinterface: *const _HNETINTERFACE, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterNetInterfaceEnum(hnetinterfaceenum: *const _HNETINTERFACEENUM, dwindex: u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterNetInterfaceOpenEnum(hcluster: *const _HCLUSTER, lpsznodename: super::super::Foundation::PWSTR, lpsznetworkname: super::super::Foundation::PWSTR) -> *mut _HNETINTERFACEENUM;
    pub fn ClusterNetworkCloseEnum(hnetworkenum: *const _HNETWORKENUM) -> u32;
    pub fn ClusterNetworkControl(hnetwork: *const _HNETWORK, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterNetworkEnum(hnetworkenum: *const _HNETWORKENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
    pub fn ClusterNetworkGetEnumCount(hnetworkenum: *const _HNETWORKENUM) -> u32;
    pub fn ClusterNetworkOpenEnum(hnetwork: *const _HNETWORK, dwtype: u32) -> *mut _HNETWORKENUM;
    pub fn ClusterNodeCloseEnum(hnodeenum: *const _HNODEENUM) -> u32;
    pub fn ClusterNodeCloseEnumEx(hnodeenum: *const _HNODEENUMEX) -> u32;
    pub fn ClusterNodeControl(hnode: *const _HNODE, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterNodeEnum(hnodeenum: *const _HNODEENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterNodeEnumEx(hnodeenum: *const _HNODEENUMEX, dwindex: u32, pitem: *mut CLUSTER_ENUM_ITEM, cbitem: *mut u32) -> u32;
    pub fn ClusterNodeGetEnumCount(hnodeenum: *const _HNODEENUM) -> u32;
    pub fn ClusterNodeGetEnumCountEx(hnodeenum: *const _HNODEENUMEX) -> u32;
    pub fn ClusterNodeOpenEnum(hnode: *const _HNODE, dwtype: u32) -> *mut _HNODEENUM;
    pub fn ClusterNodeOpenEnumEx(hnode: *const _HNODE, dwtype: u32, poptions: *const ::core::ffi::c_void) -> *mut _HNODEENUMEX;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterNodeReplacement(hcluster: *const _HCLUSTER, lpsznodenamecurrent: super::super::Foundation::PWSTR, lpsznodenamenew: super::super::Foundation::PWSTR) -> u32;
    pub fn ClusterOpenEnum(hcluster: *const _HCLUSTER, dwtype: u32) -> *mut _HCLUSENUM;
    pub fn ClusterOpenEnumEx(hcluster: *const _HCLUSTER, dwtype: u32, poptions: *const ::core::ffi::c_void) -> *mut _HCLUSENUMEX;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterPrepareSharedVolumeForBackup(lpszfilename: super::super::Foundation::PWSTR, lpszvolumepathname: super::super::Foundation::PWSTR, lpcchvolumepathname: *mut u32, lpszvolumename: super::super::Foundation::PWSTR, lpcchvolumename: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterRegBatchAddCommand(hregbatch: *const _HREGBATCH, dwcommand: CLUSTER_REG_COMMAND, wzname: super::super::Foundation::PWSTR, dwoptions: u32, lpdata: *const ::core::ffi::c_void, cbdata: u32) -> i32;
    pub fn ClusterRegBatchCloseNotification(hbatchnotification: *const _HREGBATCHNOTIFICATION) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterRegBatchReadCommand(hbatchnotification: *const _HREGBATCHNOTIFICATION, pbatchcommand: *mut CLUSTER_BATCH_COMMAND) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterRegCloseBatch(hregbatch: *const _HREGBATCH, bcommit: super::super::Foundation::BOOL, failedcommandnumber: *mut i32) -> i32;
    pub fn ClusterRegCloseBatchEx(hregbatch: *const _HREGBATCH, flags: u32, failedcommandnumber: *mut i32) -> i32;
    pub fn ClusterRegCloseBatchNotifyPort(hbatchnotifyport: *const _HREGBATCHPORT) -> i32;
    #[cfg(feature = "Win32_System_Registry")]
    pub fn ClusterRegCloseKey(hkey: super::super::System::Registry::HKEY) -> i32;
    pub fn ClusterRegCloseReadBatch(hregreadbatch: *const _HREGREADBATCH, phregreadbatchreply: *mut *mut _HREGREADBATCHREPLY) -> i32;
    pub fn ClusterRegCloseReadBatchEx(hregreadbatch: *const _HREGREADBATCH, flags: u32, phregreadbatchreply: *mut *mut _HREGREADBATCHREPLY) -> i32;
    pub fn ClusterRegCloseReadBatchReply(hregreadbatchreply: *const _HREGREADBATCHREPLY) -> i32;
    #[cfg(feature = "Win32_System_Registry")]
    pub fn ClusterRegCreateBatch(hkey: super::super::System::Registry::HKEY, phregbatch: *mut *mut _HREGBATCH) -> i32;
    #[cfg(feature = "Win32_System_Registry")]
    pub fn ClusterRegCreateBatchNotifyPort(hkey: super::super::System::Registry::HKEY, phbatchnotifyport: *mut *mut _HREGBATCHPORT) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Registry"))]
    pub fn ClusterRegCreateKey(hkey: super::super::System::Registry::HKEY, lpszsubkey: super::super::Foundation::PWSTR, dwoptions: u32, samdesired: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, phkresult: *mut super::super::System::Registry::HKEY, lpdwdisposition: *mut u32) -> i32;
    #[cfg(feature = "Win32_System_Registry")]
    pub fn ClusterRegCreateReadBatch(hkey: super::super::System::Registry::HKEY, phregreadbatch: *mut *mut _HREGREADBATCH) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ClusterRegDeleteKey(hkey: super::super::System::Registry::HKEY, lpszsubkey: super::super::Foundation::PWSTR) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ClusterRegDeleteValue(hkey: super::super::System::Registry::HKEY, lpszvaluename: super::super::Foundation::PWSTR) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ClusterRegEnumKey(hkey: super::super::System::Registry::HKEY, dwindex: u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32, lpftlastwritetime: *mut super::super::Foundation::FILETIME) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ClusterRegEnumValue(hkey: super::super::System::Registry::HKEY, dwindex: u32, lpszvaluename: super::super::Foundation::PWSTR, lpcchvaluename: *mut u32, lpdwtype: *mut u32, lpdata: *mut u8, lpcbdata: *mut u32) -> u32;
    pub fn ClusterRegGetBatchNotification(hbatchnotify: *const _HREGBATCHPORT, phbatchnotification: *mut *mut _HREGBATCHNOTIFICATION) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Registry"))]
    pub fn ClusterRegGetKeySecurity(hkey: super::super::System::Registry::HKEY, requestedinformation: u32, psecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ClusterRegOpenKey(hkey: super::super::System::Registry::HKEY, lpszsubkey: super::super::Foundation::PWSTR, samdesired: u32, phkresult: *mut super::super::System::Registry::HKEY) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ClusterRegQueryInfoKey(hkey: super::super::System::Registry::HKEY, lpcsubkeys: *const u32, lpcchmaxsubkeylen: *const u32, lpcvalues: *const u32, lpcchmaxvaluenamelen: *const u32, lpcbmaxvaluelen: *const u32, lpcbsecuritydescriptor: *const u32, lpftlastwritetime: *const super::super::Foundation::FILETIME) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ClusterRegQueryValue(hkey: super::super::System::Registry::HKEY, lpszvaluename: super::super::Foundation::PWSTR, lpdwvaluetype: *mut u32, lpdata: *mut u8, lpcbdata: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterRegReadBatchAddCommand(hregreadbatch: *const _HREGREADBATCH, wzsubkeyname: super::super::Foundation::PWSTR, wzvaluename: super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterRegReadBatchReplyNextCommand(hregreadbatchreply: *const _HREGREADBATCHREPLY, pbatchcommand: *mut CLUSTER_READ_BATCH_COMMAND) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Registry"))]
    pub fn ClusterRegSetKeySecurity(hkey: super::super::System::Registry::HKEY, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ClusterRegSetValue(hkey: super::super::System::Registry::HKEY, lpszvaluename: super::super::Foundation::PWSTR, dwtype: u32, lpdata: *const u8, cbdata: u32) -> u32;
    pub fn ClusterRegSyncDatabase(hcluster: *const _HCLUSTER, flags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterRemoveAffinityRule(hcluster: *const _HCLUSTER, rulename: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterRemoveGroupFromAffinityRule(hcluster: *const _HCLUSTER, rulename: super::super::Foundation::PWSTR, hgroup: *const _HGROUP) -> u32;
    pub fn ClusterRemoveGroupFromGroupSet(hgroup: *const _HGROUP) -> u32;
    pub fn ClusterResourceCloseEnum(hresenum: *const _HRESENUM) -> u32;
    pub fn ClusterResourceCloseEnumEx(hresourceenumex: *const _HRESENUMEX) -> u32;
    pub fn ClusterResourceControl(hresource: *const _HRESOURCE, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
    pub fn ClusterResourceControlAsUser(hresource: *const _HRESOURCE, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterResourceEnum(hresenum: *const _HRESENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterResourceEnumEx(hresourceenumex: *const _HRESENUMEX, dwindex: u32, pitem: *mut CLUSTER_RESOURCE_ENUM_ITEM, cbitem: *mut u32) -> u32;
    pub fn ClusterResourceGetEnumCount(hresenum: *const _HRESENUM) -> u32;
    pub fn ClusterResourceGetEnumCountEx(hresourceenumex: *const _HRESENUMEX) -> u32;
    pub fn ClusterResourceOpenEnum(hresource: *const _HRESOURCE, dwtype: u32) -> *mut _HRESENUM;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterResourceOpenEnumEx(hcluster: *const _HCLUSTER, lpszproperties: super::super::Foundation::PWSTR, cbproperties: u32, lpszroproperties: super::super::Foundation::PWSTR, cbroproperties: u32, dwflags: u32) -> *mut _HRESENUMEX;
    pub fn ClusterResourceTypeCloseEnum(hrestypeenum: *const _HRESTYPEENUM) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterResourceTypeControl(hcluster: *const _HCLUSTER, lpszresourcetypename: super::super::Foundation::PWSTR, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterResourceTypeControlAsUser(hcluster: *const _HCLUSTER, lpszresourcetypename: super::super::Foundation::PWSTR, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterResourceTypeEnum(hrestypeenum: *const _HRESTYPEENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
    pub fn ClusterResourceTypeGetEnumCount(hrestypeenum: *const _HRESTYPEENUM) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterResourceTypeOpenEnum(hcluster: *const _HCLUSTER, lpszresourcetypename: super::super::Foundation::PWSTR, dwtype: u32) -> *mut _HRESTYPEENUM;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterSetAccountAccess(hcluster: *const _HCLUSTER, szaccountsid: super::super::Foundation::PWSTR, dwaccess: u32, dwcontroltype: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterSharedVolumeSetSnapshotState(guidsnapshotset: ::windows_sys::core::GUID, lpszvolumename: super::super::Foundation::PWSTR, state: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClusterUpgradeFunctionalLevel(hcluster: *const _HCLUSTER, perform: super::super::Foundation::BOOL, pfnprogresscallback: PCLUSTER_UPGRADE_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateCluster(pconfig: *const CREATE_CLUSTER_CONFIG, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> *mut _HCLUSTER;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateClusterAvailabilitySet(hcluster: *const _HCLUSTER, lpavailabilitysetname: super::super::Foundation::PWSTR, pavailabilitysetconfig: *const CLUSTER_AVAILABILITY_SET_CONFIG) -> *mut _HGROUPSET;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateClusterGroup(hcluster: *const _HCLUSTER, lpszgroupname: super::super::Foundation::PWSTR) -> *mut _HGROUP;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateClusterGroupEx(hcluster: *const _HCLUSTER, lpszgroupname: super::super::Foundation::PWSTR, pgroupinfo: *const CLUSTER_CREATE_GROUP_INFO) -> *mut _HGROUP;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateClusterGroupSet(hcluster: *const _HCLUSTER, groupsetname: super::super::Foundation::PWSTR) -> *mut _HGROUPSET;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateClusterNameAccount(hcluster: *const _HCLUSTER, pconfig: *const CREATE_CLUSTER_NAME_ACCOUNT, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> u32;
    pub fn CreateClusterNotifyPort(hchange: *const _HCHANGE, hcluster: *const _HCLUSTER, dwfilter: u32, dwnotifykey: usize) -> *mut _HCHANGE;
    pub fn CreateClusterNotifyPortV2(hchange: *const _HCHANGE, hcluster: *const _HCLUSTER, filters: *const NOTIFY_FILTER_AND_TYPE, dwfiltercount: u32, dwnotifykey: usize) -> *mut _HCHANGE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateClusterResource(hgroup: *const _HGROUP, lpszresourcename: super::super::Foundation::PWSTR, lpszresourcetype: super::super::Foundation::PWSTR, dwflags: u32) -> *mut _HRESOURCE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateClusterResourceType(hcluster: *const _HCLUSTER, lpszresourcetypename: super::super::Foundation::PWSTR, lpszdisplayname: super::super::Foundation::PWSTR, lpszresourcetypedll: super::super::Foundation::PWSTR, dwlooksalivepollinterval: u32, dwisalivepollinterval: u32) -> u32;
    pub fn DeleteClusterGroup(hgroup: *const _HGROUP) -> u32;
    pub fn DeleteClusterGroupSet(hgroupset: *const _HGROUPSET) -> u32;
    pub fn DeleteClusterResource(hresource: *const _HRESOURCE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteClusterResourceType(hcluster: *const _HCLUSTER, lpszresourcetypename: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyCluster(hcluster: *const _HCLUSTER, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void, fdeletevirtualcomputerobjects: super::super::Foundation::BOOL) -> u32;
    pub fn DestroyClusterGroup(hgroup: *const _HGROUP) -> u32;
    pub fn DetermineCNOResTypeFromCluster(hcluster: *const _HCLUSTER, pcnorestype: *mut CLUSTER_MGMT_POINT_RESTYPE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DetermineCNOResTypeFromNodelist(cnodes: u32, ppsznodenames: *const super::super::Foundation::PWSTR, pcnorestype: *mut CLUSTER_MGMT_POINT_RESTYPE) -> u32;
    pub fn DetermineClusterCloudTypeFromCluster(hcluster: *const _HCLUSTER, pcloudtype: *mut CLUSTER_CLOUD_TYPE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DetermineClusterCloudTypeFromNodelist(cnodes: u32, ppsznodenames: *const super::super::Foundation::PWSTR, pcloudtype: *mut CLUSTER_CLOUD_TYPE) -> u32;
    pub fn EvictClusterNode(hnode: *const _HNODE) -> u32;
    pub fn EvictClusterNodeEx(hnode: *const _HNODE, dwtimeout: u32, phrcleanupstatus: *mut ::windows_sys::core::HRESULT) -> u32;
    pub fn FailClusterResource(hresource: *const _HRESOURCE) -> u32;
    pub fn FreeClusterCrypt(pcryptinfo: *const ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeClusterHealthFault(clusterhealthfault: *mut CLUSTER_HEALTH_FAULT) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeClusterHealthFaultArray(clusterhealthfaultarray: *mut CLUSTER_HEALTH_FAULT_ARRAY) -> u32;
    pub fn GetClusterFromGroup(hgroup: *const _HGROUP) -> *mut _HCLUSTER;
    pub fn GetClusterFromNetInterface(hnetinterface: *const _HNETINTERFACE) -> *mut _HCLUSTER;
    pub fn GetClusterFromNetwork(hnetwork: *const _HNETWORK) -> *mut _HCLUSTER;
    pub fn GetClusterFromNode(hnode: *const _HNODE) -> *mut _HCLUSTER;
    pub fn GetClusterFromResource(hresource: *const _HRESOURCE) -> *mut _HCLUSTER;
    #[cfg(feature = "Win32_System_Registry")]
    pub fn GetClusterGroupKey(hgroup: *const _HGROUP, samdesired: u32) -> super::super::System::Registry::HKEY;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterGroupState(hgroup: *const _HGROUP, lpsznodename: super::super::Foundation::PWSTR, lpcchnodename: *mut u32) -> CLUSTER_GROUP_STATE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterInformation(hcluster: *const _HCLUSTER, lpszclustername: super::super::Foundation::PWSTR, lpcchclustername: *mut u32, lpclusterinfo: *mut CLUSTERVERSIONINFO) -> u32;
    #[cfg(feature = "Win32_System_Registry")]
    pub fn GetClusterKey(hcluster: *const _HCLUSTER, samdesired: u32) -> super::super::System::Registry::HKEY;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterNetInterface(hcluster: *const _HCLUSTER, lpsznodename: super::super::Foundation::PWSTR, lpsznetworkname: super::super::Foundation::PWSTR, lpszinterfacename: super::super::Foundation::PWSTR, lpcchinterfacename: *mut u32) -> u32;
    #[cfg(feature = "Win32_System_Registry")]
    pub fn GetClusterNetInterfaceKey(hnetinterface: *const _HNETINTERFACE, samdesired: u32) -> super::super::System::Registry::HKEY;
    pub fn GetClusterNetInterfaceState(hnetinterface: *const _HNETINTERFACE) -> CLUSTER_NETINTERFACE_STATE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterNetworkId(hnetwork: *const _HNETWORK, lpsznetworkid: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
    #[cfg(feature = "Win32_System_Registry")]
    pub fn GetClusterNetworkKey(hnetwork: *const _HNETWORK, samdesired: u32) -> super::super::System::Registry::HKEY;
    pub fn GetClusterNetworkState(hnetwork: *const _HNETWORK) -> CLUSTER_NETWORK_STATE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterNodeId(hnode: *const _HNODE, lpsznodeid: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
    #[cfg(feature = "Win32_System_Registry")]
    pub fn GetClusterNodeKey(hnode: *const _HNODE, samdesired: u32) -> super::super::System::Registry::HKEY;
    pub fn GetClusterNodeState(hnode: *const _HNODE) -> CLUSTER_NODE_STATE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterNotify(hchange: *const _HCHANGE, lpdwnotifykey: *mut usize, lpdwfiltertype: *mut u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32, dwmilliseconds: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterNotifyV2(hchange: *const _HCHANGE, lpdwnotifykey: *mut usize, pfilterandtype: *mut NOTIFY_FILTER_AND_TYPE, buffer: *mut u8, lpbbuffersize: *mut u32, lpszobjectid: super::super::Foundation::PWSTR, lpcchobjectid: *mut u32, lpszparentid: super::super::Foundation::PWSTR, lpcchparentid: *mut u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32, lpsztype: super::super::Foundation::PWSTR, lpcchtype: *mut u32, dwmilliseconds: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterQuorumResource(hcluster: *const _HCLUSTER, lpszresourcename: super::super::Foundation::PWSTR, lpcchresourcename: *mut u32, lpszdevicename: super::super::Foundation::PWSTR, lpcchdevicename: *mut u32, lpdwmaxquorumlogsize: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterResourceDependencyExpression(hresource: *const _HRESOURCE, lpszdependencyexpression: super::super::Foundation::PWSTR, lpcchdependencyexpression: *mut u32) -> u32;
    #[cfg(feature = "Win32_System_Registry")]
    pub fn GetClusterResourceKey(hresource: *const _HRESOURCE, samdesired: u32) -> super::super::System::Registry::HKEY;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterResourceNetworkName(hresource: *const _HRESOURCE, lpbuffer: super::super::Foundation::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClusterResourceState(hresource: *const _HRESOURCE, lpsznodename: super::super::Foundation::PWSTR, lpcchnodename: *mut u32, lpszgroupname: super::super::Foundation::PWSTR, lpcchgroupname: *mut u32) -> CLUSTER_RESOURCE_STATE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn GetClusterResourceTypeKey(hcluster: *const _HCLUSTER, lpsztypename: super::super::Foundation::PWSTR, samdesired: u32) -> super::super::System::Registry::HKEY;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNodeCloudTypeDW(ppsznodename: super::super::Foundation::PWSTR, nodecloudtype: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNodeClusterState(lpsznodename: super::super::Foundation::PWSTR, pdwclusterstate: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNotifyEventHandle(hchange: *const _HCHANGE, lphtargetevent: *mut super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeClusterHealthFault(clusterhealthfault: *mut CLUSTER_HEALTH_FAULT) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeClusterHealthFaultArray(clusterhealthfaultarray: *mut CLUSTER_HEALTH_FAULT_ARRAY) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsFileOnClusterSharedVolume(lpszpathname: super::super::Foundation::PWSTR, pbfileisonsharedvolume: *mut super::super::Foundation::BOOL) -> u32;
    pub fn MoveClusterGroup(hgroup: *const _HGROUP, hdestinationnode: *const _HNODE) -> u32;
    pub fn MoveClusterGroupEx(hgroup: *const _HGROUP, hdestinationnode: *const _HNODE, dwmoveflags: u32, lpinbuffer: *const u8, cbinbuffersize: u32) -> u32;
    pub fn OfflineClusterGroup(hgroup: *const _HGROUP) -> u32;
    pub fn OfflineClusterGroupEx(hgroup: *const _HGROUP, dwofflineflags: u32, lpinbuffer: *const u8, cbinbuffersize: u32) -> u32;
    pub fn OfflineClusterResource(hresource: *const _HRESOURCE) -> u32;
    pub fn OfflineClusterResourceEx(hresource: *const _HRESOURCE, dwofflineflags: u32, lpinbuffer: *const u8, cbinbuffersize: u32) -> u32;
    pub fn OnlineClusterGroup(hgroup: *const _HGROUP, hdestinationnode: *const _HNODE) -> u32;
    pub fn OnlineClusterGroupEx(hgroup: *const _HGROUP, hdestinationnode: *const _HNODE, dwonlineflags: u32, lpinbuffer: *const u8, cbinbuffersize: u32) -> u32;
    pub fn OnlineClusterResource(hresource: *const _HRESOURCE) -> u32;
    pub fn OnlineClusterResourceEx(hresource: *const _HRESOURCE, dwonlineflags: u32, lpinbuffer: *const u8, cbinbuffersize: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenCluster(lpszclustername: super::super::Foundation::PWSTR) -> *mut _HCLUSTER;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterCryptProvider(lpszresource: super::super::Foundation::PWSTR, lpszprovider: *const i8, dwtype: u32, dwflags: u32) -> *mut _HCLUSCRYPTPROVIDER;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterCryptProviderEx(lpszresource: super::super::Foundation::PWSTR, lpszkeyname: super::super::Foundation::PWSTR, lpszprovider: *const i8, dwtype: u32, dwflags: u32) -> *mut _HCLUSCRYPTPROVIDER;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterEx(lpszclustername: super::super::Foundation::PWSTR, desiredaccess: u32, grantedaccess: *mut u32) -> *mut _HCLUSTER;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterGroup(hcluster: *const _HCLUSTER, lpszgroupname: super::super::Foundation::PWSTR) -> *mut _HGROUP;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterGroupEx(hcluster: *const _HCLUSTER, lpszgroupname: super::super::Foundation::PWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HGROUP;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterGroupSet(hcluster: *const _HCLUSTER, lpszgroupsetname: super::super::Foundation::PWSTR) -> *mut _HGROUPSET;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterNetInterface(hcluster: *const _HCLUSTER, lpszinterfacename: super::super::Foundation::PWSTR) -> *mut _HNETINTERFACE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterNetInterfaceEx(hcluster: *const _HCLUSTER, lpszinterfacename: super::super::Foundation::PWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HNETINTERFACE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterNetwork(hcluster: *const _HCLUSTER, lpsznetworkname: super::super::Foundation::PWSTR) -> *mut _HNETWORK;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterNetworkEx(hcluster: *const _HCLUSTER, lpsznetworkname: super::super::Foundation::PWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HNETWORK;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterNode(hcluster: *const _HCLUSTER, lpsznodename: super::super::Foundation::PWSTR) -> *mut _HNODE;
    pub fn OpenClusterNodeById(hcluster: *const _HCLUSTER, nodeid: u32) -> *mut _HNODE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterNodeEx(hcluster: *const _HCLUSTER, lpsznodename: super::super::Foundation::PWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HNODE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterResource(hcluster: *const _HCLUSTER, lpszresourcename: super::super::Foundation::PWSTR) -> *mut _HRESOURCE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClusterResourceEx(hcluster: *const _HCLUSTER, lpszresourcename: super::super::Foundation::PWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HRESOURCE;
    pub fn PauseClusterNode(hnode: *const _HNODE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PauseClusterNodeEx(hnode: *const _HNODE, bdrainnode: super::super::Foundation::BOOL, dwpauseflags: u32, hnodedraintarget: *const _HNODE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryAppInstanceVersion(appinstanceid: *const ::windows_sys::core::GUID, instanceversionhigh: *mut u64, instanceversionlow: *mut u64, versionstatus: *mut super::super::Foundation::NTSTATUS) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterAppInstance(processhandle: super::super::Foundation::HANDLE, appinstanceid: *const ::windows_sys::core::GUID, childreninheritappinstance: super::super::Foundation::BOOL) -> u32;
    pub fn RegisterAppInstanceVersion(appinstanceid: *const ::windows_sys::core::GUID, instanceversionhigh: u64, instanceversionlow: u64) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterClusterNotify(hchange: *const _HCHANGE, dwfiltertype: u32, hobject: super::super::Foundation::HANDLE, dwnotifykey: usize) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterClusterNotifyV2(hchange: *const _HCHANGE, filter: NOTIFY_FILTER_AND_TYPE, hobject: super::super::Foundation::HANDLE, dwnotifykey: usize) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterClusterResourceTypeNotifyV2(hchange: *const _HCHANGE, hcluster: *const _HCLUSTER, flags: i64, restypename: super::super::Foundation::PWSTR, dwnotifykey: usize) -> u32;
    pub fn RemoveClusterGroupDependency(hgroup: *const _HGROUP, hdependson: *const _HGROUP) -> u32;
    pub fn RemoveClusterGroupSetDependency(hgroupset: *const _HGROUPSET, hdependson: *const _HGROUPSET) -> u32;
    pub fn RemoveClusterGroupToGroupSetDependency(hgroup: *const _HGROUP, hdependson: *const _HGROUPSET) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveClusterNameAccount(hcluster: *const _HCLUSTER, bdeletecomputerobjects: super::super::Foundation::BOOL) -> u32;
    pub fn RemoveClusterResourceDependency(hresource: *const _HRESOURCE, hdependson: *const _HRESOURCE) -> u32;
    pub fn RemoveClusterResourceNode(hresource: *const _HRESOURCE, hnode: *const _HNODE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveClusterStorageNode(hcluster: *const _HCLUSTER, lpszclusterstorageenclosurename: super::super::Foundation::PWSTR, dwtimeout: u32, dwflags: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveCrossClusterGroupSetDependency(hdependentgroupset: *const _HGROUPSET, lpremoteclustername: super::super::Foundation::PWSTR, lpremotegroupsetname: super::super::Foundation::PWSTR) -> u32;
    pub fn RemoveResourceFromClusterSharedVolumes(hresource: *const _HRESOURCE) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilAddUnknownProperties(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut ::core::ffi::c_void, pcboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilCreateDirectoryTree(pszpath: super::super::Foundation::PWSTR) -> u32;
    pub fn ResUtilDupGroup(group: *mut _HGROUP, copy: *mut *mut _HGROUP) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilDupParameterBlock(poutparams: *mut u8, pinparams: *const u8, ppropertytable: *const RESUTIL_PROPERTY_ITEM) -> u32;
    pub fn ResUtilDupResource(group: *mut _HRESOURCE, copy: *mut *mut _HRESOURCE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilDupString(pszinstring: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    pub fn ResUtilEnumGroups(hcluster: *mut _HCLUSTER, hself: *mut _HGROUP, prescallback: LPGROUP_CALLBACK_EX, pparameter: *mut ::core::ffi::c_void) -> u32;
    pub fn ResUtilEnumGroupsEx(hcluster: *mut _HCLUSTER, hself: *mut _HGROUP, grouptype: CLUSGROUP_TYPE, prescallback: LPGROUP_CALLBACK_EX, pparameter: *mut ::core::ffi::c_void) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilEnumPrivateProperties(hkeyclusterkey: super::super::System::Registry::HKEY, pszoutproperties: super::super::Foundation::PWSTR, cboutpropertiessize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilEnumProperties(ppropertytable: *const RESUTIL_PROPERTY_ITEM, pszoutproperties: super::super::Foundation::PWSTR, cboutpropertiessize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilEnumResources(hself: *mut _HRESOURCE, lpszrestypename: super::super::Foundation::PWSTR, prescallback: LPRESOURCE_CALLBACK, pparameter: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilEnumResourcesEx(hcluster: *mut _HCLUSTER, hself: *mut _HRESOURCE, lpszrestypename: super::super::Foundation::PWSTR, prescallback: LPRESOURCE_CALLBACK_EX, pparameter: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilEnumResourcesEx2(hcluster: *mut _HCLUSTER, hself: *mut _HRESOURCE, lpszrestypename: super::super::Foundation::PWSTR, prescallback: LPRESOURCE_CALLBACK_EX, pparameter: *mut ::core::ffi::c_void, dwdesiredaccess: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilExpandEnvironmentStrings(pszsrc: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindBinaryProperty(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: super::super::Foundation::PWSTR, pbpropertyvalue: *mut *mut u8, pcbpropertyvaluesize: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindDependentDiskResourceDriveLetter(hcluster: *const _HCLUSTER, hresource: *const _HRESOURCE, pszdriveletter: super::super::Foundation::PWSTR, pcchdriveletter: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindDwordProperty(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: super::super::Foundation::PWSTR, pdwpropertyvalue: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindExpandSzProperty(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: super::super::Foundation::PWSTR, pszpropertyvalue: *mut super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindExpandedSzProperty(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: super::super::Foundation::PWSTR, pszpropertyvalue: *mut super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindFileTimeProperty(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: super::super::Foundation::PWSTR, pftpropertyvalue: *mut super::super::Foundation::FILETIME) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindLongProperty(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: super::super::Foundation::PWSTR, plpropertyvalue: *mut i32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindMultiSzProperty(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: super::super::Foundation::PWSTR, pszpropertyvalue: *mut super::super::Foundation::PWSTR, pcbpropertyvaluesize: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindSzProperty(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: super::super::Foundation::PWSTR, pszpropertyvalue: *mut super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFindULargeIntegerProperty(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: super::super::Foundation::PWSTR, plpropertyvalue: *mut u64) -> u32;
    pub fn ResUtilFreeEnvironment(lpenvironment: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilFreeParameterBlock(poutparams: *mut u8, pinparams: *const u8, ppropertytable: *const RESUTIL_PROPERTY_ITEM);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilGetAllProperties(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut ::core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32;
    pub fn ResUtilGetBinaryProperty(ppboutvalue: *mut *mut u8, pcboutvaluesize: *mut u32, pvaluestruct: *const CLUSPROP_BINARY, pboldvalue: *const u8, cboldvaluesize: u32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilGetBinaryValue(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, ppboutvalue: *mut *mut u8, pcboutvaluesize: *mut u32) -> u32;
    pub fn ResUtilGetClusterGroupType(hgroup: *mut _HGROUP, grouptype: *mut CLUSGROUP_TYPE) -> u32;
    pub fn ResUtilGetClusterId(hcluster: *mut _HCLUSTER, guid: *mut ::windows_sys::core::GUID) -> u32;
    pub fn ResUtilGetClusterRoleState(hcluster: *const _HCLUSTER, eclusterrole: CLUSTER_ROLE) -> CLUSTER_ROLE_STATE;
    pub fn ResUtilGetCoreClusterResources(hcluster: *const _HCLUSTER, phclusternameresource: *mut *mut _HRESOURCE, phclusteripaddressresource: *mut *mut _HRESOURCE, phclusterquorumresource: *mut *mut _HRESOURCE) -> u32;
    pub fn ResUtilGetCoreClusterResourcesEx(hclusterin: *const _HCLUSTER, phclusternameresourceout: *mut *mut _HRESOURCE, phclusterquorumresourceout: *mut *mut _HRESOURCE, dwdesiredaccess: u32) -> u32;
    pub fn ResUtilGetCoreGroup(hcluster: *mut _HCLUSTER) -> *mut _HGROUP;
    pub fn ResUtilGetDwordProperty(pdwoutvalue: *mut u32, pvaluestruct: *const CLUSPROP_DWORD, dwoldvalue: u32, dwminimum: u32, dwmaximum: u32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilGetDwordValue(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, pdwoutvalue: *mut u32, dwdefaultvalue: u32) -> u32;
    pub fn ResUtilGetEnvironmentWithNetName(hresource: *const _HRESOURCE) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetFileTimeProperty(pftoutvalue: *mut super::super::Foundation::FILETIME, pvaluestruct: *const CLUSPROP_FILETIME, ftoldvalue: super::super::Foundation::FILETIME, ftminimum: super::super::Foundation::FILETIME, ftmaximum: super::super::Foundation::FILETIME, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32;
    pub fn ResUtilGetLongProperty(ploutvalue: *mut i32, pvaluestruct: *const CLUSPROP_LONG, loldvalue: i32, lminimum: i32, lmaximum: i32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetMultiSzProperty(ppszoutvalue: *mut super::super::Foundation::PWSTR, pcboutvaluesize: *mut u32, pvaluestruct: *const CLUSPROP_SZ, pszoldvalue: super::super::Foundation::PWSTR, cboldvaluesize: u32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32;
    #[cfg(feature = "Win32_System_Registry")]
    pub fn ResUtilGetPrivateProperties(hkeyclusterkey: super::super::System::Registry::HKEY, poutpropertylist: *mut ::core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilGetProperties(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut ::core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilGetPropertiesToParameterBlock(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutparams: *mut u8, bcheckforrequiredproperties: super::super::Foundation::BOOL, psznameofpropinerror: *mut super::super::Foundation::PWSTR) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilGetProperty(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytableitem: *const RESUTIL_PROPERTY_ITEM, poutpropertyitem: *mut *mut ::core::ffi::c_void, pcboutpropertyitemsize: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetPropertyFormats(ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertyformatlist: *mut ::core::ffi::c_void, cbpropertyformatlistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilGetPropertySize(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytableitem: *const RESUTIL_PROPERTY_ITEM, pcboutpropertylistsize: *mut u32, pnpropertycount: *mut u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilGetQwordValue(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, pqwoutvalue: *mut u64, qwdefaultvalue: u64) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceDependency(hself: super::super::Foundation::HANDLE, lpszresourcetype: super::super::Foundation::PWSTR) -> *mut _HRESOURCE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceDependencyByClass(hcluster: *mut _HCLUSTER, hself: super::super::Foundation::HANDLE, prci: *mut CLUS_RESOURCE_CLASS_INFO, brecurse: super::super::Foundation::BOOL) -> *mut _HRESOURCE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceDependencyByClassEx(hcluster: *mut _HCLUSTER, hself: super::super::Foundation::HANDLE, prci: *mut CLUS_RESOURCE_CLASS_INFO, brecurse: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> *mut _HRESOURCE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceDependencyByName(hcluster: *mut _HCLUSTER, hself: super::super::Foundation::HANDLE, lpszresourcetype: super::super::Foundation::PWSTR, brecurse: super::super::Foundation::BOOL) -> *mut _HRESOURCE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceDependencyByNameEx(hcluster: *mut _HCLUSTER, hself: super::super::Foundation::HANDLE, lpszresourcetype: super::super::Foundation::PWSTR, brecurse: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> *mut _HRESOURCE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceDependencyEx(hself: super::super::Foundation::HANDLE, lpszresourcetype: super::super::Foundation::PWSTR, dwdesiredaccess: u32) -> *mut _HRESOURCE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceDependentIPAddressProps(hresource: *const _HRESOURCE, pszaddress: super::super::Foundation::PWSTR, pcchaddress: *mut u32, pszsubnetmask: super::super::Foundation::PWSTR, pcchsubnetmask: *mut u32, psznetwork: super::super::Foundation::PWSTR, pcchnetwork: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceName(hresource: *const _HRESOURCE, pszresourcename: super::super::Foundation::PWSTR, pcchresourcenameinout: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceNameDependency(lpszresourcename: super::super::Foundation::PWSTR, lpszresourcetype: super::super::Foundation::PWSTR) -> *mut _HRESOURCE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetResourceNameDependencyEx(lpszresourcename: super::super::Foundation::PWSTR, lpszresourcetype: super::super::Foundation::PWSTR, dwdesiredaccess: u32) -> *mut _HRESOURCE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGetSzProperty(ppszoutvalue: *mut super::super::Foundation::PWSTR, pvaluestruct: *const CLUSPROP_SZ, pszoldvalue: super::super::Foundation::PWSTR, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilGetSzValue(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilGroupsEqual(hself: *mut _HGROUP, hgroup: *mut _HGROUP, pequal: *mut super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilIsPathValid(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilIsResourceClassEqual(prci: *mut CLUS_RESOURCE_CLASS_INFO, hresource: *mut _HRESOURCE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilLeftPaxosIsLessThanRight(left: *const PaxosTagCStruct, right: *const PaxosTagCStruct) -> super::super::Foundation::BOOL;
    pub fn ResUtilNodeEnum(hcluster: *mut _HCLUSTER, pnodecallback: LPNODE_CALLBACK, pparameter: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilPaxosComparer(left: *const PaxosTagCStruct, right: *const PaxosTagCStruct) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilPropertyListFromParameterBlock(ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut ::core::ffi::c_void, pcboutpropertylistsize: *mut u32, pinparams: *const u8, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilRemoveResourceServiceEnvironment(pszservicename: super::super::Foundation::PWSTR, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32;
    pub fn ResUtilResourceDepEnum(hself: *mut _HRESOURCE, enumtype: u32, prescallback: LPRESOURCE_CALLBACK_EX, pparameter: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilResourceTypesEqual(lpszresourcetypename: super::super::Foundation::PWSTR, hresource: *mut _HRESOURCE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilResourcesEqual(hself: *mut _HRESOURCE, hresource: *mut _HRESOURCE) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetBinaryValue(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, pbnewvalue: *const u8, cbnewvaluesize: u32, ppboutvalue: *mut *mut u8, pcboutvaluesize: *mut u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetDwordValue(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, dwnewvalue: u32, pdwoutvalue: *mut u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetExpandSzValue(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, psznewvalue: super::super::Foundation::PWSTR, ppszoutstring: *mut super::super::Foundation::PWSTR) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetMultiSzValue(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, psznewvalue: super::super::Foundation::PWSTR, cbnewvaluesize: u32, ppszoutvalue: *mut super::super::Foundation::PWSTR, pcboutvaluesize: *mut u32) -> u32;
    #[cfg(feature = "Win32_System_Registry")]
    pub fn ResUtilSetPrivatePropertyList(hkeyclusterkey: super::super::System::Registry::HKEY, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetPropertyParameterBlock(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, pinparams: *const u8, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, poutparams: *mut u8) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetPropertyParameterBlockEx(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, pinparams: *const u8, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, bforcewrite: super::super::Foundation::BOOL, poutparams: *mut u8) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetPropertyTable(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, ballowunknownproperties: super::super::Foundation::BOOL, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, poutparams: *mut u8) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetPropertyTableEx(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, ballowunknownproperties: super::super::Foundation::BOOL, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, bforcewrite: super::super::Foundation::BOOL, poutparams: *mut u8) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetQwordValue(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, qwnewvalue: u64, pqwoutvalue: *mut u64) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilSetResourceServiceEnvironment(pszservicename: super::super::Foundation::PWSTR, hresource: *mut _HRESOURCE, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn ResUtilSetResourceServiceStartParameters(pszservicename: super::super::Foundation::PWSTR, schscmhandle: super::super::Security::SC_HANDLE, phservice: *mut isize, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn ResUtilSetResourceServiceStartParametersEx(pszservicename: super::super::Foundation::PWSTR, schscmhandle: super::super::Security::SC_HANDLE, phservice: *mut isize, dwdesiredaccess: u32, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetSzValue(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, psznewvalue: super::super::Foundation::PWSTR, ppszoutstring: *mut super::super::Foundation::PWSTR) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetUnknownProperties(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilSetValueEx(hkeyclusterkey: super::super::System::Registry::HKEY, valuename: super::super::Foundation::PWSTR, valuetype: u32, valuedata: *const u8, valuesize: u32, flags: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilStartResourceService(pszservicename: super::super::Foundation::PWSTR, phservicehandle: *mut isize) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilStopResourceService(pszservicename: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Security")]
    pub fn ResUtilStopService(hservicehandle: super::super::Security::SC_HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilTerminateServiceProcessFromResDll(dwservicepid: u32, boffline: super::super::Foundation::BOOL, pdwresourcestate: *mut u32, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32;
    pub fn ResUtilVerifyPrivatePropertyList(pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilVerifyPropertyTable(ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, ballowunknownproperties: super::super::Foundation::BOOL, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, poutparams: *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResUtilVerifyResourceService(pszservicename: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Security")]
    pub fn ResUtilVerifyService(hservicehandle: super::super::Security::SC_HANDLE) -> u32;
    pub fn ResUtilVerifyShutdownSafe(flags: u32, reason: u32, presult: *mut u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ResUtilsDeleteKeyTree(key: super::super::System::Registry::HKEY, keyname: super::super::Foundation::PWSTR, treatnokeyaserror: super::super::Foundation::BOOL) -> u32;
    pub fn ResetAllAppInstanceVersions() -> u32;
    pub fn RestartClusterResource(hresource: *const _HRESOURCE, dwflags: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RestoreClusterDatabase(lpszpathname: super::super::Foundation::PWSTR, bforce: super::super::Foundation::BOOL, lpszquorumdriveletter: super::super::Foundation::PWSTR) -> u32;
    pub fn ResumeClusterNode(hnode: *const _HNODE) -> u32;
    pub fn ResumeClusterNodeEx(hnode: *const _HNODE, eresumefailbacktype: CLUSTER_NODE_RESUME_FAILBACK_TYPE, dwresumeflagsreserved: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetAppInstanceCsvFlags(processhandle: super::super::Foundation::HANDLE, mask: u32, flags: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClusterGroupName(hgroup: *const _HGROUP, lpszgroupname: super::super::Foundation::PWSTR) -> u32;
    pub fn SetClusterGroupNodeList(hgroup: *const _HGROUP, nodecount: u32, nodelist: *const *const _HNODE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClusterGroupSetDependencyExpression(hgroupset: *const _HGROUPSET, lpszdependencyexprssion: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClusterName(hcluster: *const _HCLUSTER, lpsznewclustername: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClusterNetworkName(hnetwork: *const _HNETWORK, lpszname: super::super::Foundation::PWSTR) -> u32;
    pub fn SetClusterNetworkPriorityOrder(hcluster: *const _HCLUSTER, networkcount: u32, networklist: *const *const _HNETWORK) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClusterQuorumResource(hresource: *const _HRESOURCE, lpszdevicename: super::super::Foundation::PWSTR, dwmaxquologsize: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClusterResourceDependencyExpression(hresource: *const _HRESOURCE, lpszdependencyexpression: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClusterResourceName(hresource: *const _HRESOURCE, lpszresourcename: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClusterServiceAccountPassword(lpszclustername: super::super::Foundation::PWSTR, lpsznewpassword: super::super::Foundation::PWSTR, dwflags: u32, lpreturnstatusbuffer: *mut CLUSTER_SET_PASSWORD_STATUS, lpcbreturnstatusbuffersize: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetGroupDependencyExpression(hgroup: *const _HGROUP, lpszdependencyexpression: super::super::Foundation::PWSTR) -> u32;
}
pub const BitLockerDecrypted: i32 = 4i32;
pub const BitLockerDecrypting: i32 = 16i32;
pub const BitLockerEnabled: i32 = 1i32;
pub const BitLockerPaused: i32 = 64i32;
pub const BitLockerStopped: i32 = 128i32;
pub const BitlockerEncrypted: i32 = 8i32;
pub const BitlockerEncrypting: i32 = 32i32;
pub const CA_UPGRADE_VERSION: u32 = 1u32;
pub const CLCTL_CLUSTER_BASE: u32 = 0u32;
pub const CLCTL_UNKNOWN: i32 = 0i32;
pub const CLCTL_GET_CHARACTERISTICS: i32 = 5i32;
pub const CLCTL_GET_FLAGS: i32 = 9i32;
pub const CLCTL_GET_CLASS_INFO: i32 = 13i32;
pub const CLCTL_GET_REQUIRED_DEPENDENCIES: i32 = 17i32;
pub const CLCTL_GET_ARB_TIMEOUT: i32 = 21i32;
pub const CLCTL_GET_FAILURE_INFO: i32 = 25i32;
pub const CLCTL_GET_NAME: i32 = 41i32;
pub const CLCTL_GET_RESOURCE_TYPE: i32 = 45i32;
pub const CLCTL_GET_NODE: i32 = 49i32;
pub const CLCTL_GET_NETWORK: i32 = 53i32;
pub const CLCTL_GET_ID: i32 = 57i32;
pub const CLCTL_GET_FQDN: i32 = 61i32;
pub const CLCTL_GET_CLUSTER_SERVICE_ACCOUNT_NAME: i32 = 65i32;
pub const CLCTL_CHECK_VOTER_EVICT: i32 = 69i32;
pub const CLCTL_CHECK_VOTER_DOWN: i32 = 73i32;
pub const CLCTL_SHUTDOWN: i32 = 77i32;
pub const CLCTL_ENUM_COMMON_PROPERTIES: i32 = 81i32;
pub const CLCTL_GET_RO_COMMON_PROPERTIES: i32 = 85i32;
pub const CLCTL_GET_COMMON_PROPERTIES: i32 = 89i32;
pub const CLCTL_SET_COMMON_PROPERTIES: i32 = 4194398i32;
pub const CLCTL_VALIDATE_COMMON_PROPERTIES: i32 = 97i32;
pub const CLCTL_GET_COMMON_PROPERTY_FMTS: i32 = 101i32;
pub const CLCTL_GET_COMMON_RESOURCE_PROPERTY_FMTS: i32 = 105i32;
pub const CLCTL_ENUM_PRIVATE_PROPERTIES: i32 = 121i32;
pub const CLCTL_GET_RO_PRIVATE_PROPERTIES: i32 = 125i32;
pub const CLCTL_GET_PRIVATE_PROPERTIES: i32 = 129i32;
pub const CLCTL_SET_PRIVATE_PROPERTIES: i32 = 4194438i32;
pub const CLCTL_VALIDATE_PRIVATE_PROPERTIES: i32 = 137i32;
pub const CLCTL_GET_PRIVATE_PROPERTY_FMTS: i32 = 141i32;
pub const CLCTL_GET_PRIVATE_RESOURCE_PROPERTY_FMTS: i32 = 145i32;
pub const CLCTL_ADD_REGISTRY_CHECKPOINT: i32 = 4194466i32;
pub const CLCTL_DELETE_REGISTRY_CHECKPOINT: i32 = 4194470i32;
pub const CLCTL_GET_REGISTRY_CHECKPOINTS: i32 = 169i32;
pub const CLCTL_ADD_CRYPTO_CHECKPOINT: i32 = 4194478i32;
pub const CLCTL_DELETE_CRYPTO_CHECKPOINT: i32 = 4194482i32;
pub const CLCTL_GET_CRYPTO_CHECKPOINTS: i32 = 181i32;
pub const CLCTL_RESOURCE_UPGRADE_DLL: i32 = 4194490i32;
pub const CLCTL_ADD_REGISTRY_CHECKPOINT_64BIT: i32 = 4194494i32;
pub const CLCTL_ADD_REGISTRY_CHECKPOINT_32BIT: i32 = 4194498i32;
pub const CLCTL_GET_LOADBAL_PROCESS_LIST: i32 = 201i32;
pub const CLCTL_SET_ACCOUNT_ACCESS: i32 = 4194546i32;
pub const CLCTL_GET_NETWORK_NAME: i32 = 361i32;
pub const CLCTL_NETNAME_GET_VIRTUAL_SERVER_TOKEN: i32 = 365i32;
pub const CLCTL_NETNAME_REGISTER_DNS_RECORDS: i32 = 370i32;
pub const CLCTL_GET_DNS_NAME: i32 = 373i32;
pub const CLCTL_NETNAME_SET_PWD_INFO: i32 = 378i32;
pub const CLCTL_NETNAME_DELETE_CO: i32 = 382i32;
pub const CLCTL_NETNAME_VALIDATE_VCO: i32 = 385i32;
pub const CLCTL_NETNAME_RESET_VCO: i32 = 389i32;
pub const CLCTL_NETNAME_REPAIR_VCO: i32 = 397i32;
pub const CLCTL_STORAGE_GET_DISK_INFO: i32 = 401i32;
pub const CLCTL_STORAGE_GET_AVAILABLE_DISKS: i32 = 405i32;
pub const CLCTL_STORAGE_IS_PATH_VALID: i32 = 409i32;
pub const CLCTL_STORAGE_SYNC_CLUSDISK_DB: i32 = 4194718i32;
pub const CLCTL_STORAGE_GET_DISK_NUMBER_INFO: i32 = 417i32;
pub const CLCTL_QUERY_DELETE: i32 = 441i32;
pub const CLCTL_IPADDRESS_RENEW_LEASE: i32 = 4194750i32;
pub const CLCTL_IPADDRESS_RELEASE_LEASE: i32 = 4194754i32;
pub const CLCTL_QUERY_MAINTENANCE_MODE: i32 = 481i32;
pub const CLCTL_SET_MAINTENANCE_MODE: i32 = 4194790i32;
pub const CLCTL_STORAGE_SET_DRIVELETTER: i32 = 4194794i32;
pub const CLCTL_STORAGE_GET_DRIVELETTERS: i32 = 493i32;
pub const CLCTL_STORAGE_GET_DISK_INFO_EX: i32 = 497i32;
pub const CLCTL_STORAGE_GET_AVAILABLE_DISKS_EX: i32 = 501i32;
pub const CLCTL_STORAGE_GET_DISK_INFO_EX2: i32 = 505i32;
pub const CLCTL_STORAGE_GET_CLUSPORT_DISK_COUNT: i32 = 509i32;
pub const CLCTL_STORAGE_REMAP_DRIVELETTER: i32 = 513i32;
pub const CLCTL_STORAGE_GET_DISKID: i32 = 517i32;
pub const CLCTL_STORAGE_IS_CLUSTERABLE: i32 = 521i32;
pub const CLCTL_STORAGE_REMOVE_VM_OWNERSHIP: i32 = 4194830i32;
pub const CLCTL_STORAGE_GET_MOUNTPOINTS: i32 = 529i32;
pub const CLCTL_STORAGE_GET_DIRTY: i32 = 537i32;
pub const CLCTL_STORAGE_GET_SHARED_VOLUME_INFO: i32 = 549i32;
pub const CLCTL_STORAGE_IS_CSV_FILE: i32 = 553i32;
pub const CLCTL_STORAGE_GET_RESOURCEID: i32 = 557i32;
pub const CLCTL_VALIDATE_PATH: i32 = 561i32;
pub const CLCTL_VALIDATE_NETNAME: i32 = 565i32;
pub const CLCTL_VALIDATE_DIRECTORY: i32 = 569i32;
pub const CLCTL_BATCH_BLOCK_KEY: i32 = 574i32;
pub const CLCTL_BATCH_UNBLOCK_KEY: i32 = 577i32;
pub const CLCTL_FILESERVER_SHARE_ADD: i32 = 4194886i32;
pub const CLCTL_FILESERVER_SHARE_DEL: i32 = 4194890i32;
pub const CLCTL_FILESERVER_SHARE_MODIFY: i32 = 4194894i32;
pub const CLCTL_FILESERVER_SHARE_REPORT: i32 = 593i32;
pub const CLCTL_NETNAME_GET_OU_FOR_VCO: i32 = 4194926i32;
pub const CLCTL_ENABLE_SHARED_VOLUME_DIRECTIO: i32 = 4194954i32;
pub const CLCTL_DISABLE_SHARED_VOLUME_DIRECTIO: i32 = 4194958i32;
pub const CLCTL_GET_SHARED_VOLUME_ID: i32 = 657i32;
pub const CLCTL_SET_CSV_MAINTENANCE_MODE: i32 = 4194966i32;
pub const CLCTL_SET_SHARED_VOLUME_BACKUP_MODE: i32 = 4194970i32;
pub const CLCTL_STORAGE_GET_SHARED_VOLUME_PARTITION_NAMES: i32 = 669i32;
pub const CLCTL_STORAGE_GET_SHARED_VOLUME_STATES: i32 = 4194978i32;
pub const CLCTL_STORAGE_IS_SHARED_VOLUME: i32 = 677i32;
pub const CLCTL_GET_CLUSDB_TIMESTAMP: i32 = 681i32;
pub const CLCTL_RW_MODIFY_NOOP: i32 = 4194990i32;
pub const CLCTL_IS_QUORUM_BLOCKED: i32 = 689i32;
pub const CLCTL_POOL_GET_DRIVE_INFO: i32 = 693i32;
pub const CLCTL_GET_GUM_LOCK_OWNER: i32 = 697i32;
pub const CLCTL_GET_STUCK_NODES: i32 = 701i32;
pub const CLCTL_INJECT_GEM_FAULT: i32 = 705i32;
pub const CLCTL_INTRODUCE_GEM_REPAIR_DELAY: i32 = 709i32;
pub const CLCTL_SEND_DUMMY_GEM_MESSAGES: i32 = 713i32;
pub const CLCTL_BLOCK_GEM_SEND_RECV: i32 = 717i32;
pub const CLCTL_GET_GEMID_VECTOR: i32 = 721i32;
pub const CLCTL_ADD_CRYPTO_CHECKPOINT_EX: i32 = 4195030i32;
pub const CLCTL_GROUP_GET_LAST_MOVE_TIME: i32 = 729i32;
pub const CLCTL_SET_STORAGE_CONFIGURATION: i32 = 4195042i32;
pub const CLCTL_GET_STORAGE_CONFIGURATION: i32 = 741i32;
pub const CLCTL_GET_STORAGE_CONFIG_ATTRIBUTES: i32 = 745i32;
pub const CLCTL_REMOVE_NODE: i32 = 4195054i32;
pub const CLCTL_IS_FEATURE_INSTALLED: i32 = 753i32;
pub const CLCTL_IS_S2D_FEATURE_SUPPORTED: i32 = 757i32;
pub const CLCTL_STORAGE_GET_PHYSICAL_DISK_INFO: i32 = 761i32;
pub const CLCTL_STORAGE_GET_CLUSBFLT_PATHS: i32 = 765i32;
pub const CLCTL_STORAGE_GET_CLUSBFLT_PATHINFO: i32 = 769i32;
pub const CLCTL_CLEAR_NODE_CONNECTION_INFO: i32 = 4195078i32;
pub const CLCTL_SET_DNS_DOMAIN: i32 = 4195082i32;
pub const CTCTL_GET_ROUTESTATUS_BASIC: i32 = 781i32;
pub const CTCTL_GET_ROUTESTATUS_EXTENDED: i32 = 785i32;
pub const CTCTL_GET_FAULT_DOMAIN_STATE: i32 = 789i32;
pub const CLCTL_NETNAME_SET_PWD_INFOEX: i32 = 794i32;
pub const CLCTL_STORAGE_GET_AVAILABLE_DISKS_EX2_INT: i32 = 8161i32;
pub const CLCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS: i32 = 8417i32;
pub const CLCTL_CLOUD_WITNESS_RESOURCE_UPDATE_TOKEN: i32 = 4202726i32;
pub const CLCTL_RESOURCE_PREPARE_UPGRADE: i32 = 4202730i32;
pub const CLCTL_RESOURCE_UPGRADE_COMPLETED: i32 = 4202734i32;
pub const CLCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS_WITH_KEY: i32 = 8433i32;
pub const CLCTL_CLOUD_WITNESS_RESOURCE_UPDATE_KEY: i32 = 4202742i32;
pub const CLCTL_REPLICATION_ADD_REPLICATION_GROUP: i32 = 8514i32;
pub const CLCTL_REPLICATION_GET_LOG_INFO: i32 = 8517i32;
pub const CLCTL_REPLICATION_GET_ELIGIBLE_LOGDISKS: i32 = 8521i32;
pub const CLCTL_REPLICATION_GET_ELIGIBLE_TARGET_DATADISKS: i32 = 8525i32;
pub const CLCTL_REPLICATION_GET_ELIGIBLE_SOURCE_DATADISKS: i32 = 8529i32;
pub const CLCTL_REPLICATION_GET_REPLICATED_DISKS: i32 = 8533i32;
pub const CLCTL_REPLICATION_GET_REPLICA_VOLUMES: i32 = 8537i32;
pub const CLCTL_REPLICATION_GET_LOG_VOLUME: i32 = 8541i32;
pub const CLCTL_REPLICATION_GET_RESOURCE_GROUP: i32 = 8545i32;
pub const CLCTL_REPLICATION_GET_REPLICATED_PARTITION_INFO: i32 = 8549i32;
pub const CLCTL_GET_STATE_CHANGE_TIME: i32 = 11613i32;
pub const CLCTL_SET_CLUSTER_S2D_ENABLED: i32 = 4205922i32;
pub const CLCTL_SET_CLUSTER_S2D_CACHE_METADATA_RESERVE_BYTES: i32 = 4205934i32;
pub const CLCTL_GROUPSET_GET_GROUPS: i32 = 11633i32;
pub const CLCTL_GROUPSET_GET_PROVIDER_GROUPS: i32 = 11637i32;
pub const CLCTL_GROUPSET_GET_PROVIDER_GROUPSETS: i32 = 11641i32;
pub const CLCTL_GROUP_GET_PROVIDER_GROUPS: i32 = 11645i32;
pub const CLCTL_GROUP_GET_PROVIDER_GROUPSETS: i32 = 11649i32;
pub const CLCTL_GROUP_SET_CCF_FROM_MASTER: i32 = 4205958i32;
pub const CLCTL_GET_INFRASTRUCTURE_SOFS_BUFFER: i32 = 11657i32;
pub const CLCTL_SET_INFRASTRUCTURE_SOFS_BUFFER: i32 = 4205966i32;
pub const CLCTL_NOTIFY_INFRASTRUCTURE_SOFS_CHANGED: i32 = 4205970i32;
pub const CLCTL_SCALEOUT_COMMAND: i32 = 4205974i32;
pub const CLCTL_SCALEOUT_CONTROL: i32 = 4205978i32;
pub const CLCTL_SCALEOUT_GET_CLUSTERS: i32 = 4205981i32;
pub const CLCTL_RELOAD_AUTOLOGGER_CONFIG: i32 = 11730i32;
pub const CLCTL_STORAGE_RENAME_SHARED_VOLUME: i32 = 11734i32;
pub const CLCTL_STORAGE_RENAME_SHARED_VOLUME_GUID: i32 = 11738i32;
pub const CLCTL_ENUM_AFFINITY_RULE_NAMES: i32 = 11741i32;
pub const CLCTL_GET_NODES_IN_FD: i32 = 11745i32;
pub const CLCTL_FORCE_DB_FLUSH: i32 = 4206054i32;
pub const CLCTL_DELETE: i32 = 5242886i32;
pub const CLCTL_INSTALL_NODE: i32 = 5242890i32;
pub const CLCTL_EVICT_NODE: i32 = 5242894i32;
pub const CLCTL_ADD_DEPENDENCY: i32 = 5242898i32;
pub const CLCTL_REMOVE_DEPENDENCY: i32 = 5242902i32;
pub const CLCTL_ADD_OWNER: i32 = 5242906i32;
pub const CLCTL_REMOVE_OWNER: i32 = 5242910i32;
pub const CLCTL_SET_NAME: i32 = 5242918i32;
pub const CLCTL_CLUSTER_NAME_CHANGED: i32 = 5242922i32;
pub const CLCTL_CLUSTER_VERSION_CHANGED: i32 = 5242926i32;
pub const CLCTL_FIXUP_ON_UPGRADE: i32 = 5242930i32;
pub const CLCTL_STARTING_PHASE1: i32 = 5242934i32;
pub const CLCTL_STARTING_PHASE2: i32 = 5242938i32;
pub const CLCTL_HOLD_IO: i32 = 5242942i32;
pub const CLCTL_RESUME_IO: i32 = 5242946i32;
pub const CLCTL_FORCE_QUORUM: i32 = 5242950i32;
pub const CLCTL_INITIALIZE: i32 = 5242954i32;
pub const CLCTL_STATE_CHANGE_REASON: i32 = 5242958i32;
pub const CLCTL_PROVIDER_STATE_CHANGE: i32 = 5242962i32;
pub const CLCTL_LEAVING_GROUP: i32 = 5242966i32;
pub const CLCTL_JOINING_GROUP: i32 = 5242970i32;
pub const CLCTL_FSWITNESS_GET_EPOCH_INFO: i32 = 1048669i32;
pub const CLCTL_FSWITNESS_SET_EPOCH_INFO: i32 = 5242978i32;
pub const CLCTL_FSWITNESS_RELEASE_LOCK: i32 = 5242982i32;
pub const CLCTL_NETNAME_CREDS_NOTIFYCAM: i32 = 5242986i32;
pub const CLCTL_NOTIFY_QUORUM_STATUS: i32 = 5243006i32;
pub const CLCTL_NOTIFY_MONITOR_SHUTTING_DOWN: i32 = 1048705i32;
pub const CLCTL_UNDELETE: i32 = 5243014i32;
pub const CLCTL_GET_OPERATION_CONTEXT: i32 = 1057001i32;
pub const CLCTL_NOTIFY_OWNER_CHANGE: i32 = 5251362i32;
pub const CLCTL_VALIDATE_CHANGE_GROUP: i32 = 1057061i32;
pub const CLCTL_CHECK_DRAIN_VETO: i32 = 1057069i32;
pub const CLCTL_NOTIFY_DRAIN_COMPLETE: i32 = 1057073i32;
pub const CLCTL_GLOBAL_SHIFT: u32 = 23u32;
pub const CLCTL_INTERNAL_SHIFT: u32 = 20u32;
pub const CLCTL_MODIFY_SHIFT: u32 = 22u32;
pub const CLCTL_USER_SHIFT: u32 = 21u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLRES_CALLBACK_FUNCTION_TABLE {
    pub LogEvent: PLOG_EVENT_ROUTINE,
    pub SetResourceStatusEx: PSET_RESOURCE_STATUS_ROUTINE_EX,
    pub SetResourceLockedMode: PSET_RESOURCE_LOCKED_MODE_ROUTINE,
    pub SignalFailure: PSIGNAL_FAILURE_ROUTINE,
    pub SetResourceInMemoryNodeLocalProperties: PSET_RESOURCE_INMEMORY_NODELOCAL_PROPERTIES_ROUTINE,
    pub EndControlCall: PEND_CONTROL_CALL,
    pub EndTypeControlCall: PEND_TYPE_CONTROL_CALL,
    pub ExtendControlCall: PEXTEND_RES_CONTROL_CALL,
    pub ExtendTypeControlCall: PEXTEND_RES_TYPE_CONTROL_CALL,
    pub RaiseResTypeNotification: PRAISE_RES_TYPE_NOTIFICATION,
    pub ChangeResourceProcessForDumps: PCHANGE_RESOURCE_PROCESS_FOR_DUMPS,
    pub ChangeResTypeProcessForDumps: PCHANGE_RES_TYPE_PROCESS_FOR_DUMPS,
    pub SetInternalState: PSET_INTERNAL_STATE,
    pub SetResourceLockedModeEx: PSET_RESOURCE_LOCKED_MODE_EX_ROUTINE,
    pub RequestDump: PREQUEST_DUMP_ROUTINE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLRES_CALLBACK_FUNCTION_TABLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLRES_CALLBACK_FUNCTION_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub struct CLRES_FUNCTION_TABLE {
    pub TableSize: u32,
    pub Version: u32,
    pub Anonymous: CLRES_FUNCTION_TABLE_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::marker::Copy for CLRES_FUNCTION_TABLE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::clone::Clone for CLRES_FUNCTION_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub union CLRES_FUNCTION_TABLE_0 {
    pub V1Functions: CLRES_V1_FUNCTIONS,
    pub V2Functions: CLRES_V2_FUNCTIONS,
    pub V3Functions: CLRES_V3_FUNCTIONS,
    pub V4Functions: CLRES_V4_FUNCTIONS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::marker::Copy for CLRES_FUNCTION_TABLE_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::clone::Clone for CLRES_FUNCTION_TABLE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub struct CLRES_V1_FUNCTIONS {
    pub Open: POPEN_ROUTINE,
    pub Close: PCLOSE_ROUTINE,
    pub Online: PONLINE_ROUTINE,
    pub Offline: POFFLINE_ROUTINE,
    pub Terminate: PTERMINATE_ROUTINE,
    pub LooksAlive: PLOOKS_ALIVE_ROUTINE,
    pub IsAlive: PIS_ALIVE_ROUTINE,
    pub Arbitrate: PARBITRATE_ROUTINE,
    pub Release: PRELEASE_ROUTINE,
    pub ResourceControl: PRESOURCE_CONTROL_ROUTINE,
    pub ResourceTypeControl: PRESOURCE_TYPE_CONTROL_ROUTINE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::marker::Copy for CLRES_V1_FUNCTIONS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::clone::Clone for CLRES_V1_FUNCTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub struct CLRES_V2_FUNCTIONS {
    pub Open: POPEN_V2_ROUTINE,
    pub Close: PCLOSE_ROUTINE,
    pub Online: PONLINE_V2_ROUTINE,
    pub Offline: POFFLINE_V2_ROUTINE,
    pub Terminate: PTERMINATE_ROUTINE,
    pub LooksAlive: PLOOKS_ALIVE_ROUTINE,
    pub IsAlive: PIS_ALIVE_ROUTINE,
    pub Arbitrate: PARBITRATE_ROUTINE,
    pub Release: PRELEASE_ROUTINE,
    pub ResourceControl: PRESOURCE_CONTROL_ROUTINE,
    pub ResourceTypeControl: PRESOURCE_TYPE_CONTROL_ROUTINE,
    pub Cancel: PCANCEL_ROUTINE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::marker::Copy for CLRES_V2_FUNCTIONS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::clone::Clone for CLRES_V2_FUNCTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub struct CLRES_V3_FUNCTIONS {
    pub Open: POPEN_V2_ROUTINE,
    pub Close: PCLOSE_ROUTINE,
    pub Online: PONLINE_V2_ROUTINE,
    pub Offline: POFFLINE_V2_ROUTINE,
    pub Terminate: PTERMINATE_ROUTINE,
    pub LooksAlive: PLOOKS_ALIVE_ROUTINE,
    pub IsAlive: PIS_ALIVE_ROUTINE,
    pub Arbitrate: PARBITRATE_ROUTINE,
    pub Release: PRELEASE_ROUTINE,
    pub BeginResourceControl: PBEGIN_RESCALL_ROUTINE,
    pub BeginResourceTypeControl: PBEGIN_RESTYPECALL_ROUTINE,
    pub Cancel: PCANCEL_ROUTINE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::marker::Copy for CLRES_V3_FUNCTIONS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::clone::Clone for CLRES_V3_FUNCTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub struct CLRES_V4_FUNCTIONS {
    pub Open: POPEN_V2_ROUTINE,
    pub Close: PCLOSE_ROUTINE,
    pub Online: PONLINE_V2_ROUTINE,
    pub Offline: POFFLINE_V2_ROUTINE,
    pub Terminate: PTERMINATE_ROUTINE,
    pub LooksAlive: PLOOKS_ALIVE_ROUTINE,
    pub IsAlive: PIS_ALIVE_ROUTINE,
    pub Arbitrate: PARBITRATE_ROUTINE,
    pub Release: PRELEASE_ROUTINE,
    pub BeginResourceControl: PBEGIN_RESCALL_ROUTINE,
    pub BeginResourceTypeControl: PBEGIN_RESTYPECALL_ROUTINE,
    pub Cancel: PCANCEL_ROUTINE,
    pub BeginResourceControlAsUser: PBEGIN_RESCALL_AS_USER_ROUTINE,
    pub BeginResourceTypeControlAsUser: PBEGIN_RESTYPECALL_AS_USER_ROUTINE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::marker::Copy for CLRES_V4_FUNCTIONS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::core::clone::Clone for CLRES_V4_FUNCTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLRES_VERSION_V1_00: u32 = 256u32;
pub const CLRES_VERSION_V2_00: u32 = 512u32;
pub const CLRES_VERSION_V3_00: u32 = 768u32;
pub const CLRES_VERSION_V4_00: u32 = 1024u32;
pub const CLUADMEX_OT_NONE: i32 = 0i32;
pub const CLUADMEX_OT_CLUSTER: i32 = 1i32;
pub const CLUADMEX_OT_NODE: i32 = 2i32;
pub const CLUADMEX_OT_GROUP: i32 = 3i32;
pub const CLUADMEX_OT_RESOURCE: i32 = 4i32;
pub const CLUADMEX_OT_RESOURCETYPE: i32 = 5i32;
pub const CLUADMEX_OT_NETWORK: i32 = 6i32;
pub const CLUADMEX_OT_NETINTERFACE: i32 = 7i32;
pub const CLUSAPI_CHANGE_ACCESS: i32 = 2i32;
pub const CLUSAPI_CHANGE_RESOURCE_GROUP_FORCE_MOVE_TO_CSV: u64 = 1u64;
pub const CLUSAPI_GROUP_MOVE_FAILBACK: u32 = 16u32;
pub const CLUSAPI_GROUP_MOVE_HIGH_PRIORITY_START: u32 = 8u32;
pub const CLUSAPI_GROUP_MOVE_IGNORE_AFFINITY_RULE: u32 = 32u32;
pub const CLUSAPI_GROUP_MOVE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
pub const CLUSAPI_GROUP_MOVE_QUEUE_ENABLED: u32 = 4u32;
pub const CLUSAPI_GROUP_MOVE_RETURN_TO_SOURCE_NODE_ON_ERROR: u32 = 2u32;
pub const CLUSAPI_GROUP_OFFLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
pub const CLUSAPI_GROUP_ONLINE_BEST_POSSIBLE_NODE: u32 = 4u32;
pub const CLUSAPI_GROUP_ONLINE_IGNORE_AFFINITY_RULE: u32 = 8u32;
pub const CLUSAPI_GROUP_ONLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
pub const CLUSAPI_GROUP_ONLINE_SYNCHRONOUS: u32 = 2u32;
pub const CLUSAPI_NODE_AVOID_PLACEMENT: u32 = 2u32;
pub const CLUSAPI_NODE_PAUSE_REMAIN_ON_PAUSED_NODE_ON_MOVE_ERROR: u32 = 1u32;
pub const CLUSAPI_NODE_PAUSE_RETRY_DRAIN_ON_FAILURE: u32 = 4u32;
pub const CLUSAPI_NO_ACCESS: i32 = 4i32;
pub const CLUSAPI_READ_ACCESS: i32 = 1i32;
pub const CLUSAPI_RESOURCE_OFFLINE_DO_NOT_UPDATE_PERSISTENT_STATE: u32 = 4u32;
pub const CLUSAPI_RESOURCE_OFFLINE_FORCE_WITH_TERMINATION: u32 = 2u32;
pub const CLUSAPI_RESOURCE_OFFLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_BEING_DELETED: u32 = 8u32;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_BEING_RESTARTED: u32 = 16u32;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_MOVING: u32 = 2u32;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_NONE: u32 = 0u32;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_PREEMPTED: u32 = 32u32;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_SHUTTING_DOWN: u32 = 64u32;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_UNKNOWN: u32 = 1u32;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_USER_REQUESTED: u32 = 4u32;
pub const CLUSAPI_RESOURCE_ONLINE_BEST_POSSIBLE_NODE: u32 = 8u32;
pub const CLUSAPI_RESOURCE_ONLINE_DO_NOT_UPDATE_PERSISTENT_STATE: u32 = 2u32;
pub const CLUSAPI_RESOURCE_ONLINE_IGNORE_AFFINITY_RULE: u32 = 32u32;
pub const CLUSAPI_RESOURCE_ONLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
pub const CLUSAPI_RESOURCE_ONLINE_NECESSARY_FOR_QUORUM: u32 = 4u32;
pub const CLUSAPI_VALID_CHANGE_RESOURCE_GROUP_FLAGS: u64 = 1u64;
pub const CLUSAPI_VERSION: u32 = 2560u32;
pub const CLUSAPI_VERSION_RS3: u32 = 2560u32;
pub const CLUSAPI_VERSION_SERVER2008: u32 = 1536u32;
pub const CLUSAPI_VERSION_SERVER2008R2: u32 = 1792u32;
pub const CLUSAPI_VERSION_WINDOWS8: u32 = 1793u32;
pub const CLUSAPI_VERSION_WINDOWSBLUE: u32 = 1794u32;
pub const CLUSAPI_VERSION_WINTHRESHOLD: u32 = 1795u32;
pub const CLUSCTL_ACCESS_MODE_MASK: u32 = 3u32;
pub const CLUSCTL_ACCESS_SHIFT: u32 = 0u32;
pub const CLUSCTL_AFFINITYRULE_GET_COMMON_PROPERTIES: i32 = 150995033i32;
pub const CLUSCTL_AFFINITYRULE_GET_RO_COMMON_PROPERTIES: i32 = 150995029i32;
pub const CLUSCTL_AFFINITYRULE_SET_COMMON_PROPERTIES: i32 = 155189342i32;
pub const CLUSCTL_AFFINITYRULE_GET_ID: i32 = 150995001i32;
pub const CLUSCTL_AFFINITYRULE_GET_GROUPNAMES: i32 = 151006577i32;
pub const CLUSCTL_CLUSTER_UNKNOWN: i32 = 117440512i32;
pub const CLUSCTL_CLUSTER_GET_FQDN: i32 = 117440573i32;
pub const CLUSCTL_CLUSTER_SET_STORAGE_CONFIGURATION: i32 = 121635554i32;
pub const CLUSCTL_CLUSTER_GET_STORAGE_CONFIGURATION: i32 = 117441253i32;
pub const CLUSCTL_CLUSTER_GET_STORAGE_CONFIG_ATTRIBUTES: i32 = 117441257i32;
pub const CLUSCTL_CLUSTER_ENUM_COMMON_PROPERTIES: i32 = 117440593i32;
pub const CLUSCTL_CLUSTER_GET_RO_COMMON_PROPERTIES: i32 = 117440597i32;
pub const CLUSCTL_CLUSTER_GET_COMMON_PROPERTIES: i32 = 117440601i32;
pub const CLUSCTL_CLUSTER_SET_COMMON_PROPERTIES: i32 = 121634910i32;
pub const CLUSCTL_CLUSTER_VALIDATE_COMMON_PROPERTIES: i32 = 117440609i32;
pub const CLUSCTL_CLUSTER_ENUM_PRIVATE_PROPERTIES: i32 = 117440633i32;
pub const CLUSCTL_CLUSTER_GET_RO_PRIVATE_PROPERTIES: i32 = 117440637i32;
pub const CLUSCTL_CLUSTER_GET_PRIVATE_PROPERTIES: i32 = 117440641i32;
pub const CLUSCTL_CLUSTER_SET_PRIVATE_PROPERTIES: i32 = 121634950i32;
pub const CLUSCTL_CLUSTER_VALIDATE_PRIVATE_PROPERTIES: i32 = 117440649i32;
pub const CLUSCTL_CLUSTER_GET_COMMON_PROPERTY_FMTS: i32 = 117440613i32;
pub const CLUSCTL_CLUSTER_GET_PRIVATE_PROPERTY_FMTS: i32 = 117440653i32;
pub const CLUSCTL_CLUSTER_CHECK_VOTER_EVICT: i32 = 117440581i32;
pub const CLUSCTL_CLUSTER_CHECK_VOTER_DOWN: i32 = 117440585i32;
pub const CLUSCTL_CLUSTER_SHUTDOWN: i32 = 117440589i32;
pub const CLUSCTL_CLUSTER_BATCH_BLOCK_KEY: i32 = 117441086i32;
pub const CLUSCTL_CLUSTER_BATCH_UNBLOCK_KEY: i32 = 117441089i32;
pub const CLUSCTL_CLUSTER_GET_SHARED_VOLUME_ID: i32 = 117441169i32;
pub const CLUSCTL_CLUSTER_GET_CLUSDB_TIMESTAMP: i32 = 117441193i32;
pub const CLUSCTL_CLUSTER_GET_GUM_LOCK_OWNER: i32 = 117441209i32;
pub const CLUSCTL_CLUSTER_REMOVE_NODE: i32 = 121635566i32;
pub const CLUSCTL_CLUSTER_SET_ACCOUNT_ACCESS: i32 = 121635058i32;
pub const CLUSCTL_CLUSTER_CLEAR_NODE_CONNECTION_INFO: i32 = 121635590i32;
pub const CLUSCTL_CLUSTER_SET_DNS_DOMAIN: i32 = 121635594i32;
pub const CLUSCTL_CLUSTER_SET_CLUSTER_S2D_ENABLED: i32 = 121646434i32;
pub const CLUSCTL_CLUSTER_SET_CLUSTER_S2D_CACHE_METADATA_RESERVE_BYTES: i32 = 121646446i32;
pub const CLUSCTL_CLUSTER_STORAGE_RENAME_SHARED_VOLUME: i32 = 117452246i32;
pub const CLUSCTL_CLUSTER_STORAGE_RENAME_SHARED_VOLUME_GUID: i32 = 117452250i32;
pub const CLUSCTL_CLUSTER_RELOAD_AUTOLOGGER_CONFIG: i32 = 117452242i32;
pub const CLUSCTL_CLUSTER_ENUM_AFFINITY_RULE_NAMES: i32 = 117452253i32;
pub const CLUSCTL_CLUSTER_GET_NODES_IN_FD: i32 = 117452257i32;
pub const CLUSCTL_CLUSTER_FORCE_FLUSH_DB: i32 = 121646566i32;
pub const CLUSCTL_CLUSTER_GET_CLMUSR_TOKEN: i32 = 117440877i32;
pub const CLUSCTL_CONTROL_CODE_MASK: u32 = 4194303u32;
pub const CLUSCTL_FUNCTION_SHIFT: u32 = 2u32;
pub const CLUSCTL_GET_OPERATION_CONTEXT_PARAMS_VERSION_1: u32 = 1u32;
pub const CLUSCTL_GROUPSET_GET_COMMON_PROPERTIES: i32 = 134217817i32;
pub const CLUSCTL_GROUPSET_GET_RO_COMMON_PROPERTIES: i32 = 134217813i32;
pub const CLUSCTL_GROUPSET_SET_COMMON_PROPERTIES: i32 = 138412126i32;
pub const CLUSCTL_GROUPSET_GET_GROUPS: i32 = 134229361i32;
pub const CLUSCTL_GROUPSET_GET_PROVIDER_GROUPS: i32 = 134229365i32;
pub const CLUSCTL_GROUPSET_GET_PROVIDER_GROUPSETS: i32 = 134229369i32;
pub const CLUSCTL_GROUP_GET_PROVIDER_GROUPS: i32 = 134229373i32;
pub const CLUSCTL_GROUP_GET_PROVIDER_GROUPSETS: i32 = 134229377i32;
pub const CLUSCTL_GROUPSET_GET_ID: i32 = 134217785i32;
pub const CLUSCTL_GROUP_UNKNOWN: i32 = 50331648i32;
pub const CLUSCTL_GROUP_GET_CHARACTERISTICS: i32 = 50331653i32;
pub const CLUSCTL_GROUP_GET_FLAGS: i32 = 50331657i32;
pub const CLUSCTL_GROUP_GET_NAME: i32 = 50331689i32;
pub const CLUSCTL_GROUP_GET_ID: i32 = 50331705i32;
pub const CLUSCTL_GROUP_ENUM_COMMON_PROPERTIES: i32 = 50331729i32;
pub const CLUSCTL_GROUP_GET_RO_COMMON_PROPERTIES: i32 = 50331733i32;
pub const CLUSCTL_GROUP_GET_COMMON_PROPERTIES: i32 = 50331737i32;
pub const CLUSCTL_GROUP_SET_COMMON_PROPERTIES: i32 = 54526046i32;
pub const CLUSCTL_GROUP_VALIDATE_COMMON_PROPERTIES: i32 = 50331745i32;
pub const CLUSCTL_GROUP_ENUM_PRIVATE_PROPERTIES: i32 = 50331769i32;
pub const CLUSCTL_GROUP_GET_RO_PRIVATE_PROPERTIES: i32 = 50331773i32;
pub const CLUSCTL_GROUP_GET_PRIVATE_PROPERTIES: i32 = 50331777i32;
pub const CLUSCTL_GROUP_SET_PRIVATE_PROPERTIES: i32 = 54526086i32;
pub const CLUSCTL_GROUP_VALIDATE_PRIVATE_PROPERTIES: i32 = 50331785i32;
pub const CLUSCTL_GROUP_QUERY_DELETE: i32 = 50332089i32;
pub const CLUSCTL_GROUP_GET_COMMON_PROPERTY_FMTS: i32 = 50331749i32;
pub const CLUSCTL_GROUP_GET_PRIVATE_PROPERTY_FMTS: i32 = 50331789i32;
pub const CLUSCTL_GROUP_GET_FAILURE_INFO: i32 = 50331673i32;
pub const CLUSCTL_GROUP_GET_LAST_MOVE_TIME: i32 = 50332377i32;
pub const CLUSCTL_GROUP_SET_CCF_FROM_MASTER: i32 = 54537606i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT {
    pub GetTickCount64: u64,
    pub GetSystemTime: super::super::Foundation::SYSTEMTIME,
    pub NodeId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLUSCTL_NETINTERFACE_UNKNOWN: i32 = 100663296i32;
pub const CLUSCTL_NETINTERFACE_GET_CHARACTERISTICS: i32 = 100663301i32;
pub const CLUSCTL_NETINTERFACE_GET_FLAGS: i32 = 100663305i32;
pub const CLUSCTL_NETINTERFACE_GET_NAME: i32 = 100663337i32;
pub const CLUSCTL_NETINTERFACE_GET_ID: i32 = 100663353i32;
pub const CLUSCTL_NETINTERFACE_GET_NODE: i32 = 100663345i32;
pub const CLUSCTL_NETINTERFACE_GET_NETWORK: i32 = 100663349i32;
pub const CLUSCTL_NETINTERFACE_ENUM_COMMON_PROPERTIES: i32 = 100663377i32;
pub const CLUSCTL_NETINTERFACE_GET_RO_COMMON_PROPERTIES: i32 = 100663381i32;
pub const CLUSCTL_NETINTERFACE_GET_COMMON_PROPERTIES: i32 = 100663385i32;
pub const CLUSCTL_NETINTERFACE_SET_COMMON_PROPERTIES: i32 = 104857694i32;
pub const CLUSCTL_NETINTERFACE_VALIDATE_COMMON_PROPERTIES: i32 = 100663393i32;
pub const CLUSCTL_NETINTERFACE_ENUM_PRIVATE_PROPERTIES: i32 = 100663417i32;
pub const CLUSCTL_NETINTERFACE_GET_RO_PRIVATE_PROPERTIES: i32 = 100663421i32;
pub const CLUSCTL_NETINTERFACE_GET_PRIVATE_PROPERTIES: i32 = 100663425i32;
pub const CLUSCTL_NETINTERFACE_SET_PRIVATE_PROPERTIES: i32 = 104857734i32;
pub const CLUSCTL_NETINTERFACE_VALIDATE_PRIVATE_PROPERTIES: i32 = 100663433i32;
pub const CLUSCTL_NETINTERFACE_GET_COMMON_PROPERTY_FMTS: i32 = 100663397i32;
pub const CLUSCTL_NETINTERFACE_GET_PRIVATE_PROPERTY_FMTS: i32 = 100663437i32;
pub const CLUSCTL_NETWORK_UNKNOWN: i32 = 83886080i32;
pub const CLUSCTL_NETWORK_GET_CHARACTERISTICS: i32 = 83886085i32;
pub const CLUSCTL_NETWORK_GET_FLAGS: i32 = 83886089i32;
pub const CLUSCTL_NETWORK_GET_NAME: i32 = 83886121i32;
pub const CLUSCTL_NETWORK_GET_ID: i32 = 83886137i32;
pub const CLUSCTL_NETWORK_ENUM_COMMON_PROPERTIES: i32 = 83886161i32;
pub const CLUSCTL_NETWORK_GET_RO_COMMON_PROPERTIES: i32 = 83886165i32;
pub const CLUSCTL_NETWORK_GET_COMMON_PROPERTIES: i32 = 83886169i32;
pub const CLUSCTL_NETWORK_SET_COMMON_PROPERTIES: i32 = 88080478i32;
pub const CLUSCTL_NETWORK_VALIDATE_COMMON_PROPERTIES: i32 = 83886177i32;
pub const CLUSCTL_NETWORK_ENUM_PRIVATE_PROPERTIES: i32 = 83886201i32;
pub const CLUSCTL_NETWORK_GET_RO_PRIVATE_PROPERTIES: i32 = 83886205i32;
pub const CLUSCTL_NETWORK_GET_PRIVATE_PROPERTIES: i32 = 83886209i32;
pub const CLUSCTL_NETWORK_SET_PRIVATE_PROPERTIES: i32 = 88080518i32;
pub const CLUSCTL_NETWORK_VALIDATE_PRIVATE_PROPERTIES: i32 = 83886217i32;
pub const CLUSCTL_NETWORK_GET_COMMON_PROPERTY_FMTS: i32 = 83886181i32;
pub const CLUSCTL_NETWORK_GET_PRIVATE_PROPERTY_FMTS: i32 = 83886221i32;
pub const CLUSCTL_NODE_UNKNOWN: i32 = 67108864i32;
pub const CLUSCTL_NODE_GET_CHARACTERISTICS: i32 = 67108869i32;
pub const CLUSCTL_NODE_GET_FLAGS: i32 = 67108873i32;
pub const CLUSCTL_NODE_GET_NAME: i32 = 67108905i32;
pub const CLUSCTL_NODE_GET_ID: i32 = 67108921i32;
pub const CLUSCTL_NODE_ENUM_COMMON_PROPERTIES: i32 = 67108945i32;
pub const CLUSCTL_NODE_GET_RO_COMMON_PROPERTIES: i32 = 67108949i32;
pub const CLUSCTL_NODE_GET_COMMON_PROPERTIES: i32 = 67108953i32;
pub const CLUSCTL_NODE_SET_COMMON_PROPERTIES: i32 = 71303262i32;
pub const CLUSCTL_NODE_VALIDATE_COMMON_PROPERTIES: i32 = 67108961i32;
pub const CLUSCTL_NODE_ENUM_PRIVATE_PROPERTIES: i32 = 67108985i32;
pub const CLUSCTL_NODE_GET_RO_PRIVATE_PROPERTIES: i32 = 67108989i32;
pub const CLUSCTL_NODE_GET_PRIVATE_PROPERTIES: i32 = 67108993i32;
pub const CLUSCTL_NODE_SET_PRIVATE_PROPERTIES: i32 = 71303302i32;
pub const CLUSCTL_NODE_VALIDATE_PRIVATE_PROPERTIES: i32 = 67109001i32;
pub const CLUSCTL_NODE_GET_COMMON_PROPERTY_FMTS: i32 = 67108965i32;
pub const CLUSCTL_NODE_GET_PRIVATE_PROPERTY_FMTS: i32 = 67109005i32;
pub const CLUSCTL_NODE_GET_CLUSTER_SERVICE_ACCOUNT_NAME: i32 = 67108929i32;
pub const CLUSCTL_NODE_GET_STUCK_NODES: i32 = 67109565i32;
pub const CLUSCTL_NODE_INJECT_GEM_FAULT: i32 = 67109569i32;
pub const CLUSCTL_NODE_INTRODUCE_GEM_REPAIR_DELAY: i32 = 67109573i32;
pub const CLUSCTL_NODE_SEND_DUMMY_GEM_MESSAGES: i32 = 67109577i32;
pub const CLUSCTL_NODE_BLOCK_GEM_SEND_RECV: i32 = 67109581i32;
pub const CLUSCTL_NODE_GET_GEMID_VECTOR: i32 = 67109585i32;
pub const CLUSCTL_OBJECT_MASK: u32 = 255u32;
pub const CLUSCTL_OBJECT_SHIFT: u32 = 24u32;
pub const CLUSCTL_RESOURCE_UNKNOWN: i32 = 16777216i32;
pub const CLUSCTL_RESOURCE_GET_CHARACTERISTICS: i32 = 16777221i32;
pub const CLUSCTL_RESOURCE_GET_FLAGS: i32 = 16777225i32;
pub const CLUSCTL_RESOURCE_GET_CLASS_INFO: i32 = 16777229i32;
pub const CLUSCTL_RESOURCE_GET_REQUIRED_DEPENDENCIES: i32 = 16777233i32;
pub const CLUSCTL_RESOURCE_GET_NAME: i32 = 16777257i32;
pub const CLUSCTL_RESOURCE_GET_ID: i32 = 16777273i32;
pub const CLUSCTL_RESOURCE_GET_RESOURCE_TYPE: i32 = 16777261i32;
pub const CLUSCTL_RESOURCE_ENUM_COMMON_PROPERTIES: i32 = 16777297i32;
pub const CLUSCTL_RESOURCE_GET_RO_COMMON_PROPERTIES: i32 = 16777301i32;
pub const CLUSCTL_RESOURCE_GET_COMMON_PROPERTIES: i32 = 16777305i32;
pub const CLUSCTL_RESOURCE_SET_COMMON_PROPERTIES: i32 = 20971614i32;
pub const CLUSCTL_RESOURCE_VALIDATE_COMMON_PROPERTIES: i32 = 16777313i32;
pub const CLUSCTL_RESOURCE_GET_COMMON_PROPERTY_FMTS: i32 = 16777317i32;
pub const CLUSCTL_RESOURCE_ENUM_PRIVATE_PROPERTIES: i32 = 16777337i32;
pub const CLUSCTL_RESOURCE_GET_RO_PRIVATE_PROPERTIES: i32 = 16777341i32;
pub const CLUSCTL_RESOURCE_GET_PRIVATE_PROPERTIES: i32 = 16777345i32;
pub const CLUSCTL_RESOURCE_SET_PRIVATE_PROPERTIES: i32 = 20971654i32;
pub const CLUSCTL_RESOURCE_VALIDATE_PRIVATE_PROPERTIES: i32 = 16777353i32;
pub const CLUSCTL_RESOURCE_GET_PRIVATE_PROPERTY_FMTS: i32 = 16777357i32;
pub const CLUSCTL_RESOURCE_ADD_REGISTRY_CHECKPOINT: i32 = 20971682i32;
pub const CLUSCTL_RESOURCE_DELETE_REGISTRY_CHECKPOINT: i32 = 20971686i32;
pub const CLUSCTL_RESOURCE_GET_REGISTRY_CHECKPOINTS: i32 = 16777385i32;
pub const CLUSCTL_RESOURCE_ADD_CRYPTO_CHECKPOINT: i32 = 20971694i32;
pub const CLUSCTL_RESOURCE_DELETE_CRYPTO_CHECKPOINT: i32 = 20971698i32;
pub const CLUSCTL_RESOURCE_ADD_CRYPTO_CHECKPOINT_EX: i32 = 20972246i32;
pub const CLUSCTL_RESOURCE_GET_CRYPTO_CHECKPOINTS: i32 = 16777397i32;
pub const CLUSCTL_RESOURCE_GET_LOADBAL_PROCESS_LIST: i32 = 16777417i32;
pub const CLUSCTL_RESOURCE_GET_NETWORK_NAME: i32 = 16777577i32;
pub const CLUSCTL_RESOURCE_NETNAME_GET_VIRTUAL_SERVER_TOKEN: i32 = 16777581i32;
pub const CLUSCTL_RESOURCE_NETNAME_SET_PWD_INFO: i32 = 16777594i32;
pub const CLUSCTL_RESOURCE_NETNAME_SET_PWD_INFOEX: i32 = 16778010i32;
pub const CLUSCTL_RESOURCE_NETNAME_DELETE_CO: i32 = 16777598i32;
pub const CLUSCTL_RESOURCE_NETNAME_VALIDATE_VCO: i32 = 16777601i32;
pub const CLUSCTL_RESOURCE_NETNAME_RESET_VCO: i32 = 16777605i32;
pub const CLUSCTL_RESOURCE_NETNAME_REPAIR_VCO: i32 = 16777613i32;
pub const CLUSCTL_RESOURCE_NETNAME_REGISTER_DNS_RECORDS: i32 = 16777586i32;
pub const CLUSCTL_RESOURCE_GET_DNS_NAME: i32 = 16777589i32;
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISK_INFO: i32 = 16777617i32;
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISK_NUMBER_INFO: i32 = 16777633i32;
pub const CLUSCTL_RESOURCE_STORAGE_IS_PATH_VALID: i32 = 16777625i32;
pub const CLUSCTL_RESOURCE_QUERY_DELETE: i32 = 16777657i32;
pub const CLUSCTL_RESOURCE_UPGRADE_DLL: i32 = 20971706i32;
pub const CLUSCTL_RESOURCE_IPADDRESS_RENEW_LEASE: i32 = 20971966i32;
pub const CLUSCTL_RESOURCE_IPADDRESS_RELEASE_LEASE: i32 = 20971970i32;
pub const CLUSCTL_RESOURCE_ADD_REGISTRY_CHECKPOINT_64BIT: i32 = 20971710i32;
pub const CLUSCTL_RESOURCE_ADD_REGISTRY_CHECKPOINT_32BIT: i32 = 20971714i32;
pub const CLUSCTL_RESOURCE_QUERY_MAINTENANCE_MODE: i32 = 16777697i32;
pub const CLUSCTL_RESOURCE_SET_MAINTENANCE_MODE: i32 = 20972006i32;
pub const CLUSCTL_RESOURCE_STORAGE_SET_DRIVELETTER: i32 = 20972010i32;
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISK_INFO_EX: i32 = 16777713i32;
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISK_INFO_EX2: i32 = 16777721i32;
pub const CLUSCTL_RESOURCE_STORAGE_GET_MOUNTPOINTS: i32 = 16777745i32;
pub const CLUSCTL_RESOURCE_STORAGE_GET_DIRTY: i32 = 16777753i32;
pub const CLUSCTL_RESOURCE_STORAGE_GET_SHARED_VOLUME_INFO: i32 = 16777765i32;
pub const CLUSCTL_RESOURCE_SET_CSV_MAINTENANCE_MODE: i32 = 20972182i32;
pub const CLUSCTL_RESOURCE_ENABLE_SHARED_VOLUME_DIRECTIO: i32 = 20972170i32;
pub const CLUSCTL_RESOURCE_DISABLE_SHARED_VOLUME_DIRECTIO: i32 = 20972174i32;
pub const CLUSCTL_RESOURCE_SET_SHARED_VOLUME_BACKUP_MODE: i32 = 20972186i32;
pub const CLUSCTL_RESOURCE_STORAGE_GET_SHARED_VOLUME_PARTITION_NAMES: i32 = 16777885i32;
pub const CLUSCTL_RESOURCE_GET_FAILURE_INFO: i32 = 16777241i32;
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISKID: i32 = 16777733i32;
pub const CLUSCTL_RESOURCE_STORAGE_GET_SHARED_VOLUME_STATES: i32 = 20972194i32;
pub const CLUSCTL_RESOURCE_STORAGE_IS_SHARED_VOLUME: i32 = 16777893i32;
pub const CLUSCTL_RESOURCE_IS_QUORUM_BLOCKED: i32 = 16777905i32;
pub const CLUSCTL_RESOURCE_POOL_GET_DRIVE_INFO: i32 = 16777909i32;
pub const CLUSCTL_RESOURCE_RLUA_GET_VIRTUAL_SERVER_TOKEN: i32 = 16777581i32;
pub const CLUSCTL_RESOURCE_RLUA_SET_PWD_INFO: i32 = 16777594i32;
pub const CLUSCTL_RESOURCE_RLUA_SET_PWD_INFOEX: i32 = 16778010i32;
pub const CLUSCTL_RESOURCE_DELETE: i32 = 22020102i32;
pub const CLUSCTL_RESOURCE_UNDELETE: i32 = 22020230i32;
pub const CLUSCTL_RESOURCE_INSTALL_NODE: i32 = 22020106i32;
pub const CLUSCTL_RESOURCE_EVICT_NODE: i32 = 22020110i32;
pub const CLUSCTL_RESOURCE_ADD_DEPENDENCY: i32 = 22020114i32;
pub const CLUSCTL_RESOURCE_REMOVE_DEPENDENCY: i32 = 22020118i32;
pub const CLUSCTL_RESOURCE_ADD_OWNER: i32 = 22020122i32;
pub const CLUSCTL_RESOURCE_REMOVE_OWNER: i32 = 22020126i32;
pub const CLUSCTL_RESOURCE_SET_NAME: i32 = 22020134i32;
pub const CLUSCTL_RESOURCE_CLUSTER_NAME_CHANGED: i32 = 22020138i32;
pub const CLUSCTL_RESOURCE_CLUSTER_VERSION_CHANGED: i32 = 22020142i32;
pub const CLUSCTL_RESOURCE_FORCE_QUORUM: i32 = 22020166i32;
pub const CLUSCTL_RESOURCE_INITIALIZE: i32 = 22020170i32;
pub const CLUSCTL_RESOURCE_STATE_CHANGE_REASON: i32 = 22020174i32;
pub const CLUSCTL_RESOURCE_PROVIDER_STATE_CHANGE: i32 = 22020178i32;
pub const CLUSCTL_RESOURCE_LEAVING_GROUP: i32 = 22020182i32;
pub const CLUSCTL_RESOURCE_JOINING_GROUP: i32 = 22020186i32;
pub const CLUSCTL_RESOURCE_FSWITNESS_GET_EPOCH_INFO: i32 = 17825885i32;
pub const CLUSCTL_RESOURCE_FSWITNESS_SET_EPOCH_INFO: i32 = 22020194i32;
pub const CLUSCTL_RESOURCE_FSWITNESS_RELEASE_LOCK: i32 = 22020198i32;
pub const CLUSCTL_RESOURCE_NETNAME_CREDS_NOTIFYCAM: i32 = 22020202i32;
pub const CLUSCTL_RESOURCE_GET_OPERATION_CONTEXT: i32 = 17834217i32;
pub const CLUSCTL_RESOURCE_RW_MODIFY_NOOP: i32 = 20972206i32;
pub const CLUSCTL_RESOURCE_NOTIFY_QUORUM_STATUS: i32 = 22020222i32;
pub const CLUSCTL_RESOURCE_NOTIFY_OWNER_CHANGE: i32 = 22028578i32;
pub const CLUSCTL_RESOURCE_VALIDATE_CHANGE_GROUP: i32 = 17834277i32;
pub const CLUSCTL_RESOURCE_STORAGE_RENAME_SHARED_VOLUME: i32 = 16788950i32;
pub const CLUSCTL_RESOURCE_STORAGE_RENAME_SHARED_VOLUME_GUID: i32 = 16788954i32;
pub const CLUSCTL_CLOUD_WITNESS_RESOURCE_UPDATE_TOKEN: i32 = 20979942i32;
pub const CLUSCTL_CLOUD_WITNESS_RESOURCE_UPDATE_KEY: i32 = 20979958i32;
pub const CLUSCTL_RESOURCE_PREPARE_UPGRADE: i32 = 20979946i32;
pub const CLUSCTL_RESOURCE_UPGRADE_COMPLETED: i32 = 20979950i32;
pub const CLUSCTL_RESOURCE_GET_STATE_CHANGE_TIME: i32 = 16788829i32;
pub const CLUSCTL_RESOURCE_GET_INFRASTRUCTURE_SOFS_BUFFER: i32 = 16788873i32;
pub const CLUSCTL_RESOURCE_SET_INFRASTRUCTURE_SOFS_BUFFER: i32 = 20983182i32;
pub const CLUSCTL_RESOURCE_SCALEOUT_COMMAND: i32 = 20983190i32;
pub const CLUSCTL_RESOURCE_SCALEOUT_CONTROL: i32 = 20983194i32;
pub const CLUSCTL_RESOURCE_SCALEOUT_GET_CLUSTERS: i32 = 20983197i32;
pub const CLUSCTL_RESOURCE_CHECK_DRAIN_VETO: i32 = 17834285i32;
pub const CLUSCTL_RESOURCE_NOTIFY_DRAIN_COMPLETE: i32 = 17834289i32;
pub const CLUSCTL_RESOURCE_GET_NODES_IN_FD: i32 = 16788961i32;
#[repr(C)]
pub struct CLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub eReason: CLUSTER_RESOURCE_STATE_CHANGE_REASON,
}
impl ::core::marker::Copy for CLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT {}
impl ::core::clone::Clone for CLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLUSCTL_RESOURCE_STATE_CHANGE_REASON_VERSION_1: u32 = 1u32;
pub const CLUSCTL_RESOURCE_TYPE_UNKNOWN: i32 = 33554432i32;
pub const CLUSCTL_RESOURCE_TYPE_GET_CHARACTERISTICS: i32 = 33554437i32;
pub const CLUSCTL_RESOURCE_TYPE_GET_FLAGS: i32 = 33554441i32;
pub const CLUSCTL_RESOURCE_TYPE_GET_CLASS_INFO: i32 = 33554445i32;
pub const CLUSCTL_RESOURCE_TYPE_GET_REQUIRED_DEPENDENCIES: i32 = 33554449i32;
pub const CLUSCTL_RESOURCE_TYPE_GET_ARB_TIMEOUT: i32 = 33554453i32;
pub const CLUSCTL_RESOURCE_TYPE_ENUM_COMMON_PROPERTIES: i32 = 33554513i32;
pub const CLUSCTL_RESOURCE_TYPE_GET_RO_COMMON_PROPERTIES: i32 = 33554517i32;
pub const CLUSCTL_RESOURCE_TYPE_GET_COMMON_PROPERTIES: i32 = 33554521i32;
pub const CLUSCTL_RESOURCE_TYPE_VALIDATE_COMMON_PROPERTIES: i32 = 33554529i32;
pub const CLUSCTL_RESOURCE_TYPE_SET_COMMON_PROPERTIES: i32 = 37748830i32;
pub const CLUSCTL_RESOURCE_TYPE_GET_COMMON_PROPERTY_FMTS: i32 = 33554533i32;
pub const CLUSCTL_RESOURCE_TYPE_GET_COMMON_RESOURCE_PROPERTY_FMTS: i32 = 33554537i32;
pub const CLUSCTL_RESOURCE_TYPE_ENUM_PRIVATE_PROPERTIES: i32 = 33554553i32;
pub const CLUSCTL_RESOURCE_TYPE_GET_RO_PRIVATE_PROPERTIES: i32 = 33554557i32;
pub const CLUSCTL_RESOURCE_TYPE_GET_PRIVATE_PROPERTIES: i32 = 33554561i32;
pub const CLUSCTL_RESOURCE_TYPE_SET_PRIVATE_PROPERTIES: i32 = 37748870i32;
pub const CLUSCTL_RESOURCE_TYPE_VALIDATE_PRIVATE_PROPERTIES: i32 = 33554569i32;
pub const CLUSCTL_RESOURCE_TYPE_GET_PRIVATE_PROPERTY_FMTS: i32 = 33554573i32;
pub const CLUSCTL_RESOURCE_TYPE_GET_PRIVATE_RESOURCE_PROPERTY_FMTS: i32 = 33554577i32;
pub const CLUSCTL_RESOURCE_TYPE_GET_REGISTRY_CHECKPOINTS: i32 = 33554601i32;
pub const CLUSCTL_RESOURCE_TYPE_GET_CRYPTO_CHECKPOINTS: i32 = 33554613i32;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS: i32 = 33554837i32;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_SYNC_CLUSDISK_DB: i32 = 37749150i32;
pub const CLUSCTL_RESOURCE_TYPE_NETNAME_VALIDATE_NETNAME: i32 = 33554997i32;
pub const CLUSCTL_RESOURCE_TYPE_NETNAME_GET_OU_FOR_VCO: i32 = 37749358i32;
pub const CLUSCTL_RESOURCE_TYPE_GEN_APP_VALIDATE_PATH: i32 = 33554993i32;
pub const CLUSCTL_RESOURCE_TYPE_GEN_APP_VALIDATE_DIRECTORY: i32 = 33555001i32;
pub const CLUSCTL_RESOURCE_TYPE_GEN_SCRIPT_VALIDATE_PATH: i32 = 33554993i32;
pub const CLUSCTL_RESOURCE_TYPE_QUERY_DELETE: i32 = 33554873i32;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_DRIVELETTERS: i32 = 33554925i32;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX: i32 = 33554933i32;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_REMAP_DRIVELETTER: i32 = 33554945i32;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_DISKID: i32 = 33554949i32;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_RESOURCEID: i32 = 33554989i32;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_IS_CLUSTERABLE: i32 = 33554953i32;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_REMOVE_VM_OWNERSHIP: i32 = 37749262i32;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_IS_CSV_FILE: i32 = 16777769i32;
pub const CLUSCTL_RESOURCE_TYPE_WITNESS_VALIDATE_PATH: i32 = 33554993i32;
pub const CLUSCTL_RESOURCE_TYPE_INSTALL_NODE: i32 = 38797322i32;
pub const CLUSCTL_RESOURCE_TYPE_EVICT_NODE: i32 = 38797326i32;
pub const CLUSCTL_RESOURCE_TYPE_CLUSTER_VERSION_CHANGED: i32 = 38797358i32;
pub const CLUSCTL_RESOURCE_TYPE_FIXUP_ON_UPGRADE: i32 = 38797362i32;
pub const CLUSCTL_RESOURCE_TYPE_STARTING_PHASE1: i32 = 38797366i32;
pub const CLUSCTL_RESOURCE_TYPE_STARTING_PHASE2: i32 = 38797370i32;
pub const CLUSCTL_RESOURCE_TYPE_HOLD_IO: i32 = 38797374i32;
pub const CLUSCTL_RESOURCE_TYPE_RESUME_IO: i32 = 38797378i32;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INT: i32 = 33562593i32;
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_ELIGIBLE_LOGDISKS: i32 = 33562953i32;
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_ELIGIBLE_TARGET_DATADISKS: i32 = 33562957i32;
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_ELIGIBLE_SOURCE_DATADISKS: i32 = 33562961i32;
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_REPLICATED_DISKS: i32 = 33562965i32;
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_REPLICA_VOLUMES: i32 = 33562969i32;
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_LOG_VOLUME: i32 = 33562973i32;
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_RESOURCE_GROUP: i32 = 33562977i32;
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_REPLICATED_PARTITION_INFO: i32 = 33562981i32;
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_LOG_INFO: i32 = 33562949i32;
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_ADD_REPLICATION_GROUP: i32 = 33562946i32;
pub const CLUSCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS: i32 = 33562849i32;
pub const CLUSCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS_WITH_KEY: i32 = 33562865i32;
pub const CLUSCTL_RESOURCE_TYPE_PREPARE_UPGRADE: i32 = 37757162i32;
pub const CLUSCTL_RESOURCE_TYPE_UPGRADE_COMPLETED: i32 = 37757166i32;
pub const CLUSCTL_RESOURCE_TYPE_NOTIFY_MONITOR_SHUTTING_DOWN: i32 = 34603137i32;
pub const CLUSCTL_RESOURCE_TYPE_CHECK_DRAIN_VETO: i32 = 34611501i32;
pub const CLUSCTL_RESOURCE_TYPE_NOTIFY_DRAIN_COMPLETE: i32 = 34611505i32;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_FLAG_ADD_VOLUME_INFO: u32 = 1u32;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_FLAG_FILTER_BY_POOL: u32 = 2u32;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_FLAG_INCLUDE_NON_SHARED_DISKS: u32 = 4u32;
#[repr(C)]
pub struct CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT {
    pub dwFlags: u32,
    pub guidPoolFilter: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT {}
impl ::core::clone::Clone for CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLUSGROUPSET_STATUS_APPLICATION_READY: u64 = 8u64;
pub const CLUSGROUPSET_STATUS_GROUPS_ONLINE: u64 = 2u64;
pub const CLUSGROUPSET_STATUS_GROUPS_PENDING: u64 = 1u64;
pub const CLUSGROUPSET_STATUS_OS_HEARTBEAT: u64 = 4u64;
pub const ClusGroupTypeCoreCluster: i32 = 1i32;
pub const ClusGroupTypeAvailableStorage: i32 = 2i32;
pub const ClusGroupTypeTemporary: i32 = 3i32;
pub const ClusGroupTypeSharedVolume: i32 = 4i32;
pub const ClusGroupTypeStoragePool: i32 = 5i32;
pub const ClusGroupTypeFileServer: i32 = 100i32;
pub const ClusGroupTypePrintServer: i32 = 101i32;
pub const ClusGroupTypeDhcpServer: i32 = 102i32;
pub const ClusGroupTypeDtc: i32 = 103i32;
pub const ClusGroupTypeMsmq: i32 = 104i32;
pub const ClusGroupTypeWins: i32 = 105i32;
pub const ClusGroupTypeStandAloneDfs: i32 = 106i32;
pub const ClusGroupTypeGenericApplication: i32 = 107i32;
pub const ClusGroupTypeGenericService: i32 = 108i32;
pub const ClusGroupTypeGenericScript: i32 = 109i32;
pub const ClusGroupTypeIScsiNameService: i32 = 110i32;
pub const ClusGroupTypeVirtualMachine: i32 = 111i32;
pub const ClusGroupTypeTsSessionBroker: i32 = 112i32;
pub const ClusGroupTypeIScsiTarget: i32 = 113i32;
pub const ClusGroupTypeScaleoutFileServer: i32 = 114i32;
pub const ClusGroupTypeVMReplicaBroker: i32 = 115i32;
pub const ClusGroupTypeTaskScheduler: i32 = 116i32;
pub const ClusGroupTypeClusterUpdateAgent: i32 = 117i32;
pub const ClusGroupTypeScaleoutCluster: i32 = 118i32;
pub const ClusGroupTypeStorageReplica: i32 = 119i32;
pub const ClusGroupTypeVMReplicaCoordinator: i32 = 120i32;
pub const ClusGroupTypeCrossClusterOrchestrator: i32 = 121i32;
pub const ClusGroupTypeInfrastructureFileServer: i32 = 122i32;
pub const ClusGroupTypeCoreSddc: i32 = 123i32;
pub const ClusGroupTypeUnknown: i32 = 9999i32;
pub const CLUSGRP_STATUS_APPLICATION_READY: u64 = 1024u64;
pub const CLUSGRP_STATUS_EMBEDDED_FAILURE: u64 = 32u64;
pub const CLUSGRP_STATUS_LOCKED_MODE: u64 = 1u64;
pub const CLUSGRP_STATUS_NETWORK_FAILURE: u64 = 128u64;
pub const CLUSGRP_STATUS_OFFLINE_DUE_TO_ANTIAFFINITY_CONFLICT: u64 = 64u64;
pub const CLUSGRP_STATUS_OFFLINE_NOT_LOCAL_DISK_OWNER: u64 = 2048u64;
pub const CLUSGRP_STATUS_OS_HEARTBEAT: u64 = 512u64;
pub const CLUSGRP_STATUS_PHYSICAL_RESOURCES_LACKING: u64 = 8u64;
pub const CLUSGRP_STATUS_PREEMPTED: u64 = 2u64;
pub const CLUSGRP_STATUS_UNMONITORED: u64 = 256u64;
pub const CLUSGRP_STATUS_WAITING_FOR_DEPENDENCIES: u64 = 4096u64;
pub const CLUSGRP_STATUS_WAITING_IN_QUEUE_FOR_MOVE: u64 = 4u64;
pub const CLUSGRP_STATUS_WAITING_TO_START: u64 = 16u64;
#[repr(C)]
pub struct CLUSPROP_BINARY {
    pub __AnonymousBase_clusapi_L5129_C41: CLUSPROP_VALUE,
    pub rgb: [u8; 1],
}
impl ::core::marker::Copy for CLUSPROP_BINARY {}
impl ::core::clone::Clone for CLUSPROP_BINARY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub union CLUSPROP_BUFFER_HELPER {
    pub pb: *mut u8,
    pub pw: *mut u16,
    pub pdw: *mut u32,
    pub pl: *mut i32,
    pub psz: super::super::Foundation::PWSTR,
    pub pList: *mut CLUSPROP_LIST,
    pub pSyntax: *mut CLUSPROP_SYNTAX,
    pub pName: *mut CLUSPROP_SZ,
    pub pValue: *mut CLUSPROP_VALUE,
    pub pBinaryValue: *mut CLUSPROP_BINARY,
    pub pWordValue: *mut CLUSPROP_WORD,
    pub pDwordValue: *mut CLUSPROP_DWORD,
    pub pLongValue: *mut CLUSPROP_LONG,
    pub pULargeIntegerValue: *mut CLUSPROP_ULARGE_INTEGER,
    pub pLargeIntegerValue: *mut CLUSPROP_LARGE_INTEGER,
    pub pStringValue: *mut CLUSPROP_SZ,
    pub pMultiSzValue: *mut CLUSPROP_SZ,
    pub pSecurityDescriptor: *mut CLUSPROP_SECURITY_DESCRIPTOR,
    pub pResourceClassValue: *mut CLUSPROP_RESOURCE_CLASS,
    pub pResourceClassInfoValue: *mut CLUSPROP_RESOURCE_CLASS_INFO,
    pub pDiskSignatureValue: *mut CLUSPROP_DWORD,
    pub pScsiAddressValue: *mut CLUSPROP_SCSI_ADDRESS,
    pub pDiskNumberValue: *mut CLUSPROP_DWORD,
    pub pPartitionInfoValue: *mut CLUSPROP_PARTITION_INFO,
    pub pRequiredDependencyValue: *mut CLUSPROP_REQUIRED_DEPENDENCY,
    pub pPartitionInfoValueEx: *mut CLUSPROP_PARTITION_INFO_EX,
    pub pPartitionInfoValueEx2: *mut CLUSPROP_PARTITION_INFO_EX2,
    pub pFileTimeValue: *mut CLUSPROP_FILETIME,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::core::marker::Copy for CLUSPROP_BUFFER_HELPER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::core::clone::Clone for CLUSPROP_BUFFER_HELPER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUSPROP_DWORD {
    pub __AnonymousBase_clusapi_L5149_C40: CLUSPROP_VALUE,
    pub dw: u32,
}
impl ::core::marker::Copy for CLUSPROP_DWORD {}
impl ::core::clone::Clone for CLUSPROP_DWORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUSPROP_FILETIME {
    pub __AnonymousBase_clusapi_L5225_C14: CLUSPROP_VALUE,
    pub ft: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUSPROP_FILETIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUSPROP_FILETIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUSPROP_FTSET_INFO {
    pub __AnonymousBase_clusapi_L5555_C14: CLUSPROP_VALUE,
    pub __AnonymousBase_clusapi_L5556_C14: CLUS_FTSET_INFO,
}
impl ::core::marker::Copy for CLUSPROP_FTSET_INFO {}
impl ::core::clone::Clone for CLUSPROP_FTSET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLUSPROP_IPADDR_ENABLENETBIOS_DISABLED: i32 = 0i32;
pub const CLUSPROP_IPADDR_ENABLENETBIOS_ENABLED: i32 = 1i32;
pub const CLUSPROP_IPADDR_ENABLENETBIOS_TRACK_NIC: i32 = 2i32;
#[repr(C)]
pub struct CLUSPROP_LARGE_INTEGER {
    pub __AnonymousBase_clusapi_L5199_C14: CLUSPROP_VALUE,
    pub li: i64,
}
impl ::core::marker::Copy for CLUSPROP_LARGE_INTEGER {}
impl ::core::clone::Clone for CLUSPROP_LARGE_INTEGER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUSPROP_LIST {
    pub nPropertyCount: u32,
    pub PropertyName: CLUSPROP_SZ,
}
impl ::core::marker::Copy for CLUSPROP_LIST {}
impl ::core::clone::Clone for CLUSPROP_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUSPROP_LONG {
    pub __AnonymousBase_clusapi_L5159_C39: CLUSPROP_VALUE,
    pub l: i32,
}
impl ::core::marker::Copy for CLUSPROP_LONG {}
impl ::core::clone::Clone for CLUSPROP_LONG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUSPROP_PARTITION_INFO {
    pub __AnonymousBase_clusapi_L5507_C14: CLUSPROP_VALUE,
    pub __AnonymousBase_clusapi_L5508_C14: CLUS_PARTITION_INFO,
}
impl ::core::marker::Copy for CLUSPROP_PARTITION_INFO {}
impl ::core::clone::Clone for CLUSPROP_PARTITION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUSPROP_PARTITION_INFO_EX {
    pub __AnonymousBase_clusapi_L5519_C14: CLUSPROP_VALUE,
    pub __AnonymousBase_clusapi_L5520_C14: CLUS_PARTITION_INFO_EX,
}
impl ::core::marker::Copy for CLUSPROP_PARTITION_INFO_EX {}
impl ::core::clone::Clone for CLUSPROP_PARTITION_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUSPROP_PARTITION_INFO_EX2 {
    pub __AnonymousBase_clusapi_L5533_C14: CLUSPROP_PARTITION_INFO_EX,
    pub __AnonymousBase_clusapi_L5534_C14: CLUS_PARTITION_INFO_EX2,
}
impl ::core::marker::Copy for CLUSPROP_PARTITION_INFO_EX2 {}
impl ::core::clone::Clone for CLUSPROP_PARTITION_INFO_EX2 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLUSPROP_PIFLAG_STICKY: i32 = 1i32;
pub const CLUSPROP_PIFLAG_REMOVABLE: i32 = 2i32;
pub const CLUSPROP_PIFLAG_USABLE: i32 = 4i32;
pub const CLUSPROP_PIFLAG_DEFAULT_QUORUM: i32 = 8i32;
pub const CLUSPROP_PIFLAG_USABLE_FOR_CSV: i32 = 16i32;
pub const CLUSPROP_PIFLAG_ENCRYPTION_ENABLED: i32 = 32i32;
pub const CLUSPROP_PIFLAG_RAW: i32 = 64i32;
pub const CLUSPROP_PIFLAG_UNKNOWN: i32 = -2147483648i32;
#[repr(C)]
pub union CLUSPROP_REQUIRED_DEPENDENCY {
    pub Value: CLUSPROP_VALUE,
    pub ResClass: CLUSPROP_RESOURCE_CLASS,
    pub ResTypeName: CLUSPROP_SZ,
}
impl ::core::marker::Copy for CLUSPROP_REQUIRED_DEPENDENCY {}
impl ::core::clone::Clone for CLUSPROP_REQUIRED_DEPENDENCY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUSPROP_RESOURCE_CLASS {
    pub __AnonymousBase_clusapi_L5250_C14: CLUSPROP_VALUE,
    pub rc: CLUSTER_RESOURCE_CLASS,
}
impl ::core::marker::Copy for CLUSPROP_RESOURCE_CLASS {}
impl ::core::clone::Clone for CLUSPROP_RESOURCE_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUSPROP_RESOURCE_CLASS_INFO {
    pub __AnonymousBase_clusapi_L5261_C14: CLUSPROP_VALUE,
    pub __AnonymousBase_clusapi_L5262_C14: CLUS_RESOURCE_CLASS_INFO,
}
impl ::core::marker::Copy for CLUSPROP_RESOURCE_CLASS_INFO {}
impl ::core::clone::Clone for CLUSPROP_RESOURCE_CLASS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUSPROP_SCSI_ADDRESS {
    pub __AnonymousBase_clusapi_L5583_C14: CLUSPROP_VALUE,
    pub __AnonymousBase_clusapi_L5584_C14: CLUS_SCSI_ADDRESS,
}
impl ::core::marker::Copy for CLUSPROP_SCSI_ADDRESS {}
impl ::core::clone::Clone for CLUSPROP_SCSI_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct CLUSPROP_SECURITY_DESCRIPTOR {
    pub __AnonymousBase_clusapi_L5211_C54: CLUSPROP_VALUE,
    pub Anonymous: CLUSPROP_SECURITY_DESCRIPTOR_0,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::core::marker::Copy for CLUSPROP_SECURITY_DESCRIPTOR {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::core::clone::Clone for CLUSPROP_SECURITY_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub union CLUSPROP_SECURITY_DESCRIPTOR_0 {
    pub sd: super::super::System::SystemServices::SECURITY_DESCRIPTOR_RELATIVE,
    pub rgbSecurityDescriptor: [u8; 1],
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::core::marker::Copy for CLUSPROP_SECURITY_DESCRIPTOR_0 {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::core::clone::Clone for CLUSPROP_SECURITY_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union CLUSPROP_SYNTAX {
    pub dw: u32,
    pub Anonymous: CLUSPROP_SYNTAX_0,
}
impl ::core::marker::Copy for CLUSPROP_SYNTAX {}
impl ::core::clone::Clone for CLUSPROP_SYNTAX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUSPROP_SYNTAX_0 {
    pub wFormat: u16,
    pub wType: u16,
}
impl ::core::marker::Copy for CLUSPROP_SYNTAX_0 {}
impl ::core::clone::Clone for CLUSPROP_SYNTAX_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUSPROP_SZ {
    pub __AnonymousBase_clusapi_L5169_C37: CLUSPROP_VALUE,
    pub sz: [u16; 1],
}
impl ::core::marker::Copy for CLUSPROP_SZ {}
impl ::core::clone::Clone for CLUSPROP_SZ {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUSPROP_ULARGE_INTEGER {
    pub __AnonymousBase_clusapi_L5186_C14: CLUSPROP_VALUE,
    pub li: u64,
}
impl ::core::marker::Copy for CLUSPROP_ULARGE_INTEGER {}
impl ::core::clone::Clone for CLUSPROP_ULARGE_INTEGER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUSPROP_VALUE {
    pub Syntax: CLUSPROP_SYNTAX,
    pub cbLength: u32,
}
impl ::core::marker::Copy for CLUSPROP_VALUE {}
impl ::core::clone::Clone for CLUSPROP_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUSPROP_WORD {
    pub __AnonymousBase_clusapi_L5139_C39: CLUSPROP_VALUE,
    pub w: u16,
}
impl ::core::marker::Copy for CLUSPROP_WORD {}
impl ::core::clone::Clone for CLUSPROP_WORD {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLUSREG_DATABASE_ISOLATE_READ: u32 = 2u32;
pub const CLUSREG_DATABASE_SYNC_WRITE_TO_ALL_NODES: u32 = 1u32;
pub const CLUSRESDLL_STATUS_DO_NOT_COLLECT_WER_REPORT: u32 = 1073741824u32;
pub const CLUSRESDLL_STATUS_DUMP_NOW: u32 = 2147483648u32;
pub const CLUSRESDLL_STATUS_INSUFFICIENT_MEMORY: u32 = 16u32;
pub const CLUSRESDLL_STATUS_INSUFFICIENT_OTHER_RESOURCES: u32 = 64u32;
pub const CLUSRESDLL_STATUS_INSUFFICIENT_PROCESSOR: u32 = 32u32;
pub const CLUSRESDLL_STATUS_INVALID_PARAMETERS: u32 = 128u32;
pub const CLUSRESDLL_STATUS_NETWORK_NOT_AVAILABLE: u32 = 256u32;
pub const CLUSRESDLL_STATUS_OFFLINE_BUSY: u32 = 1u32;
pub const CLUSRESDLL_STATUS_OFFLINE_DESTINATION_REJECTED: u32 = 8u32;
pub const CLUSRESDLL_STATUS_OFFLINE_DESTINATION_THROTTLED: u32 = 4u32;
pub const CLUSRESDLL_STATUS_OFFLINE_SOURCE_THROTTLED: u32 = 2u32;
pub const CLUSRES_STATUS_APPLICATION_READY: u64 = 256u64;
pub const CLUSRES_STATUS_EMBEDDED_FAILURE: u64 = 2u64;
pub const CLUSRES_STATUS_FAILED_DUE_TO_INSUFFICIENT_CPU: u64 = 4u64;
pub const CLUSRES_STATUS_FAILED_DUE_TO_INSUFFICIENT_GENERIC_RESOURCES: u64 = 16u64;
pub const CLUSRES_STATUS_FAILED_DUE_TO_INSUFFICIENT_MEMORY: u64 = 8u64;
pub const CLUSRES_STATUS_LOCKED_MODE: u64 = 1u64;
pub const CLUSRES_STATUS_NETWORK_FAILURE: u64 = 32u64;
pub const CLUSRES_STATUS_OFFLINE_NOT_LOCAL_DISK_OWNER: u64 = 512u64;
pub const CLUSRES_STATUS_OS_HEARTBEAT: u64 = 128u64;
pub const CLUSRES_STATUS_UNMONITORED: u64 = 64u64;
pub const CLUSTERSET_OBJECT_TYPE_NONE: i32 = 0i32;
pub const CLUSTERSET_OBJECT_TYPE_MEMBER: i32 = 1i32;
pub const CLUSTERSET_OBJECT_TYPE_WORKLOAD: i32 = 2i32;
pub const CLUSTERSET_OBJECT_TYPE_DATABASE: i32 = 3i32;
#[repr(C)]
pub struct CLUSTERVERSIONINFO {
    pub dwVersionInfoSize: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub BuildNumber: u16,
    pub szVendorId: [u16; 64],
    pub szCSDVersion: [u16; 64],
    pub dwClusterHighestVersion: u32,
    pub dwClusterLowestVersion: u32,
    pub dwFlags: u32,
    pub dwReserved: u32,
}
impl ::core::marker::Copy for CLUSTERVERSIONINFO {}
impl ::core::clone::Clone for CLUSTERVERSIONINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUSTERVERSIONINFO_NT4 {
    pub dwVersionInfoSize: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub BuildNumber: u16,
    pub szVendorId: [u16; 64],
    pub szCSDVersion: [u16; 64],
}
impl ::core::marker::Copy for CLUSTERVERSIONINFO_NT4 {}
impl ::core::clone::Clone for CLUSTERVERSIONINFO_NT4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUSTER_AVAILABILITY_SET_CONFIG {
    pub dwVersion: u32,
    pub dwUpdateDomains: u32,
    pub dwFaultDomains: u32,
    pub bReserveSpareNode: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUSTER_AVAILABILITY_SET_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUSTER_AVAILABILITY_SET_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLUSTER_AVAILABILITY_SET_CONFIG_V1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUSTER_BATCH_COMMAND {
    pub Command: CLUSTER_REG_COMMAND,
    pub dwOptions: u32,
    pub wzName: super::super::Foundation::PWSTR,
    pub lpData: *mut u8,
    pub cbData: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUSTER_BATCH_COMMAND {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUSTER_BATCH_COMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLUSTER_CHANGE_NODE_STATE: i32 = 1i32;
pub const CLUSTER_CHANGE_NODE_DELETED: i32 = 2i32;
pub const CLUSTER_CHANGE_NODE_ADDED: i32 = 4i32;
pub const CLUSTER_CHANGE_NODE_PROPERTY: i32 = 8i32;
pub const CLUSTER_CHANGE_REGISTRY_NAME: i32 = 16i32;
pub const CLUSTER_CHANGE_REGISTRY_ATTRIBUTES: i32 = 32i32;
pub const CLUSTER_CHANGE_REGISTRY_VALUE: i32 = 64i32;
pub const CLUSTER_CHANGE_REGISTRY_SUBTREE: i32 = 128i32;
pub const CLUSTER_CHANGE_RESOURCE_STATE: i32 = 256i32;
pub const CLUSTER_CHANGE_RESOURCE_DELETED: i32 = 512i32;
pub const CLUSTER_CHANGE_RESOURCE_ADDED: i32 = 1024i32;
pub const CLUSTER_CHANGE_RESOURCE_PROPERTY: i32 = 2048i32;
pub const CLUSTER_CHANGE_GROUP_STATE: i32 = 4096i32;
pub const CLUSTER_CHANGE_GROUP_DELETED: i32 = 8192i32;
pub const CLUSTER_CHANGE_GROUP_ADDED: i32 = 16384i32;
pub const CLUSTER_CHANGE_GROUP_PROPERTY: i32 = 32768i32;
pub const CLUSTER_CHANGE_RESOURCE_TYPE_DELETED: i32 = 65536i32;
pub const CLUSTER_CHANGE_RESOURCE_TYPE_ADDED: i32 = 131072i32;
pub const CLUSTER_CHANGE_RESOURCE_TYPE_PROPERTY: i32 = 262144i32;
pub const CLUSTER_CHANGE_CLUSTER_RECONNECT: i32 = 524288i32;
pub const CLUSTER_CHANGE_NETWORK_STATE: i32 = 1048576i32;
pub const CLUSTER_CHANGE_NETWORK_DELETED: i32 = 2097152i32;
pub const CLUSTER_CHANGE_NETWORK_ADDED: i32 = 4194304i32;
pub const CLUSTER_CHANGE_NETWORK_PROPERTY: i32 = 8388608i32;
pub const CLUSTER_CHANGE_NETINTERFACE_STATE: i32 = 16777216i32;
pub const CLUSTER_CHANGE_NETINTERFACE_DELETED: i32 = 33554432i32;
pub const CLUSTER_CHANGE_NETINTERFACE_ADDED: i32 = 67108864i32;
pub const CLUSTER_CHANGE_NETINTERFACE_PROPERTY: i32 = 134217728i32;
pub const CLUSTER_CHANGE_QUORUM_STATE: i32 = 268435456i32;
pub const CLUSTER_CHANGE_CLUSTER_STATE: i32 = 536870912i32;
pub const CLUSTER_CHANGE_CLUSTER_PROPERTY: i32 = 1073741824i32;
pub const CLUSTER_CHANGE_HANDLE_CLOSE: i32 = -2147483648i32;
pub const CLUSTER_CHANGE_ALL: i32 = -1i32;
pub const CLUSTER_CHANGE_CLUSTER_RECONNECT_V2: i32 = 1i32;
pub const CLUSTER_CHANGE_CLUSTER_STATE_V2: i32 = 2i32;
pub const CLUSTER_CHANGE_CLUSTER_GROUP_ADDED_V2: i32 = 4i32;
pub const CLUSTER_CHANGE_CLUSTER_HANDLE_CLOSE_V2: i32 = 8i32;
pub const CLUSTER_CHANGE_CLUSTER_NETWORK_ADDED_V2: i32 = 16i32;
pub const CLUSTER_CHANGE_CLUSTER_NODE_ADDED_V2: i32 = 32i32;
pub const CLUSTER_CHANGE_CLUSTER_RESOURCE_TYPE_ADDED_V2: i32 = 64i32;
pub const CLUSTER_CHANGE_CLUSTER_COMMON_PROPERTY_V2: i32 = 128i32;
pub const CLUSTER_CHANGE_CLUSTER_PRIVATE_PROPERTY_V2: i32 = 256i32;
pub const CLUSTER_CHANGE_CLUSTER_LOST_NOTIFICATIONS_V2: i32 = 512i32;
pub const CLUSTER_CHANGE_CLUSTER_RENAME_V2: i32 = 1024i32;
pub const CLUSTER_CHANGE_CLUSTER_MEMBERSHIP_V2: i32 = 2048i32;
pub const CLUSTER_CHANGE_CLUSTER_UPGRADED_V2: i32 = 4096i32;
pub const CLUSTER_CHANGE_CLUSTER_ALL_V2: i32 = 8191i32;
pub const CLUSTER_CHANGE_GROUPSET_DELETED_v2: i32 = 1i32;
pub const CLUSTER_CHANGE_GROUPSET_COMMON_PROPERTY_V2: i32 = 2i32;
pub const CLUSTER_CHANGE_GROUPSET_PRIVATE_PROPERTY_V2: i32 = 4i32;
pub const CLUSTER_CHANGE_GROUPSET_STATE_V2: i32 = 8i32;
pub const CLUSTER_CHANGE_GROUPSET_GROUP_ADDED: i32 = 16i32;
pub const CLUSTER_CHANGE_GROUPSET_GROUP_REMOVED: i32 = 32i32;
pub const CLUSTER_CHANGE_GROUPSET_DEPENDENCIES_V2: i32 = 64i32;
pub const CLUSTER_CHANGE_GROUPSET_DEPENDENTS_V2: i32 = 128i32;
pub const CLUSTER_CHANGE_GROUPSET_HANDLE_CLOSE_v2: i32 = 256i32;
pub const CLUSTER_CHANGE_GROUPSET_ALL_V2: i32 = 511i32;
pub const CLUSTER_CHANGE_GROUP_DELETED_V2: i32 = 1i32;
pub const CLUSTER_CHANGE_GROUP_COMMON_PROPERTY_V2: i32 = 2i32;
pub const CLUSTER_CHANGE_GROUP_PRIVATE_PROPERTY_V2: i32 = 4i32;
pub const CLUSTER_CHANGE_GROUP_STATE_V2: i32 = 8i32;
pub const CLUSTER_CHANGE_GROUP_OWNER_NODE_V2: i32 = 16i32;
pub const CLUSTER_CHANGE_GROUP_PREFERRED_OWNERS_V2: i32 = 32i32;
pub const CLUSTER_CHANGE_GROUP_RESOURCE_ADDED_V2: i32 = 64i32;
pub const CLUSTER_CHANGE_GROUP_RESOURCE_GAINED_V2: i32 = 128i32;
pub const CLUSTER_CHANGE_GROUP_RESOURCE_LOST_V2: i32 = 256i32;
pub const CLUSTER_CHANGE_GROUP_HANDLE_CLOSE_V2: i32 = 512i32;
pub const CLUSTER_CHANGE_GROUP_ALL_V2: i32 = 1023i32;
pub const CLUSTER_CHANGE_NETINTERFACE_DELETED_V2: i32 = 1i32;
pub const CLUSTER_CHANGE_NETINTERFACE_COMMON_PROPERTY_V2: i32 = 2i32;
pub const CLUSTER_CHANGE_NETINTERFACE_PRIVATE_PROPERTY_V2: i32 = 4i32;
pub const CLUSTER_CHANGE_NETINTERFACE_STATE_V2: i32 = 8i32;
pub const CLUSTER_CHANGE_NETINTERFACE_HANDLE_CLOSE_V2: i32 = 16i32;
pub const CLUSTER_CHANGE_NETINTERFACE_ALL_V2: i32 = 31i32;
pub const CLUSTER_CHANGE_NETWORK_DELETED_V2: i32 = 1i32;
pub const CLUSTER_CHANGE_NETWORK_COMMON_PROPERTY_V2: i32 = 2i32;
pub const CLUSTER_CHANGE_NETWORK_PRIVATE_PROPERTY_V2: i32 = 4i32;
pub const CLUSTER_CHANGE_NETWORK_STATE_V2: i32 = 8i32;
pub const CLUSTER_CHANGE_NETWORK_HANDLE_CLOSE_V2: i32 = 16i32;
pub const CLUSTER_CHANGE_NETWORK_ALL_V2: i32 = 31i32;
pub const CLUSTER_CHANGE_UPGRADE_NODE_PREPARE: i32 = 1i32;
pub const CLUSTER_CHANGE_UPGRADE_NODE_COMMIT: i32 = 2i32;
pub const CLUSTER_CHANGE_UPGRADE_NODE_POSTCOMMIT: i32 = 4i32;
pub const CLUSTER_CHANGE_UPGRADE_ALL: i32 = 7i32;
pub const CLUSTER_CHANGE_NODE_NETINTERFACE_ADDED_V2: i32 = 1i32;
pub const CLUSTER_CHANGE_NODE_DELETED_V2: i32 = 2i32;
pub const CLUSTER_CHANGE_NODE_COMMON_PROPERTY_V2: i32 = 4i32;
pub const CLUSTER_CHANGE_NODE_PRIVATE_PROPERTY_V2: i32 = 8i32;
pub const CLUSTER_CHANGE_NODE_STATE_V2: i32 = 16i32;
pub const CLUSTER_CHANGE_NODE_GROUP_GAINED_V2: i32 = 32i32;
pub const CLUSTER_CHANGE_NODE_GROUP_LOST_V2: i32 = 64i32;
pub const CLUSTER_CHANGE_NODE_HANDLE_CLOSE_V2: i32 = 128i32;
pub const CLUSTER_CHANGE_NODE_ALL_V2: i32 = 255i32;
pub const CLUSTER_CHANGE_QUORUM_STATE_V2: i32 = 1i32;
pub const CLUSTER_CHANGE_QUORUM_ALL_V2: i32 = 1i32;
pub const CLUSTER_CHANGE_REGISTRY_ATTRIBUTES_V2: i32 = 1i32;
pub const CLUSTER_CHANGE_REGISTRY_NAME_V2: i32 = 2i32;
pub const CLUSTER_CHANGE_REGISTRY_SUBTREE_V2: i32 = 4i32;
pub const CLUSTER_CHANGE_REGISTRY_VALUE_V2: i32 = 8i32;
pub const CLUSTER_CHANGE_REGISTRY_HANDLE_CLOSE_V2: i32 = 16i32;
pub const CLUSTER_CHANGE_REGISTRY_ALL_V2: i32 = 31i32;
pub const CLUSTER_CHANGE_RESOURCE_TYPE_DELETED_V2: i32 = 1i32;
pub const CLUSTER_CHANGE_RESOURCE_TYPE_COMMON_PROPERTY_V2: i32 = 2i32;
pub const CLUSTER_CHANGE_RESOURCE_TYPE_PRIVATE_PROPERTY_V2: i32 = 4i32;
pub const CLUSTER_CHANGE_RESOURCE_TYPE_POSSIBLE_OWNERS_V2: i32 = 8i32;
pub const CLUSTER_CHANGE_RESOURCE_TYPE_DLL_UPGRADED_V2: i32 = 16i32;
pub const CLUSTER_RESOURCE_TYPE_SPECIFIC_V2: i32 = 32i32;
pub const CLUSTER_CHANGE_RESOURCE_TYPE_ALL_V2: i32 = 63i32;
pub const CLUSTER_CHANGE_RESOURCE_COMMON_PROPERTY_V2: i32 = 1i32;
pub const CLUSTER_CHANGE_RESOURCE_PRIVATE_PROPERTY_V2: i32 = 2i32;
pub const CLUSTER_CHANGE_RESOURCE_STATE_V2: i32 = 4i32;
pub const CLUSTER_CHANGE_RESOURCE_OWNER_GROUP_V2: i32 = 8i32;
pub const CLUSTER_CHANGE_RESOURCE_DEPENDENCIES_V2: i32 = 16i32;
pub const CLUSTER_CHANGE_RESOURCE_DEPENDENTS_V2: i32 = 32i32;
pub const CLUSTER_CHANGE_RESOURCE_POSSIBLE_OWNERS_V2: i32 = 64i32;
pub const CLUSTER_CHANGE_RESOURCE_DELETED_V2: i32 = 128i32;
pub const CLUSTER_CHANGE_RESOURCE_DLL_UPGRADED_V2: i32 = 256i32;
pub const CLUSTER_CHANGE_RESOURCE_HANDLE_CLOSE_V2: i32 = 512i32;
pub const CLUSTER_CHANGE_RESOURCE_TERMINAL_STATE_V2: i32 = 1024i32;
pub const CLUSTER_CHANGE_RESOURCE_ALL_V2: i32 = 2047i32;
pub const CLUSTER_CHANGE_SHARED_VOLUME_STATE_V2: i32 = 1i32;
pub const CLUSTER_CHANGE_SHARED_VOLUME_ADDED_V2: i32 = 2i32;
pub const CLUSTER_CHANGE_SHARED_VOLUME_REMOVED_V2: i32 = 4i32;
pub const CLUSTER_CHANGE_SHARED_VOLUME_ALL_V2: i32 = 7i32;
pub const CLUSTER_CHANGE_SPACEPORT_CUSTOM_PNP_V2: i32 = 1i32;
pub const CLUSTER_CLOUD_TYPE_NONE: i32 = 0i32;
pub const CLUSTER_CLOUD_TYPE_AZURE: i32 = 1i32;
pub const CLUSTER_CLOUD_TYPE_MIXED: i32 = 128i32;
pub const CLUSTER_CLOUD_TYPE_UNKNOWN: i32 = -1i32;
pub const CLUSTER_CONFIGURED: u32 = 2u32;
pub const CLUS_OBJECT_INVALID: i32 = 0i32;
pub const CLUS_OBJECT_RESOURCE: i32 = 1i32;
pub const CLUS_OBJECT_RESOURCE_TYPE: i32 = 2i32;
pub const CLUS_OBJECT_GROUP: i32 = 3i32;
pub const CLUS_OBJECT_NODE: i32 = 4i32;
pub const CLUS_OBJECT_NETWORK: i32 = 5i32;
pub const CLUS_OBJECT_NETINTERFACE: i32 = 6i32;
pub const CLUS_OBJECT_CLUSTER: i32 = 7i32;
pub const CLUS_OBJECT_GROUPSET: i32 = 8i32;
pub const CLUS_OBJECT_AFFINITYRULE: i32 = 9i32;
pub const CLUS_OBJECT_USER: i32 = 128i32;
#[repr(C)]
pub struct CLUSTER_CREATE_GROUP_INFO {
    pub dwVersion: u32,
    pub groupType: CLUSGROUP_TYPE,
}
impl ::core::marker::Copy for CLUSTER_CREATE_GROUP_INFO {}
impl ::core::clone::Clone for CLUSTER_CREATE_GROUP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLUSTER_CREATE_GROUP_INFO_VERSION: u32 = 1u32;
pub const CLUSTER_CREATE_GROUP_INFO_VERSION_1: u32 = 1u32;
pub const VolumeStateNoFaults: i32 = 0i32;
pub const VolumeStateNoDirectIO: i32 = 1i32;
pub const VolumeStateNoAccess: i32 = 2i32;
pub const VolumeStateInMaintenance: i32 = 4i32;
pub const VolumeStateDismounted: i32 = 8i32;
pub const CLUSTER_DELETE_ACCESS_CONTROL_ENTRY: u32 = 2u32;
pub const CLUSTER_ENUM_NODE: i32 = 1i32;
pub const CLUSTER_ENUM_RESTYPE: i32 = 2i32;
pub const CLUSTER_ENUM_RESOURCE: i32 = 4i32;
pub const CLUSTER_ENUM_GROUP: i32 = 8i32;
pub const CLUSTER_ENUM_NETWORK: i32 = 16i32;
pub const CLUSTER_ENUM_NETINTERFACE: i32 = 32i32;
pub const CLUSTER_ENUM_SHARED_VOLUME_GROUP: i32 = 536870912i32;
pub const CLUSTER_ENUM_SHARED_VOLUME_RESOURCE: i32 = 1073741824i32;
pub const CLUSTER_ENUM_INTERNAL_NETWORK: i32 = -2147483648i32;
pub const CLUSTER_ENUM_ALL: i32 = 63i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUSTER_ENUM_ITEM {
    pub dwVersion: u32,
    pub dwType: u32,
    pub cbId: u32,
    pub lpszId: super::super::Foundation::PWSTR,
    pub cbName: u32,
    pub lpszName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUSTER_ENUM_ITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUSTER_ENUM_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLUSTER_ENUM_ITEM_VERSION: u32 = 1u32;
pub const CLUSTER_ENUM_ITEM_VERSION_1: u32 = 1u32;
pub const ClusterGroupPreventFailback: i32 = 0i32;
pub const ClusterGroupAllowFailback: i32 = 1i32;
pub const ClusterGroupFailbackTypeCount: i32 = 2i32;
pub const CLUSTER_GROUP_ENUM_CONTAINS: i32 = 1i32;
pub const CLUSTER_GROUP_ENUM_NODES: i32 = 2i32;
pub const CLUSTER_GROUP_ENUM_ALL: i32 = 3i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUSTER_GROUP_ENUM_ITEM {
    pub dwVersion: u32,
    pub cbId: u32,
    pub lpszId: super::super::Foundation::PWSTR,
    pub cbName: u32,
    pub lpszName: super::super::Foundation::PWSTR,
    pub state: CLUSTER_GROUP_STATE,
    pub cbOwnerNode: u32,
    pub lpszOwnerNode: super::super::Foundation::PWSTR,
    pub dwFlags: u32,
    pub cbProperties: u32,
    pub pProperties: *mut ::core::ffi::c_void,
    pub cbRoProperties: u32,
    pub pRoProperties: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUSTER_GROUP_ENUM_ITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUSTER_GROUP_ENUM_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLUSTER_GROUP_ENUM_ITEM_VERSION: u32 = 1u32;
pub const CLUSTER_GROUP_ENUM_ITEM_VERSION_1: u32 = 1u32;
pub const PriorityDisabled: i32 = 0i32;
pub const PriorityLow: i32 = 1000i32;
pub const PriorityMedium: i32 = 2000i32;
pub const PriorityHigh: i32 = 3000i32;
pub const ClusterGroupStateUnknown: i32 = -1i32;
pub const ClusterGroupOnline: i32 = 0i32;
pub const ClusterGroupOffline: i32 = 1i32;
pub const ClusterGroupFailed: i32 = 2i32;
pub const ClusterGroupPartialOnline: i32 = 3i32;
pub const ClusterGroupPending: i32 = 4i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUSTER_HEALTH_FAULT {
    pub Id: super::super::Foundation::PWSTR,
    pub ErrorType: u32,
    pub ErrorCode: u32,
    pub Description: super::super::Foundation::PWSTR,
    pub Provider: super::super::Foundation::PWSTR,
    pub Flags: u32,
    pub Reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUSTER_HEALTH_FAULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUSTER_HEALTH_FAULT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLUSTER_HEALTH_FAULT_ARGS: u32 = 7u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUSTER_HEALTH_FAULT_ARRAY {
    pub numFaults: u32,
    pub faults: *mut CLUSTER_HEALTH_FAULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUSTER_HEALTH_FAULT_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUSTER_HEALTH_FAULT_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLUSTER_HEALTH_FAULT_DESCRIPTION: u32 = 3u32;
pub const CLUSTER_HEALTH_FAULT_ERRORCODE: u32 = 2u32;
pub const CLUSTER_HEALTH_FAULT_ERRORTYPE: u32 = 1u32;
pub const CLUSTER_HEALTH_FAULT_FLAGS: u32 = 5u32;
pub const CLUSTER_HEALTH_FAULT_ID: u32 = 0u32;
pub const CLUSTER_HEALTH_FAULT_PROVIDER: u32 = 4u32;
pub const CLUSTER_HEALTH_FAULT_RESERVED: u32 = 6u32;
pub const CLUSTER_INSTALLED: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUSTER_IP_ENTRY {
    pub lpszIpAddress: super::super::Foundation::PWSTR,
    pub dwPrefixLength: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUSTER_IP_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUSTER_IP_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUSTER_MEMBERSHIP_INFO {
    pub HasQuorum: super::super::Foundation::BOOL,
    pub UpnodesSize: u32,
    pub Upnodes: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUSTER_MEMBERSHIP_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUSTER_MEMBERSHIP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLUSTER_MGMT_POINT_RESTYPE_AUTO: i32 = 0i32;
pub const CLUSTER_MGMT_POINT_RESTYPE_SNN: i32 = 1i32;
pub const CLUSTER_MGMT_POINT_RESTYPE_DNN: i32 = 2i32;
pub const CLUSTER_MGMT_POINT_TYPE_NONE: i32 = 0i32;
pub const CLUSTER_MGMT_POINT_TYPE_CNO: i32 = 1i32;
pub const CLUSTER_MGMT_POINT_TYPE_DNS_ONLY: i32 = 2i32;
pub const CLUSTER_MGMT_POINT_TYPE_CNO_ONLY: i32 = 3i32;
pub const ClusterNetInterfaceStateUnknown: i32 = -1i32;
pub const ClusterNetInterfaceUnavailable: i32 = 0i32;
pub const ClusterNetInterfaceFailed: i32 = 1i32;
pub const ClusterNetInterfaceUnreachable: i32 = 2i32;
pub const ClusterNetInterfaceUp: i32 = 3i32;
pub const CLUSTER_NETWORK_ENUM_NETINTERFACES: i32 = 1i32;
pub const CLUSTER_NETWORK_ENUM_ALL: i32 = 1i32;
pub const ClusterNetworkRoleNone: i32 = 0i32;
pub const ClusterNetworkRoleInternalUse: i32 = 1i32;
pub const ClusterNetworkRoleClientAccess: i32 = 2i32;
pub const ClusterNetworkRoleInternalAndClient: i32 = 3i32;
pub const ClusterNetworkStateUnknown: i32 = -1i32;
pub const ClusterNetworkUnavailable: i32 = 0i32;
pub const ClusterNetworkDown: i32 = 1i32;
pub const ClusterNetworkPartitioned: i32 = 2i32;
pub const ClusterNetworkUp: i32 = 3i32;
pub const NodeDrainStatusNotInitiated: i32 = 0i32;
pub const NodeDrainStatusInProgress: i32 = 1i32;
pub const NodeDrainStatusCompleted: i32 = 2i32;
pub const NodeDrainStatusFailed: i32 = 3i32;
pub const ClusterNodeDrainStatusCount: i32 = 4i32;
pub const CLUSTER_NODE_ENUM_NETINTERFACES: i32 = 1i32;
pub const CLUSTER_NODE_ENUM_GROUPS: i32 = 2i32;
pub const CLUSTER_NODE_ENUM_PREFERRED_GROUPS: i32 = 4i32;
pub const CLUSTER_NODE_ENUM_ALL: i32 = 3i32;
pub const DoNotFailbackGroups: i32 = 0i32;
pub const FailbackGroupsImmediately: i32 = 1i32;
pub const FailbackGroupsPerPolicy: i32 = 2i32;
pub const ClusterNodeResumeFailbackTypeCount: i32 = 3i32;
pub const ClusterNodeStateUnknown: i32 = -1i32;
pub const ClusterNodeUp: i32 = 0i32;
pub const ClusterNodeDown: i32 = 1i32;
pub const ClusterNodePaused: i32 = 2i32;
pub const ClusterNodeJoining: i32 = 3i32;
pub const NodeStatusNormal: i32 = 0i32;
pub const NodeStatusIsolated: i32 = 1i32;
pub const NodeStatusQuarantined: i32 = 2i32;
pub const NodeStatusDrainInProgress: i32 = 4i32;
pub const NodeStatusDrainCompleted: i32 = 8i32;
pub const NodeStatusDrainFailed: i32 = 16i32;
pub const NodeStatusAvoidPlacement: i32 = 32i32;
pub const NodeStatusMax: i32 = 51i32;
pub const CLUSTER_NOTIFICATIONS_V1: i32 = 1i32;
pub const CLUSTER_NOTIFICATIONS_V2: i32 = 2i32;
pub const CLUSTER_OBJECT_TYPE_NONE: i32 = 0i32;
pub const CLUSTER_OBJECT_TYPE_CLUSTER: i32 = 1i32;
pub const CLUSTER_OBJECT_TYPE_GROUP: i32 = 2i32;
pub const CLUSTER_OBJECT_TYPE_RESOURCE: i32 = 3i32;
pub const CLUSTER_OBJECT_TYPE_RESOURCE_TYPE: i32 = 4i32;
pub const CLUSTER_OBJECT_TYPE_NETWORK_INTERFACE: i32 = 5i32;
pub const CLUSTER_OBJECT_TYPE_NETWORK: i32 = 6i32;
pub const CLUSTER_OBJECT_TYPE_NODE: i32 = 7i32;
pub const CLUSTER_OBJECT_TYPE_REGISTRY: i32 = 8i32;
pub const CLUSTER_OBJECT_TYPE_QUORUM: i32 = 9i32;
pub const CLUSTER_OBJECT_TYPE_SHARED_VOLUME: i32 = 10i32;
pub const CLUSTER_OBJECT_TYPE_GROUPSET: i32 = 13i32;
pub const CLUSTER_OBJECT_TYPE_AFFINITYRULE: i32 = 16i32;
pub const CLUSPROP_FORMAT_UNKNOWN: i32 = 0i32;
pub const CLUSPROP_FORMAT_BINARY: i32 = 1i32;
pub const CLUSPROP_FORMAT_DWORD: i32 = 2i32;
pub const CLUSPROP_FORMAT_SZ: i32 = 3i32;
pub const CLUSPROP_FORMAT_EXPAND_SZ: i32 = 4i32;
pub const CLUSPROP_FORMAT_MULTI_SZ: i32 = 5i32;
pub const CLUSPROP_FORMAT_ULARGE_INTEGER: i32 = 6i32;
pub const CLUSPROP_FORMAT_LONG: i32 = 7i32;
pub const CLUSPROP_FORMAT_EXPANDED_SZ: i32 = 8i32;
pub const CLUSPROP_FORMAT_SECURITY_DESCRIPTOR: i32 = 9i32;
pub const CLUSPROP_FORMAT_LARGE_INTEGER: i32 = 10i32;
pub const CLUSPROP_FORMAT_WORD: i32 = 11i32;
pub const CLUSPROP_FORMAT_FILETIME: i32 = 12i32;
pub const CLUSPROP_FORMAT_VALUE_LIST: i32 = 13i32;
pub const CLUSPROP_FORMAT_PROPERTY_LIST: i32 = 14i32;
pub const CLUSPROP_FORMAT_USER: i32 = 32768i32;
pub const CLUSPROP_SYNTAX_ENDMARK: u32 = 0u32;
pub const CLUSPROP_SYNTAX_NAME: u32 = 262147u32;
pub const CLUSPROP_SYNTAX_RESCLASS: u32 = 131074u32;
pub const CLUSPROP_SYNTAX_LIST_VALUE_SZ: u32 = 65539u32;
pub const CLUSPROP_SYNTAX_LIST_VALUE_EXPAND_SZ: u32 = 65540u32;
pub const CLUSPROP_SYNTAX_LIST_VALUE_DWORD: u32 = 65538u32;
pub const CLUSPROP_SYNTAX_LIST_VALUE_BINARY: u32 = 65537u32;
pub const CLUSPROP_SYNTAX_LIST_VALUE_MULTI_SZ: u32 = 65541u32;
pub const CLUSPROP_SYNTAX_LIST_VALUE_LONG: u32 = 65543u32;
pub const CLUSPROP_SYNTAX_LIST_VALUE_EXPANDED_SZ: u32 = 65544u32;
pub const CLUSPROP_SYNTAX_LIST_VALUE_SECURITY_DESCRIPTOR: u32 = 65545u32;
pub const CLUSPROP_SYNTAX_LIST_VALUE_LARGE_INTEGER: u32 = 65546u32;
pub const CLUSPROP_SYNTAX_LIST_VALUE_ULARGE_INTEGER: u32 = 65542u32;
pub const CLUSPROP_SYNTAX_LIST_VALUE_WORD: u32 = 65547u32;
pub const CLUSPROP_SYNTAX_LIST_VALUE_PROPERTY_LIST: u32 = 65550u32;
pub const CLUSPROP_SYNTAX_LIST_VALUE_FILETIME: u32 = 65548u32;
pub const CLUSPROP_SYNTAX_DISK_SIGNATURE: u32 = 327682u32;
pub const CLUSPROP_SYNTAX_SCSI_ADDRESS: u32 = 393218u32;
pub const CLUSPROP_SYNTAX_DISK_NUMBER: u32 = 458754u32;
pub const CLUSPROP_SYNTAX_PARTITION_INFO: u32 = 524289u32;
pub const CLUSPROP_SYNTAX_FTSET_INFO: u32 = 589825u32;
pub const CLUSPROP_SYNTAX_DISK_SERIALNUMBER: u32 = 655363u32;
pub const CLUSPROP_SYNTAX_DISK_GUID: u32 = 720899u32;
pub const CLUSPROP_SYNTAX_DISK_SIZE: u32 = 786438u32;
pub const CLUSPROP_SYNTAX_PARTITION_INFO_EX: u32 = 851969u32;
pub const CLUSPROP_SYNTAX_PARTITION_INFO_EX2: u32 = 917505u32;
pub const CLUSPROP_SYNTAX_STORAGE_DEVICE_ID_DESCRIPTOR: u32 = 983041u32;
pub const CLUSPROP_TYPE_UNKNOWN: i32 = -1i32;
pub const CLUSPROP_TYPE_ENDMARK: i32 = 0i32;
pub const CLUSPROP_TYPE_LIST_VALUE: i32 = 1i32;
pub const CLUSPROP_TYPE_RESCLASS: i32 = 2i32;
pub const CLUSPROP_TYPE_RESERVED1: i32 = 3i32;
pub const CLUSPROP_TYPE_NAME: i32 = 4i32;
pub const CLUSPROP_TYPE_SIGNATURE: i32 = 5i32;
pub const CLUSPROP_TYPE_SCSI_ADDRESS: i32 = 6i32;
pub const CLUSPROP_TYPE_DISK_NUMBER: i32 = 7i32;
pub const CLUSPROP_TYPE_PARTITION_INFO: i32 = 8i32;
pub const CLUSPROP_TYPE_FTSET_INFO: i32 = 9i32;
pub const CLUSPROP_TYPE_DISK_SERIALNUMBER: i32 = 10i32;
pub const CLUSPROP_TYPE_DISK_GUID: i32 = 11i32;
pub const CLUSPROP_TYPE_DISK_SIZE: i32 = 12i32;
pub const CLUSPROP_TYPE_PARTITION_INFO_EX: i32 = 13i32;
pub const CLUSPROP_TYPE_PARTITION_INFO_EX2: i32 = 14i32;
pub const CLUSPROP_TYPE_STORAGE_DEVICE_ID_DESCRIPTOR: i32 = 15i32;
pub const CLUSPROP_TYPE_USER: i32 = 32768i32;
pub const OperationalQuorum: i32 = 0i32;
pub const ModifyQuorum: i32 = 1i32;
pub const CLUSTER_QUORUM_MAINTAINED: i32 = 0i32;
pub const CLUSTER_QUORUM_LOST: i32 = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUSTER_READ_BATCH_COMMAND {
    pub Command: CLUSTER_REG_COMMAND,
    pub dwOptions: u32,
    pub wzSubkeyName: super::super::Foundation::PWSTR,
    pub wzValueName: super::super::Foundation::PWSTR,
    pub lpData: *mut u8,
    pub cbData: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUSTER_READ_BATCH_COMMAND {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUSTER_READ_BATCH_COMMAND {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLUSREG_COMMAND_NONE: i32 = 0i32;
pub const CLUSREG_SET_VALUE: i32 = 1i32;
pub const CLUSREG_CREATE_KEY: i32 = 2i32;
pub const CLUSREG_DELETE_KEY: i32 = 3i32;
pub const CLUSREG_DELETE_VALUE: i32 = 4i32;
pub const CLUSREG_SET_KEY_SECURITY: i32 = 5i32;
pub const CLUSREG_VALUE_DELETED: i32 = 6i32;
pub const CLUSREG_READ_KEY: i32 = 7i32;
pub const CLUSREG_READ_VALUE: i32 = 8i32;
pub const CLUSREG_READ_ERROR: i32 = 9i32;
pub const CLUSREG_CONTROL_COMMAND: i32 = 10i32;
pub const CLUSREG_CONDITION_EXISTS: i32 = 11i32;
pub const CLUSREG_CONDITION_NOT_EXISTS: i32 = 12i32;
pub const CLUSREG_CONDITION_IS_EQUAL: i32 = 13i32;
pub const CLUSREG_CONDITION_IS_NOT_EQUAL: i32 = 14i32;
pub const CLUSREG_CONDITION_IS_GREATER_THAN: i32 = 15i32;
pub const CLUSREG_CONDITION_IS_LESS_THAN: i32 = 16i32;
pub const CLUSREG_CONDITION_KEY_EXISTS: i32 = 17i32;
pub const CLUSREG_CONDITION_KEY_NOT_EXISTS: i32 = 18i32;
pub const CLUSREG_LAST_COMMAND: i32 = 19i32;
pub const ClusterResourceApplicationStateUnknown: i32 = 1i32;
pub const ClusterResourceApplicationOSHeartBeat: i32 = 2i32;
pub const ClusterResourceApplicationReady: i32 = 3i32;
pub const CLUS_RESCLASS_UNKNOWN: i32 = 0i32;
pub const CLUS_RESCLASS_STORAGE: i32 = 1i32;
pub const CLUS_RESCLASS_NETWORK: i32 = 2i32;
pub const CLUS_RESCLASS_USER: i32 = 32768i32;
pub const CLUSTER_RESOURCE_DEFAULT_MONITOR: i32 = 0i32;
pub const CLUSTER_RESOURCE_SEPARATE_MONITOR: i32 = 1i32;
pub const CLUSTER_RESOURCE_VALID_FLAGS: i32 = 1i32;
pub const ClusterResourceEmbeddedFailureActionNone: i32 = 0i32;
pub const ClusterResourceEmbeddedFailureActionLogOnly: i32 = 1i32;
pub const ClusterResourceEmbeddedFailureActionRecover: i32 = 2i32;
pub const CLUSTER_RESOURCE_ENUM_DEPENDS: i32 = 1i32;
pub const CLUSTER_RESOURCE_ENUM_PROVIDES: i32 = 2i32;
pub const CLUSTER_RESOURCE_ENUM_NODES: i32 = 4i32;
pub const CLUSTER_RESOURCE_ENUM_ALL: i32 = 7i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUSTER_RESOURCE_ENUM_ITEM {
    pub dwVersion: u32,
    pub cbId: u32,
    pub lpszId: super::super::Foundation::PWSTR,
    pub cbName: u32,
    pub lpszName: super::super::Foundation::PWSTR,
    pub cbOwnerGroupName: u32,
    pub lpszOwnerGroupName: super::super::Foundation::PWSTR,
    pub cbOwnerGroupId: u32,
    pub lpszOwnerGroupId: super::super::Foundation::PWSTR,
    pub cbProperties: u32,
    pub pProperties: *mut ::core::ffi::c_void,
    pub cbRoProperties: u32,
    pub pRoProperties: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUSTER_RESOURCE_ENUM_ITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUSTER_RESOURCE_ENUM_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLUSTER_RESOURCE_ENUM_ITEM_VERSION: u32 = 1u32;
pub const CLUSTER_RESOURCE_ENUM_ITEM_VERSION_1: u32 = 1u32;
pub const ClusterResourceDontRestart: i32 = 0i32;
pub const ClusterResourceRestartNoNotify: i32 = 1i32;
pub const ClusterResourceRestartNotify: i32 = 2i32;
pub const ClusterResourceRestartActionCount: i32 = 3i32;
pub const ClusterResourceStateUnknown: i32 = -1i32;
pub const ClusterResourceInherited: i32 = 0i32;
pub const ClusterResourceInitializing: i32 = 1i32;
pub const ClusterResourceOnline: i32 = 2i32;
pub const ClusterResourceOffline: i32 = 3i32;
pub const ClusterResourceFailed: i32 = 4i32;
pub const ClusterResourcePending: i32 = 128i32;
pub const ClusterResourceOnlinePending: i32 = 129i32;
pub const ClusterResourceOfflinePending: i32 = 130i32;
pub const eResourceStateChangeReasonUnknown: i32 = 0i32;
pub const eResourceStateChangeReasonMove: i32 = 1i32;
pub const eResourceStateChangeReasonFailover: i32 = 2i32;
pub const eResourceStateChangeReasonFailedMove: i32 = 3i32;
pub const eResourceStateChangeReasonShutdown: i32 = 4i32;
pub const eResourceStateChangeReasonRundown: i32 = 5i32;
pub const CLUSTER_RESOURCE_TYPE_ENUM_NODES: i32 = 1i32;
pub const CLUSTER_RESOURCE_TYPE_ENUM_RESOURCES: i32 = 2i32;
pub const CLUSTER_RESOURCE_TYPE_ENUM_ALL: i32 = 3i32;
pub const ClusterRoleDHCP: i32 = 0i32;
pub const ClusterRoleDTC: i32 = 1i32;
pub const ClusterRoleFileServer: i32 = 2i32;
pub const ClusterRoleGenericApplication: i32 = 3i32;
pub const ClusterRoleGenericScript: i32 = 4i32;
pub const ClusterRoleGenericService: i32 = 5i32;
pub const ClusterRoleISCSINameServer: i32 = 6i32;
pub const ClusterRoleMSMQ: i32 = 7i32;
pub const ClusterRoleNFS: i32 = 8i32;
pub const ClusterRolePrintServer: i32 = 9i32;
pub const ClusterRoleStandAloneNamespaceServer: i32 = 10i32;
pub const ClusterRoleVolumeShadowCopyServiceTask: i32 = 11i32;
pub const ClusterRoleWINS: i32 = 12i32;
pub const ClusterRoleTaskScheduler: i32 = 13i32;
pub const ClusterRoleNetworkFileSystem: i32 = 14i32;
pub const ClusterRoleDFSReplicatedFolder: i32 = 15i32;
pub const ClusterRoleDistributedFileSystem: i32 = 16i32;
pub const ClusterRoleDistributedNetworkName: i32 = 17i32;
pub const ClusterRoleFileShare: i32 = 18i32;
pub const ClusterRoleFileShareWitness: i32 = 19i32;
pub const ClusterRoleHardDisk: i32 = 20i32;
pub const ClusterRoleIPAddress: i32 = 21i32;
pub const ClusterRoleIPV6Address: i32 = 22i32;
pub const ClusterRoleIPV6TunnelAddress: i32 = 23i32;
pub const ClusterRoleISCSITargetServer: i32 = 24i32;
pub const ClusterRoleNetworkName: i32 = 25i32;
pub const ClusterRolePhysicalDisk: i32 = 26i32;
pub const ClusterRoleSODAFileServer: i32 = 27i32;
pub const ClusterRoleStoragePool: i32 = 28i32;
pub const ClusterRoleVirtualMachine: i32 = 29i32;
pub const ClusterRoleVirtualMachineConfiguration: i32 = 30i32;
pub const ClusterRoleVirtualMachineReplicaBroker: i32 = 31i32;
pub const ClusterRoleUnknown: i32 = -1i32;
pub const ClusterRoleClustered: i32 = 0i32;
pub const ClusterRoleUnclustered: i32 = 1i32;
pub const CLUSTER_RUNNING: u32 = 16u32;
pub const ClusterSetupPhaseInitialize: i32 = 1i32;
pub const ClusterSetupPhaseValidateNodeState: i32 = 100i32;
pub const ClusterSetupPhaseValidateNetft: i32 = 102i32;
pub const ClusterSetupPhaseValidateClusDisk: i32 = 103i32;
pub const ClusterSetupPhaseConfigureClusSvc: i32 = 104i32;
pub const ClusterSetupPhaseStartingClusSvc: i32 = 105i32;
pub const ClusterSetupPhaseQueryClusterNameAccount: i32 = 106i32;
pub const ClusterSetupPhaseValidateClusterNameAccount: i32 = 107i32;
pub const ClusterSetupPhaseCreateClusterAccount: i32 = 108i32;
pub const ClusterSetupPhaseConfigureClusterAccount: i32 = 109i32;
pub const ClusterSetupPhaseFormingCluster: i32 = 200i32;
pub const ClusterSetupPhaseAddClusterProperties: i32 = 201i32;
pub const ClusterSetupPhaseCreateResourceTypes: i32 = 202i32;
pub const ClusterSetupPhaseCreateGroups: i32 = 203i32;
pub const ClusterSetupPhaseCreateIPAddressResources: i32 = 204i32;
pub const ClusterSetupPhaseCreateNetworkName: i32 = 205i32;
pub const ClusterSetupPhaseClusterGroupOnline: i32 = 206i32;
pub const ClusterSetupPhaseGettingCurrentMembership: i32 = 300i32;
pub const ClusterSetupPhaseAddNodeToCluster: i32 = 301i32;
pub const ClusterSetupPhaseNodeUp: i32 = 302i32;
pub const ClusterSetupPhaseMoveGroup: i32 = 400i32;
pub const ClusterSetupPhaseDeleteGroup: i32 = 401i32;
pub const ClusterSetupPhaseCleanupCOs: i32 = 402i32;
pub const ClusterSetupPhaseOfflineGroup: i32 = 403i32;
pub const ClusterSetupPhaseEvictNode: i32 = 404i32;
pub const ClusterSetupPhaseCleanupNode: i32 = 405i32;
pub const ClusterSetupPhaseCoreGroupCleanup: i32 = 406i32;
pub const ClusterSetupPhaseFailureCleanup: i32 = 999i32;
pub const ClusterSetupPhaseInformational: i32 = 1i32;
pub const ClusterSetupPhaseWarning: i32 = 2i32;
pub const ClusterSetupPhaseFatal: i32 = 3i32;
pub const ClusterSetupPhaseStart: i32 = 1i32;
pub const ClusterSetupPhaseContinue: i32 = 2i32;
pub const ClusterSetupPhaseEnd: i32 = 3i32;
pub const ClusterSetupPhaseReport: i32 = 4i32;
pub const CLUSTER_SET_ACCESS_TYPE_ALLOWED: u32 = 0u32;
pub const CLUSTER_SET_ACCESS_TYPE_DENIED: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUSTER_SET_PASSWORD_STATUS {
    pub NodeId: u32,
    pub SetAttempted: super::super::Foundation::BOOLEAN,
    pub ReturnStatus: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUSTER_SET_PASSWORD_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUSTER_SET_PASSWORD_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VolumeBackupNone: i32 = 0i32;
pub const VolumeBackupInProgress: i32 = 1i32;
#[repr(C)]
pub struct CLUSTER_SHARED_VOLUME_RENAME_GUID_INPUT {
    pub __AnonymousBase_clusapi_L5475_C14: CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME,
    pub __AnonymousBase_clusapi_L5476_C14: CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME,
}
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_RENAME_GUID_INPUT {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_RENAME_GUID_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT {
    pub __AnonymousBase_clusapi_L5464_C14: CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME,
    pub __AnonymousBase_clusapi_L5465_C14: CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME,
}
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_RENAME_INPUT {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_RENAME_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME {
    pub NewVolumeName: [u16; 260],
    pub NewVolumeGuid: [u16; 50],
}
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME {
    pub NewVolumeName: [u16; 260],
}
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ClusterSharedVolumeRenameInputTypeNone: i32 = 0i32;
pub const ClusterSharedVolumeRenameInputTypeVolumeOffset: i32 = 1i32;
pub const ClusterSharedVolumeRenameInputTypeVolumeId: i32 = 2i32;
pub const ClusterSharedVolumeRenameInputTypeVolumeName: i32 = 3i32;
pub const ClusterSharedVolumeRenameInputTypeVolumeGuid: i32 = 4i32;
#[repr(C)]
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME {
    pub InputType: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE,
    pub Anonymous: CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME_0,
}
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME_0 {
    pub VolumeOffset: u64,
    pub VolumeId: [u16; 260],
    pub VolumeName: [u16; 260],
    pub VolumeGuid: [u16; 50],
}
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME_0 {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ClusterSharedVolumeSnapshotStateUnknown: i32 = 0i32;
pub const ClusterSharedVolumePrepareForHWSnapshot: i32 = 1i32;
pub const ClusterSharedVolumeHWSnapshotCompleted: i32 = 2i32;
pub const ClusterSharedVolumePrepareForFreeze: i32 = 3i32;
pub const SharedVolumeStateUnavailable: i32 = 0i32;
pub const SharedVolumeStatePaused: i32 = 1i32;
pub const SharedVolumeStateActive: i32 = 2i32;
pub const SharedVolumeStateActiveRedirected: i32 = 3i32;
pub const SharedVolumeStateActiveVolumeRedirected: i32 = 4i32;
#[repr(C)]
pub struct CLUSTER_SHARED_VOLUME_STATE_INFO {
    pub szVolumeName: [u16; 260],
    pub szNodeName: [u16; 260],
    pub VolumeState: CLUSTER_SHARED_VOLUME_STATE,
}
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_STATE_INFO {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_STATE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUSTER_SHARED_VOLUME_STATE_INFO_EX {
    pub szVolumeName: [u16; 260],
    pub szNodeName: [u16; 260],
    pub VolumeState: CLUSTER_SHARED_VOLUME_STATE,
    pub szVolumeFriendlyName: [u16; 260],
    pub RedirectedIOReason: u64,
    pub VolumeRedirectedIOReason: u64,
}
impl ::core::marker::Copy for CLUSTER_SHARED_VOLUME_STATE_INFO_EX {}
impl ::core::clone::Clone for CLUSTER_SHARED_VOLUME_STATE_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ClusterStorageNodeStateUnknown: i32 = 0i32;
pub const ClusterStorageNodeUp: i32 = 1i32;
pub const ClusterStorageNodeDown: i32 = 2i32;
pub const ClusterStorageNodePaused: i32 = 3i32;
pub const ClusterStorageNodeStarting: i32 = 4i32;
pub const ClusterStorageNodeStopping: i32 = 5i32;
pub const ClusterUpgradePhaseInitialize: i32 = 1i32;
pub const ClusterUpgradePhaseValidatingUpgrade: i32 = 2i32;
pub const ClusterUpgradePhaseUpgradingComponents: i32 = 3i32;
pub const ClusterUpgradePhaseInstallingNewComponents: i32 = 4i32;
pub const ClusterUpgradePhaseUpgradeComplete: i32 = 5i32;
#[repr(C)]
pub struct CLUSTER_VALIDATE_CSV_FILENAME {
    pub szFileName: [u16; 1],
}
impl ::core::marker::Copy for CLUSTER_VALIDATE_CSV_FILENAME {}
impl ::core::clone::Clone for CLUSTER_VALIDATE_CSV_FILENAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUSTER_VALIDATE_DIRECTORY {
    pub szPath: [u16; 1],
}
impl ::core::marker::Copy for CLUSTER_VALIDATE_DIRECTORY {}
impl ::core::clone::Clone for CLUSTER_VALIDATE_DIRECTORY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUSTER_VALIDATE_NETNAME {
    pub szNetworkName: [u16; 1],
}
impl ::core::marker::Copy for CLUSTER_VALIDATE_NETNAME {}
impl ::core::clone::Clone for CLUSTER_VALIDATE_NETNAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUSTER_VALIDATE_PATH {
    pub szPath: [u16; 1],
}
impl ::core::marker::Copy for CLUSTER_VALIDATE_PATH {}
impl ::core::clone::Clone for CLUSTER_VALIDATE_PATH {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLUSTER_VERSION_FLAG_MIXED_MODE: u32 = 1u32;
pub const CLUSTER_VERSION_UNKNOWN: u32 = 4294967295u32;
pub const CLUS_ACCESS_ANY: u32 = 0u32;
pub const CLUS_ACCESS_READ: u32 = 1u32;
pub const CLUS_ACCESS_WRITE: u32 = 2u32;
pub const CLUS_AFFINITY_RULE_NONE: i32 = 0i32;
pub const CLUS_AFFINITY_RULE_SAME_FAULT_DOMAIN: i32 = 1i32;
pub const CLUS_AFFINITY_RULE_SAME_NODE: i32 = 2i32;
pub const CLUS_AFFINITY_RULE_DIFFERENT_FAULT_DOMAIN: i32 = 3i32;
pub const CLUS_AFFINITY_RULE_DIFFERENT_NODE: i32 = 4i32;
pub const CLUS_AFFINITY_RULE_MIN: i32 = 0i32;
pub const CLUS_AFFINITY_RULE_MAX: i32 = 4i32;
pub const CLUS_CHAR_UNKNOWN: i32 = 0i32;
pub const CLUS_CHAR_QUORUM: i32 = 1i32;
pub const CLUS_CHAR_DELETE_REQUIRES_ALL_NODES: i32 = 2i32;
pub const CLUS_CHAR_LOCAL_QUORUM: i32 = 4i32;
pub const CLUS_CHAR_LOCAL_QUORUM_DEBUG: i32 = 8i32;
pub const CLUS_CHAR_REQUIRES_STATE_CHANGE_REASON: i32 = 16i32;
pub const CLUS_CHAR_BROADCAST_DELETE: i32 = 32i32;
pub const CLUS_CHAR_SINGLE_CLUSTER_INSTANCE: i32 = 64i32;
pub const CLUS_CHAR_SINGLE_GROUP_INSTANCE: i32 = 128i32;
pub const CLUS_CHAR_COEXIST_IN_SHARED_VOLUME_GROUP: i32 = 256i32;
pub const CLUS_CHAR_PLACEMENT_DATA: i32 = 512i32;
pub const CLUS_CHAR_MONITOR_DETACH: i32 = 1024i32;
pub const CLUS_CHAR_MONITOR_REATTACH: i32 = 2048i32;
pub const CLUS_CHAR_OPERATION_CONTEXT: i32 = 4096i32;
pub const CLUS_CHAR_CLONES: i32 = 8192i32;
pub const CLUS_CHAR_NOT_PREEMPTABLE: i32 = 16384i32;
pub const CLUS_CHAR_NOTIFY_NEW_OWNER: i32 = 32768i32;
pub const CLUS_CHAR_SUPPORTS_UNMONITORED_STATE: i32 = 65536i32;
pub const CLUS_CHAR_INFRASTRUCTURE: i32 = 131072i32;
pub const CLUS_CHAR_VETO_DRAIN: i32 = 262144i32;
pub const CLUS_CHAR_DRAIN_LOCAL_OFFLINE: i32 = 524288i32;
#[repr(C)]
pub struct CLUS_CHKDSK_INFO {
    pub PartitionNumber: u32,
    pub ChkdskState: u32,
    pub FileIdCount: u32,
    pub FileIdList: [u64; 1],
}
impl ::core::marker::Copy for CLUS_CHKDSK_INFO {}
impl ::core::clone::Clone for CLUS_CHKDSK_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLUS_CREATE_CRYPT_CONTAINER_NOT_FOUND: u32 = 1u32;
#[repr(C)]
pub struct CLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT {
    pub FileServerName: [u16; 16],
}
impl ::core::marker::Copy for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT {}
impl ::core::clone::Clone for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT {
    pub FileServerName: [u16; 260],
}
impl ::core::marker::Copy for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT {}
impl ::core::clone::Clone for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUS_CSV_MAINTENANCE_MODE_INFO {
    pub InMaintenance: super::super::Foundation::BOOL,
    pub VolumeName: [u16; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUS_CSV_MAINTENANCE_MODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUS_CSV_MAINTENANCE_MODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUS_CSV_VOLUME_INFO {
    pub VolumeOffset: u64,
    pub PartitionNumber: u32,
    pub FaultState: CLUSTER_CSV_VOLUME_FAULT_STATE,
    pub BackupState: CLUSTER_SHARED_VOLUME_BACKUP_STATE,
    pub szVolumeFriendlyName: [u16; 260],
    pub szVolumeName: [u16; 50],
}
impl ::core::marker::Copy for CLUS_CSV_VOLUME_INFO {}
impl ::core::clone::Clone for CLUS_CSV_VOLUME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUS_CSV_VOLUME_NAME {
    pub VolumeOffset: i64,
    pub szVolumeName: [u16; 260],
    pub szRootPath: [u16; 263],
}
impl ::core::marker::Copy for CLUS_CSV_VOLUME_NAME {}
impl ::core::clone::Clone for CLUS_CSV_VOLUME_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUS_DISK_NUMBER_INFO {
    pub DiskNumber: u32,
    pub BytesPerSector: u32,
}
impl ::core::marker::Copy for CLUS_DISK_NUMBER_INFO {}
impl ::core::clone::Clone for CLUS_DISK_NUMBER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUS_DNN_LEADER_STATUS {
    pub IsOnline: super::super::Foundation::BOOL,
    pub IsFileServerPresent: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUS_DNN_LEADER_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUS_DNN_LEADER_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUS_DNN_SODAFS_CLONE_STATUS {
    pub NodeId: u32,
    pub Status: CLUSTER_RESOURCE_STATE,
}
impl ::core::marker::Copy for CLUS_DNN_SODAFS_CLONE_STATUS {}
impl ::core::clone::Clone for CLUS_DNN_SODAFS_CLONE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLUS_FLAG_CORE: i32 = 1i32;
#[repr(C)]
pub struct CLUS_FORCE_QUORUM_INFO {
    pub dwSize: u32,
    pub dwNodeBitMask: u32,
    pub dwMaxNumberofNodes: u32,
    pub multiszNodeList: [u16; 1],
}
impl ::core::marker::Copy for CLUS_FORCE_QUORUM_INFO {}
impl ::core::clone::Clone for CLUS_FORCE_QUORUM_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUS_FTSET_INFO {
    pub dwRootSignature: u32,
    pub dwFtType: u32,
}
impl ::core::marker::Copy for CLUS_FTSET_INFO {}
impl ::core::clone::Clone for CLUS_FTSET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLUS_GLOBAL: u32 = 1u32;
pub const CLUS_GROUP_START_ALWAYS: i32 = 0i32;
pub const CLUS_GROUP_DO_NOT_START: i32 = 1i32;
pub const CLUS_GROUP_START_ALLOWED: i32 = 2i32;
pub const CLUS_GRP_MOVE_ALLOWED: u32 = 0u32;
pub const CLUS_GRP_MOVE_LOCKED: u32 = 1u32;
pub const CLUS_HYBRID_QUORUM: u32 = 1024u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUS_MAINTENANCE_MODE_INFO {
    pub InMaintenance: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUS_MAINTENANCE_MODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUS_MAINTENANCE_MODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUS_MAINTENANCE_MODE_INFOEX {
    pub InMaintenance: super::super::Foundation::BOOL,
    pub MaintainenceModeType: MAINTENANCE_MODE_TYPE_ENUM,
    pub InternalState: CLUSTER_RESOURCE_STATE,
    pub Signature: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUS_MAINTENANCE_MODE_INFOEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUS_MAINTENANCE_MODE_INFOEX {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLUS_MODIFY: u32 = 1u32;
#[repr(C)]
pub struct CLUS_NETNAME_IP_INFO_ENTRY {
    pub NodeId: u32,
    pub AddressSize: u32,
    pub Address: [u8; 1],
}
impl ::core::marker::Copy for CLUS_NETNAME_IP_INFO_ENTRY {}
impl ::core::clone::Clone for CLUS_NETNAME_IP_INFO_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL {
    pub szName: [u16; 64],
    pub NumEntries: u32,
    pub IpInfo: [CLUS_NETNAME_IP_INFO_ENTRY; 1],
}
impl ::core::marker::Copy for CLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL {}
impl ::core::clone::Clone for CLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUS_NETNAME_PWD_INFO {
    pub Flags: u32,
    pub Password: [u16; 16],
    pub CreatingDC: [u16; 258],
    pub ObjectGuid: [u16; 64],
}
impl ::core::marker::Copy for CLUS_NETNAME_PWD_INFO {}
impl ::core::clone::Clone for CLUS_NETNAME_PWD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUS_NETNAME_PWD_INFOEX {
    pub Flags: u32,
    pub Password: [u16; 128],
    pub CreatingDC: [u16; 258],
    pub ObjectGuid: [u16; 64],
}
impl ::core::marker::Copy for CLUS_NETNAME_PWD_INFOEX {}
impl ::core::clone::Clone for CLUS_NETNAME_PWD_INFOEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUS_NETNAME_VS_TOKEN_INFO {
    pub ProcessID: u32,
    pub DesiredAccess: u32,
    pub InheritHandle: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUS_NETNAME_VS_TOKEN_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUS_NETNAME_VS_TOKEN_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLUS_NODE_MAJORITY_QUORUM: u32 = 0u32;
pub const CLUS_NOT_GLOBAL: u32 = 0u32;
pub const CLUS_NO_MODIFY: u32 = 0u32;
#[repr(C)]
pub struct CLUS_PARTITION_INFO {
    pub dwFlags: u32,
    pub szDeviceName: [u16; 260],
    pub szVolumeLabel: [u16; 260],
    pub dwSerialNumber: u32,
    pub rgdwMaximumComponentLength: u32,
    pub dwFileSystemFlags: u32,
    pub szFileSystem: [u16; 32],
}
impl ::core::marker::Copy for CLUS_PARTITION_INFO {}
impl ::core::clone::Clone for CLUS_PARTITION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUS_PARTITION_INFO_EX {
    pub dwFlags: u32,
    pub szDeviceName: [u16; 260],
    pub szVolumeLabel: [u16; 260],
    pub dwSerialNumber: u32,
    pub rgdwMaximumComponentLength: u32,
    pub dwFileSystemFlags: u32,
    pub szFileSystem: [u16; 32],
    pub TotalSizeInBytes: u64,
    pub FreeSizeInBytes: u64,
    pub DeviceNumber: u32,
    pub PartitionNumber: u32,
    pub VolumeGuid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for CLUS_PARTITION_INFO_EX {}
impl ::core::clone::Clone for CLUS_PARTITION_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUS_PARTITION_INFO_EX2 {
    pub GptPartitionId: ::windows_sys::core::GUID,
    pub szPartitionName: [u16; 260],
    pub EncryptionFlags: u32,
}
impl ::core::marker::Copy for CLUS_PARTITION_INFO_EX2 {}
impl ::core::clone::Clone for CLUS_PARTITION_INFO_EX2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUS_PROVIDER_STATE_CHANGE_INFO {
    pub dwSize: u32,
    pub resourceState: CLUSTER_RESOURCE_STATE,
    pub szProviderId: [u16; 1],
}
impl ::core::marker::Copy for CLUS_PROVIDER_STATE_CHANGE_INFO {}
impl ::core::clone::Clone for CLUS_PROVIDER_STATE_CHANGE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLUS_RESDLL_OFFLINE_DO_NOT_UPDATE_PERSISTENT_STATE: u32 = 64u32;
pub const CLUS_RESDLL_OFFLINE_DUE_TO_EMBEDDED_FAILURE: u32 = 16u32;
pub const CLUS_RESDLL_OFFLINE_IGNORE_NETWORK_CONNECTIVITY: u32 = 32u32;
pub const CLUS_RESDLL_OFFLINE_IGNORE_RESOURCE_STATUS: u32 = 1u32;
pub const CLUS_RESDLL_OFFLINE_QUEUE_ENABLED: u32 = 4u32;
pub const CLUS_RESDLL_OFFLINE_RETURNING_TO_SOURCE_NODE_BECAUSE_OF_ERROR: u32 = 8u32;
pub const CLUS_RESDLL_OFFLINE_RETURN_TO_SOURCE_NODE_ON_ERROR: u32 = 2u32;
pub const CLUS_RESDLL_ONLINE_IGNORE_NETWORK_CONNECTIVITY: u32 = 16u32;
pub const CLUS_RESDLL_ONLINE_IGNORE_RESOURCE_STATUS: u32 = 2u32;
pub const CLUS_RESDLL_ONLINE_RECOVER_MONITOR_STATE: u32 = 1u32;
pub const CLUS_RESDLL_ONLINE_RESTORE_ONLINE_STATE: u32 = 8u32;
pub const CLUS_RESDLL_ONLINE_RETURN_TO_SOURCE_NODE_ON_ERROR: u32 = 4u32;
pub const CLUS_RESDLL_OPEN_DONT_DELETE_TEMP_DISK: u32 = 2u32;
pub const CLUS_RESDLL_OPEN_RECOVER_MONITOR_STATE: u32 = 1u32;
#[repr(C)]
pub struct CLUS_RESOURCE_CLASS_INFO {
    pub Anonymous: CLUS_RESOURCE_CLASS_INFO_0,
}
impl ::core::marker::Copy for CLUS_RESOURCE_CLASS_INFO {}
impl ::core::clone::Clone for CLUS_RESOURCE_CLASS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union CLUS_RESOURCE_CLASS_INFO_0 {
    pub Anonymous: CLUS_RESOURCE_CLASS_INFO_0_0,
    pub li: u64,
}
impl ::core::marker::Copy for CLUS_RESOURCE_CLASS_INFO_0 {}
impl ::core::clone::Clone for CLUS_RESOURCE_CLASS_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUS_RESOURCE_CLASS_INFO_0_0 {
    pub Anonymous: CLUS_RESOURCE_CLASS_INFO_0_0_0,
    pub SubClass: u32,
}
impl ::core::marker::Copy for CLUS_RESOURCE_CLASS_INFO_0_0 {}
impl ::core::clone::Clone for CLUS_RESOURCE_CLASS_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union CLUS_RESOURCE_CLASS_INFO_0_0_0 {
    pub dw: u32,
    pub rc: CLUSTER_RESOURCE_CLASS,
}
impl ::core::marker::Copy for CLUS_RESOURCE_CLASS_INFO_0_0_0 {}
impl ::core::clone::Clone for CLUS_RESOURCE_CLASS_INFO_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLUS_RESSUBCLASS_SHARED: i32 = -2147483648i32;
pub const CLUS_RESSUBCLASS_NETWORK_INTERNET_PROTOCOL: i32 = -2147483648i32;
pub const CLUS_RESSUBCLASS_STORAGE_SHARED_BUS: i32 = -2147483648i32;
pub const CLUS_RESSUBCLASS_STORAGE_DISK: i32 = 1073741824i32;
pub const CLUS_RESSUBCLASS_STORAGE_REPLICATION: i32 = 268435456i32;
#[repr(C)]
pub struct CLUS_SCSI_ADDRESS {
    pub Anonymous: CLUS_SCSI_ADDRESS_0,
}
impl ::core::marker::Copy for CLUS_SCSI_ADDRESS {}
impl ::core::clone::Clone for CLUS_SCSI_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union CLUS_SCSI_ADDRESS_0 {
    pub Anonymous: CLUS_SCSI_ADDRESS_0_0,
    pub dw: u32,
}
impl ::core::marker::Copy for CLUS_SCSI_ADDRESS_0 {}
impl ::core::clone::Clone for CLUS_SCSI_ADDRESS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUS_SCSI_ADDRESS_0_0 {
    pub PortNumber: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
}
impl ::core::marker::Copy for CLUS_SCSI_ADDRESS_0_0 {}
impl ::core::clone::Clone for CLUS_SCSI_ADDRESS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUS_SET_MAINTENANCE_MODE_INPUT {
    pub InMaintenance: super::super::Foundation::BOOL,
    pub ExtraParameterSize: u32,
    pub ExtraParameter: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUS_SET_MAINTENANCE_MODE_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUS_SET_MAINTENANCE_MODE_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUS_SHARED_VOLUME_BACKUP_MODE {
    pub BackupState: CLUSTER_SHARED_VOLUME_BACKUP_STATE,
    pub DelayTimerInSecs: u32,
    pub VolumeName: [u16; 260],
}
impl ::core::marker::Copy for CLUS_SHARED_VOLUME_BACKUP_MODE {}
impl ::core::clone::Clone for CLUS_SHARED_VOLUME_BACKUP_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUS_STARTING_PARAMS {
    pub dwSize: u32,
    pub bForm: super::super::Foundation::BOOL,
    pub bFirst: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUS_STARTING_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUS_STARTING_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS {
    pub AvailDrivelettersMask: u32,
}
impl ::core::marker::Copy for CLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS {}
impl ::core::clone::Clone for CLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUS_STORAGE_REMAP_DRIVELETTER {
    pub CurrentDriveLetterMask: u32,
    pub TargetDriveLetterMask: u32,
}
impl ::core::marker::Copy for CLUS_STORAGE_REMAP_DRIVELETTER {}
impl ::core::clone::Clone for CLUS_STORAGE_REMAP_DRIVELETTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct CLUS_STORAGE_SET_DRIVELETTER {
    pub PartitionNumber: u32,
    pub DriveLetterMask: u32,
}
impl ::core::marker::Copy for CLUS_STORAGE_SET_DRIVELETTER {}
impl ::core::clone::Clone for CLUS_STORAGE_SET_DRIVELETTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLUS_WORKER {
    pub hThread: super::super::Foundation::HANDLE,
    pub Terminate: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CLUS_WORKER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CLUS_WORKER {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CREATEDC_PRESENT: u32 = 2u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CREATE_CLUSTER_CONFIG {
    pub dwVersion: u32,
    pub lpszClusterName: super::super::Foundation::PWSTR,
    pub cNodes: u32,
    pub ppszNodeNames: *mut super::super::Foundation::PWSTR,
    pub cIpEntries: u32,
    pub pIpEntries: *mut CLUSTER_IP_ENTRY,
    pub fEmptyCluster: super::super::Foundation::BOOLEAN,
    pub managementPointType: CLUSTER_MGMT_POINT_TYPE,
    pub managementPointResType: CLUSTER_MGMT_POINT_RESTYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREATE_CLUSTER_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREATE_CLUSTER_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CREATE_CLUSTER_MAJOR_VERSION_MASK: u32 = 4294967040u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CREATE_CLUSTER_NAME_ACCOUNT {
    pub dwVersion: u32,
    pub lpszClusterName: super::super::Foundation::PWSTR,
    pub dwFlags: u32,
    pub pszUserName: super::super::Foundation::PWSTR,
    pub pszPassword: super::super::Foundation::PWSTR,
    pub pszDomain: super::super::Foundation::PWSTR,
    pub managementPointType: CLUSTER_MGMT_POINT_TYPE,
    pub managementPointResType: CLUSTER_MGMT_POINT_RESTYPE,
    pub bUpgradeVCOs: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CREATE_CLUSTER_NAME_ACCOUNT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CREATE_CLUSTER_NAME_ACCOUNT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CREATE_CLUSTER_VERSION: u32 = 1536u32;
pub const ClusApplication: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161317, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusCryptoKeys: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161387, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusDisk: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161379, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusDisks: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161381, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusNetInterface: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161325, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusNetInterfaces: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161327, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusNetwork: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161329, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusNetworkNetInterfaces: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161333, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusNetworks: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161331, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusNode: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161335, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusNodeNetInterfaces: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161339, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusNodes: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161337, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusPartition: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161375, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusPartitionEx: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1406475558,
    data2: 46363,
    data3: 19065,
    data4: [178, 195, 80, 72, 217, 58, 152, 252],
};
pub const ClusPartitions: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161377, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusProperties: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161343, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusProperty: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161341, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusPropertyValue: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161369, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusPropertyValueData: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161373, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusPropertyValues: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161371, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusRefObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161345, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusRegistryKeys: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161385, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusResDependencies: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161347, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusResDependents: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161389, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusResGroup: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161349, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusResGroupPreferredOwnerNodes: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161319, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusResGroupResources: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161321, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusResGroups: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161351, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusResPossibleOwnerNodes: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161357, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusResType: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161359, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusResTypePossibleOwnerNodes: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161367, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusResTypeResources: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161363, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusResTypes: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161361, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusResource: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161353, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusResources: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161355, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusScsiAddress: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161383, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusVersion: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161365, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const Cluster: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161315, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const ClusterNames: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161323, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const DNS_LENGTH: u32 = 64u32;
pub const DomainNames: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4075161313, data2: 9777, data3: 4561, data4: [137, 241, 0, 160, 201, 13, 6, 30] };
pub const FAILURE_TYPE_GENERAL: i32 = 0i32;
pub const FAILURE_TYPE_EMBEDDED: i32 = 1i32;
pub const FAILURE_TYPE_NETWORK_LOSS: i32 = 2i32;
pub const FE_UPGRADE_VERSION: u32 = 4u32;
#[repr(C)]
pub struct FILESHARE_CHANGE {
    pub Change: FILESHARE_CHANGE_ENUM,
    pub ShareName: [u16; 84],
}
impl ::core::marker::Copy for FILESHARE_CHANGE {}
impl ::core::clone::Clone for FILESHARE_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FILESHARE_CHANGE_NONE: i32 = 0i32;
pub const FILESHARE_CHANGE_ADD: i32 = 1i32;
pub const FILESHARE_CHANGE_DEL: i32 = 2i32;
pub const FILESHARE_CHANGE_MODIFY: i32 = 3i32;
#[repr(C)]
pub struct FILESHARE_CHANGE_LIST {
    pub NumEntries: u32,
    pub ChangeEntry: [FILESHARE_CHANGE; 1],
}
impl ::core::marker::Copy for FILESHARE_CHANGE_LIST {}
impl ::core::clone::Clone for FILESHARE_CHANGE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct GET_OPERATION_CONTEXT_PARAMS {
    pub Size: u32,
    pub Version: u32,
    pub Type: RESDLL_CONTEXT_OPERATION_TYPE,
    pub Priority: u32,
}
impl ::core::marker::Copy for GET_OPERATION_CONTEXT_PARAMS {}
impl ::core::clone::Clone for GET_OPERATION_CONTEXT_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const GROUPSET_READY_SETTING_APPLICATION_READY: u32 = 4u32;
pub const GROUPSET_READY_SETTING_DELAY: u32 = 1u32;
pub const GROUPSET_READY_SETTING_ONLINE: u32 = 2u32;
pub const GROUPSET_READY_SETTING_OS_HEARTBEAT: u32 = 3u32;
#[repr(C)]
pub struct GROUP_FAILURE_INFO {
    pub dwFailoverAttemptsRemaining: u32,
    pub dwFailoverPeriodRemaining: u32,
}
impl ::core::marker::Copy for GROUP_FAILURE_INFO {}
impl ::core::clone::Clone for GROUP_FAILURE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct GROUP_FAILURE_INFO_BUFFER {
    pub dwVersion: u32,
    pub Info: GROUP_FAILURE_INFO,
}
impl ::core::marker::Copy for GROUP_FAILURE_INFO_BUFFER {}
impl ::core::clone::Clone for GROUP_FAILURE_INFO_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
pub const GROUP_FAILURE_INFO_VERSION_1: u32 = 1u32;
pub const GRP_PLACEMENT_OPTIONS_MIN_VALUE: i32 = 0i32;
pub const GRP_PLACEMENT_OPTIONS_DEFAULT: i32 = 0i32;
pub const GRP_PLACEMENT_OPTIONS_DISABLE_AUTOBALANCING: i32 = 1i32;
pub const GRP_PLACEMENT_OPTIONS_ALL: i32 = 1i32;
pub const GUID_PRESENT: u32 = 1u32;
pub const HCI_UPGRADE_BIT: u32 = 32768u32;
#[repr(transparent)]
pub struct IGetClusterDataInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGetClusterDataInfo {}
impl ::core::clone::Clone for IGetClusterDataInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGetClusterGroupInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGetClusterGroupInfo {}
impl ::core::clone::Clone for IGetClusterGroupInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGetClusterNetInterfaceInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGetClusterNetInterfaceInfo {}
impl ::core::clone::Clone for IGetClusterNetInterfaceInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGetClusterNetworkInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGetClusterNetworkInfo {}
impl ::core::clone::Clone for IGetClusterNetworkInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGetClusterNodeInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGetClusterNodeInfo {}
impl ::core::clone::Clone for IGetClusterNodeInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGetClusterObjectInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGetClusterObjectInfo {}
impl ::core::clone::Clone for IGetClusterObjectInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGetClusterResourceInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGetClusterResourceInfo {}
impl ::core::clone::Clone for IGetClusterResourceInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGetClusterUIInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGetClusterUIInfo {}
impl ::core::clone::Clone for IGetClusterUIInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusApplication(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusApplication {}
impl ::core::clone::Clone for ISClusApplication {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusCryptoKeys(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusCryptoKeys {}
impl ::core::clone::Clone for ISClusCryptoKeys {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusDisk(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusDisk {}
impl ::core::clone::Clone for ISClusDisk {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusDisks(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusDisks {}
impl ::core::clone::Clone for ISClusDisks {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusNetInterface(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusNetInterface {}
impl ::core::clone::Clone for ISClusNetInterface {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusNetInterfaces(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusNetInterfaces {}
impl ::core::clone::Clone for ISClusNetInterfaces {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusNetwork(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusNetwork {}
impl ::core::clone::Clone for ISClusNetwork {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusNetworkNetInterfaces(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusNetworkNetInterfaces {}
impl ::core::clone::Clone for ISClusNetworkNetInterfaces {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusNetworks(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusNetworks {}
impl ::core::clone::Clone for ISClusNetworks {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusNode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusNode {}
impl ::core::clone::Clone for ISClusNode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusNodeNetInterfaces(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusNodeNetInterfaces {}
impl ::core::clone::Clone for ISClusNodeNetInterfaces {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusNodes(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusNodes {}
impl ::core::clone::Clone for ISClusNodes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusPartition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusPartition {}
impl ::core::clone::Clone for ISClusPartition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusPartitionEx(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusPartitionEx {}
impl ::core::clone::Clone for ISClusPartitionEx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusPartitions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusPartitions {}
impl ::core::clone::Clone for ISClusPartitions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusProperties {}
impl ::core::clone::Clone for ISClusProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusProperty(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusProperty {}
impl ::core::clone::Clone for ISClusProperty {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusPropertyValue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusPropertyValue {}
impl ::core::clone::Clone for ISClusPropertyValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusPropertyValueData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusPropertyValueData {}
impl ::core::clone::Clone for ISClusPropertyValueData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusPropertyValues(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusPropertyValues {}
impl ::core::clone::Clone for ISClusPropertyValues {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusRefObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusRefObject {}
impl ::core::clone::Clone for ISClusRefObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusRegistryKeys(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusRegistryKeys {}
impl ::core::clone::Clone for ISClusRegistryKeys {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusResDependencies(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusResDependencies {}
impl ::core::clone::Clone for ISClusResDependencies {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusResDependents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusResDependents {}
impl ::core::clone::Clone for ISClusResDependents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusResGroup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusResGroup {}
impl ::core::clone::Clone for ISClusResGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusResGroupPreferredOwnerNodes(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusResGroupPreferredOwnerNodes {}
impl ::core::clone::Clone for ISClusResGroupPreferredOwnerNodes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusResGroupResources(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusResGroupResources {}
impl ::core::clone::Clone for ISClusResGroupResources {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusResGroups(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusResGroups {}
impl ::core::clone::Clone for ISClusResGroups {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusResPossibleOwnerNodes(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusResPossibleOwnerNodes {}
impl ::core::clone::Clone for ISClusResPossibleOwnerNodes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusResType(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusResType {}
impl ::core::clone::Clone for ISClusResType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusResTypePossibleOwnerNodes(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusResTypePossibleOwnerNodes {}
impl ::core::clone::Clone for ISClusResTypePossibleOwnerNodes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusResTypeResources(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusResTypeResources {}
impl ::core::clone::Clone for ISClusResTypeResources {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusResTypes(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusResTypes {}
impl ::core::clone::Clone for ISClusResTypes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusResource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusResource {}
impl ::core::clone::Clone for ISClusResource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusResources(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusResources {}
impl ::core::clone::Clone for ISClusResources {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusScsiAddress(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusScsiAddress {}
impl ::core::clone::Clone for ISClusScsiAddress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusVersion(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusVersion {}
impl ::core::clone::Clone for ISClusVersion {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISCluster(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISCluster {}
impl ::core::clone::Clone for ISCluster {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISClusterNames(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISClusterNames {}
impl ::core::clone::Clone for ISClusterNames {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISDomainNames(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISDomainNames {}
impl ::core::clone::Clone for ISDomainNames {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWCContextMenuCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWCContextMenuCallback {}
impl ::core::clone::Clone for IWCContextMenuCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWCPropertySheetCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWCPropertySheetCallback {}
impl ::core::clone::Clone for IWCPropertySheetCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWCWizard97Callback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWCWizard97Callback {}
impl ::core::clone::Clone for IWCWizard97Callback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWCWizardCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWCWizardCallback {}
impl ::core::clone::Clone for IWCWizardCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWEExtendContextMenu(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWEExtendContextMenu {}
impl ::core::clone::Clone for IWEExtendContextMenu {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWEExtendPropertySheet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWEExtendPropertySheet {}
impl ::core::clone::Clone for IWEExtendPropertySheet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWEExtendWizard(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWEExtendWizard {}
impl ::core::clone::Clone for IWEExtendWizard {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWEExtendWizard97(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWEExtendWizard97 {}
impl ::core::clone::Clone for IWEExtendWizard97 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWEInvokeCommand(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWEInvokeCommand {}
impl ::core::clone::Clone for IWEInvokeCommand {
    fn clone(&self) -> Self {
        *self
    }
}
pub const LOCKED_MODE_FLAGS_DONT_REMOVE_FROM_MOVE_QUEUE: u32 = 1u32;
pub const LOG_INFORMATION: i32 = 0i32;
pub const LOG_WARNING: i32 = 1i32;
pub const LOG_ERROR: i32 = 2i32;
pub const LOG_SEVERE: i32 = 3i32;
pub type LPGROUP_CALLBACK_EX = unsafe extern "system" fn(param0: *mut _HCLUSTER, param1: *mut _HGROUP, param2: *mut _HGROUP, param3: *mut ::core::ffi::c_void) -> u32;
pub type LPNODE_CALLBACK = unsafe extern "system" fn(param0: *mut _HCLUSTER, param1: *mut _HNODE, param2: CLUSTER_NODE_STATE, param3: *mut ::core::ffi::c_void) -> u32;
pub type LPRESOURCE_CALLBACK = unsafe extern "system" fn(param0: *mut _HRESOURCE, param1: *mut _HRESOURCE, param2: *mut ::core::ffi::c_void) -> u32;
pub type LPRESOURCE_CALLBACK_EX = unsafe extern "system" fn(param0: *mut _HCLUSTER, param1: *mut _HRESOURCE, param2: *mut _HRESOURCE, param3: *mut ::core::ffi::c_void) -> u32;
pub const MaintenanceModeTypeDisableIsAliveCheck: i32 = 1i32;
pub const MaintenanceModeTypeOfflineResource: i32 = 2i32;
pub const MaintenanceModeTypeUnclusterResource: i32 = 3i32;
pub const MAINTENANCE_MODE_V2_SIG: u32 = 2881155087u32;
pub const MAX_CLUSTERNAME_LENGTH: u32 = 63u32;
pub const MAX_CO_PASSWORD_LENGTH: u32 = 16u32;
pub const MAX_CO_PASSWORD_LENGTHEX: u32 = 127u32;
pub const MAX_CO_PASSWORD_STORAGEEX: u32 = 128u32;
pub const MAX_CREATINGDC_LENGTH: u32 = 256u32;
pub const MAX_OBJECTID: u32 = 64u32;
pub const MN_UPGRADE_VERSION: u32 = 3u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MONITOR_STATE {
    pub LastUpdate: i64,
    pub State: RESOURCE_MONITOR_STATE,
    pub ActiveResource: super::super::Foundation::HANDLE,
    pub ResmonStop: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MONITOR_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MONITOR_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const NINETEEN_H1_UPGRADE_VERSION: u32 = 1u32;
pub const NINETEEN_H2_UPGRADE_VERSION: u32 = 2u32;
pub const NNLEN: u32 = 80u32;
pub const ClusterStateNotInstalled: i32 = 0i32;
pub const ClusterStateNotConfigured: i32 = 1i32;
pub const ClusterStateNotRunning: i32 = 3i32;
pub const ClusterStateRunning: i32 = 19i32;
#[repr(C)]
pub struct NOTIFY_FILTER_AND_TYPE {
    pub dwObjectType: u32,
    pub FilterFlags: i64,
}
impl ::core::marker::Copy for NOTIFY_FILTER_AND_TYPE {}
impl ::core::clone::Clone for NOTIFY_FILTER_AND_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const NT10_MAJOR_VERSION: u32 = 9u32;
pub const NT11_MAJOR_VERSION: u32 = 10u32;
pub const NT12_MAJOR_VERSION: u32 = 11u32;
pub const NT13_MAJOR_VERSION: u32 = 12u32;
pub const NT4SP4_MAJOR_VERSION: u32 = 2u32;
pub const NT4_MAJOR_VERSION: u32 = 1u32;
pub const NT51_MAJOR_VERSION: u32 = 4u32;
pub const NT5_MAJOR_VERSION: u32 = 3u32;
pub const NT6_MAJOR_VERSION: u32 = 5u32;
pub const NT7_MAJOR_VERSION: u32 = 6u32;
pub const NT8_MAJOR_VERSION: u32 = 7u32;
pub const NT9_MAJOR_VERSION: u32 = 8u32;
#[repr(C)]
pub struct NodeUtilizationInfoElement {
    pub Id: u64,
    pub AvailableMemory: u64,
    pub AvailableMemoryAfterReclamation: u64,
}
impl ::core::marker::Copy for NodeUtilizationInfoElement {}
impl ::core::clone::Clone for NodeUtilizationInfoElement {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PARBITRATE_ROUTINE = unsafe extern "system" fn(resource: *mut ::core::ffi::c_void, lostquorumresource: PQUORUM_RESOURCE_LOST) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PBEGIN_RESCALL_AS_USER_ROUTINE = unsafe extern "system" fn(resource: *mut ::core::ffi::c_void, tokenhandle: super::super::Foundation::HANDLE, controlcode: u32, inbuffer: *mut ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32, context: i64, returnedasynchronously: *mut super::super::Foundation::BOOL) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PBEGIN_RESCALL_ROUTINE = unsafe extern "system" fn(resource: *mut ::core::ffi::c_void, controlcode: u32, inbuffer: *mut ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32, context: i64, returnedasynchronously: *mut super::super::Foundation::BOOL) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PBEGIN_RESTYPECALL_AS_USER_ROUTINE = unsafe extern "system" fn(resourcetypename: super::super::Foundation::PWSTR, tokenhandle: super::super::Foundation::HANDLE, controlcode: u32, inbuffer: *mut ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32, context: i64, returnedasynchronously: *mut super::super::Foundation::BOOL) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PBEGIN_RESTYPECALL_ROUTINE = unsafe extern "system" fn(resourcetypename: super::super::Foundation::PWSTR, controlcode: u32, inbuffer: *mut ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32, context: i64, returnedasynchronously: *mut super::super::Foundation::BOOL) -> u32;
pub type PCANCEL_ROUTINE = unsafe extern "system" fn(resource: *mut ::core::ffi::c_void, cancelflags_reserved: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCHANGE_RESOURCE_PROCESS_FOR_DUMPS = unsafe extern "system" fn(resource: isize, processname: super::super::Foundation::PWSTR, processid: u32, isadd: super::super::Foundation::BOOL) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCHANGE_RES_TYPE_PROCESS_FOR_DUMPS = unsafe extern "system" fn(resourcetypename: super::super::Foundation::PWSTR, processname: super::super::Foundation::PWSTR, processid: u32, isadd: super::super::Foundation::BOOL) -> u32;
pub type PCLOSE_CLUSTER_CRYPT_PROVIDER = unsafe extern "system" fn(hcluscryptprovider: *const _HCLUSCRYPTPROVIDER) -> u32;
pub type PCLOSE_ROUTINE = unsafe extern "system" fn(resource: *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPIClusWorkerCheckTerminate = unsafe extern "system" fn(lpworker: *mut CLUS_WORKER) -> super::super::Foundation::BOOL;
pub type PCLUSAPI_ADD_CLUSTER_GROUP_DEPENDENCY = unsafe extern "system" fn(hdependentgroup: *const _HGROUP, hprovidergroup: *const _HGROUP) -> u32;
pub type PCLUSAPI_ADD_CLUSTER_GROUP_GROUPSET_DEPENDENCY = unsafe extern "system" fn(hdependentgroupset: *const _HGROUPSET, hprovidergroupset: *const _HGROUPSET) -> u32;
pub type PCLUSAPI_ADD_CLUSTER_GROUP_TO_GROUP_GROUPSET_DEPENDENCY = unsafe extern "system" fn(hdependentgroup: *const _HGROUP, hprovidergroupset: *const _HGROUPSET) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_ADD_CLUSTER_NODE = unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznodename: super::super::Foundation::PWSTR, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> *mut _HNODE;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_ADD_CLUSTER_NODE_EX = unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznodename: super::super::Foundation::PWSTR, dwflags: u32, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> *mut _HNODE;
pub type PCLUSAPI_ADD_CLUSTER_RESOURCE_DEPENDENCY = unsafe extern "system" fn(hresource: *mut _HRESOURCE, hdependson: *mut _HRESOURCE) -> u32;
pub type PCLUSAPI_ADD_CLUSTER_RESOURCE_NODE = unsafe extern "system" fn(hresource: *mut _HRESOURCE, hnode: *mut _HNODE) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_ADD_CROSS_CLUSTER_GROUPSET_DEPENDENCY = unsafe extern "system" fn(hdependentgroupset: *const _HGROUPSET, lpremoteclustername: super::super::Foundation::PWSTR, lpremotegroupsetname: super::super::Foundation::PWSTR) -> u32;
pub type PCLUSAPI_ADD_RESOURCE_TO_CLUSTER_SHARED_VOLUMES = unsafe extern "system" fn(hresource: *const _HRESOURCE) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_BACKUP_CLUSTER_DATABASE = unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszpathname: super::super::Foundation::PWSTR) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CAN_RESOURCE_BE_DEPENDENT = unsafe extern "system" fn(hresource: *mut _HRESOURCE, hresourcedependent: *mut _HRESOURCE) -> super::super::Foundation::BOOL;
pub type PCLUSAPI_CHANGE_CLUSTER_RESOURCE_GROUP = unsafe extern "system" fn(hresource: *mut _HRESOURCE, hgroup: *mut _HGROUP) -> u32;
pub type PCLUSAPI_CHANGE_CLUSTER_RESOURCE_GROUP_EX = unsafe extern "system" fn(hresource: *mut _HRESOURCE, hgroup: *mut _HGROUP, flags: u64) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLOSE_CLUSTER = unsafe extern "system" fn(hcluster: *const _HCLUSTER) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLOSE_CLUSTER_GROUP = unsafe extern "system" fn(hgroup: *const _HGROUP) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLOSE_CLUSTER_GROUP_GROUPSET = unsafe extern "system" fn(hgroupset: *const _HGROUPSET) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLOSE_CLUSTER_NETWORK = unsafe extern "system" fn(hnetwork: *const _HNETWORK) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLOSE_CLUSTER_NET_INTERFACE = unsafe extern "system" fn(hnetinterface: *const _HNETINTERFACE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLOSE_CLUSTER_NODE = unsafe extern "system" fn(hnode: *const _HNODE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLOSE_CLUSTER_NOTIFY_PORT = unsafe extern "system" fn(hchange: *const _HCHANGE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLOSE_CLUSTER_RESOURCE = unsafe extern "system" fn(hresource: *mut _HRESOURCE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUSTER_ADD_GROUP_TO_AFFINITY_RULE = unsafe extern "system" fn(hcluster: *const _HCLUSTER, rulename: super::super::Foundation::PWSTR, hgroup: *const _HGROUP) -> u32;
pub type PCLUSAPI_CLUSTER_ADD_GROUP_TO_GROUP_GROUPSET = unsafe extern "system" fn(hgroupset: *const _HGROUPSET, hgroup: *const _HGROUP) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUSTER_AFFINITY_RULE_CONTROL = unsafe extern "system" fn(hcluster: *const _HCLUSTER, affinityrulename: super::super::Foundation::PWSTR, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
pub type PCLUSAPI_CLUSTER_CLOSE_ENUM = unsafe extern "system" fn(henum: *const _HCLUSENUM) -> u32;
pub type PCLUSAPI_CLUSTER_CLOSE_ENUM_EX = unsafe extern "system" fn(hclusterenum: *const _HCLUSENUMEX) -> u32;
pub type PCLUSAPI_CLUSTER_CONTROL = unsafe extern "system" fn(hcluster: *const _HCLUSTER, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUSTER_CREATE_AFFINITY_RULE = unsafe extern "system" fn(hcluster: *const _HCLUSTER, rulename: super::super::Foundation::PWSTR, ruletype: CLUS_AFFINITY_RULE_TYPE) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUSTER_ENUM = unsafe extern "system" fn(henum: *const _HCLUSENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUSTER_ENUM_EX = unsafe extern "system" fn(hclusterenum: *const _HCLUSENUMEX, dwindex: u32, pitem: *mut CLUSTER_ENUM_ITEM, cbitem: *mut u32) -> u32;
pub type PCLUSAPI_CLUSTER_GET_ENUM_COUNT = unsafe extern "system" fn(henum: *const _HCLUSENUM) -> u32;
pub type PCLUSAPI_CLUSTER_GET_ENUM_COUNT_EX = unsafe extern "system" fn(hclusterenum: *const _HCLUSENUMEX) -> u32;
pub type PCLUSAPI_CLUSTER_GROUP_CLOSE_ENUM = unsafe extern "system" fn(hgroupenum: *mut _HGROUPENUM) -> u32;
pub type PCLUSAPI_CLUSTER_GROUP_CLOSE_ENUM_EX = unsafe extern "system" fn(hgroupenumex: *const _HGROUPENUMEX) -> u32;
pub type PCLUSAPI_CLUSTER_GROUP_CONTROL = unsafe extern "system" fn(hgroup: *const _HGROUP, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUSTER_GROUP_ENUM = unsafe extern "system" fn(hgroupenum: *const _HGROUPENUM, dwindex: u32, lpdwtype: *mut u32, lpszresourcename: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUSTER_GROUP_ENUM_EX = unsafe extern "system" fn(hgroupenumex: *const _HGROUPENUMEX, dwindex: u32, pitem: *mut CLUSTER_GROUP_ENUM_ITEM, cbitem: *mut u32) -> u32;
pub type PCLUSAPI_CLUSTER_GROUP_GET_ENUM_COUNT = unsafe extern "system" fn(hgroupenum: *const _HGROUPENUM) -> u32;
pub type PCLUSAPI_CLUSTER_GROUP_GET_ENUM_COUNT_EX = unsafe extern "system" fn(hgroupenumex: *const _HGROUPENUMEX) -> u32;
pub type PCLUSAPI_CLUSTER_GROUP_GROUPSET_CONTROL = unsafe extern "system" fn(hgroupset: *const _HGROUPSET, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
pub type PCLUSAPI_CLUSTER_GROUP_OPEN_ENUM = unsafe extern "system" fn(hgroup: *mut _HGROUP, dwtype: u32) -> *mut _HGROUPENUM;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUSTER_GROUP_OPEN_ENUM_EX = unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszproperties: super::super::Foundation::PWSTR, cbproperties: u32, lpszroproperties: super::super::Foundation::PWSTR, cbroproperties: u32, dwflags: u32) -> *mut _HGROUPENUMEX;
pub type PCLUSAPI_CLUSTER_NETWORK_CLOSE_ENUM = unsafe extern "system" fn(hnetworkenum: *const _HNETWORKENUM) -> u32;
pub type PCLUSAPI_CLUSTER_NETWORK_CONTROL = unsafe extern "system" fn(hnetwork: *const _HNETWORK, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUSTER_NETWORK_ENUM = unsafe extern "system" fn(hnetworkenum: *const _HNETWORKENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
pub type PCLUSAPI_CLUSTER_NETWORK_GET_ENUM_COUNT = unsafe extern "system" fn(hnetworkenum: *const _HNETWORKENUM) -> u32;
pub type PCLUSAPI_CLUSTER_NETWORK_OPEN_ENUM = unsafe extern "system" fn(hnetwork: *const _HNETWORK, dwtype: u32) -> *mut _HNETWORKENUM;
pub type PCLUSAPI_CLUSTER_NET_INTERFACE_CONTROL = unsafe extern "system" fn(hnetinterface: *const _HNETINTERFACE, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
pub type PCLUSAPI_CLUSTER_NODE_CLOSE_ENUM = unsafe extern "system" fn(hnodeenum: *const _HNODEENUM) -> u32;
pub type PCLUSAPI_CLUSTER_NODE_CLOSE_ENUM_EX = unsafe extern "system" fn(hnodeenum: *const _HNODEENUMEX) -> u32;
pub type PCLUSAPI_CLUSTER_NODE_CONTROL = unsafe extern "system" fn(hnode: *const _HNODE, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUSTER_NODE_ENUM = unsafe extern "system" fn(hnodeenum: *const _HNODEENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUSTER_NODE_ENUM_EX = unsafe extern "system" fn(hnodeenum: *const _HNODEENUMEX, dwindex: u32, pitem: *mut CLUSTER_ENUM_ITEM, cbitem: *mut u32) -> u32;
pub type PCLUSAPI_CLUSTER_NODE_GET_ENUM_COUNT = unsafe extern "system" fn(hnodeenum: *const _HNODEENUM) -> u32;
pub type PCLUSAPI_CLUSTER_NODE_GET_ENUM_COUNT_EX = unsafe extern "system" fn(hnodeenum: *const _HNODEENUMEX) -> u32;
pub type PCLUSAPI_CLUSTER_NODE_OPEN_ENUM = unsafe extern "system" fn(hnode: *const _HNODE, dwtype: u32) -> *mut _HNODEENUM;
pub type PCLUSAPI_CLUSTER_NODE_OPEN_ENUM_EX = unsafe extern "system" fn(hnode: *const _HNODE, dwtype: u32, poptions: *const ::core::ffi::c_void) -> *mut _HNODEENUMEX;
pub type PCLUSAPI_CLUSTER_OPEN_ENUM = unsafe extern "system" fn(hcluster: *const _HCLUSTER, dwtype: u32) -> *mut _HCLUSENUM;
pub type PCLUSAPI_CLUSTER_OPEN_ENUM_EX = unsafe extern "system" fn(hcluster: *const _HCLUSTER, dwtype: u32, poptions: *const ::core::ffi::c_void) -> *mut _HCLUSENUMEX;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_CLOSE_KEY = unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY) -> i32;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_CLUSTER_REG_CREATE_BATCH = unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, phregbatch: *mut *mut _HREGBATCH) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_CREATE_KEY = unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszsubkey: super::super::Foundation::PWSTR, dwoptions: u32, samdesired: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, phkresult: *mut super::super::System::Registry::HKEY, lpdwdisposition: *mut u32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_DELETE_KEY = unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszsubkey: super::super::Foundation::PWSTR) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_DELETE_VALUE = unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszvaluename: super::super::Foundation::PWSTR) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_ENUM_KEY = unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, dwindex: u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32, lpftlastwritetime: *mut super::super::Foundation::FILETIME) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_ENUM_VALUE = unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, dwindex: u32, lpszvaluename: super::super::Foundation::PWSTR, lpcchvaluename: *mut u32, lpdwtype: *mut u32, lpdata: *mut u8, lpcbdata: *mut u32) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_GET_KEY_SECURITY = unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, requestedinformation: u32, psecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_OPEN_KEY = unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszsubkey: super::super::Foundation::PWSTR, samdesired: u32, phkresult: *mut super::super::System::Registry::HKEY) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_QUERY_INFO_KEY = unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpcsubkeys: *mut u32, lpcbmaxsubkeylen: *mut u32, lpcvalues: *mut u32, lpcbmaxvaluenamelen: *mut u32, lpcbmaxvaluelen: *mut u32, lpcbsecuritydescriptor: *mut u32, lpftlastwritetime: *mut super::super::Foundation::FILETIME) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_QUERY_VALUE = unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszvaluename: super::super::Foundation::PWSTR, lpdwvaluetype: *mut u32, lpdata: *mut u8, lpcbdata: *mut u32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_SET_KEY_SECURITY = unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, securityinformation: u32, psecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_CLUSTER_REG_SET_VALUE = unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, lpszvaluename: super::super::Foundation::PWSTR, dwtype: u32, lpdata: *const u8, cbdata: u32) -> u32;
pub type PCLUSAPI_CLUSTER_REG_SYNC_DATABASE = unsafe extern "system" fn(hcluster: *mut _HCLUSTER, flags: u32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUSTER_REMOVE_AFFINITY_RULE = unsafe extern "system" fn(hcluster: *const _HCLUSTER, rulename: super::super::Foundation::PWSTR) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUSTER_REMOVE_GROUP_FROM_AFFINITY_RULE = unsafe extern "system" fn(hcluster: *const _HCLUSTER, rulename: super::super::Foundation::PWSTR, hgroup: *const _HGROUP) -> u32;
pub type PCLUSAPI_CLUSTER_REMOVE_GROUP_FROM_GROUP_GROUPSET = unsafe extern "system" fn(hgroupset: *const _HGROUPSET, hgroupname: *const _HGROUP) -> u32;
pub type PCLUSAPI_CLUSTER_RESOURCE_CLOSE_ENUM = unsafe extern "system" fn(hresenum: *mut _HRESENUM) -> u32;
pub type PCLUSAPI_CLUSTER_RESOURCE_CLOSE_ENUM_EX = unsafe extern "system" fn(hresourceenumex: *const _HRESENUMEX) -> u32;
pub type PCLUSAPI_CLUSTER_RESOURCE_CONTROL = unsafe extern "system" fn(hresource: *const _HRESOURCE, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUSTER_RESOURCE_ENUM = unsafe extern "system" fn(hresenum: *const _HRESENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUSTER_RESOURCE_ENUM_EX = unsafe extern "system" fn(hresourceenumex: *const _HRESENUMEX, dwindex: u32, pitem: *mut CLUSTER_RESOURCE_ENUM_ITEM, cbitem: *mut u32) -> u32;
pub type PCLUSAPI_CLUSTER_RESOURCE_GET_ENUM_COUNT = unsafe extern "system" fn(hresenum: *const _HRESENUM) -> u32;
pub type PCLUSAPI_CLUSTER_RESOURCE_GET_ENUM_COUNT_EX = unsafe extern "system" fn(hresourceenumex: *const _HRESENUMEX) -> u32;
pub type PCLUSAPI_CLUSTER_RESOURCE_OPEN_ENUM = unsafe extern "system" fn(hresource: *mut _HRESOURCE, dwtype: u32) -> *mut _HRESENUM;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUSTER_RESOURCE_OPEN_ENUM_EX = unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszproperties: super::super::Foundation::PWSTR, cbproperties: u32, lpszroproperties: super::super::Foundation::PWSTR, cbroproperties: u32, dwflags: u32) -> *mut _HRESENUMEX;
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_CLOSE_ENUM = unsafe extern "system" fn(hrestypeenum: *const _HRESTYPEENUM) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_CONTROL = unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszresourcetypename: super::super::Foundation::PWSTR, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_ENUM = unsafe extern "system" fn(hrestypeenum: *const _HRESTYPEENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_GET_ENUM_COUNT = unsafe extern "system" fn(hrestypeenum: *const _HRESTYPEENUM) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_OPEN_ENUM = unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszresourcetypename: super::super::Foundation::PWSTR, dwtype: u32) -> *mut _HRESTYPEENUM;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUSTER_UPGRADE = unsafe extern "system" fn(hcluster: *const _HCLUSTER, perform: super::super::Foundation::BOOL, pfnprogresscallback: PCLUSTER_UPGRADE_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUS_WORKER_CREATE = unsafe extern "system" fn(lpworker: *mut CLUS_WORKER, lpstartaddress: PWORKER_START_ROUTINE, lpparameter: *mut ::core::ffi::c_void) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CLUS_WORKER_TERMINATE = unsafe extern "system" fn(lpworker: *const CLUS_WORKER);
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CREATE_CLUSTER = unsafe extern "system" fn(pconfig: *const CREATE_CLUSTER_CONFIG, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> *mut _HCLUSTER;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CREATE_CLUSTER_AVAILABILITY_SET = unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpavailabilitysetname: super::super::Foundation::PWSTR, pavailabilitysetconfig: *const CLUSTER_AVAILABILITY_SET_CONFIG) -> *mut _HGROUPSET;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CREATE_CLUSTER_CNOLESS = unsafe extern "system" fn(pconfig: *const CREATE_CLUSTER_CONFIG, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> *mut _HCLUSTER;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CREATE_CLUSTER_GROUP = unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszgroupname: super::super::Foundation::PWSTR) -> *mut _HGROUP;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CREATE_CLUSTER_GROUPEX = unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszgroupname: super::super::Foundation::PWSTR, pgroupinfo: *const CLUSTER_CREATE_GROUP_INFO) -> *mut _HGROUP;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CREATE_CLUSTER_GROUP_GROUPSET = unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszgroupsetname: super::super::Foundation::PWSTR) -> *mut _HGROUPSET;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CREATE_CLUSTER_NAME_ACCOUNT = unsafe extern "system" fn(hcluster: *const _HCLUSTER, pconfig: *const CREATE_CLUSTER_NAME_ACCOUNT, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void) -> u32;
pub type PCLUSAPI_CREATE_CLUSTER_NOTIFY_PORT = unsafe extern "system" fn(hchange: *const _HCHANGE, hcluster: *const _HCLUSTER, dwfilter: u32, dwnotifykey: usize) -> *mut _HCHANGE;
pub type PCLUSAPI_CREATE_CLUSTER_NOTIFY_PORT_V2 = unsafe extern "system" fn(hchange: *const _HCHANGE, hcluster: *const _HCLUSTER, filters: *const NOTIFY_FILTER_AND_TYPE, dwfiltercount: u32, dwnotifykey: usize) -> *mut _HCHANGE;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CREATE_CLUSTER_RESOURCE = unsafe extern "system" fn(hgroup: *mut _HGROUP, lpszresourcename: super::super::Foundation::PWSTR, lpszresourcetype: super::super::Foundation::PWSTR, dwflags: u32) -> *mut _HRESOURCE;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_CREATE_CLUSTER_RESOURCE_TYPE = unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszresourcetypename: super::super::Foundation::PWSTR, lpszdisplayname: super::super::Foundation::PWSTR, lpszresourcetypedll: super::super::Foundation::PWSTR, dwlooksalivepollinterval: u32, dwisalivepollinterval: u32) -> u32;
pub type PCLUSAPI_DELETE_CLUSTER_GROUP = unsafe extern "system" fn(hgroup: *mut _HGROUP) -> u32;
pub type PCLUSAPI_DELETE_CLUSTER_GROUP_GROUPSET = unsafe extern "system" fn(hgroupset: *const _HGROUPSET) -> u32;
pub type PCLUSAPI_DELETE_CLUSTER_RESOURCE = unsafe extern "system" fn(hresource: *mut _HRESOURCE) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_DELETE_CLUSTER_RESOURCE_TYPE = unsafe extern "system" fn(hcluster: *mut _HCLUSTER, lpszresourcetypename: super::super::Foundation::PWSTR) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_DESTROY_CLUSTER = unsafe extern "system" fn(hcluster: *const _HCLUSTER, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const ::core::ffi::c_void, fdeletevirtualcomputerobjects: super::super::Foundation::BOOL) -> u32;
pub type PCLUSAPI_DESTROY_CLUSTER_GROUP = unsafe extern "system" fn(hgroup: *mut _HGROUP) -> u32;
pub type PCLUSAPI_EVICT_CLUSTER_NODE = unsafe extern "system" fn(hnode: *const _HNODE) -> u32;
pub type PCLUSAPI_EVICT_CLUSTER_NODE_EX = unsafe extern "system" fn(hnode: *const _HNODE, dwtimeout: u32, phrcleanupstatus: *mut ::windows_sys::core::HRESULT) -> u32;
pub type PCLUSAPI_FAIL_CLUSTER_RESOURCE = unsafe extern "system" fn(hresource: *mut _HRESOURCE) -> u32;
pub type PCLUSAPI_GET_CLUSTER_FROM_GROUP = unsafe extern "system" fn(hgroup: *const _HGROUP) -> *mut _HCLUSTER;
pub type PCLUSAPI_GET_CLUSTER_FROM_GROUP_GROUPSET = unsafe extern "system" fn(hgroupset: *const _HGROUPSET) -> *mut _HCLUSTER;
pub type PCLUSAPI_GET_CLUSTER_FROM_NETWORK = unsafe extern "system" fn(hnetwork: *const _HNETWORK) -> *mut _HCLUSTER;
pub type PCLUSAPI_GET_CLUSTER_FROM_NET_INTERFACE = unsafe extern "system" fn(hnetinterface: *const _HNETINTERFACE) -> *mut _HCLUSTER;
pub type PCLUSAPI_GET_CLUSTER_FROM_NODE = unsafe extern "system" fn(hnode: *const _HNODE) -> *mut _HCLUSTER;
pub type PCLUSAPI_GET_CLUSTER_FROM_RESOURCE = unsafe extern "system" fn(hresource: *const _HRESOURCE) -> *mut _HCLUSTER;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_GROUP_KEY = unsafe extern "system" fn(hgroup: *mut _HGROUP, samdesired: u32) -> super::super::System::Registry::HKEY;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_GET_CLUSTER_GROUP_STATE = unsafe extern "system" fn(hgroup: *const _HGROUP, lpsznodename: super::super::Foundation::PWSTR, lpcchnodename: *mut u32) -> CLUSTER_GROUP_STATE;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_GET_CLUSTER_INFORMATION = unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszclustername: super::super::Foundation::PWSTR, lpcchclustername: *mut u32, lpclusterinfo: *mut CLUSTERVERSIONINFO) -> u32;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_KEY = unsafe extern "system" fn(hcluster: *mut _HCLUSTER, samdesired: u32) -> super::super::System::Registry::HKEY;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_GET_CLUSTER_NETWORK_ID = unsafe extern "system" fn(hnetwork: *const _HNETWORK, lpsznetworkid: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_NETWORK_KEY = unsafe extern "system" fn(hnetwork: *const _HNETWORK, samdesired: u32) -> super::super::System::Registry::HKEY;
pub type PCLUSAPI_GET_CLUSTER_NETWORK_STATE = unsafe extern "system" fn(hnetwork: *const _HNETWORK) -> CLUSTER_NETWORK_STATE;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_GET_CLUSTER_NET_INTERFACE = unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznodename: super::super::Foundation::PWSTR, lpsznetworkname: super::super::Foundation::PWSTR, lpszinterfacename: super::super::Foundation::PWSTR, lpcchinterfacename: *mut u32) -> u32;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_NET_INTERFACE_KEY = unsafe extern "system" fn(hnetinterface: *const _HNETINTERFACE, samdesired: u32) -> super::super::System::Registry::HKEY;
pub type PCLUSAPI_GET_CLUSTER_NET_INTERFACE_STATE = unsafe extern "system" fn(hnetinterface: *const _HNETINTERFACE) -> CLUSTER_NETINTERFACE_STATE;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_GET_CLUSTER_NODE_ID = unsafe extern "system" fn(hnode: *const _HNODE, lpsznodeid: super::super::Foundation::PWSTR, lpcchname: *mut u32) -> u32;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_NODE_KEY = unsafe extern "system" fn(hnode: *mut _HNODE, samdesired: u32) -> super::super::System::Registry::HKEY;
pub type PCLUSAPI_GET_CLUSTER_NODE_STATE = unsafe extern "system" fn(hnode: *const _HNODE) -> CLUSTER_NODE_STATE;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_GET_CLUSTER_NOTIFY = unsafe extern "system" fn(hchange: *const _HCHANGE, lpdwnotifykey: *mut usize, lpdwfiltertype: *mut u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32, dwmilliseconds: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_GET_CLUSTER_NOTIFY_V2 =
    unsafe extern "system" fn(hchange: *const _HCHANGE, lpdwnotifykey: *mut usize, pfilterandtype: *mut NOTIFY_FILTER_AND_TYPE, buffer: *mut u8, lpcchbuffersize: *mut u32, lpszobjectid: super::super::Foundation::PWSTR, lpcchobjectid: *mut u32, lpszparentid: super::super::Foundation::PWSTR, lpcchparentid: *mut u32, lpszname: super::super::Foundation::PWSTR, lpcchname: *mut u32, lpsztype: super::super::Foundation::PWSTR, lpcchtype: *mut u32, dwmilliseconds: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_GET_CLUSTER_QUORUM_RESOURCE = unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszresourcename: super::super::Foundation::PWSTR, lpcchresourcename: *mut u32, lpszdevicename: super::super::Foundation::PWSTR, lpcchdevicename: *mut u32, lpdwmaxquorumlogsize: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_DEPENDENCY_EXPRESSION = unsafe extern "system" fn(hresource: *const _HRESOURCE, lpszdependencyexpression: super::super::Foundation::PWSTR, lpcchdependencyexpression: *mut u32) -> u32;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_KEY = unsafe extern "system" fn(hresource: *mut _HRESOURCE, samdesired: u32) -> super::super::System::Registry::HKEY;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_NETWORK_NAME = unsafe extern "system" fn(hresource: *const _HRESOURCE, lpbuffer: super::super::Foundation::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_STATE = unsafe extern "system" fn(hresource: *const _HRESOURCE, lpsznodename: super::super::Foundation::PWSTR, lpcchnodename: *mut u32, lpszgroupname: super::super::Foundation::PWSTR, lpcchgroupname: *mut u32) -> CLUSTER_RESOURCE_STATE;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_TYPE_KEY = unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsztypename: super::super::Foundation::PWSTR, samdesired: u32) -> super::super::System::Registry::HKEY;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_GET_NODE_CLUSTER_STATE = unsafe extern "system" fn(lpsznodename: super::super::Foundation::PWSTR, pdwclusterstate: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_GET_NOTIFY_EVENT_HANDLE_V2 = unsafe extern "system" fn(hchange: *const _HCHANGE, lphtargetevent: *mut super::super::Foundation::HANDLE) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_IS_FILE_ON_CLUSTER_SHARED_VOLUME = unsafe extern "system" fn(lpszpathname: super::super::Foundation::PWSTR, pbfileisonsharedvolume: *mut super::super::Foundation::BOOL) -> u32;
pub type PCLUSAPI_MOVE_CLUSTER_GROUP = unsafe extern "system" fn(hgroup: *const _HGROUP, hdestinationnode: *const _HNODE) -> u32;
pub type PCLUSAPI_OFFLINE_CLUSTER_GROUP = unsafe extern "system" fn(hgroup: *mut _HGROUP) -> u32;
pub type PCLUSAPI_OFFLINE_CLUSTER_RESOURCE = unsafe extern "system" fn(hresource: *mut _HRESOURCE) -> u32;
pub type PCLUSAPI_ONLINE_CLUSTER_GROUP = unsafe extern "system" fn(hgroup: *const _HGROUP, hdestinationnode: *const _HNODE) -> u32;
pub type PCLUSAPI_ONLINE_CLUSTER_RESOURCE = unsafe extern "system" fn(hresource: *mut _HRESOURCE) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_OPEN_CLUSTER = unsafe extern "system" fn(lpszclustername: super::super::Foundation::PWSTR) -> *mut _HCLUSTER;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_OPEN_CLUSTER_EX = unsafe extern "system" fn(lpszclustername: super::super::Foundation::PWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HCLUSTER;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_OPEN_CLUSTER_GROUP = unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszgroupname: super::super::Foundation::PWSTR) -> *mut _HGROUP;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_OPEN_CLUSTER_GROUP_EX = unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszgroupname: super::super::Foundation::PWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HGROUP;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_OPEN_CLUSTER_GROUP_GROUPSET = unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszgroupsetname: super::super::Foundation::PWSTR) -> *mut _HGROUPSET;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_OPEN_CLUSTER_NETINTERFACE_EX = unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznetinterfacename: super::super::Foundation::PWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HNETINTERFACE;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_OPEN_CLUSTER_NETWORK = unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznetworkname: super::super::Foundation::PWSTR) -> *mut _HNETWORK;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_OPEN_CLUSTER_NETWORK_EX = unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznetworkname: super::super::Foundation::PWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HNETWORK;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_OPEN_CLUSTER_NET_INTERFACE = unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszinterfacename: super::super::Foundation::PWSTR) -> *mut _HNETINTERFACE;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_OPEN_CLUSTER_NODE = unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznodename: super::super::Foundation::PWSTR) -> *mut _HNODE;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_OPEN_CLUSTER_NODE_EX = unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznodename: super::super::Foundation::PWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HNODE;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_OPEN_CLUSTER_RESOURCE = unsafe extern "system" fn(hcluster: *mut _HCLUSTER, lpszresourcename: super::super::Foundation::PWSTR) -> *mut _HRESOURCE;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_OPEN_CLUSTER_RESOURCE_EX = unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszresourcename: super::super::Foundation::PWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> *mut _HRESOURCE;
pub type PCLUSAPI_OPEN_NODE_BY_ID = unsafe extern "system" fn(hcluster: *const _HCLUSTER, nodeid: u32) -> *mut _HNODE;
pub type PCLUSAPI_PAUSE_CLUSTER_NODE = unsafe extern "system" fn(hnode: *const _HNODE) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_PAUSE_CLUSTER_NODE_EX = unsafe extern "system" fn(hnode: *const _HNODE, bdrainnode: super::super::Foundation::BOOL, dwpauseflags: u32, hnodedraintarget: *const _HNODE) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_REGISTER_CLUSTER_NOTIFY = unsafe extern "system" fn(hchange: *const _HCHANGE, dwfiltertype: u32, hobject: super::super::Foundation::HANDLE, dwnotifykey: usize) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_REGISTER_CLUSTER_NOTIFY_V2 = unsafe extern "system" fn(hchange: *const _HCHANGE, filter: NOTIFY_FILTER_AND_TYPE, hobject: super::super::Foundation::HANDLE, dwnotifykey: usize) -> u32;
pub type PCLUSAPI_REMOVE_CLUSTER_GROUP_DEPENDENCY = unsafe extern "system" fn(hgroup: *const _HGROUP, hdependson: *const _HGROUP) -> u32;
pub type PCLUSAPI_REMOVE_CLUSTER_GROUP_GROUPSET_DEPENDENCY = unsafe extern "system" fn(hgroupset: *const _HGROUPSET, hdependson: *const _HGROUPSET) -> u32;
pub type PCLUSAPI_REMOVE_CLUSTER_GROUP_TO_GROUP_GROUPSET_DEPENDENCY = unsafe extern "system" fn(hgroup: *const _HGROUP, hdependson: *const _HGROUPSET) -> u32;
pub type PCLUSAPI_REMOVE_CLUSTER_NAME_ACCOUNT = unsafe extern "system" fn(hcluster: *const _HCLUSTER) -> u32;
pub type PCLUSAPI_REMOVE_CLUSTER_RESOURCE_DEPENDENCY = unsafe extern "system" fn(hresource: *mut _HRESOURCE, hdependson: *mut _HRESOURCE) -> u32;
pub type PCLUSAPI_REMOVE_CLUSTER_RESOURCE_NODE = unsafe extern "system" fn(hresource: *mut _HRESOURCE, hnode: *mut _HNODE) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_REMOVE_CROSS_CLUSTER_GROUPSET_DEPENDENCY = unsafe extern "system" fn(hdependentgroupset: *const _HGROUPSET, lpremoteclustername: super::super::Foundation::PWSTR, lpremotegroupsetname: super::super::Foundation::PWSTR) -> u32;
pub type PCLUSAPI_REMOVE_RESOURCE_FROM_CLUSTER_SHARED_VOLUMES = unsafe extern "system" fn(hresource: *const _HRESOURCE) -> u32;
pub type PCLUSAPI_RESTART_CLUSTER_RESOURCE = unsafe extern "system" fn(hresource: *mut _HRESOURCE, dwflags: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_RESTORE_CLUSTER_DATABASE = unsafe extern "system" fn(lpszpathname: super::super::Foundation::PWSTR, bforce: super::super::Foundation::BOOL, lpszquorumdriveletter: super::super::Foundation::PWSTR) -> u32;
pub type PCLUSAPI_RESUME_CLUSTER_NODE = unsafe extern "system" fn(hnode: *const _HNODE) -> u32;
pub type PCLUSAPI_RESUME_CLUSTER_NODE_EX = unsafe extern "system" fn(hnode: *const _HNODE, eresumefailbacktype: CLUSTER_NODE_RESUME_FAILBACK_TYPE, dwresumeflagsreserved: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_SET_CLUSTER_GROUP_GROUPSET_DEPENDENCY_EXPRESSION = unsafe extern "system" fn(hgroupset: *const _HGROUPSET, lpszdependencyexpression: super::super::Foundation::PWSTR) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_SET_CLUSTER_GROUP_NAME = unsafe extern "system" fn(hgroup: *mut _HGROUP, lpszgroupname: super::super::Foundation::PWSTR) -> u32;
pub type PCLUSAPI_SET_CLUSTER_GROUP_NODE_LIST = unsafe extern "system" fn(hgroup: *const _HGROUP, nodecount: u32, nodelist: *const *const _HNODE) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_SET_CLUSTER_NETWORK_NAME = unsafe extern "system" fn(hnetwork: *const _HNETWORK, lpszname: super::super::Foundation::PWSTR) -> u32;
pub type PCLUSAPI_SET_CLUSTER_NETWORK_PRIORITY_ORDER = unsafe extern "system" fn(hcluster: *const _HCLUSTER, networkcount: u32, networklist: *const *const _HNETWORK) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_SET_CLUSTER_QUORUM_RESOURCE = unsafe extern "system" fn(hresource: *const _HRESOURCE, lpszdevicename: super::super::Foundation::PWSTR, dwmaxquologsize: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_SET_CLUSTER_RESOURCE_DEPENDENCY_EXPRESSION = unsafe extern "system" fn(hresource: *const _HRESOURCE, lpszdependencyexpression: super::super::Foundation::PWSTR) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_SET_CLUSTER_RESOURCE_NAME = unsafe extern "system" fn(hresource: *mut _HRESOURCE, lpszresourcename: super::super::Foundation::PWSTR) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_SET_CLUSTER_SERVICE_ACCOUNT_PASSWORD = unsafe extern "system" fn(lpszclustername: super::super::Foundation::PWSTR, lpsznewpassword: super::super::Foundation::PWSTR, dwflags: u32, lpreturnstatusbuffer: *mut CLUSTER_SET_PASSWORD_STATUS, lpcbreturnstatusbuffersize: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_SET_GROUP_DEPENDENCY_EXPRESSION = unsafe extern "system" fn(hgroupset: *const _HGROUP, lpszdependencyexpression: super::super::Foundation::PWSTR) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_SHARED_VOLUME_SET_SNAPSHOT_STATE = unsafe extern "system" fn(guidsnapshotset: ::windows_sys::core::GUID, lpszvolumename: super::super::Foundation::PWSTR, state: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSAPI_SetClusterName = unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznewclustername: super::super::Foundation::PWSTR) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSTER_CLEAR_BACKUP_STATE_FOR_SHARED_VOLUME = unsafe extern "system" fn(lpszvolumepathname: super::super::Foundation::PWSTR) -> u32;
pub type PCLUSTER_DECRYPT = unsafe extern "system" fn(hcluscryptprovider: *const _HCLUSCRYPTPROVIDER, pcryptinput: *const u8, cbcryptinput: u32, ppcryptoutput: *mut *mut u8, pcbcryptoutput: *mut u32) -> u32;
pub type PCLUSTER_ENCRYPT = unsafe extern "system" fn(hcluscryptprovider: *const _HCLUSCRYPTPROVIDER, pdata: *const u8, cbdata: u32, ppdata: *mut *mut u8, pcbdata: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSTER_GET_VOLUME_NAME_FOR_VOLUME_MOUNT_POINT = unsafe extern "system" fn(lpszvolumemountpoint: super::super::Foundation::PWSTR, lpszvolumename: super::super::Foundation::PWSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSTER_GET_VOLUME_PATH_NAME = unsafe extern "system" fn(lpszfilename: super::super::Foundation::PWSTR, lpszvolumepathname: super::super::Foundation::PWSTR, cchbufferlength: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSTER_IS_PATH_ON_SHARED_VOLUME = unsafe extern "system" fn(lpszpathname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSTER_PREPARE_SHARED_VOLUME_FOR_BACKUP = unsafe extern "system" fn(lpszfilename: super::super::Foundation::PWSTR, lpszvolumepathname: super::super::Foundation::PWSTR, lpcchvolumepathname: *mut u32, lpszvolumename: super::super::Foundation::PWSTR, lpcchvolumename: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSTER_REG_BATCH_ADD_COMMAND = unsafe extern "system" fn(hregbatch: *const _HREGBATCH, dwcommand: CLUSTER_REG_COMMAND, wzname: super::super::Foundation::PWSTR, dwoptions: u32, lpdata: *const ::core::ffi::c_void, cbdata: u32) -> i32;
pub type PCLUSTER_REG_BATCH_CLOSE_NOTIFICATION = unsafe extern "system" fn(hbatchnotification: *const _HREGBATCHNOTIFICATION) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSTER_REG_BATCH_READ_COMMAND = unsafe extern "system" fn(hbatchnotification: *const _HREGBATCHNOTIFICATION, pbatchcommand: *mut CLUSTER_BATCH_COMMAND) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSTER_REG_CLOSE_BATCH = unsafe extern "system" fn(hregbatch: *const _HREGBATCH, bcommit: super::super::Foundation::BOOL, failedcommandnumber: *mut i32) -> i32;
pub type PCLUSTER_REG_CLOSE_BATCH_NOTIFY_PORT = unsafe extern "system" fn(hbatchnotifyport: *const _HREGBATCHPORT) -> i32;
pub type PCLUSTER_REG_CLOSE_READ_BATCH = unsafe extern "system" fn(hregreadbatch: *const _HREGREADBATCH, phregreadbatchreply: *mut *mut _HREGREADBATCHREPLY) -> i32;
pub type PCLUSTER_REG_CLOSE_READ_BATCH_EX = unsafe extern "system" fn(hregreadbatch: *const _HREGREADBATCH, flags: u32, phregreadbatchreply: *mut *mut _HREGREADBATCHREPLY) -> i32;
pub type PCLUSTER_REG_CLOSE_READ_BATCH_REPLY = unsafe extern "system" fn(hregreadbatchreply: *const _HREGREADBATCHREPLY) -> i32;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSTER_REG_CREATE_BATCH_NOTIFY_PORT = unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, phbatchnotifyport: *mut *mut _HREGBATCHPORT) -> i32;
#[cfg(feature = "Win32_System_Registry")]
pub type PCLUSTER_REG_CREATE_READ_BATCH = unsafe extern "system" fn(hkey: super::super::System::Registry::HKEY, phregreadbatch: *mut *mut _HREGREADBATCH) -> i32;
pub type PCLUSTER_REG_GET_BATCH_NOTIFICATION = unsafe extern "system" fn(hbatchnotify: *const _HREGBATCHPORT, phbatchnotification: *mut *mut _HREGBATCHNOTIFICATION) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSTER_REG_READ_BATCH_ADD_COMMAND = unsafe extern "system" fn(hregreadbatch: *const _HREGREADBATCH, wzsubkeyname: super::super::Foundation::PWSTR, wzvaluename: super::super::Foundation::PWSTR) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSTER_REG_READ_BATCH_REPLY_NEXT_COMMAND = unsafe extern "system" fn(hregreadbatchreply: *const _HREGREADBATCHREPLY, pbatchcommand: *mut CLUSTER_READ_BATCH_COMMAND) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSTER_SETUP_PROGRESS_CALLBACK = unsafe extern "system" fn(pvcallbackarg: *mut ::core::ffi::c_void, esetupphase: CLUSTER_SETUP_PHASE, ephasetype: CLUSTER_SETUP_PHASE_TYPE, ephaseseverity: CLUSTER_SETUP_PHASE_SEVERITY, dwpercentcomplete: u32, lpszobjectname: super::super::Foundation::PWSTR, dwstatus: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSTER_SET_ACCOUNT_ACCESS = unsafe extern "system" fn(hcluster: *const _HCLUSTER, szaccountsid: super::super::Foundation::PWSTR, dwaccess: u32, dwcontroltype: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PCLUSTER_UPGRADE_PROGRESS_CALLBACK = unsafe extern "system" fn(pvcallbackarg: *mut ::core::ffi::c_void, eupgradephase: CLUSTER_UPGRADE_PHASE) -> super::super::Foundation::BOOL;
pub type PEND_CONTROL_CALL = unsafe extern "system" fn(context: i64, status: u32) -> u32;
pub type PEND_TYPE_CONTROL_CALL = unsafe extern "system" fn(context: i64, status: u32) -> u32;
pub type PEXTEND_RES_CONTROL_CALL = unsafe extern "system" fn(context: i64, newtimeoutinms: u32) -> u32;
pub type PEXTEND_RES_TYPE_CONTROL_CALL = unsafe extern "system" fn(context: i64, newtimeoutinms: u32) -> u32;
pub type PFREE_CLUSTER_CRYPT = unsafe extern "system" fn(pcryptinfo: *const ::core::ffi::c_void) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PIS_ALIVE_ROUTINE = unsafe extern "system" fn(resource: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
pub const PLACEMENT_OPTIONS_MIN_VALUE: i32 = 0i32;
pub const PLACEMENT_OPTIONS_DEFAULT_PLACEMENT_OPTIONS: i32 = 0i32;
pub const PLACEMENT_OPTIONS_DISABLE_CSV_VM_DEPENDENCY: i32 = 1i32;
pub const PLACEMENT_OPTIONS_CONSIDER_OFFLINE_VMS: i32 = 2i32;
pub const PLACEMENT_OPTIONS_DONT_USE_MEMORY: i32 = 4i32;
pub const PLACEMENT_OPTIONS_DONT_USE_CPU: i32 = 8i32;
pub const PLACEMENT_OPTIONS_DONT_USE_LOCAL_TEMP_DISK: i32 = 16i32;
pub const PLACEMENT_OPTIONS_DONT_RESUME_VMS_WITH_EXISTING_TEMP_DISK: i32 = 32i32;
pub const PLACEMENT_OPTIONS_SAVE_VMS_WITH_LOCAL_DISK_ON_DRAIN_OVERWRITE: i32 = 64i32;
pub const PLACEMENT_OPTIONS_DONT_RESUME_AVAILABILTY_SET_VMS_WITH_EXISTING_TEMP_DISK: i32 = 128i32;
pub const PLACEMENT_OPTIONS_SAVE_AVAILABILTY_SET_VMS_WITH_LOCAL_DISK_ON_DRAIN_OVERWRITE: i32 = 256i32;
pub const PLACEMENT_OPTIONS_AVAILABILITY_SET_DOMAIN_AFFINITY: i32 = 512i32;
pub const PLACEMENT_OPTIONS_ALL: i32 = 1023i32;
#[cfg(feature = "Win32_Foundation")]
pub type PLOG_EVENT_ROUTINE = unsafe extern "system" fn(resourcehandle: isize, loglevel: LOG_LEVEL, formatstring: super::super::Foundation::PWSTR);
#[cfg(feature = "Win32_Foundation")]
pub type PLOOKS_ALIVE_ROUTINE = unsafe extern "system" fn(resource: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
pub type POFFLINE_ROUTINE = unsafe extern "system" fn(resource: *mut ::core::ffi::c_void) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type POFFLINE_V2_ROUTINE = unsafe extern "system" fn(resource: *const ::core::ffi::c_void, destinationnodename: super::super::Foundation::PWSTR, offlineflags: u32, inbuffer: *const u8, inbuffersize: u32, reserved: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PONLINE_ROUTINE = unsafe extern "system" fn(resource: *mut ::core::ffi::c_void, eventhandle: *mut super::super::Foundation::HANDLE) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PONLINE_V2_ROUTINE = unsafe extern "system" fn(resource: *const ::core::ffi::c_void, eventhandle: *mut super::super::Foundation::HANDLE, onlineflags: u32, inbuffer: *const u8, inbuffersize: u32, reserved: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type POPEN_CLUSTER_CRYPT_PROVIDER = unsafe extern "system" fn(lpszresource: super::super::Foundation::PWSTR, lpszprovider: *const i8, dwtype: u32, dwflags: u32) -> *mut _HCLUSCRYPTPROVIDER;
#[cfg(feature = "Win32_Foundation")]
pub type POPEN_CLUSTER_CRYPT_PROVIDEREX = unsafe extern "system" fn(lpszresource: super::super::Foundation::PWSTR, lpszkeyname: super::super::Foundation::PWSTR, lpszprovider: *const i8, dwtype: u32, dwflags: u32) -> *mut _HCLUSCRYPTPROVIDER;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type POPEN_ROUTINE = unsafe extern "system" fn(resourcename: super::super::Foundation::PWSTR, resourcekey: super::super::System::Registry::HKEY, resourcehandle: isize) -> *mut ::core::ffi::c_void;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type POPEN_V2_ROUTINE = unsafe extern "system" fn(resourcename: super::super::Foundation::PWSTR, resourcekey: super::super::System::Registry::HKEY, resourcehandle: isize, openflags: u32) -> *mut ::core::ffi::c_void;
#[repr(C)]
pub struct POST_UPGRADE_VERSION_INFO {
    pub newMajorVersion: u32,
    pub newUpgradeVersion: u32,
    pub oldMajorVersion: u32,
    pub oldUpgradeVersion: u32,
    pub reserved: u32,
}
impl ::core::marker::Copy for POST_UPGRADE_VERSION_INFO {}
impl ::core::clone::Clone for POST_UPGRADE_VERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
pub type PQUERY_APPINSTANCE_VERSION = unsafe extern "system" fn(appinstanceid: *const ::windows_sys::core::GUID, instanceversionhigh: *mut u64, instanceversionlow: *mut u64, versionstatus: *mut super::super::Foundation::NTSTATUS) -> u32;
pub type PQUORUM_RESOURCE_LOST = unsafe extern "system" fn(resource: isize);
#[cfg(feature = "Win32_Foundation")]
pub type PRAISE_RES_TYPE_NOTIFICATION = unsafe extern "system" fn(resourcetype: super::super::Foundation::PWSTR, ppayload: *const u8, payloadsize: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PREGISTER_APPINSTANCE = unsafe extern "system" fn(processhandle: super::super::Foundation::HANDLE, appinstanceid: *const ::windows_sys::core::GUID, childreninheritappinstance: super::super::Foundation::BOOL) -> u32;
pub type PREGISTER_APPINSTANCE_VERSION = unsafe extern "system" fn(appinstanceid: *const ::windows_sys::core::GUID, instanceversionhigh: u64, instanceversionlow: u64) -> u32;
pub type PRELEASE_ROUTINE = unsafe extern "system" fn(resource: *mut ::core::ffi::c_void) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PREQUEST_DUMP_ROUTINE = unsafe extern "system" fn(resourcehandle: isize, dumpduetocallinprogress: super::super::Foundation::BOOL, dumpdelayinms: u32) -> u32;
pub type PRESET_ALL_APPINSTANCE_VERSIONS = unsafe extern "system" fn() -> u32;
pub type PRESOURCE_CONTROL_ROUTINE = unsafe extern "system" fn(resource: *mut ::core::ffi::c_void, controlcode: u32, inbuffer: *mut ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESOURCE_TYPE_CONTROL_ROUTINE = unsafe extern "system" fn(resourcetypename: super::super::Foundation::PWSTR, controlcode: u32, inbuffer: *mut ::core::ffi::c_void, inbuffersize: u32, outbuffer: *mut ::core::ffi::c_void, outbuffersize: u32, bytesreturned: *mut u32) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_ADD_UNKNOWN_PROPERTIES = unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut ::core::ffi::c_void, pcboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_CREATE_DIRECTORY_TREE = unsafe extern "system" fn(pszpath: super::super::Foundation::PWSTR) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_DUP_PARAMETER_BLOCK = unsafe extern "system" fn(poutparams: *mut u8, pinparams: *const u8, ppropertytable: *const RESUTIL_PROPERTY_ITEM) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_DUP_STRING = unsafe extern "system" fn(pszinstring: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_ENUM_PRIVATE_PROPERTIES = unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszoutproperties: super::super::Foundation::PWSTR, cboutpropertiessize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_ENUM_PROPERTIES = unsafe extern "system" fn(ppropertytable: *const RESUTIL_PROPERTY_ITEM, pszoutproperties: super::super::Foundation::PWSTR, cboutpropertiessize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_ENUM_RESOURCES = unsafe extern "system" fn(hself: *mut _HRESOURCE, lpszrestypename: super::super::Foundation::PWSTR, prescallback: LPRESOURCE_CALLBACK, pparameter: *mut ::core::ffi::c_void) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_ENUM_RESOURCES_EX = unsafe extern "system" fn(hcluster: *mut _HCLUSTER, hself: *mut _HRESOURCE, lpszrestypename: super::super::Foundation::PWSTR, prescallback: LPRESOURCE_CALLBACK_EX, pparameter: *mut ::core::ffi::c_void) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_ENUM_RESOURCES_EX2 = unsafe extern "system" fn(hcluster: *mut _HCLUSTER, hself: *mut _HRESOURCE, lpszrestypename: super::super::Foundation::PWSTR, prescallback: LPRESOURCE_CALLBACK_EX, pparameter: *mut ::core::ffi::c_void, dwdesiredaccess: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_EXPAND_ENVIRONMENT_STRINGS = unsafe extern "system" fn(pszsrc: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_FIND_BINARY_PROPERTY = unsafe extern "system" fn(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: super::super::Foundation::PWSTR, pbpropertyvalue: *mut *mut u8, pcbpropertyvaluesize: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_FIND_DEPENDENT_DISK_RESOURCE_DRIVE_LETTER = unsafe extern "system" fn(hcluster: *const _HCLUSTER, hresource: *const _HRESOURCE, pszdriveletter: super::super::Foundation::PWSTR, pcchdriveletter: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_FIND_DWORD_PROPERTY = unsafe extern "system" fn(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: super::super::Foundation::PWSTR, pdwpropertyvalue: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_FIND_EXPANDED_SZ_PROPERTY = unsafe extern "system" fn(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: super::super::Foundation::PWSTR, pszpropertyvalue: *mut super::super::Foundation::PWSTR) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_FIND_EXPAND_SZ_PROPERTY = unsafe extern "system" fn(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: super::super::Foundation::PWSTR, pszpropertyvalue: *mut super::super::Foundation::PWSTR) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_FIND_FILETIME_PROPERTY = unsafe extern "system" fn(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: super::super::Foundation::PWSTR, pftpropertyvalue: *mut super::super::Foundation::FILETIME) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_FIND_LONG_PROPERTY = unsafe extern "system" fn(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: super::super::Foundation::PWSTR, plpropertyvalue: *mut i32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_FIND_MULTI_SZ_PROPERTY = unsafe extern "system" fn(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: super::super::Foundation::PWSTR, pszpropertyvalue: *mut super::super::Foundation::PWSTR, pcbpropertyvaluesize: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_FIND_SZ_PROPERTY = unsafe extern "system" fn(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: super::super::Foundation::PWSTR, pszpropertyvalue: *mut super::super::Foundation::PWSTR) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_FIND_ULARGEINTEGER_PROPERTY = unsafe extern "system" fn(ppropertylist: *const ::core::ffi::c_void, cbpropertylistsize: u32, pszpropertyname: super::super::Foundation::PWSTR, plpropertyvalue: *mut u64) -> u32;
pub type PRESUTIL_FREE_ENVIRONMENT = unsafe extern "system" fn(lpenvironment: *mut ::core::ffi::c_void) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_FREE_PARAMETER_BLOCK = unsafe extern "system" fn(poutparams: *mut u8, pinparams: *const u8, ppropertytable: *const RESUTIL_PROPERTY_ITEM);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_GET_ALL_PROPERTIES = unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut ::core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32;
pub type PRESUTIL_GET_BINARY_PROPERTY = unsafe extern "system" fn(ppboutvalue: *mut *mut u8, pcboutvaluesize: *mut u32, pvaluestruct: *const CLUSPROP_BINARY, pboldvalue: *const u8, cboldvaluesize: u32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_GET_BINARY_VALUE = unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, ppboutvalue: *mut *mut u8, pcboutvaluesize: *mut u32) -> u32;
pub type PRESUTIL_GET_CORE_CLUSTER_RESOURCES = unsafe extern "system" fn(hcluster: *const _HCLUSTER, phclusternameresource: *mut *mut _HRESOURCE, phclusteripaddressresource: *mut *mut _HRESOURCE, phclusterquorumresource: *mut *mut _HRESOURCE) -> u32;
pub type PRESUTIL_GET_CORE_CLUSTER_RESOURCES_EX = unsafe extern "system" fn(hclusterin: *const _HCLUSTER, phclusternameresourceout: *mut *mut _HRESOURCE, phclusteripaddressresourceout: *mut *mut _HRESOURCE, phclusterquorumresourceout: *mut *mut _HRESOURCE, dwdesiredaccess: u32) -> u32;
pub type PRESUTIL_GET_DWORD_PROPERTY = unsafe extern "system" fn(pdwoutvalue: *mut u32, pvaluestruct: *const CLUSPROP_DWORD, dwoldvalue: u32, dwminimum: u32, dwmaximum: u32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_GET_DWORD_VALUE = unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, pdwoutvalue: *mut u32, dwdefaultvalue: u32) -> u32;
pub type PRESUTIL_GET_ENVIRONMENT_WITH_NET_NAME = unsafe extern "system" fn(hresource: *const _HRESOURCE) -> *mut ::core::ffi::c_void;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_GET_EXPAND_SZ_VALUE = unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, bexpand: super::super::Foundation::BOOL) -> super::super::Foundation::PWSTR;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_FILETIME_PROPERTY = unsafe extern "system" fn(pftoutvalue: *mut super::super::Foundation::FILETIME, pvaluestruct: *const CLUSPROP_FILETIME, ftoldvalue: super::super::Foundation::FILETIME, ftminimum: super::super::Foundation::FILETIME, ftmaximum: super::super::Foundation::FILETIME, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32;
pub type PRESUTIL_GET_LONG_PROPERTY = unsafe extern "system" fn(ploutvalue: *mut i32, pvaluestruct: *const CLUSPROP_LONG, loldvalue: i32, lminimum: i32, lmaximum: i32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_MULTI_SZ_PROPERTY = unsafe extern "system" fn(ppszoutvalue: *mut super::super::Foundation::PWSTR, pcboutvaluesize: *mut u32, pvaluestruct: *const CLUSPROP_SZ, pszoldvalue: super::super::Foundation::PWSTR, cboldvaluesize: u32, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_GET_PRIVATE_PROPERTIES = unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, poutpropertylist: *mut ::core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_GET_PROPERTIES = unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut ::core::ffi::c_void, cboutpropertylistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_GET_PROPERTIES_TO_PARAMETER_BLOCK = unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutparams: *mut u8, bcheckforrequiredproperties: super::super::Foundation::BOOL, psznameofpropinerror: *mut super::super::Foundation::PWSTR) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_GET_PROPERTY = unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytableitem: *const RESUTIL_PROPERTY_ITEM, poutpropertyitem: *mut *mut ::core::ffi::c_void, pcboutpropertyitemsize: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_PROPERTY_FORMATS = unsafe extern "system" fn(ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertyformatlist: *mut ::core::ffi::c_void, cbpropertyformatlistsize: u32, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_GET_PROPERTY_SIZE = unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytableitem: *const RESUTIL_PROPERTY_ITEM, pcboutpropertylistsize: *mut u32, pnpropertycount: *mut u32) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_GET_QWORD_VALUE = unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, pqwoutvalue: *mut u64, qwdefaultvalue: u64) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY = unsafe extern "system" fn(hself: super::super::Foundation::HANDLE, lpszresourcetype: super::super::Foundation::PWSTR) -> *mut _HRESOURCE;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_CLASS = unsafe extern "system" fn(hcluster: *mut _HCLUSTER, hself: super::super::Foundation::HANDLE, prci: *mut CLUS_RESOURCE_CLASS_INFO, brecurse: super::super::Foundation::BOOL) -> *mut _HRESOURCE;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_CLASS_EX = unsafe extern "system" fn(hcluster: *mut _HCLUSTER, hself: super::super::Foundation::HANDLE, prci: *mut CLUS_RESOURCE_CLASS_INFO, brecurse: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> *mut _HRESOURCE;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_NAME = unsafe extern "system" fn(hcluster: *mut _HCLUSTER, hself: super::super::Foundation::HANDLE, lpszresourcetype: super::super::Foundation::PWSTR, brecurse: super::super::Foundation::BOOL) -> *mut _HRESOURCE;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY_BY_NAME_EX = unsafe extern "system" fn(hcluster: *mut _HCLUSTER, hself: super::super::Foundation::HANDLE, lpszresourcetype: super::super::Foundation::PWSTR, brecurse: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> *mut _HRESOURCE;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_RESOURCE_DEPENDENCY_EX = unsafe extern "system" fn(hself: super::super::Foundation::HANDLE, lpszresourcetype: super::super::Foundation::PWSTR, dwdesiredaccess: u32) -> *mut _HRESOURCE;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_RESOURCE_DEPENDENTIP_ADDRESS_PROPS = unsafe extern "system" fn(hresource: *const _HRESOURCE, pszaddress: super::super::Foundation::PWSTR, pcchaddress: *mut u32, pszsubnetmask: super::super::Foundation::PWSTR, pcchsubnetmask: *mut u32, psznetwork: super::super::Foundation::PWSTR, pcchnetwork: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_RESOURCE_NAME = unsafe extern "system" fn(hresource: *const _HRESOURCE, pszresourcename: super::super::Foundation::PWSTR, pcchresourcenameinout: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_RESOURCE_NAME_DEPENDENCY = unsafe extern "system" fn(lpszresourcename: super::super::Foundation::PWSTR, lpszresourcetype: super::super::Foundation::PWSTR) -> *mut _HRESOURCE;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_RESOURCE_NAME_DEPENDENCY_EX = unsafe extern "system" fn(lpszresourcename: super::super::Foundation::PWSTR, lpszresourcetype: super::super::Foundation::PWSTR, dwdesiredaccess: u32) -> *mut _HRESOURCE;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_GET_SZ_PROPERTY = unsafe extern "system" fn(ppszoutvalue: *mut super::super::Foundation::PWSTR, pvaluestruct: *const CLUSPROP_SZ, pszoldvalue: super::super::Foundation::PWSTR, pppropertylist: *mut *mut u8, pcbpropertylistsize: *mut u32) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_GET_SZ_VALUE = unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_IS_PATH_VALID = unsafe extern "system" fn(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_IS_RESOURCE_CLASS_EQUAL = unsafe extern "system" fn(prci: *mut CLUS_RESOURCE_CLASS_INFO, hresource: *mut _HRESOURCE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_PROPERTY_LIST_FROM_PARAMETER_BLOCK = unsafe extern "system" fn(ppropertytable: *const RESUTIL_PROPERTY_ITEM, poutpropertylist: *mut ::core::ffi::c_void, pcboutpropertylistsize: *mut u32, pinparams: *const u8, pcbbytesreturned: *mut u32, pcbrequired: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_REMOVE_RESOURCE_SERVICE_ENVIRONMENT = unsafe extern "system" fn(pszservicename: super::super::Foundation::PWSTR, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_RESOURCES_EQUAL = unsafe extern "system" fn(hself: *mut _HRESOURCE, hresource: *mut _HRESOURCE) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_RESOURCE_TYPES_EQUAL = unsafe extern "system" fn(lpszresourcetypename: super::super::Foundation::PWSTR, hresource: *mut _HRESOURCE) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_SET_BINARY_VALUE = unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, pbnewvalue: *const u8, cbnewvaluesize: u32, ppboutvalue: *mut *mut u8, pcboutvaluesize: *mut u32) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_SET_DWORD_VALUE = unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, dwnewvalue: u32, pdwoutvalue: *mut u32) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_SET_EXPAND_SZ_VALUE = unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, psznewvalue: super::super::Foundation::PWSTR, ppszoutstring: *mut super::super::Foundation::PWSTR) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_SET_MULTI_SZ_VALUE = unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, psznewvalue: super::super::Foundation::PWSTR, cbnewvaluesize: u32, ppszoutvalue: *mut super::super::Foundation::PWSTR, pcboutvaluesize: *mut u32) -> u32;
#[cfg(feature = "Win32_System_Registry")]
pub type PRESUTIL_SET_PRIVATE_PROPERTY_LIST = unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_SET_PROPERTY_PARAMETER_BLOCK = unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, pinparams: *const u8, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, poutparams: *mut u8) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_SET_PROPERTY_PARAMETER_BLOCK_EX = unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, pinparams: *const u8, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, bforcewrite: super::super::Foundation::BOOL, poutparams: *mut u8) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_SET_PROPERTY_TABLE = unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, ballowunknownproperties: super::super::Foundation::BOOL, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, poutparams: *mut u8) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_SET_PROPERTY_TABLE_EX = unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, ballowunknownproperties: super::super::Foundation::BOOL, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, bforcewrite: super::super::Foundation::BOOL, poutparams: *mut u8) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_SET_QWORD_VALUE = unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, qwnewvalue: u64, pqwoutvalue: *mut u64) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_SET_RESOURCE_SERVICE_ENVIRONMENT = unsafe extern "system" fn(pszservicename: super::super::Foundation::PWSTR, hresource: *mut _HRESOURCE, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type PRESUTIL_SET_RESOURCE_SERVICE_START_PARAMETERS = unsafe extern "system" fn(pszservicename: super::super::Foundation::PWSTR, schscmhandle: super::super::Security::SC_HANDLE, phservice: *mut isize, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type PRESUTIL_SET_RESOURCE_SERVICE_START_PARAMETERS_EX = unsafe extern "system" fn(pszservicename: super::super::Foundation::PWSTR, schscmhandle: super::super::Security::SC_HANDLE, phservice: *mut isize, dwdesiredaccess: u32, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_SET_SZ_VALUE = unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, pszvaluename: super::super::Foundation::PWSTR, psznewvalue: super::super::Foundation::PWSTR, ppszoutstring: *mut super::super::Foundation::PWSTR) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PRESUTIL_SET_UNKNOWN_PROPERTIES = unsafe extern "system" fn(hkeyclusterkey: super::super::System::Registry::HKEY, ppropertytable: *const RESUTIL_PROPERTY_ITEM, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_START_RESOURCE_SERVICE = unsafe extern "system" fn(pszservicename: super::super::Foundation::PWSTR, phservicehandle: *mut isize) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_STOP_RESOURCE_SERVICE = unsafe extern "system" fn(pszservicename: super::super::Foundation::PWSTR) -> u32;
#[cfg(feature = "Win32_Security")]
pub type PRESUTIL_STOP_SERVICE = unsafe extern "system" fn(hservicehandle: super::super::Security::SC_HANDLE) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_TERMINATE_SERVICE_PROCESS_FROM_RES_DLL = unsafe extern "system" fn(dwservicepid: u32, boffline: super::super::Foundation::BOOL, pdwresourcestate: *mut u32, pfnlogevent: PLOG_EVENT_ROUTINE, hresourcehandle: isize) -> u32;
pub type PRESUTIL_VERIFY_PRIVATE_PROPERTY_LIST = unsafe extern "system" fn(pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_VERIFY_PROPERTY_TABLE = unsafe extern "system" fn(ppropertytable: *const RESUTIL_PROPERTY_ITEM, reserved: *mut ::core::ffi::c_void, ballowunknownproperties: super::super::Foundation::BOOL, pinpropertylist: *const ::core::ffi::c_void, cbinpropertylistsize: u32, poutparams: *mut u8) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PRESUTIL_VERIFY_RESOURCE_SERVICE = unsafe extern "system" fn(pszservicename: super::super::Foundation::PWSTR) -> u32;
#[cfg(feature = "Win32_Security")]
pub type PRESUTIL_VERIFY_SERVICE = unsafe extern "system" fn(hservicehandle: super::super::Security::SC_HANDLE) -> u32;
pub type PRES_UTIL_VERIFY_SHUTDOWN_SAFE = unsafe extern "system" fn(flags: u32, reason: u32, presult: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PSET_INTERNAL_STATE = unsafe extern "system" fn(param0: isize, statetype: CLUSTER_RESOURCE_APPLICATION_STATE, active: super::super::Foundation::BOOL) -> u32;
pub type PSET_RESOURCE_INMEMORY_NODELOCAL_PROPERTIES_ROUTINE = unsafe extern "system" fn(resourcehandle: isize, propertylistbuffer: *const u8, propertylistbuffersize: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PSET_RESOURCE_LOCKED_MODE_EX_ROUTINE = unsafe extern "system" fn(resourcehandle: isize, lockedmodeenabled: super::super::Foundation::BOOL, lockedmodereason: u32, lockedmodeflags: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PSET_RESOURCE_LOCKED_MODE_ROUTINE = unsafe extern "system" fn(resourcehandle: isize, lockedmodeenabled: super::super::Foundation::BOOL, lockedmodereason: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PSET_RESOURCE_STATUS_ROUTINE = unsafe extern "system" fn(resourcehandle: isize, resourcestatus: *mut RESOURCE_STATUS) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PSET_RESOURCE_STATUS_ROUTINE_EX = unsafe extern "system" fn(resourcehandle: isize, resourcestatus: *mut RESOURCE_STATUS_EX) -> u32;
pub type PSIGNAL_FAILURE_ROUTINE = unsafe extern "system" fn(resourcehandle: isize, failuretype: FAILURE_TYPE, applicationspecificerrorcode: u32) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PSTARTUP_EX_ROUTINE = unsafe extern "system" fn(resourcetype: super::super::Foundation::PWSTR, minversionsupported: u32, maxversionsupported: u32, monitorcallbackfunctions: *mut CLRES_CALLBACK_FUNCTION_TABLE, resourcedllinterfacefunctions: *mut *mut CLRES_FUNCTION_TABLE) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PSTARTUP_ROUTINE = unsafe extern "system" fn(resourcetype: super::super::Foundation::PWSTR, minversionsupported: u32, maxversionsupported: u32, setresourcestatus: PSET_RESOURCE_STATUS_ROUTINE, logevent: PLOG_EVENT_ROUTINE, functiontable: *mut *mut CLRES_FUNCTION_TABLE) -> u32;
pub type PTERMINATE_ROUTINE = unsafe extern "system" fn(resource: *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
pub type PWORKER_START_ROUTINE = unsafe extern "system" fn(pworker: *mut CLUS_WORKER, lpthreadparameter: *mut ::core::ffi::c_void) -> u32;
#[repr(C)]
pub struct PaxosTagCStruct {
    pub __padding__PaxosTagVtable: u64,
    pub __padding__NextEpochVtable: u64,
    pub __padding__NextEpoch_DateTimeVtable: u64,
    pub NextEpoch_DateTime_ticks: u64,
    pub NextEpoch_Value: i32,
    pub __padding__BoundryNextEpoch: u32,
    pub __padding__EpochVtable: u64,
    pub __padding__Epoch_DateTimeVtable: u64,
    pub Epoch_DateTime_ticks: u64,
    pub Epoch_Value: i32,
    pub __padding__BoundryEpoch: u32,
    pub Sequence: i32,
    pub __padding__BoundrySequence: u32,
}
impl ::core::marker::Copy for PaxosTagCStruct {}
impl ::core::clone::Clone for PaxosTagCStruct {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ResdllContextOperationTypeFailback: i32 = 0i32;
pub const ResdllContextOperationTypeDrain: i32 = 1i32;
pub const ResdllContextOperationTypeDrainFailure: i32 = 2i32;
pub const ResdllContextOperationTypeEmbeddedFailure: i32 = 3i32;
pub const ResdllContextOperationTypePreemption: i32 = 4i32;
pub const ResdllContextOperationTypeNetworkDisconnect: i32 = 5i32;
pub const ResdllContextOperationTypeNetworkDisconnectMoveRetry: i32 = 6i32;
pub const ResourceExitStateContinue: i32 = 0i32;
pub const ResourceExitStateTerminate: i32 = 1i32;
pub const ResourceExitStateMax: i32 = 2i32;
#[repr(C)]
pub struct RESOURCE_FAILURE_INFO {
    pub dwRestartAttemptsRemaining: u32,
    pub dwRestartPeriodRemaining: u32,
}
impl ::core::marker::Copy for RESOURCE_FAILURE_INFO {}
impl ::core::clone::Clone for RESOURCE_FAILURE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RESOURCE_FAILURE_INFO_BUFFER {
    pub dwVersion: u32,
    pub Info: RESOURCE_FAILURE_INFO,
}
impl ::core::marker::Copy for RESOURCE_FAILURE_INFO_BUFFER {}
impl ::core::clone::Clone for RESOURCE_FAILURE_INFO_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RESOURCE_FAILURE_INFO_VERSION_1: u32 = 1u32;
pub const RmonInitializing: i32 = 0i32;
pub const RmonIdle: i32 = 1i32;
pub const RmonStartingResource: i32 = 2i32;
pub const RmonInitializingResource: i32 = 3i32;
pub const RmonOnlineResource: i32 = 4i32;
pub const RmonOfflineResource: i32 = 5i32;
pub const RmonShutdownResource: i32 = 6i32;
pub const RmonDeletingResource: i32 = 7i32;
pub const RmonIsAlivePoll: i32 = 8i32;
pub const RmonLooksAlivePoll: i32 = 9i32;
pub const RmonArbitrateResource: i32 = 10i32;
pub const RmonReleaseResource: i32 = 11i32;
pub const RmonResourceControl: i32 = 12i32;
pub const RmonResourceTypeControl: i32 = 13i32;
pub const RmonTerminateResource: i32 = 14i32;
pub const RmonDeadlocked: i32 = 15i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RESOURCE_STATUS {
    pub ResourceState: CLUSTER_RESOURCE_STATE,
    pub CheckPoint: u32,
    pub WaitHint: u32,
    pub EventHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESOURCE_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESOURCE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RESOURCE_STATUS_EX {
    pub ResourceState: CLUSTER_RESOURCE_STATE,
    pub CheckPoint: u32,
    pub EventHandle: super::super::Foundation::HANDLE,
    pub ApplicationSpecificErrorCode: u32,
    pub Flags: u32,
    pub WaitHint: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESOURCE_STATUS_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESOURCE_STATUS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RESOURCE_TERMINAL_FAILURE_INFO_BUFFER {
    pub isTerminalFailure: super::super::Foundation::BOOL,
    pub restartPeriodRemaining: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESOURCE_TERMINAL_FAILURE_INFO_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESOURCE_TERMINAL_FAILURE_INFO_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RESTYPE_MONITOR_SHUTTING_DOWN_CLUSSVC_CRASH: u32 = 2u32;
pub const RESTYPE_MONITOR_SHUTTING_DOWN_NODE_STOP: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RESUTIL_FILETIME_DATA {
    pub Default: super::super::Foundation::FILETIME,
    pub Minimum: super::super::Foundation::FILETIME,
    pub Maximum: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESUTIL_FILETIME_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESUTIL_FILETIME_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RESUTIL_LARGEINT_DATA {
    pub Default: i64,
    pub Minimum: i64,
    pub Maximum: i64,
}
impl ::core::marker::Copy for RESUTIL_LARGEINT_DATA {}
impl ::core::clone::Clone for RESUTIL_LARGEINT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RESUTIL_PROPERTY_ITEM {
    pub Name: super::super::Foundation::PWSTR,
    pub KeyName: super::super::Foundation::PWSTR,
    pub Format: u32,
    pub Anonymous: RESUTIL_PROPERTY_ITEM_0,
    pub Minimum: u32,
    pub Maximum: u32,
    pub Flags: u32,
    pub Offset: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESUTIL_PROPERTY_ITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESUTIL_PROPERTY_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union RESUTIL_PROPERTY_ITEM_0 {
    pub DefaultPtr: usize,
    pub Default: u32,
    pub lpDefault: *mut ::core::ffi::c_void,
    pub LargeIntData: *mut RESUTIL_LARGEINT_DATA,
    pub ULargeIntData: *mut RESUTIL_ULARGEINT_DATA,
    pub FileTimeData: *mut RESUTIL_FILETIME_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESUTIL_PROPERTY_ITEM_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESUTIL_PROPERTY_ITEM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RESUTIL_PROPITEM_IN_MEMORY: u32 = 8u32;
pub const RESUTIL_PROPITEM_READ_ONLY: u32 = 1u32;
pub const RESUTIL_PROPITEM_REQUIRED: u32 = 2u32;
pub const RESUTIL_PROPITEM_SIGNED: u32 = 4u32;
#[repr(C)]
pub struct RESUTIL_ULARGEINT_DATA {
    pub Default: u64,
    pub Minimum: u64,
    pub Maximum: u64,
}
impl ::core::marker::Copy for RESUTIL_ULARGEINT_DATA {}
impl ::core::clone::Clone for RESUTIL_ULARGEINT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RS3_UPGRADE_VERSION: u32 = 1u32;
pub const RS4_UPGRADE_VERSION: u32 = 2u32;
pub const RS5_UPGRADE_VERSION: u32 = 3u32;
pub const RedirectedIOReasonBitLockerInitializing: u64 = 16u64;
pub const RedirectedIOReasonFileSystemTiering: u64 = 8u64;
pub const RedirectedIOReasonMax: u64 = 9223372036854775808u64;
pub const RedirectedIOReasonReFs: u64 = 32u64;
pub const RedirectedIOReasonUnsafeFileSystemFilter: u64 = 2u64;
pub const RedirectedIOReasonUnsafeVolumeFilter: u64 = 4u64;
pub const RedirectedIOReasonUserRequest: u64 = 1u64;
#[repr(C)]
pub struct ResourceUtilizationInfoElement {
    pub PhysicalNumaId: u64,
    pub CurrentMemory: u64,
}
impl ::core::marker::Copy for ResourceUtilizationInfoElement {}
impl ::core::clone::Clone for ResourceUtilizationInfoElement {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SET_APPINSTANCE_CSV_FLAGS_VALID_ONLY_IF_CSV_COORDINATOR: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub type SET_APP_INSTANCE_CSV_FLAGS = unsafe extern "system" fn(processhandle: super::super::Foundation::HANDLE, mask: u32, flags: u32) -> u32;
pub const SrDiskReplicationEligibleNone: i32 = 0i32;
pub const SrDiskReplicationEligibleYes: i32 = 1i32;
pub const SrDiskReplicationEligibleOffline: i32 = 2i32;
pub const SrDiskReplicationEligibleNotGpt: i32 = 3i32;
pub const SrDiskReplicationEligiblePartitionLayoutMismatch: i32 = 4i32;
pub const SrDiskReplicationEligibleInsufficientFreeSpace: i32 = 5i32;
pub const SrDiskReplicationEligibleNotInSameSite: i32 = 6i32;
pub const SrDiskReplicationEligibleInSameSite: i32 = 7i32;
pub const SrDiskReplicationEligibleFileSystemNotSupported: i32 = 8i32;
pub const SrDiskReplicationEligibleAlreadyInReplication: i32 = 9i32;
pub const SrDiskReplicationEligibleSameAsSpecifiedDisk: i32 = 10i32;
pub const SrDiskReplicationEligibleOther: i32 = 9999i32;
pub const SrReplicatedDiskTypeNone: i32 = 0i32;
pub const SrReplicatedDiskTypeSource: i32 = 1i32;
pub const SrReplicatedDiskTypeLogSource: i32 = 2i32;
pub const SrReplicatedDiskTypeDestination: i32 = 3i32;
pub const SrReplicatedDiskTypeLogDestination: i32 = 4i32;
pub const SrReplicatedDiskTypeNotInParthership: i32 = 5i32;
pub const SrReplicatedDiskTypeLogNotInParthership: i32 = 6i32;
pub const SrReplicatedDiskTypeOther: i32 = 7i32;
pub const SR_REPLICATED_PARTITION_DISALLOW_MULTINODE_IO: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP {
    pub ReplicationGroupName: [u16; 260],
    pub Description: [u16; 260],
    pub LogPath: [u16; 260],
    pub MaxLogSizeInBytes: u64,
    pub LogType: u16,
    pub ReplicationMode: u32,
    pub MinimumPartnersInSync: u32,
    pub EnableWriteConsistency: super::super::Foundation::BOOLEAN,
    pub EnableEncryption: super::super::Foundation::BOOLEAN,
    pub CertificateThumbprint: [u16; 260],
    pub VolumeNameCount: u32,
    pub VolumeNames: [u16; 260],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT {
    pub Result: u32,
    pub ErrorString: [u16; 260],
}
impl ::core::marker::Copy for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT {}
impl ::core::clone::Clone for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SR_RESOURCE_TYPE_DISK_INFO {
    pub Reason: SR_DISK_REPLICATION_ELIGIBLE,
    pub DiskGuid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for SR_RESOURCE_TYPE_DISK_INFO {}
impl ::core::clone::Clone for SR_RESOURCE_TYPE_DISK_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT {
    pub Count: u16,
    pub DiskInfo: [SR_RESOURCE_TYPE_DISK_INFO; 1],
}
impl ::core::marker::Copy for SR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT {}
impl ::core::clone::Clone for SR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS {
    pub DataDiskGuid: ::windows_sys::core::GUID,
    pub IncludeOfflineDisks: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SR_RESOURCE_TYPE_QUERY_ELIGIBLE_SOURCE_DATADISKS {
    pub DataDiskGuid: ::windows_sys::core::GUID,
    pub IncludeAvailableStoargeDisks: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_SOURCE_DATADISKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_SOURCE_DATADISKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SR_RESOURCE_TYPE_QUERY_ELIGIBLE_TARGET_DATADISKS {
    pub SourceDataDiskGuid: ::windows_sys::core::GUID,
    pub TargetReplicationGroupGuid: ::windows_sys::core::GUID,
    pub SkipConnectivityCheck: super::super::Foundation::BOOLEAN,
    pub IncludeOfflineDisks: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_TARGET_DATADISKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SR_RESOURCE_TYPE_QUERY_ELIGIBLE_TARGET_DATADISKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SR_RESOURCE_TYPE_REPLICATED_DISK {
    pub Type: SR_REPLICATED_DISK_TYPE,
    pub ClusterDiskResourceGuid: ::windows_sys::core::GUID,
    pub ReplicationGroupId: ::windows_sys::core::GUID,
    pub ReplicationGroupName: [u16; 260],
}
impl ::core::marker::Copy for SR_RESOURCE_TYPE_REPLICATED_DISK {}
impl ::core::clone::Clone for SR_RESOURCE_TYPE_REPLICATED_DISK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT {
    pub Count: u16,
    pub ReplicatedDisks: [SR_RESOURCE_TYPE_REPLICATED_DISK; 1],
}
impl ::core::marker::Copy for SR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT {}
impl ::core::clone::Clone for SR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY {
    pub Count: u32,
    pub PartitionArray: [SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO; 1],
}
impl ::core::marker::Copy for SR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY {}
impl ::core::clone::Clone for SR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO {
    pub PartitionOffset: u64,
    pub Capabilities: u32,
}
impl ::core::marker::Copy for SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO {}
impl ::core::clone::Clone for SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const VmResdllContextTurnOff: i32 = 0i32;
pub const VmResdllContextSave: i32 = 1i32;
pub const VmResdllContextShutdown: i32 = 2i32;
pub const VmResdllContextShutdownForce: i32 = 3i32;
pub const VmResdllContextLiveMigration: i32 = 4i32;
pub const VolumeRedirectedIOReasonMax: u64 = 9223372036854775808u64;
pub const VolumeRedirectedIOReasonNoDiskConnectivity: u64 = 1u64;
pub const VolumeRedirectedIOReasonStorageSpaceNotAttached: u64 = 2u64;
pub const VolumeRedirectedIOReasonVolumeReplicationEnabled: u64 = 4u64;
pub const WS2016_RTM_UPGRADE_VERSION: u32 = 8u32;
pub const WS2016_TP4_UPGRADE_VERSION: u32 = 6u32;
pub const WS2016_TP5_UPGRADE_VERSION: u32 = 7u32;
#[repr(C)]
pub struct WitnessTagHelper {
    pub Version: i32,
    pub paxosToValidate: PaxosTagCStruct,
}
impl ::core::marker::Copy for WitnessTagHelper {}
impl ::core::clone::Clone for WitnessTagHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WitnessTagUpdateHelper {
    pub Version: i32,
    pub paxosToSet: PaxosTagCStruct,
    pub paxosToValidate: PaxosTagCStruct,
}
impl ::core::marker::Copy for WitnessTagUpdateHelper {}
impl ::core::clone::Clone for WitnessTagUpdateHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct _HCHANGE(pub u8);
#[repr(C)]
pub struct _HCLUSCRYPTPROVIDER(pub u8);
#[repr(C)]
pub struct _HCLUSENUM(pub u8);
#[repr(C)]
pub struct _HCLUSENUMEX(pub u8);
#[repr(C)]
pub struct _HCLUSTER(pub u8);
#[repr(C)]
pub struct _HGROUP(pub u8);
#[repr(C)]
pub struct _HGROUPENUM(pub u8);
#[repr(C)]
pub struct _HGROUPENUMEX(pub u8);
#[repr(C)]
pub struct _HGROUPSET(pub u8);
#[repr(C)]
pub struct _HGROUPSETENUM(pub u8);
#[repr(C)]
pub struct _HNETINTERFACE(pub u8);
#[repr(C)]
pub struct _HNETINTERFACEENUM(pub u8);
#[repr(C)]
pub struct _HNETWORK(pub u8);
#[repr(C)]
pub struct _HNETWORKENUM(pub u8);
#[repr(C)]
pub struct _HNODE(pub u8);
#[repr(C)]
pub struct _HNODEENUM(pub u8);
#[repr(C)]
pub struct _HNODEENUMEX(pub u8);
#[repr(C)]
pub struct _HREGBATCH(pub u8);
#[repr(C)]
pub struct _HREGBATCHNOTIFICATION(pub u8);
#[repr(C)]
pub struct _HREGBATCHPORT(pub u8);
#[repr(C)]
pub struct _HREGREADBATCH(pub u8);
#[repr(C)]
pub struct _HREGREADBATCHREPLY(pub u8);
#[repr(C)]
pub struct _HRESENUM(pub u8);
#[repr(C)]
pub struct _HRESENUMEX(pub u8);
#[repr(C)]
pub struct _HRESOURCE(pub u8);
#[repr(C)]
pub struct _HRESTYPEENUM(pub u8);
