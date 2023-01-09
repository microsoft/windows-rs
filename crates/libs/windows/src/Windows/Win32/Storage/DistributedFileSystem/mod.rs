#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsAdd<P0, P1, P2, P3>(dfsentrypath: P0, servername: P1, sharename: P2, comment: P3, flags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn NetDfsAdd ( dfsentrypath : :: windows::core::PCWSTR , servername : :: windows::core::PCWSTR , sharename : :: windows::core::PCWSTR , comment : :: windows::core::PCWSTR , flags : u32 ) -> u32 );
    NetDfsAdd(dfsentrypath.into().abi(), servername.into().abi(), sharename.into().abi(), comment.into().abi(), flags)
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsAddFtRoot<P0, P1, P2, P3>(servername: P0, rootshare: P1, ftdfsname: P2, comment: P3, flags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn NetDfsAddFtRoot ( servername : :: windows::core::PCWSTR , rootshare : :: windows::core::PCWSTR , ftdfsname : :: windows::core::PCWSTR , comment : :: windows::core::PCWSTR , flags : u32 ) -> u32 );
    NetDfsAddFtRoot(servername.into().abi(), rootshare.into().abi(), ftdfsname.into().abi(), comment.into().abi(), flags)
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsAddRootTarget<P0, P1, P2>(pdfspath: P0, ptargetpath: P1, majorversion: u32, pcomment: P2, flags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn NetDfsAddRootTarget ( pdfspath : :: windows::core::PCWSTR , ptargetpath : :: windows::core::PCWSTR , majorversion : u32 , pcomment : :: windows::core::PCWSTR , flags : u32 ) -> u32 );
    NetDfsAddRootTarget(pdfspath.into().abi(), ptargetpath.into().abi(), majorversion, pcomment.into().abi(), flags)
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsAddStdRoot<P0, P1, P2>(servername: P0, rootshare: P1, comment: P2, flags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn NetDfsAddStdRoot ( servername : :: windows::core::PCWSTR , rootshare : :: windows::core::PCWSTR , comment : :: windows::core::PCWSTR , flags : u32 ) -> u32 );
    NetDfsAddStdRoot(servername.into().abi(), rootshare.into().abi(), comment.into().abi(), flags)
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsEnum<P0>(dfsname: P0, level: u32, prefmaxlen: u32, buffer: *mut *mut u8, entriesread: *mut u32, resumehandle: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn NetDfsEnum ( dfsname : :: windows::core::PCWSTR , level : u32 , prefmaxlen : u32 , buffer : *mut *mut u8 , entriesread : *mut u32 , resumehandle : *mut u32 ) -> u32 );
    NetDfsEnum(dfsname.into().abi(), level, prefmaxlen, buffer, entriesread, resumehandle)
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsGetClientInfo<P0, P1, P2>(dfsentrypath: P0, servername: P1, sharename: P2, level: u32, buffer: *mut *mut u8) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn NetDfsGetClientInfo ( dfsentrypath : :: windows::core::PCWSTR , servername : :: windows::core::PCWSTR , sharename : :: windows::core::PCWSTR , level : u32 , buffer : *mut *mut u8 ) -> u32 );
    NetDfsGetClientInfo(dfsentrypath.into().abi(), servername.into().abi(), sharename.into().abi(), level, buffer)
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetDfsGetFtContainerSecurity<P0>(domainname: P0, securityinformation: u32, ppsecuritydescriptor: *mut super::super::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn NetDfsGetFtContainerSecurity ( domainname : :: windows::core::PCWSTR , securityinformation : u32 , ppsecuritydescriptor : *mut super::super::Security:: PSECURITY_DESCRIPTOR , lpcbsecuritydescriptor : *mut u32 ) -> u32 );
    NetDfsGetFtContainerSecurity(domainname.into().abi(), securityinformation, ppsecuritydescriptor, lpcbsecuritydescriptor)
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsGetInfo<P0, P1, P2>(dfsentrypath: P0, servername: P1, sharename: P2, level: u32, buffer: *mut *mut u8) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn NetDfsGetInfo ( dfsentrypath : :: windows::core::PCWSTR , servername : :: windows::core::PCWSTR , sharename : :: windows::core::PCWSTR , level : u32 , buffer : *mut *mut u8 ) -> u32 );
    NetDfsGetInfo(dfsentrypath.into().abi(), servername.into().abi(), sharename.into().abi(), level, buffer)
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetDfsGetSecurity<P0>(dfsentrypath: P0, securityinformation: u32, ppsecuritydescriptor: *mut super::super::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn NetDfsGetSecurity ( dfsentrypath : :: windows::core::PCWSTR , securityinformation : u32 , ppsecuritydescriptor : *mut super::super::Security:: PSECURITY_DESCRIPTOR , lpcbsecuritydescriptor : *mut u32 ) -> u32 );
    NetDfsGetSecurity(dfsentrypath.into().abi(), securityinformation, ppsecuritydescriptor, lpcbsecuritydescriptor)
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetDfsGetStdContainerSecurity<P0>(machinename: P0, securityinformation: u32, ppsecuritydescriptor: *mut super::super::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn NetDfsGetStdContainerSecurity ( machinename : :: windows::core::PCWSTR , securityinformation : u32 , ppsecuritydescriptor : *mut super::super::Security:: PSECURITY_DESCRIPTOR , lpcbsecuritydescriptor : *mut u32 ) -> u32 );
    NetDfsGetStdContainerSecurity(machinename.into().abi(), securityinformation, ppsecuritydescriptor, lpcbsecuritydescriptor)
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsGetSupportedNamespaceVersion<P0>(origin: DFS_NAMESPACE_VERSION_ORIGIN, pname: P0, ppversioninfo: *mut *mut DFS_SUPPORTED_NAMESPACE_VERSION_INFO) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn NetDfsGetSupportedNamespaceVersion ( origin : DFS_NAMESPACE_VERSION_ORIGIN , pname : :: windows::core::PCWSTR , ppversioninfo : *mut *mut DFS_SUPPORTED_NAMESPACE_VERSION_INFO ) -> u32 );
    NetDfsGetSupportedNamespaceVersion(origin, pname.into().abi(), ppversioninfo)
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsMove<P0, P1>(olddfsentrypath: P0, newdfsentrypath: P1, flags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn NetDfsMove ( olddfsentrypath : :: windows::core::PCWSTR , newdfsentrypath : :: windows::core::PCWSTR , flags : u32 ) -> u32 );
    NetDfsMove(olddfsentrypath.into().abi(), newdfsentrypath.into().abi(), flags)
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsRemove<P0, P1, P2>(dfsentrypath: P0, servername: P1, sharename: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn NetDfsRemove ( dfsentrypath : :: windows::core::PCWSTR , servername : :: windows::core::PCWSTR , sharename : :: windows::core::PCWSTR ) -> u32 );
    NetDfsRemove(dfsentrypath.into().abi(), servername.into().abi(), sharename.into().abi())
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsRemoveFtRoot<P0, P1, P2>(servername: P0, rootshare: P1, ftdfsname: P2, flags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn NetDfsRemoveFtRoot ( servername : :: windows::core::PCWSTR , rootshare : :: windows::core::PCWSTR , ftdfsname : :: windows::core::PCWSTR , flags : u32 ) -> u32 );
    NetDfsRemoveFtRoot(servername.into().abi(), rootshare.into().abi(), ftdfsname.into().abi(), flags)
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsRemoveFtRootForced<P0, P1, P2, P3>(domainname: P0, servername: P1, rootshare: P2, ftdfsname: P3, flags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn NetDfsRemoveFtRootForced ( domainname : :: windows::core::PCWSTR , servername : :: windows::core::PCWSTR , rootshare : :: windows::core::PCWSTR , ftdfsname : :: windows::core::PCWSTR , flags : u32 ) -> u32 );
    NetDfsRemoveFtRootForced(domainname.into().abi(), servername.into().abi(), rootshare.into().abi(), ftdfsname.into().abi(), flags)
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsRemoveRootTarget<P0, P1>(pdfspath: P0, ptargetpath: P1, flags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn NetDfsRemoveRootTarget ( pdfspath : :: windows::core::PCWSTR , ptargetpath : :: windows::core::PCWSTR , flags : u32 ) -> u32 );
    NetDfsRemoveRootTarget(pdfspath.into().abi(), ptargetpath.into().abi(), flags)
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsRemoveStdRoot<P0, P1>(servername: P0, rootshare: P1, flags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn NetDfsRemoveStdRoot ( servername : :: windows::core::PCWSTR , rootshare : :: windows::core::PCWSTR , flags : u32 ) -> u32 );
    NetDfsRemoveStdRoot(servername.into().abi(), rootshare.into().abi(), flags)
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsSetClientInfo<P0, P1, P2>(dfsentrypath: P0, servername: P1, sharename: P2, level: u32, buffer: *const u8) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn NetDfsSetClientInfo ( dfsentrypath : :: windows::core::PCWSTR , servername : :: windows::core::PCWSTR , sharename : :: windows::core::PCWSTR , level : u32 , buffer : *const u8 ) -> u32 );
    NetDfsSetClientInfo(dfsentrypath.into().abi(), servername.into().abi(), sharename.into().abi(), level, buffer)
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetDfsSetFtContainerSecurity<P0, P1>(domainname: P0, securityinformation: u32, psecuritydescriptor: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<super::super::Security::PSECURITY_DESCRIPTOR>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn NetDfsSetFtContainerSecurity ( domainname : :: windows::core::PCWSTR , securityinformation : u32 , psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR ) -> u32 );
    NetDfsSetFtContainerSecurity(domainname.into().abi(), securityinformation, psecuritydescriptor.into())
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsSetInfo<P0, P1, P2>(dfsentrypath: P0, servername: P1, sharename: P2, level: u32, buffer: *const u8) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn NetDfsSetInfo ( dfsentrypath : :: windows::core::PCWSTR , servername : :: windows::core::PCWSTR , sharename : :: windows::core::PCWSTR , level : u32 , buffer : *const u8 ) -> u32 );
    NetDfsSetInfo(dfsentrypath.into().abi(), servername.into().abi(), sharename.into().abi(), level, buffer)
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetDfsSetSecurity<P0, P1>(dfsentrypath: P0, securityinformation: u32, psecuritydescriptor: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<super::super::Security::PSECURITY_DESCRIPTOR>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn NetDfsSetSecurity ( dfsentrypath : :: windows::core::PCWSTR , securityinformation : u32 , psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR ) -> u32 );
    NetDfsSetSecurity(dfsentrypath.into().abi(), securityinformation, psecuritydescriptor.into())
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetDfsSetStdContainerSecurity<P0, P1>(machinename: P0, securityinformation: u32, psecuritydescriptor: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<super::super::Security::PSECURITY_DESCRIPTOR>,
{
    ::windows::core::link ! ( "netapi32.dll""system" fn NetDfsSetStdContainerSecurity ( machinename : :: windows::core::PCWSTR , securityinformation : u32 , psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR ) -> u32 );
    NetDfsSetStdContainerSecurity(machinename.into().abi(), securityinformation, psecuritydescriptor.into())
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_ADD_VOLUME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_FORCE_REMOVE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_MOVE_FLAG_REPLACE_IF_EXISTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_PROPERTY_FLAG_ABDE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_PROPERTY_FLAG_CLUSTER_ENABLED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_PROPERTY_FLAG_INSITE_REFERRALS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_PROPERTY_FLAG_ROOT_SCALABILITY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_PROPERTY_FLAG_SITE_COSTING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_PROPERTY_FLAG_TARGET_FAILBACK: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_RESTORE_VOLUME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_SITE_PRIMARY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_STORAGE_FLAVOR_UNUSED2: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_STORAGE_STATES: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_STORAGE_STATE_ACTIVE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_STORAGE_STATE_OFFLINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_STORAGE_STATE_ONLINE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_VOLUME_FLAVORS: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_VOLUME_FLAVOR_AD_BLOB: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_VOLUME_FLAVOR_STANDALONE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_VOLUME_FLAVOR_UNUSED1: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_VOLUME_STATES: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_VOLUME_STATE_FORCE_SYNC: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_VOLUME_STATE_INCONSISTENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_VOLUME_STATE_OFFLINE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_VOLUME_STATE_OK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_VOLUME_STATE_ONLINE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_VOLUME_STATE_RESYNCHRONIZE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_VOLUME_STATE_STANDBY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const FSCTL_DFS_BASE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const FSCTL_DFS_GET_PKT_ENTRY_STATE: u32 = 401340u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const NET_DFS_SETDC_FLAGS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const NET_DFS_SETDC_INITPKT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const NET_DFS_SETDC_TIMEOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DFS_NAMESPACE_VERSION_ORIGIN(pub i32);
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_NAMESPACE_VERSION_ORIGIN_COMBINED: DFS_NAMESPACE_VERSION_ORIGIN = DFS_NAMESPACE_VERSION_ORIGIN(0i32);
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_NAMESPACE_VERSION_ORIGIN_SERVER: DFS_NAMESPACE_VERSION_ORIGIN = DFS_NAMESPACE_VERSION_ORIGIN(1i32);
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_NAMESPACE_VERSION_ORIGIN_DOMAIN: DFS_NAMESPACE_VERSION_ORIGIN = DFS_NAMESPACE_VERSION_ORIGIN(2i32);
impl ::core::marker::Copy for DFS_NAMESPACE_VERSION_ORIGIN {}
impl ::core::clone::Clone for DFS_NAMESPACE_VERSION_ORIGIN {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DFS_NAMESPACE_VERSION_ORIGIN {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DFS_TARGET_PRIORITY_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DfsInvalidPriorityClass: DFS_TARGET_PRIORITY_CLASS = DFS_TARGET_PRIORITY_CLASS(-1i32);
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DfsSiteCostNormalPriorityClass: DFS_TARGET_PRIORITY_CLASS = DFS_TARGET_PRIORITY_CLASS(0i32);
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DfsGlobalHighPriorityClass: DFS_TARGET_PRIORITY_CLASS = DFS_TARGET_PRIORITY_CLASS(1i32);
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DfsSiteCostHighPriorityClass: DFS_TARGET_PRIORITY_CLASS = DFS_TARGET_PRIORITY_CLASS(2i32);
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DfsSiteCostLowPriorityClass: DFS_TARGET_PRIORITY_CLASS = DFS_TARGET_PRIORITY_CLASS(3i32);
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DfsGlobalLowPriorityClass: DFS_TARGET_PRIORITY_CLASS = DFS_TARGET_PRIORITY_CLASS(4i32);
impl ::core::marker::Copy for DFS_TARGET_PRIORITY_CLASS {}
impl ::core::clone::Clone for DFS_TARGET_PRIORITY_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DFS_TARGET_PRIORITY_CLASS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_GET_PKT_ENTRY_STATE_ARG {
    pub DfsEntryPathLen: u16,
    pub ServerNameLen: u16,
    pub ShareNameLen: u16,
    pub Level: u32,
    pub Buffer: [u16; 1],
}
impl ::core::marker::Copy for DFS_GET_PKT_ENTRY_STATE_ARG {}
impl ::core::clone::Clone for DFS_GET_PKT_ENTRY_STATE_ARG {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DFS_GET_PKT_ENTRY_STATE_ARG {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_1 {
    pub EntryPath: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DFS_INFO_1 {}
impl ::core::clone::Clone for DFS_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_100 {
    pub Comment: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DFS_INFO_100 {}
impl ::core::clone::Clone for DFS_INFO_100 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_100 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_101 {
    pub State: u32,
}
impl ::core::marker::Copy for DFS_INFO_101 {}
impl ::core::clone::Clone for DFS_INFO_101 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_101 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_102 {
    pub Timeout: u32,
}
impl ::core::marker::Copy for DFS_INFO_102 {}
impl ::core::clone::Clone for DFS_INFO_102 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_102 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_103 {
    pub PropertyFlagMask: u32,
    pub PropertyFlags: u32,
}
impl ::core::marker::Copy for DFS_INFO_103 {}
impl ::core::clone::Clone for DFS_INFO_103 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_103 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_104 {
    pub TargetPriority: DFS_TARGET_PRIORITY,
}
impl ::core::marker::Copy for DFS_INFO_104 {}
impl ::core::clone::Clone for DFS_INFO_104 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_104 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_105 {
    pub Comment: ::windows::core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub PropertyFlagMask: u32,
    pub PropertyFlags: u32,
}
impl ::core::marker::Copy for DFS_INFO_105 {}
impl ::core::clone::Clone for DFS_INFO_105 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_105 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_106 {
    pub State: u32,
    pub TargetPriority: DFS_TARGET_PRIORITY,
}
impl ::core::marker::Copy for DFS_INFO_106 {}
impl ::core::clone::Clone for DFS_INFO_106 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_106 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct DFS_INFO_107 {
    pub Comment: ::windows::core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub PropertyFlagMask: u32,
    pub PropertyFlags: u32,
    pub SdLengthReserved: u32,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for DFS_INFO_107 {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for DFS_INFO_107 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security")]
unsafe impl ::windows::core::Abi for DFS_INFO_107 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct DFS_INFO_150 {
    pub SdLengthReserved: u32,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for DFS_INFO_150 {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for DFS_INFO_150 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security")]
unsafe impl ::windows::core::Abi for DFS_INFO_150 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct DFS_INFO_1_32 {
    pub EntryPath: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DFS_INFO_1_32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DFS_INFO_1_32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for DFS_INFO_1_32 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_2 {
    pub EntryPath: ::windows::core::PWSTR,
    pub Comment: ::windows::core::PWSTR,
    pub State: u32,
    pub NumberOfStorages: u32,
}
impl ::core::marker::Copy for DFS_INFO_2 {}
impl ::core::clone::Clone for DFS_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_200 {
    pub FtDfsName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DFS_INFO_200 {}
impl ::core::clone::Clone for DFS_INFO_200 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_200 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct DFS_INFO_2_32 {
    pub EntryPath: u32,
    pub Comment: u32,
    pub State: u32,
    pub NumberOfStorages: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DFS_INFO_2_32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DFS_INFO_2_32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for DFS_INFO_2_32 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_3 {
    pub EntryPath: ::windows::core::PWSTR,
    pub Comment: ::windows::core::PWSTR,
    pub State: u32,
    pub NumberOfStorages: u32,
    pub Storage: *mut DFS_STORAGE_INFO,
}
impl ::core::marker::Copy for DFS_INFO_3 {}
impl ::core::clone::Clone for DFS_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_3 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_300 {
    pub Flags: u32,
    pub DfsName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DFS_INFO_300 {}
impl ::core::clone::Clone for DFS_INFO_300 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_300 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct DFS_INFO_3_32 {
    pub EntryPath: u32,
    pub Comment: u32,
    pub State: u32,
    pub NumberOfStorages: u32,
    pub Storage: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DFS_INFO_3_32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DFS_INFO_3_32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for DFS_INFO_3_32 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_4 {
    pub EntryPath: ::windows::core::PWSTR,
    pub Comment: ::windows::core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: ::windows::core::GUID,
    pub NumberOfStorages: u32,
    pub Storage: *mut DFS_STORAGE_INFO,
}
impl ::core::marker::Copy for DFS_INFO_4 {}
impl ::core::clone::Clone for DFS_INFO_4 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_4 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct DFS_INFO_4_32 {
    pub EntryPath: u32,
    pub Comment: u32,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: ::windows::core::GUID,
    pub NumberOfStorages: u32,
    pub Storage: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DFS_INFO_4_32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DFS_INFO_4_32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for DFS_INFO_4_32 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_5 {
    pub EntryPath: ::windows::core::PWSTR,
    pub Comment: ::windows::core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: ::windows::core::GUID,
    pub PropertyFlags: u32,
    pub MetadataSize: u32,
    pub NumberOfStorages: u32,
}
impl ::core::marker::Copy for DFS_INFO_5 {}
impl ::core::clone::Clone for DFS_INFO_5 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_5 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_50 {
    pub NamespaceMajorVersion: u32,
    pub NamespaceMinorVersion: u32,
    pub NamespaceCapabilities: u64,
}
impl ::core::marker::Copy for DFS_INFO_50 {}
impl ::core::clone::Clone for DFS_INFO_50 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_50 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_6 {
    pub EntryPath: ::windows::core::PWSTR,
    pub Comment: ::windows::core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: ::windows::core::GUID,
    pub PropertyFlags: u32,
    pub MetadataSize: u32,
    pub NumberOfStorages: u32,
    pub Storage: *mut DFS_STORAGE_INFO_1,
}
impl ::core::marker::Copy for DFS_INFO_6 {}
impl ::core::clone::Clone for DFS_INFO_6 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_6 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_7 {
    pub GenerationGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for DFS_INFO_7 {}
impl ::core::clone::Clone for DFS_INFO_7 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_7 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct DFS_INFO_8 {
    pub EntryPath: ::windows::core::PWSTR,
    pub Comment: ::windows::core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: ::windows::core::GUID,
    pub PropertyFlags: u32,
    pub MetadataSize: u32,
    pub SdLengthReserved: u32,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
    pub NumberOfStorages: u32,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for DFS_INFO_8 {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for DFS_INFO_8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security")]
unsafe impl ::windows::core::Abi for DFS_INFO_8 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct DFS_INFO_9 {
    pub EntryPath: ::windows::core::PWSTR,
    pub Comment: ::windows::core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: ::windows::core::GUID,
    pub PropertyFlags: u32,
    pub MetadataSize: u32,
    pub SdLengthReserved: u32,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
    pub NumberOfStorages: u32,
    pub Storage: *mut DFS_STORAGE_INFO_1,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for DFS_INFO_9 {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for DFS_INFO_9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security")]
unsafe impl ::windows::core::Abi for DFS_INFO_9 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_SITELIST_INFO {
    pub cSites: u32,
    pub Site: [DFS_SITENAME_INFO; 1],
}
impl ::core::marker::Copy for DFS_SITELIST_INFO {}
impl ::core::clone::Clone for DFS_SITELIST_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DFS_SITELIST_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_SITENAME_INFO {
    pub SiteFlags: u32,
    pub SiteName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DFS_SITENAME_INFO {}
impl ::core::clone::Clone for DFS_SITENAME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DFS_SITENAME_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_STORAGE_INFO {
    pub State: u32,
    pub ServerName: ::windows::core::PWSTR,
    pub ShareName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DFS_STORAGE_INFO {}
impl ::core::clone::Clone for DFS_STORAGE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DFS_STORAGE_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct DFS_STORAGE_INFO_0_32 {
    pub State: u32,
    pub ServerName: u32,
    pub ShareName: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DFS_STORAGE_INFO_0_32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DFS_STORAGE_INFO_0_32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for DFS_STORAGE_INFO_0_32 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_STORAGE_INFO_1 {
    pub State: u32,
    pub ServerName: ::windows::core::PWSTR,
    pub ShareName: ::windows::core::PWSTR,
    pub TargetPriority: DFS_TARGET_PRIORITY,
}
impl ::core::marker::Copy for DFS_STORAGE_INFO_1 {}
impl ::core::clone::Clone for DFS_STORAGE_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DFS_STORAGE_INFO_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_SUPPORTED_NAMESPACE_VERSION_INFO {
    pub DomainDfsMajorVersion: u32,
    pub DomainDfsMinorVersion: u32,
    pub DomainDfsCapabilities: u64,
    pub StandaloneDfsMajorVersion: u32,
    pub StandaloneDfsMinorVersion: u32,
    pub StandaloneDfsCapabilities: u64,
}
impl ::core::marker::Copy for DFS_SUPPORTED_NAMESPACE_VERSION_INFO {}
impl ::core::clone::Clone for DFS_SUPPORTED_NAMESPACE_VERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DFS_SUPPORTED_NAMESPACE_VERSION_INFO {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_TARGET_PRIORITY {
    pub TargetPriorityClass: DFS_TARGET_PRIORITY_CLASS,
    pub TargetPriorityRank: u16,
    pub Reserved: u16,
}
impl ::core::marker::Copy for DFS_TARGET_PRIORITY {}
impl ::core::clone::Clone for DFS_TARGET_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DFS_TARGET_PRIORITY {
    type Abi = Self;
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
