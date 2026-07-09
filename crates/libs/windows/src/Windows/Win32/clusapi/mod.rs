#[inline]
pub unsafe fn AddClusterGroupDependency(hdependentgroup: *const _HGROUP, hprovidergroup: *const _HGROUP) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn AddClusterGroupDependency(hdependentgroup : *const _HGROUP, hprovidergroup : *const _HGROUP) -> u32);
    unsafe { AddClusterGroupDependency(hdependentgroup, hprovidergroup) }
}
#[inline]
pub unsafe fn AddClusterGroupDependencyEx<P2>(hdependentgroup: *const _HGROUP, hprovidergroup: *const _HGROUP, lpszreason: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn AddClusterGroupDependencyEx(hdependentgroup : *const _HGROUP, hprovidergroup : *const _HGROUP, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { AddClusterGroupDependencyEx(hdependentgroup, hprovidergroup, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn AddClusterGroupSetDependency(hdependentgroupset: *const _HGROUPSET, hprovidergroupset: *const _HGROUPSET) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn AddClusterGroupSetDependency(hdependentgroupset : *const _HGROUPSET, hprovidergroupset : *const _HGROUPSET) -> u32);
    unsafe { AddClusterGroupSetDependency(hdependentgroupset, hprovidergroupset) }
}
#[inline]
pub unsafe fn AddClusterGroupSetDependencyEx<P2>(hdependentgroupset: *const _HGROUPSET, hprovidergroupset: *const _HGROUPSET, lpszreason: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn AddClusterGroupSetDependencyEx(hdependentgroupset : *const _HGROUPSET, hprovidergroupset : *const _HGROUPSET, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { AddClusterGroupSetDependencyEx(hdependentgroupset, hprovidergroupset, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn AddClusterGroupToGroupSetDependency(hdependentgroup: *const _HGROUP, hprovidergroupset: *const _HGROUPSET) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn AddClusterGroupToGroupSetDependency(hdependentgroup : *const _HGROUP, hprovidergroupset : *const _HGROUPSET) -> u32);
    unsafe { AddClusterGroupToGroupSetDependency(hdependentgroup, hprovidergroupset) }
}
#[inline]
pub unsafe fn AddClusterGroupToGroupSetDependencyEx<P2>(hdependentgroup: *const _HGROUP, hprovidergroupset: *const _HGROUPSET, lpszreason: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn AddClusterGroupToGroupSetDependencyEx(hdependentgroup : *const _HGROUP, hprovidergroupset : *const _HGROUPSET, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { AddClusterGroupToGroupSetDependencyEx(hdependentgroup, hprovidergroupset, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn AddClusterNode<P1>(hcluster: *const _HCLUSTER, lpsznodename: P1, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: Option<*const core::ffi::c_void>) -> HNODE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn AddClusterNode(hcluster : *const _HCLUSTER, lpsznodename : windows_core::PCWSTR, pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg : *const core::ffi::c_void) -> HNODE);
    unsafe { AddClusterNode(hcluster, lpsznodename.param().abi(), pfnprogresscallback, pvcallbackarg.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn AddClusterNodeEx<P1>(hcluster: *const _HCLUSTER, lpsznodename: P1, dwflags: u32, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: Option<*const core::ffi::c_void>) -> HNODE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn AddClusterNodeEx(hcluster : *const _HCLUSTER, lpsznodename : windows_core::PCWSTR, dwflags : u32, pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg : *const core::ffi::c_void) -> HNODE);
    unsafe { AddClusterNodeEx(hcluster, lpsznodename.param().abi(), dwflags, pfnprogresscallback, pvcallbackarg.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn AddClusterResourceDependency(hresource: *const _HRESOURCE, hdependson: *const _HRESOURCE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn AddClusterResourceDependency(hresource : *const _HRESOURCE, hdependson : *const _HRESOURCE) -> u32);
    unsafe { AddClusterResourceDependency(hresource, hdependson) }
}
#[inline]
pub unsafe fn AddClusterResourceDependencyEx<P2>(hresource: *const _HRESOURCE, hdependson: *const _HRESOURCE, lpszreason: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn AddClusterResourceDependencyEx(hresource : *const _HRESOURCE, hdependson : *const _HRESOURCE, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { AddClusterResourceDependencyEx(hresource, hdependson, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn AddClusterResourceNode(hresource: *const _HRESOURCE, hnode: *const _HNODE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn AddClusterResourceNode(hresource : *const _HRESOURCE, hnode : *const _HNODE) -> u32);
    unsafe { AddClusterResourceNode(hresource, hnode) }
}
#[inline]
pub unsafe fn AddClusterResourceNodeEx<P2>(hresource: *const _HRESOURCE, hnode: *const _HNODE, lpszreason: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn AddClusterResourceNodeEx(hresource : *const _HRESOURCE, hnode : *const _HNODE, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { AddClusterResourceNodeEx(hresource, hnode, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn AddClusterStorageNode<P1, P4, P5>(hcluster: *const _HCLUSTER, lpsznodename: P1, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: Option<*const core::ffi::c_void>, lpszclusterstoragenodedescription: P4, lpszclusterstoragenodelocation: P5) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn AddClusterStorageNode(hcluster : *const _HCLUSTER, lpsznodename : windows_core::PCWSTR, pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg : *const core::ffi::c_void, lpszclusterstoragenodedescription : windows_core::PCWSTR, lpszclusterstoragenodelocation : windows_core::PCWSTR) -> u32);
    unsafe { AddClusterStorageNode(hcluster, lpsznodename.param().abi(), pfnprogresscallback, pvcallbackarg.unwrap_or(core::mem::zeroed()) as _, lpszclusterstoragenodedescription.param().abi(), lpszclusterstoragenodelocation.param().abi()) }
}
#[inline]
pub unsafe fn AddCrossClusterGroupSetDependency<P1, P2>(hdependentgroupset: *const _HGROUPSET, lpremoteclustername: P1, lpremotegroupsetname: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn AddCrossClusterGroupSetDependency(hdependentgroupset : *const _HGROUPSET, lpremoteclustername : windows_core::PCWSTR, lpremotegroupsetname : windows_core::PCWSTR) -> u32);
    unsafe { AddCrossClusterGroupSetDependency(hdependentgroupset, lpremoteclustername.param().abi(), lpremotegroupsetname.param().abi()) }
}
#[inline]
pub unsafe fn AddResourceToClusterSharedVolumes(hresource: *const _HRESOURCE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn AddResourceToClusterSharedVolumes(hresource : *const _HRESOURCE) -> u32);
    unsafe { AddResourceToClusterSharedVolumes(hresource) }
}
#[inline]
pub unsafe fn BackupClusterDatabase<P1>(hcluster: *const _HCLUSTER, lpszpathname: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn BackupClusterDatabase(hcluster : *const _HCLUSTER, lpszpathname : windows_core::PCWSTR) -> u32);
    unsafe { BackupClusterDatabase(hcluster, lpszpathname.param().abi()) }
}
#[inline]
pub unsafe fn CanResourceBeDependent(hresource: *const _HRESOURCE, hresourcedependent: *const _HRESOURCE) -> windows_core::BOOL {
    windows_core::link!("clusapi.dll" "system" fn CanResourceBeDependent(hresource : *const _HRESOURCE, hresourcedependent : *const _HRESOURCE) -> windows_core::BOOL);
    unsafe { CanResourceBeDependent(hresource, hresourcedependent) }
}
#[inline]
pub unsafe fn CancelClusterGroupOperation(hgroup: *const _HGROUP, dwcancelflags_reserved: u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn CancelClusterGroupOperation(hgroup : *const _HGROUP, dwcancelflags_reserved : u32) -> u32);
    unsafe { CancelClusterGroupOperation(hgroup, dwcancelflags_reserved) }
}
#[inline]
pub unsafe fn ChangeClusterResourceGroup(hresource: *const _HRESOURCE, hgroup: *const _HGROUP) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ChangeClusterResourceGroup(hresource : *const _HRESOURCE, hgroup : *const _HGROUP) -> u32);
    unsafe { ChangeClusterResourceGroup(hresource, hgroup) }
}
#[inline]
pub unsafe fn ChangeClusterResourceGroupEx(hresource: *const _HRESOURCE, hgroup: *const _HGROUP, flags: u64) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ChangeClusterResourceGroupEx(hresource : *const _HRESOURCE, hgroup : *const _HGROUP, flags : u64) -> u32);
    unsafe { ChangeClusterResourceGroupEx(hresource, hgroup, flags) }
}
#[inline]
pub unsafe fn ChangeClusterResourceGroupEx2<P3>(hresource: *const _HRESOURCE, hgroup: *const _HGROUP, flags: u64, lpszreason: P3) -> u32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ChangeClusterResourceGroupEx2(hresource : *const _HRESOURCE, hgroup : *const _HGROUP, flags : u64, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ChangeClusterResourceGroupEx2(hresource, hgroup, flags, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn CloseCluster(hcluster: *const _HCLUSTER) -> windows_core::BOOL {
    windows_core::link!("clusapi.dll" "system" fn CloseCluster(hcluster : *const _HCLUSTER) -> windows_core::BOOL);
    unsafe { CloseCluster(hcluster) }
}
#[inline]
pub unsafe fn CloseClusterGroup(hgroup: *const _HGROUP) -> windows_core::BOOL {
    windows_core::link!("clusapi.dll" "system" fn CloseClusterGroup(hgroup : *const _HGROUP) -> windows_core::BOOL);
    unsafe { CloseClusterGroup(hgroup) }
}
#[inline]
pub unsafe fn CloseClusterGroupSet(hgroupset: *const _HGROUPSET) -> windows_core::BOOL {
    windows_core::link!("clusapi.dll" "system" fn CloseClusterGroupSet(hgroupset : *const _HGROUPSET) -> windows_core::BOOL);
    unsafe { CloseClusterGroupSet(hgroupset) }
}
#[inline]
pub unsafe fn CloseClusterNetInterface(hnetinterface: *const _HNETINTERFACE) -> windows_core::BOOL {
    windows_core::link!("clusapi.dll" "system" fn CloseClusterNetInterface(hnetinterface : *const _HNETINTERFACE) -> windows_core::BOOL);
    unsafe { CloseClusterNetInterface(hnetinterface) }
}
#[inline]
pub unsafe fn CloseClusterNetwork(hnetwork: *const _HNETWORK) -> windows_core::BOOL {
    windows_core::link!("clusapi.dll" "system" fn CloseClusterNetwork(hnetwork : *const _HNETWORK) -> windows_core::BOOL);
    unsafe { CloseClusterNetwork(hnetwork) }
}
#[inline]
pub unsafe fn CloseClusterNode(hnode: *const _HNODE) -> windows_core::BOOL {
    windows_core::link!("clusapi.dll" "system" fn CloseClusterNode(hnode : *const _HNODE) -> windows_core::BOOL);
    unsafe { CloseClusterNode(hnode) }
}
#[inline]
pub unsafe fn CloseClusterNotifyPort(hchange: *const _HCHANGE) -> windows_core::BOOL {
    windows_core::link!("clusapi.dll" "system" fn CloseClusterNotifyPort(hchange : *const _HCHANGE) -> windows_core::BOOL);
    unsafe { CloseClusterNotifyPort(hchange) }
}
#[inline]
pub unsafe fn CloseClusterResource(hresource: *const _HRESOURCE) -> windows_core::BOOL {
    windows_core::link!("clusapi.dll" "system" fn CloseClusterResource(hresource : *const _HRESOURCE) -> windows_core::BOOL);
    unsafe { CloseClusterResource(hresource) }
}
#[inline]
pub unsafe fn ClusapiSetReasonHandler(lphandler: *const CLUSAPI_REASON_HANDLER) -> PCLUSAPI_REASON_HANDLER {
    windows_core::link!("clusapi.dll" "system" fn ClusapiSetReasonHandler(lphandler : *const CLUSAPI_REASON_HANDLER) -> PCLUSAPI_REASON_HANDLER);
    unsafe { ClusapiSetReasonHandler(lphandler) }
}
#[inline]
pub unsafe fn ClusterAddGroupToAffinityRule<P1>(hcluster: *mut _HCLUSTER, rulename: P1, hgroup: *mut _HGROUP) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterAddGroupToAffinityRule(hcluster : *mut _HCLUSTER, rulename : windows_core::PCWSTR, hgroup : *mut _HGROUP) -> u32);
    unsafe { ClusterAddGroupToAffinityRule(hcluster as _, rulename.param().abi(), hgroup as _) }
}
#[inline]
pub unsafe fn ClusterAddGroupToGroupSet(hgroupset: *const _HGROUPSET, hgroup: *const _HGROUP) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterAddGroupToGroupSet(hgroupset : *const _HGROUPSET, hgroup : *const _HGROUP) -> u32);
    unsafe { ClusterAddGroupToGroupSet(hgroupset, hgroup) }
}
#[inline]
pub unsafe fn ClusterAddGroupToGroupSetWithDomains(hgroupset: *const _HGROUPSET, hgroup: *const _HGROUP, faultdomain: u32, updatedomain: u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterAddGroupToGroupSetWithDomains(hgroupset : *const _HGROUPSET, hgroup : *const _HGROUP, faultdomain : u32, updatedomain : u32) -> u32);
    unsafe { ClusterAddGroupToGroupSetWithDomains(hgroupset, hgroup, faultdomain, updatedomain) }
}
#[inline]
pub unsafe fn ClusterAddGroupToGroupSetWithDomainsEx<P4>(hgroupset: *const _HGROUPSET, hgroup: *const _HGROUP, faultdomain: u32, updatedomain: u32, lpszreason: P4) -> u32
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterAddGroupToGroupSetWithDomainsEx(hgroupset : *const _HGROUPSET, hgroup : *const _HGROUP, faultdomain : u32, updatedomain : u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterAddGroupToGroupSetWithDomainsEx(hgroupset, hgroup, faultdomain, updatedomain, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn ClusterAffinityRuleControl<P1>(hcluster: *const _HCLUSTER, affinityrulename: P1, hhostnode: Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, cbinbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, cboutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterAffinityRuleControl(hcluster : *const _HCLUSTER, affinityrulename : windows_core::PCWSTR, hhostnode : *const _HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, cbinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, cboutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    unsafe { ClusterAffinityRuleControl(hcluster, affinityrulename.param().abi(), hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, cbinbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, cboutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterCloseEnum(henum: *const _HCLUSENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterCloseEnum(henum : *const _HCLUSENUM) -> u32);
    unsafe { ClusterCloseEnum(henum) }
}
#[inline]
pub unsafe fn ClusterCloseEnumEx(hclusterenum: *const _HCLUSENUMEX) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterCloseEnumEx(hclusterenum : *const _HCLUSENUMEX) -> u32);
    unsafe { ClusterCloseEnumEx(hclusterenum) }
}
#[inline]
pub unsafe fn ClusterControl(hcluster: *const _HCLUSTER, hhostnode: Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterControl(hcluster : *const _HCLUSTER, hhostnode : *const _HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    unsafe { ClusterControl(hcluster, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterControlEx<P8>(hcluster: *const _HCLUSTER, hhostnode: Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P8) -> u32
where
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterControlEx(hcluster : *const _HCLUSTER, hhostnode : *const _HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterControlEx(hcluster, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn ClusterCreateAffinityRule<P1>(hcluster: *mut _HCLUSTER, rulename: P1, ruletype: CLUS_AFFINITY_RULE_TYPE) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterCreateAffinityRule(hcluster : *mut _HCLUSTER, rulename : windows_core::PCWSTR, ruletype : CLUS_AFFINITY_RULE_TYPE) -> u32);
    unsafe { ClusterCreateAffinityRule(hcluster as _, rulename.param().abi(), ruletype) }
}
#[inline]
pub unsafe fn ClusterEnum(henum: *const _HCLUSENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterEnum(henum : *const _HCLUSENUM, dwindex : u32, lpdwtype : *mut u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    unsafe { ClusterEnum(henum, dwindex, lpdwtype as _, core::mem::transmute(lpszname), lpcchname as _) }
}
#[inline]
pub unsafe fn ClusterEnumEx(hclusterenum: *const _HCLUSENUMEX, dwindex: u32, pitem: *mut CLUSTER_ENUM_ITEM, cbitem: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterEnumEx(hclusterenum : *const _HCLUSENUMEX, dwindex : u32, pitem : *mut CLUSTER_ENUM_ITEM, cbitem : *mut u32) -> u32);
    unsafe { ClusterEnumEx(hclusterenum, dwindex, pitem as _, cbitem as _) }
}
#[inline]
pub unsafe fn ClusterGetEnumCount(henum: *const _HCLUSENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterGetEnumCount(henum : *const _HCLUSENUM) -> u32);
    unsafe { ClusterGetEnumCount(henum) }
}
#[inline]
pub unsafe fn ClusterGetEnumCountEx(hclusterenum: *const _HCLUSENUMEX) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterGetEnumCountEx(hclusterenum : *const _HCLUSENUMEX) -> u32);
    unsafe { ClusterGetEnumCountEx(hclusterenum) }
}
#[inline]
pub unsafe fn ClusterGroupCloseEnum(hgroupenum: *const _HGROUPENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupCloseEnum(hgroupenum : *const _HGROUPENUM) -> u32);
    unsafe { ClusterGroupCloseEnum(hgroupenum) }
}
#[inline]
pub unsafe fn ClusterGroupCloseEnumEx(hgroupenumex: *const _HGROUPENUMEX) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupCloseEnumEx(hgroupenumex : *const _HGROUPENUMEX) -> u32);
    unsafe { ClusterGroupCloseEnumEx(hgroupenumex) }
}
#[inline]
pub unsafe fn ClusterGroupControl(hgroup: *const _HGROUP, hhostnode: Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupControl(hgroup : *const _HGROUP, hhostnode : *const _HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    unsafe { ClusterGroupControl(hgroup, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterGroupControlEx<P8>(hgroup: *const _HGROUP, hhostnode: Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P8) -> u32
where
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupControlEx(hgroup : *const _HGROUP, hhostnode : *const _HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterGroupControlEx(hgroup, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn ClusterGroupEnum(hgroupenum: *const _HGROUPENUM, dwindex: u32, lpdwtype: *mut u32, lpszresourcename: windows_core::PWSTR, lpcchname: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupEnum(hgroupenum : *const _HGROUPENUM, dwindex : u32, lpdwtype : *mut u32, lpszresourcename : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    unsafe { ClusterGroupEnum(hgroupenum, dwindex, lpdwtype as _, core::mem::transmute(lpszresourcename), lpcchname as _) }
}
#[inline]
pub unsafe fn ClusterGroupEnumEx(hgroupenumex: *const _HGROUPENUMEX, dwindex: u32, pitem: *mut CLUSTER_GROUP_ENUM_ITEM, cbitem: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupEnumEx(hgroupenumex : *const _HGROUPENUMEX, dwindex : u32, pitem : *mut CLUSTER_GROUP_ENUM_ITEM, cbitem : *mut u32) -> u32);
    unsafe { ClusterGroupEnumEx(hgroupenumex, dwindex, pitem as _, cbitem as _) }
}
#[inline]
pub unsafe fn ClusterGroupGetEnumCount(hgroupenum: *const _HGROUPENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupGetEnumCount(hgroupenum : *const _HGROUPENUM) -> u32);
    unsafe { ClusterGroupGetEnumCount(hgroupenum) }
}
#[inline]
pub unsafe fn ClusterGroupGetEnumCountEx(hgroupenumex: *const _HGROUPENUMEX) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupGetEnumCountEx(hgroupenumex : *const _HGROUPENUMEX) -> u32);
    unsafe { ClusterGroupGetEnumCountEx(hgroupenumex) }
}
#[inline]
pub unsafe fn ClusterGroupOpenEnum(hgroup: *const _HGROUP, dwtype: u32) -> HGROUPENUM {
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupOpenEnum(hgroup : *const _HGROUP, dwtype : u32) -> HGROUPENUM);
    unsafe { ClusterGroupOpenEnum(hgroup, dwtype) }
}
#[inline]
pub unsafe fn ClusterGroupOpenEnumEx<P1, P3>(hcluster: *const _HCLUSTER, lpszproperties: P1, cbproperties: u32, lpszroproperties: P3, cbroproperties: u32, dwflags: u32) -> HGROUPENUMEX
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupOpenEnumEx(hcluster : *const _HCLUSTER, lpszproperties : windows_core::PCWSTR, cbproperties : u32, lpszroproperties : windows_core::PCWSTR, cbroproperties : u32, dwflags : u32) -> HGROUPENUMEX);
    unsafe { ClusterGroupOpenEnumEx(hcluster, lpszproperties.param().abi(), cbproperties, lpszroproperties.param().abi(), cbroproperties, dwflags) }
}
#[inline]
pub unsafe fn ClusterGroupSetCloseEnum(hgroupsetenum: *mut _HGROUPSETENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupSetCloseEnum(hgroupsetenum : *mut _HGROUPSETENUM) -> u32);
    unsafe { ClusterGroupSetCloseEnum(hgroupsetenum as _) }
}
#[inline]
pub unsafe fn ClusterGroupSetControl(hgroupset: *const _HGROUPSET, hhostnode: Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, cbinbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, cboutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupSetControl(hgroupset : *const _HGROUPSET, hhostnode : *const _HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, cbinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, cboutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    unsafe { ClusterGroupSetControl(hgroupset, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, cbinbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, cboutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterGroupSetControlEx<P8>(hgroupset: *const _HGROUPSET, hhostnode: Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, cbinbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, cboutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P8) -> u32
where
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupSetControlEx(hgroupset : *const _HGROUPSET, hhostnode : *const _HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, cbinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, cboutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterGroupSetControlEx(hgroupset, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, cbinbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, cboutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn ClusterGroupSetEnum(hgroupsetenum: *const _HGROUPSETENUM, dwindex: u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupSetEnum(hgroupsetenum : *const _HGROUPSETENUM, dwindex : u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    unsafe { ClusterGroupSetEnum(hgroupsetenum, dwindex, core::mem::transmute(lpszname), lpcchname as _) }
}
#[inline]
pub unsafe fn ClusterGroupSetGetEnumCount(hgroupsetenum: *mut _HGROUPSETENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupSetGetEnumCount(hgroupsetenum : *mut _HGROUPSETENUM) -> u32);
    unsafe { ClusterGroupSetGetEnumCount(hgroupsetenum as _) }
}
#[inline]
pub unsafe fn ClusterGroupSetOpenEnum(hcluster: *mut _HCLUSTER) -> HGROUPSETENUM {
    windows_core::link!("clusapi.dll" "system" fn ClusterGroupSetOpenEnum(hcluster : *mut _HCLUSTER) -> HGROUPSETENUM);
    unsafe { ClusterGroupSetOpenEnum(hcluster as _) }
}
#[inline]
pub unsafe fn ClusterNetInterfaceCloseEnum(hnetinterfaceenum: *const _HNETINTERFACEENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNetInterfaceCloseEnum(hnetinterfaceenum : *const _HNETINTERFACEENUM) -> u32);
    unsafe { ClusterNetInterfaceCloseEnum(hnetinterfaceenum) }
}
#[inline]
pub unsafe fn ClusterNetInterfaceControl(hnetinterface: *const _HNETINTERFACE, hhostnode: Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNetInterfaceControl(hnetinterface : *const _HNETINTERFACE, hhostnode : *const _HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    unsafe { ClusterNetInterfaceControl(hnetinterface, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterNetInterfaceControlEx<P8>(hnetinterface: *const _HNETINTERFACE, hhostnode: Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P8) -> u32
where
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterNetInterfaceControlEx(hnetinterface : *const _HNETINTERFACE, hhostnode : *const _HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterNetInterfaceControlEx(hnetinterface, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn ClusterNetInterfaceEnum(hnetinterfaceenum: *const _HNETINTERFACEENUM, dwindex: u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNetInterfaceEnum(hnetinterfaceenum : *const _HNETINTERFACEENUM, dwindex : u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    unsafe { ClusterNetInterfaceEnum(hnetinterfaceenum, dwindex, core::mem::transmute(lpszname), lpcchname as _) }
}
#[inline]
pub unsafe fn ClusterNetInterfaceOpenEnum<P1, P2>(hcluster: *const _HCLUSTER, lpsznodename: P1, lpsznetworkname: P2) -> HNETINTERFACEENUM
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterNetInterfaceOpenEnum(hcluster : *const _HCLUSTER, lpsznodename : windows_core::PCWSTR, lpsznetworkname : windows_core::PCWSTR) -> HNETINTERFACEENUM);
    unsafe { ClusterNetInterfaceOpenEnum(hcluster, lpsznodename.param().abi(), lpsznetworkname.param().abi()) }
}
#[inline]
pub unsafe fn ClusterNetworkCloseEnum(hnetworkenum: *const _HNETWORKENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNetworkCloseEnum(hnetworkenum : *const _HNETWORKENUM) -> u32);
    unsafe { ClusterNetworkCloseEnum(hnetworkenum) }
}
#[inline]
pub unsafe fn ClusterNetworkControl(hnetwork: *const _HNETWORK, hhostnode: Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNetworkControl(hnetwork : *const _HNETWORK, hhostnode : *const _HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    unsafe { ClusterNetworkControl(hnetwork, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterNetworkControlEx<P8>(hnetwork: *const _HNETWORK, hhostnode: Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P8) -> u32
where
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterNetworkControlEx(hnetwork : *const _HNETWORK, hhostnode : *const _HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterNetworkControlEx(hnetwork, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn ClusterNetworkEnum(hnetworkenum: *const _HNETWORKENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNetworkEnum(hnetworkenum : *const _HNETWORKENUM, dwindex : u32, lpdwtype : *mut u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    unsafe { ClusterNetworkEnum(hnetworkenum, dwindex, lpdwtype as _, core::mem::transmute(lpszname), lpcchname as _) }
}
#[inline]
pub unsafe fn ClusterNetworkGetEnumCount(hnetworkenum: *const _HNETWORKENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNetworkGetEnumCount(hnetworkenum : *const _HNETWORKENUM) -> u32);
    unsafe { ClusterNetworkGetEnumCount(hnetworkenum) }
}
#[inline]
pub unsafe fn ClusterNetworkOpenEnum(hnetwork: *const _HNETWORK, dwtype: u32) -> HNETWORKENUM {
    windows_core::link!("clusapi.dll" "system" fn ClusterNetworkOpenEnum(hnetwork : *const _HNETWORK, dwtype : u32) -> HNETWORKENUM);
    unsafe { ClusterNetworkOpenEnum(hnetwork, dwtype) }
}
#[inline]
pub unsafe fn ClusterNodeCloseEnum(hnodeenum: *const _HNODEENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNodeCloseEnum(hnodeenum : *const _HNODEENUM) -> u32);
    unsafe { ClusterNodeCloseEnum(hnodeenum) }
}
#[inline]
pub unsafe fn ClusterNodeCloseEnumEx(hnodeenum: *const _HNODEENUMEX) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNodeCloseEnumEx(hnodeenum : *const _HNODEENUMEX) -> u32);
    unsafe { ClusterNodeCloseEnumEx(hnodeenum) }
}
#[inline]
pub unsafe fn ClusterNodeControl(hnode: *const _HNODE, hhostnode: Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNodeControl(hnode : *const _HNODE, hhostnode : *const _HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    unsafe { ClusterNodeControl(hnode, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterNodeControlEx<P8>(hnode: *const _HNODE, hhostnode: Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P8) -> u32
where
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterNodeControlEx(hnode : *const _HNODE, hhostnode : *const _HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterNodeControlEx(hnode, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn ClusterNodeEnum(hnodeenum: *const _HNODEENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNodeEnum(hnodeenum : *const _HNODEENUM, dwindex : u32, lpdwtype : *mut u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    unsafe { ClusterNodeEnum(hnodeenum, dwindex, lpdwtype as _, core::mem::transmute(lpszname), lpcchname as _) }
}
#[inline]
pub unsafe fn ClusterNodeEnumEx(hnodeenum: *const _HNODEENUMEX, dwindex: u32, pitem: *mut CLUSTER_ENUM_ITEM, cbitem: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNodeEnumEx(hnodeenum : *const _HNODEENUMEX, dwindex : u32, pitem : *mut CLUSTER_ENUM_ITEM, cbitem : *mut u32) -> u32);
    unsafe { ClusterNodeEnumEx(hnodeenum, dwindex, pitem as _, cbitem as _) }
}
#[inline]
pub unsafe fn ClusterNodeGetEnumCount(hnodeenum: *const _HNODEENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNodeGetEnumCount(hnodeenum : *const _HNODEENUM) -> u32);
    unsafe { ClusterNodeGetEnumCount(hnodeenum) }
}
#[inline]
pub unsafe fn ClusterNodeGetEnumCountEx(hnodeenum: *const _HNODEENUMEX) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterNodeGetEnumCountEx(hnodeenum : *const _HNODEENUMEX) -> u32);
    unsafe { ClusterNodeGetEnumCountEx(hnodeenum) }
}
#[inline]
pub unsafe fn ClusterNodeOpenEnum(hnode: *const _HNODE, dwtype: u32) -> HNODEENUM {
    windows_core::link!("clusapi.dll" "system" fn ClusterNodeOpenEnum(hnode : *const _HNODE, dwtype : u32) -> HNODEENUM);
    unsafe { ClusterNodeOpenEnum(hnode, dwtype) }
}
#[inline]
pub unsafe fn ClusterNodeOpenEnumEx(hnode: *const _HNODE, dwtype: u32, poptions: Option<*const core::ffi::c_void>) -> HNODEENUMEX {
    windows_core::link!("clusapi.dll" "system" fn ClusterNodeOpenEnumEx(hnode : *const _HNODE, dwtype : u32, poptions : *const core::ffi::c_void) -> HNODEENUMEX);
    unsafe { ClusterNodeOpenEnumEx(hnode, dwtype, poptions.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterNodeReplacement<P1, P2>(hcluster: *mut _HCLUSTER, lpsznodenamecurrent: P1, lpsznodenamenew: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterNodeReplacement(hcluster : *mut _HCLUSTER, lpsznodenamecurrent : windows_core::PCWSTR, lpsznodenamenew : windows_core::PCWSTR) -> u32);
    unsafe { ClusterNodeReplacement(hcluster as _, lpsznodenamecurrent.param().abi(), lpsznodenamenew.param().abi()) }
}
#[inline]
pub unsafe fn ClusterOpenEnum(hcluster: *const _HCLUSTER, dwtype: u32) -> HCLUSENUM {
    windows_core::link!("clusapi.dll" "system" fn ClusterOpenEnum(hcluster : *const _HCLUSTER, dwtype : u32) -> HCLUSENUM);
    unsafe { ClusterOpenEnum(hcluster, dwtype) }
}
#[inline]
pub unsafe fn ClusterOpenEnumEx(hcluster: *const _HCLUSTER, dwtype: u32, poptions: Option<*const core::ffi::c_void>) -> HCLUSENUMEX {
    windows_core::link!("clusapi.dll" "system" fn ClusterOpenEnumEx(hcluster : *const _HCLUSTER, dwtype : u32, poptions : *const core::ffi::c_void) -> HCLUSENUMEX);
    unsafe { ClusterOpenEnumEx(hcluster, dwtype, poptions.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterRegBatchAddCommand<P2>(hregbatch: *const _HREGBATCH, dwcommand: CLUSTER_REG_COMMAND, wzname: P2, dwoptions: u32, lpdata: Option<*const core::ffi::c_void>, cbdata: u32) -> i32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRegBatchAddCommand(hregbatch : *const _HREGBATCH, dwcommand : CLUSTER_REG_COMMAND, wzname : windows_core::PCWSTR, dwoptions : u32, lpdata : *const core::ffi::c_void, cbdata : u32) -> i32);
    unsafe { ClusterRegBatchAddCommand(hregbatch, dwcommand, wzname.param().abi(), dwoptions, lpdata.unwrap_or(core::mem::zeroed()) as _, cbdata) }
}
#[inline]
pub unsafe fn ClusterRegBatchCloseNotification(hbatchnotification: *const _HREGBATCHNOTIFICATION) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegBatchCloseNotification(hbatchnotification : *const _HREGBATCHNOTIFICATION) -> i32);
    unsafe { ClusterRegBatchCloseNotification(hbatchnotification) }
}
#[inline]
pub unsafe fn ClusterRegBatchReadCommand(hbatchnotification: *const _HREGBATCHNOTIFICATION, pbatchcommand: *mut CLUSTER_BATCH_COMMAND) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegBatchReadCommand(hbatchnotification : *const _HREGBATCHNOTIFICATION, pbatchcommand : *mut CLUSTER_BATCH_COMMAND) -> i32);
    unsafe { ClusterRegBatchReadCommand(hbatchnotification, pbatchcommand as _) }
}
#[inline]
pub unsafe fn ClusterRegCloseBatch(hregbatch: *const _HREGBATCH, bcommit: bool, failedcommandnumber: Option<*mut i32>) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegCloseBatch(hregbatch : *const _HREGBATCH, bcommit : windows_core::BOOL, failedcommandnumber : *mut i32) -> i32);
    unsafe { ClusterRegCloseBatch(hregbatch, bcommit.into(), failedcommandnumber.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterRegCloseBatchEx(hregbatch: *const _HREGBATCH, flags: u32, failedcommandnumber: Option<*mut i32>) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegCloseBatchEx(hregbatch : *const _HREGBATCH, flags : u32, failedcommandnumber : *mut i32) -> i32);
    unsafe { ClusterRegCloseBatchEx(hregbatch, flags, failedcommandnumber.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterRegCloseBatchNotifyPort(hbatchnotifyport: *const _HREGBATCHPORT) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegCloseBatchNotifyPort(hbatchnotifyport : *const _HREGBATCHPORT) -> i32);
    unsafe { ClusterRegCloseBatchNotifyPort(hbatchnotifyport) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn ClusterRegCloseKey(hkey: super::minwindef::HKEY) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegCloseKey(hkey : super::minwindef::HKEY) -> i32);
    unsafe { ClusterRegCloseKey(hkey) }
}
#[inline]
pub unsafe fn ClusterRegCloseReadBatch(hregreadbatch: *const _HREGREADBATCH, phregreadbatchreply: *mut HREGREADBATCHREPLY) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegCloseReadBatch(hregreadbatch : *const _HREGREADBATCH, phregreadbatchreply : *mut HREGREADBATCHREPLY) -> i32);
    unsafe { ClusterRegCloseReadBatch(hregreadbatch, phregreadbatchreply as _) }
}
#[inline]
pub unsafe fn ClusterRegCloseReadBatchEx(hregreadbatch: *const _HREGREADBATCH, flags: u32, phregreadbatchreply: *mut HREGREADBATCHREPLY) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegCloseReadBatchEx(hregreadbatch : *const _HREGREADBATCH, flags : u32, phregreadbatchreply : *mut HREGREADBATCHREPLY) -> i32);
    unsafe { ClusterRegCloseReadBatchEx(hregreadbatch, flags, phregreadbatchreply as _) }
}
#[inline]
pub unsafe fn ClusterRegCloseReadBatchReply(hregreadbatchreply: *const _HREGREADBATCHREPLY) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegCloseReadBatchReply(hregreadbatchreply : *const _HREGREADBATCHREPLY) -> i32);
    unsafe { ClusterRegCloseReadBatchReply(hregreadbatchreply) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn ClusterRegCreateBatch(hkey: Option<super::minwindef::HKEY>, phregbatch: *mut HREGBATCH) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegCreateBatch(hkey : super::minwindef::HKEY, phregbatch : *mut HREGBATCH) -> i32);
    unsafe { ClusterRegCreateBatch(hkey.unwrap_or(core::mem::zeroed()) as _, phregbatch as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn ClusterRegCreateBatchNotifyPort(hkey: super::minwindef::HKEY, phbatchnotifyport: *mut HREGBATCHPORT) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegCreateBatchNotifyPort(hkey : super::minwindef::HKEY, phbatchnotifyport : *mut HREGBATCHPORT) -> i32);
    unsafe { ClusterRegCreateBatchNotifyPort(hkey, phbatchnotifyport as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn ClusterRegCreateKey<P1>(hkey: super::minwindef::HKEY, lpszsubkey: P1, dwoptions: u32, samdesired: super::winreg::REGSAM, lpsecurityattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, phkresult: *mut super::minwindef::HKEY, lpdwdisposition: Option<*mut u32>) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRegCreateKey(hkey : super::minwindef::HKEY, lpszsubkey : windows_core::PCWSTR, dwoptions : u32, samdesired : super::winreg::REGSAM, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, phkresult : *mut super::minwindef::HKEY, lpdwdisposition : *mut u32) -> i32);
    unsafe { ClusterRegCreateKey(hkey, lpszsubkey.param().abi(), dwoptions, samdesired, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _, phkresult as _, lpdwdisposition.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn ClusterRegCreateKeyEx<P1, P7>(hkey: super::minwindef::HKEY, lpsubkey: P1, dwoptions: u32, samdesired: super::winreg::REGSAM, lpsecurityattributes: *mut super::minwinbase::SECURITY_ATTRIBUTES, phkresult: *mut super::minwindef::HKEY, lpdwdisposition: *mut u32, lpszreason: P7) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRegCreateKeyEx(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCWSTR, dwoptions : u32, samdesired : super::winreg::REGSAM, lpsecurityattributes : *mut super::minwinbase::SECURITY_ATTRIBUTES, phkresult : *mut super::minwindef::HKEY, lpdwdisposition : *mut u32, lpszreason : windows_core::PCWSTR) -> i32);
    unsafe { ClusterRegCreateKeyEx(hkey, lpsubkey.param().abi(), dwoptions, samdesired, lpsecurityattributes as _, phkresult as _, lpdwdisposition as _, lpszreason.param().abi()) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn ClusterRegCreateReadBatch(hkey: super::minwindef::HKEY, phregreadbatch: *mut HREGREADBATCH) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegCreateReadBatch(hkey : super::minwindef::HKEY, phregreadbatch : *mut HREGREADBATCH) -> i32);
    unsafe { ClusterRegCreateReadBatch(hkey, phregreadbatch as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn ClusterRegDeleteKey<P1>(hkey: super::minwindef::HKEY, lpszsubkey: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRegDeleteKey(hkey : super::minwindef::HKEY, lpszsubkey : windows_core::PCWSTR) -> i32);
    unsafe { ClusterRegDeleteKey(hkey, lpszsubkey.param().abi()) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn ClusterRegDeleteKeyEx<P1, P2>(hkey: super::minwindef::HKEY, lpsubkey: P1, lpszreason: P2) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRegDeleteKeyEx(hkey : super::minwindef::HKEY, lpsubkey : windows_core::PCWSTR, lpszreason : windows_core::PCWSTR) -> i32);
    unsafe { ClusterRegDeleteKeyEx(hkey, lpsubkey.param().abi(), lpszreason.param().abi()) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn ClusterRegDeleteValue<P1>(hkey: super::minwindef::HKEY, lpszvaluename: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRegDeleteValue(hkey : super::minwindef::HKEY, lpszvaluename : windows_core::PCWSTR) -> u32);
    unsafe { ClusterRegDeleteValue(hkey, lpszvaluename.param().abi()) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn ClusterRegDeleteValueEx<P1, P2>(hkey: super::minwindef::HKEY, lpszvaluename: P1, lpszreason: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRegDeleteValueEx(hkey : super::minwindef::HKEY, lpszvaluename : windows_core::PCWSTR, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterRegDeleteValueEx(hkey, lpszvaluename.param().abi(), lpszreason.param().abi()) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn ClusterRegEnumKey(hkey: super::minwindef::HKEY, dwindex: u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32, lpftlastwritetime: Option<*mut super::minwindef::FILETIME>) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegEnumKey(hkey : super::minwindef::HKEY, dwindex : u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32, lpftlastwritetime : *mut super::minwindef::FILETIME) -> i32);
    unsafe { ClusterRegEnumKey(hkey, dwindex, core::mem::transmute(lpszname), lpcchname as _, lpftlastwritetime.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn ClusterRegEnumValue(hkey: super::minwindef::HKEY, dwindex: u32, lpszvaluename: windows_core::PWSTR, lpcchvaluename: *mut u32, lpdwtype: Option<*mut u32>, lpdata: Option<*mut u8>, lpcbdata: Option<*mut u32>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegEnumValue(hkey : super::minwindef::HKEY, dwindex : u32, lpszvaluename : windows_core::PWSTR, lpcchvaluename : *mut u32, lpdwtype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> u32);
    unsafe { ClusterRegEnumValue(hkey, dwindex, core::mem::transmute(lpszvaluename), lpcchvaluename as _, lpdwtype.unwrap_or(core::mem::zeroed()) as _, lpdata.unwrap_or(core::mem::zeroed()) as _, lpcbdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterRegGetBatchNotification(hbatchnotify: *const _HREGBATCHPORT, phbatchnotification: *mut HREGBATCHNOTIFICATION) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegGetBatchNotification(hbatchnotify : *const _HREGBATCHPORT, phbatchnotification : *mut HREGBATCHNOTIFICATION) -> i32);
    unsafe { ClusterRegGetBatchNotification(hbatchnotify, phbatchnotification as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn ClusterRegGetKeySecurity(hkey: super::minwindef::HKEY, requestedinformation: super::winnt::SECURITY_INFORMATION, psecuritydescriptor: super::winnt::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegGetKeySecurity(hkey : super::minwindef::HKEY, requestedinformation : super::winnt::SECURITY_INFORMATION, psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor : *mut u32) -> i32);
    unsafe { ClusterRegGetKeySecurity(hkey, requestedinformation, psecuritydescriptor as _, lpcbsecuritydescriptor as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn ClusterRegOpenKey<P1>(hkey: super::minwindef::HKEY, lpszsubkey: P1, samdesired: super::winreg::REGSAM, phkresult: *mut super::minwindef::HKEY) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRegOpenKey(hkey : super::minwindef::HKEY, lpszsubkey : windows_core::PCWSTR, samdesired : super::winreg::REGSAM, phkresult : *mut super::minwindef::HKEY) -> i32);
    unsafe { ClusterRegOpenKey(hkey, lpszsubkey.param().abi(), samdesired, phkresult as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn ClusterRegQueryInfoKey(hkey: super::minwindef::HKEY, lpcsubkeys: *const u32, lpcchmaxsubkeylen: *const u32, lpcvalues: *const u32, lpcchmaxvaluenamelen: *const u32, lpcbmaxvaluelen: *const u32, lpcbsecuritydescriptor: *const u32, lpftlastwritetime: *const super::minwindef::FILETIME) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegQueryInfoKey(hkey : super::minwindef::HKEY, lpcsubkeys : *const u32, lpcchmaxsubkeylen : *const u32, lpcvalues : *const u32, lpcchmaxvaluenamelen : *const u32, lpcbmaxvaluelen : *const u32, lpcbsecuritydescriptor : *const u32, lpftlastwritetime : *const super::minwindef::FILETIME) -> i32);
    unsafe { ClusterRegQueryInfoKey(hkey, lpcsubkeys, lpcchmaxsubkeylen, lpcvalues, lpcchmaxvaluenamelen, lpcbmaxvaluelen, lpcbsecuritydescriptor, lpftlastwritetime) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn ClusterRegQueryValue<P1>(hkey: super::minwindef::HKEY, lpszvaluename: P1, lpdwvaluetype: Option<*mut u32>, lpdata: Option<*mut u8>, lpcbdata: Option<*mut u32>) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRegQueryValue(hkey : super::minwindef::HKEY, lpszvaluename : windows_core::PCWSTR, lpdwvaluetype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> i32);
    unsafe { ClusterRegQueryValue(hkey, lpszvaluename.param().abi(), lpdwvaluetype.unwrap_or(core::mem::zeroed()) as _, lpdata.unwrap_or(core::mem::zeroed()) as _, lpcbdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterRegReadBatchAddCommand<P1, P2>(hregreadbatch: *const _HREGREADBATCH, wzsubkeyname: P1, wzvaluename: P2) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRegReadBatchAddCommand(hregreadbatch : *const _HREGREADBATCH, wzsubkeyname : windows_core::PCWSTR, wzvaluename : windows_core::PCWSTR) -> i32);
    unsafe { ClusterRegReadBatchAddCommand(hregreadbatch, wzsubkeyname.param().abi(), wzvaluename.param().abi()) }
}
#[inline]
pub unsafe fn ClusterRegReadBatchReplyNextCommand(hregreadbatchreply: *const _HREGREADBATCHREPLY, pbatchcommand: *mut CLUSTER_READ_BATCH_COMMAND) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegReadBatchReplyNextCommand(hregreadbatchreply : *const _HREGREADBATCHREPLY, pbatchcommand : *mut CLUSTER_READ_BATCH_COMMAND) -> i32);
    unsafe { ClusterRegReadBatchReplyNextCommand(hregreadbatchreply, pbatchcommand as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn ClusterRegSetKeySecurity(hkey: super::minwindef::HKEY, securityinformation: super::winnt::SECURITY_INFORMATION, psecuritydescriptor: super::winnt::PSECURITY_DESCRIPTOR) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegSetKeySecurity(hkey : super::minwindef::HKEY, securityinformation : super::winnt::SECURITY_INFORMATION, psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR) -> i32);
    unsafe { ClusterRegSetKeySecurity(hkey, securityinformation, psecuritydescriptor) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn ClusterRegSetKeySecurityEx<P3>(hkey: super::minwindef::HKEY, securityinformation: super::winnt::SECURITY_INFORMATION, psecuritydescriptor: super::winnt::PSECURITY_DESCRIPTOR, lpszreason: P3) -> i32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRegSetKeySecurityEx(hkey : super::minwindef::HKEY, securityinformation : super::winnt::SECURITY_INFORMATION, psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, lpszreason : windows_core::PCWSTR) -> i32);
    unsafe { ClusterRegSetKeySecurityEx(hkey, securityinformation, psecuritydescriptor, lpszreason.param().abi()) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn ClusterRegSetValue<P1>(hkey: super::minwindef::HKEY, lpszvaluename: P1, dwtype: u32, lpdata: *const u8, cbdata: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRegSetValue(hkey : super::minwindef::HKEY, lpszvaluename : windows_core::PCWSTR, dwtype : u32, lpdata : *const u8, cbdata : u32) -> u32);
    unsafe { ClusterRegSetValue(hkey, lpszvaluename.param().abi(), dwtype, lpdata, cbdata) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn ClusterRegSetValueEx<P1, P5>(hkey: super::minwindef::HKEY, lpszvaluename: P1, dwtype: u32, lpdata: *const u8, cbdata: u32, lpszreason: P5) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRegSetValueEx(hkey : super::minwindef::HKEY, lpszvaluename : windows_core::PCWSTR, dwtype : u32, lpdata : *const u8, cbdata : u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterRegSetValueEx(hkey, lpszvaluename.param().abi(), dwtype, lpdata, cbdata, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn ClusterRegSyncDatabase(hcluster: *const _HCLUSTER, flags: u32) -> i32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRegSyncDatabase(hcluster : *const _HCLUSTER, flags : u32) -> i32);
    unsafe { ClusterRegSyncDatabase(hcluster, flags) }
}
#[inline]
pub unsafe fn ClusterRemoveAffinityRule<P1>(hcluster: *mut _HCLUSTER, rulename: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRemoveAffinityRule(hcluster : *mut _HCLUSTER, rulename : windows_core::PCWSTR) -> u32);
    unsafe { ClusterRemoveAffinityRule(hcluster as _, rulename.param().abi()) }
}
#[inline]
pub unsafe fn ClusterRemoveGroupFromAffinityRule<P1>(hcluster: *mut _HCLUSTER, rulename: P1, hgroup: *mut _HGROUP) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRemoveGroupFromAffinityRule(hcluster : *mut _HCLUSTER, rulename : windows_core::PCWSTR, hgroup : *mut _HGROUP) -> u32);
    unsafe { ClusterRemoveGroupFromAffinityRule(hcluster as _, rulename.param().abi(), hgroup as _) }
}
#[inline]
pub unsafe fn ClusterRemoveGroupFromGroupSet(hgroup: *const _HGROUP) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterRemoveGroupFromGroupSet(hgroup : *const _HGROUP) -> u32);
    unsafe { ClusterRemoveGroupFromGroupSet(hgroup) }
}
#[inline]
pub unsafe fn ClusterRemoveGroupFromGroupSetEx<P1>(hgroup: *const _HGROUP, lpszreason: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterRemoveGroupFromGroupSetEx(hgroup : *const _HGROUP, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterRemoveGroupFromGroupSetEx(hgroup, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn ClusterResourceCloseEnum(hresenum: *const _HRESENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceCloseEnum(hresenum : *const _HRESENUM) -> u32);
    unsafe { ClusterResourceCloseEnum(hresenum) }
}
#[inline]
pub unsafe fn ClusterResourceCloseEnumEx(hresourceenumex: *const _HRESENUMEX) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceCloseEnumEx(hresourceenumex : *const _HRESENUMEX) -> u32);
    unsafe { ClusterResourceCloseEnumEx(hresourceenumex) }
}
#[inline]
pub unsafe fn ClusterResourceControl(hresource: *const _HRESOURCE, hhostnode: Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, cbinbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, cboutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceControl(hresource : *const _HRESOURCE, hhostnode : *const _HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, cbinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, cboutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    unsafe { ClusterResourceControl(hresource, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, cbinbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, cboutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterResourceControlAsUser(hresource: *const _HRESOURCE, hhostnode: Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, cbinbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, cboutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceControlAsUser(hresource : *const _HRESOURCE, hhostnode : *const _HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, cbinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, cboutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    unsafe { ClusterResourceControlAsUser(hresource, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, cbinbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, cboutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterResourceControlAsUserEx<P8>(hresource: *const _HRESOURCE, hhostnode: Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, cbinbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, cboutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P8) -> u32
where
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceControlAsUserEx(hresource : *const _HRESOURCE, hhostnode : *const _HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, cbinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, cboutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterResourceControlAsUserEx(hresource, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, cbinbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, cboutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn ClusterResourceControlEx<P8>(hresource: *const _HRESOURCE, hhostnode: Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, cbinbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, cboutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P8) -> u32
where
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceControlEx(hresource : *const _HRESOURCE, hhostnode : *const _HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, cbinbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, cboutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterResourceControlEx(hresource, hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, cbinbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, cboutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn ClusterResourceEnum(hresenum: *const _HRESENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceEnum(hresenum : *const _HRESENUM, dwindex : u32, lpdwtype : *mut u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    unsafe { ClusterResourceEnum(hresenum, dwindex, lpdwtype as _, core::mem::transmute(lpszname), lpcchname as _) }
}
#[inline]
pub unsafe fn ClusterResourceEnumEx(hresourceenumex: *const _HRESENUMEX, dwindex: u32, pitem: *mut CLUSTER_RESOURCE_ENUM_ITEM, cbitem: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceEnumEx(hresourceenumex : *const _HRESENUMEX, dwindex : u32, pitem : *mut CLUSTER_RESOURCE_ENUM_ITEM, cbitem : *mut u32) -> u32);
    unsafe { ClusterResourceEnumEx(hresourceenumex, dwindex, pitem as _, cbitem as _) }
}
#[inline]
pub unsafe fn ClusterResourceGetEnumCount(hresenum: *const _HRESENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceGetEnumCount(hresenum : *const _HRESENUM) -> u32);
    unsafe { ClusterResourceGetEnumCount(hresenum) }
}
#[inline]
pub unsafe fn ClusterResourceGetEnumCountEx(hresourceenumex: *const _HRESENUMEX) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceGetEnumCountEx(hresourceenumex : *const _HRESENUMEX) -> u32);
    unsafe { ClusterResourceGetEnumCountEx(hresourceenumex) }
}
#[inline]
pub unsafe fn ClusterResourceOpenEnum(hresource: *const _HRESOURCE, dwtype: u32) -> HRESENUM {
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceOpenEnum(hresource : *const _HRESOURCE, dwtype : u32) -> HRESENUM);
    unsafe { ClusterResourceOpenEnum(hresource, dwtype) }
}
#[inline]
pub unsafe fn ClusterResourceOpenEnumEx<P1, P3>(hcluster: *const _HCLUSTER, lpszproperties: P1, cbproperties: u32, lpszroproperties: P3, cbroproperties: u32, dwflags: u32) -> HRESENUMEX
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceOpenEnumEx(hcluster : *const _HCLUSTER, lpszproperties : windows_core::PCWSTR, cbproperties : u32, lpszroproperties : windows_core::PCWSTR, cbroproperties : u32, dwflags : u32) -> HRESENUMEX);
    unsafe { ClusterResourceOpenEnumEx(hcluster, lpszproperties.param().abi(), cbproperties, lpszroproperties.param().abi(), cbroproperties, dwflags) }
}
#[inline]
pub unsafe fn ClusterResourceTypeCloseEnum(hrestypeenum: *const _HRESTYPEENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceTypeCloseEnum(hrestypeenum : *const _HRESTYPEENUM) -> u32);
    unsafe { ClusterResourceTypeCloseEnum(hrestypeenum) }
}
#[inline]
pub unsafe fn ClusterResourceTypeControl<P1>(hcluster: *const _HCLUSTER, lpszresourcetypename: P1, hhostnode: Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceTypeControl(hcluster : *const _HCLUSTER, lpszresourcetypename : windows_core::PCWSTR, hhostnode : *const _HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    unsafe { ClusterResourceTypeControl(hcluster, lpszresourcetypename.param().abi(), hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterResourceTypeControlAsUser<P1>(hcluster: *const _HCLUSTER, lpszresourcetypename: P1, hhostnode: Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceTypeControlAsUser(hcluster : *const _HCLUSTER, lpszresourcetypename : windows_core::PCWSTR, hhostnode : *const _HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32) -> u32);
    unsafe { ClusterResourceTypeControlAsUser(hcluster, lpszresourcetypename.param().abi(), hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ClusterResourceTypeControlAsUserEx<P1, P9>(hcluster: *const _HCLUSTER, lpszresourcetypename: P1, hhostnode: Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P9) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P9: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceTypeControlAsUserEx(hcluster : *const _HCLUSTER, lpszresourcetypename : windows_core::PCWSTR, hhostnode : *const _HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterResourceTypeControlAsUserEx(hcluster, lpszresourcetypename.param().abi(), hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn ClusterResourceTypeControlEx<P1, P9>(hcluster: *const _HCLUSTER, lpszresourcetypename: P1, hhostnode: Option<*const _HNODE>, dwcontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpszreason: P9) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P9: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceTypeControlEx(hcluster : *const _HCLUSTER, lpszresourcetypename : windows_core::PCWSTR, hhostnode : *const _HNODE, dwcontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ClusterResourceTypeControlEx(hcluster, lpszresourcetypename.param().abi(), hhostnode.unwrap_or(core::mem::zeroed()) as _, dwcontrolcode, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn ClusterResourceTypeEnum(hrestypeenum: *const _HRESTYPEENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceTypeEnum(hrestypeenum : *const _HRESTYPEENUM, dwindex : u32, lpdwtype : *mut u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    unsafe { ClusterResourceTypeEnum(hrestypeenum, dwindex, lpdwtype as _, core::mem::transmute(lpszname), lpcchname as _) }
}
#[inline]
pub unsafe fn ClusterResourceTypeGetEnumCount(hrestypeenum: *const _HRESTYPEENUM) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceTypeGetEnumCount(hrestypeenum : *const _HRESTYPEENUM) -> u32);
    unsafe { ClusterResourceTypeGetEnumCount(hrestypeenum) }
}
#[inline]
pub unsafe fn ClusterResourceTypeOpenEnum<P1>(hcluster: *const _HCLUSTER, lpszresourcetypename: P1, dwtype: u32) -> HRESTYPEENUM
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterResourceTypeOpenEnum(hcluster : *const _HCLUSTER, lpszresourcetypename : windows_core::PCWSTR, dwtype : u32) -> HRESTYPEENUM);
    unsafe { ClusterResourceTypeOpenEnum(hcluster, lpszresourcetypename.param().abi(), dwtype) }
}
#[inline]
pub unsafe fn ClusterSetAccountAccess<P1>(hcluster: *const _HCLUSTER, szaccountsid: P1, dwaccess: u32, dwcontroltype: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterSetAccountAccess(hcluster : *const _HCLUSTER, szaccountsid : windows_core::PCWSTR, dwaccess : u32, dwcontroltype : u32) -> u32);
    unsafe { ClusterSetAccountAccess(hcluster, szaccountsid.param().abi(), dwaccess, dwcontroltype) }
}
#[inline]
pub unsafe fn ClusterSharedVolumeSetSnapshotState<P1>(guidsnapshotset: windows_core::GUID, lpszvolumename: P1, state: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ClusterSharedVolumeSetSnapshotState(guidsnapshotset : windows_core::GUID, lpszvolumename : windows_core::PCWSTR, state : CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE) -> u32);
    unsafe { ClusterSharedVolumeSetSnapshotState(core::mem::transmute(guidsnapshotset), lpszvolumename.param().abi(), state) }
}
#[inline]
pub unsafe fn ClusterUpgradeFunctionalLevel(hcluster: *const _HCLUSTER, perform: bool, pfnprogresscallback: PCLUSTER_UPGRADE_PROGRESS_CALLBACK, pvcallbackarg: Option<*const core::ffi::c_void>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ClusterUpgradeFunctionalLevel(hcluster : *const _HCLUSTER, perform : windows_core::BOOL, pfnprogresscallback : PCLUSTER_UPGRADE_PROGRESS_CALLBACK, pvcallbackarg : *const core::ffi::c_void) -> u32);
    unsafe { ClusterUpgradeFunctionalLevel(hcluster, perform.into(), pfnprogresscallback, pvcallbackarg.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CreateCluster(pconfig: *const CREATE_CLUSTER_CONFIG, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: Option<*const core::ffi::c_void>) -> HCLUSTER {
    windows_core::link!("clusapi.dll" "system" fn CreateCluster(pconfig : *const CREATE_CLUSTER_CONFIG, pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg : *const core::ffi::c_void) -> HCLUSTER);
    unsafe { CreateCluster(pconfig, pfnprogresscallback, pvcallbackarg.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CreateClusterAvailabilitySet<P1>(hcluster: *const _HCLUSTER, lpavailabilitysetname: P1, pavailabilitysetconfig: *const CLUSTER_AVAILABILITY_SET_CONFIG) -> HGROUPSET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn CreateClusterAvailabilitySet(hcluster : *const _HCLUSTER, lpavailabilitysetname : windows_core::PCWSTR, pavailabilitysetconfig : *const CLUSTER_AVAILABILITY_SET_CONFIG) -> HGROUPSET);
    unsafe { CreateClusterAvailabilitySet(hcluster, lpavailabilitysetname.param().abi(), pavailabilitysetconfig) }
}
#[inline]
pub unsafe fn CreateClusterGroup<P1>(hcluster: *const _HCLUSTER, lpszgroupname: P1) -> HGROUP
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn CreateClusterGroup(hcluster : *const _HCLUSTER, lpszgroupname : windows_core::PCWSTR) -> HGROUP);
    unsafe { CreateClusterGroup(hcluster, lpszgroupname.param().abi()) }
}
#[inline]
pub unsafe fn CreateClusterGroupEx<P1>(hcluster: *const _HCLUSTER, lpszgroupname: P1, pgroupinfo: Option<*const CLUSTER_CREATE_GROUP_INFO>) -> HGROUP
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn CreateClusterGroupEx(hcluster : *const _HCLUSTER, lpszgroupname : windows_core::PCWSTR, pgroupinfo : *const CLUSTER_CREATE_GROUP_INFO) -> HGROUP);
    unsafe { CreateClusterGroupEx(hcluster, lpszgroupname.param().abi(), pgroupinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CreateClusterGroupSet<P1>(hcluster: *const _HCLUSTER, groupsetname: P1) -> HGROUPSET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn CreateClusterGroupSet(hcluster : *const _HCLUSTER, groupsetname : windows_core::PCWSTR) -> HGROUPSET);
    unsafe { CreateClusterGroupSet(hcluster, groupsetname.param().abi()) }
}
#[inline]
pub unsafe fn CreateClusterNameAccount(hcluster: *const _HCLUSTER, pconfig: *const CREATE_CLUSTER_NAME_ACCOUNT, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: Option<*const core::ffi::c_void>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn CreateClusterNameAccount(hcluster : *const _HCLUSTER, pconfig : *const CREATE_CLUSTER_NAME_ACCOUNT, pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg : *const core::ffi::c_void) -> u32);
    unsafe { CreateClusterNameAccount(hcluster, pconfig, pfnprogresscallback, pvcallbackarg.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CreateClusterNotifyPort(hchange: *const _HCHANGE, hcluster: *const _HCLUSTER, dwfilter: u32, dwnotifykey: usize) -> HCHANGE {
    windows_core::link!("clusapi.dll" "system" fn CreateClusterNotifyPort(hchange : *const _HCHANGE, hcluster : *const _HCLUSTER, dwfilter : u32, dwnotifykey : usize) -> HCHANGE);
    unsafe { CreateClusterNotifyPort(hchange, hcluster, dwfilter, dwnotifykey) }
}
#[inline]
pub unsafe fn CreateClusterNotifyPortV2(hchange: *const _HCHANGE, hcluster: *const _HCLUSTER, filters: *const NOTIFY_FILTER_AND_TYPE, dwfiltercount: u32, dwnotifykey: usize) -> HCHANGE {
    windows_core::link!("clusapi.dll" "system" fn CreateClusterNotifyPortV2(hchange : *const _HCHANGE, hcluster : *const _HCLUSTER, filters : *const NOTIFY_FILTER_AND_TYPE, dwfiltercount : u32, dwnotifykey : usize) -> HCHANGE);
    unsafe { CreateClusterNotifyPortV2(hchange, hcluster, filters, dwfiltercount, dwnotifykey) }
}
#[inline]
pub unsafe fn CreateClusterResource<P1, P2>(hgroup: *const _HGROUP, lpszresourcename: P1, lpszresourcetype: P2, dwflags: u32) -> HRESOURCE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn CreateClusterResource(hgroup : *const _HGROUP, lpszresourcename : windows_core::PCWSTR, lpszresourcetype : windows_core::PCWSTR, dwflags : u32) -> HRESOURCE);
    unsafe { CreateClusterResource(hgroup, lpszresourcename.param().abi(), lpszresourcetype.param().abi(), dwflags) }
}
#[inline]
pub unsafe fn CreateClusterResourceEx<P1, P2, P4>(hgroup: *const _HGROUP, lpszresourcename: P1, lpszresourcetype: P2, dwflags: u32, lpszreason: P4) -> HRESOURCE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn CreateClusterResourceEx(hgroup : *const _HGROUP, lpszresourcename : windows_core::PCWSTR, lpszresourcetype : windows_core::PCWSTR, dwflags : u32, lpszreason : windows_core::PCWSTR) -> HRESOURCE);
    unsafe { CreateClusterResourceEx(hgroup, lpszresourcename.param().abi(), lpszresourcetype.param().abi(), dwflags, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn CreateClusterResourceType<P1, P2, P3>(hcluster: *const _HCLUSTER, lpszresourcetypename: P1, lpszdisplayname: P2, lpszresourcetypedll: P3, dwlooksalivepollinterval: u32, dwisalivepollinterval: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn CreateClusterResourceType(hcluster : *const _HCLUSTER, lpszresourcetypename : windows_core::PCWSTR, lpszdisplayname : windows_core::PCWSTR, lpszresourcetypedll : windows_core::PCWSTR, dwlooksalivepollinterval : u32, dwisalivepollinterval : u32) -> u32);
    unsafe { CreateClusterResourceType(hcluster, lpszresourcetypename.param().abi(), lpszdisplayname.param().abi(), lpszresourcetypedll.param().abi(), dwlooksalivepollinterval, dwisalivepollinterval) }
}
#[inline]
pub unsafe fn CreateClusterResourceTypeEx<P1, P2, P3, P6>(hcluster: *const _HCLUSTER, lpszresourcetypename: P1, lpszdisplayname: P2, lpszresourcetypedll: P3, dwlooksalivepollinterval: u32, dwisalivepollinterval: u32, lpszreason: P6) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn CreateClusterResourceTypeEx(hcluster : *const _HCLUSTER, lpszresourcetypename : windows_core::PCWSTR, lpszdisplayname : windows_core::PCWSTR, lpszresourcetypedll : windows_core::PCWSTR, dwlooksalivepollinterval : u32, dwisalivepollinterval : u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { CreateClusterResourceTypeEx(hcluster, lpszresourcetypename.param().abi(), lpszdisplayname.param().abi(), lpszresourcetypedll.param().abi(), dwlooksalivepollinterval, dwisalivepollinterval, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn DeleteClusterGroup(hgroup: *const _HGROUP) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn DeleteClusterGroup(hgroup : *const _HGROUP) -> u32);
    unsafe { DeleteClusterGroup(hgroup) }
}
#[inline]
pub unsafe fn DeleteClusterGroupEx<P1>(hgroup: *const _HGROUP, lpszreason: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn DeleteClusterGroupEx(hgroup : *const _HGROUP, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { DeleteClusterGroupEx(hgroup, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn DeleteClusterGroupSet(hgroupset: *const _HGROUPSET) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn DeleteClusterGroupSet(hgroupset : *const _HGROUPSET) -> u32);
    unsafe { DeleteClusterGroupSet(hgroupset) }
}
#[inline]
pub unsafe fn DeleteClusterGroupSetEx<P1>(hgroupset: *const _HGROUPSET, lpszreason: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn DeleteClusterGroupSetEx(hgroupset : *const _HGROUPSET, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { DeleteClusterGroupSetEx(hgroupset, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn DeleteClusterResource(hresource: *const _HRESOURCE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn DeleteClusterResource(hresource : *const _HRESOURCE) -> u32);
    unsafe { DeleteClusterResource(hresource) }
}
#[inline]
pub unsafe fn DeleteClusterResourceEx<P1>(hresource: *const _HRESOURCE, lpszreason: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn DeleteClusterResourceEx(hresource : *const _HRESOURCE, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { DeleteClusterResourceEx(hresource, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn DeleteClusterResourceType<P1>(hcluster: *const _HCLUSTER, lpszresourcetypename: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn DeleteClusterResourceType(hcluster : *const _HCLUSTER, lpszresourcetypename : windows_core::PCWSTR) -> u32);
    unsafe { DeleteClusterResourceType(hcluster, lpszresourcetypename.param().abi()) }
}
#[inline]
pub unsafe fn DeleteClusterResourceTypeEx<P1, P2>(hcluster: *const _HCLUSTER, lpsztypename: P1, lpszreason: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn DeleteClusterResourceTypeEx(hcluster : *const _HCLUSTER, lpsztypename : windows_core::PCWSTR, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { DeleteClusterResourceTypeEx(hcluster, lpsztypename.param().abi(), lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn DestroyCluster(hcluster: *const _HCLUSTER, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: Option<*const core::ffi::c_void>, fdeletevirtualcomputerobjects: bool) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn DestroyCluster(hcluster : *const _HCLUSTER, pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg : *const core::ffi::c_void, fdeletevirtualcomputerobjects : windows_core::BOOL) -> u32);
    unsafe { DestroyCluster(hcluster, pfnprogresscallback, pvcallbackarg.unwrap_or(core::mem::zeroed()) as _, fdeletevirtualcomputerobjects.into()) }
}
#[inline]
pub unsafe fn DestroyClusterGroup(hgroup: *const _HGROUP) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn DestroyClusterGroup(hgroup : *const _HGROUP) -> u32);
    unsafe { DestroyClusterGroup(hgroup) }
}
#[inline]
pub unsafe fn DestroyClusterGroupEx<P1>(hgroup: *const _HGROUP, lpszreason: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn DestroyClusterGroupEx(hgroup : *const _HGROUP, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { DestroyClusterGroupEx(hgroup, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn DetermineCNOResTypeFromCluster(hcluster: *const _HCLUSTER, pcnorestype: *mut CLUSTER_MGMT_POINT_RESTYPE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn DetermineCNOResTypeFromCluster(hcluster : *const _HCLUSTER, pcnorestype : *mut CLUSTER_MGMT_POINT_RESTYPE) -> u32);
    unsafe { DetermineCNOResTypeFromCluster(hcluster, pcnorestype as _) }
}
#[inline]
pub unsafe fn DetermineCNOResTypeFromNodelist(cnodes: u32, ppsznodenames: *const windows_core::PCWSTR, pcnorestype: *mut CLUSTER_MGMT_POINT_RESTYPE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn DetermineCNOResTypeFromNodelist(cnodes : u32, ppsznodenames : *const windows_core::PCWSTR, pcnorestype : *mut CLUSTER_MGMT_POINT_RESTYPE) -> u32);
    unsafe { DetermineCNOResTypeFromNodelist(cnodes, ppsznodenames, pcnorestype as _) }
}
#[inline]
pub unsafe fn DetermineClusterCloudTypeFromCluster(hcluster: *const _HCLUSTER, pcloudtype: *mut CLUSTER_CLOUD_TYPE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn DetermineClusterCloudTypeFromCluster(hcluster : *const _HCLUSTER, pcloudtype : *mut CLUSTER_CLOUD_TYPE) -> u32);
    unsafe { DetermineClusterCloudTypeFromCluster(hcluster, pcloudtype as _) }
}
#[inline]
pub unsafe fn DetermineClusterCloudTypeFromNodelist(cnodes: u32, ppsznodenames: *const windows_core::PCWSTR, pcloudtype: *mut CLUSTER_CLOUD_TYPE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn DetermineClusterCloudTypeFromNodelist(cnodes : u32, ppsznodenames : *const windows_core::PCWSTR, pcloudtype : *mut CLUSTER_CLOUD_TYPE) -> u32);
    unsafe { DetermineClusterCloudTypeFromNodelist(cnodes, ppsznodenames, pcloudtype as _) }
}
#[inline]
pub unsafe fn EvictClusterNode(hnode: *const _HNODE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn EvictClusterNode(hnode : *const _HNODE) -> u32);
    unsafe { EvictClusterNode(hnode) }
}
#[inline]
pub unsafe fn EvictClusterNodeEx(hnode: *const _HNODE, dwtimeout: u32, phrcleanupstatus: *mut windows_core::HRESULT) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn EvictClusterNodeEx(hnode : *const _HNODE, dwtimeout : u32, phrcleanupstatus : *mut windows_core::HRESULT) -> u32);
    unsafe { EvictClusterNodeEx(hnode, dwtimeout, phrcleanupstatus as _) }
}
#[inline]
pub unsafe fn EvictClusterNodeEx2<P3>(hnode: *const _HNODE, dwtimeout: u32, phrcleanupstatus: *mut windows_core::HRESULT, lpszreason: P3) -> u32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn EvictClusterNodeEx2(hnode : *const _HNODE, dwtimeout : u32, phrcleanupstatus : *mut windows_core::HRESULT, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { EvictClusterNodeEx2(hnode, dwtimeout, phrcleanupstatus as _, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn FailClusterResource(hresource: *const _HRESOURCE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn FailClusterResource(hresource : *const _HRESOURCE) -> u32);
    unsafe { FailClusterResource(hresource) }
}
#[inline]
pub unsafe fn FailClusterResourceEx<P1>(hresource: *const _HRESOURCE, lpszreason: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn FailClusterResourceEx(hresource : *const _HRESOURCE, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { FailClusterResourceEx(hresource, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn GetClusterFromGroup(hgroup: *const _HGROUP) -> HCLUSTER {
    windows_core::link!("clusapi.dll" "system" fn GetClusterFromGroup(hgroup : *const _HGROUP) -> HCLUSTER);
    unsafe { GetClusterFromGroup(hgroup) }
}
#[inline]
pub unsafe fn GetClusterFromNetInterface(hnetinterface: *const _HNETINTERFACE) -> HCLUSTER {
    windows_core::link!("clusapi.dll" "system" fn GetClusterFromNetInterface(hnetinterface : *const _HNETINTERFACE) -> HCLUSTER);
    unsafe { GetClusterFromNetInterface(hnetinterface) }
}
#[inline]
pub unsafe fn GetClusterFromNetwork(hnetwork: *const _HNETWORK) -> HCLUSTER {
    windows_core::link!("clusapi.dll" "system" fn GetClusterFromNetwork(hnetwork : *const _HNETWORK) -> HCLUSTER);
    unsafe { GetClusterFromNetwork(hnetwork) }
}
#[inline]
pub unsafe fn GetClusterFromNode(hnode: *const _HNODE) -> HCLUSTER {
    windows_core::link!("clusapi.dll" "system" fn GetClusterFromNode(hnode : *const _HNODE) -> HCLUSTER);
    unsafe { GetClusterFromNode(hnode) }
}
#[inline]
pub unsafe fn GetClusterFromResource(hresource: *const _HRESOURCE) -> HCLUSTER {
    windows_core::link!("clusapi.dll" "system" fn GetClusterFromResource(hresource : *const _HRESOURCE) -> HCLUSTER);
    unsafe { GetClusterFromResource(hresource) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn GetClusterGroupKey(hgroup: *const _HGROUP, samdesired: super::winreg::REGSAM) -> super::minwindef::HKEY {
    windows_core::link!("clusapi.dll" "system" fn GetClusterGroupKey(hgroup : *const _HGROUP, samdesired : super::winreg::REGSAM) -> super::minwindef::HKEY);
    unsafe { GetClusterGroupKey(hgroup, samdesired) }
}
#[inline]
pub unsafe fn GetClusterGroupState(hgroup: *const _HGROUP, lpsznodename: Option<windows_core::PWSTR>, lpcchnodename: Option<*mut u32>) -> CLUSTER_GROUP_STATE {
    windows_core::link!("clusapi.dll" "system" fn GetClusterGroupState(hgroup : *const _HGROUP, lpsznodename : windows_core::PWSTR, lpcchnodename : *mut u32) -> CLUSTER_GROUP_STATE);
    unsafe { GetClusterGroupState(hgroup, lpsznodename.unwrap_or(core::mem::zeroed()) as _, lpcchnodename.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetClusterInformation(hcluster: *const _HCLUSTER, lpszclustername: windows_core::PWSTR, lpcchclustername: *mut u32, lpclusterinfo: Option<*mut CLUSTERVERSIONINFO>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn GetClusterInformation(hcluster : *const _HCLUSTER, lpszclustername : windows_core::PWSTR, lpcchclustername : *mut u32, lpclusterinfo : *mut CLUSTERVERSIONINFO) -> u32);
    unsafe { GetClusterInformation(hcluster, core::mem::transmute(lpszclustername), lpcchclustername as _, lpclusterinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn GetClusterKey(hcluster: *const _HCLUSTER, samdesired: super::winreg::REGSAM) -> super::minwindef::HKEY {
    windows_core::link!("clusapi.dll" "system" fn GetClusterKey(hcluster : *const _HCLUSTER, samdesired : super::winreg::REGSAM) -> super::minwindef::HKEY);
    unsafe { GetClusterKey(hcluster, samdesired) }
}
#[inline]
pub unsafe fn GetClusterNetInterface<P1, P2>(hcluster: *const _HCLUSTER, lpsznodename: P1, lpsznetworkname: P2, lpszinterfacename: windows_core::PWSTR, lpcchinterfacename: *mut u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn GetClusterNetInterface(hcluster : *const _HCLUSTER, lpsznodename : windows_core::PCWSTR, lpsznetworkname : windows_core::PCWSTR, lpszinterfacename : windows_core::PWSTR, lpcchinterfacename : *mut u32) -> u32);
    unsafe { GetClusterNetInterface(hcluster, lpsznodename.param().abi(), lpsznetworkname.param().abi(), core::mem::transmute(lpszinterfacename), lpcchinterfacename as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn GetClusterNetInterfaceKey(hnetinterface: *const _HNETINTERFACE, samdesired: super::winreg::REGSAM) -> super::minwindef::HKEY {
    windows_core::link!("clusapi.dll" "system" fn GetClusterNetInterfaceKey(hnetinterface : *const _HNETINTERFACE, samdesired : super::winreg::REGSAM) -> super::minwindef::HKEY);
    unsafe { GetClusterNetInterfaceKey(hnetinterface, samdesired) }
}
#[inline]
pub unsafe fn GetClusterNetInterfaceState(hnetinterface: *const _HNETINTERFACE) -> CLUSTER_NETINTERFACE_STATE {
    windows_core::link!("clusapi.dll" "system" fn GetClusterNetInterfaceState(hnetinterface : *const _HNETINTERFACE) -> CLUSTER_NETINTERFACE_STATE);
    unsafe { GetClusterNetInterfaceState(hnetinterface) }
}
#[inline]
pub unsafe fn GetClusterNetworkId(hnetwork: *const _HNETWORK, lpsznetworkid: windows_core::PWSTR, lpcchname: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn GetClusterNetworkId(hnetwork : *const _HNETWORK, lpsznetworkid : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    unsafe { GetClusterNetworkId(hnetwork, core::mem::transmute(lpsznetworkid), lpcchname as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn GetClusterNetworkKey(hnetwork: *const _HNETWORK, samdesired: super::winreg::REGSAM) -> super::minwindef::HKEY {
    windows_core::link!("clusapi.dll" "system" fn GetClusterNetworkKey(hnetwork : *const _HNETWORK, samdesired : super::winreg::REGSAM) -> super::minwindef::HKEY);
    unsafe { GetClusterNetworkKey(hnetwork, samdesired) }
}
#[inline]
pub unsafe fn GetClusterNetworkState(hnetwork: *const _HNETWORK) -> CLUSTER_NETWORK_STATE {
    windows_core::link!("clusapi.dll" "system" fn GetClusterNetworkState(hnetwork : *const _HNETWORK) -> CLUSTER_NETWORK_STATE);
    unsafe { GetClusterNetworkState(hnetwork) }
}
#[inline]
pub unsafe fn GetClusterNodeId(hnode: Option<*const _HNODE>, lpsznodeid: windows_core::PWSTR, lpcchname: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn GetClusterNodeId(hnode : *const _HNODE, lpsznodeid : windows_core::PWSTR, lpcchname : *mut u32) -> u32);
    unsafe { GetClusterNodeId(hnode.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lpsznodeid), lpcchname as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn GetClusterNodeKey(hnode: *const _HNODE, samdesired: super::winreg::REGSAM) -> super::minwindef::HKEY {
    windows_core::link!("clusapi.dll" "system" fn GetClusterNodeKey(hnode : *const _HNODE, samdesired : super::winreg::REGSAM) -> super::minwindef::HKEY);
    unsafe { GetClusterNodeKey(hnode, samdesired) }
}
#[inline]
pub unsafe fn GetClusterNodeState(hnode: *const _HNODE) -> CLUSTER_NODE_STATE {
    windows_core::link!("clusapi.dll" "system" fn GetClusterNodeState(hnode : *const _HNODE) -> CLUSTER_NODE_STATE);
    unsafe { GetClusterNodeState(hnode) }
}
#[inline]
pub unsafe fn GetClusterNotify(hchange: *const _HCHANGE, lpdwnotifykey: *mut usize, lpdwfiltertype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32, dwmilliseconds: u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn GetClusterNotify(hchange : *const _HCHANGE, lpdwnotifykey : *mut usize, lpdwfiltertype : *mut u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32, dwmilliseconds : u32) -> u32);
    unsafe { GetClusterNotify(hchange, lpdwnotifykey as _, lpdwfiltertype as _, core::mem::transmute(lpszname), lpcchname as _, dwmilliseconds) }
}
#[inline]
pub unsafe fn GetClusterNotifyV2(hchange: *const _HCHANGE, lpdwnotifykey: *mut usize, pfilterandtype: Option<*mut NOTIFY_FILTER_AND_TYPE>, buffer: Option<*mut u8>, lpbbuffersize: Option<*mut u32>, lpszobjectid: Option<windows_core::PWSTR>, lpcchobjectid: Option<*mut u32>, lpszparentid: Option<windows_core::PWSTR>, lpcchparentid: Option<*mut u32>, lpszname: Option<windows_core::PWSTR>, lpcchname: Option<*mut u32>, lpsztype: Option<windows_core::PWSTR>, lpcchtype: Option<*mut u32>, dwmilliseconds: Option<u32>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn GetClusterNotifyV2(hchange : *const _HCHANGE, lpdwnotifykey : *mut usize, pfilterandtype : *mut NOTIFY_FILTER_AND_TYPE, buffer : *mut u8, lpbbuffersize : *mut u32, lpszobjectid : windows_core::PWSTR, lpcchobjectid : *mut u32, lpszparentid : windows_core::PWSTR, lpcchparentid : *mut u32, lpszname : windows_core::PWSTR, lpcchname : *mut u32, lpsztype : windows_core::PWSTR, lpcchtype : *mut u32, dwmilliseconds : u32) -> u32);
    unsafe {
        GetClusterNotifyV2(
            hchange,
            lpdwnotifykey as _,
            pfilterandtype.unwrap_or(core::mem::zeroed()) as _,
            buffer.unwrap_or(core::mem::zeroed()) as _,
            lpbbuffersize.unwrap_or(core::mem::zeroed()) as _,
            lpszobjectid.unwrap_or(core::mem::zeroed()) as _,
            lpcchobjectid.unwrap_or(core::mem::zeroed()) as _,
            lpszparentid.unwrap_or(core::mem::zeroed()) as _,
            lpcchparentid.unwrap_or(core::mem::zeroed()) as _,
            lpszname.unwrap_or(core::mem::zeroed()) as _,
            lpcchname.unwrap_or(core::mem::zeroed()) as _,
            lpsztype.unwrap_or(core::mem::zeroed()) as _,
            lpcchtype.unwrap_or(core::mem::zeroed()) as _,
            dwmilliseconds.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[inline]
pub unsafe fn GetClusterQuorumResource(hcluster: *const _HCLUSTER, lpszresourcename: windows_core::PWSTR, lpcchresourcename: *mut u32, lpszdevicename: windows_core::PWSTR, lpcchdevicename: *mut u32, lpdwmaxquorumlogsize: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn GetClusterQuorumResource(hcluster : *const _HCLUSTER, lpszresourcename : windows_core::PWSTR, lpcchresourcename : *mut u32, lpszdevicename : windows_core::PWSTR, lpcchdevicename : *mut u32, lpdwmaxquorumlogsize : *mut u32) -> u32);
    unsafe { GetClusterQuorumResource(hcluster, core::mem::transmute(lpszresourcename), lpcchresourcename as _, core::mem::transmute(lpszdevicename), lpcchdevicename as _, lpdwmaxquorumlogsize as _) }
}
#[inline]
pub unsafe fn GetClusterResourceDependencyExpression(hresource: *const _HRESOURCE, lpszdependencyexpression: Option<windows_core::PWSTR>, lpcchdependencyexpression: *mut u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn GetClusterResourceDependencyExpression(hresource : *const _HRESOURCE, lpszdependencyexpression : windows_core::PWSTR, lpcchdependencyexpression : *mut u32) -> u32);
    unsafe { GetClusterResourceDependencyExpression(hresource, lpszdependencyexpression.unwrap_or(core::mem::zeroed()) as _, lpcchdependencyexpression as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn GetClusterResourceKey(hresource: *const _HRESOURCE, samdesired: super::winreg::REGSAM) -> super::minwindef::HKEY {
    windows_core::link!("clusapi.dll" "system" fn GetClusterResourceKey(hresource : *const _HRESOURCE, samdesired : super::winreg::REGSAM) -> super::minwindef::HKEY);
    unsafe { GetClusterResourceKey(hresource, samdesired) }
}
#[inline]
pub unsafe fn GetClusterResourceNetworkName(hresource: *const _HRESOURCE, lpbuffer: windows_core::PWSTR, nsize: *mut u32) -> windows_core::BOOL {
    windows_core::link!("clusapi.dll" "system" fn GetClusterResourceNetworkName(hresource : *const _HRESOURCE, lpbuffer : windows_core::PWSTR, nsize : *mut u32) -> windows_core::BOOL);
    unsafe { GetClusterResourceNetworkName(hresource, core::mem::transmute(lpbuffer), nsize as _) }
}
#[inline]
pub unsafe fn GetClusterResourceState(hresource: *const _HRESOURCE, lpsznodename: Option<windows_core::PWSTR>, lpcchnodename: Option<*mut u32>, lpszgroupname: Option<windows_core::PWSTR>, lpcchgroupname: Option<*mut u32>) -> CLUSTER_RESOURCE_STATE {
    windows_core::link!("clusapi.dll" "system" fn GetClusterResourceState(hresource : *const _HRESOURCE, lpsznodename : windows_core::PWSTR, lpcchnodename : *mut u32, lpszgroupname : windows_core::PWSTR, lpcchgroupname : *mut u32) -> CLUSTER_RESOURCE_STATE);
    unsafe { GetClusterResourceState(hresource, lpsznodename.unwrap_or(core::mem::zeroed()) as _, lpcchnodename.unwrap_or(core::mem::zeroed()) as _, lpszgroupname.unwrap_or(core::mem::zeroed()) as _, lpcchgroupname.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_winreg"))]
#[inline]
pub unsafe fn GetClusterResourceTypeKey<P1>(hcluster: *const _HCLUSTER, lpsztypename: P1, samdesired: super::winreg::REGSAM) -> super::minwindef::HKEY
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn GetClusterResourceTypeKey(hcluster : *const _HCLUSTER, lpsztypename : windows_core::PCWSTR, samdesired : super::winreg::REGSAM) -> super::minwindef::HKEY);
    unsafe { GetClusterResourceTypeKey(hcluster, lpsztypename.param().abi(), samdesired) }
}
#[inline]
pub unsafe fn GetNodeCloudTypeDW<P0>(ppsznodename: P0, nodecloudtype: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn GetNodeCloudTypeDW(ppsznodename : windows_core::PCWSTR, nodecloudtype : *mut u32) -> u32);
    unsafe { GetNodeCloudTypeDW(ppsznodename.param().abi(), nodecloudtype as _) }
}
#[inline]
pub unsafe fn GetNodeClusterState<P0>(lpsznodename: P0, pdwclusterstate: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn GetNodeClusterState(lpsznodename : windows_core::PCWSTR, pdwclusterstate : *mut u32) -> u32);
    unsafe { GetNodeClusterState(lpsznodename.param().abi(), pdwclusterstate as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetNotifyEventHandle(hchange: *const _HCHANGE, lphtargetevent: *mut super::winnt::HANDLE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn GetNotifyEventHandle(hchange : *const _HCHANGE, lphtargetevent : *mut super::winnt::HANDLE) -> u32);
    unsafe { GetNotifyEventHandle(hchange, lphtargetevent as _) }
}
#[inline]
pub unsafe fn IsFileOnClusterSharedVolume<P0>(lpszpathname: P0, pbfileisonsharedvolume: *mut windows_core::BOOL) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn IsFileOnClusterSharedVolume(lpszpathname : windows_core::PCWSTR, pbfileisonsharedvolume : *mut windows_core::BOOL) -> u32);
    unsafe { IsFileOnClusterSharedVolume(lpszpathname.param().abi(), pbfileisonsharedvolume as _) }
}
#[inline]
pub unsafe fn MoveClusterGroup(hgroup: *const _HGROUP, hdestinationnode: Option<*const _HNODE>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn MoveClusterGroup(hgroup : *const _HGROUP, hdestinationnode : *const _HNODE) -> u32);
    unsafe { MoveClusterGroup(hgroup, hdestinationnode.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MoveClusterGroupEx(hgroup: *const _HGROUP, hdestinationnode: Option<*const _HNODE>, dwmoveflags: u32, lpinbuffer: Option<&[u8]>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn MoveClusterGroupEx(hgroup : *const _HGROUP, hdestinationnode : *const _HNODE, dwmoveflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32) -> u32);
    unsafe { MoveClusterGroupEx(hgroup, hdestinationnode.unwrap_or(core::mem::zeroed()) as _, dwmoveflags, core::mem::transmute(lpinbuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn MoveClusterGroupEx2<P5>(hgroup: *const _HGROUP, hdestinationnode: Option<*const _HNODE>, dwmoveflags: u32, lpinbuffer: Option<&[u8]>, lpszreason: P5) -> u32
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn MoveClusterGroupEx2(hgroup : *const _HGROUP, hdestinationnode : *const _HNODE, dwmoveflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { MoveClusterGroupEx2(hgroup, hdestinationnode.unwrap_or(core::mem::zeroed()) as _, dwmoveflags, core::mem::transmute(lpinbuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.map_or(0, |slice| slice.len().try_into().unwrap()), lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn OfflineClusterGroup(hgroup: *const _HGROUP) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn OfflineClusterGroup(hgroup : *const _HGROUP) -> u32);
    unsafe { OfflineClusterGroup(hgroup) }
}
#[inline]
pub unsafe fn OfflineClusterGroupEx(hgroup: *const _HGROUP, dwofflineflags: u32, lpinbuffer: Option<&[u8]>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn OfflineClusterGroupEx(hgroup : *const _HGROUP, dwofflineflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32) -> u32);
    unsafe { OfflineClusterGroupEx(hgroup, dwofflineflags, core::mem::transmute(lpinbuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn OfflineClusterGroupEx2<P4>(hgroup: *mut _HGROUP, dwofflineflags: u32, lpinbuffer: *mut u8, cbinbuffersize: u32, lpszreason: P4) -> u32
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OfflineClusterGroupEx2(hgroup : *mut _HGROUP, dwofflineflags : u32, lpinbuffer : *mut u8, cbinbuffersize : u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { OfflineClusterGroupEx2(hgroup as _, dwofflineflags, lpinbuffer as _, cbinbuffersize, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn OfflineClusterResource(hresource: *const _HRESOURCE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn OfflineClusterResource(hresource : *const _HRESOURCE) -> u32);
    unsafe { OfflineClusterResource(hresource) }
}
#[inline]
pub unsafe fn OfflineClusterResourceEx(hresource: *const _HRESOURCE, dwofflineflags: u32, lpinbuffer: Option<&[u8]>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn OfflineClusterResourceEx(hresource : *const _HRESOURCE, dwofflineflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32) -> u32);
    unsafe { OfflineClusterResourceEx(hresource, dwofflineflags, core::mem::transmute(lpinbuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn OfflineClusterResourceEx2<P4>(hresource: *const _HRESOURCE, dwofflineflags: u32, lpinbuffer: Option<&[u8]>, lpszreason: P4) -> u32
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OfflineClusterResourceEx2(hresource : *const _HRESOURCE, dwofflineflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { OfflineClusterResourceEx2(hresource, dwofflineflags, core::mem::transmute(lpinbuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.map_or(0, |slice| slice.len().try_into().unwrap()), lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn OnlineClusterGroup(hgroup: *const _HGROUP, hdestinationnode: Option<*const _HNODE>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn OnlineClusterGroup(hgroup : *const _HGROUP, hdestinationnode : *const _HNODE) -> u32);
    unsafe { OnlineClusterGroup(hgroup, hdestinationnode.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn OnlineClusterGroupEx(hgroup: *const _HGROUP, hdestinationnode: Option<*const _HNODE>, dwonlineflags: u32, lpinbuffer: Option<&[u8]>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn OnlineClusterGroupEx(hgroup : *const _HGROUP, hdestinationnode : *const _HNODE, dwonlineflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32) -> u32);
    unsafe { OnlineClusterGroupEx(hgroup, hdestinationnode.unwrap_or(core::mem::zeroed()) as _, dwonlineflags, core::mem::transmute(lpinbuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn OnlineClusterGroupEx2<P5>(hgroup: *const _HGROUP, hdestinationnode: Option<*const _HNODE>, dwonlineflags: u32, lpinbuffer: Option<&[u8]>, lpszreason: P5) -> u32
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OnlineClusterGroupEx2(hgroup : *const _HGROUP, hdestinationnode : *const _HNODE, dwonlineflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { OnlineClusterGroupEx2(hgroup, hdestinationnode.unwrap_or(core::mem::zeroed()) as _, dwonlineflags, core::mem::transmute(lpinbuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.map_or(0, |slice| slice.len().try_into().unwrap()), lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn OnlineClusterResource(hresource: *const _HRESOURCE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn OnlineClusterResource(hresource : *const _HRESOURCE) -> u32);
    unsafe { OnlineClusterResource(hresource) }
}
#[inline]
pub unsafe fn OnlineClusterResourceEx(hresource: *const _HRESOURCE, dwonlineflags: u32, lpinbuffer: Option<&[u8]>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn OnlineClusterResourceEx(hresource : *const _HRESOURCE, dwonlineflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32) -> u32);
    unsafe { OnlineClusterResourceEx(hresource, dwonlineflags, core::mem::transmute(lpinbuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn OnlineClusterResourceEx2<P4>(hresource: *const _HRESOURCE, dwonlineflags: u32, lpinbuffer: Option<&[u8]>, lpszreason: P4) -> u32
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OnlineClusterResourceEx2(hresource : *const _HRESOURCE, dwonlineflags : u32, lpinbuffer : *const u8, cbinbuffersize : u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { OnlineClusterResourceEx2(hresource, dwonlineflags, core::mem::transmute(lpinbuffer.map_or(core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.map_or(0, |slice| slice.len().try_into().unwrap()), lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn OpenCluster<P0>(lpszclustername: P0) -> HCLUSTER
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OpenCluster(lpszclustername : windows_core::PCWSTR) -> HCLUSTER);
    unsafe { OpenCluster(lpszclustername.param().abi()) }
}
#[inline]
pub unsafe fn OpenClusterEx<P0>(lpszclustername: P0, desiredaccess: u32, grantedaccess: Option<*mut u32>) -> HCLUSTER
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OpenClusterEx(lpszclustername : windows_core::PCWSTR, desiredaccess : u32, grantedaccess : *mut u32) -> HCLUSTER);
    unsafe { OpenClusterEx(lpszclustername.param().abi(), desiredaccess, grantedaccess.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn OpenClusterGroup<P1>(hcluster: *const _HCLUSTER, lpszgroupname: P1) -> HGROUP
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OpenClusterGroup(hcluster : *const _HCLUSTER, lpszgroupname : windows_core::PCWSTR) -> HGROUP);
    unsafe { OpenClusterGroup(hcluster, lpszgroupname.param().abi()) }
}
#[inline]
pub unsafe fn OpenClusterGroupEx<P1>(hcluster: *const _HCLUSTER, lpszgroupname: P1, dwdesiredaccess: u32, lpdwgrantedaccess: Option<*mut u32>) -> HGROUP
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OpenClusterGroupEx(hcluster : *const _HCLUSTER, lpszgroupname : windows_core::PCWSTR, dwdesiredaccess : u32, lpdwgrantedaccess : *mut u32) -> HGROUP);
    unsafe { OpenClusterGroupEx(hcluster, lpszgroupname.param().abi(), dwdesiredaccess, lpdwgrantedaccess.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn OpenClusterGroupSet<P1>(hcluster: *const _HCLUSTER, lpszgroupsetname: P1) -> HGROUPSET
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OpenClusterGroupSet(hcluster : *const _HCLUSTER, lpszgroupsetname : windows_core::PCWSTR) -> HGROUPSET);
    unsafe { OpenClusterGroupSet(hcluster, lpszgroupsetname.param().abi()) }
}
#[inline]
pub unsafe fn OpenClusterNetInterface<P1>(hcluster: *const _HCLUSTER, lpszinterfacename: P1) -> HNETINTERFACE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OpenClusterNetInterface(hcluster : *const _HCLUSTER, lpszinterfacename : windows_core::PCWSTR) -> HNETINTERFACE);
    unsafe { OpenClusterNetInterface(hcluster, lpszinterfacename.param().abi()) }
}
#[inline]
pub unsafe fn OpenClusterNetInterfaceEx<P1>(hcluster: *const _HCLUSTER, lpszinterfacename: P1, dwdesiredaccess: u32, lpdwgrantedaccess: Option<*mut u32>) -> HNETINTERFACE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OpenClusterNetInterfaceEx(hcluster : *const _HCLUSTER, lpszinterfacename : windows_core::PCWSTR, dwdesiredaccess : u32, lpdwgrantedaccess : *mut u32) -> HNETINTERFACE);
    unsafe { OpenClusterNetInterfaceEx(hcluster, lpszinterfacename.param().abi(), dwdesiredaccess, lpdwgrantedaccess.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn OpenClusterNetwork<P1>(hcluster: *const _HCLUSTER, lpsznetworkname: P1) -> HNETWORK
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OpenClusterNetwork(hcluster : *const _HCLUSTER, lpsznetworkname : windows_core::PCWSTR) -> HNETWORK);
    unsafe { OpenClusterNetwork(hcluster, lpsznetworkname.param().abi()) }
}
#[inline]
pub unsafe fn OpenClusterNetworkEx<P1>(hcluster: *const _HCLUSTER, lpsznetworkname: P1, dwdesiredaccess: u32, lpdwgrantedaccess: Option<*mut u32>) -> HNETWORK
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OpenClusterNetworkEx(hcluster : *const _HCLUSTER, lpsznetworkname : windows_core::PCWSTR, dwdesiredaccess : u32, lpdwgrantedaccess : *mut u32) -> HNETWORK);
    unsafe { OpenClusterNetworkEx(hcluster, lpsznetworkname.param().abi(), dwdesiredaccess, lpdwgrantedaccess.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn OpenClusterNode<P1>(hcluster: *const _HCLUSTER, lpsznodename: P1) -> HNODE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OpenClusterNode(hcluster : *const _HCLUSTER, lpsznodename : windows_core::PCWSTR) -> HNODE);
    unsafe { OpenClusterNode(hcluster, lpsznodename.param().abi()) }
}
#[inline]
pub unsafe fn OpenClusterNodeById(hcluster: *mut _HCLUSTER, nodeid: u32) -> HNODE {
    windows_core::link!("clusapi.dll" "system" fn OpenClusterNodeById(hcluster : *mut _HCLUSTER, nodeid : u32) -> HNODE);
    unsafe { OpenClusterNodeById(hcluster as _, nodeid) }
}
#[inline]
pub unsafe fn OpenClusterNodeEx<P1>(hcluster: *const _HCLUSTER, lpsznodename: P1, dwdesiredaccess: u32, lpdwgrantedaccess: Option<*mut u32>) -> HNODE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OpenClusterNodeEx(hcluster : *const _HCLUSTER, lpsznodename : windows_core::PCWSTR, dwdesiredaccess : u32, lpdwgrantedaccess : *mut u32) -> HNODE);
    unsafe { OpenClusterNodeEx(hcluster, lpsznodename.param().abi(), dwdesiredaccess, lpdwgrantedaccess.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn OpenClusterResource<P1>(hcluster: *const _HCLUSTER, lpszresourcename: P1) -> HRESOURCE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OpenClusterResource(hcluster : *const _HCLUSTER, lpszresourcename : windows_core::PCWSTR) -> HRESOURCE);
    unsafe { OpenClusterResource(hcluster, lpszresourcename.param().abi()) }
}
#[inline]
pub unsafe fn OpenClusterResourceEx<P1>(hcluster: *const _HCLUSTER, lpszresourcename: P1, dwdesiredaccess: u32, lpdwgrantedaccess: Option<*mut u32>) -> HRESOURCE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn OpenClusterResourceEx(hcluster : *const _HCLUSTER, lpszresourcename : windows_core::PCWSTR, dwdesiredaccess : u32, lpdwgrantedaccess : *mut u32) -> HRESOURCE);
    unsafe { OpenClusterResourceEx(hcluster, lpszresourcename.param().abi(), dwdesiredaccess, lpdwgrantedaccess.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PauseClusterNode(hnode: *const _HNODE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn PauseClusterNode(hnode : *const _HNODE) -> u32);
    unsafe { PauseClusterNode(hnode) }
}
#[inline]
pub unsafe fn PauseClusterNodeEx(hnode: *const _HNODE, bdrainnode: bool, dwpauseflags: u32, hnodedraintarget: Option<*const _HNODE>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn PauseClusterNodeEx(hnode : *const _HNODE, bdrainnode : windows_core::BOOL, dwpauseflags : u32, hnodedraintarget : *const _HNODE) -> u32);
    unsafe { PauseClusterNodeEx(hnode, bdrainnode.into(), dwpauseflags, hnodedraintarget.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PauseClusterNodeEx2<P4>(hnode: *const _HNODE, bdrainnode: bool, dwpauseflags: u32, hnodedraintarget: Option<*const _HNODE>, lpszreason: P4) -> u32
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn PauseClusterNodeEx2(hnode : *const _HNODE, bdrainnode : windows_core::BOOL, dwpauseflags : u32, hnodedraintarget : *const _HNODE, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { PauseClusterNodeEx2(hnode, bdrainnode.into(), dwpauseflags, hnodedraintarget.unwrap_or(core::mem::zeroed()) as _, lpszreason.param().abi()) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn RegisterClusterNotify(hchange: *const _HCHANGE, dwfiltertype: u32, hobject: super::winnt::HANDLE, dwnotifykey: usize) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn RegisterClusterNotify(hchange : *const _HCHANGE, dwfiltertype : u32, hobject : super::winnt::HANDLE, dwnotifykey : usize) -> u32);
    unsafe { RegisterClusterNotify(hchange, dwfiltertype, hobject, dwnotifykey) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn RegisterClusterNotifyV2(hchange: *const _HCHANGE, filter: NOTIFY_FILTER_AND_TYPE, hobject: super::winnt::HANDLE, dwnotifykey: usize) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn RegisterClusterNotifyV2(hchange : *const _HCHANGE, filter : NOTIFY_FILTER_AND_TYPE, hobject : super::winnt::HANDLE, dwnotifykey : usize) -> u32);
    unsafe { RegisterClusterNotifyV2(hchange, core::mem::transmute(filter), hobject, dwnotifykey) }
}
#[inline]
pub unsafe fn RegisterClusterResourceTypeNotifyV2<P3>(hchange: *mut _HCHANGE, hcluster: *mut _HCLUSTER, flags: i64, restypename: P3, dwnotifykey: usize) -> u32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn RegisterClusterResourceTypeNotifyV2(hchange : *mut _HCHANGE, hcluster : *mut _HCLUSTER, flags : i64, restypename : windows_core::PCWSTR, dwnotifykey : usize) -> u32);
    unsafe { RegisterClusterResourceTypeNotifyV2(hchange as _, hcluster as _, flags, restypename.param().abi(), dwnotifykey) }
}
#[inline]
pub unsafe fn RemoveClusterGroupDependency(hgroup: *const _HGROUP, hdependson: *const _HGROUP) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn RemoveClusterGroupDependency(hgroup : *const _HGROUP, hdependson : *const _HGROUP) -> u32);
    unsafe { RemoveClusterGroupDependency(hgroup, hdependson) }
}
#[inline]
pub unsafe fn RemoveClusterGroupDependencyEx<P2>(hgroup: *const _HGROUP, hdependson: *const _HGROUP, lpszreason: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn RemoveClusterGroupDependencyEx(hgroup : *const _HGROUP, hdependson : *const _HGROUP, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { RemoveClusterGroupDependencyEx(hgroup, hdependson, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn RemoveClusterGroupSetDependency(hgroupset: *const _HGROUPSET, hdependson: *const _HGROUPSET) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn RemoveClusterGroupSetDependency(hgroupset : *const _HGROUPSET, hdependson : *const _HGROUPSET) -> u32);
    unsafe { RemoveClusterGroupSetDependency(hgroupset, hdependson) }
}
#[inline]
pub unsafe fn RemoveClusterGroupSetDependencyEx<P2>(hgroupset: *const _HGROUPSET, hdependson: *const _HGROUPSET, lpszreason: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn RemoveClusterGroupSetDependencyEx(hgroupset : *const _HGROUPSET, hdependson : *const _HGROUPSET, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { RemoveClusterGroupSetDependencyEx(hgroupset, hdependson, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn RemoveClusterGroupToGroupSetDependency(hgroup: *const _HGROUP, hdependson: *const _HGROUPSET) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn RemoveClusterGroupToGroupSetDependency(hgroup : *const _HGROUP, hdependson : *const _HGROUPSET) -> u32);
    unsafe { RemoveClusterGroupToGroupSetDependency(hgroup, hdependson) }
}
#[inline]
pub unsafe fn RemoveClusterGroupToGroupSetDependencyEx<P2>(hgroup: *const _HGROUP, hdependson: *const _HGROUPSET, lpszreason: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn RemoveClusterGroupToGroupSetDependencyEx(hgroup : *const _HGROUP, hdependson : *const _HGROUPSET, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { RemoveClusterGroupToGroupSetDependencyEx(hgroup, hdependson, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn RemoveClusterNameAccount(hcluster: *const _HCLUSTER, bdeletecomputerobjects: bool) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn RemoveClusterNameAccount(hcluster : *const _HCLUSTER, bdeletecomputerobjects : windows_core::BOOL) -> u32);
    unsafe { RemoveClusterNameAccount(hcluster, bdeletecomputerobjects.into()) }
}
#[inline]
pub unsafe fn RemoveClusterResourceDependency(hresource: *const _HRESOURCE, hdependson: *const _HRESOURCE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn RemoveClusterResourceDependency(hresource : *const _HRESOURCE, hdependson : *const _HRESOURCE) -> u32);
    unsafe { RemoveClusterResourceDependency(hresource, hdependson) }
}
#[inline]
pub unsafe fn RemoveClusterResourceDependencyEx<P2>(hresource: *const _HRESOURCE, hdependson: *const _HRESOURCE, lpszreason: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn RemoveClusterResourceDependencyEx(hresource : *const _HRESOURCE, hdependson : *const _HRESOURCE, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { RemoveClusterResourceDependencyEx(hresource, hdependson, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn RemoveClusterResourceNode(hresource: *const _HRESOURCE, hnode: *const _HNODE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn RemoveClusterResourceNode(hresource : *const _HRESOURCE, hnode : *const _HNODE) -> u32);
    unsafe { RemoveClusterResourceNode(hresource, hnode) }
}
#[inline]
pub unsafe fn RemoveClusterResourceNodeEx<P2>(hresource: *const _HRESOURCE, hnode: *const _HNODE, lpszreason: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn RemoveClusterResourceNodeEx(hresource : *const _HRESOURCE, hnode : *const _HNODE, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { RemoveClusterResourceNodeEx(hresource, hnode, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn RemoveClusterStorageNode<P1>(hcluster: *mut _HCLUSTER, lpszclusterstorageenclosurename: P1, dwtimeout: u32, dwflags: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn RemoveClusterStorageNode(hcluster : *mut _HCLUSTER, lpszclusterstorageenclosurename : windows_core::PCWSTR, dwtimeout : u32, dwflags : u32) -> u32);
    unsafe { RemoveClusterStorageNode(hcluster as _, lpszclusterstorageenclosurename.param().abi(), dwtimeout, dwflags) }
}
#[inline]
pub unsafe fn RemoveCrossClusterGroupSetDependency<P1, P2>(hdependentgroupset: *const _HGROUPSET, lpremoteclustername: P1, lpremotegroupsetname: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn RemoveCrossClusterGroupSetDependency(hdependentgroupset : *const _HGROUPSET, lpremoteclustername : windows_core::PCWSTR, lpremotegroupsetname : windows_core::PCWSTR) -> u32);
    unsafe { RemoveCrossClusterGroupSetDependency(hdependentgroupset, lpremoteclustername.param().abi(), lpremotegroupsetname.param().abi()) }
}
#[inline]
pub unsafe fn RemoveResourceFromClusterSharedVolumes(hresource: *const _HRESOURCE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn RemoveResourceFromClusterSharedVolumes(hresource : *const _HRESOURCE) -> u32);
    unsafe { RemoveResourceFromClusterSharedVolumes(hresource) }
}
#[inline]
pub unsafe fn RepairClusterNameAccount(hcluster: *const _HCLUSTER, pconfig: *const REPAIR_CLUSTER_NAME_ACCOUNT_CONFIG, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: Option<*const core::ffi::c_void>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn RepairClusterNameAccount(hcluster : *const _HCLUSTER, pconfig : *const REPAIR_CLUSTER_NAME_ACCOUNT_CONFIG, pfnprogresscallback : PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg : *const core::ffi::c_void) -> u32);
    unsafe { RepairClusterNameAccount(hcluster, pconfig, pfnprogresscallback, pvcallbackarg.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RestartClusterResource(hresource: *const _HRESOURCE, dwflags: u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn RestartClusterResource(hresource : *const _HRESOURCE, dwflags : u32) -> u32);
    unsafe { RestartClusterResource(hresource, dwflags) }
}
#[inline]
pub unsafe fn RestartClusterResourceEx<P2>(hresource: *const _HRESOURCE, dwflags: u32, lpszreason: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn RestartClusterResourceEx(hresource : *const _HRESOURCE, dwflags : u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { RestartClusterResourceEx(hresource, dwflags, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn RestoreClusterDatabase<P0, P2>(lpszpathname: P0, bforce: bool, lpszquorumdriveletter: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn RestoreClusterDatabase(lpszpathname : windows_core::PCWSTR, bforce : windows_core::BOOL, lpszquorumdriveletter : windows_core::PCWSTR) -> u32);
    unsafe { RestoreClusterDatabase(lpszpathname.param().abi(), bforce.into(), lpszquorumdriveletter.param().abi()) }
}
#[inline]
pub unsafe fn ResumeClusterNode(hnode: *const _HNODE) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ResumeClusterNode(hnode : *const _HNODE) -> u32);
    unsafe { ResumeClusterNode(hnode) }
}
#[inline]
pub unsafe fn ResumeClusterNodeEx(hnode: *const _HNODE, eresumefailbacktype: CLUSTER_NODE_RESUME_FAILBACK_TYPE, dwresumeflagsreserved: u32) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn ResumeClusterNodeEx(hnode : *const _HNODE, eresumefailbacktype : CLUSTER_NODE_RESUME_FAILBACK_TYPE, dwresumeflagsreserved : u32) -> u32);
    unsafe { ResumeClusterNodeEx(hnode, eresumefailbacktype, dwresumeflagsreserved) }
}
#[inline]
pub unsafe fn ResumeClusterNodeEx2<P3>(hnode: *const _HNODE, eresumefailbacktype: CLUSTER_NODE_RESUME_FAILBACK_TYPE, dwresumeflagsreserved: u32, lpszreason: P3) -> u32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn ResumeClusterNodeEx2(hnode : *const _HNODE, eresumefailbacktype : CLUSTER_NODE_RESUME_FAILBACK_TYPE, dwresumeflagsreserved : u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { ResumeClusterNodeEx2(hnode, eresumefailbacktype, dwresumeflagsreserved, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn SetClusterGroupName<P1>(hgroup: *const _HGROUP, lpszgroupname: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterGroupName(hgroup : *const _HGROUP, lpszgroupname : windows_core::PCWSTR) -> u32);
    unsafe { SetClusterGroupName(hgroup, lpszgroupname.param().abi()) }
}
#[inline]
pub unsafe fn SetClusterGroupNameEx<P1, P2>(hgroup: *const _HGROUP, lpszgroupname: P1, lpszreason: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterGroupNameEx(hgroup : *const _HGROUP, lpszgroupname : windows_core::PCWSTR, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { SetClusterGroupNameEx(hgroup, lpszgroupname.param().abi(), lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn SetClusterGroupNodeList(hgroup: *const _HGROUP, nodelist: Option<&[HNODE]>) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn SetClusterGroupNodeList(hgroup : *const _HGROUP, nodecount : u32, nodelist : *const HNODE) -> u32);
    unsafe { SetClusterGroupNodeList(hgroup, nodelist.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(nodelist.map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[inline]
pub unsafe fn SetClusterGroupNodeListEx<P3>(hgroup: *mut _HGROUP, nodecount: u32, nodelist: *mut HNODE, lpszreason: P3) -> u32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterGroupNodeListEx(hgroup : *mut _HGROUP, nodecount : u32, nodelist : *mut HNODE, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { SetClusterGroupNodeListEx(hgroup as _, nodecount, nodelist as _, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn SetClusterGroupSetDependencyExpression<P1>(hgroupset: *const _HGROUPSET, lpszdependencyexprssion: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterGroupSetDependencyExpression(hgroupset : *const _HGROUPSET, lpszdependencyexprssion : windows_core::PCWSTR) -> u32);
    unsafe { SetClusterGroupSetDependencyExpression(hgroupset, lpszdependencyexprssion.param().abi()) }
}
#[inline]
pub unsafe fn SetClusterGroupSetDependencyExpressionEx<P1, P2>(hgroupset: *const _HGROUPSET, lpszdependencyexpression: P1, lpszreason: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterGroupSetDependencyExpressionEx(hgroupset : *const _HGROUPSET, lpszdependencyexpression : windows_core::PCWSTR, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { SetClusterGroupSetDependencyExpressionEx(hgroupset, lpszdependencyexpression.param().abi(), lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn SetClusterName<P1>(hcluster: *const _HCLUSTER, lpsznewclustername: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterName(hcluster : *const _HCLUSTER, lpsznewclustername : windows_core::PCWSTR) -> u32);
    unsafe { SetClusterName(hcluster, lpsznewclustername.param().abi()) }
}
#[inline]
pub unsafe fn SetClusterNameEx<P1, P2>(hcluster: *const _HCLUSTER, lpsznewclustername: P1, lpszreason: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterNameEx(hcluster : *const _HCLUSTER, lpsznewclustername : windows_core::PCWSTR, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { SetClusterNameEx(hcluster, lpsznewclustername.param().abi(), lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn SetClusterNetworkName<P1>(hnetwork: *const _HNETWORK, lpszname: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterNetworkName(hnetwork : *const _HNETWORK, lpszname : windows_core::PCWSTR) -> u32);
    unsafe { SetClusterNetworkName(hnetwork, lpszname.param().abi()) }
}
#[inline]
pub unsafe fn SetClusterNetworkNameEx<P1, P2>(hnetwork: *const _HNETWORK, lpszname: P1, lpszreason: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterNetworkNameEx(hnetwork : *const _HNETWORK, lpszname : windows_core::PCWSTR, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { SetClusterNetworkNameEx(hnetwork, lpszname.param().abi(), lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn SetClusterNetworkPriorityOrder(hcluster: *const _HCLUSTER, networklist: &[HNETWORK]) -> u32 {
    windows_core::link!("clusapi.dll" "system" fn SetClusterNetworkPriorityOrder(hcluster : *const _HCLUSTER, networkcount : u32, networklist : *const HNETWORK) -> u32);
    unsafe { SetClusterNetworkPriorityOrder(hcluster, networklist.len().try_into().unwrap(), core::mem::transmute(networklist.as_ptr())) }
}
#[inline]
pub unsafe fn SetClusterQuorumResource<P1>(hresource: *const _HRESOURCE, lpszdevicename: P1, dwmaxquologsize: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterQuorumResource(hresource : *const _HRESOURCE, lpszdevicename : windows_core::PCWSTR, dwmaxquologsize : u32) -> u32);
    unsafe { SetClusterQuorumResource(hresource, lpszdevicename.param().abi(), dwmaxquologsize) }
}
#[inline]
pub unsafe fn SetClusterQuorumResourceEx<P1, P3>(hresource: *const _HRESOURCE, lpszdevicename: P1, dwmaxquorumlogsize: u32, lpszreason: P3) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterQuorumResourceEx(hresource : *const _HRESOURCE, lpszdevicename : windows_core::PCWSTR, dwmaxquorumlogsize : u32, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { SetClusterQuorumResourceEx(hresource, lpszdevicename.param().abi(), dwmaxquorumlogsize, lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn SetClusterResourceDependencyExpression<P1>(hresource: *const _HRESOURCE, lpszdependencyexpression: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterResourceDependencyExpression(hresource : *const _HRESOURCE, lpszdependencyexpression : windows_core::PCWSTR) -> u32);
    unsafe { SetClusterResourceDependencyExpression(hresource, lpszdependencyexpression.param().abi()) }
}
#[inline]
pub unsafe fn SetClusterResourceName<P1>(hresource: *const _HRESOURCE, lpszresourcename: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterResourceName(hresource : *const _HRESOURCE, lpszresourcename : windows_core::PCWSTR) -> u32);
    unsafe { SetClusterResourceName(hresource, lpszresourcename.param().abi()) }
}
#[inline]
pub unsafe fn SetClusterResourceNameEx<P1, P2>(hresource: *const _HRESOURCE, lpszresourcename: P1, lpszreason: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterResourceNameEx(hresource : *const _HRESOURCE, lpszresourcename : windows_core::PCWSTR, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { SetClusterResourceNameEx(hresource, lpszresourcename.param().abi(), lpszreason.param().abi()) }
}
#[inline]
pub unsafe fn SetClusterServiceAccountPassword<P0, P1>(lpszclustername: P0, lpsznewpassword: P1, dwflags: u32, lpreturnstatusbuffer: Option<*mut CLUSTER_SET_PASSWORD_STATUS>, lpcbreturnstatusbuffersize: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetClusterServiceAccountPassword(lpszclustername : windows_core::PCWSTR, lpsznewpassword : windows_core::PCWSTR, dwflags : u32, lpreturnstatusbuffer : *mut CLUSTER_SET_PASSWORD_STATUS, lpcbreturnstatusbuffersize : *mut u32) -> u32);
    unsafe { SetClusterServiceAccountPassword(lpszclustername.param().abi(), lpsznewpassword.param().abi(), dwflags, lpreturnstatusbuffer.unwrap_or(core::mem::zeroed()) as _, lpcbreturnstatusbuffersize as _) }
}
#[inline]
pub unsafe fn SetGroupDependencyExpression<P1>(hgroup: *const _HGROUP, lpszdependencyexpression: P1) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetGroupDependencyExpression(hgroup : *const _HGROUP, lpszdependencyexpression : windows_core::PCWSTR) -> u32);
    unsafe { SetGroupDependencyExpression(hgroup, lpszdependencyexpression.param().abi()) }
}
#[inline]
pub unsafe fn SetGroupDependencyExpressionEx<P1, P2>(hgroup: *const _HGROUP, lpszdependencyexpression: P1, lpszreason: P2) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clusapi.dll" "system" fn SetGroupDependencyExpressionEx(hgroup : *const _HGROUP, lpszdependencyexpression : windows_core::PCWSTR, lpszreason : windows_core::PCWSTR) -> u32);
    unsafe { SetGroupDependencyExpressionEx(hgroup, lpszdependencyexpression.param().abi(), lpszreason.param().abi()) }
}
pub const BitLockerDecrypted: u32 = 4;
pub const BitLockerDecrypting: u32 = 16;
pub const BitLockerEnabled: u32 = 1;
pub const BitLockerFlagsAll: u32 = 253;
pub const BitLockerPaused: u32 = 64;
pub const BitLockerStopped: u32 = 128;
pub const BitlockerEncrypted: u32 = 8;
pub const BitlockerEncrypting: u32 = 32;
pub const CA_UPGRADE_VERSION: u32 = 1;
pub type CGAFT = i32;
pub const CLCTL_ADD_CRYPTO_CHECKPOINT: CLCTL_CODES = 4194478;
pub const CLCTL_ADD_CRYPTO_CHECKPOINT_EX: CLCTL_CODES = 4195030;
pub const CLCTL_ADD_DEPENDENCY: CLCTL_CODES = 5242898;
pub const CLCTL_ADD_OWNER: CLCTL_CODES = 5242906;
pub const CLCTL_ADD_REGISTRY_CHECKPOINT: CLCTL_CODES = 4194466;
pub const CLCTL_ADD_REGISTRY_CHECKPOINT_32BIT: CLCTL_CODES = 4194498;
pub const CLCTL_ADD_REGISTRY_CHECKPOINT_64BIT: CLCTL_CODES = 4194494;
pub const CLCTL_BATCH_BLOCK_KEY: CLCTL_CODES = 574;
pub const CLCTL_BATCH_UNBLOCK_KEY: CLCTL_CODES = 577;
pub const CLCTL_BLOCK_GEM_SEND_RECV: CLCTL_CODES = 717;
pub const CLCTL_CHECK_DRAIN_VETO: CLCTL_CODES = 1057069;
pub const CLCTL_CHECK_VOTER_DOWN: CLCTL_CODES = 73;
pub const CLCTL_CHECK_VOTER_DOWN_WITNESS: CLCTL_CODES = 113;
pub const CLCTL_CHECK_VOTER_EVICT: CLCTL_CODES = 69;
pub const CLCTL_CHECK_VOTER_EVICT_WITNESS: CLCTL_CODES = 109;
pub const CLCTL_CLEAR_NODE_CONNECTION_INFO: CLCTL_CODES = 4195078;
pub const CLCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS: CLCTL_CODES = 8417;
pub const CLCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS_WITH_KEY: CLCTL_CODES = 8433;
pub const CLCTL_CLOUD_WITNESS_RESOURCE_UPDATE_KEY: CLCTL_CODES = 4202742;
pub const CLCTL_CLOUD_WITNESS_RESOURCE_UPDATE_TOKEN: CLCTL_CODES = 4202726;
pub const CLCTL_CLUSTER_BASE: u32 = 0;
pub const CLCTL_CLUSTER_NAME_CHANGED: CLCTL_CODES = 5242922;
pub const CLCTL_CLUSTER_VERSION_CHANGED: CLCTL_CODES = 5242926;
pub type CLCTL_CODES = i32;
pub const CLCTL_DELETE: CLCTL_CODES = 5242886;
pub const CLCTL_DELETE_CRYPTO_CHECKPOINT: CLCTL_CODES = 4194482;
pub const CLCTL_DELETE_REGISTRY_CHECKPOINT: CLCTL_CODES = 4194470;
pub const CLCTL_DISABLE_SHARED_VOLUME_DIRECTIO: CLCTL_CODES = 4194958;
pub const CLCTL_ENABLE_SHARED_VOLUME_DIRECTIO: CLCTL_CODES = 4194954;
pub const CLCTL_ENUM_AFFINITY_RULE_NAMES: CLCTL_CODES = 11741;
pub const CLCTL_ENUM_COMMON_PROPERTIES: CLCTL_CODES = 81;
pub const CLCTL_ENUM_PRIVATE_PROPERTIES: CLCTL_CODES = 121;
pub const CLCTL_EVICT_NODE: CLCTL_CODES = 5242894;
pub const CLCTL_FILESERVER_SHARE_ADD: CLCTL_CODES = 4194886;
pub const CLCTL_FILESERVER_SHARE_DEL: CLCTL_CODES = 4194890;
pub const CLCTL_FILESERVER_SHARE_MODIFY: CLCTL_CODES = 4194894;
pub const CLCTL_FILESERVER_SHARE_REPORT: CLCTL_CODES = 593;
pub const CLCTL_FIXUP_ON_UPGRADE: CLCTL_CODES = 5242930;
pub const CLCTL_FORCE_DB_FLUSH: CLCTL_CODES = 4206054;
pub const CLCTL_FORCE_QUORUM: CLCTL_CODES = 5242950;
pub const CLCTL_FSWITNESS_GET_EPOCH_INFO: CLCTL_CODES = 1048669;
pub const CLCTL_FSWITNESS_RELEASE_LOCK: CLCTL_CODES = 5242982;
pub const CLCTL_FSWITNESS_SET_EPOCH_INFO: CLCTL_CODES = 5242978;
pub const CLCTL_GET_ARB_TIMEOUT: CLCTL_CODES = 21;
pub const CLCTL_GET_CHARACTERISTICS: CLCTL_CODES = 5;
pub const CLCTL_GET_CLASS_INFO: CLCTL_CODES = 13;
pub const CLCTL_GET_CLUSDB_TIMESTAMP: CLCTL_CODES = 681;
pub const CLCTL_GET_CLUSTER_SERVICE_ACCOUNT_NAME: CLCTL_CODES = 65;
pub const CLCTL_GET_COMMON_PROPERTIES: CLCTL_CODES = 89;
pub const CLCTL_GET_COMMON_PROPERTY_FMTS: CLCTL_CODES = 101;
pub const CLCTL_GET_COMMON_RESOURCE_PROPERTY_FMTS: CLCTL_CODES = 105;
pub const CLCTL_GET_CRYPTO_CHECKPOINTS: CLCTL_CODES = 181;
pub const CLCTL_GET_DNS_NAME: CLCTL_CODES = 373;
pub const CLCTL_GET_FAILURE_INFO: CLCTL_CODES = 25;
pub const CLCTL_GET_FLAGS: CLCTL_CODES = 9;
pub const CLCTL_GET_FQDN: CLCTL_CODES = 61;
pub const CLCTL_GET_GEMID_VECTOR: CLCTL_CODES = 721;
pub const CLCTL_GET_GUM_LOCK_OWNER: CLCTL_CODES = 697;
pub const CLCTL_GET_ID: CLCTL_CODES = 57;
pub const CLCTL_GET_INFRASTRUCTURE_SOFS_BUFFER: CLCTL_CODES = 11657;
pub const CLCTL_GET_LOADBAL_PROCESS_LIST: CLCTL_CODES = 201;
pub const CLCTL_GET_NAME: CLCTL_CODES = 41;
pub const CLCTL_GET_NETWORK: CLCTL_CODES = 53;
pub const CLCTL_GET_NETWORK_NAME: CLCTL_CODES = 361;
pub const CLCTL_GET_NODE: CLCTL_CODES = 49;
pub const CLCTL_GET_NODES_IN_FD: CLCTL_CODES = 11745;
pub const CLCTL_GET_NODE_NETWORK_CONNECTIVITY: CLCTL_CODES = 797;
pub const CLCTL_GET_OPERATION_CONTEXT: CLCTL_CODES = 1057001;
pub const CLCTL_GET_PRIVATE_PROPERTIES: CLCTL_CODES = 129;
pub const CLCTL_GET_PRIVATE_PROPERTY_FMTS: CLCTL_CODES = 141;
pub const CLCTL_GET_PRIVATE_RESOURCE_PROPERTY_FMTS: CLCTL_CODES = 145;
pub const CLCTL_GET_REGISTRY_CHECKPOINTS: CLCTL_CODES = 169;
pub const CLCTL_GET_REQUIRED_DEPENDENCIES: CLCTL_CODES = 17;
pub const CLCTL_GET_RESOURCE_TYPE: CLCTL_CODES = 45;
pub const CLCTL_GET_RO_COMMON_PROPERTIES: CLCTL_CODES = 85;
pub const CLCTL_GET_RO_PRIVATE_PROPERTIES: CLCTL_CODES = 125;
pub const CLCTL_GET_SHARED_VOLUME_ID: CLCTL_CODES = 657;
pub const CLCTL_GET_STATE_CHANGE_TIME: CLCTL_CODES = 11613;
pub const CLCTL_GET_STORAGE_CONFIGURATION: CLCTL_CODES = 741;
pub const CLCTL_GET_STORAGE_CONFIG_ATTRIBUTES: CLCTL_CODES = 745;
pub const CLCTL_GET_STUCK_NODES: CLCTL_CODES = 701;
pub const CLCTL_GLOBAL_MASK: u32 = 8388608;
pub const CLCTL_GLOBAL_SHIFT: u32 = 23;
pub const CLCTL_GROUPSET_GET_GROUPS: CLCTL_CODES = 11633;
pub const CLCTL_GROUPSET_GET_PROVIDER_GROUPS: CLCTL_CODES = 11637;
pub const CLCTL_GROUPSET_GET_PROVIDER_GROUPSETS: CLCTL_CODES = 11641;
pub const CLCTL_GROUP_GET_LAST_MOVE_TIME: CLCTL_CODES = 729;
pub const CLCTL_GROUP_GET_PROVIDER_GROUPS: CLCTL_CODES = 11645;
pub const CLCTL_GROUP_GET_PROVIDER_GROUPSETS: CLCTL_CODES = 11649;
pub const CLCTL_GROUP_SET_CCF_FROM_MASTER: CLCTL_CODES = 4205958;
pub const CLCTL_HOLD_IO: CLCTL_CODES = 5242942;
pub const CLCTL_INITIALIZE: CLCTL_CODES = 5242954;
pub const CLCTL_INJECT_GEM_FAULT: CLCTL_CODES = 705;
pub const CLCTL_INSTALL_NODE: CLCTL_CODES = 5242890;
pub const CLCTL_INTERNAL_MASK: u32 = 1048576;
pub const CLCTL_INTERNAL_SHIFT: u32 = 20;
pub const CLCTL_INTRODUCE_GEM_REPAIR_DELAY: CLCTL_CODES = 709;
pub const CLCTL_IPADDRESS_RELEASE_LEASE: CLCTL_CODES = 4194754;
pub const CLCTL_IPADDRESS_RENEW_LEASE: CLCTL_CODES = 4194750;
pub const CLCTL_IS_FEATURE_INSTALLED: CLCTL_CODES = 753;
pub const CLCTL_IS_QUORUM_BLOCKED: CLCTL_CODES = 689;
pub const CLCTL_IS_S2D_FEATURE_SUPPORTED: CLCTL_CODES = 757;
pub const CLCTL_JOINING_GROUP: CLCTL_CODES = 5242970;
pub const CLCTL_LEAVING_GROUP: CLCTL_CODES = 5242966;
pub const CLCTL_MODIFY_MASK: u32 = 4194304;
pub const CLCTL_MODIFY_SHIFT: u32 = 22;
pub const CLCTL_NETNAME_CREDS_NOTIFYCAM: CLCTL_CODES = 5242986;
pub const CLCTL_NETNAME_DELETE_CO: CLCTL_CODES = 382;
pub const CLCTL_NETNAME_GET_OU_FOR_VCO: CLCTL_CODES = 4194926;
pub const CLCTL_NETNAME_GET_VIRTUAL_SERVER_TOKEN: CLCTL_CODES = 365;
pub const CLCTL_NETNAME_REGISTER_DNS_RECORDS: CLCTL_CODES = 370;
pub const CLCTL_NETNAME_REPAIR_VCO: CLCTL_CODES = 397;
pub const CLCTL_NETNAME_RESET_VCO: CLCTL_CODES = 389;
pub const CLCTL_NETNAME_SET_PWD_INFO: CLCTL_CODES = 378;
pub const CLCTL_NETNAME_SET_PWD_INFOEX: CLCTL_CODES = 794;
pub const CLCTL_NETNAME_VALIDATE_VCO: CLCTL_CODES = 385;
pub const CLCTL_NOTIFY_DRAIN_COMPLETE: CLCTL_CODES = 1057073;
pub const CLCTL_NOTIFY_INFRASTRUCTURE_SOFS_CHANGED: CLCTL_CODES = 4205970;
pub const CLCTL_NOTIFY_MONITOR_SHUTTING_DOWN: CLCTL_CODES = 1048705;
pub const CLCTL_NOTIFY_OWNER_CHANGE: CLCTL_CODES = 5251362;
pub const CLCTL_NOTIFY_QUORUM_STATUS: CLCTL_CODES = 5243006;
pub const CLCTL_POOL_GET_DRIVE_INFO: CLCTL_CODES = 693;
pub const CLCTL_PROVIDER_STATE_CHANGE: CLCTL_CODES = 5242962;
pub const CLCTL_QUERY_DELETE: CLCTL_CODES = 441;
pub const CLCTL_QUERY_MAINTENANCE_MODE: CLCTL_CODES = 481;
pub const CLCTL_RELOAD_AUTOLOGGER_CONFIG: CLCTL_CODES = 11730;
pub const CLCTL_REMOVE_DEPENDENCY: CLCTL_CODES = 5242902;
pub const CLCTL_REMOVE_NODE: CLCTL_CODES = 4195054;
pub const CLCTL_REMOVE_OWNER: CLCTL_CODES = 5242910;
pub const CLCTL_REPLICATION_ADD_REPLICATION_GROUP: CLCTL_CODES = 8514;
pub const CLCTL_REPLICATION_GET_ELIGIBLE_LOGDISKS: CLCTL_CODES = 8521;
pub const CLCTL_REPLICATION_GET_ELIGIBLE_SOURCE_DATADISKS: CLCTL_CODES = 8529;
pub const CLCTL_REPLICATION_GET_ELIGIBLE_TARGET_DATADISKS: CLCTL_CODES = 8525;
pub const CLCTL_REPLICATION_GET_LOG_INFO: CLCTL_CODES = 8517;
pub const CLCTL_REPLICATION_GET_LOG_VOLUME: CLCTL_CODES = 8541;
pub const CLCTL_REPLICATION_GET_REPLICATED_DISKS: CLCTL_CODES = 8533;
pub const CLCTL_REPLICATION_GET_REPLICATED_PARTITION_INFO: CLCTL_CODES = 8549;
pub const CLCTL_REPLICATION_GET_REPLICA_VOLUMES: CLCTL_CODES = 8537;
pub const CLCTL_REPLICATION_GET_RESOURCE_GROUP: CLCTL_CODES = 8545;
pub const CLCTL_RESOURCE_PREPARE_UPGRADE: CLCTL_CODES = 4202730;
pub const CLCTL_RESOURCE_UPGRADE_COMPLETED: CLCTL_CODES = 4202734;
pub const CLCTL_RESOURCE_UPGRADE_DLL: CLCTL_CODES = 4194490;
pub const CLCTL_RESUME_IO: CLCTL_CODES = 5242946;
pub const CLCTL_RLUA_GET_PWD: CLCTL_CODES = 802;
pub const CLCTL_RW_MODIFY_NOOP: CLCTL_CODES = 4194990;
pub const CLCTL_SCALEOUT_COMMAND: CLCTL_CODES = 4205974;
pub const CLCTL_SCALEOUT_CONTROL: CLCTL_CODES = 4205978;
pub const CLCTL_SCALEOUT_GET_CLUSTERS: CLCTL_CODES = 4205981;
pub const CLCTL_SEND_DUMMY_GEM_MESSAGES: CLCTL_CODES = 713;
pub const CLCTL_SET_ACCOUNT_ACCESS: CLCTL_CODES = 4194546;
pub const CLCTL_SET_CLUSTER_S2D_CACHE_METADATA_RESERVE_BYTES: CLCTL_CODES = 4205934;
pub const CLCTL_SET_CLUSTER_S2D_ENABLED: CLCTL_CODES = 4205922;
pub const CLCTL_SET_COMMON_PROPERTIES: CLCTL_CODES = 4194398;
pub const CLCTL_SET_CSV_MAINTENANCE_MODE: CLCTL_CODES = 4194966;
pub const CLCTL_SET_DNS_DOMAIN: CLCTL_CODES = 4195082;
pub const CLCTL_SET_INFRASTRUCTURE_SOFS_BUFFER: CLCTL_CODES = 4205966;
pub const CLCTL_SET_MAINTENANCE_MODE: CLCTL_CODES = 4194790;
pub const CLCTL_SET_NAME: CLCTL_CODES = 5242918;
pub const CLCTL_SET_PRIVATE_PROPERTIES: CLCTL_CODES = 4194438;
pub const CLCTL_SET_SHARED_VOLUME_BACKUP_MODE: CLCTL_CODES = 4194970;
pub const CLCTL_SET_STORAGE_CONFIGURATION: CLCTL_CODES = 4195042;
pub const CLCTL_SHUTDOWN: CLCTL_CODES = 77;
pub const CLCTL_STARTING_PHASE1: CLCTL_CODES = 5242934;
pub const CLCTL_STARTING_PHASE2: CLCTL_CODES = 5242938;
pub const CLCTL_STATE_CHANGE_REASON: CLCTL_CODES = 5242958;
pub const CLCTL_STORAGE_GET_AVAILABLE_DISKS: CLCTL_CODES = 405;
pub const CLCTL_STORAGE_GET_AVAILABLE_DISKS_EX: CLCTL_CODES = 501;
pub const CLCTL_STORAGE_GET_AVAILABLE_DISKS_EX2_INT: CLCTL_CODES = 8161;
pub const CLCTL_STORAGE_GET_CLUSBFLT_PATHINFO: CLCTL_CODES = 769;
pub const CLCTL_STORAGE_GET_CLUSBFLT_PATHS: CLCTL_CODES = 765;
pub const CLCTL_STORAGE_GET_CLUSPORT_DISK_COUNT: CLCTL_CODES = 509;
pub const CLCTL_STORAGE_GET_DIRTY: CLCTL_CODES = 537;
pub const CLCTL_STORAGE_GET_DISKID: CLCTL_CODES = 517;
pub const CLCTL_STORAGE_GET_DISK_INFO: CLCTL_CODES = 401;
pub const CLCTL_STORAGE_GET_DISK_INFO_EX: CLCTL_CODES = 497;
pub const CLCTL_STORAGE_GET_DISK_INFO_EX2: CLCTL_CODES = 505;
pub const CLCTL_STORAGE_GET_DISK_NUMBER_INFO: CLCTL_CODES = 417;
pub const CLCTL_STORAGE_GET_DRIVELETTERS: CLCTL_CODES = 493;
pub const CLCTL_STORAGE_GET_MOUNTPOINTS: CLCTL_CODES = 529;
pub const CLCTL_STORAGE_GET_PHYSICAL_DISK_INFO: CLCTL_CODES = 761;
pub const CLCTL_STORAGE_GET_RESOURCEID: CLCTL_CODES = 557;
pub const CLCTL_STORAGE_GET_SHARED_VOLUME_INFO: CLCTL_CODES = 549;
pub const CLCTL_STORAGE_GET_SHARED_VOLUME_PARTITION_NAMES: CLCTL_CODES = 669;
pub const CLCTL_STORAGE_GET_SHARED_VOLUME_STATES: CLCTL_CODES = 4194978;
pub const CLCTL_STORAGE_IS_CLUSTERABLE: CLCTL_CODES = 521;
pub const CLCTL_STORAGE_IS_CSV_FILE: CLCTL_CODES = 553;
pub const CLCTL_STORAGE_IS_PATH_VALID: CLCTL_CODES = 409;
pub const CLCTL_STORAGE_IS_SHARED_VOLUME: CLCTL_CODES = 677;
pub const CLCTL_STORAGE_REMAP_DRIVELETTER: CLCTL_CODES = 513;
pub const CLCTL_STORAGE_REMOVE_VM_OWNERSHIP: CLCTL_CODES = 4194830;
pub const CLCTL_STORAGE_RENAME_SHARED_VOLUME: CLCTL_CODES = 11734;
pub const CLCTL_STORAGE_RENAME_SHARED_VOLUME_GUID: CLCTL_CODES = 11738;
pub const CLCTL_STORAGE_SET_DRIVELETTER: CLCTL_CODES = 4194794;
pub const CLCTL_STORAGE_SYNC_CLUSDISK_DB: CLCTL_CODES = 4194718;
pub const CLCTL_UNDELETE: CLCTL_CODES = 5243014;
pub const CLCTL_UNKNOWN: CLCTL_CODES = 0;
pub const CLCTL_USER_BASE: u32 = 2097152;
pub const CLCTL_USER_MASK: u32 = 2097152;
pub const CLCTL_USER_SHIFT: u32 = 21;
pub const CLCTL_VALIDATE_CHANGE_GROUP: CLCTL_CODES = 1057061;
pub const CLCTL_VALIDATE_COMMON_PROPERTIES: CLCTL_CODES = 97;
pub const CLCTL_VALIDATE_DIRECTORY: CLCTL_CODES = 569;
pub const CLCTL_VALIDATE_NETNAME: CLCTL_CODES = 565;
pub const CLCTL_VALIDATE_PATH: CLCTL_CODES = 561;
pub const CLCTL_VALIDATE_PRIVATE_PROPERTIES: CLCTL_CODES = 137;
pub const CLOUD_WITNESS_CONTAINER_NAME: windows_core::PCWSTR = windows_core::w!("msft-cloud-witness");
pub const CLUSAPI_ALL_ACCESS: u32 = 3;
pub const CLUSAPI_CHANGE_ACCESS: u32 = 2;
pub const CLUSAPI_CHANGE_RESOURCE_GROUP_FORCE_MOVE_TO_CSV: u32 = 1;
pub const CLUSAPI_GROUP_MOVE_FAILBACK: u32 = 16;
pub const CLUSAPI_GROUP_MOVE_HIGH_PRIORITY_START: u32 = 8;
pub const CLUSAPI_GROUP_MOVE_IGNORE_AFFINITY_RULE: u32 = 32;
pub const CLUSAPI_GROUP_MOVE_IGNORE_RESOURCE_STATUS: u32 = 1;
pub const CLUSAPI_GROUP_MOVE_QUEUE_ENABLED: u32 = 4;
pub const CLUSAPI_GROUP_MOVE_RETURN_TO_SOURCE_NODE_ON_ERROR: u32 = 2;
pub const CLUSAPI_GROUP_OFFLINE_IGNORE_RESOURCE_STATUS: u32 = 1;
pub const CLUSAPI_GROUP_ONLINE_BEST_POSSIBLE_NODE: u32 = 4;
pub const CLUSAPI_GROUP_ONLINE_IGNORE_AFFINITY_RULE: u32 = 8;
pub const CLUSAPI_GROUP_ONLINE_IGNORE_RESOURCE_STATUS: u32 = 1;
pub const CLUSAPI_GROUP_ONLINE_SYNCHRONOUS: u32 = 2;
pub const CLUSAPI_NODE_AVOID_PLACEMENT: u32 = 2;
pub const CLUSAPI_NODE_PAUSE_REMAIN_ON_PAUSED_NODE_ON_MOVE_ERROR: u32 = 1;
pub const CLUSAPI_NODE_PAUSE_RETRY_DRAIN_ON_FAILURE: u32 = 4;
pub const CLUSAPI_NODE_RESUME_FAILBACK_PINNED_VMS_ONLY: u32 = 4;
pub const CLUSAPI_NODE_RESUME_FAILBACK_STORAGE: u32 = 1;
pub const CLUSAPI_NODE_RESUME_FAILBACK_VMS: u32 = 2;
pub const CLUSAPI_NODE_RESUME_FAILBACK_VMS_FORCEFULLY: u32 = 8;
pub const CLUSAPI_NO_ACCESS: u32 = 4;
pub const CLUSAPI_READ_ACCESS: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CLUSAPI_REASON_HANDLER {
    pub lpParameter: *mut core::ffi::c_void,
    pub pfnHandler: PCLUSAPI_PFN_REASON_HANDLER,
}
impl Default for CLUSAPI_REASON_HANDLER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLUSAPI_RESOURCE_OFFLINE_DO_NOT_UPDATE_PERSISTENT_STATE: u32 = 4;
pub const CLUSAPI_RESOURCE_OFFLINE_FORCE_WITH_TERMINATION: u32 = 2;
pub const CLUSAPI_RESOURCE_OFFLINE_IGNORE_RESOURCE_STATUS: u32 = 1;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_BEING_DELETED: u32 = 8;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_BEING_RESTARTED: u32 = 16;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_MOVING: u32 = 2;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_NONE: u32 = 0;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_PREEMPTED: u32 = 32;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_SHUTTING_DOWN: u32 = 64;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_UNKNOWN: u32 = 1;
pub const CLUSAPI_RESOURCE_OFFLINE_REASON_USER_REQUESTED: u32 = 4;
pub const CLUSAPI_RESOURCE_ONLINE_BEST_POSSIBLE_NODE: u32 = 8;
pub const CLUSAPI_RESOURCE_ONLINE_DO_NOT_UPDATE_PERSISTENT_STATE: u32 = 2;
pub const CLUSAPI_RESOURCE_ONLINE_IGNORE_AFFINITY_RULE: u32 = 32;
pub const CLUSAPI_RESOURCE_ONLINE_IGNORE_RESOURCE_STATUS: u32 = 1;
pub const CLUSAPI_RESOURCE_ONLINE_NECESSARY_FOR_QUORUM: u32 = 4;
pub const CLUSAPI_VALID_CHANGE_RESOURCE_GROUP_FLAGS: u32 = 1;
pub const CLUSAPI_VERSION: u32 = 3079;
pub const CLUSAPI_VERSION_CU: u32 = 3075;
pub const CLUSAPI_VERSION_DT: u32 = 3079;
pub const CLUSAPI_VERSION_GA: u32 = 3077;
pub const CLUSAPI_VERSION_GE: u32 = 3078;
pub const CLUSAPI_VERSION_NI: u32 = 2572;
pub const CLUSAPI_VERSION_RS3: u32 = 2560;
pub const CLUSAPI_VERSION_SERVER2008: u32 = 1536;
pub const CLUSAPI_VERSION_SERVER2008R2: u32 = 1792;
pub const CLUSAPI_VERSION_WINDOWS8: u32 = 1793;
pub const CLUSAPI_VERSION_WINDOWSBLUE: u32 = 1794;
pub const CLUSAPI_VERSION_WINTHRESHOLD: u32 = 1795;
pub const CLUSAPI_VERSION_ZN: u32 = 3076;
pub const CLUSCTL_ACCESS_MODE_MASK: u32 = 3;
pub const CLUSCTL_ACCESS_SHIFT: u32 = 0;
pub type CLUSCTL_AFFINITYRULE_CODES = i32;
pub const CLUSCTL_AFFINITYRULE_GET_COMMON_PROPERTIES: CLUSCTL_AFFINITYRULE_CODES = 150995033;
pub const CLUSCTL_AFFINITYRULE_GET_GROUPNAMES: CLUSCTL_AFFINITYRULE_CODES = 151006577;
pub const CLUSCTL_AFFINITYRULE_GET_ID: CLUSCTL_AFFINITYRULE_CODES = 150995001;
pub const CLUSCTL_AFFINITYRULE_GET_RO_COMMON_PROPERTIES: CLUSCTL_AFFINITYRULE_CODES = 150995029;
pub const CLUSCTL_AFFINITYRULE_SET_COMMON_PROPERTIES: CLUSCTL_AFFINITYRULE_CODES = 155189342;
pub const CLUSCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS: CLUSCTL_RESOURCE_TYPE_CODES = 33562849;
pub const CLUSCTL_CLOUD_WITNESS_RESOURCE_TYPE_VALIDATE_CREDENTIALS_WITH_KEY: CLUSCTL_RESOURCE_TYPE_CODES = 33562865;
pub const CLUSCTL_CLOUD_WITNESS_RESOURCE_UPDATE_KEY: CLUSCTL_RESOURCE_CODES = 20979958;
pub const CLUSCTL_CLOUD_WITNESS_RESOURCE_UPDATE_TOKEN: CLUSCTL_RESOURCE_CODES = 20979942;
pub const CLUSCTL_CLUSTER_BATCH_BLOCK_KEY: CLUSCTL_CLUSTER_CODES = 117441086;
pub const CLUSCTL_CLUSTER_BATCH_UNBLOCK_KEY: CLUSCTL_CLUSTER_CODES = 117441089;
pub const CLUSCTL_CLUSTER_CHECK_VOTER_DOWN: CLUSCTL_CLUSTER_CODES = 117440585;
pub const CLUSCTL_CLUSTER_CHECK_VOTER_DOWN_WITNESS: CLUSCTL_CLUSTER_CODES = 117440625;
pub const CLUSCTL_CLUSTER_CHECK_VOTER_EVICT: CLUSCTL_CLUSTER_CODES = 117440581;
pub const CLUSCTL_CLUSTER_CHECK_VOTER_EVICT_WITNESS: CLUSCTL_CLUSTER_CODES = 117440621;
pub const CLUSCTL_CLUSTER_CLEAR_NODE_CONNECTION_INFO: CLUSCTL_CLUSTER_CODES = 121635590;
pub type CLUSCTL_CLUSTER_CODES = i32;
pub const CLUSCTL_CLUSTER_ENUM_AFFINITY_RULE_NAMES: CLUSCTL_CLUSTER_CODES = 117452253;
pub const CLUSCTL_CLUSTER_ENUM_COMMON_PROPERTIES: CLUSCTL_CLUSTER_CODES = 117440593;
pub const CLUSCTL_CLUSTER_ENUM_PRIVATE_PROPERTIES: CLUSCTL_CLUSTER_CODES = 117440633;
pub const CLUSCTL_CLUSTER_FORCE_FLUSH_DB: CLUSCTL_CLUSTER_CODES = 121646566;
pub const CLUSCTL_CLUSTER_GET_CLMUSR_TOKEN: CLUSCTL_CLUSTER_CODES = 117440877;
pub const CLUSCTL_CLUSTER_GET_CLUSDB_TIMESTAMP: CLUSCTL_CLUSTER_CODES = 117441193;
pub const CLUSCTL_CLUSTER_GET_COMMON_PROPERTIES: CLUSCTL_CLUSTER_CODES = 117440601;
pub const CLUSCTL_CLUSTER_GET_COMMON_PROPERTY_FMTS: CLUSCTL_CLUSTER_CODES = 117440613;
pub const CLUSCTL_CLUSTER_GET_FQDN: CLUSCTL_CLUSTER_CODES = 117440573;
pub const CLUSCTL_CLUSTER_GET_GUM_LOCK_OWNER: CLUSCTL_CLUSTER_CODES = 117441209;
pub const CLUSCTL_CLUSTER_GET_NODES_IN_FD: CLUSCTL_CLUSTER_CODES = 117452257;
pub const CLUSCTL_CLUSTER_GET_PRIVATE_PROPERTIES: CLUSCTL_CLUSTER_CODES = 117440641;
pub const CLUSCTL_CLUSTER_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_CLUSTER_CODES = 117440653;
pub const CLUSCTL_CLUSTER_GET_RO_COMMON_PROPERTIES: CLUSCTL_CLUSTER_CODES = 117440597;
pub const CLUSCTL_CLUSTER_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_CLUSTER_CODES = 117440637;
pub const CLUSCTL_CLUSTER_GET_SHARED_VOLUME_ID: CLUSCTL_CLUSTER_CODES = 117441169;
pub const CLUSCTL_CLUSTER_GET_STORAGE_CONFIGURATION: CLUSCTL_CLUSTER_CODES = 117441253;
pub const CLUSCTL_CLUSTER_GET_STORAGE_CONFIG_ATTRIBUTES: CLUSCTL_CLUSTER_CODES = 117441257;
pub const CLUSCTL_CLUSTER_RELOAD_AUTOLOGGER_CONFIG: CLUSCTL_CLUSTER_CODES = 117452242;
pub const CLUSCTL_CLUSTER_REMOVE_NODE: CLUSCTL_CLUSTER_CODES = 121635566;
pub const CLUSCTL_CLUSTER_SET_ACCOUNT_ACCESS: CLUSCTL_CLUSTER_CODES = 121635058;
pub const CLUSCTL_CLUSTER_SET_CLUSTER_S2D_CACHE_METADATA_RESERVE_BYTES: CLUSCTL_CLUSTER_CODES = 121646446;
pub const CLUSCTL_CLUSTER_SET_CLUSTER_S2D_ENABLED: CLUSCTL_CLUSTER_CODES = 121646434;
pub const CLUSCTL_CLUSTER_SET_COMMON_PROPERTIES: CLUSCTL_CLUSTER_CODES = 121634910;
pub const CLUSCTL_CLUSTER_SET_DNS_DOMAIN: CLUSCTL_CLUSTER_CODES = 121635594;
pub const CLUSCTL_CLUSTER_SET_PRIVATE_PROPERTIES: CLUSCTL_CLUSTER_CODES = 121634950;
pub const CLUSCTL_CLUSTER_SET_STORAGE_CONFIGURATION: CLUSCTL_CLUSTER_CODES = 121635554;
pub const CLUSCTL_CLUSTER_SHUTDOWN: CLUSCTL_CLUSTER_CODES = 117440589;
pub const CLUSCTL_CLUSTER_STORAGE_RENAME_SHARED_VOLUME: CLUSCTL_CLUSTER_CODES = 117452246;
pub const CLUSCTL_CLUSTER_STORAGE_RENAME_SHARED_VOLUME_GUID: CLUSCTL_CLUSTER_CODES = 117452250;
pub const CLUSCTL_CLUSTER_UNKNOWN: CLUSCTL_CLUSTER_CODES = 117440512;
pub const CLUSCTL_CLUSTER_VALIDATE_COMMON_PROPERTIES: CLUSCTL_CLUSTER_CODES = 117440609;
pub const CLUSCTL_CLUSTER_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_CLUSTER_CODES = 117440649;
pub const CLUSCTL_CONTROL_CODE_MASK: u32 = 4194303;
pub const CLUSCTL_FUNCTION_SHIFT: u32 = 2;
pub type CLUSCTL_GROUPSET_CODES = i32;
pub const CLUSCTL_GROUPSET_GET_COMMON_PROPERTIES: CLUSCTL_GROUPSET_CODES = 134217817;
pub const CLUSCTL_GROUPSET_GET_GROUPS: CLUSCTL_GROUPSET_CODES = 134229361;
pub const CLUSCTL_GROUPSET_GET_ID: CLUSCTL_GROUPSET_CODES = 134217785;
pub const CLUSCTL_GROUPSET_GET_PROVIDER_GROUPS: CLUSCTL_GROUPSET_CODES = 134229365;
pub const CLUSCTL_GROUPSET_GET_PROVIDER_GROUPSETS: CLUSCTL_GROUPSET_CODES = 134229369;
pub const CLUSCTL_GROUPSET_GET_RO_COMMON_PROPERTIES: CLUSCTL_GROUPSET_CODES = 134217813;
pub const CLUSCTL_GROUPSET_SET_COMMON_PROPERTIES: CLUSCTL_GROUPSET_CODES = 138412126;
pub type CLUSCTL_GROUP_CODES = i32;
pub const CLUSCTL_GROUP_ENUM_COMMON_PROPERTIES: CLUSCTL_GROUP_CODES = 50331729;
pub const CLUSCTL_GROUP_ENUM_PRIVATE_PROPERTIES: CLUSCTL_GROUP_CODES = 50331769;
pub const CLUSCTL_GROUP_GET_CHARACTERISTICS: CLUSCTL_GROUP_CODES = 50331653;
pub const CLUSCTL_GROUP_GET_COMMON_PROPERTIES: CLUSCTL_GROUP_CODES = 50331737;
pub const CLUSCTL_GROUP_GET_COMMON_PROPERTY_FMTS: CLUSCTL_GROUP_CODES = 50331749;
pub const CLUSCTL_GROUP_GET_FAILURE_INFO: CLUSCTL_GROUP_CODES = 50331673;
pub const CLUSCTL_GROUP_GET_FLAGS: CLUSCTL_GROUP_CODES = 50331657;
pub const CLUSCTL_GROUP_GET_ID: CLUSCTL_GROUP_CODES = 50331705;
pub const CLUSCTL_GROUP_GET_LAST_MOVE_TIME: CLUSCTL_GROUP_CODES = 50332377;
#[repr(C)]
#[cfg(feature = "Win32_minwinbase")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT {
    pub GetTickCount64: u64,
    pub GetSystemTime: super::minwinbase::SYSTEMTIME,
    pub NodeId: u32,
}
pub const CLUSCTL_GROUP_GET_NAME: CLUSCTL_GROUP_CODES = 50331689;
pub const CLUSCTL_GROUP_GET_PRIVATE_PROPERTIES: CLUSCTL_GROUP_CODES = 50331777;
pub const CLUSCTL_GROUP_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_GROUP_CODES = 50331789;
pub const CLUSCTL_GROUP_GET_PROVIDER_GROUPS: CLUSCTL_GROUPSET_CODES = 134229373;
pub const CLUSCTL_GROUP_GET_PROVIDER_GROUPSETS: CLUSCTL_GROUPSET_CODES = 134229377;
pub const CLUSCTL_GROUP_GET_RO_COMMON_PROPERTIES: CLUSCTL_GROUP_CODES = 50331733;
pub const CLUSCTL_GROUP_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_GROUP_CODES = 50331773;
pub const CLUSCTL_GROUP_QUERY_DELETE: CLUSCTL_GROUP_CODES = 50332089;
pub const CLUSCTL_GROUP_SET_CCF_FROM_MASTER: CLUSCTL_GROUP_CODES = 54537606;
pub const CLUSCTL_GROUP_SET_COMMON_PROPERTIES: CLUSCTL_GROUP_CODES = 54526046;
pub const CLUSCTL_GROUP_SET_PRIVATE_PROPERTIES: CLUSCTL_GROUP_CODES = 54526086;
pub const CLUSCTL_GROUP_UNKNOWN: CLUSCTL_GROUP_CODES = 50331648;
pub const CLUSCTL_GROUP_VALIDATE_COMMON_PROPERTIES: CLUSCTL_GROUP_CODES = 50331745;
pub const CLUSCTL_GROUP_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_GROUP_CODES = 50331785;
pub type CLUSCTL_NETINTERFACE_CODES = i32;
pub const CLUSCTL_NETINTERFACE_ENUM_COMMON_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = 100663377;
pub const CLUSCTL_NETINTERFACE_ENUM_PRIVATE_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = 100663417;
pub const CLUSCTL_NETINTERFACE_GET_CHARACTERISTICS: CLUSCTL_NETINTERFACE_CODES = 100663301;
pub const CLUSCTL_NETINTERFACE_GET_COMMON_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = 100663385;
pub const CLUSCTL_NETINTERFACE_GET_COMMON_PROPERTY_FMTS: CLUSCTL_NETINTERFACE_CODES = 100663397;
pub const CLUSCTL_NETINTERFACE_GET_FLAGS: CLUSCTL_NETINTERFACE_CODES = 100663305;
pub const CLUSCTL_NETINTERFACE_GET_ID: CLUSCTL_NETINTERFACE_CODES = 100663353;
pub const CLUSCTL_NETINTERFACE_GET_NAME: CLUSCTL_NETINTERFACE_CODES = 100663337;
pub const CLUSCTL_NETINTERFACE_GET_NETWORK: CLUSCTL_NETINTERFACE_CODES = 100663349;
pub const CLUSCTL_NETINTERFACE_GET_NODE: CLUSCTL_NETINTERFACE_CODES = 100663345;
pub const CLUSCTL_NETINTERFACE_GET_PRIVATE_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = 100663425;
pub const CLUSCTL_NETINTERFACE_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_NETINTERFACE_CODES = 100663437;
pub const CLUSCTL_NETINTERFACE_GET_RO_COMMON_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = 100663381;
pub const CLUSCTL_NETINTERFACE_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = 100663421;
pub const CLUSCTL_NETINTERFACE_SET_COMMON_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = 104857694;
pub const CLUSCTL_NETINTERFACE_SET_PRIVATE_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = 104857734;
pub const CLUSCTL_NETINTERFACE_UNKNOWN: CLUSCTL_NETINTERFACE_CODES = 100663296;
pub const CLUSCTL_NETINTERFACE_VALIDATE_COMMON_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = 100663393;
pub const CLUSCTL_NETINTERFACE_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_NETINTERFACE_CODES = 100663433;
pub type CLUSCTL_NETWORK_CODES = i32;
pub const CLUSCTL_NETWORK_ENUM_COMMON_PROPERTIES: CLUSCTL_NETWORK_CODES = 83886161;
pub const CLUSCTL_NETWORK_ENUM_PRIVATE_PROPERTIES: CLUSCTL_NETWORK_CODES = 83886201;
pub const CLUSCTL_NETWORK_GET_CHARACTERISTICS: CLUSCTL_NETWORK_CODES = 83886085;
pub const CLUSCTL_NETWORK_GET_COMMON_PROPERTIES: CLUSCTL_NETWORK_CODES = 83886169;
pub const CLUSCTL_NETWORK_GET_COMMON_PROPERTY_FMTS: CLUSCTL_NETWORK_CODES = 83886181;
pub const CLUSCTL_NETWORK_GET_FLAGS: CLUSCTL_NETWORK_CODES = 83886089;
pub const CLUSCTL_NETWORK_GET_ID: CLUSCTL_NETWORK_CODES = 83886137;
pub const CLUSCTL_NETWORK_GET_NAME: CLUSCTL_NETWORK_CODES = 83886121;
pub const CLUSCTL_NETWORK_GET_PRIVATE_PROPERTIES: CLUSCTL_NETWORK_CODES = 83886209;
pub const CLUSCTL_NETWORK_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_NETWORK_CODES = 83886221;
pub const CLUSCTL_NETWORK_GET_RO_COMMON_PROPERTIES: CLUSCTL_NETWORK_CODES = 83886165;
pub const CLUSCTL_NETWORK_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_NETWORK_CODES = 83886205;
pub const CLUSCTL_NETWORK_SET_COMMON_PROPERTIES: CLUSCTL_NETWORK_CODES = 88080478;
pub const CLUSCTL_NETWORK_SET_PRIVATE_PROPERTIES: CLUSCTL_NETWORK_CODES = 88080518;
pub const CLUSCTL_NETWORK_UNKNOWN: CLUSCTL_NETWORK_CODES = 83886080;
pub const CLUSCTL_NETWORK_VALIDATE_COMMON_PROPERTIES: CLUSCTL_NETWORK_CODES = 83886177;
pub const CLUSCTL_NETWORK_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_NETWORK_CODES = 83886217;
pub const CLUSCTL_NODE_BLOCK_GEM_SEND_RECV: CLUSCTL_NODE_CODES = 67109581;
pub type CLUSCTL_NODE_CODES = i32;
pub const CLUSCTL_NODE_ENUM_COMMON_PROPERTIES: CLUSCTL_NODE_CODES = 67108945;
pub const CLUSCTL_NODE_ENUM_PRIVATE_PROPERTIES: CLUSCTL_NODE_CODES = 67108985;
pub const CLUSCTL_NODE_GET_CHARACTERISTICS: CLUSCTL_NODE_CODES = 67108869;
pub const CLUSCTL_NODE_GET_CLUSTER_SERVICE_ACCOUNT_NAME: CLUSCTL_NODE_CODES = 67108929;
pub const CLUSCTL_NODE_GET_COMMON_PROPERTIES: CLUSCTL_NODE_CODES = 67108953;
pub const CLUSCTL_NODE_GET_COMMON_PROPERTY_FMTS: CLUSCTL_NODE_CODES = 67108965;
pub const CLUSCTL_NODE_GET_FLAGS: CLUSCTL_NODE_CODES = 67108873;
pub const CLUSCTL_NODE_GET_GEMID_VECTOR: CLUSCTL_NODE_CODES = 67109585;
pub const CLUSCTL_NODE_GET_ID: CLUSCTL_NODE_CODES = 67108921;
pub const CLUSCTL_NODE_GET_NAME: CLUSCTL_NODE_CODES = 67108905;
pub const CLUSCTL_NODE_GET_PRIVATE_PROPERTIES: CLUSCTL_NODE_CODES = 67108993;
pub const CLUSCTL_NODE_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_NODE_CODES = 67109005;
pub const CLUSCTL_NODE_GET_RO_COMMON_PROPERTIES: CLUSCTL_NODE_CODES = 67108949;
pub const CLUSCTL_NODE_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_NODE_CODES = 67108989;
pub const CLUSCTL_NODE_GET_STUCK_NODES: CLUSCTL_NODE_CODES = 67109565;
pub const CLUSCTL_NODE_INJECT_GEM_FAULT: CLUSCTL_NODE_CODES = 67109569;
pub const CLUSCTL_NODE_INTRODUCE_GEM_REPAIR_DELAY: CLUSCTL_NODE_CODES = 67109573;
pub const CLUSCTL_NODE_SEND_DUMMY_GEM_MESSAGES: CLUSCTL_NODE_CODES = 67109577;
pub const CLUSCTL_NODE_SET_COMMON_PROPERTIES: CLUSCTL_NODE_CODES = 71303262;
pub const CLUSCTL_NODE_SET_PRIVATE_PROPERTIES: CLUSCTL_NODE_CODES = 71303302;
pub const CLUSCTL_NODE_UNKNOWN: CLUSCTL_NODE_CODES = 67108864;
pub const CLUSCTL_NODE_VALIDATE_COMMON_PROPERTIES: CLUSCTL_NODE_CODES = 67108961;
pub const CLUSCTL_NODE_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_NODE_CODES = 67109001;
pub const CLUSCTL_OBJECT_MASK: u32 = 255;
pub const CLUSCTL_OBJECT_SHIFT: u32 = 24;
pub const CLUSCTL_RESOURCE_ADD_CRYPTO_CHECKPOINT: CLUSCTL_RESOURCE_CODES = 20971694;
pub const CLUSCTL_RESOURCE_ADD_CRYPTO_CHECKPOINT_EX: CLUSCTL_RESOURCE_CODES = 20972246;
pub const CLUSCTL_RESOURCE_ADD_DEPENDENCY: CLUSCTL_RESOURCE_CODES = 22020114;
pub const CLUSCTL_RESOURCE_ADD_OWNER: CLUSCTL_RESOURCE_CODES = 22020122;
pub const CLUSCTL_RESOURCE_ADD_REGISTRY_CHECKPOINT: CLUSCTL_RESOURCE_CODES = 20971682;
pub const CLUSCTL_RESOURCE_ADD_REGISTRY_CHECKPOINT_32BIT: CLUSCTL_RESOURCE_CODES = 20971714;
pub const CLUSCTL_RESOURCE_ADD_REGISTRY_CHECKPOINT_64BIT: CLUSCTL_RESOURCE_CODES = 20971710;
pub const CLUSCTL_RESOURCE_CHECK_DRAIN_VETO: CLUSCTL_RESOURCE_CODES = 17834285;
pub const CLUSCTL_RESOURCE_CLUSTER_NAME_CHANGED: CLUSCTL_RESOURCE_CODES = 22020138;
pub const CLUSCTL_RESOURCE_CLUSTER_VERSION_CHANGED: CLUSCTL_RESOURCE_CODES = 22020142;
pub type CLUSCTL_RESOURCE_CODES = i32;
pub const CLUSCTL_RESOURCE_DELETE: CLUSCTL_RESOURCE_CODES = 22020102;
pub const CLUSCTL_RESOURCE_DELETE_CRYPTO_CHECKPOINT: CLUSCTL_RESOURCE_CODES = 20971698;
pub const CLUSCTL_RESOURCE_DELETE_REGISTRY_CHECKPOINT: CLUSCTL_RESOURCE_CODES = 20971686;
pub const CLUSCTL_RESOURCE_DISABLE_SHARED_VOLUME_DIRECTIO: CLUSCTL_RESOURCE_CODES = 20972174;
pub const CLUSCTL_RESOURCE_ENABLE_SHARED_VOLUME_DIRECTIO: CLUSCTL_RESOURCE_CODES = 20972170;
pub const CLUSCTL_RESOURCE_ENUM_COMMON_PROPERTIES: CLUSCTL_RESOURCE_CODES = 16777297;
pub const CLUSCTL_RESOURCE_ENUM_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_CODES = 16777337;
pub const CLUSCTL_RESOURCE_EVICT_NODE: CLUSCTL_RESOURCE_CODES = 22020110;
pub const CLUSCTL_RESOURCE_FORCE_QUORUM: CLUSCTL_RESOURCE_CODES = 22020166;
pub const CLUSCTL_RESOURCE_FSWITNESS_GET_EPOCH_INFO: CLUSCTL_RESOURCE_CODES = 17825885;
pub const CLUSCTL_RESOURCE_FSWITNESS_RELEASE_LOCK: CLUSCTL_RESOURCE_CODES = 22020198;
pub const CLUSCTL_RESOURCE_FSWITNESS_SET_EPOCH_INFO: CLUSCTL_RESOURCE_CODES = 22020194;
pub const CLUSCTL_RESOURCE_GET_CHARACTERISTICS: CLUSCTL_RESOURCE_CODES = 16777221;
pub const CLUSCTL_RESOURCE_GET_CLASS_INFO: CLUSCTL_RESOURCE_CODES = 16777229;
pub const CLUSCTL_RESOURCE_GET_COMMON_PROPERTIES: CLUSCTL_RESOURCE_CODES = 16777305;
pub const CLUSCTL_RESOURCE_GET_COMMON_PROPERTY_FMTS: CLUSCTL_RESOURCE_CODES = 16777317;
pub const CLUSCTL_RESOURCE_GET_CRYPTO_CHECKPOINTS: CLUSCTL_RESOURCE_CODES = 16777397;
pub const CLUSCTL_RESOURCE_GET_DNS_NAME: CLUSCTL_RESOURCE_CODES = 16777589;
pub const CLUSCTL_RESOURCE_GET_FAILURE_INFO: CLUSCTL_RESOURCE_CODES = 16777241;
pub const CLUSCTL_RESOURCE_GET_FLAGS: CLUSCTL_RESOURCE_CODES = 16777225;
pub const CLUSCTL_RESOURCE_GET_ID: CLUSCTL_RESOURCE_CODES = 16777273;
pub const CLUSCTL_RESOURCE_GET_INFRASTRUCTURE_SOFS_BUFFER: CLUSCTL_RESOURCE_CODES = 16788873;
pub const CLUSCTL_RESOURCE_GET_LOADBAL_PROCESS_LIST: CLUSCTL_RESOURCE_CODES = 16777417;
pub const CLUSCTL_RESOURCE_GET_NAME: CLUSCTL_RESOURCE_CODES = 16777257;
pub const CLUSCTL_RESOURCE_GET_NETWORK_NAME: CLUSCTL_RESOURCE_CODES = 16777577;
pub const CLUSCTL_RESOURCE_GET_NODES_IN_FD: CLUSCTL_RESOURCE_CODES = 16788961;
pub const CLUSCTL_RESOURCE_GET_OPERATION_CONTEXT: CLUSCTL_RESOURCE_CODES = 17834217;
pub const CLUSCTL_RESOURCE_GET_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_CODES = 16777345;
pub const CLUSCTL_RESOURCE_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_RESOURCE_CODES = 16777357;
pub const CLUSCTL_RESOURCE_GET_REGISTRY_CHECKPOINTS: CLUSCTL_RESOURCE_CODES = 16777385;
pub const CLUSCTL_RESOURCE_GET_REQUIRED_DEPENDENCIES: CLUSCTL_RESOURCE_CODES = 16777233;
pub const CLUSCTL_RESOURCE_GET_RESOURCE_TYPE: CLUSCTL_RESOURCE_CODES = 16777261;
pub const CLUSCTL_RESOURCE_GET_RO_COMMON_PROPERTIES: CLUSCTL_RESOURCE_CODES = 16777301;
pub const CLUSCTL_RESOURCE_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_CODES = 16777341;
pub const CLUSCTL_RESOURCE_GET_STATE_CHANGE_TIME: CLUSCTL_RESOURCE_CODES = 16788829;
pub const CLUSCTL_RESOURCE_INITIALIZE: CLUSCTL_RESOURCE_CODES = 22020170;
pub const CLUSCTL_RESOURCE_INSTALL_NODE: CLUSCTL_RESOURCE_CODES = 22020106;
pub const CLUSCTL_RESOURCE_IPADDRESS_RELEASE_LEASE: CLUSCTL_RESOURCE_CODES = 20971970;
pub const CLUSCTL_RESOURCE_IPADDRESS_RENEW_LEASE: CLUSCTL_RESOURCE_CODES = 20971966;
pub const CLUSCTL_RESOURCE_IS_QUORUM_BLOCKED: CLUSCTL_RESOURCE_CODES = 16777905;
pub const CLUSCTL_RESOURCE_JOINING_GROUP: CLUSCTL_RESOURCE_CODES = 22020186;
pub const CLUSCTL_RESOURCE_LEAVING_GROUP: CLUSCTL_RESOURCE_CODES = 22020182;
pub const CLUSCTL_RESOURCE_NETNAME_CREDS_NOTIFYCAM: CLUSCTL_RESOURCE_CODES = 22020202;
pub const CLUSCTL_RESOURCE_NETNAME_DELETE_CO: CLUSCTL_RESOURCE_CODES = 16777598;
pub const CLUSCTL_RESOURCE_NETNAME_GET_VIRTUAL_SERVER_TOKEN: CLUSCTL_RESOURCE_CODES = 16777581;
pub const CLUSCTL_RESOURCE_NETNAME_REGISTER_DNS_RECORDS: CLUSCTL_RESOURCE_CODES = 16777586;
pub const CLUSCTL_RESOURCE_NETNAME_REPAIR_VCO: CLUSCTL_RESOURCE_CODES = 16777613;
pub const CLUSCTL_RESOURCE_NETNAME_RESET_VCO: CLUSCTL_RESOURCE_CODES = 16777605;
pub const CLUSCTL_RESOURCE_NETNAME_SET_PWD_INFO: CLUSCTL_RESOURCE_CODES = 16777594;
pub const CLUSCTL_RESOURCE_NETNAME_SET_PWD_INFOEX: CLUSCTL_RESOURCE_CODES = 16778010;
pub const CLUSCTL_RESOURCE_NETNAME_VALIDATE_VCO: CLUSCTL_RESOURCE_CODES = 16777601;
pub const CLUSCTL_RESOURCE_NOTIFY_DRAIN_COMPLETE: CLUSCTL_RESOURCE_CODES = 17834289;
pub const CLUSCTL_RESOURCE_NOTIFY_OWNER_CHANGE: CLUSCTL_RESOURCE_CODES = 22028578;
pub const CLUSCTL_RESOURCE_NOTIFY_QUORUM_STATUS: CLUSCTL_RESOURCE_CODES = 22020222;
pub const CLUSCTL_RESOURCE_POOL_GET_DRIVE_INFO: CLUSCTL_RESOURCE_CODES = 16777909;
pub const CLUSCTL_RESOURCE_PREPARE_UPGRADE: CLUSCTL_RESOURCE_CODES = 20979946;
pub const CLUSCTL_RESOURCE_PROVIDER_STATE_CHANGE: CLUSCTL_RESOURCE_CODES = 22020178;
pub const CLUSCTL_RESOURCE_QUERY_DELETE: CLUSCTL_RESOURCE_CODES = 16777657;
pub const CLUSCTL_RESOURCE_QUERY_MAINTENANCE_MODE: CLUSCTL_RESOURCE_CODES = 16777697;
pub const CLUSCTL_RESOURCE_REMOVE_DEPENDENCY: CLUSCTL_RESOURCE_CODES = 22020118;
pub const CLUSCTL_RESOURCE_REMOVE_OWNER: CLUSCTL_RESOURCE_CODES = 22020126;
pub const CLUSCTL_RESOURCE_RLUA_GET_PWD: CLUSCTL_RESOURCE_CODES = 16778018;
pub const CLUSCTL_RESOURCE_RLUA_GET_VIRTUAL_SERVER_TOKEN: CLUSCTL_RESOURCE_CODES = 16777581;
pub const CLUSCTL_RESOURCE_RLUA_SET_PWD_INFO: CLUSCTL_RESOURCE_CODES = 16777594;
pub const CLUSCTL_RESOURCE_RLUA_SET_PWD_INFOEX: CLUSCTL_RESOURCE_CODES = 16778010;
pub const CLUSCTL_RESOURCE_RW_MODIFY_NOOP: CLUSCTL_RESOURCE_CODES = 20972206;
pub const CLUSCTL_RESOURCE_SCALEOUT_COMMAND: CLUSCTL_RESOURCE_CODES = 20983190;
pub const CLUSCTL_RESOURCE_SCALEOUT_CONTROL: CLUSCTL_RESOURCE_CODES = 20983194;
pub const CLUSCTL_RESOURCE_SCALEOUT_GET_CLUSTERS: CLUSCTL_RESOURCE_CODES = 20983197;
pub const CLUSCTL_RESOURCE_SET_COMMON_PROPERTIES: CLUSCTL_RESOURCE_CODES = 20971614;
pub const CLUSCTL_RESOURCE_SET_CSV_MAINTENANCE_MODE: CLUSCTL_RESOURCE_CODES = 20972182;
pub const CLUSCTL_RESOURCE_SET_INFRASTRUCTURE_SOFS_BUFFER: CLUSCTL_RESOURCE_CODES = 20983182;
pub const CLUSCTL_RESOURCE_SET_MAINTENANCE_MODE: CLUSCTL_RESOURCE_CODES = 20972006;
pub const CLUSCTL_RESOURCE_SET_NAME: CLUSCTL_RESOURCE_CODES = 22020134;
pub const CLUSCTL_RESOURCE_SET_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_CODES = 20971654;
pub const CLUSCTL_RESOURCE_SET_SHARED_VOLUME_BACKUP_MODE: CLUSCTL_RESOURCE_CODES = 20972186;
pub const CLUSCTL_RESOURCE_STATE_CHANGE_REASON: CLUSCTL_RESOURCE_CODES = 22020174;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT {
    pub dwSize: u32,
    pub dwVersion: u32,
    pub eReason: CLUSTER_RESOURCE_STATE_CHANGE_REASON,
}
pub const CLUSCTL_RESOURCE_STATE_CHANGE_REASON_VERSION_1: u32 = 1;
pub const CLUSCTL_RESOURCE_STORAGE_GET_DIRTY: CLUSCTL_RESOURCE_CODES = 16777753;
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISKID: CLUSCTL_RESOURCE_CODES = 16777733;
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISK_INFO: CLUSCTL_RESOURCE_CODES = 16777617;
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISK_INFO_EX: CLUSCTL_RESOURCE_CODES = 16777713;
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISK_INFO_EX2: CLUSCTL_RESOURCE_CODES = 16777721;
pub const CLUSCTL_RESOURCE_STORAGE_GET_DISK_NUMBER_INFO: CLUSCTL_RESOURCE_CODES = 16777633;
pub const CLUSCTL_RESOURCE_STORAGE_GET_MOUNTPOINTS: CLUSCTL_RESOURCE_CODES = 16777745;
pub const CLUSCTL_RESOURCE_STORAGE_GET_SHARED_VOLUME_INFO: CLUSCTL_RESOURCE_CODES = 16777765;
pub const CLUSCTL_RESOURCE_STORAGE_GET_SHARED_VOLUME_PARTITION_NAMES: CLUSCTL_RESOURCE_CODES = 16777885;
pub const CLUSCTL_RESOURCE_STORAGE_GET_SHARED_VOLUME_STATES: CLUSCTL_RESOURCE_CODES = 20972194;
pub const CLUSCTL_RESOURCE_STORAGE_IS_PATH_VALID: CLUSCTL_RESOURCE_CODES = 16777625;
pub const CLUSCTL_RESOURCE_STORAGE_IS_SHARED_VOLUME: CLUSCTL_RESOURCE_CODES = 16777893;
pub const CLUSCTL_RESOURCE_STORAGE_RENAME_SHARED_VOLUME: CLUSCTL_RESOURCE_CODES = 16788950;
pub const CLUSCTL_RESOURCE_STORAGE_RENAME_SHARED_VOLUME_GUID: CLUSCTL_RESOURCE_CODES = 16788954;
pub const CLUSCTL_RESOURCE_STORAGE_SET_DRIVELETTER: CLUSCTL_RESOURCE_CODES = 20972010;
pub const CLUSCTL_RESOURCE_TYPE_CHECK_DRAIN_VETO: CLUSCTL_RESOURCE_TYPE_CODES = 34611501;
pub const CLUSCTL_RESOURCE_TYPE_CLUSTER_VERSION_CHANGED: CLUSCTL_RESOURCE_TYPE_CODES = 38797358;
pub type CLUSCTL_RESOURCE_TYPE_CODES = i32;
pub const CLUSCTL_RESOURCE_TYPE_ENUM_COMMON_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = 33554513;
pub const CLUSCTL_RESOURCE_TYPE_ENUM_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = 33554553;
pub const CLUSCTL_RESOURCE_TYPE_EVICT_NODE: CLUSCTL_RESOURCE_TYPE_CODES = 38797326;
pub const CLUSCTL_RESOURCE_TYPE_FIXUP_ON_UPGRADE: CLUSCTL_RESOURCE_TYPE_CODES = 38797362;
pub const CLUSCTL_RESOURCE_TYPE_GEN_APP_VALIDATE_DIRECTORY: CLUSCTL_RESOURCE_TYPE_CODES = 33555001;
pub const CLUSCTL_RESOURCE_TYPE_GEN_APP_VALIDATE_PATH: CLUSCTL_RESOURCE_TYPE_CODES = 33554993;
pub const CLUSCTL_RESOURCE_TYPE_GEN_SCRIPT_VALIDATE_PATH: CLUSCTL_RESOURCE_TYPE_CODES = 33554993;
pub const CLUSCTL_RESOURCE_TYPE_GET_ARB_TIMEOUT: CLUSCTL_RESOURCE_TYPE_CODES = 33554453;
pub const CLUSCTL_RESOURCE_TYPE_GET_CHARACTERISTICS: CLUSCTL_RESOURCE_TYPE_CODES = 33554437;
pub const CLUSCTL_RESOURCE_TYPE_GET_CLASS_INFO: CLUSCTL_RESOURCE_TYPE_CODES = 33554445;
pub const CLUSCTL_RESOURCE_TYPE_GET_COMMON_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = 33554521;
pub const CLUSCTL_RESOURCE_TYPE_GET_COMMON_PROPERTY_FMTS: CLUSCTL_RESOURCE_TYPE_CODES = 33554533;
pub const CLUSCTL_RESOURCE_TYPE_GET_COMMON_RESOURCE_PROPERTY_FMTS: CLUSCTL_RESOURCE_TYPE_CODES = 33554537;
pub const CLUSCTL_RESOURCE_TYPE_GET_CRYPTO_CHECKPOINTS: CLUSCTL_RESOURCE_TYPE_CODES = 33554613;
pub const CLUSCTL_RESOURCE_TYPE_GET_FLAGS: CLUSCTL_RESOURCE_TYPE_CODES = 33554441;
pub const CLUSCTL_RESOURCE_TYPE_GET_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = 33554561;
pub const CLUSCTL_RESOURCE_TYPE_GET_PRIVATE_PROPERTY_FMTS: CLUSCTL_RESOURCE_TYPE_CODES = 33554573;
pub const CLUSCTL_RESOURCE_TYPE_GET_PRIVATE_RESOURCE_PROPERTY_FMTS: CLUSCTL_RESOURCE_TYPE_CODES = 33554577;
pub const CLUSCTL_RESOURCE_TYPE_GET_REGISTRY_CHECKPOINTS: CLUSCTL_RESOURCE_TYPE_CODES = 33554601;
pub const CLUSCTL_RESOURCE_TYPE_GET_REQUIRED_DEPENDENCIES: CLUSCTL_RESOURCE_TYPE_CODES = 33554449;
pub const CLUSCTL_RESOURCE_TYPE_GET_RO_COMMON_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = 33554517;
pub const CLUSCTL_RESOURCE_TYPE_GET_RO_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = 33554557;
pub const CLUSCTL_RESOURCE_TYPE_HOLD_IO: CLUSCTL_RESOURCE_TYPE_CODES = 38797374;
pub const CLUSCTL_RESOURCE_TYPE_INSTALL_NODE: CLUSCTL_RESOURCE_TYPE_CODES = 38797322;
pub const CLUSCTL_RESOURCE_TYPE_NETNAME_GET_OU_FOR_VCO: CLUSCTL_RESOURCE_TYPE_CODES = 37749358;
pub const CLUSCTL_RESOURCE_TYPE_NETNAME_VALIDATE_NETNAME: CLUSCTL_RESOURCE_TYPE_CODES = 33554997;
pub const CLUSCTL_RESOURCE_TYPE_NOTIFY_DRAIN_COMPLETE: CLUSCTL_RESOURCE_TYPE_CODES = 34611505;
pub const CLUSCTL_RESOURCE_TYPE_NOTIFY_MONITOR_SHUTTING_DOWN: CLUSCTL_RESOURCE_TYPE_CODES = 34603137;
pub const CLUSCTL_RESOURCE_TYPE_PREPARE_UPGRADE: CLUSCTL_RESOURCE_TYPE_CODES = 37757162;
pub const CLUSCTL_RESOURCE_TYPE_QUERY_DELETE: CLUSCTL_RESOURCE_TYPE_CODES = 33554873;
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_ADD_REPLICATION_GROUP: CLUSCTL_RESOURCE_TYPE_CODES = 33562946;
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_ELIGIBLE_LOGDISKS: CLUSCTL_RESOURCE_TYPE_CODES = 33562953;
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_ELIGIBLE_SOURCE_DATADISKS: CLUSCTL_RESOURCE_TYPE_CODES = 33562961;
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_ELIGIBLE_TARGET_DATADISKS: CLUSCTL_RESOURCE_TYPE_CODES = 33562957;
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_LOG_INFO: CLUSCTL_RESOURCE_TYPE_CODES = 33562949;
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_LOG_VOLUME: CLUSCTL_RESOURCE_TYPE_CODES = 33562973;
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_REPLICATED_DISKS: CLUSCTL_RESOURCE_TYPE_CODES = 33562965;
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_REPLICATED_PARTITION_INFO: CLUSCTL_RESOURCE_TYPE_CODES = 33562981;
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_REPLICA_VOLUMES: CLUSCTL_RESOURCE_TYPE_CODES = 33562969;
pub const CLUSCTL_RESOURCE_TYPE_REPLICATION_GET_RESOURCE_GROUP: CLUSCTL_RESOURCE_TYPE_CODES = 33562977;
pub const CLUSCTL_RESOURCE_TYPE_RESUME_IO: CLUSCTL_RESOURCE_TYPE_CODES = 38797378;
pub const CLUSCTL_RESOURCE_TYPE_SET_COMMON_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = 37748830;
pub const CLUSCTL_RESOURCE_TYPE_SET_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = 37748870;
pub const CLUSCTL_RESOURCE_TYPE_STARTING_PHASE1: CLUSCTL_RESOURCE_TYPE_CODES = 38797366;
pub const CLUSCTL_RESOURCE_TYPE_STARTING_PHASE2: CLUSCTL_RESOURCE_TYPE_CODES = 38797370;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS: CLUSCTL_RESOURCE_TYPE_CODES = 33554837;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX: CLUSCTL_RESOURCE_TYPE_CODES = 33554933;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_FLAG_ADD_VOLUME_INFO: u32 = 1;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_FLAG_FILTER_BY_POOL: u32 = 2;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_FLAG_INCLUDE_NON_SHARED_DISKS: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT {
    pub dwFlags: u32,
    pub guidPoolFilter: windows_core::GUID,
}
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INT: CLUSCTL_RESOURCE_TYPE_CODES = 33562593;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_DISKID: CLUSCTL_RESOURCE_TYPE_CODES = 33554949;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_DRIVELETTERS: CLUSCTL_RESOURCE_TYPE_CODES = 33554925;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_GET_RESOURCEID: CLUSCTL_RESOURCE_TYPE_CODES = 33554989;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_IS_CLUSTERABLE: CLUSCTL_RESOURCE_TYPE_CODES = 33554953;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_IS_CSV_FILE: CLUSCTL_RESOURCE_TYPE_CODES = 16777769;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_REMAP_DRIVELETTER: CLUSCTL_RESOURCE_TYPE_CODES = 33554945;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_REMOVE_VM_OWNERSHIP: CLUSCTL_RESOURCE_TYPE_CODES = 37749262;
pub const CLUSCTL_RESOURCE_TYPE_STORAGE_SYNC_CLUSDISK_DB: CLUSCTL_RESOURCE_TYPE_CODES = 37749150;
pub const CLUSCTL_RESOURCE_TYPE_UNKNOWN: CLUSCTL_RESOURCE_TYPE_CODES = 33554432;
pub const CLUSCTL_RESOURCE_TYPE_UPGRADE_COMPLETED: CLUSCTL_RESOURCE_TYPE_CODES = 37757166;
pub const CLUSCTL_RESOURCE_TYPE_VALIDATE_COMMON_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = 33554529;
pub const CLUSCTL_RESOURCE_TYPE_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_TYPE_CODES = 33554569;
pub const CLUSCTL_RESOURCE_TYPE_WITNESS_VALIDATE_PATH: CLUSCTL_RESOURCE_TYPE_CODES = 33554993;
pub const CLUSCTL_RESOURCE_UNDELETE: CLUSCTL_RESOURCE_CODES = 22020230;
pub const CLUSCTL_RESOURCE_UNKNOWN: CLUSCTL_RESOURCE_CODES = 16777216;
pub const CLUSCTL_RESOURCE_UPGRADE_COMPLETED: CLUSCTL_RESOURCE_CODES = 20979950;
pub const CLUSCTL_RESOURCE_UPGRADE_DLL: CLUSCTL_RESOURCE_CODES = 20971706;
pub const CLUSCTL_RESOURCE_VALIDATE_CHANGE_GROUP: CLUSCTL_RESOURCE_CODES = 17834277;
pub const CLUSCTL_RESOURCE_VALIDATE_COMMON_PROPERTIES: CLUSCTL_RESOURCE_CODES = 16777313;
pub const CLUSCTL_RESOURCE_VALIDATE_PRIVATE_PROPERTIES: CLUSCTL_RESOURCE_CODES = 16777353;
pub const CLUSGROUPSET_STATUS_APPLICATION_READY: u32 = 8;
pub const CLUSGROUPSET_STATUS_GROUPS_ONLINE: u32 = 2;
pub const CLUSGROUPSET_STATUS_GROUPS_PENDING: u32 = 1;
pub const CLUSGROUPSET_STATUS_OS_HEARTBEAT: u32 = 4;
pub type CLUSGROUP_TYPE = i32;
pub const CLUSGRP_STATUS_APPLICATION_READY: u32 = 1024;
pub const CLUSGRP_STATUS_EMBEDDED_FAILURE: u32 = 32;
pub const CLUSGRP_STATUS_LOCKED_MODE: u32 = 1;
pub const CLUSGRP_STATUS_NETWORK_FAILURE: u32 = 128;
pub const CLUSGRP_STATUS_OFFLINE_DUE_TO_ANTIAFFINITY_CONFLICT: u32 = 64;
pub const CLUSGRP_STATUS_OFFLINE_NOT_LOCAL_DISK_OWNER: u32 = 2048;
pub const CLUSGRP_STATUS_OS_HEARTBEAT: u32 = 512;
pub const CLUSGRP_STATUS_PHYSICAL_RESOURCES_LACKING: u32 = 8;
pub const CLUSGRP_STATUS_PREEMPTED: u32 = 2;
pub const CLUSGRP_STATUS_UNMONITORED: u32 = 256;
pub const CLUSGRP_STATUS_WAITING_FOR_DEPENDENCIES: u32 = 4096;
pub const CLUSGRP_STATUS_WAITING_IN_QUEUE_FOR_MOVE: u32 = 4;
pub const CLUSGRP_STATUS_WAITING_TO_START: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_BINARY {
    pub Base: CLUSPROP_VALUE,
    pub rgb: [u8; 0],
}
impl Default for CLUSPROP_BINARY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub union CLUSPROP_BUFFER_HELPER {
    pub pb: *mut u8,
    pub pw: *mut u16,
    pub pdw: *mut u32,
    pub pl: *mut i32,
    pub psz: windows_core::PWSTR,
    pub pList: PCLUSPROP_LIST,
    pub pSyntax: PCLUSPROP_SYNTAX,
    pub pName: PCLUSPROP_PROPERTY_NAME,
    pub pValue: PCLUSPROP_VALUE,
    pub pBinaryValue: PCLUSPROP_BINARY,
    pub pWordValue: PCLUSPROP_WORD,
    pub pDwordValue: PCLUSPROP_DWORD,
    pub pLongValue: PCLUSPROP_LONG,
    pub pULargeIntegerValue: PCLUSPROP_ULARGE_INTEGER,
    pub pLargeIntegerValue: PCLUSPROP_LARGE_INTEGER,
    pub pStringValue: PCLUSPROP_SZ,
    pub pMultiSzValue: PCLUSPROP_MULTI_SZ,
    pub pSecurityDescriptor: PCLUSPROP_SECURITY_DESCRIPTOR,
    pub pResourceClassValue: PCLUSPROP_RESOURCE_CLASS,
    pub pResourceClassInfoValue: PCLUSPROP_RESOURCE_CLASS_INFO,
    pub pDiskSignatureValue: PCLUSPROP_DISK_SIGNATURE,
    pub pScsiAddressValue: PCLUSPROP_SCSI_ADDRESS,
    pub pDiskNumberValue: PCLUSPROP_DISK_NUMBER,
    pub pPartitionInfoValue: PCLUSPROP_PARTITION_INFO,
    pub pRequiredDependencyValue: PCLUSPROP_REQUIRED_DEPENDENCY,
    pub pPartitionInfoValueEx: PCLUSPROP_PARTITION_INFO_EX,
    pub pPartitionInfoValueEx2: PCLUSPROP_PARTITION_INFO_EX2,
    pub pFileTimeValue: PCLUSPROP_FILETIME,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for CLUSPROP_BUFFER_HELPER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CLUSPROP_DISK_NUMBER = CLUSPROP_DWORD;
pub type CLUSPROP_DISK_SIGNATURE = CLUSPROP_DWORD;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_DWORD {
    pub Base: CLUSPROP_VALUE,
    pub dw: u32,
}
impl Default for CLUSPROP_DWORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct CLUSPROP_FILETIME {
    pub Base: CLUSPROP_VALUE,
    pub ft: super::minwindef::FILETIME,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for CLUSPROP_FILETIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLUSPROP_FORMAT_BINARY: CLUSTER_PROPERTY_FORMAT = 1;
pub const CLUSPROP_FORMAT_DWORD: CLUSTER_PROPERTY_FORMAT = 2;
pub const CLUSPROP_FORMAT_EXPANDED_SZ: CLUSTER_PROPERTY_FORMAT = 8;
pub const CLUSPROP_FORMAT_EXPAND_SZ: CLUSTER_PROPERTY_FORMAT = 4;
pub const CLUSPROP_FORMAT_FILETIME: CLUSTER_PROPERTY_FORMAT = 12;
pub const CLUSPROP_FORMAT_LARGE_INTEGER: CLUSTER_PROPERTY_FORMAT = 10;
pub const CLUSPROP_FORMAT_LONG: CLUSTER_PROPERTY_FORMAT = 7;
pub const CLUSPROP_FORMAT_MULTI_SZ: CLUSTER_PROPERTY_FORMAT = 5;
pub const CLUSPROP_FORMAT_PROPERTY_LIST: CLUSTER_PROPERTY_FORMAT = 14;
pub const CLUSPROP_FORMAT_SECURITY_DESCRIPTOR: CLUSTER_PROPERTY_FORMAT = 9;
pub const CLUSPROP_FORMAT_SZ: CLUSTER_PROPERTY_FORMAT = 3;
pub const CLUSPROP_FORMAT_ULARGE_INTEGER: CLUSTER_PROPERTY_FORMAT = 6;
pub const CLUSPROP_FORMAT_UNKNOWN: CLUSTER_PROPERTY_FORMAT = 0;
pub const CLUSPROP_FORMAT_USER: CLUSTER_PROPERTY_FORMAT = 32768;
pub const CLUSPROP_FORMAT_VALUE_LIST: CLUSTER_PROPERTY_FORMAT = 13;
pub const CLUSPROP_FORMAT_WORD: CLUSTER_PROPERTY_FORMAT = 11;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_FTSET_INFO {
    pub Base: CLUSPROP_VALUE,
    pub Base2: CLUS_FTSET_INFO,
}
impl Default for CLUSPROP_FTSET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CLUSPROP_IPADDR_ENABLENETBIOS = i32;
pub const CLUSPROP_IPADDR_ENABLENETBIOS_DISABLED: CLUSPROP_IPADDR_ENABLENETBIOS = 0;
pub const CLUSPROP_IPADDR_ENABLENETBIOS_ENABLED: CLUSPROP_IPADDR_ENABLENETBIOS = 1;
pub const CLUSPROP_IPADDR_ENABLENETBIOS_TRACK_NIC: CLUSPROP_IPADDR_ENABLENETBIOS = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_LARGE_INTEGER {
    pub Base: CLUSPROP_VALUE,
    pub li: i64,
}
impl Default for CLUSPROP_LARGE_INTEGER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_LIST {
    pub nPropertyCount: u32,
    pub PropertyName: CLUSPROP_PROPERTY_NAME,
}
impl Default for CLUSPROP_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_LONG {
    pub Base: CLUSPROP_VALUE,
    pub l: i32,
}
impl Default for CLUSPROP_LONG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CLUSPROP_MULTI_SZ = CLUSPROP_SZ;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_PARTITION_INFO {
    pub Base: CLUSPROP_VALUE,
    pub Base2: CLUS_PARTITION_INFO,
}
impl Default for CLUSPROP_PARTITION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_PARTITION_INFO_EX {
    pub Base: CLUSPROP_VALUE,
    pub Base2: CLUS_PARTITION_INFO_EX,
}
impl Default for CLUSPROP_PARTITION_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_PARTITION_INFO_EX2 {
    pub Base: CLUSPROP_PARTITION_INFO_EX,
    pub Base2: CLUS_PARTITION_INFO_EX2,
}
impl Default for CLUSPROP_PARTITION_INFO_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CLUSPROP_PIFLAGS = i32;
pub const CLUSPROP_PIFLAG_DEFAULT_QUORUM: CLUSPROP_PIFLAGS = 8;
pub const CLUSPROP_PIFLAG_ENCRYPTION_ENABLED: CLUSPROP_PIFLAGS = 32;
pub const CLUSPROP_PIFLAG_RAW: CLUSPROP_PIFLAGS = 64;
pub const CLUSPROP_PIFLAG_REMOVABLE: CLUSPROP_PIFLAGS = 2;
pub const CLUSPROP_PIFLAG_STICKY: CLUSPROP_PIFLAGS = 1;
pub const CLUSPROP_PIFLAG_UNKNOWN: CLUSPROP_PIFLAGS = -2147483648;
pub const CLUSPROP_PIFLAG_USABLE: CLUSPROP_PIFLAGS = 4;
pub const CLUSPROP_PIFLAG_USABLE_FOR_CSV: CLUSPROP_PIFLAGS = 16;
pub type CLUSPROP_PROPERTY_NAME = CLUSPROP_SZ;
#[repr(C)]
#[derive(Clone, Copy)]
pub union CLUSPROP_REQUIRED_DEPENDENCY {
    pub Value: CLUSPROP_VALUE,
    pub ResClass: CLUSPROP_RESOURCE_CLASS,
    pub ResTypeName: CLUSPROP_SZ,
}
impl Default for CLUSPROP_REQUIRED_DEPENDENCY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_RESOURCE_CLASS {
    pub Base: CLUSPROP_VALUE,
    pub rc: CLUSTER_RESOURCE_CLASS,
}
impl Default for CLUSPROP_RESOURCE_CLASS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_RESOURCE_CLASS_INFO {
    pub Base: CLUSPROP_VALUE,
    pub Base2: CLUS_RESOURCE_CLASS_INFO,
}
impl Default for CLUSPROP_RESOURCE_CLASS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_SCSI_ADDRESS {
    pub Base: CLUSPROP_VALUE,
    pub Base2: CLUS_SCSI_ADDRESS,
}
impl Default for CLUSPROP_SCSI_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct CLUSPROP_SECURITY_DESCRIPTOR {
    pub Base: CLUSPROP_VALUE,
    pub Anonymous: CLUSPROP_SECURITY_DESCRIPTOR_0,
}
#[cfg(feature = "Win32_winnt")]
impl Default for CLUSPROP_SECURITY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub union CLUSPROP_SECURITY_DESCRIPTOR_0 {
    pub sd: super::winnt::SECURITY_DESCRIPTOR_RELATIVE,
    pub rgbSecurityDescriptor: [u8; 0],
}
#[cfg(feature = "Win32_winnt")]
impl Default for CLUSPROP_SECURITY_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CLUSPROP_SYNTAX {
    pub dw: u32,
    pub Anonymous: CLUSPROP_SYNTAX_0,
}
impl Default for CLUSPROP_SYNTAX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUSPROP_SYNTAX_0 {
    pub wFormat: u16,
    pub wType: u16,
}
pub const CLUSPROP_SYNTAX_DISK_GUID: CLUSTER_PROPERTY_SYNTAX = 720899;
pub const CLUSPROP_SYNTAX_DISK_NUMBER: CLUSTER_PROPERTY_SYNTAX = 458754;
pub const CLUSPROP_SYNTAX_DISK_SERIALNUMBER: CLUSTER_PROPERTY_SYNTAX = 655363;
pub const CLUSPROP_SYNTAX_DISK_SIGNATURE: CLUSTER_PROPERTY_SYNTAX = 327682;
pub const CLUSPROP_SYNTAX_DISK_SIZE: CLUSTER_PROPERTY_SYNTAX = 786438;
pub const CLUSPROP_SYNTAX_ENDMARK: CLUSTER_PROPERTY_SYNTAX = 0;
pub const CLUSPROP_SYNTAX_FTSET_INFO: CLUSTER_PROPERTY_SYNTAX = 589825;
pub const CLUSPROP_SYNTAX_LIST_VALUE_BINARY: CLUSTER_PROPERTY_SYNTAX = 65537;
pub const CLUSPROP_SYNTAX_LIST_VALUE_DWORD: CLUSTER_PROPERTY_SYNTAX = 65538;
pub const CLUSPROP_SYNTAX_LIST_VALUE_EXPANDED_SZ: CLUSTER_PROPERTY_SYNTAX = 65544;
pub const CLUSPROP_SYNTAX_LIST_VALUE_EXPAND_SZ: CLUSTER_PROPERTY_SYNTAX = 65540;
pub const CLUSPROP_SYNTAX_LIST_VALUE_FILETIME: CLUSTER_PROPERTY_SYNTAX = 65548;
pub const CLUSPROP_SYNTAX_LIST_VALUE_LARGE_INTEGER: CLUSTER_PROPERTY_SYNTAX = 65546;
pub const CLUSPROP_SYNTAX_LIST_VALUE_LONG: CLUSTER_PROPERTY_SYNTAX = 65543;
pub const CLUSPROP_SYNTAX_LIST_VALUE_MULTI_SZ: CLUSTER_PROPERTY_SYNTAX = 65541;
pub const CLUSPROP_SYNTAX_LIST_VALUE_PROPERTY_LIST: CLUSTER_PROPERTY_SYNTAX = 65550;
pub const CLUSPROP_SYNTAX_LIST_VALUE_SECURITY_DESCRIPTOR: CLUSTER_PROPERTY_SYNTAX = 65545;
pub const CLUSPROP_SYNTAX_LIST_VALUE_SZ: CLUSTER_PROPERTY_SYNTAX = 65539;
pub const CLUSPROP_SYNTAX_LIST_VALUE_ULARGE_INTEGER: CLUSTER_PROPERTY_SYNTAX = 65542;
pub const CLUSPROP_SYNTAX_LIST_VALUE_WORD: CLUSTER_PROPERTY_SYNTAX = 65547;
pub const CLUSPROP_SYNTAX_NAME: CLUSTER_PROPERTY_SYNTAX = 262147;
pub const CLUSPROP_SYNTAX_PARTITION_INFO: CLUSTER_PROPERTY_SYNTAX = 524289;
pub const CLUSPROP_SYNTAX_PARTITION_INFO_EX: CLUSTER_PROPERTY_SYNTAX = 851969;
pub const CLUSPROP_SYNTAX_PARTITION_INFO_EX2: CLUSTER_PROPERTY_SYNTAX = 917505;
pub const CLUSPROP_SYNTAX_RESCLASS: CLUSTER_PROPERTY_SYNTAX = 131074;
pub const CLUSPROP_SYNTAX_SCSI_ADDRESS: CLUSTER_PROPERTY_SYNTAX = 393218;
pub const CLUSPROP_SYNTAX_STORAGE_DEVICE_ID_DESCRIPTOR: CLUSTER_PROPERTY_SYNTAX = 983041;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_SZ {
    pub Base: CLUSPROP_VALUE,
    pub sz: [u16; 0],
}
impl Default for CLUSPROP_SZ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLUSPROP_TYPE_DISK_GUID: CLUSTER_PROPERTY_TYPE = 11;
pub const CLUSPROP_TYPE_DISK_NUMBER: CLUSTER_PROPERTY_TYPE = 7;
pub const CLUSPROP_TYPE_DISK_SERIALNUMBER: CLUSTER_PROPERTY_TYPE = 10;
pub const CLUSPROP_TYPE_DISK_SIZE: CLUSTER_PROPERTY_TYPE = 12;
pub const CLUSPROP_TYPE_ENDMARK: CLUSTER_PROPERTY_TYPE = 0;
pub const CLUSPROP_TYPE_FTSET_INFO: CLUSTER_PROPERTY_TYPE = 9;
pub const CLUSPROP_TYPE_LIST_VALUE: CLUSTER_PROPERTY_TYPE = 1;
pub const CLUSPROP_TYPE_NAME: CLUSTER_PROPERTY_TYPE = 4;
pub const CLUSPROP_TYPE_PARTITION_INFO: CLUSTER_PROPERTY_TYPE = 8;
pub const CLUSPROP_TYPE_PARTITION_INFO_EX: CLUSTER_PROPERTY_TYPE = 13;
pub const CLUSPROP_TYPE_PARTITION_INFO_EX2: CLUSTER_PROPERTY_TYPE = 14;
pub const CLUSPROP_TYPE_RESCLASS: CLUSTER_PROPERTY_TYPE = 2;
pub const CLUSPROP_TYPE_RESERVED1: CLUSTER_PROPERTY_TYPE = 3;
pub const CLUSPROP_TYPE_SCSI_ADDRESS: CLUSTER_PROPERTY_TYPE = 6;
pub const CLUSPROP_TYPE_SIGNATURE: CLUSTER_PROPERTY_TYPE = 5;
pub const CLUSPROP_TYPE_STORAGE_DEVICE_ID_DESCRIPTOR: CLUSTER_PROPERTY_TYPE = 15;
pub const CLUSPROP_TYPE_UNKNOWN: CLUSTER_PROPERTY_TYPE = -1;
pub const CLUSPROP_TYPE_USER: CLUSTER_PROPERTY_TYPE = 32768;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_ULARGE_INTEGER {
    pub Base: CLUSPROP_VALUE,
    pub li: u64,
}
impl Default for CLUSPROP_ULARGE_INTEGER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_VALUE {
    pub Syntax: CLUSPROP_SYNTAX,
    pub cbLength: u32,
}
impl Default for CLUSPROP_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSPROP_WORD {
    pub Base: CLUSPROP_VALUE,
    pub w: u16,
}
impl Default for CLUSPROP_WORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLUSREG_COMMAND_NONE: CLUSTER_REG_COMMAND = 0;
pub const CLUSREG_CONDITION_EXISTS: CLUSTER_REG_COMMAND = 11;
pub const CLUSREG_CONDITION_IS_EQUAL: CLUSTER_REG_COMMAND = 13;
pub const CLUSREG_CONDITION_IS_GREATER_THAN: CLUSTER_REG_COMMAND = 15;
pub const CLUSREG_CONDITION_IS_LESS_THAN: CLUSTER_REG_COMMAND = 16;
pub const CLUSREG_CONDITION_IS_NOT_EQUAL: CLUSTER_REG_COMMAND = 14;
pub const CLUSREG_CONDITION_KEY_EXISTS: CLUSTER_REG_COMMAND = 17;
pub const CLUSREG_CONDITION_KEY_NOT_EXISTS: CLUSTER_REG_COMMAND = 18;
pub const CLUSREG_CONDITION_NOT_EXISTS: CLUSTER_REG_COMMAND = 12;
pub const CLUSREG_CONTROL_COMMAND: CLUSTER_REG_COMMAND = 10;
pub const CLUSREG_CREATE_KEY: CLUSTER_REG_COMMAND = 2;
pub const CLUSREG_DATABASE_ISOLATE_READ: u32 = 2;
pub const CLUSREG_DATABASE_SYNC_WRITE_TO_ALL_NODES: u32 = 1;
pub const CLUSREG_DELETE_KEY: CLUSTER_REG_COMMAND = 3;
pub const CLUSREG_DELETE_VALUE: CLUSTER_REG_COMMAND = 4;
pub const CLUSREG_KEYNAME_OBJECTGUIDS: windows_core::PCWSTR = windows_core::w!("ObjectGUIDs");
pub const CLUSREG_LAST_COMMAND: CLUSTER_REG_COMMAND = 19;
pub const CLUSREG_NAME_ACCELERATED_NETWORKING_ENABLED: windows_core::PCWSTR = windows_core::w!("AcceleratedNetworkingEnabled");
pub const CLUSREG_NAME_ACCELERATED_NETWORKING_NODE_RESERVE: windows_core::PCWSTR = windows_core::w!("AcceleratedNetworkingNodeReserve");
pub const CLUSREG_NAME_AFFINITYRULE_ENABLED: windows_core::PCWSTR = windows_core::w!("Enabled");
pub const CLUSREG_NAME_AFFINITYRULE_GROUPS: windows_core::PCWSTR = windows_core::w!("Groups");
pub const CLUSREG_NAME_AFFINITYRULE_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const CLUSREG_NAME_AFFINITYRULE_SOFTANTIAFFINITY: windows_core::PCWSTR = windows_core::w!("SoftAntiAffinity");
pub const CLUSREG_NAME_AFFINITYRULE_TYPE: windows_core::PCWSTR = windows_core::w!("RuleType");
pub const CLUSREG_NAME_CLOUDWITNESS_ACCOUNT_NAME: windows_core::PCWSTR = windows_core::w!("AccountName");
pub const CLUSREG_NAME_CLOUDWITNESS_CONTAINER_NAME: windows_core::PCWSTR = windows_core::w!("ContainerName");
pub const CLUSREG_NAME_CLOUDWITNESS_ENDPOINT_INFO: windows_core::PCWSTR = windows_core::w!("EndpointInfo");
pub const CLUSREG_NAME_CLOUDWITNESS_MANAGED_IDENTITY: windows_core::PCWSTR = windows_core::w!("IsManagedIdentity");
pub const CLUSREG_NAME_CLOUDWITNESS_PRIMARY_KEY: windows_core::PCWSTR = windows_core::w!("PrimaryKey");
pub const CLUSREG_NAME_CLOUDWITNESS_PRIMARY_TOKEN: windows_core::PCWSTR = windows_core::w!("PrimaryToken");
pub const CLUSREG_NAME_CLUS_DEFAULT_NETWORK_ROLE: windows_core::PCWSTR = windows_core::w!("DefaultNetworkRole");
pub const CLUSREG_NAME_CLUS_DESC: windows_core::PCWSTR = windows_core::w!("Description");
pub const CLUSREG_NAME_CLUS_SD: windows_core::PCWSTR = windows_core::w!("Security Descriptor");
pub const CLUSREG_NAME_CROSS_SITE_DELAY: windows_core::PCWSTR = windows_core::w!("CrossSiteDelay");
pub const CLUSREG_NAME_CROSS_SITE_THRESHOLD: windows_core::PCWSTR = windows_core::w!("CrossSiteThreshold");
pub const CLUSREG_NAME_CROSS_SUBNET_DELAY: windows_core::PCWSTR = windows_core::w!("CrossSubnetDelay");
pub const CLUSREG_NAME_CROSS_SUBNET_THRESHOLD: windows_core::PCWSTR = windows_core::w!("CrossSubnetThreshold");
pub const CLUSREG_NAME_CSV_BLOCK_CACHE: windows_core::PCWSTR = windows_core::w!("BlockCacheSize");
pub const CLUSREG_NAME_CSV_MDS_SD: windows_core::PCWSTR = windows_core::w!("SharedVolumeSecurityDescriptor");
pub const CLUSREG_NAME_DATABASE_READ_WRITE_MODE: windows_core::PCWSTR = windows_core::w!("DatabaseReadWriteMode");
pub const CLUSREG_NAME_DDA_DEVICE_ALLOCATIONS: windows_core::PCWSTR = windows_core::w!("DdaDeviceAllocations");
pub const CLUSREG_NAME_DHCP_BACKUP_PATH: windows_core::PCWSTR = windows_core::w!("BackupPath");
pub const CLUSREG_NAME_DHCP_DATABASE_PATH: windows_core::PCWSTR = windows_core::w!("DatabasePath");
pub const CLUSREG_NAME_DRAIN_ON_SHUTDOWN: windows_core::PCWSTR = windows_core::w!("DrainOnShutdown");
pub const CLUSREG_NAME_ENABLED_EVENT_LOGS: windows_core::PCWSTR = windows_core::w!("EnabledEventLogs");
pub const CLUSREG_NAME_FAILOVER_MOVE_MIGRATION_TYPE: windows_core::PCWSTR = windows_core::w!("FailoverMoveMigrationType");
pub const CLUSREG_NAME_FILESHR_CA_TIMEOUT: windows_core::PCWSTR = windows_core::w!("CATimeout");
pub const CLUSREG_NAME_FILESHR_HIDE_SUBDIR_SHARES: windows_core::PCWSTR = windows_core::w!("HideSubDirShares");
pub const CLUSREG_NAME_FILESHR_IS_DFS_ROOT: windows_core::PCWSTR = windows_core::w!("IsDfsRoot");
pub const CLUSREG_NAME_FILESHR_MAX_USERS: windows_core::PCWSTR = windows_core::w!("MaxUsers");
pub const CLUSREG_NAME_FILESHR_PATH: windows_core::PCWSTR = windows_core::w!("Path");
pub const CLUSREG_NAME_FILESHR_QOS_FLOWSCOPE: windows_core::PCWSTR = windows_core::w!("QosFlowScope");
pub const CLUSREG_NAME_FILESHR_QOS_POLICYID: windows_core::PCWSTR = windows_core::w!("QosPolicyId");
pub const CLUSREG_NAME_FILESHR_REMARK: windows_core::PCWSTR = windows_core::w!("Remark");
pub const CLUSREG_NAME_FILESHR_SD: windows_core::PCWSTR = windows_core::w!("Security Descriptor");
pub const CLUSREG_NAME_FILESHR_SERVER_NAME: windows_core::PCWSTR = windows_core::w!("ServerName");
pub const CLUSREG_NAME_FILESHR_SHARE_FLAGS: windows_core::PCWSTR = windows_core::w!("ShareFlags");
pub const CLUSREG_NAME_FILESHR_SHARE_NAME: windows_core::PCWSTR = windows_core::w!("ShareName");
pub const CLUSREG_NAME_FILESHR_SHARE_SUBDIRS: windows_core::PCWSTR = windows_core::w!("ShareSubDirs");
pub const CLUSREG_NAME_FIXQUORUM: windows_core::PCWSTR = windows_core::w!("FixQuorum");
pub const CLUSREG_NAME_FSWITNESS_ARB_DELAY: windows_core::PCWSTR = windows_core::w!("ArbitrationDelay");
pub const CLUSREG_NAME_FSWITNESS_IMPERSONATE_CNO: windows_core::PCWSTR = windows_core::w!("ImpersonateCNO");
pub const CLUSREG_NAME_FSWITNESS_SHARE_PATH: windows_core::PCWSTR = windows_core::w!("SharePath");
pub const CLUSREG_NAME_FUNCTIONAL_LEVEL: windows_core::PCWSTR = windows_core::w!("ClusterFunctionalLevel");
pub const CLUSREG_NAME_GENAPP_COMMAND_LINE: windows_core::PCWSTR = windows_core::w!("CommandLine");
pub const CLUSREG_NAME_GENAPP_CURRENT_DIRECTORY: windows_core::PCWSTR = windows_core::w!("CurrentDirectory");
pub const CLUSREG_NAME_GENAPP_USE_NETWORK_NAME: windows_core::PCWSTR = windows_core::w!("UseNetworkName");
pub const CLUSREG_NAME_GENSCRIPT_SCRIPT_FILEPATH: windows_core::PCWSTR = windows_core::w!("ScriptFilepath");
pub const CLUSREG_NAME_GENSVC_SERVICE_NAME: windows_core::PCWSTR = windows_core::w!("ServiceName");
pub const CLUSREG_NAME_GENSVC_STARTUP_PARAMS: windows_core::PCWSTR = windows_core::w!("StartupParameters");
pub const CLUSREG_NAME_GENSVC_USE_NETWORK_NAME: windows_core::PCWSTR = windows_core::w!("UseNetworkName");
pub const CLUSREG_NAME_GPUP_DEVICE_ALLOCATIONS: windows_core::PCWSTR = windows_core::w!("GpupDeviceAllocations");
pub const CLUSREG_NAME_GROUPSET_AVAILABILITY_SET_INDEX_TO_NODE_MAPPING: windows_core::PCWSTR = windows_core::w!("NodeDomainInfo");
pub const CLUSREG_NAME_GROUPSET_FAULT_DOMAINS: windows_core::PCWSTR = windows_core::w!("FaultDomains");
pub const CLUSREG_NAME_GROUPSET_IS_AVAILABILITY_SET: windows_core::PCWSTR = windows_core::w!("IsAvailabilitySet");
pub const CLUSREG_NAME_GROUPSET_IS_GLOBAL: windows_core::PCWSTR = windows_core::w!("IsGlobal");
pub const CLUSREG_NAME_GROUPSET_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const CLUSREG_NAME_GROUPSET_RESERVE_NODE: windows_core::PCWSTR = windows_core::w!("ReserveSpareNode");
pub const CLUSREG_NAME_GROUPSET_STARTUP_COUNT: windows_core::PCWSTR = windows_core::w!("StartupCount");
pub const CLUSREG_NAME_GROUPSET_STARTUP_DELAY: windows_core::PCWSTR = windows_core::w!("StartupDelay");
pub const CLUSREG_NAME_GROUPSET_STARTUP_SETTING: windows_core::PCWSTR = windows_core::w!("StartupSetting");
pub const CLUSREG_NAME_GROUPSET_STATUS_INFORMATION: windows_core::PCWSTR = windows_core::w!("StatusInformation");
pub const CLUSREG_NAME_GROUPSET_UPDATE_DOMAINS: windows_core::PCWSTR = windows_core::w!("UpdateDomains");
pub const CLUSREG_NAME_GROUP_DEPENDENCY_TIMEOUT: windows_core::PCWSTR = windows_core::w!("GroupDependencyTimeout");
pub const CLUSREG_NAME_GRP_ANTI_AFFINITY_CLASS_NAME: windows_core::PCWSTR = windows_core::w!("AntiAffinityClassNames");
pub const CLUSREG_NAME_GRP_CCF_EPOCH: windows_core::PCWSTR = windows_core::w!("CCFEpoch");
pub const CLUSREG_NAME_GRP_CCF_EPOCH_HIGH: windows_core::PCWSTR = windows_core::w!("CCFEpochHigh");
pub const CLUSREG_NAME_GRP_COLD_START_SETTING: windows_core::PCWSTR = windows_core::w!("ColdStartSetting");
pub const CLUSREG_NAME_GRP_DEFAULT_OWNER: windows_core::PCWSTR = windows_core::w!("DefaultOwner");
pub const CLUSREG_NAME_GRP_DESC: windows_core::PCWSTR = windows_core::w!("Description");
pub const CLUSREG_NAME_GRP_FAILBACK_TYPE: windows_core::PCWSTR = windows_core::w!("AutoFailbackType");
pub const CLUSREG_NAME_GRP_FAILBACK_WIN_END: windows_core::PCWSTR = windows_core::w!("FailbackWindowEnd");
pub const CLUSREG_NAME_GRP_FAILBACK_WIN_START: windows_core::PCWSTR = windows_core::w!("FailbackWindowStart");
pub const CLUSREG_NAME_GRP_FAILOVER_PERIOD: windows_core::PCWSTR = windows_core::w!("FailoverPeriod");
pub const CLUSREG_NAME_GRP_FAILOVER_THRESHOLD: windows_core::PCWSTR = windows_core::w!("FailoverThreshold");
pub const CLUSREG_NAME_GRP_FAULT_DOMAIN: windows_core::PCWSTR = windows_core::w!("FaultDomain");
pub const CLUSREG_NAME_GRP_LOCK_MOVE: windows_core::PCWSTR = windows_core::w!("LockedFromMoving");
pub const CLUSREG_NAME_GRP_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const CLUSREG_NAME_GRP_PERSISTENT_STATE: windows_core::PCWSTR = windows_core::w!("PersistentState");
pub const CLUSREG_NAME_GRP_PLACEMENT_OPTIONS: windows_core::PCWSTR = windows_core::w!("PlacementOptions");
pub const CLUSREG_NAME_GRP_PREFERRED_SITE: windows_core::PCWSTR = windows_core::w!("PreferredSite");
pub const CLUSREG_NAME_GRP_PRIORITY: windows_core::PCWSTR = windows_core::w!("Priority");
pub const CLUSREG_NAME_GRP_RESILIENCY_PERIOD: windows_core::PCWSTR = windows_core::w!("ResiliencyPeriod");
pub const CLUSREG_NAME_GRP_START_DELAY: windows_core::PCWSTR = windows_core::w!("GroupStartDelay");
pub const CLUSREG_NAME_GRP_STATUS_INFORMATION: windows_core::PCWSTR = windows_core::w!("StatusInformation");
pub const CLUSREG_NAME_GRP_TYPE: windows_core::PCWSTR = windows_core::w!("GroupType");
pub const CLUSREG_NAME_GRP_UPDATE_DOMAIN: windows_core::PCWSTR = windows_core::w!("UpdateDomain");
pub const CLUSREG_NAME_IGNORE_PERSISTENT_STATE: windows_core::PCWSTR = windows_core::w!("IgnorePersistentStateOnStartup");
pub const CLUSREG_NAME_IPADDR_ADDRESS: windows_core::PCWSTR = windows_core::w!("Address");
pub const CLUSREG_NAME_IPADDR_DHCP_ADDRESS: windows_core::PCWSTR = windows_core::w!("DhcpAddress");
pub const CLUSREG_NAME_IPADDR_DHCP_SERVER: windows_core::PCWSTR = windows_core::w!("DhcpServer");
pub const CLUSREG_NAME_IPADDR_DHCP_SUBNET_MASK: windows_core::PCWSTR = windows_core::w!("DhcpSubnetMask");
pub const CLUSREG_NAME_IPADDR_ENABLE_DHCP: windows_core::PCWSTR = windows_core::w!("EnableDhcp");
pub const CLUSREG_NAME_IPADDR_ENABLE_NETBIOS: windows_core::PCWSTR = windows_core::w!("EnableNetBIOS");
pub const CLUSREG_NAME_IPADDR_LEASE_OBTAINED_TIME: windows_core::PCWSTR = windows_core::w!("LeaseObtainedTime");
pub const CLUSREG_NAME_IPADDR_LEASE_TERMINATES_TIME: windows_core::PCWSTR = windows_core::w!("LeaseExpiresTime");
pub const CLUSREG_NAME_IPADDR_NETWORK: windows_core::PCWSTR = windows_core::w!("Network");
pub const CLUSREG_NAME_IPADDR_OVERRIDE_ADDRMATCH: windows_core::PCWSTR = windows_core::w!("OverrideAddressMatch");
pub const CLUSREG_NAME_IPADDR_PROBE_FAILURE_THRESHOLD: windows_core::PCWSTR = windows_core::w!("ProbeFailureThreshold");
pub const CLUSREG_NAME_IPADDR_PROBE_PORT: windows_core::PCWSTR = windows_core::w!("ProbePort");
pub const CLUSREG_NAME_IPADDR_SHARED_NETNAME: windows_core::PCWSTR = windows_core::w!("SharedNetname");
pub const CLUSREG_NAME_IPADDR_SUBNET_MASK: windows_core::PCWSTR = windows_core::w!("SubnetMask");
pub const CLUSREG_NAME_IPADDR_T1: windows_core::PCWSTR = windows_core::w!("T1");
pub const CLUSREG_NAME_IPADDR_T2: windows_core::PCWSTR = windows_core::w!("T2");
pub const CLUSREG_NAME_IPV6_NATIVE_ADDRESS: windows_core::PCWSTR = windows_core::w!("Address");
pub const CLUSREG_NAME_IPV6_NATIVE_NETWORK: windows_core::PCWSTR = windows_core::w!("Network");
pub const CLUSREG_NAME_IPV6_NATIVE_PREFIX_LENGTH: windows_core::PCWSTR = windows_core::w!("PrefixLength");
pub const CLUSREG_NAME_IPV6_TUNNEL_ADDRESS: windows_core::PCWSTR = windows_core::w!("Address");
pub const CLUSREG_NAME_IPV6_TUNNEL_TUNNELTYPE: windows_core::PCWSTR = windows_core::w!("TunnelType");
pub const CLUSREG_NAME_KEYVALUESTORE_MANAGERNAME: windows_core::PCWSTR = windows_core::w!("ManagerName");
pub const CLUSREG_NAME_KEYVALUESTORE_MANAGERPATH: windows_core::PCWSTR = windows_core::w!("ManagerPath");
pub const CLUSREG_NAME_KEYVALUESTORE_NAME: windows_core::PCWSTR = windows_core::w!("KeyValueStores");
pub const CLUSREG_NAME_LAST_RECENT_EVENTS_RESET_TIME: windows_core::PCWSTR = windows_core::w!("RecentEventsResetTime");
pub const CLUSREG_NAME_LOG_FILE_PATH: windows_core::PCWSTR = windows_core::w!("LogFilePath");
pub const CLUSREG_NAME_MAX_PARALLEL_MIGRATIONS: windows_core::PCWSTR = windows_core::w!("MaximumParallelMigrations");
pub const CLUSREG_NAME_MESSAGE_BUFFER_LENGTH: windows_core::PCWSTR = windows_core::w!("MessageBufferLength");
pub const CLUSREG_NAME_MIXED_MODE: windows_core::PCWSTR = windows_core::w!("MixedMode");
pub const CLUSREG_NAME_NETFT_IPSEC_ENABLED: windows_core::PCWSTR = windows_core::w!("NetftIPSecEnabled");
pub const CLUSREG_NAME_NETIFACE_ADAPTER_ID: windows_core::PCWSTR = windows_core::w!("AdapterId");
pub const CLUSREG_NAME_NETIFACE_ADAPTER_NAME: windows_core::PCWSTR = windows_core::w!("Adapter");
pub const CLUSREG_NAME_NETIFACE_ADDRESS: windows_core::PCWSTR = windows_core::w!("Address");
pub const CLUSREG_NAME_NETIFACE_DESC: windows_core::PCWSTR = windows_core::w!("Description");
pub const CLUSREG_NAME_NETIFACE_DHCP_ENABLED: windows_core::PCWSTR = windows_core::w!("DhcpEnabled");
pub const CLUSREG_NAME_NETIFACE_IPV4_ADDRESSES: windows_core::PCWSTR = windows_core::w!("IPv4Addresses");
pub const CLUSREG_NAME_NETIFACE_IPV6_ADDRESSES: windows_core::PCWSTR = windows_core::w!("IPv6Addresses");
pub const CLUSREG_NAME_NETIFACE_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const CLUSREG_NAME_NETIFACE_NETWORK: windows_core::PCWSTR = windows_core::w!("Network");
pub const CLUSREG_NAME_NETIFACE_NODE: windows_core::PCWSTR = windows_core::w!("Node");
pub const CLUSREG_NAME_NETNAME_AD_AWARE: windows_core::PCWSTR = windows_core::w!("ADAware");
pub const CLUSREG_NAME_NETNAME_ALIASES: windows_core::PCWSTR = windows_core::w!("Aliases");
pub const CLUSREG_NAME_NETNAME_CONTAINERGUID: windows_core::PCWSTR = windows_core::w!("CryptoContainerGUID");
pub const CLUSREG_NAME_NETNAME_CREATING_DC: windows_core::PCWSTR = windows_core::w!("CreatingDC");
pub const CLUSREG_NAME_NETNAME_DNN_DISABLE_CLONES: windows_core::PCWSTR = windows_core::w!("DisableClones");
pub const CLUSREG_NAME_NETNAME_DNS_AUTHENTICATION: windows_core::PCWSTR = windows_core::w!("DnsAuthenticationMode");
pub const CLUSREG_NAME_NETNAME_DNS_NAME: windows_core::PCWSTR = windows_core::w!("DnsName");
pub const CLUSREG_NAME_NETNAME_DNS_SUFFIX: windows_core::PCWSTR = windows_core::w!("DnsSuffix");
pub const CLUSREG_NAME_NETNAME_EXCLUDE_NETWORKS: windows_core::PCWSTR = windows_core::w!("ExcludeNetworks");
pub const CLUSREG_NAME_NETNAME_HOST_TTL: windows_core::PCWSTR = windows_core::w!("HostRecordTTL");
pub const CLUSREG_NAME_NETNAME_IN_USE_NETWORKS: windows_core::PCWSTR = windows_core::w!("InUseNetworks");
pub const CLUSREG_NAME_NETNAME_LAST_DNS_UPDATE: windows_core::PCWSTR = windows_core::w!("LastDNSUpdateTime");
pub const CLUSREG_NAME_NETNAME_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const CLUSREG_NAME_NETNAME_OBJECT_ID: windows_core::PCWSTR = windows_core::w!("ObjectGUID");
pub const CLUSREG_NAME_NETNAME_PUBLISH_PTR: windows_core::PCWSTR = windows_core::w!("PublishPTRRecords");
pub const CLUSREG_NAME_NETNAME_REGISTER_ALL_IP: windows_core::PCWSTR = windows_core::w!("RegisterAllProvidersIP");
pub const CLUSREG_NAME_NETNAME_REMAP_PIPE_NAMES: windows_core::PCWSTR = windows_core::w!("RemapPipeNames");
pub const CLUSREG_NAME_NETNAME_REMOVEVCO_ONDELETE: windows_core::PCWSTR = windows_core::w!("DeleteVcoOnResCleanup");
pub const CLUSREG_NAME_NETNAME_RESOURCE_DATA: windows_core::PCWSTR = windows_core::w!("ResourceData");
pub const CLUSREG_NAME_NETNAME_STATUS_DNS: windows_core::PCWSTR = windows_core::w!("StatusDNS");
pub const CLUSREG_NAME_NETNAME_STATUS_KERBEROS: windows_core::PCWSTR = windows_core::w!("StatusKerberos");
pub const CLUSREG_NAME_NETNAME_STATUS_NETBIOS: windows_core::PCWSTR = windows_core::w!("StatusNetBIOS");
pub const CLUSREG_NAME_NETNAME_USE_DNS: windows_core::PCWSTR = windows_core::w!("DnsEnabled");
pub const CLUSREG_NAME_NETNAME_USE_DYNAMIC_DNS: windows_core::PCWSTR = windows_core::w!("UseDynamicDNS");
pub const CLUSREG_NAME_NETNAME_VCO_CONTAINER: windows_core::PCWSTR = windows_core::w!("VcoContainer");
pub const CLUSREG_NAME_NET_ADDRESS: windows_core::PCWSTR = windows_core::w!("Address");
pub const CLUSREG_NAME_NET_ADDRESS_MASK: windows_core::PCWSTR = windows_core::w!("AddressMask");
pub const CLUSREG_NAME_NET_AUTOMETRIC: windows_core::PCWSTR = windows_core::w!("AutoMetric");
pub const CLUSREG_NAME_NET_DESC: windows_core::PCWSTR = windows_core::w!("Description");
pub const CLUSREG_NAME_NET_IPV4_ADDRESSES: windows_core::PCWSTR = windows_core::w!("IPv4Addresses");
pub const CLUSREG_NAME_NET_IPV4_PREFIXLENGTHS: windows_core::PCWSTR = windows_core::w!("IPv4PrefixLengths");
pub const CLUSREG_NAME_NET_IPV6_ADDRESSES: windows_core::PCWSTR = windows_core::w!("IPv6Addresses");
pub const CLUSREG_NAME_NET_IPV6_PREFIXLENGTHS: windows_core::PCWSTR = windows_core::w!("IPv6PrefixLengths");
pub const CLUSREG_NAME_NET_METRIC: windows_core::PCWSTR = windows_core::w!("Metric");
pub const CLUSREG_NAME_NET_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const CLUSREG_NAME_NET_RDMA_CAPABLE: windows_core::PCWSTR = windows_core::w!("RdmaCapable");
pub const CLUSREG_NAME_NET_ROLE: windows_core::PCWSTR = windows_core::w!("Role");
pub const CLUSREG_NAME_NET_RSS_CAPABLE: windows_core::PCWSTR = windows_core::w!("RssCapable");
pub const CLUSREG_NAME_NET_SPEED: windows_core::PCWSTR = windows_core::w!("LinkSpeed");
pub const CLUSREG_NAME_NODE_BUILD_NUMBER: windows_core::PCWSTR = windows_core::w!("BuildNumber");
pub const CLUSREG_NAME_NODE_CSDVERSION: windows_core::PCWSTR = windows_core::w!("CSDVersion");
pub const CLUSREG_NAME_NODE_DESC: windows_core::PCWSTR = windows_core::w!("Description");
pub const CLUSREG_NAME_NODE_DRAIN_ERROR_CODE: windows_core::PCWSTR = windows_core::w!("DrainErrorCode");
pub const CLUSREG_NAME_NODE_DRAIN_STATUS: windows_core::PCWSTR = windows_core::w!("NodeDrainStatus");
pub const CLUSREG_NAME_NODE_DRAIN_TARGET: windows_core::PCWSTR = windows_core::w!("NodeDrainTarget");
pub const CLUSREG_NAME_NODE_DYNAMIC_WEIGHT: windows_core::PCWSTR = windows_core::w!("DynamicWeight");
pub const CLUSREG_NAME_NODE_FAILBACK_ERROR_CODE: windows_core::PCWSTR = windows_core::w!("FailbackErrorCode");
pub const CLUSREG_NAME_NODE_FAILBACK_STATUS: windows_core::PCWSTR = windows_core::w!("NodeFailbackStatus");
pub const CLUSREG_NAME_NODE_FAULT_DOMAIN: windows_core::PCWSTR = windows_core::w!("FaultDomain");
pub const CLUSREG_NAME_NODE_FDID: windows_core::PCWSTR = windows_core::w!("FaultDomainId");
pub const CLUSREG_NAME_NODE_HIGHEST_VERSION: windows_core::PCWSTR = windows_core::w!("NodeHighestVersion");
pub const CLUSREG_NAME_NODE_HYPERTHREADING_ENABLED: windows_core::PCWSTR = windows_core::w!("HyperthreadingEnabled");
pub const CLUSREG_NAME_NODE_IS_PRIMARY: windows_core::PCWSTR = windows_core::w!("IsPrimary");
pub const CLUSREG_NAME_NODE_LOWEST_VERSION: windows_core::PCWSTR = windows_core::w!("NodeLowestVersion");
pub const CLUSREG_NAME_NODE_MAJOR_VERSION: windows_core::PCWSTR = windows_core::w!("MajorVersion");
pub const CLUSREG_NAME_NODE_MANUFACTURER: windows_core::PCWSTR = windows_core::w!("Manufacturer");
pub const CLUSREG_NAME_NODE_MINOR_VERSION: windows_core::PCWSTR = windows_core::w!("MinorVersion");
pub const CLUSREG_NAME_NODE_MODEL: windows_core::PCWSTR = windows_core::w!("Model");
pub const CLUSREG_NAME_NODE_NAME: windows_core::PCWSTR = windows_core::w!("NodeName");
pub const CLUSREG_NAME_NODE_NEEDS_PQ: windows_core::PCWSTR = windows_core::w!("NeedsPreventQuorum");
pub const CLUSREG_NAME_NODE_SERIALNUMBER: windows_core::PCWSTR = windows_core::w!("SerialNumber");
pub const CLUSREG_NAME_NODE_STATUS_INFO: windows_core::PCWSTR = windows_core::w!("StatusInformation");
pub const CLUSREG_NAME_NODE_UNIQUEID: windows_core::PCWSTR = windows_core::w!("UniqueID");
pub const CLUSREG_NAME_NODE_WEIGHT: windows_core::PCWSTR = windows_core::w!("NodeWeight");
pub const CLUSREG_NAME_PHYSDISK_CSVBLOCKCACHE: windows_core::PCWSTR = windows_core::w!("EnableBlockCache");
pub const CLUSREG_NAME_PHYSDISK_CSVSNAPSHOTAGELIMIT: windows_core::PCWSTR = windows_core::w!("SnapshotAgeLimit");
pub const CLUSREG_NAME_PHYSDISK_CSVSNAPSHOTDIFFAREASIZE: windows_core::PCWSTR = windows_core::w!("SnapshotDiffSize");
pub const CLUSREG_NAME_PHYSDISK_CSVWRITETHROUGH: windows_core::PCWSTR = windows_core::w!("CsvEnforceWriteThrough");
pub const CLUSREG_NAME_PHYSDISK_DISKARBINTERVAL: windows_core::PCWSTR = windows_core::w!("DiskArbInterval");
pub const CLUSREG_NAME_PHYSDISK_DISKARBTYPE: windows_core::PCWSTR = windows_core::w!("DiskArbType");
pub const CLUSREG_NAME_PHYSDISK_DISKGUID: windows_core::PCWSTR = windows_core::w!("DiskGuid");
pub const CLUSREG_NAME_PHYSDISK_DISKIDGUID: windows_core::PCWSTR = windows_core::w!("DiskIdGuid");
pub const CLUSREG_NAME_PHYSDISK_DISKIDTYPE: windows_core::PCWSTR = windows_core::w!("DiskIdType");
pub const CLUSREG_NAME_PHYSDISK_DISKIODELAY: windows_core::PCWSTR = windows_core::w!("MaxIoLatency");
pub const CLUSREG_NAME_PHYSDISK_DISKPATH: windows_core::PCWSTR = windows_core::w!("DiskPath");
pub const CLUSREG_NAME_PHYSDISK_DISKRECOVERYACTION: windows_core::PCWSTR = windows_core::w!("DiskRecoveryAction");
pub const CLUSREG_NAME_PHYSDISK_DISKRELOAD: windows_core::PCWSTR = windows_core::w!("DiskReload");
pub const CLUSREG_NAME_PHYSDISK_DISKRUNCHKDSK: windows_core::PCWSTR = windows_core::w!("DiskRunChkDsk");
pub const CLUSREG_NAME_PHYSDISK_DISKSIGNATURE: windows_core::PCWSTR = windows_core::w!("DiskSignature");
pub const CLUSREG_NAME_PHYSDISK_DISKUNIQUEIDS: windows_core::PCWSTR = windows_core::w!("DiskUniqueIds");
pub const CLUSREG_NAME_PHYSDISK_DISKVOLUMEINFO: windows_core::PCWSTR = windows_core::w!("DiskVolumeInfo");
pub const CLUSREG_NAME_PHYSDISK_FASTONLINEARBITRATE: windows_core::PCWSTR = windows_core::w!("FastOnlineArbitrate");
pub const CLUSREG_NAME_PHYSDISK_MAINTMODE: windows_core::PCWSTR = windows_core::w!("MaintenanceMode");
pub const CLUSREG_NAME_PHYSDISK_MIGRATEFIXUP: windows_core::PCWSTR = windows_core::w!("MigrateDriveLetters");
pub const CLUSREG_NAME_PHYSDISK_SPACEIDGUID: windows_core::PCWSTR = windows_core::w!("VirtualDiskId");
pub const CLUSREG_NAME_PHYSDISK_VOLSNAPACTIVATETIMEOUT: windows_core::PCWSTR = windows_core::w!("VolsnapActivateTimeout");
pub const CLUSREG_NAME_PLACEMENT_OPTIONS: windows_core::PCWSTR = windows_core::w!("PlacementOptions");
pub const CLUSREG_NAME_PLUMB_ALL_CROSS_SUBNET_ROUTES: windows_core::PCWSTR = windows_core::w!("PlumbAllCrossSubnetRoutes");
pub const CLUSREG_NAME_PREVENTQUORUM: windows_core::PCWSTR = windows_core::w!("PreventQuorum");
pub const CLUSREG_NAME_PRTSPOOL_DEFAULT_SPOOL_DIR: windows_core::PCWSTR = windows_core::w!("DefaultSpoolDirectory");
pub const CLUSREG_NAME_PRTSPOOL_TIMEOUT: windows_core::PCWSTR = windows_core::w!("JobCompletionTimeout");
pub const CLUSREG_NAME_QUARANTINE_DURATION: windows_core::PCWSTR = windows_core::w!("QuarantineDuration");
pub const CLUSREG_NAME_QUARANTINE_THRESHOLD: windows_core::PCWSTR = windows_core::w!("QuarantineThreshold");
pub const CLUSREG_NAME_QUORUM_ARBITRATION_TIMEOUT: windows_core::PCWSTR = windows_core::w!("QuorumArbitrationTimeMax");
pub const CLUSREG_NAME_RESILIENCY_DEFAULT_SECONDS: windows_core::PCWSTR = windows_core::w!("ResiliencyDefaultPeriod");
pub const CLUSREG_NAME_RESILIENCY_LEVEL: windows_core::PCWSTR = windows_core::w!("ResiliencyLevel");
pub const CLUSREG_NAME_RESTYPE_ADMIN_EXTENSIONS: windows_core::PCWSTR = windows_core::w!("AdminExtensions");
pub const CLUSREG_NAME_RESTYPE_DESC: windows_core::PCWSTR = windows_core::w!("Description");
pub const CLUSREG_NAME_RESTYPE_DLL_NAME: windows_core::PCWSTR = windows_core::w!("DllName");
pub const CLUSREG_NAME_RESTYPE_DUMP_LOG_QUERY: windows_core::PCWSTR = windows_core::w!("DumpLogQuery");
pub const CLUSREG_NAME_RESTYPE_DUMP_POLICY: windows_core::PCWSTR = windows_core::w!("DumpPolicy");
pub const CLUSREG_NAME_RESTYPE_DUMP_SERVICES: windows_core::PCWSTR = windows_core::w!("DumpServices");
pub const CLUSREG_NAME_RESTYPE_ENABLED_EVENT_LOGS: windows_core::PCWSTR = windows_core::w!("EnabledEventLogs");
pub const CLUSREG_NAME_RESTYPE_MAX_MONITORS: windows_core::PCWSTR = windows_core::w!("MaximumMonitors");
pub const CLUSREG_NAME_RESTYPE_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const CLUSREG_NAME_RESTYPE_WPR_PROFILES: windows_core::PCWSTR = windows_core::w!("WprProfiles");
pub const CLUSREG_NAME_RESTYPE_WPR_START_AFTER: windows_core::PCWSTR = windows_core::w!("WprStartAfter");
pub const CLUSREG_NAME_RES_DATA1: windows_core::PCWSTR = windows_core::w!("ResourceSpecificData1");
pub const CLUSREG_NAME_RES_DATA2: windows_core::PCWSTR = windows_core::w!("ResourceSpecificData2");
pub const CLUSREG_NAME_RES_DEADLOCK_TIMEOUT: windows_core::PCWSTR = windows_core::w!("DeadlockTimeout");
pub const CLUSREG_NAME_RES_DESC: windows_core::PCWSTR = windows_core::w!("Description");
pub const CLUSREG_NAME_RES_EMBEDDED_FAILURE_ACTION: windows_core::PCWSTR = windows_core::w!("EmbeddedFailureAction");
pub const CLUSREG_NAME_RES_IS_ALIVE: windows_core::PCWSTR = windows_core::w!("IsAlivePollInterval");
pub const CLUSREG_NAME_RES_LAST_OPERATION_STATUS_CODE: windows_core::PCWSTR = windows_core::w!("LastOperationStatusCode");
pub const CLUSREG_NAME_RES_LOOKS_ALIVE: windows_core::PCWSTR = windows_core::w!("LooksAlivePollInterval");
pub const CLUSREG_NAME_RES_MONITOR_PID: windows_core::PCWSTR = windows_core::w!("MonitorProcessId");
pub const CLUSREG_NAME_RES_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const CLUSREG_NAME_RES_PENDING_TIMEOUT: windows_core::PCWSTR = windows_core::w!("PendingTimeout");
pub const CLUSREG_NAME_RES_PERSISTENT_STATE: windows_core::PCWSTR = windows_core::w!("PersistentState");
pub const CLUSREG_NAME_RES_RESTART_ACTION: windows_core::PCWSTR = windows_core::w!("RestartAction");
pub const CLUSREG_NAME_RES_RESTART_DELAY: windows_core::PCWSTR = windows_core::w!("RestartDelay");
pub const CLUSREG_NAME_RES_RESTART_PERIOD: windows_core::PCWSTR = windows_core::w!("RestartPeriod");
pub const CLUSREG_NAME_RES_RESTART_THRESHOLD: windows_core::PCWSTR = windows_core::w!("RestartThreshold");
pub const CLUSREG_NAME_RES_RETRY_PERIOD_ON_FAILURE: windows_core::PCWSTR = windows_core::w!("RetryPeriodOnFailure");
pub const CLUSREG_NAME_RES_SEPARATE_MONITOR: windows_core::PCWSTR = windows_core::w!("SeparateMonitor");
pub const CLUSREG_NAME_RES_STATUS: windows_core::PCWSTR = windows_core::w!("ResourceSpecificStatus");
pub const CLUSREG_NAME_RES_STATUS_INFORMATION: windows_core::PCWSTR = windows_core::w!("StatusInformation");
pub const CLUSREG_NAME_RES_TYPE: windows_core::PCWSTR = windows_core::w!("Type");
pub const CLUSREG_NAME_ROUTE_HISTORY_LENGTH: windows_core::PCWSTR = windows_core::w!("RouteHistoryLength");
pub const CLUSREG_NAME_SAME_SUBNET_DELAY: windows_core::PCWSTR = windows_core::w!("SameSubnetDelay");
pub const CLUSREG_NAME_SAME_SUBNET_THRESHOLD: windows_core::PCWSTR = windows_core::w!("SameSubnetThreshold");
pub const CLUSREG_NAME_SHUTDOWN_TIMEOUT_MINUTES: windows_core::PCWSTR = windows_core::w!("ShutdownTimeoutInMinutes");
pub const CLUSREG_NAME_SOFS_SMBASYMMETRYMODE: windows_core::PCWSTR = windows_core::w!("SmbAsymmetryMode");
pub const CLUSREG_NAME_START_MEMORY: windows_core::PCWSTR = windows_core::w!("StartMemory");
pub const CLUSREG_NAME_STORAGESPACE_DESCRIPTION: windows_core::PCWSTR = windows_core::w!("VirtualDiskDescription");
pub const CLUSREG_NAME_STORAGESPACE_HEALTH: windows_core::PCWSTR = windows_core::w!("VirtualDiskHealth");
pub const CLUSREG_NAME_STORAGESPACE_NAME: windows_core::PCWSTR = windows_core::w!("VirtualDiskName");
pub const CLUSREG_NAME_STORAGESPACE_POOLARBITRATE: windows_core::PCWSTR = windows_core::w!("Arbitrate");
pub const CLUSREG_NAME_STORAGESPACE_POOLCONSUMEDCAPACITY: windows_core::PCWSTR = windows_core::w!("ConsumedCapacity");
pub const CLUSREG_NAME_STORAGESPACE_POOLDESC: windows_core::PCWSTR = windows_core::w!("Description");
pub const CLUSREG_NAME_STORAGESPACE_POOLDRIVEIDS: windows_core::PCWSTR = windows_core::w!("DriveIds");
pub const CLUSREG_NAME_STORAGESPACE_POOLHEALTH: windows_core::PCWSTR = windows_core::w!("Health");
pub const CLUSREG_NAME_STORAGESPACE_POOLIDGUID: windows_core::PCWSTR = windows_core::w!("PoolId");
pub const CLUSREG_NAME_STORAGESPACE_POOLNAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const CLUSREG_NAME_STORAGESPACE_POOLQUORUMSHARE: windows_core::PCWSTR = windows_core::w!("PoolQuorumShare");
pub const CLUSREG_NAME_STORAGESPACE_POOLQUORUMUSERACCOUNT: windows_core::PCWSTR = windows_core::w!("PoolQuorumUserAccount");
pub const CLUSREG_NAME_STORAGESPACE_POOLREEVALTIMEOUT: windows_core::PCWSTR = windows_core::w!("ReEvaluatePlacementTimeout");
pub const CLUSREG_NAME_STORAGESPACE_POOLSTATE: windows_core::PCWSTR = windows_core::w!("State");
pub const CLUSREG_NAME_STORAGESPACE_POOLTOTALCAPACITY: windows_core::PCWSTR = windows_core::w!("TotalCapacity");
pub const CLUSREG_NAME_STORAGESPACE_PROVISIONING: windows_core::PCWSTR = windows_core::w!("VirtualDiskProvisioning");
pub const CLUSREG_NAME_STORAGESPACE_RESILIENCYCOLUMNS: windows_core::PCWSTR = windows_core::w!("VirtualDiskResiliencyColumns");
pub const CLUSREG_NAME_STORAGESPACE_RESILIENCYINTERLEAVE: windows_core::PCWSTR = windows_core::w!("VirtualDiskResiliencyInterleave");
pub const CLUSREG_NAME_STORAGESPACE_RESILIENCYTYPE: windows_core::PCWSTR = windows_core::w!("VirtualDiskResiliencyType");
pub const CLUSREG_NAME_STORAGESPACE_STATE: windows_core::PCWSTR = windows_core::w!("VirtualDiskState");
pub const CLUSREG_NAME_UPGRADE_VERSION: windows_core::PCWSTR = windows_core::w!("ClusterUpgradeVersion");
pub const CLUSREG_NAME_VIP_ADAPTER_NAME: windows_core::PCWSTR = windows_core::w!("AdapterName");
pub const CLUSREG_NAME_VIP_ADDRESS: windows_core::PCWSTR = windows_core::w!("Address");
pub const CLUSREG_NAME_VIP_PREFIX_LENGTH: windows_core::PCWSTR = windows_core::w!("PrefixLength");
pub const CLUSREG_NAME_VIP_RDID: windows_core::PCWSTR = windows_core::w!("RDID");
pub const CLUSREG_NAME_VIP_VSID: windows_core::PCWSTR = windows_core::w!("VSID");
pub const CLUSREG_NAME_VIRTUAL_NUMA_COUNT: windows_core::PCWSTR = windows_core::w!("VirtualNumaCount");
pub const CLUSREG_NAME_VSSTASK_APPNAME: windows_core::PCWSTR = windows_core::w!("ApplicationName");
pub const CLUSREG_NAME_VSSTASK_APPPARAMS: windows_core::PCWSTR = windows_core::w!("ApplicationParams");
pub const CLUSREG_NAME_VSSTASK_CURRENTDIRECTORY: windows_core::PCWSTR = windows_core::w!("CurrentDirectory");
pub const CLUSREG_NAME_VSSTASK_TRIGGERARRAY: windows_core::PCWSTR = windows_core::w!("TriggerArray");
pub const CLUSREG_NAME_WINS_BACKUP_PATH: windows_core::PCWSTR = windows_core::w!("BackupPath");
pub const CLUSREG_NAME_WINS_DATABASE_PATH: windows_core::PCWSTR = windows_core::w!("DatabasePath");
pub const CLUSREG_NAME_WITNESS_DYNAMIC_WEIGHT: windows_core::PCWSTR = windows_core::w!("WitnessDynamicWeight");
pub const CLUSREG_READ_ERROR: CLUSTER_REG_COMMAND = 9;
pub const CLUSREG_READ_KEY: CLUSTER_REG_COMMAND = 7;
pub const CLUSREG_READ_VALUE: CLUSTER_REG_COMMAND = 8;
pub const CLUSREG_SET_KEY_SECURITY: CLUSTER_REG_COMMAND = 5;
pub const CLUSREG_SET_VALUE: CLUSTER_REG_COMMAND = 1;
pub const CLUSREG_VALUE_DELETED: CLUSTER_REG_COMMAND = 6;
pub const CLUSRES_STATUS_APPLICATION_READY: u32 = 256;
pub const CLUSRES_STATUS_EMBEDDED_FAILURE: u32 = 2;
pub const CLUSRES_STATUS_FAILED_DUE_TO_INSUFFICIENT_CPU: u32 = 4;
pub const CLUSRES_STATUS_FAILED_DUE_TO_INSUFFICIENT_GENERIC_RESOURCES: u32 = 16;
pub const CLUSRES_STATUS_FAILED_DUE_TO_INSUFFICIENT_MEMORY: u32 = 8;
pub const CLUSRES_STATUS_LOCKED_MODE: u32 = 1;
pub const CLUSRES_STATUS_NETWORK_FAILURE: u32 = 32;
pub const CLUSRES_STATUS_OFFLINE_NOT_LOCAL_DISK_OWNER: u32 = 512;
pub const CLUSRES_STATUS_OS_HEARTBEAT: u32 = 128;
pub const CLUSRES_STATUS_UNMONITORED: u32 = 64;
pub type CLUSTERSET_OBJECT_TYPE = i32;
pub const CLUSTERSET_OBJECT_TYPE_DATABASE: CLUSTERSET_OBJECT_TYPE = 3;
pub const CLUSTERSET_OBJECT_TYPE_MEMBER: CLUSTERSET_OBJECT_TYPE = 1;
pub const CLUSTERSET_OBJECT_TYPE_NONE: CLUSTERSET_OBJECT_TYPE = 0;
pub const CLUSTERSET_OBJECT_TYPE_WORKLOAD: CLUSTERSET_OBJECT_TYPE = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Default for CLUSTERVERSIONINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTERVERSIONINFO_NT4 {
    pub dwVersionInfoSize: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub BuildNumber: u16,
    pub szVendorId: [u16; 64],
    pub szCSDVersion: [u16; 64],
}
impl Default for CLUSTERVERSIONINFO_NT4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLUSTER_ADD_EVICT_DELAY: windows_core::PCWSTR = windows_core::w!("AddEvictDelay");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUSTER_AVAILABILITY_SET_CONFIG {
    pub dwVersion: u32,
    pub dwUpdateDomains: u32,
    pub dwFaultDomains: u32,
    pub bReserveSpareNode: windows_core::BOOL,
}
pub const CLUSTER_AVAILABILITY_SET_CONFIG_V1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTER_BATCH_COMMAND {
    pub Command: CLUSTER_REG_COMMAND,
    pub dwOptions: u32,
    pub wzName: windows_core::PCWSTR,
    pub lpData: *const u8,
    pub cbData: u32,
}
impl Default for CLUSTER_BATCH_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CLUSTER_CHANGE = i32;
pub const CLUSTER_CHANGE_ALL: CLUSTER_CHANGE = -1;
pub const CLUSTER_CHANGE_CLUSTER_ALL_V2: CLUSTER_CHANGE_CLUSTER_V2 = 8191;
pub const CLUSTER_CHANGE_CLUSTER_COMMON_PROPERTY_V2: CLUSTER_CHANGE_CLUSTER_V2 = 128;
pub const CLUSTER_CHANGE_CLUSTER_GROUP_ADDED_V2: CLUSTER_CHANGE_CLUSTER_V2 = 4;
pub const CLUSTER_CHANGE_CLUSTER_HANDLE_CLOSE_V2: CLUSTER_CHANGE_CLUSTER_V2 = 8;
pub const CLUSTER_CHANGE_CLUSTER_LOST_NOTIFICATIONS_V2: CLUSTER_CHANGE_CLUSTER_V2 = 512;
pub const CLUSTER_CHANGE_CLUSTER_MEMBERSHIP_V2: CLUSTER_CHANGE_CLUSTER_V2 = 2048;
pub const CLUSTER_CHANGE_CLUSTER_NETWORK_ADDED_V2: CLUSTER_CHANGE_CLUSTER_V2 = 16;
pub const CLUSTER_CHANGE_CLUSTER_NODE_ADDED_V2: CLUSTER_CHANGE_CLUSTER_V2 = 32;
pub const CLUSTER_CHANGE_CLUSTER_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_CLUSTER_V2 = 256;
pub const CLUSTER_CHANGE_CLUSTER_PROPERTY: CLUSTER_CHANGE = 1073741824;
pub const CLUSTER_CHANGE_CLUSTER_RECONNECT: CLUSTER_CHANGE = 524288;
pub const CLUSTER_CHANGE_CLUSTER_RECONNECT_V2: CLUSTER_CHANGE_CLUSTER_V2 = 1;
pub const CLUSTER_CHANGE_CLUSTER_RENAME_V2: CLUSTER_CHANGE_CLUSTER_V2 = 1024;
pub const CLUSTER_CHANGE_CLUSTER_RESOURCE_TYPE_ADDED_V2: CLUSTER_CHANGE_CLUSTER_V2 = 64;
pub const CLUSTER_CHANGE_CLUSTER_STATE: CLUSTER_CHANGE = 536870912;
pub const CLUSTER_CHANGE_CLUSTER_STATE_V2: CLUSTER_CHANGE_CLUSTER_V2 = 2;
pub const CLUSTER_CHANGE_CLUSTER_UPGRADED_V2: CLUSTER_CHANGE_CLUSTER_V2 = 4096;
pub type CLUSTER_CHANGE_CLUSTER_V2 = i32;
pub const CLUSTER_CHANGE_GROUPSET_ALL_V2: CLUSTER_CHANGE_GROUPSET_V2 = 511;
pub const CLUSTER_CHANGE_GROUPSET_COMMON_PROPERTY_V2: CLUSTER_CHANGE_GROUPSET_V2 = 2;
pub const CLUSTER_CHANGE_GROUPSET_DELETED_v2: CLUSTER_CHANGE_GROUPSET_V2 = 1;
pub const CLUSTER_CHANGE_GROUPSET_DEPENDENCIES_V2: CLUSTER_CHANGE_GROUPSET_V2 = 64;
pub const CLUSTER_CHANGE_GROUPSET_DEPENDENTS_V2: CLUSTER_CHANGE_GROUPSET_V2 = 128;
pub const CLUSTER_CHANGE_GROUPSET_GROUP_ADDED: CLUSTER_CHANGE_GROUPSET_V2 = 16;
pub const CLUSTER_CHANGE_GROUPSET_GROUP_REMOVED: CLUSTER_CHANGE_GROUPSET_V2 = 32;
pub const CLUSTER_CHANGE_GROUPSET_HANDLE_CLOSE_v2: CLUSTER_CHANGE_GROUPSET_V2 = 256;
pub const CLUSTER_CHANGE_GROUPSET_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_GROUPSET_V2 = 4;
pub const CLUSTER_CHANGE_GROUPSET_STATE_V2: CLUSTER_CHANGE_GROUPSET_V2 = 8;
pub type CLUSTER_CHANGE_GROUPSET_V2 = i32;
pub const CLUSTER_CHANGE_GROUP_ADDED: CLUSTER_CHANGE = 16384;
pub const CLUSTER_CHANGE_GROUP_ALL_V2: CLUSTER_CHANGE_GROUP_V2 = 1023;
pub const CLUSTER_CHANGE_GROUP_COMMON_PROPERTY_V2: CLUSTER_CHANGE_GROUP_V2 = 2;
pub const CLUSTER_CHANGE_GROUP_DELETED: CLUSTER_CHANGE = 8192;
pub const CLUSTER_CHANGE_GROUP_DELETED_V2: CLUSTER_CHANGE_GROUP_V2 = 1;
pub const CLUSTER_CHANGE_GROUP_HANDLE_CLOSE_V2: CLUSTER_CHANGE_GROUP_V2 = 512;
pub const CLUSTER_CHANGE_GROUP_OWNER_NODE_V2: CLUSTER_CHANGE_GROUP_V2 = 16;
pub const CLUSTER_CHANGE_GROUP_PREFERRED_OWNERS_V2: CLUSTER_CHANGE_GROUP_V2 = 32;
pub const CLUSTER_CHANGE_GROUP_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_GROUP_V2 = 4;
pub const CLUSTER_CHANGE_GROUP_PROPERTY: CLUSTER_CHANGE = 32768;
pub const CLUSTER_CHANGE_GROUP_RESOURCE_ADDED_V2: CLUSTER_CHANGE_GROUP_V2 = 64;
pub const CLUSTER_CHANGE_GROUP_RESOURCE_GAINED_V2: CLUSTER_CHANGE_GROUP_V2 = 128;
pub const CLUSTER_CHANGE_GROUP_RESOURCE_LOST_V2: CLUSTER_CHANGE_GROUP_V2 = 256;
pub const CLUSTER_CHANGE_GROUP_STATE: CLUSTER_CHANGE = 4096;
pub const CLUSTER_CHANGE_GROUP_STATE_V2: CLUSTER_CHANGE_GROUP_V2 = 8;
pub type CLUSTER_CHANGE_GROUP_V2 = i32;
pub const CLUSTER_CHANGE_HANDLE_CLOSE: CLUSTER_CHANGE = -2147483648;
pub const CLUSTER_CHANGE_NETINTERFACE_ADDED: CLUSTER_CHANGE = 67108864;
pub const CLUSTER_CHANGE_NETINTERFACE_ALL_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = 31;
pub const CLUSTER_CHANGE_NETINTERFACE_COMMON_PROPERTY_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = 2;
pub const CLUSTER_CHANGE_NETINTERFACE_DELETED: CLUSTER_CHANGE = 33554432;
pub const CLUSTER_CHANGE_NETINTERFACE_DELETED_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = 1;
pub const CLUSTER_CHANGE_NETINTERFACE_HANDLE_CLOSE_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = 16;
pub const CLUSTER_CHANGE_NETINTERFACE_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = 4;
pub const CLUSTER_CHANGE_NETINTERFACE_PROPERTY: CLUSTER_CHANGE = 134217728;
pub const CLUSTER_CHANGE_NETINTERFACE_STATE: CLUSTER_CHANGE = 16777216;
pub const CLUSTER_CHANGE_NETINTERFACE_STATE_V2: CLUSTER_CHANGE_NETINTERFACE_V2 = 8;
pub type CLUSTER_CHANGE_NETINTERFACE_V2 = i32;
pub const CLUSTER_CHANGE_NETWORK_ADDED: CLUSTER_CHANGE = 4194304;
pub const CLUSTER_CHANGE_NETWORK_ALL_V2: CLUSTER_CHANGE_NETWORK_V2 = 31;
pub const CLUSTER_CHANGE_NETWORK_COMMON_PROPERTY_V2: CLUSTER_CHANGE_NETWORK_V2 = 2;
pub const CLUSTER_CHANGE_NETWORK_DELETED: CLUSTER_CHANGE = 2097152;
pub const CLUSTER_CHANGE_NETWORK_DELETED_V2: CLUSTER_CHANGE_NETWORK_V2 = 1;
pub const CLUSTER_CHANGE_NETWORK_HANDLE_CLOSE_V2: CLUSTER_CHANGE_NETWORK_V2 = 16;
pub const CLUSTER_CHANGE_NETWORK_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_NETWORK_V2 = 4;
pub const CLUSTER_CHANGE_NETWORK_PROPERTY: CLUSTER_CHANGE = 8388608;
pub const CLUSTER_CHANGE_NETWORK_STATE: CLUSTER_CHANGE = 1048576;
pub const CLUSTER_CHANGE_NETWORK_STATE_V2: CLUSTER_CHANGE_NETWORK_V2 = 8;
pub type CLUSTER_CHANGE_NETWORK_V2 = i32;
pub const CLUSTER_CHANGE_NODE_ADDED: CLUSTER_CHANGE = 4;
pub const CLUSTER_CHANGE_NODE_ALL_V2: CLUSTER_CHANGE_NODE_V2 = 255;
pub const CLUSTER_CHANGE_NODE_COMMON_PROPERTY_V2: CLUSTER_CHANGE_NODE_V2 = 4;
pub const CLUSTER_CHANGE_NODE_DELETED: CLUSTER_CHANGE = 2;
pub const CLUSTER_CHANGE_NODE_DELETED_V2: CLUSTER_CHANGE_NODE_V2 = 2;
pub const CLUSTER_CHANGE_NODE_GROUP_GAINED_V2: CLUSTER_CHANGE_NODE_V2 = 32;
pub const CLUSTER_CHANGE_NODE_GROUP_LOST_V2: CLUSTER_CHANGE_NODE_V2 = 64;
pub const CLUSTER_CHANGE_NODE_HANDLE_CLOSE_V2: CLUSTER_CHANGE_NODE_V2 = 128;
pub const CLUSTER_CHANGE_NODE_NETINTERFACE_ADDED_V2: CLUSTER_CHANGE_NODE_V2 = 1;
pub const CLUSTER_CHANGE_NODE_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_NODE_V2 = 8;
pub const CLUSTER_CHANGE_NODE_PROPERTY: CLUSTER_CHANGE = 8;
pub const CLUSTER_CHANGE_NODE_STATE: CLUSTER_CHANGE = 1;
pub const CLUSTER_CHANGE_NODE_STATE_V2: CLUSTER_CHANGE_NODE_V2 = 16;
pub type CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 = i32;
pub type CLUSTER_CHANGE_NODE_V2 = i32;
pub const CLUSTER_CHANGE_QUORUM_ALL_V2: CLUSTER_CHANGE_QUORUM_V2 = 1;
pub const CLUSTER_CHANGE_QUORUM_STATE: CLUSTER_CHANGE = 268435456;
pub const CLUSTER_CHANGE_QUORUM_STATE_V2: CLUSTER_CHANGE_QUORUM_V2 = 1;
pub type CLUSTER_CHANGE_QUORUM_V2 = i32;
pub const CLUSTER_CHANGE_REGISTRY_ALL_V2: CLUSTER_CHANGE_REGISTRY_V2 = 31;
pub const CLUSTER_CHANGE_REGISTRY_ATTRIBUTES: CLUSTER_CHANGE = 32;
pub const CLUSTER_CHANGE_REGISTRY_ATTRIBUTES_V2: CLUSTER_CHANGE_REGISTRY_V2 = 1;
pub const CLUSTER_CHANGE_REGISTRY_HANDLE_CLOSE_V2: CLUSTER_CHANGE_REGISTRY_V2 = 16;
pub const CLUSTER_CHANGE_REGISTRY_NAME: CLUSTER_CHANGE = 16;
pub const CLUSTER_CHANGE_REGISTRY_NAME_V2: CLUSTER_CHANGE_REGISTRY_V2 = 2;
pub const CLUSTER_CHANGE_REGISTRY_SUBTREE: CLUSTER_CHANGE = 128;
pub const CLUSTER_CHANGE_REGISTRY_SUBTREE_V2: CLUSTER_CHANGE_REGISTRY_V2 = 4;
pub type CLUSTER_CHANGE_REGISTRY_V2 = i32;
pub const CLUSTER_CHANGE_REGISTRY_VALUE: CLUSTER_CHANGE = 64;
pub const CLUSTER_CHANGE_REGISTRY_VALUE_V2: CLUSTER_CHANGE_REGISTRY_V2 = 8;
pub const CLUSTER_CHANGE_RESOURCE_ADDED: CLUSTER_CHANGE = 1024;
pub const CLUSTER_CHANGE_RESOURCE_ALL_V2: CLUSTER_CHANGE_RESOURCE_V2 = 2047;
pub const CLUSTER_CHANGE_RESOURCE_COMMON_PROPERTY_V2: CLUSTER_CHANGE_RESOURCE_V2 = 1;
pub const CLUSTER_CHANGE_RESOURCE_DELETED: CLUSTER_CHANGE = 512;
pub const CLUSTER_CHANGE_RESOURCE_DELETED_V2: CLUSTER_CHANGE_RESOURCE_V2 = 128;
pub const CLUSTER_CHANGE_RESOURCE_DEPENDENCIES_V2: CLUSTER_CHANGE_RESOURCE_V2 = 16;
pub const CLUSTER_CHANGE_RESOURCE_DEPENDENTS_V2: CLUSTER_CHANGE_RESOURCE_V2 = 32;
pub const CLUSTER_CHANGE_RESOURCE_DLL_UPGRADED_V2: CLUSTER_CHANGE_RESOURCE_V2 = 256;
pub const CLUSTER_CHANGE_RESOURCE_HANDLE_CLOSE_V2: CLUSTER_CHANGE_RESOURCE_V2 = 512;
pub const CLUSTER_CHANGE_RESOURCE_OWNER_GROUP_V2: CLUSTER_CHANGE_RESOURCE_V2 = 8;
pub const CLUSTER_CHANGE_RESOURCE_POSSIBLE_OWNERS_V2: CLUSTER_CHANGE_RESOURCE_V2 = 64;
pub const CLUSTER_CHANGE_RESOURCE_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_RESOURCE_V2 = 2;
pub const CLUSTER_CHANGE_RESOURCE_PROPERTY: CLUSTER_CHANGE = 2048;
pub const CLUSTER_CHANGE_RESOURCE_STATE: CLUSTER_CHANGE = 256;
pub const CLUSTER_CHANGE_RESOURCE_STATE_V2: CLUSTER_CHANGE_RESOURCE_V2 = 4;
pub const CLUSTER_CHANGE_RESOURCE_TERMINAL_STATE_V2: CLUSTER_CHANGE_RESOURCE_V2 = 1024;
pub const CLUSTER_CHANGE_RESOURCE_TYPE_ADDED: CLUSTER_CHANGE = 131072;
pub const CLUSTER_CHANGE_RESOURCE_TYPE_ALL_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = 63;
pub const CLUSTER_CHANGE_RESOURCE_TYPE_COMMON_PROPERTY_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = 2;
pub const CLUSTER_CHANGE_RESOURCE_TYPE_DELETED: CLUSTER_CHANGE = 65536;
pub const CLUSTER_CHANGE_RESOURCE_TYPE_DELETED_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = 1;
pub const CLUSTER_CHANGE_RESOURCE_TYPE_DLL_UPGRADED_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = 16;
pub const CLUSTER_CHANGE_RESOURCE_TYPE_POSSIBLE_OWNERS_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = 8;
pub const CLUSTER_CHANGE_RESOURCE_TYPE_PRIVATE_PROPERTY_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = 4;
pub const CLUSTER_CHANGE_RESOURCE_TYPE_PROPERTY: CLUSTER_CHANGE = 262144;
pub type CLUSTER_CHANGE_RESOURCE_TYPE_V2 = i32;
pub type CLUSTER_CHANGE_RESOURCE_V2 = i32;
pub const CLUSTER_CHANGE_SHARED_VOLUME_ADDED_V2: CLUSTER_CHANGE_SHARED_VOLUME_V2 = 2;
pub const CLUSTER_CHANGE_SHARED_VOLUME_ALL_V2: CLUSTER_CHANGE_SHARED_VOLUME_V2 = 7;
pub const CLUSTER_CHANGE_SHARED_VOLUME_REMOVED_V2: CLUSTER_CHANGE_SHARED_VOLUME_V2 = 4;
pub const CLUSTER_CHANGE_SHARED_VOLUME_STATE_V2: CLUSTER_CHANGE_SHARED_VOLUME_V2 = 1;
pub type CLUSTER_CHANGE_SHARED_VOLUME_V2 = i32;
pub const CLUSTER_CHANGE_SPACEPORT_CUSTOM_PNP_V2: CLUSTER_CHANGE_SPACEPORT_V2 = 1;
pub type CLUSTER_CHANGE_SPACEPORT_V2 = i32;
pub const CLUSTER_CHANGE_UPGRADE_ALL: CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 = 7;
pub const CLUSTER_CHANGE_UPGRADE_NODE_COMMIT: CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 = 2;
pub const CLUSTER_CHANGE_UPGRADE_NODE_POSTCOMMIT: CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 = 4;
pub const CLUSTER_CHANGE_UPGRADE_NODE_PREPARE: CLUSTER_CHANGE_NODE_UPGRADE_PHASE_V2 = 1;
pub type CLUSTER_CLOUD_TYPE = i32;
pub const CLUSTER_CLOUD_TYPE_AZURE: CLUSTER_CLOUD_TYPE = 1;
pub const CLUSTER_CLOUD_TYPE_MIXED: CLUSTER_CLOUD_TYPE = 128;
pub const CLUSTER_CLOUD_TYPE_NONE: CLUSTER_CLOUD_TYPE = 0;
pub const CLUSTER_CLOUD_TYPE_UNKNOWN: CLUSTER_CLOUD_TYPE = -1;
pub const CLUSTER_CONFIGURED: u32 = 2;
pub type CLUSTER_CONTROL_OBJECT = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUSTER_CREATE_GROUP_INFO {
    pub dwVersion: u32,
    pub groupType: CLUSGROUP_TYPE,
}
pub const CLUSTER_CREATE_GROUP_INFO_VERSION: u32 = 1;
pub const CLUSTER_CREATE_GROUP_INFO_VERSION_1: u32 = 1;
pub const CLUSTER_CSA_VSS_STATE: windows_core::PCWSTR = windows_core::w!("BackupInProgress");
pub const CLUSTER_CSV_COMPATIBLE_FILTERS: windows_core::PCWSTR = windows_core::w!("SharedVolumeCompatibleFilters");
pub const CLUSTER_CSV_INCOMPATIBLE_FILTERS: windows_core::PCWSTR = windows_core::w!("SharedVolumeIncompatibleFilters");
pub type CLUSTER_CSV_VOLUME_FAULT_STATE = i32;
pub const CLUSTER_DELETE_ACCESS_CONTROL_ENTRY: u32 = 2;
pub const CLUSTER_ENFORCED_ANTIAFFINITY: windows_core::PCWSTR = windows_core::w!("ClusterEnforcedAntiaffinity");
pub type CLUSTER_ENUM = i32;
pub const CLUSTER_ENUM_ALL: CLUSTER_ENUM = 63;
pub const CLUSTER_ENUM_CAPACITY_NODE: CLUSTER_ENUM = 268435456;
pub const CLUSTER_ENUM_GROUP: CLUSTER_ENUM = 8;
pub const CLUSTER_ENUM_INTERNAL_NETWORK: CLUSTER_ENUM = -2147483648;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUSTER_ENUM_ITEM {
    pub dwVersion: u32,
    pub dwType: u32,
    pub cbId: u32,
    pub lpszId: windows_core::PWSTR,
    pub cbName: u32,
    pub lpszName: windows_core::PWSTR,
}
pub const CLUSTER_ENUM_ITEM_VERSION: u32 = 1;
pub const CLUSTER_ENUM_ITEM_VERSION_1: u32 = 1;
pub const CLUSTER_ENUM_NETINTERFACE: CLUSTER_ENUM = 32;
pub const CLUSTER_ENUM_NETWORK: CLUSTER_ENUM = 16;
pub const CLUSTER_ENUM_NODE: CLUSTER_ENUM = 1;
pub const CLUSTER_ENUM_RESOURCE: CLUSTER_ENUM = 4;
pub const CLUSTER_ENUM_RESTYPE: CLUSTER_ENUM = 2;
pub const CLUSTER_ENUM_SHARED_VOLUME_GROUP: CLUSTER_ENUM = 536870912;
pub const CLUSTER_ENUM_SHARED_VOLUME_RESOURCE: CLUSTER_ENUM = 1073741824;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CLUSTER_GROUP_AUTOFAILBACK_TYPE(pub CGAFT);
pub type CLUSTER_GROUP_ENUM = i32;
pub const CLUSTER_GROUP_ENUM_ALL: CLUSTER_GROUP_ENUM = 3;
pub const CLUSTER_GROUP_ENUM_CONTAINS: CLUSTER_GROUP_ENUM = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTER_GROUP_ENUM_ITEM {
    pub dwVersion: u32,
    pub cbId: u32,
    pub lpszId: windows_core::PWSTR,
    pub cbName: u32,
    pub lpszName: windows_core::PWSTR,
    pub state: CLUSTER_GROUP_STATE,
    pub cbOwnerNode: u32,
    pub lpszOwnerNode: windows_core::PWSTR,
    pub dwFlags: u32,
    pub cbProperties: u32,
    pub pProperties: *mut core::ffi::c_void,
    pub cbRoProperties: u32,
    pub pRoProperties: *mut core::ffi::c_void,
}
impl Default for CLUSTER_GROUP_ENUM_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLUSTER_GROUP_ENUM_ITEM_VERSION: u32 = 1;
pub const CLUSTER_GROUP_ENUM_ITEM_VERSION_1: u32 = 1;
pub const CLUSTER_GROUP_ENUM_NODES: CLUSTER_GROUP_ENUM = 2;
pub type CLUSTER_GROUP_PRIORITY = i32;
pub type CLUSTER_GROUP_STATE = i32;
pub const CLUSTER_GROUP_WAIT_DELAY: windows_core::PCWSTR = windows_core::w!("ClusterGroupWaitDelay");
pub const CLUSTER_HANG_RECOVERY_ACTION_KEYNAME: windows_core::PCWSTR = windows_core::w!("HangRecoveryAction");
pub const CLUSTER_HANG_TIMEOUT_KEYNAME: windows_core::PCWSTR = windows_core::w!("ClusSvcHangTimeout");
pub const CLUSTER_INSTALLED: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUSTER_IP_ENTRY {
    pub lpszIpAddress: windows_core::PCWSTR,
    pub dwPrefixLength: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTER_MEMBERSHIP_INFO {
    pub HasQuorum: windows_core::BOOL,
    pub UpnodesSize: u32,
    pub Upnodes: [u8; 1],
}
impl Default for CLUSTER_MEMBERSHIP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CLUSTER_MGMT_POINT_RESTYPE = i32;
pub const CLUSTER_MGMT_POINT_RESTYPE_AUTO: CLUSTER_MGMT_POINT_RESTYPE = 0;
pub const CLUSTER_MGMT_POINT_RESTYPE_DNN: CLUSTER_MGMT_POINT_RESTYPE = 2;
pub const CLUSTER_MGMT_POINT_RESTYPE_SNN: CLUSTER_MGMT_POINT_RESTYPE = 1;
pub type CLUSTER_MGMT_POINT_TYPE = i32;
pub const CLUSTER_MGMT_POINT_TYPE_CNO: CLUSTER_MGMT_POINT_TYPE = 1;
pub const CLUSTER_MGMT_POINT_TYPE_CNO_ONLY: CLUSTER_MGMT_POINT_TYPE = 3;
pub const CLUSTER_MGMT_POINT_TYPE_DNS_ONLY: CLUSTER_MGMT_POINT_TYPE = 2;
pub const CLUSTER_MGMT_POINT_TYPE_NONE: CLUSTER_MGMT_POINT_TYPE = 0;
pub const CLUSTER_NAME_AUTO_BALANCER_LEVEL: windows_core::PCWSTR = windows_core::w!("AutoBalancerLevel");
pub const CLUSTER_NAME_AUTO_BALANCER_MODE: windows_core::PCWSTR = windows_core::w!("AutoBalancerMode");
pub const CLUSTER_NAME_PREFERRED_SITE: windows_core::PCWSTR = windows_core::w!("PreferredSite");
pub type CLUSTER_NETINTERFACE_STATE = i32;
pub type CLUSTER_NETWORK_ENUM = i32;
pub const CLUSTER_NETWORK_ENUM_ALL: CLUSTER_NETWORK_ENUM = 1;
pub const CLUSTER_NETWORK_ENUM_NETINTERFACES: CLUSTER_NETWORK_ENUM = 1;
pub type CLUSTER_NETWORK_ROLE = i32;
pub type CLUSTER_NETWORK_STATE = i32;
pub type CLUSTER_NODE_DRAIN_STATUS = i32;
pub type CLUSTER_NODE_ENUM = i32;
pub const CLUSTER_NODE_ENUM_ALL: CLUSTER_NODE_ENUM = 3;
pub const CLUSTER_NODE_ENUM_GROUPS: CLUSTER_NODE_ENUM = 2;
pub const CLUSTER_NODE_ENUM_NETINTERFACES: CLUSTER_NODE_ENUM = 1;
pub const CLUSTER_NODE_ENUM_PREFERRED_GROUPS: CLUSTER_NODE_ENUM = 4;
pub type CLUSTER_NODE_FAILBACK_STATUS = i32;
pub type CLUSTER_NODE_RESUME_FAILBACK_TYPE = i32;
pub type CLUSTER_NODE_STATE = i32;
pub type CLUSTER_NODE_STATUS = i32;
pub const CLUSTER_NOTIFICATIONS_V1: CLUSTER_NOTIFICATIONS_VERSION = 1;
pub const CLUSTER_NOTIFICATIONS_V2: CLUSTER_NOTIFICATIONS_VERSION = 2;
pub type CLUSTER_NOTIFICATIONS_VERSION = i32;
pub type CLUSTER_OBJECT_TYPE = i32;
pub const CLUSTER_OBJECT_TYPE_AFFINITYRULE: CLUSTER_OBJECT_TYPE = 16;
pub const CLUSTER_OBJECT_TYPE_CLUSTER: CLUSTER_OBJECT_TYPE = 1;
pub const CLUSTER_OBJECT_TYPE_FAULTDOMAIN: CLUSTER_OBJECT_TYPE = 17;
pub const CLUSTER_OBJECT_TYPE_GROUP: CLUSTER_OBJECT_TYPE = 2;
pub const CLUSTER_OBJECT_TYPE_GROUPSET: CLUSTER_OBJECT_TYPE = 13;
pub const CLUSTER_OBJECT_TYPE_NETWORK: CLUSTER_OBJECT_TYPE = 6;
pub const CLUSTER_OBJECT_TYPE_NETWORK_INTERFACE: CLUSTER_OBJECT_TYPE = 5;
pub const CLUSTER_OBJECT_TYPE_NODE: CLUSTER_OBJECT_TYPE = 7;
pub const CLUSTER_OBJECT_TYPE_NONE: CLUSTER_OBJECT_TYPE = 0;
pub const CLUSTER_OBJECT_TYPE_QUORUM: CLUSTER_OBJECT_TYPE = 9;
pub const CLUSTER_OBJECT_TYPE_REGISTRY: CLUSTER_OBJECT_TYPE = 8;
pub const CLUSTER_OBJECT_TYPE_RESOURCE: CLUSTER_OBJECT_TYPE = 3;
pub const CLUSTER_OBJECT_TYPE_RESOURCE_TYPE: CLUSTER_OBJECT_TYPE = 4;
pub const CLUSTER_OBJECT_TYPE_SHARED_VOLUME: CLUSTER_OBJECT_TYPE = 10;
pub type CLUSTER_PROPERTY_FORMAT = i32;
pub type CLUSTER_PROPERTY_SYNTAX = i32;
pub type CLUSTER_PROPERTY_TYPE = i32;
pub const CLUSTER_QUORUM_LOST: CLUSTER_QUORUM_VALUE = 1;
pub const CLUSTER_QUORUM_MAINTAINED: CLUSTER_QUORUM_VALUE = 0;
pub type CLUSTER_QUORUM_TYPE = i32;
pub type CLUSTER_QUORUM_VALUE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTER_READ_BATCH_COMMAND {
    pub Command: CLUSTER_REG_COMMAND,
    pub dwOptions: u32,
    pub wzSubkeyName: windows_core::PCWSTR,
    pub wzValueName: windows_core::PCWSTR,
    pub lpData: *const u8,
    pub cbData: u32,
}
impl Default for CLUSTER_READ_BATCH_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CLUSTER_REG_COMMAND = i32;
pub const CLUSTER_REQUEST_REPLY_TIMEOUT: windows_core::PCWSTR = windows_core::w!("RequestReplyTimeout");
pub type CLUSTER_RESOURCE_CLASS = i32;
pub type CLUSTER_RESOURCE_CREATE_FLAGS = i32;
pub const CLUSTER_RESOURCE_DEFAULT_MONITOR: CLUSTER_RESOURCE_CREATE_FLAGS = 0;
pub type CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION = i32;
pub type CLUSTER_RESOURCE_ENUM = i32;
pub const CLUSTER_RESOURCE_ENUM_ALL: CLUSTER_RESOURCE_ENUM = 7;
pub const CLUSTER_RESOURCE_ENUM_DEPENDS: CLUSTER_RESOURCE_ENUM = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTER_RESOURCE_ENUM_ITEM {
    pub dwVersion: u32,
    pub cbId: u32,
    pub lpszId: windows_core::PWSTR,
    pub cbName: u32,
    pub lpszName: windows_core::PWSTR,
    pub cbOwnerGroupName: u32,
    pub lpszOwnerGroupName: windows_core::PWSTR,
    pub cbOwnerGroupId: u32,
    pub lpszOwnerGroupId: windows_core::PWSTR,
    pub cbProperties: u32,
    pub pProperties: *mut core::ffi::c_void,
    pub cbRoProperties: u32,
    pub pRoProperties: *mut core::ffi::c_void,
}
impl Default for CLUSTER_RESOURCE_ENUM_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLUSTER_RESOURCE_ENUM_ITEM_VERSION: u32 = 1;
pub const CLUSTER_RESOURCE_ENUM_ITEM_VERSION_1: u32 = 1;
pub const CLUSTER_RESOURCE_ENUM_NODES: CLUSTER_RESOURCE_ENUM = 4;
pub const CLUSTER_RESOURCE_ENUM_PROVIDES: CLUSTER_RESOURCE_ENUM = 2;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CLUSTER_RESOURCE_RESTART_ACTION(pub CRRA);
pub const CLUSTER_RESOURCE_SEPARATE_MONITOR: CLUSTER_RESOURCE_CREATE_FLAGS = 1;
pub type CLUSTER_RESOURCE_STATE = i32;
pub type CLUSTER_RESOURCE_STATE_CHANGE_REASON = i32;
pub type CLUSTER_RESOURCE_TYPE_ENUM = i32;
pub const CLUSTER_RESOURCE_TYPE_ENUM_ALL: CLUSTER_RESOURCE_TYPE_ENUM = 3;
pub const CLUSTER_RESOURCE_TYPE_ENUM_NODES: CLUSTER_RESOURCE_TYPE_ENUM = 1;
pub const CLUSTER_RESOURCE_TYPE_ENUM_RESOURCES: CLUSTER_RESOURCE_TYPE_ENUM = 2;
pub const CLUSTER_RESOURCE_TYPE_SPECIFIC_V2: CLUSTER_CHANGE_RESOURCE_TYPE_V2 = 32;
pub const CLUSTER_RESOURCE_VALID_FLAGS: CLUSTER_RESOURCE_CREATE_FLAGS = 1;
pub const CLUSTER_RUNNING: u32 = 16;
pub const CLUSTER_S2D_BUS_TYPES: windows_core::PCWSTR = windows_core::w!("S2DBusTypes");
pub const CLUSTER_S2D_CACHE_BEHAVIOR_FLAGS: windows_core::PCWSTR = windows_core::w!("S2DCacheBehavior");
pub const CLUSTER_S2D_CACHE_DESIRED_STATE: windows_core::PCWSTR = windows_core::w!("S2DCacheDesiredState");
pub const CLUSTER_S2D_CACHE_FLASH_RESERVE_PERCENT: windows_core::PCWSTR = windows_core::w!("S2DCacheFlashReservePercent");
pub const CLUSTER_S2D_CACHE_METADATA_RESERVE: windows_core::PCWSTR = windows_core::w!("S2DCacheMetadataReserveBytes");
pub const CLUSTER_S2D_CACHE_PAGE_SIZE_KBYTES: windows_core::PCWSTR = windows_core::w!("S2DCachePageSizeKBytes");
pub const CLUSTER_S2D_ENABLED: windows_core::PCWSTR = windows_core::w!("S2DEnabled");
pub const CLUSTER_S2D_IO_LATENCY_THRESHOLD: windows_core::PCWSTR = windows_core::w!("S2DIOLatencyThreshold");
pub const CLUSTER_S2D_OPTIMIZATIONS: windows_core::PCWSTR = windows_core::w!("S2DOptimizations");
pub type CLUSTER_SETUP_PHASE = i32;
pub type CLUSTER_SETUP_PHASE_SEVERITY = i32;
pub type CLUSTER_SETUP_PHASE_TYPE = i32;
pub const CLUSTER_SET_ACCESS_TYPE_ALLOWED: u32 = 0;
pub const CLUSTER_SET_ACCESS_TYPE_DENIED: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUSTER_SET_PASSWORD_STATUS {
    pub NodeId: u32,
    pub SetAttempted: bool,
    pub ReturnStatus: u32,
}
pub const CLUSTER_SHARED_VOLUMES_ROOT: windows_core::PCWSTR = windows_core::w!("SharedVolumesRoot");
pub type CLUSTER_SHARED_VOLUME_BACKUP_STATE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSTER_SHARED_VOLUME_RENAME_GUID_INPUT {
    pub Base: CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME,
    pub Base2: CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME,
}
impl Default for CLUSTER_SHARED_VOLUME_RENAME_GUID_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT {
    pub Base: CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME,
    pub Base2: CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME,
}
impl Default for CLUSTER_SHARED_VOLUME_RENAME_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME {
    pub NewVolumeName: [u16; 260],
    pub NewVolumeGuid: [u16; 50],
}
impl Default for CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME {
    pub NewVolumeName: [u16; 260],
}
impl Default for CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME {
    pub InputType: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE,
    pub Anonymous: CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME_0,
}
impl Default for CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME_0 {
    pub VolumeOffset: u64,
    pub VolumeId: [u16; 260],
    pub VolumeName: [u16; 260],
    pub VolumeGuid: [u16; 50],
}
impl Default for CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE = i32;
pub type CLUSTER_SHARED_VOLUME_STATE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTER_SHARED_VOLUME_STATE_INFO {
    pub szVolumeName: [u16; 260],
    pub szNodeName: [u16; 260],
    pub VolumeState: CLUSTER_SHARED_VOLUME_STATE,
}
impl Default for CLUSTER_SHARED_VOLUME_STATE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTER_SHARED_VOLUME_STATE_INFO_EX {
    pub szVolumeName: [u16; 260],
    pub szNodeName: [u16; 260],
    pub VolumeState: CLUSTER_SHARED_VOLUME_STATE,
    pub szVolumeFriendlyName: [u16; 260],
    pub RedirectedIOReason: u64,
    pub VolumeRedirectedIOReason: u64,
}
impl Default for CLUSTER_SHARED_VOLUME_STATE_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLUSTER_SHARED_VOLUME_VSS_WRITER_OPERATION_TIMEOUT: windows_core::PCWSTR = windows_core::w!("SharedVolumeVssWriterOperationTimeout");
pub type CLUSTER_STORAGENODE_STATE = i32;
pub type CLUSTER_UPGRADE_PHASE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTER_VALIDATE_CSV_FILENAME {
    pub szFileName: [u16; 0],
}
impl Default for CLUSTER_VALIDATE_CSV_FILENAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTER_VALIDATE_DIRECTORY {
    pub szPath: [u16; 0],
}
impl Default for CLUSTER_VALIDATE_DIRECTORY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTER_VALIDATE_NETNAME {
    pub szNetworkName: [u16; 0],
}
impl Default for CLUSTER_VALIDATE_NETNAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUSTER_VALIDATE_PATH {
    pub szPath: [u16; 0],
}
impl Default for CLUSTER_VALIDATE_PATH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLUSTER_VERSION_FLAG_MIXED_MODE: u32 = 1;
pub const CLUSTER_VERSION_UNKNOWN: u32 = 4294967295;
pub const CLUSTER_WITNESS_DATABASE_WRITE_TIMEOUT: windows_core::PCWSTR = windows_core::w!("WitnessDatabaseWriteTimeout");
pub const CLUSTER_WITNESS_FAILED_RESTART_INTERVAL: windows_core::PCWSTR = windows_core::w!("WitnessRestartInterval");
pub const CLUS_ACCESS_ANY: u32 = 0;
pub const CLUS_ACCESS_READ: u32 = 1;
pub const CLUS_ACCESS_WRITE: u32 = 2;
pub type CLUS_ADAPTER_EXCLUSION_TYPE = i32;
pub const CLUS_ADAPTER_EXCLUSION_TYPE_DESCRIPTION: CLUS_ADAPTER_EXCLUSION_TYPE = 1;
pub const CLUS_ADAPTER_EXCLUSION_TYPE_FRIENDLYNAME: CLUS_ADAPTER_EXCLUSION_TYPE = 2;
pub const CLUS_ADAPTER_EXCLUSION_TYPE_IPPREFIX: CLUS_ADAPTER_EXCLUSION_TYPE = 0;
pub const CLUS_AFFINITY_RULE_DIFFERENT_FAULT_DOMAIN: CLUS_AFFINITY_RULE_TYPE = 3;
pub const CLUS_AFFINITY_RULE_DIFFERENT_NODE: CLUS_AFFINITY_RULE_TYPE = 4;
pub const CLUS_AFFINITY_RULE_MAX: CLUS_AFFINITY_RULE_TYPE = 4;
pub const CLUS_AFFINITY_RULE_MIN: CLUS_AFFINITY_RULE_TYPE = 0;
pub const CLUS_AFFINITY_RULE_NONE: CLUS_AFFINITY_RULE_TYPE = 0;
pub const CLUS_AFFINITY_RULE_SAME_FAULT_DOMAIN: CLUS_AFFINITY_RULE_TYPE = 1;
pub const CLUS_AFFINITY_RULE_SAME_NODE: CLUS_AFFINITY_RULE_TYPE = 2;
pub type CLUS_AFFINITY_RULE_TYPE = i32;
pub type CLUS_CHARACTERISTICS = i32;
pub const CLUS_CHAR_BROADCAST_DELETE: CLUS_CHARACTERISTICS = 32;
pub const CLUS_CHAR_CLONES: CLUS_CHARACTERISTICS = 8192;
pub const CLUS_CHAR_COEXIST_IN_SHARED_VOLUME_GROUP: CLUS_CHARACTERISTICS = 256;
pub const CLUS_CHAR_DELETE_REQUIRES_ALL_NODES: CLUS_CHARACTERISTICS = 2;
pub const CLUS_CHAR_DRAIN_LOCAL_OFFLINE: CLUS_CHARACTERISTICS = 524288;
pub const CLUS_CHAR_INFRASTRUCTURE: CLUS_CHARACTERISTICS = 131072;
pub const CLUS_CHAR_LOCAL_QUORUM: CLUS_CHARACTERISTICS = 4;
pub const CLUS_CHAR_LOCAL_QUORUM_DEBUG: CLUS_CHARACTERISTICS = 8;
pub const CLUS_CHAR_MONITOR_DETACH: CLUS_CHARACTERISTICS = 1024;
pub const CLUS_CHAR_MONITOR_REATTACH: CLUS_CHARACTERISTICS = 2048;
pub const CLUS_CHAR_NOTIFY_NEW_OWNER: CLUS_CHARACTERISTICS = 32768;
pub const CLUS_CHAR_NOT_PREEMPTABLE: CLUS_CHARACTERISTICS = 16384;
pub const CLUS_CHAR_OPERATION_CONTEXT: CLUS_CHARACTERISTICS = 4096;
pub const CLUS_CHAR_PLACEMENT_DATA: CLUS_CHARACTERISTICS = 512;
pub const CLUS_CHAR_QUORUM: CLUS_CHARACTERISTICS = 1;
pub const CLUS_CHAR_REQUIRES_STATE_CHANGE_REASON: CLUS_CHARACTERISTICS = 16;
pub const CLUS_CHAR_SINGLE_CLUSTER_INSTANCE: CLUS_CHARACTERISTICS = 64;
pub const CLUS_CHAR_SINGLE_GROUP_INSTANCE: CLUS_CHARACTERISTICS = 128;
pub const CLUS_CHAR_SUPPORTS_UNMONITORED_STATE: CLUS_CHARACTERISTICS = 65536;
pub const CLUS_CHAR_UNKNOWN: CLUS_CHARACTERISTICS = 0;
pub const CLUS_CHAR_VETO_DRAIN: CLUS_CHARACTERISTICS = 262144;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_CHKDSK_INFO {
    pub PartitionNumber: u32,
    pub ChkdskState: u32,
    pub FileIdCount: u32,
    pub FileIdList: [u64; 1],
}
impl Default for CLUS_CHKDSK_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT {
    pub FileServerName: [u16; 16],
}
impl Default for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT {
    pub FileServerName: [u16; 260],
}
impl Default for CLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_CSV_MAINTENANCE_MODE_INFO {
    pub InMaintenance: windows_core::BOOL,
    pub VolumeName: [u16; 260],
}
impl Default for CLUS_CSV_MAINTENANCE_MODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_CSV_VOLUME_INFO {
    pub VolumeOffset: u64,
    pub PartitionNumber: u32,
    pub FaultState: CLUSTER_CSV_VOLUME_FAULT_STATE,
    pub BackupState: CLUSTER_SHARED_VOLUME_BACKUP_STATE,
    pub szVolumeFriendlyName: [u16; 260],
    pub szVolumeName: [u16; 50],
}
impl Default for CLUS_CSV_VOLUME_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_CSV_VOLUME_NAME {
    pub VolumeOffset: i64,
    pub szVolumeName: [u16; 260],
    pub szRootPath: [u16; 263],
}
impl Default for CLUS_CSV_VOLUME_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUS_DISK_NUMBER_INFO {
    pub DiskNumber: u32,
    pub BytesPerSector: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUS_DNN_LEADER_STATUS {
    pub IsOnline: windows_core::BOOL,
    pub IsFileServerPresent: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUS_DNN_SODAFS_CLONE_STATUS {
    pub NodeId: u32,
    pub Status: CLUSTER_RESOURCE_STATE,
}
pub type CLUS_FLAGS = i32;
pub const CLUS_FLAG_CORE: CLUS_FLAGS = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_FORCE_QUORUM_INFO {
    pub dwSize: u32,
    pub dwNodeBitMask: u32,
    pub dwMaxNumberofNodes: u32,
    pub multiszNodeList: [u16; 1],
}
impl Default for CLUS_FORCE_QUORUM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUS_FTSET_INFO {
    pub dwRootSignature: u32,
    pub dwFtType: u32,
}
pub const CLUS_GLOBAL: u32 = 1;
pub const CLUS_GROUP_DO_NOT_START: CLUS_GROUP_START_SETTING = 1;
pub const CLUS_GROUP_START_ALLOWED: CLUS_GROUP_START_SETTING = 2;
pub const CLUS_GROUP_START_ALWAYS: CLUS_GROUP_START_SETTING = 0;
pub type CLUS_GROUP_START_SETTING = i32;
pub const CLUS_GRP_MOVE_ALLOWED: u32 = 0;
pub const CLUS_GRP_MOVE_LOCKED: u32 = 1;
pub const CLUS_HYBRID_QUORUM: u32 = 1024;
pub const CLUS_LEGACY_QUORUM: u32 = 4194304;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUS_MAINTENANCE_MODE_INFO {
    pub InMaintenance: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUS_MAINTENANCE_MODE_INFOEX {
    pub InMaintenance: windows_core::BOOL,
    pub MaintainenceModeType: MAINTENANCE_MODE_TYPE_ENUM,
    pub InternalState: CLUSTER_RESOURCE_STATE,
    pub Signature: u32,
}
pub const CLUS_MODIFY: u32 = 1;
pub const CLUS_NAME_RES_TYPE_CLUSTER_GROUPID: windows_core::PCWSTR = windows_core::w!("ClusterGroupId");
pub const CLUS_NAME_RES_TYPE_DATA_RESID: windows_core::PCWSTR = windows_core::w!("DataResourceId");
pub const CLUS_NAME_RES_TYPE_LOG_MULTIPLE: windows_core::PCWSTR = windows_core::w!("LogSizeMultiple");
pub const CLUS_NAME_RES_TYPE_LOG_RESID: windows_core::PCWSTR = windows_core::w!("LogResourceId");
pub const CLUS_NAME_RES_TYPE_LOG_VOLUME: windows_core::PCWSTR = windows_core::w!("LogVolume");
pub const CLUS_NAME_RES_TYPE_MINIMUM_LOG_SIZE: windows_core::PCWSTR = windows_core::w!("MinimumLogSizeInBytes");
pub const CLUS_NAME_RES_TYPE_REPLICATION_GROUPID: windows_core::PCWSTR = windows_core::w!("ReplicationGroupId");
pub const CLUS_NAME_RES_TYPE_REPLICATION_GROUP_TYPE: windows_core::PCWSTR = windows_core::w!("ReplicationClusterGroupType");
pub const CLUS_NAME_RES_TYPE_SOURCE_RESID: windows_core::PCWSTR = windows_core::w!("SourceResourceId");
pub const CLUS_NAME_RES_TYPE_SOURCE_VOLUMES: windows_core::PCWSTR = windows_core::w!("SourceVolumes");
pub const CLUS_NAME_RES_TYPE_TARGET_RESID: windows_core::PCWSTR = windows_core::w!("TargetResourceId");
pub const CLUS_NAME_RES_TYPE_TARGET_VOLUMES: windows_core::PCWSTR = windows_core::w!("TargetVolumes");
pub const CLUS_NAME_RES_TYPE_UNIT_LOG_SIZE_CHANGE: windows_core::PCWSTR = windows_core::w!("UnitOfLogSizeChangeInBytes");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_NETNAME_IP_INFO_ENTRY {
    pub NodeId: u32,
    pub AddressSize: u32,
    pub Address: [u8; 1],
}
impl Default for CLUS_NETNAME_IP_INFO_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL {
    pub szName: [u16; 64],
    pub NumEntries: u32,
    pub IpInfo: [CLUS_NETNAME_IP_INFO_ENTRY; 1],
}
impl Default for CLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CLUS_NETNAME_PWD_INFO = CLUS_RLUA_PWD_INFO;
pub type CLUS_NETNAME_PWD_INFOEX = CLUS_RLUA_PWD_INFOEX;
pub type CLUS_NETNAME_VS_TOKEN_INFO = CLUS_VS_TOKEN_INFO;
pub const CLUS_NODE_MAJORITY_QUORUM: u32 = 0;
pub const CLUS_NOT_GLOBAL: u32 = 0;
pub const CLUS_NO_MODIFY: u32 = 0;
pub const CLUS_OBJECT_AFFINITYRULE: CLUSTER_CONTROL_OBJECT = 9;
pub const CLUS_OBJECT_CLUSTER: CLUSTER_CONTROL_OBJECT = 7;
pub const CLUS_OBJECT_GROUP: CLUSTER_CONTROL_OBJECT = 3;
pub const CLUS_OBJECT_GROUPSET: CLUSTER_CONTROL_OBJECT = 8;
pub const CLUS_OBJECT_INVALID: CLUSTER_CONTROL_OBJECT = 0;
pub const CLUS_OBJECT_NETINTERFACE: CLUSTER_CONTROL_OBJECT = 6;
pub const CLUS_OBJECT_NETWORK: CLUSTER_CONTROL_OBJECT = 5;
pub const CLUS_OBJECT_NODE: CLUSTER_CONTROL_OBJECT = 4;
pub const CLUS_OBJECT_RESOURCE: CLUSTER_CONTROL_OBJECT = 1;
pub const CLUS_OBJECT_RESOURCE_TYPE: CLUSTER_CONTROL_OBJECT = 2;
pub const CLUS_OBJECT_USER: CLUSTER_CONTROL_OBJECT = 128;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_PARTITION_INFO {
    pub dwFlags: u32,
    pub szDeviceName: [u16; 260],
    pub szVolumeLabel: [u16; 260],
    pub dwSerialNumber: u32,
    pub rgdwMaximumComponentLength: u32,
    pub dwFileSystemFlags: u32,
    pub szFileSystem: [u16; 32],
}
impl Default for CLUS_PARTITION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub VolumeGuid: windows_core::GUID,
}
impl Default for CLUS_PARTITION_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_PARTITION_INFO_EX2 {
    pub GptPartitionId: windows_core::GUID,
    pub szPartitionName: [u16; 260],
    pub EncryptionFlags: u32,
}
impl Default for CLUS_PARTITION_INFO_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_PROVIDER_STATE_CHANGE_INFO {
    pub dwSize: u32,
    pub resourceState: CLUSTER_RESOURCE_STATE,
    pub szProviderId: [u16; 1],
}
impl Default for CLUS_PROVIDER_STATE_CHANGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLUS_RESCLASS_NETWORK: CLUSTER_RESOURCE_CLASS = 2;
pub const CLUS_RESCLASS_STORAGE: CLUSTER_RESOURCE_CLASS = 1;
pub const CLUS_RESCLASS_UNKNOWN: CLUSTER_RESOURCE_CLASS = 0;
pub const CLUS_RESCLASS_USER: CLUSTER_RESOURCE_CLASS = 32768;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUS_RESOURCE_CLASS_INFO {
    pub Anonymous: CLUS_RESOURCE_CLASS_INFO_0,
}
impl Default for CLUS_RESOURCE_CLASS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CLUS_RESOURCE_CLASS_INFO_0 {
    pub Anonymous: CLUS_RESOURCE_CLASS_INFO_0_0,
    pub li: u64,
}
impl Default for CLUS_RESOURCE_CLASS_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUS_RESOURCE_CLASS_INFO_0_0 {
    pub Anonymous: CLUS_RESOURCE_CLASS_INFO_0_0_0,
    pub SubClass: u32,
}
impl Default for CLUS_RESOURCE_CLASS_INFO_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CLUS_RESOURCE_CLASS_INFO_0_0_0 {
    pub dw: u32,
    pub rc: CLUSTER_RESOURCE_CLASS,
}
impl Default for CLUS_RESOURCE_CLASS_INFO_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CLUS_RESSUBCLASS = i32;
pub type CLUS_RESSUBCLASS_NETWORK = i32;
pub const CLUS_RESSUBCLASS_NETWORK_INTERNET_PROTOCOL: CLUS_RESSUBCLASS_NETWORK = -2147483648;
pub const CLUS_RESSUBCLASS_SHARED: CLUS_RESSUBCLASS = -2147483648;
pub type CLUS_RESSUBCLASS_STORAGE = i32;
pub const CLUS_RESSUBCLASS_STORAGE_DISK: CLUS_RESSUBCLASS_STORAGE = 1073741824;
pub const CLUS_RESSUBCLASS_STORAGE_REPLICATION: CLUS_RESSUBCLASS_STORAGE = 268435456;
pub const CLUS_RESSUBCLASS_STORAGE_SHARED_BUS: CLUS_RESSUBCLASS_STORAGE = -2147483648;
pub const CLUS_RESTYPE_NAME_CAU: windows_core::PCWSTR = windows_core::w!("ClusterAwareUpdatingResource");
pub const CLUS_RESTYPE_NAME_CLOUD_WITNESS: windows_core::PCWSTR = windows_core::w!("Cloud Witness");
pub const CLUS_RESTYPE_NAME_CONTAINER: windows_core::PCWSTR = windows_core::w!("Container");
pub const CLUS_RESTYPE_NAME_CROSS_CLUSTER: windows_core::PCWSTR = windows_core::w!("Cross Cluster Dependency Orchestrator");
pub const CLUS_RESTYPE_NAME_DFS: windows_core::PCWSTR = windows_core::w!("Distributed File System");
pub const CLUS_RESTYPE_NAME_DFSR: windows_core::PCWSTR = windows_core::w!("DFS Replicated Folder");
pub const CLUS_RESTYPE_NAME_DHCP: windows_core::PCWSTR = windows_core::w!("DHCP Service");
pub const CLUS_RESTYPE_NAME_DNN: windows_core::PCWSTR = windows_core::w!("Distributed Network Name");
pub const CLUS_RESTYPE_NAME_FILESERVER: windows_core::PCWSTR = windows_core::w!("File Server");
pub const CLUS_RESTYPE_NAME_FILESHR: windows_core::PCWSTR = windows_core::w!("File Share");
pub const CLUS_RESTYPE_NAME_FSWITNESS: windows_core::PCWSTR = windows_core::w!("File Share Witness");
pub const CLUS_RESTYPE_NAME_GENAPP: windows_core::PCWSTR = windows_core::w!("Generic Application");
pub const CLUS_RESTYPE_NAME_GENSCRIPT: windows_core::PCWSTR = windows_core::w!("Generic Script");
pub const CLUS_RESTYPE_NAME_GENSVC: windows_core::PCWSTR = windows_core::w!("Generic Service");
pub const CLUS_RESTYPE_NAME_HARDDISK: windows_core::PCWSTR = windows_core::w!("Physical Disk");
pub const CLUS_RESTYPE_NAME_HCSVM: windows_core::PCWSTR = windows_core::w!("HCS Virtual Machine");
pub const CLUS_RESTYPE_NAME_HEALTH_SERVICE: windows_core::PCWSTR = windows_core::w!("Health Service");
pub const CLUS_RESTYPE_NAME_IPADDR: windows_core::PCWSTR = windows_core::w!("IP Address");
pub const CLUS_RESTYPE_NAME_IPV6_NATIVE: windows_core::PCWSTR = windows_core::w!("IPv6 Address");
pub const CLUS_RESTYPE_NAME_IPV6_TUNNEL: windows_core::PCWSTR = windows_core::w!("IPv6 Tunnel Address");
pub const CLUS_RESTYPE_NAME_ISCSITARGET: windows_core::PCWSTR = windows_core::w!("iSCSI Target Server");
pub const CLUS_RESTYPE_NAME_ISNS: windows_core::PCWSTR = windows_core::w!("Microsoft iSNS");
pub const CLUS_RESTYPE_NAME_KEY_VALUE_STORE: windows_core::PCWSTR = windows_core::w!("Key Value Store");
pub const CLUS_RESTYPE_NAME_MSDTC: windows_core::PCWSTR = windows_core::w!("Distributed Transaction Coordinator");
pub const CLUS_RESTYPE_NAME_MSMQ: windows_core::PCWSTR = windows_core::w!("Microsoft Message Queue Server");
pub const CLUS_RESTYPE_NAME_MSMQ_TRIGGER: windows_core::PCWSTR = windows_core::w!("MSMQTriggers");
pub const CLUS_RESTYPE_NAME_NAT: windows_core::PCWSTR = windows_core::w!("Nat");
pub const CLUS_RESTYPE_NAME_NETNAME: windows_core::PCWSTR = windows_core::w!("Network Name");
pub const CLUS_RESTYPE_NAME_NETWORK_FILE_SYSTEM: windows_core::PCWSTR = windows_core::w!("Network File System");
pub const CLUS_RESTYPE_NAME_NEW_MSMQ: windows_core::PCWSTR = windows_core::w!("MSMQ");
pub const CLUS_RESTYPE_NAME_NFS: windows_core::PCWSTR = windows_core::w!("NFS Share");
pub const CLUS_RESTYPE_NAME_NFS_MSNS: windows_core::PCWSTR = windows_core::w!("NFS Multi Server Namespace");
pub const CLUS_RESTYPE_NAME_NFS_V2: windows_core::PCWSTR = windows_core::w!("Network File System");
pub const CLUS_RESTYPE_NAME_NV_PROVIDER_ADDRESS: windows_core::PCWSTR = windows_core::w!("Provider Address");
pub const CLUS_RESTYPE_NAME_PRTSPLR: windows_core::PCWSTR = windows_core::w!("Print Spooler");
pub const CLUS_RESTYPE_NAME_SCALEOUT_MASTER: windows_core::PCWSTR = windows_core::w!("Scaleout Master");
pub const CLUS_RESTYPE_NAME_SCALEOUT_WORKER: windows_core::PCWSTR = windows_core::w!("Scaleout Worker");
pub const CLUS_RESTYPE_NAME_SDDC_MANAGEMENT: windows_core::PCWSTR = windows_core::w!("SDDC Management");
pub const CLUS_RESTYPE_NAME_SODAFILESERVER: windows_core::PCWSTR = windows_core::w!("Scale Out File Server");
pub const CLUS_RESTYPE_NAME_STORAGE_POLICIES: windows_core::PCWSTR = windows_core::w!("Storage Policies");
pub const CLUS_RESTYPE_NAME_STORAGE_POOL: windows_core::PCWSTR = windows_core::w!("Storage Pool");
pub const CLUS_RESTYPE_NAME_STORAGE_REPLICA: windows_core::PCWSTR = windows_core::w!("Storage Replica");
pub const CLUS_RESTYPE_NAME_STORQOS: windows_core::PCWSTR = windows_core::w!("Storage QoS Policy Manager");
pub const CLUS_RESTYPE_NAME_TASKSCHEDULER: windows_core::PCWSTR = windows_core::w!("Task Scheduler");
pub const CLUS_RESTYPE_NAME_VIRTUAL_IPV4: windows_core::PCWSTR = windows_core::w!("Disjoint IPv4 Address");
pub const CLUS_RESTYPE_NAME_VIRTUAL_IPV6: windows_core::PCWSTR = windows_core::w!("Disjoint IPv6 Address");
pub const CLUS_RESTYPE_NAME_VM: windows_core::PCWSTR = windows_core::w!("Virtual Machine");
pub const CLUS_RESTYPE_NAME_VMREPLICA_BROKER: windows_core::PCWSTR = windows_core::w!("Virtual Machine Replication Broker");
pub const CLUS_RESTYPE_NAME_VMREPLICA_COORDINATOR: windows_core::PCWSTR = windows_core::w!("Virtual Machine Replication Coordinator");
pub const CLUS_RESTYPE_NAME_VM_CONFIG: windows_core::PCWSTR = windows_core::w!("Virtual Machine Configuration");
pub const CLUS_RESTYPE_NAME_VM_WMI: windows_core::PCWSTR = windows_core::w!("Virtual Machine Cluster WMI");
pub const CLUS_RESTYPE_NAME_VSSTASK: windows_core::PCWSTR = windows_core::w!("Volume Shadow Copy Service Task");
pub const CLUS_RESTYPE_NAME_WINS: windows_core::PCWSTR = windows_core::w!("WINS Service");
pub const CLUS_RES_NAME_SCALEOUT_MASTER: windows_core::PCWSTR = windows_core::w!("Scaleout Master");
pub const CLUS_RES_NAME_SCALEOUT_WORKER: windows_core::PCWSTR = windows_core::w!("Scaleout Worker");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_RLUA_PWD_INFO {
    pub Flags: u32,
    pub Password: [u16; 16],
    pub CreatingDC: [u16; 258],
    pub ObjectGuid: [u16; 64],
}
impl Default for CLUS_RLUA_PWD_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_RLUA_PWD_INFOEX {
    pub Flags: u32,
    pub Password: [u16; 128],
    pub CreatingDC: [u16; 258],
    pub ObjectGuid: [u16; 64],
}
impl Default for CLUS_RLUA_PWD_INFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLUS_SCSI_ADDRESS {
    pub Anonymous: CLUS_SCSI_ADDRESS_0,
}
impl Default for CLUS_SCSI_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CLUS_SCSI_ADDRESS_0 {
    pub Anonymous: CLUS_SCSI_ADDRESS_0_0,
    pub dw: u32,
}
impl Default for CLUS_SCSI_ADDRESS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUS_SCSI_ADDRESS_0_0 {
    pub PortNumber: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_SET_MAINTENANCE_MODE_INPUT {
    pub InMaintenance: windows_core::BOOL,
    pub ExtraParameterSize: u32,
    pub ExtraParameter: [u8; 1],
}
impl Default for CLUS_SET_MAINTENANCE_MODE_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLUS_SHARED_VOLUME_BACKUP_MODE {
    pub BackupState: CLUSTER_SHARED_VOLUME_BACKUP_STATE,
    pub DelayTimerInSecs: u32,
    pub VolumeName: [u16; 260],
}
impl Default for CLUS_SHARED_VOLUME_BACKUP_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUS_STARTING_PARAMS {
    pub dwSize: u32,
    pub bForm: windows_core::BOOL,
    pub bFirst: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS {
    pub AvailDrivelettersMask: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUS_STORAGE_REMAP_DRIVELETTER {
    pub CurrentDriveLetterMask: u32,
    pub TargetDriveLetterMask: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUS_STORAGE_SET_DRIVELETTER {
    pub PartitionNumber: u32,
    pub DriveLetterMask: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLUS_VS_TOKEN_INFO {
    pub ProcessID: u32,
    pub DesiredAccess: u32,
    pub InheritHandle: windows_core::BOOL,
}
pub const CREATEDC_PRESENT: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CREATE_CLUSTER_CONFIG {
    pub dwVersion: u32,
    pub lpszClusterName: windows_core::PCWSTR,
    pub cNodes: u32,
    pub ppszNodeNames: *mut windows_core::PCWSTR,
    pub cIpEntries: u32,
    pub pIpEntries: PCLUSTER_IP_ENTRY,
    pub fEmptyCluster: bool,
    pub managementPointType: CLUSTER_MGMT_POINT_TYPE,
    pub managementPointResType: CLUSTER_MGMT_POINT_RESTYPE,
    pub pszUserName: windows_core::PCWSTR,
    pub pszPassword: windows_core::PCWSTR,
    pub pszDomain: windows_core::PCWSTR,
}
impl Default for CREATE_CLUSTER_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CREATE_CLUSTER_MAJOR_VERSION_MASK: u32 = 4294967040;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CREATE_CLUSTER_NAME_ACCOUNT {
    pub dwVersion: u32,
    pub lpszClusterName: windows_core::PCWSTR,
    pub dwFlags: u32,
    pub pszUserName: windows_core::PCWSTR,
    pub pszPassword: windows_core::PCWSTR,
    pub pszDomain: windows_core::PCWSTR,
    pub managementPointType: CLUSTER_MGMT_POINT_TYPE,
    pub managementPointResType: CLUSTER_MGMT_POINT_RESTYPE,
    pub bUpgradeVCOs: bool,
}
pub const CREATE_CLUSTER_VERSION: u32 = 1536;
pub type CRRA = i32;
pub const CTCTL_GET_FAULT_DOMAIN_STATE: CLCTL_CODES = 789;
pub const CTCTL_GET_ROUTESTATUS_BASIC: CLCTL_CODES = 781;
pub const CTCTL_GET_ROUTESTATUS_EXTENDED: CLCTL_CODES = 785;
pub const CU_UPGRADE_VERSION: u32 = 3;
pub const ClusGroupTypeAvailableStorage: CLUSGROUP_TYPE = 2;
pub const ClusGroupTypeClusterUpdateAgent: CLUSGROUP_TYPE = 117;
pub const ClusGroupTypeCoreCluster: CLUSGROUP_TYPE = 1;
pub const ClusGroupTypeCoreSddc: CLUSGROUP_TYPE = 123;
pub const ClusGroupTypeCrossClusterOrchestrator: CLUSGROUP_TYPE = 121;
pub const ClusGroupTypeDhcpServer: CLUSGROUP_TYPE = 102;
pub const ClusGroupTypeDtc: CLUSGROUP_TYPE = 103;
pub const ClusGroupTypeFileServer: CLUSGROUP_TYPE = 100;
pub const ClusGroupTypeGenericApplication: CLUSGROUP_TYPE = 107;
pub const ClusGroupTypeGenericScript: CLUSGROUP_TYPE = 109;
pub const ClusGroupTypeGenericService: CLUSGROUP_TYPE = 108;
pub const ClusGroupTypeHcsVirtualMachine: CLUSGROUP_TYPE = 126;
pub const ClusGroupTypeIScsiNameService: CLUSGROUP_TYPE = 110;
pub const ClusGroupTypeIScsiTarget: CLUSGROUP_TYPE = 113;
pub const ClusGroupTypeInfrastructureFileServer: CLUSGROUP_TYPE = 122;
pub const ClusGroupTypeKeyValueStoreManager: CLUSGROUP_TYPE = 125;
pub const ClusGroupTypeMetaVirtualMachine: CLUSGROUP_TYPE = 127;
pub const ClusGroupTypeMsmq: CLUSGROUP_TYPE = 104;
pub const ClusGroupTypePrintServer: CLUSGROUP_TYPE = 101;
pub const ClusGroupTypeScaleoutCluster: CLUSGROUP_TYPE = 118;
pub const ClusGroupTypeScaleoutFileServer: CLUSGROUP_TYPE = 114;
pub const ClusGroupTypeSharedVolume: CLUSGROUP_TYPE = 4;
pub const ClusGroupTypeStandAloneDfs: CLUSGROUP_TYPE = 106;
pub const ClusGroupTypeStoragePool: CLUSGROUP_TYPE = 5;
pub const ClusGroupTypeStorageReplica: CLUSGROUP_TYPE = 119;
pub const ClusGroupTypeTaskScheduler: CLUSGROUP_TYPE = 116;
pub const ClusGroupTypeTemporary: CLUSGROUP_TYPE = 3;
pub const ClusGroupTypeTsSessionBroker: CLUSGROUP_TYPE = 112;
pub const ClusGroupTypeUnknown: CLUSGROUP_TYPE = 9999;
pub const ClusGroupTypeUserManager: CLUSGROUP_TYPE = 124;
pub const ClusGroupTypeVMReplicaBroker: CLUSGROUP_TYPE = 115;
pub const ClusGroupTypeVMReplicaCoordinator: CLUSGROUP_TYPE = 120;
pub const ClusGroupTypeVirtualMachine: CLUSGROUP_TYPE = 111;
pub const ClusGroupTypeWins: CLUSGROUP_TYPE = 105;
pub const ClusterGroupAllowFailback: CGAFT = 1;
pub const ClusterGroupFailbackTypeCount: CGAFT = 2;
pub const ClusterGroupFailed: CLUSTER_GROUP_STATE = 2;
pub const ClusterGroupOffline: CLUSTER_GROUP_STATE = 1;
pub const ClusterGroupOnline: CLUSTER_GROUP_STATE = 0;
pub const ClusterGroupPartialOnline: CLUSTER_GROUP_STATE = 3;
pub const ClusterGroupPending: CLUSTER_GROUP_STATE = 4;
pub const ClusterGroupPreventFailback: CGAFT = 0;
pub const ClusterGroupStateUnknown: CLUSTER_GROUP_STATE = -1;
pub const ClusterNetInterfaceFailed: CLUSTER_NETINTERFACE_STATE = 1;
pub const ClusterNetInterfaceStateUnknown: CLUSTER_NETINTERFACE_STATE = -1;
pub const ClusterNetInterfaceUnavailable: CLUSTER_NETINTERFACE_STATE = 0;
pub const ClusterNetInterfaceUnreachable: CLUSTER_NETINTERFACE_STATE = 2;
pub const ClusterNetInterfaceUp: CLUSTER_NETINTERFACE_STATE = 3;
pub const ClusterNetworkDown: CLUSTER_NETWORK_STATE = 1;
pub const ClusterNetworkPartitioned: CLUSTER_NETWORK_STATE = 2;
pub const ClusterNetworkRoleClientAccess: CLUSTER_NETWORK_ROLE = 2;
pub const ClusterNetworkRoleInternalAndClient: CLUSTER_NETWORK_ROLE = 3;
pub const ClusterNetworkRoleInternalUse: CLUSTER_NETWORK_ROLE = 1;
pub const ClusterNetworkRoleNone: CLUSTER_NETWORK_ROLE = 0;
pub const ClusterNetworkStateUnknown: CLUSTER_NETWORK_STATE = -1;
pub const ClusterNetworkUnavailable: CLUSTER_NETWORK_STATE = 0;
pub const ClusterNetworkUp: CLUSTER_NETWORK_STATE = 3;
pub const ClusterNodeDown: CLUSTER_NODE_STATE = 1;
pub const ClusterNodeDrainStatusCount: CLUSTER_NODE_DRAIN_STATUS = 4;
pub const ClusterNodeFailbackStatusCount: CLUSTER_NODE_FAILBACK_STATUS = 4;
pub const ClusterNodeJoining: CLUSTER_NODE_STATE = 3;
pub const ClusterNodePaused: CLUSTER_NODE_STATE = 2;
pub const ClusterNodeResumeFailbackTypeCount: CLUSTER_NODE_RESUME_FAILBACK_TYPE = 3;
pub const ClusterNodeStateUnknown: CLUSTER_NODE_STATE = -1;
pub const ClusterNodeUp: CLUSTER_NODE_STATE = 0;
pub const ClusterResourceDontRestart: CRRA = 0;
pub const ClusterResourceEmbeddedFailureActionLogOnly: CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION = 1;
pub const ClusterResourceEmbeddedFailureActionNone: CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION = 0;
pub const ClusterResourceEmbeddedFailureActionRecover: CLUSTER_RESOURCE_EMBEDDED_FAILURE_ACTION = 2;
pub const ClusterResourceFailed: CLUSTER_RESOURCE_STATE = 4;
pub const ClusterResourceInherited: CLUSTER_RESOURCE_STATE = 0;
pub const ClusterResourceInitializing: CLUSTER_RESOURCE_STATE = 1;
pub const ClusterResourceOffline: CLUSTER_RESOURCE_STATE = 3;
pub const ClusterResourceOfflinePending: CLUSTER_RESOURCE_STATE = 130;
pub const ClusterResourceOnline: CLUSTER_RESOURCE_STATE = 2;
pub const ClusterResourceOnlinePending: CLUSTER_RESOURCE_STATE = 129;
pub const ClusterResourcePending: CLUSTER_RESOURCE_STATE = 128;
pub const ClusterResourceRestartActionCount: CRRA = 3;
pub const ClusterResourceRestartNoNotify: CRRA = 1;
pub const ClusterResourceRestartNotify: CRRA = 2;
pub const ClusterResourceStateUnknown: CLUSTER_RESOURCE_STATE = -1;
pub const ClusterSetupPhaseAddClusterProperties: CLUSTER_SETUP_PHASE = 201;
pub const ClusterSetupPhaseAddNodeToCluster: CLUSTER_SETUP_PHASE = 301;
pub const ClusterSetupPhaseApplyNetworkATCIntents: CLUSTER_SETUP_PHASE = 303;
pub const ClusterSetupPhaseCleanupCOs: CLUSTER_SETUP_PHASE = 402;
pub const ClusterSetupPhaseCleanupNode: CLUSTER_SETUP_PHASE = 405;
pub const ClusterSetupPhaseClusterGroupOnline: CLUSTER_SETUP_PHASE = 206;
pub const ClusterSetupPhaseConfigureClusSvc: CLUSTER_SETUP_PHASE = 104;
pub const ClusterSetupPhaseConfigureClusterAccount: CLUSTER_SETUP_PHASE = 109;
pub const ClusterSetupPhaseContinue: CLUSTER_SETUP_PHASE_TYPE = 2;
pub const ClusterSetupPhaseCoreGroupCleanup: CLUSTER_SETUP_PHASE = 406;
pub const ClusterSetupPhaseCreateClusterAccount: CLUSTER_SETUP_PHASE = 108;
pub const ClusterSetupPhaseCreateGroups: CLUSTER_SETUP_PHASE = 203;
pub const ClusterSetupPhaseCreateIPAddressResources: CLUSTER_SETUP_PHASE = 204;
pub const ClusterSetupPhaseCreateNetworkName: CLUSTER_SETUP_PHASE = 205;
pub const ClusterSetupPhaseCreateResourceTypes: CLUSTER_SETUP_PHASE = 202;
pub const ClusterSetupPhaseDeleteGroup: CLUSTER_SETUP_PHASE = 401;
pub const ClusterSetupPhaseEnd: CLUSTER_SETUP_PHASE_TYPE = 3;
pub const ClusterSetupPhaseEvictNode: CLUSTER_SETUP_PHASE = 404;
pub const ClusterSetupPhaseFailureCleanup: CLUSTER_SETUP_PHASE = 999;
pub const ClusterSetupPhaseFatal: CLUSTER_SETUP_PHASE_SEVERITY = 3;
pub const ClusterSetupPhaseFormingCluster: CLUSTER_SETUP_PHASE = 200;
pub const ClusterSetupPhaseGettingCurrentMembership: CLUSTER_SETUP_PHASE = 300;
pub const ClusterSetupPhaseInformational: CLUSTER_SETUP_PHASE_SEVERITY = 1;
pub const ClusterSetupPhaseInitialize: CLUSTER_SETUP_PHASE = 1;
pub const ClusterSetupPhaseMoveGroup: CLUSTER_SETUP_PHASE = 400;
pub const ClusterSetupPhaseNodeUp: CLUSTER_SETUP_PHASE = 302;
pub const ClusterSetupPhaseOfflineGroup: CLUSTER_SETUP_PHASE = 403;
pub const ClusterSetupPhaseQueryClusterNameAccount: CLUSTER_SETUP_PHASE = 106;
pub const ClusterSetupPhaseRepairCNOAccount: CLUSTER_SETUP_PHASE = 500;
pub const ClusterSetupPhaseRepairDNSPermissions: CLUSTER_SETUP_PHASE = 501;
pub const ClusterSetupPhaseReport: CLUSTER_SETUP_PHASE_TYPE = 4;
pub const ClusterSetupPhaseStart: CLUSTER_SETUP_PHASE_TYPE = 1;
pub const ClusterSetupPhaseStartingClusSvc: CLUSTER_SETUP_PHASE = 105;
pub const ClusterSetupPhaseValidateClusDisk: CLUSTER_SETUP_PHASE = 103;
pub const ClusterSetupPhaseValidateClusterNameAccount: CLUSTER_SETUP_PHASE = 107;
pub const ClusterSetupPhaseValidateNetft: CLUSTER_SETUP_PHASE = 102;
pub const ClusterSetupPhaseValidateNodeState: CLUSTER_SETUP_PHASE = 100;
pub const ClusterSetupPhaseWarning: CLUSTER_SETUP_PHASE_SEVERITY = 2;
pub const ClusterSharedVolumeHWSnapshotCompleted: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE = 2;
pub const ClusterSharedVolumePrepareForFreeze: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE = 3;
pub const ClusterSharedVolumePrepareForHWSnapshot: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE = 1;
pub const ClusterSharedVolumeRenameInputTypeNone: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE = 0;
pub const ClusterSharedVolumeRenameInputTypeVolumeGuid: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE = 4;
pub const ClusterSharedVolumeRenameInputTypeVolumeId: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE = 2;
pub const ClusterSharedVolumeRenameInputTypeVolumeName: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE = 3;
pub const ClusterSharedVolumeRenameInputTypeVolumeOffset: CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE = 1;
pub const ClusterSharedVolumeSnapshotStateUnknown: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE = 0;
pub const ClusterStateNotConfigured: NODE_CLUSTER_STATE = 1;
pub const ClusterStateNotInstalled: NODE_CLUSTER_STATE = 0;
pub const ClusterStateNotRunning: NODE_CLUSTER_STATE = 3;
pub const ClusterStateRunning: NODE_CLUSTER_STATE = 19;
pub const ClusterStorageNodeDown: CLUSTER_STORAGENODE_STATE = 2;
pub const ClusterStorageNodePaused: CLUSTER_STORAGENODE_STATE = 3;
pub const ClusterStorageNodeStarting: CLUSTER_STORAGENODE_STATE = 4;
pub const ClusterStorageNodeStateUnknown: CLUSTER_STORAGENODE_STATE = 0;
pub const ClusterStorageNodeStopping: CLUSTER_STORAGENODE_STATE = 5;
pub const ClusterStorageNodeUp: CLUSTER_STORAGENODE_STATE = 1;
pub const ClusterUpgradePhaseInitialize: CLUSTER_UPGRADE_PHASE = 1;
pub const ClusterUpgradePhaseInstallingNewComponents: CLUSTER_UPGRADE_PHASE = 4;
pub const ClusterUpgradePhaseUpgradeComplete: CLUSTER_UPGRADE_PHASE = 5;
pub const ClusterUpgradePhaseUpgradingComponents: CLUSTER_UPGRADE_PHASE = 3;
pub const ClusterUpgradePhaseValidatingUpgrade: CLUSTER_UPGRADE_PHASE = 2;
pub const DNS_LENGTH: u32 = 64;
pub const DT_UPGRADE_VERSION: u32 = 1;
pub const DoNotFailbackGroups: CLUSTER_NODE_RESUME_FAILBACK_TYPE = 0;
pub const ENABLE_CLUSTER_SHARED_VOLUMES: windows_core::PCWSTR = windows_core::w!("EnableSharedVolumes");
pub const FE_22H2_UPGRADE_VERSION: u32 = 5;
pub const FE_UPGRADE_VERSION: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILESHARE_CHANGE {
    pub Change: FILESHARE_CHANGE_ENUM,
    pub ShareName: [u16; 84],
}
impl Default for FILESHARE_CHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILESHARE_CHANGE_ADD: FILESHARE_CHANGE_ENUM = 1;
pub const FILESHARE_CHANGE_DEL: FILESHARE_CHANGE_ENUM = 2;
pub type FILESHARE_CHANGE_ENUM = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILESHARE_CHANGE_LIST {
    pub NumEntries: u32,
    pub ChangeEntry: [FILESHARE_CHANGE; 0],
}
impl Default for FILESHARE_CHANGE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILESHARE_CHANGE_MODIFY: FILESHARE_CHANGE_ENUM = 3;
pub const FILESHARE_CHANGE_NONE: FILESHARE_CHANGE_ENUM = 0;
pub const FailbackGroupsImmediately: CLUSTER_NODE_RESUME_FAILBACK_TYPE = 1;
pub const FailbackGroupsPerPolicy: CLUSTER_NODE_RESUME_FAILBACK_TYPE = 2;
pub const GA_UPGRADE_VERSION: u32 = 5;
pub const GE_UPGRADE_VERSION: u32 = 6;
pub const GROUPSET_READY_SETTING_APPLICATION_READY: u32 = 4;
pub const GROUPSET_READY_SETTING_DELAY: u32 = 1;
pub const GROUPSET_READY_SETTING_ONLINE: u32 = 2;
pub const GROUPSET_READY_SETTING_OS_HEARTBEAT: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GROUP_FAILURE_INFO {
    pub dwFailoverAttemptsRemaining: u32,
    pub dwFailoverPeriodRemaining: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GROUP_FAILURE_INFO_BUFFER {
    pub dwVersion: u32,
    pub Info: GROUP_FAILURE_INFO,
}
pub const GROUP_FAILURE_INFO_VERSION_1: u32 = 1;
pub type GRP_PLACEMENT_OPTIONS = i32;
pub const GRP_PLACEMENT_OPTIONS_ALL: GRP_PLACEMENT_OPTIONS = 1;
pub const GRP_PLACEMENT_OPTIONS_DEFAULT: GRP_PLACEMENT_OPTIONS = 0;
pub const GRP_PLACEMENT_OPTIONS_DISABLE_AUTOBALANCING: GRP_PLACEMENT_OPTIONS = 1;
pub const GRP_PLACEMENT_OPTIONS_MIN_VALUE: GRP_PLACEMENT_OPTIONS = 0;
pub const GUID_PRESENT: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCHANGE(pub *mut _HCHANGE);
impl HCHANGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HCHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HCI_UPGRADE_BIT: u32 = 32768;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCLUSENUM(pub *mut _HCLUSENUM);
impl HCLUSENUM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HCLUSENUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCLUSENUMEX(pub *mut _HCLUSENUMEX);
impl HCLUSENUMEX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HCLUSENUMEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HCLUSTER(pub *mut _HCLUSTER);
impl HCLUSTER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HCLUSTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HGROUP(pub *mut _HGROUP);
impl HGROUP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HGROUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HGROUPENUM(pub *mut _HGROUPENUM);
impl HGROUPENUM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HGROUPENUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HGROUPENUMEX(pub *mut _HGROUPENUMEX);
impl HGROUPENUMEX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HGROUPENUMEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HGROUPSET(pub *mut _HGROUPSET);
impl HGROUPSET {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HGROUPSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HGROUPSETENUM(pub *mut _HGROUPSETENUM);
impl HGROUPSETENUM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HGROUPSETENUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HKEYVALUEBATCH(pub *mut _HKEYVALUEBATCH);
impl HKEYVALUEBATCH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HKEYVALUEBATCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HKEYVALUEBATCHNOTIFICATION(pub *mut _HKEYVALUEBATCHNOTIFICATION);
impl HKEYVALUEBATCHNOTIFICATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HKEYVALUEBATCHNOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HKEYVALUEREADBATCH(pub *mut _HKEYVALUEREADBATCH);
impl HKEYVALUEREADBATCH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HKEYVALUEREADBATCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HKEYVALUEREADBATCHREPLY(pub *mut _HKEYVALUEREADBATCHREPLY);
impl HKEYVALUEREADBATCHREPLY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HKEYVALUEREADBATCHREPLY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HKEYVALUESTORE(pub *mut _HKEYVALUESTORE);
impl HKEYVALUESTORE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HKEYVALUESTORE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HNETINTERFACE(pub *mut _HNETINTERFACE);
impl HNETINTERFACE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HNETINTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HNETINTERFACEENUM(pub *mut _HNETINTERFACEENUM);
impl HNETINTERFACEENUM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HNETINTERFACEENUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HNETWORK(pub *mut _HNETWORK);
impl HNETWORK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HNETWORK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HNETWORKENUM(pub *mut _HNETWORKENUM);
impl HNETWORKENUM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HNETWORKENUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HNODE(pub *mut _HNODE);
impl HNODE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HNODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HNODEENUM(pub *mut _HNODEENUM);
impl HNODEENUM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HNODEENUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HNODEENUMEX(pub *mut _HNODEENUMEX);
impl HNODEENUMEX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HNODEENUMEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HREGBATCH(pub *mut _HREGBATCH);
impl HREGBATCH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HREGBATCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HREGBATCHNOTIFICATION(pub *mut _HREGBATCHNOTIFICATION);
impl HREGBATCHNOTIFICATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HREGBATCHNOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HREGBATCHPORT(pub *mut _HREGBATCHPORT);
impl HREGBATCHPORT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HREGBATCHPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HREGREADBATCH(pub *mut _HREGREADBATCH);
impl HREGREADBATCH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HREGREADBATCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HREGREADBATCHREPLY(pub *mut _HREGREADBATCHREPLY);
impl HREGREADBATCHREPLY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HREGREADBATCHREPLY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HRESENUM(pub *mut _HRESENUM);
impl HRESENUM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HRESENUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HRESENUMEX(pub *mut _HRESENUMEX);
impl HRESENUMEX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HRESENUMEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HRESOURCE(pub *mut _HRESOURCE);
impl HRESOURCE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HRESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HRESTYPEENUM(pub *mut _HRESTYPEENUM);
impl HRESTYPEENUM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HRESTYPEENUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCLUSTERVERSIONINFO(pub *mut CLUSTERVERSIONINFO);
impl LPCLUSTERVERSIONINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCLUSTERVERSIONINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MAINTENANCE_MODE_TYPE_ENUM = i32;
pub const MAINTENANCE_MODE_V2_SIG: u32 = 2881155087;
pub const MAX_CLUSTERNAME_LENGTH: u32 = 63;
pub const MAX_CO_PASSWORD_LENGTH: u32 = 16;
pub const MAX_CO_PASSWORD_LENGTHEX: u32 = 127;
pub const MAX_CO_PASSWORD_STORAGEEX: u32 = 128;
pub const MAX_CREATINGDC_LENGTH: u32 = 256;
pub const MAX_OBJECTID: u32 = 64;
pub const MINIMUM_NEVER_PREEMPT_PRIORITY: windows_core::PCWSTR = windows_core::w!("MinimumNeverPreemptPriority");
pub const MINIMUM_PREEMPTOR_PRIORITY: windows_core::PCWSTR = windows_core::w!("MinimumPreemptorPriority");
pub const MN_UPGRADE_VERSION: u32 = 3;
pub const MaintenanceModeTypeDisableIsAliveCheck: MAINTENANCE_MODE_TYPE_ENUM = 1;
pub const MaintenanceModeTypeOfflineResource: MAINTENANCE_MODE_TYPE_ENUM = 2;
pub const MaintenanceModeTypeUnclusterResource: MAINTENANCE_MODE_TYPE_ENUM = 3;
pub const ModifyQuorum: CLUSTER_QUORUM_TYPE = 1;
pub const NINETEEN_H1_UPGRADE_VERSION: u32 = 1;
pub const NINETEEN_H2_UPGRADE_VERSION: u32 = 2;
pub const NI_UPGRADE_VERSION: u32 = 2;
pub type NODE_CLUSTER_STATE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NOTIFY_FILTER_AND_TYPE {
    pub dwObjectType: u32,
    pub FilterFlags: i64,
}
pub const NT10_MAJOR_VERSION: u32 = 9;
pub const NT11_MAJOR_VERSION: u32 = 10;
pub const NT12_MAJOR_VERSION: u32 = 11;
pub const NT13_MAJOR_VERSION: u32 = 12;
pub const NT14_MAJOR_VERSION: u32 = 14;
pub const NT4SP4_MAJOR_VERSION: u32 = 2;
pub const NT4_MAJOR_VERSION: u32 = 1;
pub const NT51_MAJOR_VERSION: u32 = 4;
pub const NT5_MAJOR_VERSION: u32 = 3;
pub const NT6_MAJOR_VERSION: u32 = 5;
pub const NT7_MAJOR_VERSION: u32 = 6;
pub const NT8_MAJOR_VERSION: u32 = 7;
pub const NT9_MAJOR_VERSION: u32 = 8;
pub const NodeDrainStatusCompleted: CLUSTER_NODE_DRAIN_STATUS = 2;
pub const NodeDrainStatusFailed: CLUSTER_NODE_DRAIN_STATUS = 3;
pub const NodeDrainStatusInProgress: CLUSTER_NODE_DRAIN_STATUS = 1;
pub const NodeDrainStatusNotInitiated: CLUSTER_NODE_DRAIN_STATUS = 0;
pub const NodeFailbackStatusCompleted: CLUSTER_NODE_FAILBACK_STATUS = 2;
pub const NodeFailbackStatusFailed: CLUSTER_NODE_FAILBACK_STATUS = 3;
pub const NodeFailbackStatusInProgress: CLUSTER_NODE_FAILBACK_STATUS = 1;
pub const NodeFailbackStatusNotInitiated: CLUSTER_NODE_FAILBACK_STATUS = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NodeSriovInfo {
    pub VFTotal: u32,
    pub VFUsed: u32,
    pub QPTotal: u32,
    pub QPUsed: u32,
}
pub const NodeStatusAvoidPlacement: CLUSTER_NODE_STATUS = 32;
pub const NodeStatusDrainCompleted: CLUSTER_NODE_STATUS = 8;
pub const NodeStatusDrainFailed: CLUSTER_NODE_STATUS = 16;
pub const NodeStatusDrainInProgress: CLUSTER_NODE_STATUS = 4;
pub const NodeStatusIsolated: CLUSTER_NODE_STATUS = 1;
pub const NodeStatusMax: CLUSTER_NODE_STATUS = 51;
pub const NodeStatusNormal: CLUSTER_NODE_STATUS = 0;
pub const NodeStatusQuarantined: CLUSTER_NODE_STATUS = 2;
pub const OperationalQuorum: CLUSTER_QUORUM_TYPE = 0;
pub type PCLUSAPI_ADD_CLUSTER_GROUP_DEPENDENCY = Option<unsafe extern "system" fn(hdependentgroup: *const _HGROUP, hprovidergroup: *const _HGROUP) -> u32>;
pub type PCLUSAPI_ADD_CLUSTER_GROUP_DEPENDENCY_EX = Option<unsafe extern "system" fn(hdependentgroup: *const _HGROUP, hprovidergroup: *const _HGROUP, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_ADD_CLUSTER_GROUP_GROUPSET_DEPENDENCY = Option<unsafe extern "system" fn(hdependentgroupset: *const _HGROUPSET, hprovidergroupset: *const _HGROUPSET) -> u32>;
pub type PCLUSAPI_ADD_CLUSTER_GROUP_GROUPSET_DEPENDENCY_EX = Option<unsafe extern "system" fn(hdependentgroupset: *const _HGROUPSET, hprovidergroupset: *const _HGROUPSET, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_ADD_CLUSTER_GROUP_TO_GROUP_GROUPSET_DEPENDENCY = Option<unsafe extern "system" fn(hdependentgroup: *const _HGROUP, hprovidergroupset: *const _HGROUPSET) -> u32>;
pub type PCLUSAPI_ADD_CLUSTER_GROUP_TO_GROUP_GROUPSET_DEPENDENCY_EX = Option<unsafe extern "system" fn(hdependentgroup: *const _HGROUP, hprovidergroupset: *const _HGROUPSET, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_ADD_CLUSTER_NODE = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznodename: windows_core::PCWSTR, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const core::ffi::c_void) -> HNODE>;
pub type PCLUSAPI_ADD_CLUSTER_NODE_EX = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznodename: windows_core::PCWSTR, dwflags: u32, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const core::ffi::c_void) -> HNODE>;
pub type PCLUSAPI_ADD_CLUSTER_RESOURCE_DEPENDENCY = Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, hdependson: *mut _HRESOURCE) -> u32>;
pub type PCLUSAPI_ADD_CLUSTER_RESOURCE_DEPENDENCY_EX = Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, hdependson: *const _HRESOURCE, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_ADD_CLUSTER_RESOURCE_NODE = Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, hnode: *mut _HNODE) -> u32>;
pub type PCLUSAPI_ADD_CLUSTER_RESOURCE_NODE_EX = Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, hnode: *const _HNODE, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_ADD_CROSS_CLUSTER_GROUPSET_DEPENDENCY = Option<unsafe extern "system" fn(hdependentgroupset: *const _HGROUPSET, lpremoteclustername: windows_core::PCWSTR, lpremotegroupsetname: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_ADD_RESOURCE_TO_CLUSTER_SHARED_VOLUMES = Option<unsafe extern "system" fn(hresource: *const _HRESOURCE) -> u32>;
pub type PCLUSAPI_BACKUP_CLUSTER_DATABASE = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszpathname: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CAN_RESOURCE_BE_DEPENDENT = Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, hresourcedependent: *mut _HRESOURCE) -> windows_core::BOOL>;
pub type PCLUSAPI_CHANGE_CLUSTER_RESOURCE_GROUP = Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, hgroup: *mut _HGROUP) -> u32>;
pub type PCLUSAPI_CHANGE_CLUSTER_RESOURCE_GROUP_EX = Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, hgroup: *mut _HGROUP, flags: u64) -> u32>;
pub type PCLUSAPI_CHANGE_CLUSTER_RESOURCE_GROUP_EX2 = Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, hgroup: *const _HGROUP, flags: u64, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLOSE_CLUSTER = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER) -> windows_core::BOOL>;
pub type PCLUSAPI_CLOSE_CLUSTER_GROUP = Option<unsafe extern "system" fn(hgroup: *const _HGROUP) -> windows_core::BOOL>;
pub type PCLUSAPI_CLOSE_CLUSTER_GROUP_GROUPSET = Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET) -> windows_core::BOOL>;
pub type PCLUSAPI_CLOSE_CLUSTER_NETWORK = Option<unsafe extern "system" fn(hnetwork: *const _HNETWORK) -> windows_core::BOOL>;
pub type PCLUSAPI_CLOSE_CLUSTER_NET_INTERFACE = Option<unsafe extern "system" fn(hnetinterface: *const _HNETINTERFACE) -> windows_core::BOOL>;
pub type PCLUSAPI_CLOSE_CLUSTER_NODE = Option<unsafe extern "system" fn(hnode: *const _HNODE) -> windows_core::BOOL>;
pub type PCLUSAPI_CLOSE_CLUSTER_NOTIFY_PORT = Option<unsafe extern "system" fn(hchange: *const _HCHANGE) -> windows_core::BOOL>;
pub type PCLUSAPI_CLOSE_CLUSTER_RESOURCE = Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE) -> windows_core::BOOL>;
pub type PCLUSAPI_CLUSTER_ADD_GROUP_TO_AFFINITY_RULE = Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, rulename: windows_core::PCWSTR, hgroup: *mut _HGROUP) -> u32>;
pub type PCLUSAPI_CLUSTER_ADD_GROUP_TO_GROUPSET_WITH_DOMAINS_EX = Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET, hgroup: *const _HGROUP, faultdomain: u32, updatedomain: u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_ADD_GROUP_TO_GROUP_GROUPSET = Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET, hgroup: *const _HGROUP) -> u32>;
pub type PCLUSAPI_CLUSTER_AFFINITY_RULE_CONTROL = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, affinityrulename: windows_core::PCWSTR, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_CLOSE_ENUM = Option<unsafe extern "system" fn(henum: *const _HCLUSENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_CLOSE_ENUM_EX = Option<unsafe extern "system" fn(hclusterenum: *const _HCLUSENUMEX) -> u32>;
pub type PCLUSAPI_CLUSTER_CONTROL = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_CONTROL_EX = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_CREATE_AFFINITY_RULE = Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, rulename: windows_core::PCWSTR, ruletype: CLUS_AFFINITY_RULE_TYPE) -> u32>;
pub type PCLUSAPI_CLUSTER_ENUM = Option<unsafe extern "system" fn(henum: *const _HCLUSENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_ENUM_EX = Option<unsafe extern "system" fn(hclusterenum: *const _HCLUSENUMEX, dwindex: u32, pitem: *mut CLUSTER_ENUM_ITEM, cbitem: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_GET_ENUM_COUNT = Option<unsafe extern "system" fn(henum: *const _HCLUSENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_GET_ENUM_COUNT_EX = Option<unsafe extern "system" fn(hclusterenum: *const _HCLUSENUMEX) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_CLOSE_ENUM = Option<unsafe extern "system" fn(hgroupenum: *mut _HGROUPENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_CLOSE_ENUM_EX = Option<unsafe extern "system" fn(hgroupenumex: *const _HGROUPENUMEX) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_CONTROL = Option<unsafe extern "system" fn(hgroup: *const _HGROUP, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_CONTROL_EX = Option<unsafe extern "system" fn(hgroup: *const _HGROUP, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_ENUM = Option<unsafe extern "system" fn(hgroupenum: *const _HGROUPENUM, dwindex: u32, lpdwtype: *mut u32, lpszresourcename: windows_core::PWSTR, lpcchname: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_ENUM_EX = Option<unsafe extern "system" fn(hgroupenumex: *const _HGROUPENUMEX, dwindex: u32, pitem: *mut CLUSTER_GROUP_ENUM_ITEM, cbitem: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_GET_ENUM_COUNT = Option<unsafe extern "system" fn(hgroupenum: *const _HGROUPENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_GET_ENUM_COUNT_EX = Option<unsafe extern "system" fn(hgroupenumex: *const _HGROUPENUMEX) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_GROUPSET_CONTROL = Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_GROUPSET_CONTROL_EX = Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_GROUP_OPEN_ENUM = Option<unsafe extern "system" fn(hgroup: *mut _HGROUP, dwtype: u32) -> HGROUPENUM>;
pub type PCLUSAPI_CLUSTER_GROUP_OPEN_ENUM_EX = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszproperties: windows_core::PCWSTR, cbproperties: u32, lpszroproperties: windows_core::PCWSTR, cbroproperties: u32, dwflags: u32) -> HGROUPENUMEX>;
pub type PCLUSAPI_CLUSTER_NETWORK_CLOSE_ENUM = Option<unsafe extern "system" fn(hnetworkenum: *const _HNETWORKENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_NETWORK_CONTROL = Option<unsafe extern "system" fn(hnetwork: *const _HNETWORK, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_NETWORK_CONTROL_EX = Option<unsafe extern "system" fn(hnetwork: *const _HNETWORK, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_NETWORK_ENUM = Option<unsafe extern "system" fn(hnetworkenum: *const _HNETWORKENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_NETWORK_GET_ENUM_COUNT = Option<unsafe extern "system" fn(hnetworkenum: *const _HNETWORKENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_NETWORK_OPEN_ENUM = Option<unsafe extern "system" fn(hnetwork: *const _HNETWORK, dwtype: u32) -> HNETWORKENUM>;
pub type PCLUSAPI_CLUSTER_NET_INTERFACE_CONTROL = Option<unsafe extern "system" fn(hnetinterface: *const _HNETINTERFACE, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_NET_INTERFACE_CONTROL_EX = Option<unsafe extern "system" fn(hnetinterface: *const _HNETINTERFACE, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_NODE_CLOSE_ENUM = Option<unsafe extern "system" fn(hnodeenum: *const _HNODEENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_NODE_CLOSE_ENUM_EX = Option<unsafe extern "system" fn(hnodeenum: *const _HNODEENUMEX) -> u32>;
pub type PCLUSAPI_CLUSTER_NODE_CONTROL = Option<unsafe extern "system" fn(hnode: *const _HNODE, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_NODE_CONTROL_EX = Option<unsafe extern "system" fn(hnode: *const _HNODE, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_NODE_ENUM = Option<unsafe extern "system" fn(hnodeenum: *const _HNODEENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_NODE_ENUM_EX = Option<unsafe extern "system" fn(hnodeenum: *const _HNODEENUMEX, dwindex: u32, pitem: *mut CLUSTER_ENUM_ITEM, cbitem: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_NODE_GET_ENUM_COUNT = Option<unsafe extern "system" fn(hnodeenum: *const _HNODEENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_NODE_GET_ENUM_COUNT_EX = Option<unsafe extern "system" fn(hnodeenum: *const _HNODEENUMEX) -> u32>;
pub type PCLUSAPI_CLUSTER_NODE_OPEN_ENUM = Option<unsafe extern "system" fn(hnode: *const _HNODE, dwtype: u32) -> HNODEENUM>;
pub type PCLUSAPI_CLUSTER_NODE_OPEN_ENUM_EX = Option<unsafe extern "system" fn(hnode: *const _HNODE, dwtype: u32, poptions: *const core::ffi::c_void) -> HNODEENUMEX>;
pub type PCLUSAPI_CLUSTER_OPEN_ENUM = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, dwtype: u32) -> HCLUSENUM>;
pub type PCLUSAPI_CLUSTER_OPEN_ENUM_EX = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, dwtype: u32, poptions: *const core::ffi::c_void) -> HCLUSENUMEX>;
#[cfg(feature = "Win32_minwindef")]
pub type PCLUSAPI_CLUSTER_REG_CLOSE_KEY = Option<unsafe extern "system" fn(hkey: super::minwindef::HKEY) -> i32>;
#[cfg(feature = "Win32_minwindef")]
pub type PCLUSAPI_CLUSTER_REG_CREATE_BATCH = Option<unsafe extern "system" fn(hkey: super::minwindef::HKEY, phregbatch: *mut HREGBATCH) -> i32>;
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_winreg"))]
pub type PCLUSAPI_CLUSTER_REG_CREATE_KEY = Option<unsafe extern "system" fn(hkey: super::minwindef::HKEY, lpszsubkey: windows_core::PCWSTR, dwoptions: u32, samdesired: super::winreg::REGSAM, lpsecurityattributes: *const super::minwinbase::SECURITY_ATTRIBUTES, phkresult: *mut super::minwindef::HKEY, lpdwdisposition: *mut u32) -> i32>;
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_winreg"))]
pub type PCLUSAPI_CLUSTER_REG_CREATE_KEY_EX = Option<unsafe extern "system" fn(hkey: super::minwindef::HKEY, lpszsubkey: windows_core::PCWSTR, dwoptions: u32, samdesired: super::winreg::REGSAM, lpsecurityattributes: *const super::minwinbase::SECURITY_ATTRIBUTES, phkresult: *mut super::minwindef::HKEY, lpdwdisposition: *mut u32, lpszreason: windows_core::PCWSTR) -> i32>;
#[cfg(feature = "Win32_minwindef")]
pub type PCLUSAPI_CLUSTER_REG_DELETE_KEY = Option<unsafe extern "system" fn(hkey: super::minwindef::HKEY, lpszsubkey: windows_core::PCWSTR) -> i32>;
#[cfg(feature = "Win32_minwindef")]
pub type PCLUSAPI_CLUSTER_REG_DELETE_KEY_EX = Option<unsafe extern "system" fn(hkey: super::minwindef::HKEY, lpsubkey: windows_core::PCWSTR, lpszreason: windows_core::PCWSTR) -> i32>;
#[cfg(feature = "Win32_minwindef")]
pub type PCLUSAPI_CLUSTER_REG_DELETE_VALUE = Option<unsafe extern "system" fn(hkey: super::minwindef::HKEY, lpszvaluename: windows_core::PCWSTR) -> u32>;
#[cfg(feature = "Win32_minwindef")]
pub type PCLUSAPI_CLUSTER_REG_DELETE_VALUE_EX = Option<unsafe extern "system" fn(hkey: super::minwindef::HKEY, lpszvaluename: windows_core::PCWSTR, lpszreason: windows_core::PCWSTR) -> u32>;
#[cfg(feature = "Win32_minwindef")]
pub type PCLUSAPI_CLUSTER_REG_ENUM_KEY = Option<unsafe extern "system" fn(hkey: super::minwindef::HKEY, dwindex: u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32, lpftlastwritetime: *mut super::minwindef::FILETIME) -> i32>;
#[cfg(feature = "Win32_minwindef")]
pub type PCLUSAPI_CLUSTER_REG_ENUM_VALUE = Option<unsafe extern "system" fn(hkey: super::minwindef::HKEY, dwindex: u32, lpszvaluename: windows_core::PWSTR, lpcchvaluename: *mut u32, lpdwtype: *mut u32, lpdata: *mut u8, lpcbdata: *mut u32) -> u32>;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PCLUSAPI_CLUSTER_REG_GET_KEY_SECURITY = Option<unsafe extern "system" fn(hkey: super::minwindef::HKEY, requestedinformation: super::winnt::SECURITY_INFORMATION, psecuritydescriptor: super::winnt::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> i32>;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_winreg"))]
pub type PCLUSAPI_CLUSTER_REG_OPEN_KEY = Option<unsafe extern "system" fn(hkey: super::minwindef::HKEY, lpszsubkey: windows_core::PCWSTR, samdesired: super::winreg::REGSAM, phkresult: *mut super::minwindef::HKEY) -> i32>;
#[cfg(feature = "Win32_minwindef")]
pub type PCLUSAPI_CLUSTER_REG_QUERY_INFO_KEY = Option<unsafe extern "system" fn(hkey: super::minwindef::HKEY, lpcsubkeys: *mut u32, lpcbmaxsubkeylen: *mut u32, lpcvalues: *mut u32, lpcbmaxvaluenamelen: *mut u32, lpcbmaxvaluelen: *mut u32, lpcbsecuritydescriptor: *mut u32, lpftlastwritetime: *mut super::minwindef::FILETIME) -> i32>;
#[cfg(feature = "Win32_minwindef")]
pub type PCLUSAPI_CLUSTER_REG_QUERY_VALUE = Option<unsafe extern "system" fn(hkey: super::minwindef::HKEY, lpszvaluename: windows_core::PCWSTR, lpdwvaluetype: *mut u32, lpdata: *mut u8, lpcbdata: *mut u32) -> i32>;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PCLUSAPI_CLUSTER_REG_SET_KEY_SECURITY = Option<unsafe extern "system" fn(hkey: super::minwindef::HKEY, securityinformation: super::winnt::SECURITY_INFORMATION, psecuritydescriptor: super::winnt::PSECURITY_DESCRIPTOR) -> i32>;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PCLUSAPI_CLUSTER_REG_SET_KEY_SECURITY_EX = Option<unsafe extern "system" fn(hkey: super::minwindef::HKEY, securityinformation: super::winnt::SECURITY_INFORMATION, psecuritydescriptor: super::winnt::PSECURITY_DESCRIPTOR, lpszreason: windows_core::PCWSTR) -> i32>;
#[cfg(feature = "Win32_minwindef")]
pub type PCLUSAPI_CLUSTER_REG_SET_VALUE = Option<unsafe extern "system" fn(hkey: super::minwindef::HKEY, lpszvaluename: windows_core::PCWSTR, dwtype: u32, lpdata: *const u8, cbdata: u32) -> u32>;
#[cfg(feature = "Win32_minwindef")]
pub type PCLUSAPI_CLUSTER_REG_SET_VALUE_EX = Option<unsafe extern "system" fn(hkey: super::minwindef::HKEY, lpszvaluename: windows_core::PCWSTR, dwtype: u32, lpdata: *const u8, cbdata: u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_REG_SYNC_DATABASE = Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, flags: u32) -> i32>;
pub type PCLUSAPI_CLUSTER_REMOVE_AFFINITY_RULE = Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, rulename: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_REMOVE_GROUP_FROM_AFFINITY_RULE = Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, rulename: windows_core::PCWSTR, hgroup: *mut _HGROUP) -> u32>;
pub type PCLUSAPI_CLUSTER_REMOVE_GROUP_FROM_GROUPSET = Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET) -> u32>;
pub type PCLUSAPI_CLUSTER_REMOVE_GROUP_FROM_GROUPSET_EX = Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_CLOSE_ENUM = Option<unsafe extern "system" fn(hresenum: *mut _HRESENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_CLOSE_ENUM_EX = Option<unsafe extern "system" fn(hresourceenumex: *const _HRESENUMEX) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_CONTROL = Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_CONTROL_AS_USER_EX = Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_CONTROL_EX = Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, cbinbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, cboutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_ENUM = Option<unsafe extern "system" fn(hresenum: *const _HRESENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_ENUM_EX = Option<unsafe extern "system" fn(hresourceenumex: *const _HRESENUMEX, dwindex: u32, pitem: *mut CLUSTER_RESOURCE_ENUM_ITEM, cbitem: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_GET_ENUM_COUNT = Option<unsafe extern "system" fn(hresenum: *const _HRESENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_GET_ENUM_COUNT_EX = Option<unsafe extern "system" fn(hresourceenumex: *const _HRESENUMEX) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_OPEN_ENUM = Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, dwtype: u32) -> HRESENUM>;
pub type PCLUSAPI_CLUSTER_RESOURCE_OPEN_ENUM_EX = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszproperties: windows_core::PCWSTR, cbproperties: u32, lpszroproperties: windows_core::PCWSTR, cbroproperties: u32, dwflags: u32) -> HRESENUMEX>;
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_CLOSE_ENUM = Option<unsafe extern "system" fn(hrestypeenum: *const _HRESTYPEENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_CONTROL = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszresourcetypename: windows_core::PCWSTR, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_CONTROL_AS_USER_EX = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszresourcetypename: windows_core::PCWSTR, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_CONTROL_EX = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszresourcetypename: windows_core::PCWSTR, hhostnode: *const _HNODE, dwcontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_ENUM = Option<unsafe extern "system" fn(hrestypeenum: *const _HRESTYPEENUM, dwindex: u32, lpdwtype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_GET_ENUM_COUNT = Option<unsafe extern "system" fn(hrestypeenum: *const _HRESTYPEENUM) -> u32>;
pub type PCLUSAPI_CLUSTER_RESOURCE_TYPE_OPEN_ENUM = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszresourcetypename: windows_core::PCWSTR, dwtype: u32) -> HRESTYPEENUM>;
pub type PCLUSAPI_CLUSTER_UPGRADE = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, perform: windows_core::BOOL, pfnprogresscallback: PCLUSTER_UPGRADE_PROGRESS_CALLBACK, pvcallbackarg: *const core::ffi::c_void) -> u32>;
pub type PCLUSAPI_CREATE_CLUSTER = Option<unsafe extern "system" fn(pconfig: *const CREATE_CLUSTER_CONFIG, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const core::ffi::c_void) -> HCLUSTER>;
pub type PCLUSAPI_CREATE_CLUSTER_AVAILABILITY_SET = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpavailabilitysetname: windows_core::PCWSTR, pavailabilitysetconfig: *const CLUSTER_AVAILABILITY_SET_CONFIG) -> HGROUPSET>;
pub type PCLUSAPI_CREATE_CLUSTER_CNOLESS = Option<unsafe extern "system" fn(pconfig: *const CREATE_CLUSTER_CONFIG, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const core::ffi::c_void) -> HCLUSTER>;
pub type PCLUSAPI_CREATE_CLUSTER_GROUP = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszgroupname: windows_core::PCWSTR) -> HGROUP>;
pub type PCLUSAPI_CREATE_CLUSTER_GROUPEX = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszgroupname: windows_core::PCWSTR, pgroupinfo: *const CLUSTER_CREATE_GROUP_INFO) -> HGROUP>;
pub type PCLUSAPI_CREATE_CLUSTER_GROUP_GROUPSET = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszgroupsetname: windows_core::PCWSTR) -> HGROUPSET>;
pub type PCLUSAPI_CREATE_CLUSTER_NAME_ACCOUNT = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, pconfig: *const CREATE_CLUSTER_NAME_ACCOUNT, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const core::ffi::c_void) -> u32>;
pub type PCLUSAPI_CREATE_CLUSTER_NOTIFY_PORT = Option<unsafe extern "system" fn(hchange: *const _HCHANGE, hcluster: *const _HCLUSTER, dwfilter: u32, dwnotifykey: usize) -> HCHANGE>;
pub type PCLUSAPI_CREATE_CLUSTER_NOTIFY_PORT_V2 = Option<unsafe extern "system" fn(hchange: *const _HCHANGE, hcluster: *const _HCLUSTER, filters: *const NOTIFY_FILTER_AND_TYPE, dwfiltercount: u32, dwnotifykey: usize) -> HCHANGE>;
pub type PCLUSAPI_CREATE_CLUSTER_RESOURCE = Option<unsafe extern "system" fn(hgroup: *mut _HGROUP, lpszresourcename: windows_core::PCWSTR, lpszresourcetype: windows_core::PCWSTR, dwflags: u32) -> HRESOURCE>;
pub type PCLUSAPI_CREATE_CLUSTER_RESOURCE_EX = Option<unsafe extern "system" fn(hgroup: *const _HGROUP, lpszresourcename: windows_core::PCWSTR, lpszresourcetype: windows_core::PCWSTR, dwflags: u32, lpszreason: windows_core::PCWSTR) -> HRESOURCE>;
pub type PCLUSAPI_CREATE_CLUSTER_RESOURCE_TYPE = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszresourcetypename: windows_core::PCWSTR, lpszdisplayname: windows_core::PCWSTR, lpszresourcetypedll: windows_core::PCWSTR, dwlooksalivepollinterval: u32, dwisalivepollinterval: u32) -> u32>;
pub type PCLUSAPI_CREATE_CLUSTER_RESOURCE_TYPE_EX = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszresourcetypename: windows_core::PCWSTR, lpszdisplayname: windows_core::PCWSTR, lpszresourcetypedll: windows_core::PCWSTR, dwlooksalivepollinterval: u32, dwisalivepollinterval: u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_DELETE_CLUSTER_GROUP = Option<unsafe extern "system" fn(hgroup: *mut _HGROUP) -> u32>;
pub type PCLUSAPI_DELETE_CLUSTER_GROUP_EX = Option<unsafe extern "system" fn(hgroup: *const _HGROUP, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_DELETE_CLUSTER_GROUP_GROUPSET = Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET) -> u32>;
pub type PCLUSAPI_DELETE_CLUSTER_GROUP_GROUPSET_EX = Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_DELETE_CLUSTER_RESOURCE = Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE) -> u32>;
pub type PCLUSAPI_DELETE_CLUSTER_RESOURCE_EX = Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_DELETE_CLUSTER_RESOURCE_TYPE = Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, lpszresourcetypename: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_DELETE_CLUSTER_RESOURCE_TYPE_EX = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsztypename: windows_core::PCWSTR, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_DESTROY_CLUSTER = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, pfnprogresscallback: PCLUSTER_SETUP_PROGRESS_CALLBACK, pvcallbackarg: *const core::ffi::c_void, fdeletevirtualcomputerobjects: windows_core::BOOL) -> u32>;
pub type PCLUSAPI_DESTROY_CLUSTER_GROUP = Option<unsafe extern "system" fn(hgroup: *mut _HGROUP) -> u32>;
pub type PCLUSAPI_DESTROY_CLUSTER_GROUP_EX = Option<unsafe extern "system" fn(hgroup: *const _HGROUP, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_EVICT_CLUSTER_NODE = Option<unsafe extern "system" fn(hnode: *const _HNODE) -> u32>;
pub type PCLUSAPI_EVICT_CLUSTER_NODE_EX = Option<unsafe extern "system" fn(hnode: *const _HNODE, dwtimeout: u32, phrcleanupstatus: *mut windows_core::HRESULT) -> u32>;
pub type PCLUSAPI_EVICT_CLUSTER_NODE_EX2 = Option<unsafe extern "system" fn(hnode: *const _HNODE, dwtimeout: u32, phrcleanupstatus: *mut windows_core::HRESULT, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_FAIL_CLUSTER_RESOURCE = Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE) -> u32>;
pub type PCLUSAPI_FAIL_CLUSTER_RESOURCE_EX = Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_GET_CLUSTER_FROM_GROUP = Option<unsafe extern "system" fn(hgroup: *const _HGROUP) -> HCLUSTER>;
pub type PCLUSAPI_GET_CLUSTER_FROM_GROUP_GROUPSET = Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET) -> HCLUSTER>;
pub type PCLUSAPI_GET_CLUSTER_FROM_NETWORK = Option<unsafe extern "system" fn(hnetwork: *const _HNETWORK) -> HCLUSTER>;
pub type PCLUSAPI_GET_CLUSTER_FROM_NET_INTERFACE = Option<unsafe extern "system" fn(hnetinterface: *const _HNETINTERFACE) -> HCLUSTER>;
pub type PCLUSAPI_GET_CLUSTER_FROM_NODE = Option<unsafe extern "system" fn(hnode: *const _HNODE) -> HCLUSTER>;
pub type PCLUSAPI_GET_CLUSTER_FROM_RESOURCE = Option<unsafe extern "system" fn(hresource: *const _HRESOURCE) -> HCLUSTER>;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_winreg"))]
pub type PCLUSAPI_GET_CLUSTER_GROUP_KEY = Option<unsafe extern "system" fn(hgroup: *mut _HGROUP, samdesired: super::winreg::REGSAM) -> super::minwindef::HKEY>;
pub type PCLUSAPI_GET_CLUSTER_GROUP_STATE = Option<unsafe extern "system" fn(hgroup: *const _HGROUP, lpsznodename: windows_core::PWSTR, lpcchnodename: *mut u32) -> CLUSTER_GROUP_STATE>;
pub type PCLUSAPI_GET_CLUSTER_INFORMATION = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszclustername: windows_core::PWSTR, lpcchclustername: *mut u32, lpclusterinfo: *mut CLUSTERVERSIONINFO) -> u32>;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_winreg"))]
pub type PCLUSAPI_GET_CLUSTER_KEY = Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, samdesired: super::winreg::REGSAM) -> super::minwindef::HKEY>;
pub type PCLUSAPI_GET_CLUSTER_NETWORK_ID = Option<unsafe extern "system" fn(hnetwork: *const _HNETWORK, lpsznetworkid: windows_core::PWSTR, lpcchname: *mut u32) -> u32>;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_winreg"))]
pub type PCLUSAPI_GET_CLUSTER_NETWORK_KEY = Option<unsafe extern "system" fn(hnetwork: *const _HNETWORK, samdesired: super::winreg::REGSAM) -> super::minwindef::HKEY>;
pub type PCLUSAPI_GET_CLUSTER_NETWORK_STATE = Option<unsafe extern "system" fn(hnetwork: *const _HNETWORK) -> CLUSTER_NETWORK_STATE>;
pub type PCLUSAPI_GET_CLUSTER_NET_INTERFACE = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznodename: windows_core::PCWSTR, lpsznetworkname: windows_core::PCWSTR, lpszinterfacename: windows_core::PWSTR, lpcchinterfacename: *mut u32) -> u32>;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_winreg"))]
pub type PCLUSAPI_GET_CLUSTER_NET_INTERFACE_KEY = Option<unsafe extern "system" fn(hnetinterface: *const _HNETINTERFACE, samdesired: super::winreg::REGSAM) -> super::minwindef::HKEY>;
pub type PCLUSAPI_GET_CLUSTER_NET_INTERFACE_STATE = Option<unsafe extern "system" fn(hnetinterface: *const _HNETINTERFACE) -> CLUSTER_NETINTERFACE_STATE>;
pub type PCLUSAPI_GET_CLUSTER_NODE_ID = Option<unsafe extern "system" fn(hnode: *const _HNODE, lpsznodeid: windows_core::PWSTR, lpcchname: *mut u32) -> u32>;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_winreg"))]
pub type PCLUSAPI_GET_CLUSTER_NODE_KEY = Option<unsafe extern "system" fn(hnode: *mut _HNODE, samdesired: super::winreg::REGSAM) -> super::minwindef::HKEY>;
pub type PCLUSAPI_GET_CLUSTER_NODE_STATE = Option<unsafe extern "system" fn(hnode: *const _HNODE) -> CLUSTER_NODE_STATE>;
pub type PCLUSAPI_GET_CLUSTER_NOTIFY = Option<unsafe extern "system" fn(hchange: *const _HCHANGE, lpdwnotifykey: *mut usize, lpdwfiltertype: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32, dwmilliseconds: u32) -> u32>;
pub type PCLUSAPI_GET_CLUSTER_NOTIFY_V2 = Option<unsafe extern "system" fn(hchange: *const _HCHANGE, lpdwnotifykey: *mut usize, pfilterandtype: *mut NOTIFY_FILTER_AND_TYPE, buffer: *mut u8, lpcchbuffersize: *mut u32, lpszobjectid: windows_core::PWSTR, lpcchobjectid: *mut u32, lpszparentid: windows_core::PWSTR, lpcchparentid: *mut u32, lpszname: windows_core::PWSTR, lpcchname: *mut u32, lpsztype: windows_core::PWSTR, lpcchtype: *mut u32, dwmilliseconds: u32) -> u32>;
pub type PCLUSAPI_GET_CLUSTER_QUORUM_RESOURCE = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszresourcename: windows_core::PWSTR, lpcchresourcename: *mut u32, lpszdevicename: windows_core::PWSTR, lpcchdevicename: *mut u32, lpdwmaxquorumlogsize: *mut u32) -> u32>;
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_DEPENDENCY_EXPRESSION = Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, lpszdependencyexpression: windows_core::PWSTR, lpcchdependencyexpression: *mut u32) -> u32>;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_winreg"))]
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_KEY = Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, samdesired: super::winreg::REGSAM) -> super::minwindef::HKEY>;
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_NETWORK_NAME = Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, lpbuffer: windows_core::PWSTR, nsize: *mut u32) -> windows_core::BOOL>;
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_STATE = Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, lpsznodename: windows_core::PWSTR, lpcchnodename: *mut u32, lpszgroupname: windows_core::PWSTR, lpcchgroupname: *mut u32) -> CLUSTER_RESOURCE_STATE>;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt", feature = "Win32_winreg"))]
pub type PCLUSAPI_GET_CLUSTER_RESOURCE_TYPE_KEY = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsztypename: windows_core::PCWSTR, samdesired: super::winreg::REGSAM) -> super::minwindef::HKEY>;
pub type PCLUSAPI_GET_NODE_CLUSTER_STATE = Option<unsafe extern "system" fn(lpsznodename: windows_core::PCWSTR, pdwclusterstate: *mut u32) -> u32>;
#[cfg(feature = "Win32_winnt")]
pub type PCLUSAPI_GET_NOTIFY_EVENT_HANDLE_V2 = Option<unsafe extern "system" fn(hchange: *const _HCHANGE, lphtargetevent: *mut super::winnt::HANDLE) -> u32>;
pub type PCLUSAPI_IS_FILE_ON_CLUSTER_SHARED_VOLUME = Option<unsafe extern "system" fn(lpszpathname: windows_core::PCWSTR, pbfileisonsharedvolume: *mut windows_core::BOOL) -> u32>;
pub type PCLUSAPI_MOVE_CLUSTER_GROUP = Option<unsafe extern "system" fn(hgroup: *const _HGROUP, hdestinationnode: *const _HNODE) -> u32>;
pub type PCLUSAPI_OFFLINE_CLUSTER_GROUP = Option<unsafe extern "system" fn(hgroup: *mut _HGROUP) -> u32>;
pub type PCLUSAPI_OFFLINE_CLUSTER_RESOURCE = Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE) -> u32>;
pub type PCLUSAPI_ONLINE_CLUSTER_GROUP = Option<unsafe extern "system" fn(hgroup: *const _HGROUP, hdestinationnode: *const _HNODE) -> u32>;
pub type PCLUSAPI_ONLINE_CLUSTER_RESOURCE = Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE) -> u32>;
pub type PCLUSAPI_OPEN_CLUSTER = Option<unsafe extern "system" fn(lpszclustername: windows_core::PCWSTR) -> HCLUSTER>;
pub type PCLUSAPI_OPEN_CLUSTER_EX = Option<unsafe extern "system" fn(lpszclustername: windows_core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> HCLUSTER>;
pub type PCLUSAPI_OPEN_CLUSTER_GROUP = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszgroupname: windows_core::PCWSTR) -> HGROUP>;
pub type PCLUSAPI_OPEN_CLUSTER_GROUP_EX = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszgroupname: windows_core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> HGROUP>;
pub type PCLUSAPI_OPEN_CLUSTER_GROUP_GROUPSET = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszgroupsetname: windows_core::PCWSTR) -> HGROUPSET>;
pub type PCLUSAPI_OPEN_CLUSTER_NETINTERFACE_EX = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznetinterfacename: windows_core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> HNETINTERFACE>;
pub type PCLUSAPI_OPEN_CLUSTER_NETWORK = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznetworkname: windows_core::PCWSTR) -> HNETWORK>;
pub type PCLUSAPI_OPEN_CLUSTER_NETWORK_EX = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznetworkname: windows_core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> HNETWORK>;
pub type PCLUSAPI_OPEN_CLUSTER_NET_INTERFACE = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszinterfacename: windows_core::PCWSTR) -> HNETINTERFACE>;
pub type PCLUSAPI_OPEN_CLUSTER_NODE = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznodename: windows_core::PCWSTR) -> HNODE>;
pub type PCLUSAPI_OPEN_CLUSTER_NODE_EX = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznodename: windows_core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> HNODE>;
pub type PCLUSAPI_OPEN_CLUSTER_RESOURCE = Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, lpszresourcename: windows_core::PCWSTR) -> HRESOURCE>;
pub type PCLUSAPI_OPEN_CLUSTER_RESOURCE_EX = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpszresourcename: windows_core::PCWSTR, dwdesiredaccess: u32, lpdwgrantedaccess: *mut u32) -> HRESOURCE>;
pub type PCLUSAPI_OPEN_NODE_BY_ID = Option<unsafe extern "system" fn(hcluster: *mut _HCLUSTER, nodeid: u32) -> HNODE>;
pub type PCLUSAPI_PAUSE_CLUSTER_NODE = Option<unsafe extern "system" fn(hnode: *const _HNODE) -> u32>;
pub type PCLUSAPI_PAUSE_CLUSTER_NODE_EX = Option<unsafe extern "system" fn(hnode: *const _HNODE, bdrainnode: windows_core::BOOL, dwpauseflags: u32, hnodedraintarget: *const _HNODE) -> u32>;
pub type PCLUSAPI_PAUSE_CLUSTER_NODE_EX2 = Option<unsafe extern "system" fn(hnode: *const _HNODE, bdrainnode: windows_core::BOOL, dwpauseflags: u32, hnodedraintarget: *const _HNODE, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_PFN_REASON_HANDLER = Option<unsafe extern "system" fn(lpparameter: *const core::ffi::c_void, hcluster: *const _HCLUSTER, szreason: windows_core::PWSTR, lpsize: *mut u32) -> windows_core::BOOL>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSAPI_REASON_HANDLER(pub *mut CLUSAPI_REASON_HANDLER);
impl PCLUSAPI_REASON_HANDLER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSAPI_REASON_HANDLER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
pub type PCLUSAPI_REGISTER_CLUSTER_NOTIFY = Option<unsafe extern "system" fn(hchange: *const _HCHANGE, dwfiltertype: u32, hobject: super::winnt::HANDLE, dwnotifykey: usize) -> u32>;
#[cfg(feature = "Win32_winnt")]
pub type PCLUSAPI_REGISTER_CLUSTER_NOTIFY_V2 = Option<unsafe extern "system" fn(hchange: *const _HCHANGE, filter: NOTIFY_FILTER_AND_TYPE, hobject: super::winnt::HANDLE, dwnotifykey: usize) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_GROUP_DEPENDENCY = Option<unsafe extern "system" fn(hgroup: *const _HGROUP, hdependson: *const _HGROUP) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_GROUP_DEPENDENCY_EX = Option<unsafe extern "system" fn(hgroup: *const _HGROUP, hdependson: *const _HGROUP, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_GROUP_GROUPSET_DEPENDENCY = Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET, hdependson: *const _HGROUPSET) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_GROUP_GROUPSET_DEPENDENCY_EX = Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET, hdependson: *const _HGROUPSET, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_GROUP_TO_GROUP_GROUPSET_DEPENDENCY = Option<unsafe extern "system" fn(hgroup: *const _HGROUP, hdependson: *const _HGROUPSET) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_GROUP_TO_GROUP_GROUPSET_DEPENDENCY_EX = Option<unsafe extern "system" fn(hgroup: *const _HGROUP, hdependson: *const _HGROUPSET, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_NAME_ACCOUNT = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_RESOURCE_DEPENDENCY = Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, hdependson: *mut _HRESOURCE) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_RESOURCE_DEPENDENCY_EX = Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, hdependson: *const _HRESOURCE, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_RESOURCE_NODE = Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, hnode: *mut _HNODE) -> u32>;
pub type PCLUSAPI_REMOVE_CLUSTER_RESOURCE_NODE_EX = Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, hnode: *const _HNODE, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_REMOVE_CROSS_CLUSTER_GROUPSET_DEPENDENCY = Option<unsafe extern "system" fn(hdependentgroupset: *const _HGROUPSET, lpremoteclustername: windows_core::PCWSTR, lpremotegroupsetname: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_REMOVE_RESOURCE_FROM_CLUSTER_SHARED_VOLUMES = Option<unsafe extern "system" fn(hresource: *const _HRESOURCE) -> u32>;
pub type PCLUSAPI_REPAIR_CLUSTER_NAME_ACCOUNT = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER) -> u32>;
pub type PCLUSAPI_RESTART_CLUSTER_RESOURCE = Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, dwflags: u32) -> u32>;
pub type PCLUSAPI_RESTART_CLUSTER_RESOURCE_EX = Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, dwflags: u32) -> u32>;
pub type PCLUSAPI_RESTORE_CLUSTER_DATABASE = Option<unsafe extern "system" fn(lpszpathname: windows_core::PCWSTR, bforce: windows_core::BOOL, lpszquorumdriveletter: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_RESUME_CLUSTER_NODE = Option<unsafe extern "system" fn(hnode: *const _HNODE) -> u32>;
pub type PCLUSAPI_RESUME_CLUSTER_NODE_EX = Option<unsafe extern "system" fn(hnode: *const _HNODE, eresumefailbacktype: CLUSTER_NODE_RESUME_FAILBACK_TYPE, dwresumeflagsreserved: u32) -> u32>;
pub type PCLUSAPI_RESUME_CLUSTER_NODE_EX2 = Option<unsafe extern "system" fn(hnode: *const _HNODE, eresumefailbacktype: CLUSTER_NODE_RESUME_FAILBACK_TYPE, dwresumeflagsreserved: u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_GROUP_GROUPSET_DEPENDENCY_EXPRESSION = Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET, lpszdependencyexpression: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_GROUP_GROUPSET_DEPENDENCY_EXPRESSION_EX = Option<unsafe extern "system" fn(hgroupset: *const _HGROUPSET, lpszdependencyexpression: windows_core::PCWSTR, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_GROUP_NAME = Option<unsafe extern "system" fn(hgroup: *mut _HGROUP, lpszgroupname: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_GROUP_NAME_EX = Option<unsafe extern "system" fn(hgroup: *const _HGROUP, lpszgroupname: windows_core::PCWSTR, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_GROUP_NODE_LIST = Option<unsafe extern "system" fn(hgroup: *const _HGROUP, nodecount: u32, nodelist: *const HNODE) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_GROUP_NODE_LIST_EX = Option<unsafe extern "system" fn(hgroup: *const _HGROUP, nodecount: u32, nodelist: *const HNODE, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_NAME_EX = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznewclustername: windows_core::PCWSTR, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_NETWORK_NAME = Option<unsafe extern "system" fn(hnetwork: *const _HNETWORK, lpszname: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_NETWORK_NAME_EX = Option<unsafe extern "system" fn(hnetwork: *const _HNETWORK, lpszname: windows_core::PCWSTR, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_NETWORK_PRIORITY_ORDER = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, networkcount: u32, networklist: *const HNETWORK) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_QUORUM_RESOURCE = Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, lpszdevicename: windows_core::PCWSTR, dwmaxquologsize: u32) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_QUORUM_RESOURCE_EX = Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, lpszdevicename: windows_core::PCWSTR, dwmaxquorumlogsize: u32, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_RESOURCE_DEPENDENCY_EXPRESSION = Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, lpszdependencyexpression: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_RESOURCE_NAME = Option<unsafe extern "system" fn(hresource: *mut _HRESOURCE, lpszresourcename: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_RESOURCE_NAME_EX = Option<unsafe extern "system" fn(hresource: *const _HRESOURCE, lpszresourcename: windows_core::PCWSTR, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_CLUSTER_SERVICE_ACCOUNT_PASSWORD = Option<unsafe extern "system" fn(lpszclustername: windows_core::PCWSTR, lpsznewpassword: windows_core::PCWSTR, dwflags: u32, lpreturnstatusbuffer: *mut CLUSTER_SET_PASSWORD_STATUS, lpcbreturnstatusbuffersize: *mut u32) -> u32>;
pub type PCLUSAPI_SET_GROUP_DEPENDENCY_EXPRESSION = Option<unsafe extern "system" fn(hgroupset: *const _HGROUP, lpszdependencyexpression: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_GROUP_DEPENDENCY_EXPRESSION_EX = Option<unsafe extern "system" fn(hgroup: *const _HGROUP, lpszdependencyexpression: windows_core::PCWSTR, lpszreason: windows_core::PCWSTR) -> u32>;
pub type PCLUSAPI_SET_REASON_HANDLER = Option<unsafe extern "system" fn(lphandler: *const CLUSAPI_REASON_HANDLER) -> PCLUSAPI_REASON_HANDLER>;
pub type PCLUSAPI_SHARED_VOLUME_SET_SNAPSHOT_STATE = Option<unsafe extern "system" fn(guidsnapshotset: windows_core::GUID, lpszvolumename: windows_core::PCWSTR, state: CLUSTER_SHARED_VOLUME_SNAPSHOT_STATE) -> u32>;
pub type PCLUSAPI_SetClusterName = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, lpsznewclustername: windows_core::PCWSTR) -> u32>;
#[cfg(feature = "Win32_minwinbase")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT(pub *mut CLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT);
#[cfg(feature = "Win32_minwinbase")]
impl PCLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwinbase")]
impl Default for PCLUSCTL_GROUP_GET_LAST_MOVE_TIME_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT(pub *mut CLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT);
impl PCLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSCTL_RESOURCE_STATE_CHANGE_REASON_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT(pub *mut CLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT);
impl PCLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSCTL_RESOURCE_TYPE_STORAGE_GET_AVAILABLE_DISKS_EX2_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSGROUP_TYPE(pub *mut CLUSGROUP_TYPE);
impl PCLUSGROUP_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSGROUP_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSPROP_BINARY(pub *mut CLUSPROP_BINARY);
impl PCLUSPROP_BINARY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSPROP_BINARY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSPROP_BUFFER_HELPER(pub *mut CLUSPROP_BUFFER_HELPER);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl PCLUSPROP_BUFFER_HELPER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for PCLUSPROP_BUFFER_HELPER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSPROP_DISK_NUMBER(pub *mut CLUSPROP_DWORD);
impl PCLUSPROP_DISK_NUMBER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSPROP_DISK_NUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSPROP_DISK_SIGNATURE(pub *mut CLUSPROP_DWORD);
impl PCLUSPROP_DISK_SIGNATURE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSPROP_DISK_SIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSPROP_DWORD(pub *mut CLUSPROP_DWORD);
impl PCLUSPROP_DWORD {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSPROP_DWORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSPROP_FILETIME(pub *mut CLUSPROP_FILETIME);
#[cfg(feature = "Win32_minwindef")]
impl PCLUSPROP_FILETIME {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PCLUSPROP_FILETIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSPROP_FTSET_INFO(pub *mut CLUSPROP_FTSET_INFO);
impl PCLUSPROP_FTSET_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSPROP_FTSET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSPROP_LARGE_INTEGER(pub *mut CLUSPROP_LARGE_INTEGER);
impl PCLUSPROP_LARGE_INTEGER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSPROP_LARGE_INTEGER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSPROP_LIST(pub *mut CLUSPROP_LIST);
impl PCLUSPROP_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSPROP_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSPROP_LONG(pub *mut CLUSPROP_LONG);
impl PCLUSPROP_LONG {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSPROP_LONG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSPROP_MULTI_SZ(pub *mut CLUSPROP_SZ);
impl PCLUSPROP_MULTI_SZ {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSPROP_MULTI_SZ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSPROP_PARTITION_INFO(pub *mut CLUSPROP_PARTITION_INFO);
impl PCLUSPROP_PARTITION_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSPROP_PARTITION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSPROP_PARTITION_INFO_EX(pub *mut CLUSPROP_PARTITION_INFO_EX);
impl PCLUSPROP_PARTITION_INFO_EX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSPROP_PARTITION_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSPROP_PARTITION_INFO_EX2(pub *mut CLUSPROP_PARTITION_INFO_EX2);
impl PCLUSPROP_PARTITION_INFO_EX2 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSPROP_PARTITION_INFO_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSPROP_PROPERTY_NAME(pub *mut CLUSPROP_SZ);
impl PCLUSPROP_PROPERTY_NAME {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSPROP_PROPERTY_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSPROP_REQUIRED_DEPENDENCY(pub *mut CLUSPROP_REQUIRED_DEPENDENCY);
impl PCLUSPROP_REQUIRED_DEPENDENCY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSPROP_REQUIRED_DEPENDENCY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSPROP_RESOURCE_CLASS(pub *mut CLUSPROP_RESOURCE_CLASS);
impl PCLUSPROP_RESOURCE_CLASS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSPROP_RESOURCE_CLASS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSPROP_RESOURCE_CLASS_INFO(pub *mut CLUSPROP_RESOURCE_CLASS_INFO);
impl PCLUSPROP_RESOURCE_CLASS_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSPROP_RESOURCE_CLASS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSPROP_SCSI_ADDRESS(pub *mut CLUSPROP_SCSI_ADDRESS);
impl PCLUSPROP_SCSI_ADDRESS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSPROP_SCSI_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSPROP_SECURITY_DESCRIPTOR(pub *mut CLUSPROP_SECURITY_DESCRIPTOR);
#[cfg(feature = "Win32_winnt")]
impl PCLUSPROP_SECURITY_DESCRIPTOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PCLUSPROP_SECURITY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSPROP_SYNTAX(pub *mut CLUSPROP_SYNTAX);
impl PCLUSPROP_SYNTAX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSPROP_SYNTAX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSPROP_SZ(pub *mut CLUSPROP_SZ);
impl PCLUSPROP_SZ {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSPROP_SZ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSPROP_ULARGE_INTEGER(pub *mut CLUSPROP_ULARGE_INTEGER);
impl PCLUSPROP_ULARGE_INTEGER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSPROP_ULARGE_INTEGER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSPROP_VALUE(pub *mut CLUSPROP_VALUE);
impl PCLUSPROP_VALUE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSPROP_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSPROP_WORD(pub *mut CLUSPROP_WORD);
impl PCLUSPROP_WORD {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSPROP_WORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSTERVERSIONINFO(pub *mut CLUSTERVERSIONINFO);
impl PCLUSTERVERSIONINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSTERVERSIONINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSTERVERSIONINFO_NT4(pub *mut CLUSTERVERSIONINFO_NT4);
impl PCLUSTERVERSIONINFO_NT4 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSTERVERSIONINFO_NT4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSTER_AVAILABILITY_SET_CONFIG(pub *mut CLUSTER_AVAILABILITY_SET_CONFIG);
impl PCLUSTER_AVAILABILITY_SET_CONFIG {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSTER_AVAILABILITY_SET_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSTER_CLOUD_TYPE(pub *mut CLUSTER_CLOUD_TYPE);
impl PCLUSTER_CLOUD_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSTER_CLOUD_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSTER_CREATE_GROUP_INFO(pub *mut CLUSTER_CREATE_GROUP_INFO);
impl PCLUSTER_CREATE_GROUP_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSTER_CREATE_GROUP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSTER_CSV_VOLUME_FAULT_STATE(pub *mut CLUSTER_CSV_VOLUME_FAULT_STATE);
impl PCLUSTER_CSV_VOLUME_FAULT_STATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSTER_CSV_VOLUME_FAULT_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSTER_ENUM_ITEM(pub *mut CLUSTER_ENUM_ITEM);
impl PCLUSTER_ENUM_ITEM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSTER_ENUM_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSTER_GROUP_ENUM_ITEM(pub *mut CLUSTER_GROUP_ENUM_ITEM);
impl PCLUSTER_GROUP_ENUM_ITEM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSTER_GROUP_ENUM_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSTER_IP_ENTRY(pub *mut CLUSTER_IP_ENTRY);
impl PCLUSTER_IP_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSTER_IP_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSTER_MEMBERSHIP_INFO(pub *mut CLUSTER_MEMBERSHIP_INFO);
impl PCLUSTER_MEMBERSHIP_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSTER_MEMBERSHIP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSTER_MGMT_POINT_RESTYPE(pub *mut CLUSTER_MGMT_POINT_RESTYPE);
impl PCLUSTER_MGMT_POINT_RESTYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSTER_MGMT_POINT_RESTYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PCLUSTER_REG_BATCH_ADD_COMMAND = Option<unsafe extern "system" fn(hregbatch: *const _HREGBATCH, dwcommand: CLUSTER_REG_COMMAND, wzname: windows_core::PCWSTR, dwoptions: u32, lpdata: *const core::ffi::c_void, cbdata: u32) -> i32>;
pub type PCLUSTER_REG_BATCH_CLOSE_NOTIFICATION = Option<unsafe extern "system" fn(hbatchnotification: *const _HREGBATCHNOTIFICATION) -> i32>;
pub type PCLUSTER_REG_BATCH_READ_COMMAND = Option<unsafe extern "system" fn(hbatchnotification: *const _HREGBATCHNOTIFICATION, pbatchcommand: *mut CLUSTER_BATCH_COMMAND) -> i32>;
pub type PCLUSTER_REG_CLOSE_BATCH = Option<unsafe extern "system" fn(hregbatch: *const _HREGBATCH, bcommit: windows_core::BOOL, failedcommandnumber: *mut i32) -> i32>;
pub type PCLUSTER_REG_CLOSE_BATCH_NOTIFY_PORT = Option<unsafe extern "system" fn(hbatchnotifyport: *const _HREGBATCHPORT) -> i32>;
pub type PCLUSTER_REG_CLOSE_READ_BATCH = Option<unsafe extern "system" fn(hregreadbatch: *const _HREGREADBATCH, phregreadbatchreply: *mut HREGREADBATCHREPLY) -> i32>;
pub type PCLUSTER_REG_CLOSE_READ_BATCH_EX = Option<unsafe extern "system" fn(hregreadbatch: *const _HREGREADBATCH, flags: u32, phregreadbatchreply: *mut HREGREADBATCHREPLY) -> i32>;
pub type PCLUSTER_REG_CLOSE_READ_BATCH_REPLY = Option<unsafe extern "system" fn(hregreadbatchreply: *const _HREGREADBATCHREPLY) -> i32>;
#[cfg(feature = "Win32_minwindef")]
pub type PCLUSTER_REG_CREATE_BATCH_NOTIFY_PORT = Option<unsafe extern "system" fn(hkey: super::minwindef::HKEY, phbatchnotifyport: *mut HREGBATCHPORT) -> i32>;
#[cfg(feature = "Win32_minwindef")]
pub type PCLUSTER_REG_CREATE_READ_BATCH = Option<unsafe extern "system" fn(hkey: super::minwindef::HKEY, phregreadbatch: *mut HREGREADBATCH) -> i32>;
pub type PCLUSTER_REG_GET_BATCH_NOTIFICATION = Option<unsafe extern "system" fn(hbatchnotify: *const _HREGBATCHPORT, phbatchnotification: *mut HREGBATCHNOTIFICATION) -> i32>;
pub type PCLUSTER_REG_READ_BATCH_ADD_COMMAND = Option<unsafe extern "system" fn(hregreadbatch: *const _HREGREADBATCH, wzsubkeyname: windows_core::PCWSTR, wzvaluename: windows_core::PCWSTR) -> i32>;
pub type PCLUSTER_REG_READ_BATCH_REPLY_NEXT_COMMAND = Option<unsafe extern "system" fn(hregreadbatchreply: *const _HREGREADBATCHREPLY, pbatchcommand: *mut CLUSTER_READ_BATCH_COMMAND) -> i32>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSTER_RESOURCE_ENUM_ITEM(pub *mut CLUSTER_RESOURCE_ENUM_ITEM);
impl PCLUSTER_RESOURCE_ENUM_ITEM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSTER_RESOURCE_ENUM_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PCLUSTER_SETUP_PROGRESS_CALLBACK = Option<unsafe extern "system" fn(pvcallbackarg: *mut core::ffi::c_void, esetupphase: CLUSTER_SETUP_PHASE, ephasetype: CLUSTER_SETUP_PHASE_TYPE, ephaseseverity: CLUSTER_SETUP_PHASE_SEVERITY, dwpercentcomplete: u32, lpszobjectname: windows_core::PCWSTR, dwstatus: u32) -> windows_core::BOOL>;
pub type PCLUSTER_SET_ACCOUNT_ACCESS = Option<unsafe extern "system" fn(hcluster: *const _HCLUSTER, szaccountsid: windows_core::PCWSTR, dwaccess: u32, dwcontroltype: u32) -> u32>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSTER_SET_PASSWORD_STATUS(pub *mut CLUSTER_SET_PASSWORD_STATUS);
impl PCLUSTER_SET_PASSWORD_STATUS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSTER_SET_PASSWORD_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSTER_SHARED_VOLUME_BACKUP_STATE(pub *mut CLUSTER_SHARED_VOLUME_BACKUP_STATE);
impl PCLUSTER_SHARED_VOLUME_BACKUP_STATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSTER_SHARED_VOLUME_BACKUP_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSTER_SHARED_VOLUME_RENAME_GUID_INPUT(pub *mut CLUSTER_SHARED_VOLUME_RENAME_GUID_INPUT);
impl PCLUSTER_SHARED_VOLUME_RENAME_GUID_INPUT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSTER_SHARED_VOLUME_RENAME_GUID_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSTER_SHARED_VOLUME_RENAME_INPUT(pub *mut CLUSTER_SHARED_VOLUME_RENAME_INPUT);
impl PCLUSTER_SHARED_VOLUME_RENAME_INPUT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSTER_SHARED_VOLUME_RENAME_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME(pub *mut CLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME);
impl PCLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSTER_SHARED_VOLUME_RENAME_INPUT_GUID_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME(pub *mut CLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME);
impl PCLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSTER_SHARED_VOLUME_RENAME_INPUT_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE(pub *mut CLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE);
impl PCLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSTER_SHARED_VOLUME_RENAME_INPUT_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME(pub *mut CLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME);
impl PCLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSTER_SHARED_VOLUME_RENAME_INPUT_VOLUME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSTER_SHARED_VOLUME_STATE(pub *mut CLUSTER_SHARED_VOLUME_STATE);
impl PCLUSTER_SHARED_VOLUME_STATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSTER_SHARED_VOLUME_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSTER_SHARED_VOLUME_STATE_INFO(pub *mut CLUSTER_SHARED_VOLUME_STATE_INFO);
impl PCLUSTER_SHARED_VOLUME_STATE_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSTER_SHARED_VOLUME_STATE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSTER_SHARED_VOLUME_STATE_INFO_EX(pub *mut CLUSTER_SHARED_VOLUME_STATE_INFO_EX);
impl PCLUSTER_SHARED_VOLUME_STATE_INFO_EX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSTER_SHARED_VOLUME_STATE_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PCLUSTER_UPGRADE_PROGRESS_CALLBACK = Option<unsafe extern "system" fn(pvcallbackarg: *mut core::ffi::c_void, eupgradephase: CLUSTER_UPGRADE_PHASE) -> windows_core::BOOL>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSTER_VALIDATE_CSV_FILENAME(pub *mut CLUSTER_VALIDATE_CSV_FILENAME);
impl PCLUSTER_VALIDATE_CSV_FILENAME {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSTER_VALIDATE_CSV_FILENAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSTER_VALIDATE_DIRECTORY(pub *mut CLUSTER_VALIDATE_DIRECTORY);
impl PCLUSTER_VALIDATE_DIRECTORY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSTER_VALIDATE_DIRECTORY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSTER_VALIDATE_NETNAME(pub *mut CLUSTER_VALIDATE_NETNAME);
impl PCLUSTER_VALIDATE_NETNAME {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSTER_VALIDATE_NETNAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUSTER_VALIDATE_PATH(pub *mut CLUSTER_VALIDATE_PATH);
impl PCLUSTER_VALIDATE_PATH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUSTER_VALIDATE_PATH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_CHKDSK_INFO(pub *mut CLUS_CHKDSK_INFO);
impl PCLUS_CHKDSK_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_CHKDSK_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT(pub *mut CLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT);
impl PCLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_CREATE_INFRASTRUCTURE_FILESERVER_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT(pub *mut CLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT);
impl PCLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_CREATE_INFRASTRUCTURE_FILESERVER_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_CSV_MAINTENANCE_MODE_INFO(pub *mut CLUS_CSV_MAINTENANCE_MODE_INFO);
impl PCLUS_CSV_MAINTENANCE_MODE_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_CSV_MAINTENANCE_MODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_CSV_VOLUME_INFO(pub *mut CLUS_CSV_VOLUME_INFO);
impl PCLUS_CSV_VOLUME_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_CSV_VOLUME_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_CSV_VOLUME_NAME(pub *mut CLUS_CSV_VOLUME_NAME);
impl PCLUS_CSV_VOLUME_NAME {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_CSV_VOLUME_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_DISK_NUMBER_INFO(pub *mut CLUS_DISK_NUMBER_INFO);
impl PCLUS_DISK_NUMBER_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_DISK_NUMBER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_DNN_LEADER_STATUS(pub *mut CLUS_DNN_LEADER_STATUS);
impl PCLUS_DNN_LEADER_STATUS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_DNN_LEADER_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_DNN_SODAFS_CLONE_STATUS(pub *mut CLUS_DNN_SODAFS_CLONE_STATUS);
impl PCLUS_DNN_SODAFS_CLONE_STATUS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_DNN_SODAFS_CLONE_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_FORCE_QUORUM_INFO(pub *mut CLUS_FORCE_QUORUM_INFO);
impl PCLUS_FORCE_QUORUM_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_FORCE_QUORUM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_FTSET_INFO(pub *mut CLUS_FTSET_INFO);
impl PCLUS_FTSET_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_FTSET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_MAINTENANCE_MODE_INFO(pub *mut CLUS_MAINTENANCE_MODE_INFO);
impl PCLUS_MAINTENANCE_MODE_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_MAINTENANCE_MODE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_MAINTENANCE_MODE_INFOEX(pub *mut CLUS_MAINTENANCE_MODE_INFOEX);
impl PCLUS_MAINTENANCE_MODE_INFOEX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_MAINTENANCE_MODE_INFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_NETNAME_IP_INFO_ENTRY(pub *mut CLUS_NETNAME_IP_INFO_ENTRY);
impl PCLUS_NETNAME_IP_INFO_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_NETNAME_IP_INFO_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL(pub *mut CLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL);
impl PCLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_NETNAME_IP_INFO_FOR_MULTICHANNEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_NETNAME_PWD_INFO(pub *mut CLUS_RLUA_PWD_INFO);
impl PCLUS_NETNAME_PWD_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_NETNAME_PWD_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_NETNAME_PWD_INFOEX(pub *mut CLUS_RLUA_PWD_INFOEX);
impl PCLUS_NETNAME_PWD_INFOEX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_NETNAME_PWD_INFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_NETNAME_VS_TOKEN_INFO(pub *mut CLUS_VS_TOKEN_INFO);
impl PCLUS_NETNAME_VS_TOKEN_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_NETNAME_VS_TOKEN_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_PARTITION_INFO(pub *mut CLUS_PARTITION_INFO);
impl PCLUS_PARTITION_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_PARTITION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_PARTITION_INFO_EX(pub *mut CLUS_PARTITION_INFO_EX);
impl PCLUS_PARTITION_INFO_EX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_PARTITION_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_PARTITION_INFO_EX2(pub *mut CLUS_PARTITION_INFO_EX2);
impl PCLUS_PARTITION_INFO_EX2 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_PARTITION_INFO_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_PROVIDER_STATE_CHANGE_INFO(pub *mut CLUS_PROVIDER_STATE_CHANGE_INFO);
impl PCLUS_PROVIDER_STATE_CHANGE_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_PROVIDER_STATE_CHANGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_RESOURCE_CLASS_INFO(pub *mut CLUS_RESOURCE_CLASS_INFO);
impl PCLUS_RESOURCE_CLASS_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_RESOURCE_CLASS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_RLUA_PWD_INFO(pub *mut CLUS_RLUA_PWD_INFO);
impl PCLUS_RLUA_PWD_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_RLUA_PWD_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_RLUA_PWD_INFOEX(pub *mut CLUS_RLUA_PWD_INFOEX);
impl PCLUS_RLUA_PWD_INFOEX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_RLUA_PWD_INFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_SCSI_ADDRESS(pub *mut CLUS_SCSI_ADDRESS);
impl PCLUS_SCSI_ADDRESS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_SCSI_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_SET_MAINTENANCE_MODE_INPUT(pub *mut CLUS_SET_MAINTENANCE_MODE_INPUT);
impl PCLUS_SET_MAINTENANCE_MODE_INPUT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_SET_MAINTENANCE_MODE_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_SHARED_VOLUME_BACKUP_MODE(pub *mut CLUS_SHARED_VOLUME_BACKUP_MODE);
impl PCLUS_SHARED_VOLUME_BACKUP_MODE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_SHARED_VOLUME_BACKUP_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_STARTING_PARAMS(pub *mut CLUS_STARTING_PARAMS);
impl PCLUS_STARTING_PARAMS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_STARTING_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS(pub *mut CLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS);
impl PCLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_STORAGE_GET_AVAILABLE_DRIVELETTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_STORAGE_REMAP_DRIVELETTER(pub *mut CLUS_STORAGE_REMAP_DRIVELETTER);
impl PCLUS_STORAGE_REMAP_DRIVELETTER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_STORAGE_REMAP_DRIVELETTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_STORAGE_SET_DRIVELETTER(pub *mut CLUS_STORAGE_SET_DRIVELETTER);
impl PCLUS_STORAGE_SET_DRIVELETTER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_STORAGE_SET_DRIVELETTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLUS_VS_TOKEN_INFO(pub *mut CLUS_VS_TOKEN_INFO);
impl PCLUS_VS_TOKEN_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLUS_VS_TOKEN_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCREATE_CLUSTER_CONFIG(pub *mut CREATE_CLUSTER_CONFIG);
impl PCREATE_CLUSTER_CONFIG {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCREATE_CLUSTER_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCREATE_CLUSTER_NAME_ACCOUNT(pub *mut CREATE_CLUSTER_NAME_ACCOUNT);
impl PCREATE_CLUSTER_NAME_ACCOUNT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCREATE_CLUSTER_NAME_ACCOUNT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFILESHARE_CHANGE(pub *mut FILESHARE_CHANGE);
impl PFILESHARE_CHANGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFILESHARE_CHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFILESHARE_CHANGE_ENUM(pub *mut FILESHARE_CHANGE_ENUM);
impl PFILESHARE_CHANGE_ENUM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFILESHARE_CHANGE_ENUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFILESHARE_CHANGE_LIST(pub *mut FILESHARE_CHANGE_LIST);
impl PFILESHARE_CHANGE_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFILESHARE_CHANGE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PGROUP_FAILURE_INFO(pub *mut GROUP_FAILURE_INFO);
impl PGROUP_FAILURE_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PGROUP_FAILURE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PGROUP_FAILURE_INFO_BUFFER(pub *mut GROUP_FAILURE_INFO_BUFFER);
impl PGROUP_FAILURE_INFO_BUFFER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PGROUP_FAILURE_INFO_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PLACEMENT_OPTIONS = i32;
pub const PLACEMENT_OPTIONS_ALL: PLACEMENT_OPTIONS = 1023;
pub const PLACEMENT_OPTIONS_AVAILABILITY_SET_DOMAIN_AFFINITY: PLACEMENT_OPTIONS = 512;
pub const PLACEMENT_OPTIONS_CONSIDER_OFFLINE_VMS: PLACEMENT_OPTIONS = 2;
pub const PLACEMENT_OPTIONS_DEFAULT_PLACEMENT_OPTIONS: PLACEMENT_OPTIONS = 0;
pub const PLACEMENT_OPTIONS_DISABLE_CSV_VM_DEPENDENCY: PLACEMENT_OPTIONS = 1;
pub const PLACEMENT_OPTIONS_DONT_RESUME_AVAILABILTY_SET_VMS_WITH_EXISTING_TEMP_DISK: PLACEMENT_OPTIONS = 128;
pub const PLACEMENT_OPTIONS_DONT_RESUME_VMS_WITH_EXISTING_TEMP_DISK: PLACEMENT_OPTIONS = 32;
pub const PLACEMENT_OPTIONS_DONT_USE_CPU: PLACEMENT_OPTIONS = 8;
pub const PLACEMENT_OPTIONS_DONT_USE_LOCAL_TEMP_DISK: PLACEMENT_OPTIONS = 16;
pub const PLACEMENT_OPTIONS_DONT_USE_MEMORY: PLACEMENT_OPTIONS = 4;
pub const PLACEMENT_OPTIONS_MIN_VALUE: PLACEMENT_OPTIONS = 0;
pub const PLACEMENT_OPTIONS_SAVE_AVAILABILTY_SET_VMS_WITH_LOCAL_DISK_ON_DRAIN_OVERWRITE: PLACEMENT_OPTIONS = 256;
pub const PLACEMENT_OPTIONS_SAVE_VMS_WITH_LOCAL_DISK_ON_DRAIN_OVERWRITE: PLACEMENT_OPTIONS = 64;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMAINTENANCE_MODE_TYPE_ENUM(pub *mut MAINTENANCE_MODE_TYPE_ENUM);
impl PMAINTENANCE_MODE_TYPE_ENUM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMAINTENANCE_MODE_TYPE_ENUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNOTIFY_FILTER_AND_TYPE(pub *mut NOTIFY_FILTER_AND_TYPE);
impl PNOTIFY_FILTER_AND_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNOTIFY_FILTER_AND_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PREPAIR_CLUSTER_NAME_ACCOUNT_CONFIG(pub *mut REPAIR_CLUSTER_NAME_ACCOUNT_CONFIG);
impl PREPAIR_CLUSTER_NAME_ACCOUNT_CONFIG {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PREPAIR_CLUSTER_NAME_ACCOUNT_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRESOURCE_FAILURE_INFO(pub *mut RESOURCE_FAILURE_INFO);
impl PRESOURCE_FAILURE_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRESOURCE_FAILURE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRESOURCE_FAILURE_INFO_BUFFER(pub *mut RESOURCE_FAILURE_INFO_BUFFER);
impl PRESOURCE_FAILURE_INFO_BUFFER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRESOURCE_FAILURE_INFO_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRESOURCE_TERMINAL_FAILURE_INFO_BUFFER(pub *mut RESOURCE_TERMINAL_FAILURE_INFO_BUFFER);
impl PRESOURCE_TERMINAL_FAILURE_INFO_BUFFER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRESOURCE_TERMINAL_FAILURE_INFO_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSR_DISK_REPLICATION_ELIGIBLE(pub *mut SR_DISK_REPLICATION_ELIGIBLE);
impl PSR_DISK_REPLICATION_ELIGIBLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSR_DISK_REPLICATION_ELIGIBLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSR_REPLICATED_DISK_TYPE(pub *mut SR_REPLICATED_DISK_TYPE);
impl PSR_REPLICATED_DISK_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSR_REPLICATED_DISK_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSR_RESOURCE_TYPE_ADD_REPLICATION_GROUP(pub *mut SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP);
impl PSR_RESOURCE_TYPE_ADD_REPLICATION_GROUP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSR_RESOURCE_TYPE_ADD_REPLICATION_GROUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT(pub *mut SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT);
impl PSR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSR_RESOURCE_TYPE_DISK_INFO(pub *mut SR_RESOURCE_TYPE_DISK_INFO);
impl PSR_RESOURCE_TYPE_DISK_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSR_RESOURCE_TYPE_DISK_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT(pub *mut SR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT);
impl PSR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS(pub *mut SR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS);
impl PSR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSR_RESOURCE_TYPE_QUERY_ELIGIBLE_SOURCE_DATADISKS(pub *mut SR_RESOURCE_TYPE_QUERY_ELIGIBLE_SOURCE_DATADISKS);
impl PSR_RESOURCE_TYPE_QUERY_ELIGIBLE_SOURCE_DATADISKS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSR_RESOURCE_TYPE_QUERY_ELIGIBLE_SOURCE_DATADISKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSR_RESOURCE_TYPE_QUERY_ELIGIBLE_TARGET_DATADISKS(pub *mut SR_RESOURCE_TYPE_QUERY_ELIGIBLE_TARGET_DATADISKS);
impl PSR_RESOURCE_TYPE_QUERY_ELIGIBLE_TARGET_DATADISKS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSR_RESOURCE_TYPE_QUERY_ELIGIBLE_TARGET_DATADISKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSR_RESOURCE_TYPE_REPLICATED_DISK(pub *mut SR_RESOURCE_TYPE_REPLICATED_DISK);
impl PSR_RESOURCE_TYPE_REPLICATED_DISK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSR_RESOURCE_TYPE_REPLICATED_DISK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT(pub *mut SR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT);
impl PSR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY(pub *mut SR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY);
impl PSR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO(pub *mut SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO);
impl PSR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PriorityDisabled: CLUSTER_GROUP_PRIORITY = 0;
pub const PriorityHigh: CLUSTER_GROUP_PRIORITY = 3000;
pub const PriorityLow: CLUSTER_GROUP_PRIORITY = 1000;
pub const PriorityMedium: CLUSTER_GROUP_PRIORITY = 2000;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REPAIR_CLUSTER_NAME_ACCOUNT_CONFIG {
    pub dwVersion: u32,
    pub dwFlags: u32,
    pub pszUserName: windows_core::PCWSTR,
    pub pszPassword: windows_core::PCWSTR,
    pub pszDomain: windows_core::PCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RESOURCE_FAILURE_INFO {
    pub dwRestartAttemptsRemaining: u32,
    pub dwRestartPeriodRemaining: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RESOURCE_FAILURE_INFO_BUFFER {
    pub dwVersion: u32,
    pub Info: RESOURCE_FAILURE_INFO,
}
pub const RESOURCE_FAILURE_INFO_VERSION_1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RESOURCE_TERMINAL_FAILURE_INFO_BUFFER {
    pub isTerminalFailure: windows_core::BOOL,
    pub restartPeriodRemaining: u32,
}
pub const RS3_UPGRADE_VERSION: u32 = 1;
pub const RS4_UPGRADE_VERSION: u32 = 2;
pub const RS5_UPGRADE_VERSION: u32 = 3;
pub const RedirectedIOReasonBitLockerInitializing: u32 = 16;
pub const RedirectedIOReasonFileSystemTiering: u32 = 8;
pub const RedirectedIOReasonMax: u64 = 9223372036854775808;
pub const RedirectedIOReasonReFs: u32 = 32;
pub const RedirectedIOReasonUnsafeFileSystemFilter: u32 = 2;
pub const RedirectedIOReasonUnsafeVolumeFilter: u32 = 4;
pub const RedirectedIOReasonUserRequest: u32 = 1;
pub const SE_UPGRADE_VERSION: u32 = 2;
pub type SR_DISK_REPLICATION_ELIGIBLE = i32;
pub type SR_REPLICATED_DISK_TYPE = i32;
pub const SR_REPLICATED_PARTITION_DISALLOW_MULTINODE_IO: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP {
    pub ReplicationGroupName: [u16; 260],
    pub Description: [u16; 260],
    pub LogPath: [u16; 260],
    pub MaxLogSizeInBytes: u64,
    pub LogType: u16,
    pub ReplicationMode: u32,
    pub MinimumPartnersInSync: u32,
    pub EnableWriteConsistency: bool,
    pub EnableEncryption: bool,
    pub EnableCompression: bool,
    pub CertificateThumbprint: [u16; 260],
    pub VolumeNameCount: u32,
    pub VolumeNames: [[u16; 260]; 1],
}
impl Default for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT {
    pub Result: u32,
    pub ErrorString: [u16; 260],
}
impl Default for SR_RESOURCE_TYPE_ADD_REPLICATION_GROUP_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SR_RESOURCE_TYPE_DISK_INFO {
    pub Reason: SR_DISK_REPLICATION_ELIGIBLE,
    pub DiskGuid: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT {
    pub Count: u16,
    pub DiskInfo: [SR_RESOURCE_TYPE_DISK_INFO; 1],
}
impl Default for SR_RESOURCE_TYPE_ELIGIBLE_DISKS_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SR_RESOURCE_TYPE_QUERY_ELIGIBLE_LOGDISKS {
    pub DataDiskGuid: windows_core::GUID,
    pub IncludeOfflineDisks: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SR_RESOURCE_TYPE_QUERY_ELIGIBLE_SOURCE_DATADISKS {
    pub DataDiskGuid: windows_core::GUID,
    pub IncludeAvailableStoargeDisks: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SR_RESOURCE_TYPE_QUERY_ELIGIBLE_TARGET_DATADISKS {
    pub SourceDataDiskGuid: windows_core::GUID,
    pub TargetReplicationGroupGuid: windows_core::GUID,
    pub SkipConnectivityCheck: bool,
    pub IncludeOfflineDisks: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SR_RESOURCE_TYPE_REPLICATED_DISK {
    pub Type: SR_REPLICATED_DISK_TYPE,
    pub ClusterDiskResourceGuid: windows_core::GUID,
    pub ReplicationGroupId: windows_core::GUID,
    pub ReplicationGroupName: [u16; 260],
}
impl Default for SR_RESOURCE_TYPE_REPLICATED_DISK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT {
    pub Count: u16,
    pub ReplicatedDisks: [SR_RESOURCE_TYPE_REPLICATED_DISK; 1],
}
impl Default for SR_RESOURCE_TYPE_REPLICATED_DISKS_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY {
    pub Count: u32,
    pub PartitionArray: [SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO; 1],
}
impl Default for SR_RESOURCE_TYPE_REPLICATED_PARTITION_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SR_RESOURCE_TYPE_REPLICATED_PARTITION_INFO {
    pub PartitionOffset: u64,
    pub Capabilities: u32,
}
pub const SharedVolumeStateActive: CLUSTER_SHARED_VOLUME_STATE = 2;
pub const SharedVolumeStateActiveRedirected: CLUSTER_SHARED_VOLUME_STATE = 3;
pub const SharedVolumeStateActiveVolumeRedirected: CLUSTER_SHARED_VOLUME_STATE = 4;
pub const SharedVolumeStatePaused: CLUSTER_SHARED_VOLUME_STATE = 1;
pub const SharedVolumeStateUnavailable: CLUSTER_SHARED_VOLUME_STATE = 0;
pub const SrDiskReplicationEligibleAlreadyInReplication: SR_DISK_REPLICATION_ELIGIBLE = 9;
pub const SrDiskReplicationEligibleFileSystemNotSupported: SR_DISK_REPLICATION_ELIGIBLE = 8;
pub const SrDiskReplicationEligibleInSameSite: SR_DISK_REPLICATION_ELIGIBLE = 7;
pub const SrDiskReplicationEligibleInsufficientFreeSpace: SR_DISK_REPLICATION_ELIGIBLE = 5;
pub const SrDiskReplicationEligibleNone: SR_DISK_REPLICATION_ELIGIBLE = 0;
pub const SrDiskReplicationEligibleNotGpt: SR_DISK_REPLICATION_ELIGIBLE = 3;
pub const SrDiskReplicationEligibleNotInSameSite: SR_DISK_REPLICATION_ELIGIBLE = 6;
pub const SrDiskReplicationEligibleOffline: SR_DISK_REPLICATION_ELIGIBLE = 2;
pub const SrDiskReplicationEligibleOther: SR_DISK_REPLICATION_ELIGIBLE = 9999;
pub const SrDiskReplicationEligiblePartitionLayoutMismatch: SR_DISK_REPLICATION_ELIGIBLE = 4;
pub const SrDiskReplicationEligibleSameAsSpecifiedDisk: SR_DISK_REPLICATION_ELIGIBLE = 10;
pub const SrDiskReplicationEligibleYes: SR_DISK_REPLICATION_ELIGIBLE = 1;
pub const SrReplicatedDiskTypeDestination: SR_REPLICATED_DISK_TYPE = 3;
pub const SrReplicatedDiskTypeLogDestination: SR_REPLICATED_DISK_TYPE = 4;
pub const SrReplicatedDiskTypeLogNotInParthership: SR_REPLICATED_DISK_TYPE = 6;
pub const SrReplicatedDiskTypeLogSource: SR_REPLICATED_DISK_TYPE = 2;
pub const SrReplicatedDiskTypeNone: SR_REPLICATED_DISK_TYPE = 0;
pub const SrReplicatedDiskTypeNotInParthership: SR_REPLICATED_DISK_TYPE = 5;
pub const SrReplicatedDiskTypeOther: SR_REPLICATED_DISK_TYPE = 7;
pub const SrReplicatedDiskTypeSource: SR_REPLICATED_DISK_TYPE = 1;
pub const USE_CLIENT_ACCESS_NETWORKS_FOR_CSV: windows_core::PCWSTR = windows_core::w!("UseClientAccessNetworksForSharedVolumes");
pub const VolumeBackupInProgress: CLUSTER_SHARED_VOLUME_BACKUP_STATE = 1;
pub const VolumeBackupNone: CLUSTER_SHARED_VOLUME_BACKUP_STATE = 0;
pub const VolumeRedirectedIOReasonMax: u64 = 9223372036854775808;
pub const VolumeRedirectedIOReasonNoDiskConnectivity: u32 = 1;
pub const VolumeRedirectedIOReasonStorageSpaceNotAttached: u32 = 2;
pub const VolumeRedirectedIOReasonVolumeReplicationEnabled: u32 = 4;
pub const VolumeStateDismounted: CLUSTER_CSV_VOLUME_FAULT_STATE = 8;
pub const VolumeStateInMaintenance: CLUSTER_CSV_VOLUME_FAULT_STATE = 4;
pub const VolumeStateNoAccess: CLUSTER_CSV_VOLUME_FAULT_STATE = 2;
pub const VolumeStateNoDirectIO: CLUSTER_CSV_VOLUME_FAULT_STATE = 1;
pub const VolumeStateNoFaults: CLUSTER_CSV_VOLUME_FAULT_STATE = 0;
pub const WS2016_RTM_UPGRADE_VERSION: u32 = 8;
pub const WS2016_TP4_UPGRADE_VERSION: u32 = 6;
pub const WS2016_TP5_UPGRADE_VERSION: u32 = 7;
pub const ZN_UPGRADE_VERSION: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HCHANGE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HCLUSENUM(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HCLUSENUMEX(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HCLUSTER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HGROUP(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HGROUPENUM(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HGROUPENUMEX(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HGROUPSET(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HGROUPSETENUM(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HKEYVALUEBATCH(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HKEYVALUEBATCHNOTIFICATION(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HKEYVALUEREADBATCH(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HKEYVALUEREADBATCHREPLY(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HKEYVALUESTORE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HNETINTERFACE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HNETINTERFACEENUM(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HNETWORK(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HNETWORKENUM(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HNODE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HNODEENUM(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HNODEENUMEX(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HREGBATCH(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HREGBATCHNOTIFICATION(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HREGBATCHPORT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HREGREADBATCH(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HREGREADBATCHREPLY(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HRESENUM(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HRESENUMEX(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HRESOURCE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _HRESTYPEENUM(pub u8);
pub const eResourceStateChangeReasonFailedMove: CLUSTER_RESOURCE_STATE_CHANGE_REASON = 3;
pub const eResourceStateChangeReasonFailover: CLUSTER_RESOURCE_STATE_CHANGE_REASON = 2;
pub const eResourceStateChangeReasonMove: CLUSTER_RESOURCE_STATE_CHANGE_REASON = 1;
pub const eResourceStateChangeReasonRundown: CLUSTER_RESOURCE_STATE_CHANGE_REASON = 5;
pub const eResourceStateChangeReasonShutdown: CLUSTER_RESOURCE_STATE_CHANGE_REASON = 4;
pub const eResourceStateChangeReasonUnknown: CLUSTER_RESOURCE_STATE_CHANGE_REASON = 0;
