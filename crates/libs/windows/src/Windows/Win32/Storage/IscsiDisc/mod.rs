#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn AddISNSServerA<P0>(address: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn AddISNSServerA ( address : ::windows::core::PCSTR ) -> u32 );
    AddISNSServerA(address.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn AddISNSServerW<P0>(address: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn AddISNSServerW ( address : ::windows::core::PCWSTR ) -> u32 );
    AddISNSServerW(address.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn AddIScsiConnectionA(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, reserved: *mut ::core::ffi::c_void, initiatorportnumber: u32, targetportal: *mut ISCSI_TARGET_PORTALA, securityflags: u64, loginoptions: *mut ISCSI_LOGIN_OPTIONS, key: ::core::option::Option<&[u8]>, connectionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn AddIScsiConnectionA ( uniquesessionid : *mut ISCSI_UNIQUE_SESSION_ID , reserved : *mut ::core::ffi::c_void , initiatorportnumber : u32 , targetportal : *mut ISCSI_TARGET_PORTALA , securityflags : u64 , loginoptions : *mut ISCSI_LOGIN_OPTIONS , keysize : u32 , key : ::windows::core::PCSTR , connectionid : *mut ISCSI_UNIQUE_SESSION_ID ) -> u32 );
    AddIScsiConnectionA(uniquesessionid, reserved, initiatorportnumber, targetportal, securityflags, loginoptions, key.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(key.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), connectionid)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn AddIScsiConnectionW(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, reserved: *mut ::core::ffi::c_void, initiatorportnumber: u32, targetportal: *mut ISCSI_TARGET_PORTALW, securityflags: u64, loginoptions: *mut ISCSI_LOGIN_OPTIONS, key: ::core::option::Option<&[u8]>, connectionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn AddIScsiConnectionW ( uniquesessionid : *mut ISCSI_UNIQUE_SESSION_ID , reserved : *mut ::core::ffi::c_void , initiatorportnumber : u32 , targetportal : *mut ISCSI_TARGET_PORTALW , securityflags : u64 , loginoptions : *mut ISCSI_LOGIN_OPTIONS , keysize : u32 , key : ::windows::core::PCSTR , connectionid : *mut ISCSI_UNIQUE_SESSION_ID ) -> u32 );
    AddIScsiConnectionW(uniquesessionid, reserved, initiatorportnumber, targetportal, securityflags, loginoptions, key.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(key.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), connectionid)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn AddIScsiSendTargetPortalA<P0>(initiatorinstance: P0, initiatorportnumber: u32, loginoptions: *mut ISCSI_LOGIN_OPTIONS, securityflags: u64, portal: *mut ISCSI_TARGET_PORTALA) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn AddIScsiSendTargetPortalA ( initiatorinstance : ::windows::core::PCSTR , initiatorportnumber : u32 , loginoptions : *mut ISCSI_LOGIN_OPTIONS , securityflags : u64 , portal : *mut ISCSI_TARGET_PORTALA ) -> u32 );
    AddIScsiSendTargetPortalA(initiatorinstance.into_param().abi(), initiatorportnumber, loginoptions, securityflags, portal)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn AddIScsiSendTargetPortalW<P0>(initiatorinstance: P0, initiatorportnumber: u32, loginoptions: *mut ISCSI_LOGIN_OPTIONS, securityflags: u64, portal: *mut ISCSI_TARGET_PORTALW) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn AddIScsiSendTargetPortalW ( initiatorinstance : ::windows::core::PCWSTR , initiatorportnumber : u32 , loginoptions : *mut ISCSI_LOGIN_OPTIONS , securityflags : u64 , portal : *mut ISCSI_TARGET_PORTALW ) -> u32 );
    AddIScsiSendTargetPortalW(initiatorinstance.into_param().abi(), initiatorportnumber, loginoptions, securityflags, portal)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddIScsiStaticTargetA<P0, P1, P2>(targetname: P0, targetalias: P1, targetflags: u32, persist: P2, mappings: *mut ISCSI_TARGET_MAPPINGA, loginoptions: *mut ISCSI_LOGIN_OPTIONS, portalgroup: *mut ISCSI_TARGET_PORTAL_GROUPA) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn AddIScsiStaticTargetA ( targetname : ::windows::core::PCSTR , targetalias : ::windows::core::PCSTR , targetflags : u32 , persist : super::super::Foundation:: BOOLEAN , mappings : *mut ISCSI_TARGET_MAPPINGA , loginoptions : *mut ISCSI_LOGIN_OPTIONS , portalgroup : *mut ISCSI_TARGET_PORTAL_GROUPA ) -> u32 );
    AddIScsiStaticTargetA(targetname.into_param().abi(), targetalias.into_param().abi(), targetflags, persist.into_param().abi(), mappings, loginoptions, portalgroup)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddIScsiStaticTargetW<P0, P1, P2>(targetname: P0, targetalias: P1, targetflags: u32, persist: P2, mappings: *mut ISCSI_TARGET_MAPPINGW, loginoptions: *mut ISCSI_LOGIN_OPTIONS, portalgroup: *mut ISCSI_TARGET_PORTAL_GROUPW) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn AddIScsiStaticTargetW ( targetname : ::windows::core::PCWSTR , targetalias : ::windows::core::PCWSTR , targetflags : u32 , persist : super::super::Foundation:: BOOLEAN , mappings : *mut ISCSI_TARGET_MAPPINGW , loginoptions : *mut ISCSI_LOGIN_OPTIONS , portalgroup : *mut ISCSI_TARGET_PORTAL_GROUPW ) -> u32 );
    AddIScsiStaticTargetW(targetname.into_param().abi(), targetalias.into_param().abi(), targetflags, persist.into_param().abi(), mappings, loginoptions, portalgroup)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn AddPersistentIScsiDeviceA<P0>(devicepath: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn AddPersistentIScsiDeviceA ( devicepath : ::windows::core::PCSTR ) -> u32 );
    AddPersistentIScsiDeviceA(devicepath.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn AddPersistentIScsiDeviceW<P0>(devicepath: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn AddPersistentIScsiDeviceW ( devicepath : ::windows::core::PCWSTR ) -> u32 );
    AddPersistentIScsiDeviceW(devicepath.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn AddRadiusServerA<P0>(address: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn AddRadiusServerA ( address : ::windows::core::PCSTR ) -> u32 );
    AddRadiusServerA(address.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn AddRadiusServerW<P0>(address: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn AddRadiusServerW ( address : ::windows::core::PCWSTR ) -> u32 );
    AddRadiusServerW(address.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn ClearPersistentIScsiDevices() -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn ClearPersistentIScsiDevices ( ) -> u32 );
    ClearPersistentIScsiDevices()
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_System_Ioctl\"`*"]
#[cfg(feature = "Win32_System_Ioctl")]
#[inline]
pub unsafe fn GetDevicesForIScsiSessionA(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, devicecount: *mut u32, devices: *mut ISCSI_DEVICE_ON_SESSIONA) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn GetDevicesForIScsiSessionA ( uniquesessionid : *mut ISCSI_UNIQUE_SESSION_ID , devicecount : *mut u32 , devices : *mut ISCSI_DEVICE_ON_SESSIONA ) -> u32 );
    GetDevicesForIScsiSessionA(uniquesessionid, devicecount, devices)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_System_Ioctl\"`*"]
#[cfg(feature = "Win32_System_Ioctl")]
#[inline]
pub unsafe fn GetDevicesForIScsiSessionW(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, devicecount: *mut u32, devices: *mut ISCSI_DEVICE_ON_SESSIONW) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn GetDevicesForIScsiSessionW ( uniquesessionid : *mut ISCSI_UNIQUE_SESSION_ID , devicecount : *mut u32 , devices : *mut ISCSI_DEVICE_ON_SESSIONW ) -> u32 );
    GetDevicesForIScsiSessionW(uniquesessionid, devicecount, devices)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn GetIScsiIKEInfoA<P0>(initiatorname: P0, initiatorportnumber: u32, reserved: *mut u32, authinfo: *mut IKE_AUTHENTICATION_INFORMATION) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn GetIScsiIKEInfoA ( initiatorname : ::windows::core::PCSTR , initiatorportnumber : u32 , reserved : *mut u32 , authinfo : *mut IKE_AUTHENTICATION_INFORMATION ) -> u32 );
    GetIScsiIKEInfoA(initiatorname.into_param().abi(), initiatorportnumber, reserved, authinfo)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn GetIScsiIKEInfoW<P0>(initiatorname: P0, initiatorportnumber: u32, reserved: *mut u32, authinfo: *mut IKE_AUTHENTICATION_INFORMATION) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn GetIScsiIKEInfoW ( initiatorname : ::windows::core::PCWSTR , initiatorportnumber : u32 , reserved : *mut u32 , authinfo : *mut IKE_AUTHENTICATION_INFORMATION ) -> u32 );
    GetIScsiIKEInfoW(initiatorname.into_param().abi(), initiatorportnumber, reserved, authinfo)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn GetIScsiInitiatorNodeNameA(initiatornodename: ::windows::core::PSTR) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn GetIScsiInitiatorNodeNameA ( initiatornodename : ::windows::core::PSTR ) -> u32 );
    GetIScsiInitiatorNodeNameA(::core::mem::transmute(initiatornodename))
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn GetIScsiInitiatorNodeNameW(initiatornodename: ::windows::core::PWSTR) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn GetIScsiInitiatorNodeNameW ( initiatornodename : ::windows::core::PWSTR ) -> u32 );
    GetIScsiInitiatorNodeNameW(::core::mem::transmute(initiatornodename))
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn GetIScsiSessionListA(buffersize: *mut u32, sessioncount: *mut u32, sessioninfo: *mut ISCSI_SESSION_INFOA) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn GetIScsiSessionListA ( buffersize : *mut u32 , sessioncount : *mut u32 , sessioninfo : *mut ISCSI_SESSION_INFOA ) -> u32 );
    GetIScsiSessionListA(buffersize, sessioncount, sessioninfo)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetIScsiSessionListEx(buffersize: *mut u32, sessioncountptr: *mut u32, sessioninfo: *mut ISCSI_SESSION_INFO_EX) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn GetIScsiSessionListEx ( buffersize : *mut u32 , sessioncountptr : *mut u32 , sessioninfo : *mut ISCSI_SESSION_INFO_EX ) -> u32 );
    GetIScsiSessionListEx(buffersize, sessioncountptr, sessioninfo)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn GetIScsiSessionListW(buffersize: *mut u32, sessioncount: *mut u32, sessioninfo: *mut ISCSI_SESSION_INFOW) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn GetIScsiSessionListW ( buffersize : *mut u32 , sessioncount : *mut u32 , sessioninfo : *mut ISCSI_SESSION_INFOW ) -> u32 );
    GetIScsiSessionListW(buffersize, sessioncount, sessioninfo)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn GetIScsiTargetInformationA<P0, P1>(targetname: P0, discoverymechanism: P1, infoclass: TARGET_INFORMATION_CLASS, buffersize: *mut u32, buffer: *mut ::core::ffi::c_void) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn GetIScsiTargetInformationA ( targetname : ::windows::core::PCSTR , discoverymechanism : ::windows::core::PCSTR , infoclass : TARGET_INFORMATION_CLASS , buffersize : *mut u32 , buffer : *mut ::core::ffi::c_void ) -> u32 );
    GetIScsiTargetInformationA(targetname.into_param().abi(), discoverymechanism.into_param().abi(), infoclass, buffersize, buffer)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn GetIScsiTargetInformationW<P0, P1>(targetname: P0, discoverymechanism: P1, infoclass: TARGET_INFORMATION_CLASS, buffersize: *mut u32, buffer: *mut ::core::ffi::c_void) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn GetIScsiTargetInformationW ( targetname : ::windows::core::PCWSTR , discoverymechanism : ::windows::core::PCWSTR , infoclass : TARGET_INFORMATION_CLASS , buffersize : *mut u32 , buffer : *mut ::core::ffi::c_void ) -> u32 );
    GetIScsiTargetInformationW(targetname.into_param().abi(), discoverymechanism.into_param().abi(), infoclass, buffersize, buffer)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn GetIScsiVersionInformation(versioninfo: *mut ISCSI_VERSION_INFO) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn GetIScsiVersionInformation ( versioninfo : *mut ISCSI_VERSION_INFO ) -> u32 );
    GetIScsiVersionInformation(versioninfo)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoginIScsiTargetA<P0, P1, P2, P3>(targetname: P0, isinformationalsession: P1, initiatorinstance: P2, initiatorportnumber: u32, targetportal: *mut ISCSI_TARGET_PORTALA, securityflags: u64, mappings: *mut ISCSI_TARGET_MAPPINGA, loginoptions: *mut ISCSI_LOGIN_OPTIONS, key: ::core::option::Option<&[u8]>, ispersistent: P3, uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, uniqueconnectionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn LoginIScsiTargetA ( targetname : ::windows::core::PCSTR , isinformationalsession : super::super::Foundation:: BOOLEAN , initiatorinstance : ::windows::core::PCSTR , initiatorportnumber : u32 , targetportal : *mut ISCSI_TARGET_PORTALA , securityflags : u64 , mappings : *mut ISCSI_TARGET_MAPPINGA , loginoptions : *mut ISCSI_LOGIN_OPTIONS , keysize : u32 , key : ::windows::core::PCSTR , ispersistent : super::super::Foundation:: BOOLEAN , uniquesessionid : *mut ISCSI_UNIQUE_SESSION_ID , uniqueconnectionid : *mut ISCSI_UNIQUE_SESSION_ID ) -> u32 );
    LoginIScsiTargetA(targetname.into_param().abi(), isinformationalsession.into_param().abi(), initiatorinstance.into_param().abi(), initiatorportnumber, targetportal, securityflags, mappings, loginoptions, key.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(key.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ispersistent.into_param().abi(), uniquesessionid, uniqueconnectionid)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LoginIScsiTargetW<P0, P1, P2, P3>(targetname: P0, isinformationalsession: P1, initiatorinstance: P2, initiatorportnumber: u32, targetportal: *mut ISCSI_TARGET_PORTALW, securityflags: u64, mappings: *mut ISCSI_TARGET_MAPPINGW, loginoptions: *mut ISCSI_LOGIN_OPTIONS, key: ::core::option::Option<&[u8]>, ispersistent: P3, uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, uniqueconnectionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn LoginIScsiTargetW ( targetname : ::windows::core::PCWSTR , isinformationalsession : super::super::Foundation:: BOOLEAN , initiatorinstance : ::windows::core::PCWSTR , initiatorportnumber : u32 , targetportal : *mut ISCSI_TARGET_PORTALW , securityflags : u64 , mappings : *mut ISCSI_TARGET_MAPPINGW , loginoptions : *mut ISCSI_LOGIN_OPTIONS , keysize : u32 , key : ::windows::core::PCSTR , ispersistent : super::super::Foundation:: BOOLEAN , uniquesessionid : *mut ISCSI_UNIQUE_SESSION_ID , uniqueconnectionid : *mut ISCSI_UNIQUE_SESSION_ID ) -> u32 );
    LoginIScsiTargetW(targetname.into_param().abi(), isinformationalsession.into_param().abi(), initiatorinstance.into_param().abi(), initiatorportnumber, targetportal, securityflags, mappings, loginoptions, key.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(key.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ispersistent.into_param().abi(), uniquesessionid, uniqueconnectionid)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn LogoutIScsiTarget(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn LogoutIScsiTarget ( uniquesessionid : *mut ISCSI_UNIQUE_SESSION_ID ) -> u32 );
    LogoutIScsiTarget(uniquesessionid)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn RefreshISNSServerA<P0>(address: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn RefreshISNSServerA ( address : ::windows::core::PCSTR ) -> u32 );
    RefreshISNSServerA(address.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn RefreshISNSServerW<P0>(address: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn RefreshISNSServerW ( address : ::windows::core::PCWSTR ) -> u32 );
    RefreshISNSServerW(address.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn RefreshIScsiSendTargetPortalA<P0>(initiatorinstance: P0, initiatorportnumber: u32, portal: *mut ISCSI_TARGET_PORTALA) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn RefreshIScsiSendTargetPortalA ( initiatorinstance : ::windows::core::PCSTR , initiatorportnumber : u32 , portal : *mut ISCSI_TARGET_PORTALA ) -> u32 );
    RefreshIScsiSendTargetPortalA(initiatorinstance.into_param().abi(), initiatorportnumber, portal)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn RefreshIScsiSendTargetPortalW<P0>(initiatorinstance: P0, initiatorportnumber: u32, portal: *mut ISCSI_TARGET_PORTALW) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn RefreshIScsiSendTargetPortalW ( initiatorinstance : ::windows::core::PCWSTR , initiatorportnumber : u32 , portal : *mut ISCSI_TARGET_PORTALW ) -> u32 );
    RefreshIScsiSendTargetPortalW(initiatorinstance.into_param().abi(), initiatorportnumber, portal)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn RemoveISNSServerA<P0>(address: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn RemoveISNSServerA ( address : ::windows::core::PCSTR ) -> u32 );
    RemoveISNSServerA(address.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn RemoveISNSServerW<P0>(address: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn RemoveISNSServerW ( address : ::windows::core::PCWSTR ) -> u32 );
    RemoveISNSServerW(address.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn RemoveIScsiConnection(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, connectionid: *mut ISCSI_UNIQUE_SESSION_ID) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn RemoveIScsiConnection ( uniquesessionid : *mut ISCSI_UNIQUE_SESSION_ID , connectionid : *mut ISCSI_UNIQUE_SESSION_ID ) -> u32 );
    RemoveIScsiConnection(uniquesessionid, connectionid)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn RemoveIScsiPersistentTargetA<P0, P1>(initiatorinstance: P0, initiatorportnumber: u32, targetname: P1, portal: *mut ISCSI_TARGET_PORTALA) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn RemoveIScsiPersistentTargetA ( initiatorinstance : ::windows::core::PCSTR , initiatorportnumber : u32 , targetname : ::windows::core::PCSTR , portal : *mut ISCSI_TARGET_PORTALA ) -> u32 );
    RemoveIScsiPersistentTargetA(initiatorinstance.into_param().abi(), initiatorportnumber, targetname.into_param().abi(), portal)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn RemoveIScsiPersistentTargetW<P0, P1>(initiatorinstance: P0, initiatorportnumber: u32, targetname: P1, portal: *mut ISCSI_TARGET_PORTALW) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn RemoveIScsiPersistentTargetW ( initiatorinstance : ::windows::core::PCWSTR , initiatorportnumber : u32 , targetname : ::windows::core::PCWSTR , portal : *mut ISCSI_TARGET_PORTALW ) -> u32 );
    RemoveIScsiPersistentTargetW(initiatorinstance.into_param().abi(), initiatorportnumber, targetname.into_param().abi(), portal)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn RemoveIScsiSendTargetPortalA<P0>(initiatorinstance: P0, initiatorportnumber: u32, portal: *mut ISCSI_TARGET_PORTALA) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn RemoveIScsiSendTargetPortalA ( initiatorinstance : ::windows::core::PCSTR , initiatorportnumber : u32 , portal : *mut ISCSI_TARGET_PORTALA ) -> u32 );
    RemoveIScsiSendTargetPortalA(initiatorinstance.into_param().abi(), initiatorportnumber, portal)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn RemoveIScsiSendTargetPortalW<P0>(initiatorinstance: P0, initiatorportnumber: u32, portal: *mut ISCSI_TARGET_PORTALW) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn RemoveIScsiSendTargetPortalW ( initiatorinstance : ::windows::core::PCWSTR , initiatorportnumber : u32 , portal : *mut ISCSI_TARGET_PORTALW ) -> u32 );
    RemoveIScsiSendTargetPortalW(initiatorinstance.into_param().abi(), initiatorportnumber, portal)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn RemoveIScsiStaticTargetA<P0>(targetname: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn RemoveIScsiStaticTargetA ( targetname : ::windows::core::PCSTR ) -> u32 );
    RemoveIScsiStaticTargetA(targetname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn RemoveIScsiStaticTargetW<P0>(targetname: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn RemoveIScsiStaticTargetW ( targetname : ::windows::core::PCWSTR ) -> u32 );
    RemoveIScsiStaticTargetW(targetname.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn RemovePersistentIScsiDeviceA<P0>(devicepath: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn RemovePersistentIScsiDeviceA ( devicepath : ::windows::core::PCSTR ) -> u32 );
    RemovePersistentIScsiDeviceA(devicepath.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn RemovePersistentIScsiDeviceW<P0>(devicepath: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn RemovePersistentIScsiDeviceW ( devicepath : ::windows::core::PCWSTR ) -> u32 );
    RemovePersistentIScsiDeviceW(devicepath.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn RemoveRadiusServerA<P0>(address: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn RemoveRadiusServerA ( address : ::windows::core::PCSTR ) -> u32 );
    RemoveRadiusServerA(address.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn RemoveRadiusServerW<P0>(address: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn RemoveRadiusServerW ( address : ::windows::core::PCWSTR ) -> u32 );
    RemoveRadiusServerW(address.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn ReportActiveIScsiTargetMappingsA(buffersize: *mut u32, mappingcount: *mut u32, mappings: *mut ISCSI_TARGET_MAPPINGA) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn ReportActiveIScsiTargetMappingsA ( buffersize : *mut u32 , mappingcount : *mut u32 , mappings : *mut ISCSI_TARGET_MAPPINGA ) -> u32 );
    ReportActiveIScsiTargetMappingsA(buffersize, mappingcount, mappings)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn ReportActiveIScsiTargetMappingsW(buffersize: *mut u32, mappingcount: *mut u32, mappings: *mut ISCSI_TARGET_MAPPINGW) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn ReportActiveIScsiTargetMappingsW ( buffersize : *mut u32 , mappingcount : *mut u32 , mappings : *mut ISCSI_TARGET_MAPPINGW ) -> u32 );
    ReportActiveIScsiTargetMappingsW(buffersize, mappingcount, mappings)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn ReportISNSServerListA(buffersizeinchar: *mut u32, buffer: ::windows::core::PSTR) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn ReportISNSServerListA ( buffersizeinchar : *mut u32 , buffer : ::windows::core::PSTR ) -> u32 );
    ReportISNSServerListA(buffersizeinchar, ::core::mem::transmute(buffer))
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn ReportISNSServerListW(buffersizeinchar: *mut u32, buffer: ::windows::core::PWSTR) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn ReportISNSServerListW ( buffersizeinchar : *mut u32 , buffer : ::windows::core::PWSTR ) -> u32 );
    ReportISNSServerListW(buffersizeinchar, ::core::mem::transmute(buffer))
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn ReportIScsiInitiatorListA(buffersize: *mut u32, buffer: ::windows::core::PSTR) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn ReportIScsiInitiatorListA ( buffersize : *mut u32 , buffer : ::windows::core::PSTR ) -> u32 );
    ReportIScsiInitiatorListA(buffersize, ::core::mem::transmute(buffer))
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn ReportIScsiInitiatorListW(buffersize: *mut u32, buffer: ::windows::core::PWSTR) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn ReportIScsiInitiatorListW ( buffersize : *mut u32 , buffer : ::windows::core::PWSTR ) -> u32 );
    ReportIScsiInitiatorListW(buffersize, ::core::mem::transmute(buffer))
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReportIScsiPersistentLoginsA(count: *mut u32, persistentlogininfo: *mut PERSISTENT_ISCSI_LOGIN_INFOA, buffersizeinbytes: *mut u32) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn ReportIScsiPersistentLoginsA ( count : *mut u32 , persistentlogininfo : *mut PERSISTENT_ISCSI_LOGIN_INFOA , buffersizeinbytes : *mut u32 ) -> u32 );
    ReportIScsiPersistentLoginsA(count, persistentlogininfo, buffersizeinbytes)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReportIScsiPersistentLoginsW(count: *mut u32, persistentlogininfo: *mut PERSISTENT_ISCSI_LOGIN_INFOW, buffersizeinbytes: *mut u32) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn ReportIScsiPersistentLoginsW ( count : *mut u32 , persistentlogininfo : *mut PERSISTENT_ISCSI_LOGIN_INFOW , buffersizeinbytes : *mut u32 ) -> u32 );
    ReportIScsiPersistentLoginsW(count, persistentlogininfo, buffersizeinbytes)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn ReportIScsiSendTargetPortalsA(portalcount: *mut u32, portalinfo: *mut ISCSI_TARGET_PORTAL_INFOA) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn ReportIScsiSendTargetPortalsA ( portalcount : *mut u32 , portalinfo : *mut ISCSI_TARGET_PORTAL_INFOA ) -> u32 );
    ReportIScsiSendTargetPortalsA(portalcount, portalinfo)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn ReportIScsiSendTargetPortalsExA(portalcount: *mut u32, portalinfosize: *mut u32, portalinfo: *mut ISCSI_TARGET_PORTAL_INFO_EXA) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn ReportIScsiSendTargetPortalsExA ( portalcount : *mut u32 , portalinfosize : *mut u32 , portalinfo : *mut ISCSI_TARGET_PORTAL_INFO_EXA ) -> u32 );
    ReportIScsiSendTargetPortalsExA(portalcount, portalinfosize, portalinfo)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn ReportIScsiSendTargetPortalsExW(portalcount: *mut u32, portalinfosize: *mut u32, portalinfo: *mut ISCSI_TARGET_PORTAL_INFO_EXW) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn ReportIScsiSendTargetPortalsExW ( portalcount : *mut u32 , portalinfosize : *mut u32 , portalinfo : *mut ISCSI_TARGET_PORTAL_INFO_EXW ) -> u32 );
    ReportIScsiSendTargetPortalsExW(portalcount, portalinfosize, portalinfo)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn ReportIScsiSendTargetPortalsW(portalcount: *mut u32, portalinfo: *mut ISCSI_TARGET_PORTAL_INFOW) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn ReportIScsiSendTargetPortalsW ( portalcount : *mut u32 , portalinfo : *mut ISCSI_TARGET_PORTAL_INFOW ) -> u32 );
    ReportIScsiSendTargetPortalsW(portalcount, portalinfo)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn ReportIScsiTargetPortalsA<P0, P1>(initiatorname: P0, targetname: P1, targetportaltag: *mut u16, elementcount: *mut u32, portals: *mut ISCSI_TARGET_PORTALA) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn ReportIScsiTargetPortalsA ( initiatorname : ::windows::core::PCSTR , targetname : ::windows::core::PCSTR , targetportaltag : *mut u16 , elementcount : *mut u32 , portals : *mut ISCSI_TARGET_PORTALA ) -> u32 );
    ReportIScsiTargetPortalsA(initiatorname.into_param().abi(), targetname.into_param().abi(), targetportaltag, elementcount, portals)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn ReportIScsiTargetPortalsW<P0, P1>(initiatorname: P0, targetname: P1, targetportaltag: *mut u16, elementcount: *mut u32, portals: *mut ISCSI_TARGET_PORTALW) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn ReportIScsiTargetPortalsW ( initiatorname : ::windows::core::PCWSTR , targetname : ::windows::core::PCWSTR , targetportaltag : *mut u16 , elementcount : *mut u32 , portals : *mut ISCSI_TARGET_PORTALW ) -> u32 );
    ReportIScsiTargetPortalsW(initiatorname.into_param().abi(), targetname.into_param().abi(), targetportaltag, elementcount, portals)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReportIScsiTargetsA<P0>(forceupdate: P0, buffersize: *mut u32, buffer: ::windows::core::PSTR) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn ReportIScsiTargetsA ( forceupdate : super::super::Foundation:: BOOLEAN , buffersize : *mut u32 , buffer : ::windows::core::PSTR ) -> u32 );
    ReportIScsiTargetsA(forceupdate.into_param().abi(), buffersize, ::core::mem::transmute(buffer))
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReportIScsiTargetsW<P0>(forceupdate: P0, buffersize: *mut u32, buffer: ::windows::core::PWSTR) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn ReportIScsiTargetsW ( forceupdate : super::super::Foundation:: BOOLEAN , buffersize : *mut u32 , buffer : ::windows::core::PWSTR ) -> u32 );
    ReportIScsiTargetsW(forceupdate.into_param().abi(), buffersize, ::core::mem::transmute(buffer))
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn ReportPersistentIScsiDevicesA(buffersizeinchar: *mut u32, buffer: ::windows::core::PSTR) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn ReportPersistentIScsiDevicesA ( buffersizeinchar : *mut u32 , buffer : ::windows::core::PSTR ) -> u32 );
    ReportPersistentIScsiDevicesA(buffersizeinchar, ::core::mem::transmute(buffer))
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn ReportPersistentIScsiDevicesW(buffersizeinchar: *mut u32, buffer: ::windows::core::PWSTR) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn ReportPersistentIScsiDevicesW ( buffersizeinchar : *mut u32 , buffer : ::windows::core::PWSTR ) -> u32 );
    ReportPersistentIScsiDevicesW(buffersizeinchar, ::core::mem::transmute(buffer))
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn ReportRadiusServerListA(buffersizeinchar: *mut u32, buffer: ::windows::core::PSTR) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn ReportRadiusServerListA ( buffersizeinchar : *mut u32 , buffer : ::windows::core::PSTR ) -> u32 );
    ReportRadiusServerListA(buffersizeinchar, ::core::mem::transmute(buffer))
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn ReportRadiusServerListW(buffersizeinchar: *mut u32, buffer: ::windows::core::PWSTR) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn ReportRadiusServerListW ( buffersizeinchar : *mut u32 , buffer : ::windows::core::PWSTR ) -> u32 );
    ReportRadiusServerListW(buffersizeinchar, ::core::mem::transmute(buffer))
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn SendScsiInquiry(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, lun: u64, evpdcmddt: u8, pagecode: u8, scsistatus: *mut u8, responsesize: *mut u32, responsebuffer: *mut u8, sensesize: *mut u32, sensebuffer: *mut u8) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn SendScsiInquiry ( uniquesessionid : *mut ISCSI_UNIQUE_SESSION_ID , lun : u64 , evpdcmddt : u8 , pagecode : u8 , scsistatus : *mut u8 , responsesize : *mut u32 , responsebuffer : *mut u8 , sensesize : *mut u32 , sensebuffer : *mut u8 ) -> u32 );
    SendScsiInquiry(uniquesessionid, lun, evpdcmddt, pagecode, scsistatus, responsesize, responsebuffer, sensesize, sensebuffer)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn SendScsiReadCapacity(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, lun: u64, scsistatus: *mut u8, responsesize: *mut u32, responsebuffer: *mut u8, sensesize: *mut u32, sensebuffer: *mut u8) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn SendScsiReadCapacity ( uniquesessionid : *mut ISCSI_UNIQUE_SESSION_ID , lun : u64 , scsistatus : *mut u8 , responsesize : *mut u32 , responsebuffer : *mut u8 , sensesize : *mut u32 , sensebuffer : *mut u8 ) -> u32 );
    SendScsiReadCapacity(uniquesessionid, lun, scsistatus, responsesize, responsebuffer, sensesize, sensebuffer)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn SendScsiReportLuns(uniquesessionid: *mut ISCSI_UNIQUE_SESSION_ID, scsistatus: *mut u8, responsesize: *mut u32, responsebuffer: *mut u8, sensesize: *mut u32, sensebuffer: *mut u8) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn SendScsiReportLuns ( uniquesessionid : *mut ISCSI_UNIQUE_SESSION_ID , scsistatus : *mut u8 , responsesize : *mut u32 , responsebuffer : *mut u8 , sensesize : *mut u32 , sensebuffer : *mut u8 ) -> u32 );
    SendScsiReportLuns(uniquesessionid, scsistatus, responsesize, responsebuffer, sensesize, sensebuffer)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetIScsiGroupPresharedKey<P0>(keylength: u32, key: *mut u8, persist: P0) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn SetIScsiGroupPresharedKey ( keylength : u32 , key : *mut u8 , persist : super::super::Foundation:: BOOLEAN ) -> u32 );
    SetIScsiGroupPresharedKey(keylength, key, persist.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetIScsiIKEInfoA<P0, P1>(initiatorname: P0, initiatorportnumber: u32, authinfo: *mut IKE_AUTHENTICATION_INFORMATION, persist: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn SetIScsiIKEInfoA ( initiatorname : ::windows::core::PCSTR , initiatorportnumber : u32 , authinfo : *mut IKE_AUTHENTICATION_INFORMATION , persist : super::super::Foundation:: BOOLEAN ) -> u32 );
    SetIScsiIKEInfoA(initiatorname.into_param().abi(), initiatorportnumber, authinfo, persist.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetIScsiIKEInfoW<P0, P1>(initiatorname: P0, initiatorportnumber: u32, authinfo: *mut IKE_AUTHENTICATION_INFORMATION, persist: P1) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn SetIScsiIKEInfoW ( initiatorname : ::windows::core::PCWSTR , initiatorportnumber : u32 , authinfo : *mut IKE_AUTHENTICATION_INFORMATION , persist : super::super::Foundation:: BOOLEAN ) -> u32 );
    SetIScsiIKEInfoW(initiatorname.into_param().abi(), initiatorportnumber, authinfo, persist.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn SetIScsiInitiatorCHAPSharedSecret(sharedsecretlength: u32, sharedsecret: *mut u8) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn SetIScsiInitiatorCHAPSharedSecret ( sharedsecretlength : u32 , sharedsecret : *mut u8 ) -> u32 );
    SetIScsiInitiatorCHAPSharedSecret(sharedsecretlength, sharedsecret)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn SetIScsiInitiatorNodeNameA<P0>(initiatornodename: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn SetIScsiInitiatorNodeNameA ( initiatornodename : ::windows::core::PCSTR ) -> u32 );
    SetIScsiInitiatorNodeNameA(initiatornodename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn SetIScsiInitiatorNodeNameW<P0>(initiatornodename: P0) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn SetIScsiInitiatorNodeNameW ( initiatornodename : ::windows::core::PCWSTR ) -> u32 );
    SetIScsiInitiatorNodeNameW(initiatornodename.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn SetIScsiInitiatorRADIUSSharedSecret(sharedsecretlength: u32, sharedsecret: *mut u8) -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn SetIScsiInitiatorRADIUSSharedSecret ( sharedsecretlength : u32 , sharedsecret : *mut u8 ) -> u32 );
    SetIScsiInitiatorRADIUSSharedSecret(sharedsecretlength, sharedsecret)
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetIScsiTunnelModeOuterAddressA<P0, P1, P2, P3>(initiatorname: P0, initiatorportnumber: u32, destinationaddress: P1, outermodeaddress: P2, persist: P3) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCSTR>,
    P3: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn SetIScsiTunnelModeOuterAddressA ( initiatorname : ::windows::core::PCSTR , initiatorportnumber : u32 , destinationaddress : ::windows::core::PCSTR , outermodeaddress : ::windows::core::PCSTR , persist : super::super::Foundation:: BOOLEAN ) -> u32 );
    SetIScsiTunnelModeOuterAddressA(initiatorname.into_param().abi(), initiatorportnumber, destinationaddress.into_param().abi(), outermodeaddress.into_param().abi(), persist.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetIScsiTunnelModeOuterAddressW<P0, P1, P2, P3>(initiatorname: P0, initiatorportnumber: u32, destinationaddress: P1, outermodeaddress: P2, persist: P3) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
{
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn SetIScsiTunnelModeOuterAddressW ( initiatorname : ::windows::core::PCWSTR , initiatorportnumber : u32 , destinationaddress : ::windows::core::PCWSTR , outermodeaddress : ::windows::core::PCWSTR , persist : super::super::Foundation:: BOOLEAN ) -> u32 );
    SetIScsiTunnelModeOuterAddressW(initiatorname.into_param().abi(), initiatorportnumber, destinationaddress.into_param().abi(), outermodeaddress.into_param().abi(), persist.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn SetupPersistentIScsiDevices() -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn SetupPersistentIScsiDevices ( ) -> u32 );
    SetupPersistentIScsiDevices()
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[inline]
pub unsafe fn SetupPersistentIScsiVolumes() -> u32 {
    ::windows_targets::link ! ( "iscsidsc.dll""system" fn SetupPersistentIScsiVolumes ( ) -> u32 );
    SetupPersistentIScsiVolumes()
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ATA_FLAGS_48BIT_COMMAND: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ATA_FLAGS_DATA_IN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ATA_FLAGS_DATA_OUT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ATA_FLAGS_DRDY_REQUIRED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ATA_FLAGS_NO_MULTIPLE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ATA_FLAGS_USE_DMA: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const DD_SCSI_DEVICE_NAME: ::windows::core::PCSTR = ::windows::core::s!("\\Device\\ScsiPort");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const DUMP_DRIVER_NAME_LENGTH: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const DUMP_EX_FLAG_DRIVER_FULL_PATH_SUPPORT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const DUMP_EX_FLAG_RESUME_SUPPORT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const DUMP_EX_FLAG_SUPPORT_64BITMEMORY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const DUMP_EX_FLAG_SUPPORT_DD_TELEMETRY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const DUMP_POINTERS_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const DUMP_POINTERS_VERSION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const DUMP_POINTERS_VERSION_3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const DUMP_POINTERS_VERSION_4: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FILE_DEVICE_SCSI: u32 = 27u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_FUNCTION_ACTIVATE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_FUNCTION_DOWNLOAD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_FUNCTION_GET_INFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_REQUEST_BLOCK_STRUCTURE_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_REQUEST_FLAG_CONTROLLER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_REQUEST_FLAG_FIRST_SEGMENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_REQUEST_FLAG_LAST_SEGMENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_REQUEST_FLAG_REPLACE_EXISTING_IMAGE: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_REQUEST_FLAG_SWITCH_TO_EXISTING_FIRMWARE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_COMMAND_ABORT: u32 = 133u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_CONTROLLER_ERROR: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_DEVICE_ERROR: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_END_OF_MEDIA: u32 = 134u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_ERROR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_ID_NOT_FOUND: u32 = 131u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_ILLEGAL_LENGTH: u32 = 135u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_ILLEGAL_REQUEST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_INPUT_BUFFER_TOO_BIG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_INTERFACE_CRC_ERROR: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_INVALID_IMAGE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_INVALID_PARAMETER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_INVALID_SLOT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_MEDIA_CHANGE: u32 = 130u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_MEDIA_CHANGE_REQUEST: u32 = 132u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_OUTPUT_BUFFER_TOO_SMALL: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_POWER_CYCLE_REQUIRED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const FIRMWARE_STATUS_UNCORRECTABLE_DATA_ERROR: u32 = 129u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const HYBRID_FUNCTION_DEMOTE_BY_SIZE: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const HYBRID_FUNCTION_DISABLE_CACHING_MEDIUM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const HYBRID_FUNCTION_ENABLE_CACHING_MEDIUM: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const HYBRID_FUNCTION_GET_INFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const HYBRID_FUNCTION_SET_DIRTY_THRESHOLD: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const HYBRID_REQUEST_BLOCK_STRUCTURE_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const HYBRID_REQUEST_INFO_STRUCTURE_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const HYBRID_STATUS_ENABLE_REFCOUNT_HOLD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const HYBRID_STATUS_ILLEGAL_REQUEST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const HYBRID_STATUS_INVALID_PARAMETER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const HYBRID_STATUS_OUTPUT_BUFFER_TOO_SMALL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const HYBRID_STATUS_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ID_FQDN: ::windows::core::PCSTR = ::windows::core::s!("2");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ID_IPV4_ADDR: ::windows::core::PCSTR = ::windows::core::s!("1");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ID_IPV6_ADDR: ::windows::core::PCSTR = ::windows::core::s!("5");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ID_USER_FQDN: ::windows::core::PCSTR = ::windows::core::s!("3");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_ATA_MINIPORT: u32 = 315444u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_ATA_PASS_THROUGH: u32 = 315436u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_ATA_PASS_THROUGH_DIRECT: u32 = 315440u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_IDE_PASS_THROUGH: u32 = 315432u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MINIPORT_PROCESS_SERVICE_IRP: u32 = 315448u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MINIPORT_SIGNATURE_DSM_GENERAL: ::windows::core::PCSTR = ::windows::core::s!("MPDSMGEN");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MINIPORT_SIGNATURE_DSM_NOTIFICATION: ::windows::core::PCSTR = ::windows::core::s!("MPDSM   ");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MINIPORT_SIGNATURE_ENDURANCE_INFO: ::windows::core::PCSTR = ::windows::core::s!("ENDURINF");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MINIPORT_SIGNATURE_FIRMWARE: ::windows::core::PCSTR = ::windows::core::s!("FIRMWARE");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MINIPORT_SIGNATURE_HYBRDISK: ::windows::core::PCSTR = ::windows::core::s!("HYBRDISK");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MINIPORT_SIGNATURE_QUERY_PHYSICAL_TOPOLOGY: ::windows::core::PCSTR = ::windows::core::s!("TOPOLOGY");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MINIPORT_SIGNATURE_QUERY_PROTOCOL: ::windows::core::PCSTR = ::windows::core::s!("PROTOCOL");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MINIPORT_SIGNATURE_QUERY_TEMPERATURE: ::windows::core::PCSTR = ::windows::core::s!("TEMPERAT");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MINIPORT_SIGNATURE_SCSIDISK: ::windows::core::PCSTR = ::windows::core::s!("SCSIDISK");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MINIPORT_SIGNATURE_SET_PROTOCOL: ::windows::core::PCSTR = ::windows::core::s!("SETPROTO");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MINIPORT_SIGNATURE_SET_TEMPERATURE_THRESHOLD: ::windows::core::PCSTR = ::windows::core::s!("SETTEMPT");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MPIO_PASS_THROUGH_PATH: u32 = 315452u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MPIO_PASS_THROUGH_PATH_DIRECT: u32 = 315456u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MPIO_PASS_THROUGH_PATH_DIRECT_EX: u32 = 315472u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_MPIO_PASS_THROUGH_PATH_EX: u32 = 315468u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_SCSI_BASE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_SCSI_FREE_DUMP_POINTERS: u32 = 266276u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_SCSI_GET_ADDRESS: u32 = 266264u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_SCSI_GET_CAPABILITIES: u32 = 266256u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_SCSI_GET_DUMP_POINTERS: u32 = 266272u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_SCSI_GET_INQUIRY_DATA: u32 = 266252u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_SCSI_MINIPORT: u32 = 315400u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_SCSI_PASS_THROUGH: u32 = 315396u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_SCSI_PASS_THROUGH_DIRECT: u32 = 315412u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_SCSI_PASS_THROUGH_DIRECT_EX: u32 = 315464u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_SCSI_PASS_THROUGH_EX: u32 = 315460u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IOCTL_SCSI_RESCAN_BUS: u32 = 266268u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_FLAG_ALLOW_PORTAL_HOPPING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_FLAG_MULTIPATH_ENABLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_FLAG_REQUIRE_IPSEC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_FLAG_RESERVED1: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_FLAG_USE_RADIUS_RESPONSE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_FLAG_USE_RADIUS_VERIFICATION: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_OPTIONS_AUTH_TYPE: ::windows::core::PCSTR = ::windows::core::s!("0x00000080");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_OPTIONS_DATA_DIGEST: ::windows::core::PCSTR = ::windows::core::s!("0x00000002");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_OPTIONS_DEFAULT_TIME_2_RETAIN: ::windows::core::PCSTR = ::windows::core::s!("0x00000010");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_OPTIONS_DEFAULT_TIME_2_WAIT: ::windows::core::PCSTR = ::windows::core::s!("0x00000008");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_OPTIONS_HEADER_DIGEST: ::windows::core::PCSTR = ::windows::core::s!("0x00000001");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_OPTIONS_MAXIMUM_CONNECTIONS: ::windows::core::PCSTR = ::windows::core::s!("0x00000004");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_OPTIONS_PASSWORD: ::windows::core::PCSTR = ::windows::core::s!("0x00000040");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_OPTIONS_USERNAME: ::windows::core::PCSTR = ::windows::core::s!("0x00000020");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_LOGIN_OPTIONS_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_SECURITY_FLAG_AGGRESSIVE_MODE_ENABLED: ::windows::core::PCSTR = ::windows::core::s!("0x00000008");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_SECURITY_FLAG_IKE_IPSEC_ENABLED: ::windows::core::PCSTR = ::windows::core::s!("0x00000002");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_SECURITY_FLAG_MAIN_MODE_ENABLED: ::windows::core::PCSTR = ::windows::core::s!("0x00000004");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_SECURITY_FLAG_PFS_ENABLED: ::windows::core::PCSTR = ::windows::core::s!("0x00000010");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_SECURITY_FLAG_TRANSPORT_MODE_PREFERRED: ::windows::core::PCSTR = ::windows::core::s!("0x00000020");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_SECURITY_FLAG_TUNNEL_MODE_PREFERRED: ::windows::core::PCSTR = ::windows::core::s!("0x00000040");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_SECURITY_FLAG_VALID: ::windows::core::PCSTR = ::windows::core::s!("0x00000001");
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_TARGET_FLAG_HIDE_STATIC_TARGET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_TARGET_FLAG_MERGE_TARGET_INFORMATION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MAX_ISCSI_ALIAS_LEN: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MAX_ISCSI_DISCOVERY_DOMAIN_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MAX_ISCSI_HBANAME_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MAX_ISCSI_NAME_LEN: u32 = 223u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MAX_ISCSI_PORTAL_ADDRESS_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MAX_ISCSI_PORTAL_ALIAS_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MAX_ISCSI_PORTAL_NAME_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MAX_ISCSI_TEXT_ADDRESS_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MAX_RADIUS_ADDRESS_LEN: u32 = 41u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MINIPORT_DSM_NOTIFICATION_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MINIPORT_DSM_NOTIFICATION_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MINIPORT_DSM_NOTIFY_FLAG_BEGIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MINIPORT_DSM_NOTIFY_FLAG_END: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MINIPORT_DSM_PROFILE_CRASHDUMP_FILE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MINIPORT_DSM_PROFILE_HIBERNATION_FILE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MINIPORT_DSM_PROFILE_PAGE_FILE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MINIPORT_DSM_PROFILE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MPIO_IOCTL_FLAG_INVOLVE_DSM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MPIO_IOCTL_FLAG_USE_PATHID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MPIO_IOCTL_FLAG_USE_SCSIADDRESS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_ADD_LBAS_PINNED_SET: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_FLUSH_NVCACHE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_NVCACHE_INFO: u32 = 236u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_NVCACHE_POWER_MODE_RETURN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_NVCACHE_POWER_MODE_SET: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_NVSEPARATED_FLUSH: u32 = 193u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_NVSEPARATED_INFO: u32 = 192u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_NVSEPARATED_WB_DISABLE: u32 = 194u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_NVSEPARATED_WB_REVERT_DEFAULT: u32 = 195u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_PASS_HINT_PAYLOAD: u32 = 224u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_QUERY_ASCENDER_STATUS: u32 = 208u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_QUERY_CACHE_MISS: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_QUERY_HYBRID_DISK_STATUS: u32 = 209u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_QUERY_PINNED_SET: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_REMOVE_LBAS_PINNED_SET: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_FUNCTION_SPINDLE_STATUS: u32 = 229u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_ILLEGAL_REQUEST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_INPUT_DATA_OVERRUN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_INPUT_DATA_UNDERRUN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_INVALID_PARAMETER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_OUTPUT_DATA_OVERRUN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_OUTPUT_DATA_UNDERRUN: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NRB_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NV_SEP_CACHE_PARAMETER_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NV_SEP_CACHE_PARAMETER_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const SCSI_IOCTL_DATA_BIDIRECTIONAL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const SCSI_IOCTL_DATA_IN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const SCSI_IOCTL_DATA_OUT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const SCSI_IOCTL_DATA_UNSPECIFIED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_DIAGNOSTIC_STATUS_BUFFER_TOO_SMALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_DIAGNOSTIC_STATUS_INVALID_PARAMETER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_DIAGNOSTIC_STATUS_INVALID_SIGNATURE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_DIAGNOSTIC_STATUS_INVALID_TARGET_TYPE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_DIAGNOSTIC_STATUS_MORE_DATA: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_DIAGNOSTIC_STATUS_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_DIAGNOSTIC_STATUS_UNSUPPORTED_VERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_FIRMWARE_ACTIVATE_STRUCTURE_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_FIRMWARE_DOWNLOAD_STRUCTURE_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_FIRMWARE_DOWNLOAD_STRUCTURE_VERSION_V2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_FIRMWARE_INFO_INVALID_SLOT: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_FIRMWARE_INFO_STRUCTURE_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_FIRMWARE_INFO_STRUCTURE_VERSION_V2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const STORAGE_FIRMWARE_SLOT_INFO_V2_REVISION_LENGTH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ScsiRawInterfaceGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53f56309_b6bf_11d0_94f2_00a0c91efb8b);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const WmiScsiAddressGuid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53f5630f_b6bf_11d0_94f2_00a0c91efb8b);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IKE_AUTHENTICATION_METHOD(pub i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const IKE_AUTHENTICATION_PRESHARED_KEY_METHOD: IKE_AUTHENTICATION_METHOD = IKE_AUTHENTICATION_METHOD(1i32);
impl ::core::marker::Copy for IKE_AUTHENTICATION_METHOD {}
impl ::core::clone::Clone for IKE_AUTHENTICATION_METHOD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IKE_AUTHENTICATION_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IKE_AUTHENTICATION_METHOD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IKE_AUTHENTICATION_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKE_AUTHENTICATION_METHOD").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ISCSI_AUTH_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_NO_AUTH_TYPE: ISCSI_AUTH_TYPES = ISCSI_AUTH_TYPES(0i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_CHAP_AUTH_TYPE: ISCSI_AUTH_TYPES = ISCSI_AUTH_TYPES(1i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_MUTUAL_CHAP_AUTH_TYPE: ISCSI_AUTH_TYPES = ISCSI_AUTH_TYPES(2i32);
impl ::core::marker::Copy for ISCSI_AUTH_TYPES {}
impl ::core::clone::Clone for ISCSI_AUTH_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ISCSI_AUTH_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ISCSI_AUTH_TYPES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ISCSI_AUTH_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISCSI_AUTH_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ISCSI_DIGEST_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_DIGEST_TYPE_NONE: ISCSI_DIGEST_TYPES = ISCSI_DIGEST_TYPES(0i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_DIGEST_TYPE_CRC32C: ISCSI_DIGEST_TYPES = ISCSI_DIGEST_TYPES(1i32);
impl ::core::marker::Copy for ISCSI_DIGEST_TYPES {}
impl ::core::clone::Clone for ISCSI_DIGEST_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ISCSI_DIGEST_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ISCSI_DIGEST_TYPES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ISCSI_DIGEST_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISCSI_DIGEST_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MP_STORAGE_DIAGNOSTIC_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MpStorageDiagnosticLevelDefault: MP_STORAGE_DIAGNOSTIC_LEVEL = MP_STORAGE_DIAGNOSTIC_LEVEL(0i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MpStorageDiagnosticLevelMax: MP_STORAGE_DIAGNOSTIC_LEVEL = MP_STORAGE_DIAGNOSTIC_LEVEL(1i32);
impl ::core::marker::Copy for MP_STORAGE_DIAGNOSTIC_LEVEL {}
impl ::core::clone::Clone for MP_STORAGE_DIAGNOSTIC_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MP_STORAGE_DIAGNOSTIC_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MP_STORAGE_DIAGNOSTIC_LEVEL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MP_STORAGE_DIAGNOSTIC_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MP_STORAGE_DIAGNOSTIC_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MP_STORAGE_DIAGNOSTIC_TARGET_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MpStorageDiagnosticTargetTypeUndefined: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE = MP_STORAGE_DIAGNOSTIC_TARGET_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MpStorageDiagnosticTargetTypeMiniport: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE = MP_STORAGE_DIAGNOSTIC_TARGET_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MpStorageDiagnosticTargetTypeHbaFirmware: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE = MP_STORAGE_DIAGNOSTIC_TARGET_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const MpStorageDiagnosticTargetTypeMax: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE = MP_STORAGE_DIAGNOSTIC_TARGET_TYPE(4i32);
impl ::core::marker::Copy for MP_STORAGE_DIAGNOSTIC_TARGET_TYPE {}
impl ::core::clone::Clone for MP_STORAGE_DIAGNOSTIC_TARGET_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MP_STORAGE_DIAGNOSTIC_TARGET_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MP_STORAGE_DIAGNOSTIC_TARGET_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MP_STORAGE_DIAGNOSTIC_TARGET_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MP_STORAGE_DIAGNOSTIC_TARGET_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NVCACHE_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NvCacheStatusUnknown: NVCACHE_STATUS = NVCACHE_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NvCacheStatusDisabling: NVCACHE_STATUS = NVCACHE_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NvCacheStatusDisabled: NVCACHE_STATUS = NVCACHE_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NvCacheStatusEnabled: NVCACHE_STATUS = NVCACHE_STATUS(3i32);
impl ::core::marker::Copy for NVCACHE_STATUS {}
impl ::core::clone::Clone for NVCACHE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NVCACHE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NVCACHE_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NVCACHE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NVCACHE_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NVCACHE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NvCacheTypeUnknown: NVCACHE_TYPE = NVCACHE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NvCacheTypeNone: NVCACHE_TYPE = NVCACHE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NvCacheTypeWriteBack: NVCACHE_TYPE = NVCACHE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NvCacheTypeWriteThrough: NVCACHE_TYPE = NVCACHE_TYPE(3i32);
impl ::core::marker::Copy for NVCACHE_TYPE {}
impl ::core::clone::Clone for NVCACHE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NVCACHE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NVCACHE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NVCACHE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NVCACHE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NV_SEP_WRITE_CACHE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NVSEPWriteCacheTypeUnknown: NV_SEP_WRITE_CACHE_TYPE = NV_SEP_WRITE_CACHE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NVSEPWriteCacheTypeNone: NV_SEP_WRITE_CACHE_TYPE = NV_SEP_WRITE_CACHE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NVSEPWriteCacheTypeWriteBack: NV_SEP_WRITE_CACHE_TYPE = NV_SEP_WRITE_CACHE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const NVSEPWriteCacheTypeWriteThrough: NV_SEP_WRITE_CACHE_TYPE = NV_SEP_WRITE_CACHE_TYPE(3i32);
impl ::core::marker::Copy for NV_SEP_WRITE_CACHE_TYPE {}
impl ::core::clone::Clone for NV_SEP_WRITE_CACHE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NV_SEP_WRITE_CACHE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NV_SEP_WRITE_CACHE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NV_SEP_WRITE_CACHE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NV_SEP_WRITE_CACHE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TARGETPROTOCOLTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ISCSI_TCP_PROTOCOL_TYPE: TARGETPROTOCOLTYPE = TARGETPROTOCOLTYPE(0i32);
impl ::core::marker::Copy for TARGETPROTOCOLTYPE {}
impl ::core::clone::Clone for TARGETPROTOCOLTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TARGETPROTOCOLTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TARGETPROTOCOLTYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TARGETPROTOCOLTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TARGETPROTOCOLTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TARGET_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const ProtocolType: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(0i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const TargetAlias: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(1i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const DiscoveryMechanisms: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(2i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const PortalGroups: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(3i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const PersistentTargetMappings: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(4i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const InitiatorName: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(5i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const TargetFlags: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(6i32);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub const LoginOptions: TARGET_INFORMATION_CLASS = TARGET_INFORMATION_CLASS(7i32);
impl ::core::marker::Copy for TARGET_INFORMATION_CLASS {}
impl ::core::clone::Clone for TARGET_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TARGET_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TARGET_INFORMATION_CLASS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TARGET_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TARGET_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ATA_PASS_THROUGH_DIRECT {
    pub Length: u16,
    pub AtaFlags: u16,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub ReservedAsUchar: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub ReservedAsUlong: u32,
    pub DataBuffer: *mut ::core::ffi::c_void,
    pub PreviousTaskFile: [u8; 8],
    pub CurrentTaskFile: [u8; 8],
}
impl ::core::marker::Copy for ATA_PASS_THROUGH_DIRECT {}
impl ::core::clone::Clone for ATA_PASS_THROUGH_DIRECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ATA_PASS_THROUGH_DIRECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATA_PASS_THROUGH_DIRECT")
            .field("Length", &self.Length)
            .field("AtaFlags", &self.AtaFlags)
            .field("PathId", &self.PathId)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .field("ReservedAsUchar", &self.ReservedAsUchar)
            .field("DataTransferLength", &self.DataTransferLength)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("ReservedAsUlong", &self.ReservedAsUlong)
            .field("DataBuffer", &self.DataBuffer)
            .field("PreviousTaskFile", &self.PreviousTaskFile)
            .field("CurrentTaskFile", &self.CurrentTaskFile)
            .finish()
    }
}
impl ::windows::core::TypeKind for ATA_PASS_THROUGH_DIRECT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ATA_PASS_THROUGH_DIRECT {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.AtaFlags == other.AtaFlags && self.PathId == other.PathId && self.TargetId == other.TargetId && self.Lun == other.Lun && self.ReservedAsUchar == other.ReservedAsUchar && self.DataTransferLength == other.DataTransferLength && self.TimeOutValue == other.TimeOutValue && self.ReservedAsUlong == other.ReservedAsUlong && self.DataBuffer == other.DataBuffer && self.PreviousTaskFile == other.PreviousTaskFile && self.CurrentTaskFile == other.CurrentTaskFile
    }
}
impl ::core::cmp::Eq for ATA_PASS_THROUGH_DIRECT {}
impl ::core::default::Default for ATA_PASS_THROUGH_DIRECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct ATA_PASS_THROUGH_DIRECT32 {
    pub Length: u16,
    pub AtaFlags: u16,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub ReservedAsUchar: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub ReservedAsUlong: u32,
    pub DataBuffer: *mut ::core::ffi::c_void,
    pub PreviousTaskFile: [u8; 8],
    pub CurrentTaskFile: [u8; 8],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for ATA_PASS_THROUGH_DIRECT32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for ATA_PASS_THROUGH_DIRECT32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for ATA_PASS_THROUGH_DIRECT32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATA_PASS_THROUGH_DIRECT32")
            .field("Length", &self.Length)
            .field("AtaFlags", &self.AtaFlags)
            .field("PathId", &self.PathId)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .field("ReservedAsUchar", &self.ReservedAsUchar)
            .field("DataTransferLength", &self.DataTransferLength)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("ReservedAsUlong", &self.ReservedAsUlong)
            .field("DataBuffer", &self.DataBuffer)
            .field("PreviousTaskFile", &self.PreviousTaskFile)
            .field("CurrentTaskFile", &self.CurrentTaskFile)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows::core::TypeKind for ATA_PASS_THROUGH_DIRECT32 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for ATA_PASS_THROUGH_DIRECT32 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.AtaFlags == other.AtaFlags && self.PathId == other.PathId && self.TargetId == other.TargetId && self.Lun == other.Lun && self.ReservedAsUchar == other.ReservedAsUchar && self.DataTransferLength == other.DataTransferLength && self.TimeOutValue == other.TimeOutValue && self.ReservedAsUlong == other.ReservedAsUlong && self.DataBuffer == other.DataBuffer && self.PreviousTaskFile == other.PreviousTaskFile && self.CurrentTaskFile == other.CurrentTaskFile
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for ATA_PASS_THROUGH_DIRECT32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for ATA_PASS_THROUGH_DIRECT32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ATA_PASS_THROUGH_EX {
    pub Length: u16,
    pub AtaFlags: u16,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub ReservedAsUchar: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub ReservedAsUlong: u32,
    pub DataBufferOffset: usize,
    pub PreviousTaskFile: [u8; 8],
    pub CurrentTaskFile: [u8; 8],
}
impl ::core::marker::Copy for ATA_PASS_THROUGH_EX {}
impl ::core::clone::Clone for ATA_PASS_THROUGH_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ATA_PASS_THROUGH_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATA_PASS_THROUGH_EX")
            .field("Length", &self.Length)
            .field("AtaFlags", &self.AtaFlags)
            .field("PathId", &self.PathId)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .field("ReservedAsUchar", &self.ReservedAsUchar)
            .field("DataTransferLength", &self.DataTransferLength)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("ReservedAsUlong", &self.ReservedAsUlong)
            .field("DataBufferOffset", &self.DataBufferOffset)
            .field("PreviousTaskFile", &self.PreviousTaskFile)
            .field("CurrentTaskFile", &self.CurrentTaskFile)
            .finish()
    }
}
impl ::windows::core::TypeKind for ATA_PASS_THROUGH_EX {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ATA_PASS_THROUGH_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.AtaFlags == other.AtaFlags && self.PathId == other.PathId && self.TargetId == other.TargetId && self.Lun == other.Lun && self.ReservedAsUchar == other.ReservedAsUchar && self.DataTransferLength == other.DataTransferLength && self.TimeOutValue == other.TimeOutValue && self.ReservedAsUlong == other.ReservedAsUlong && self.DataBufferOffset == other.DataBufferOffset && self.PreviousTaskFile == other.PreviousTaskFile && self.CurrentTaskFile == other.CurrentTaskFile
    }
}
impl ::core::cmp::Eq for ATA_PASS_THROUGH_EX {}
impl ::core::default::Default for ATA_PASS_THROUGH_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct ATA_PASS_THROUGH_EX32 {
    pub Length: u16,
    pub AtaFlags: u16,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub ReservedAsUchar: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub ReservedAsUlong: u32,
    pub DataBufferOffset: u32,
    pub PreviousTaskFile: [u8; 8],
    pub CurrentTaskFile: [u8; 8],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for ATA_PASS_THROUGH_EX32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for ATA_PASS_THROUGH_EX32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for ATA_PASS_THROUGH_EX32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATA_PASS_THROUGH_EX32")
            .field("Length", &self.Length)
            .field("AtaFlags", &self.AtaFlags)
            .field("PathId", &self.PathId)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .field("ReservedAsUchar", &self.ReservedAsUchar)
            .field("DataTransferLength", &self.DataTransferLength)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("ReservedAsUlong", &self.ReservedAsUlong)
            .field("DataBufferOffset", &self.DataBufferOffset)
            .field("PreviousTaskFile", &self.PreviousTaskFile)
            .field("CurrentTaskFile", &self.CurrentTaskFile)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows::core::TypeKind for ATA_PASS_THROUGH_EX32 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for ATA_PASS_THROUGH_EX32 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.AtaFlags == other.AtaFlags && self.PathId == other.PathId && self.TargetId == other.TargetId && self.Lun == other.Lun && self.ReservedAsUchar == other.ReservedAsUchar && self.DataTransferLength == other.DataTransferLength && self.TimeOutValue == other.TimeOutValue && self.ReservedAsUlong == other.ReservedAsUlong && self.DataBufferOffset == other.DataBufferOffset && self.PreviousTaskFile == other.PreviousTaskFile && self.CurrentTaskFile == other.CurrentTaskFile
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for ATA_PASS_THROUGH_EX32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for ATA_PASS_THROUGH_EX32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct DSM_NOTIFICATION_REQUEST_BLOCK {
    pub Size: u32,
    pub Version: u32,
    pub NotifyFlags: u32,
    pub DataSetProfile: u32,
    pub Reserved: [u32; 3],
    pub DataSetRangesCount: u32,
    pub DataSetRanges: [MP_DEVICE_DATA_SET_RANGE; 1],
}
impl ::core::marker::Copy for DSM_NOTIFICATION_REQUEST_BLOCK {}
impl ::core::clone::Clone for DSM_NOTIFICATION_REQUEST_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DSM_NOTIFICATION_REQUEST_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DSM_NOTIFICATION_REQUEST_BLOCK").field("Size", &self.Size).field("Version", &self.Version).field("NotifyFlags", &self.NotifyFlags).field("DataSetProfile", &self.DataSetProfile).field("Reserved", &self.Reserved).field("DataSetRangesCount", &self.DataSetRangesCount).field("DataSetRanges", &self.DataSetRanges).finish()
    }
}
impl ::windows::core::TypeKind for DSM_NOTIFICATION_REQUEST_BLOCK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DSM_NOTIFICATION_REQUEST_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.NotifyFlags == other.NotifyFlags && self.DataSetProfile == other.DataSetProfile && self.Reserved == other.Reserved && self.DataSetRangesCount == other.DataSetRangesCount && self.DataSetRanges == other.DataSetRanges
    }
}
impl ::core::cmp::Eq for DSM_NOTIFICATION_REQUEST_BLOCK {}
impl ::core::default::Default for DSM_NOTIFICATION_REQUEST_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct DUMP_DRIVER {
    pub DumpDriverList: *mut ::core::ffi::c_void,
    pub DriverName: [u16; 15],
    pub BaseName: [u16; 15],
}
impl ::core::marker::Copy for DUMP_DRIVER {}
impl ::core::clone::Clone for DUMP_DRIVER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DUMP_DRIVER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DUMP_DRIVER").field("DumpDriverList", &self.DumpDriverList).field("DriverName", &self.DriverName).field("BaseName", &self.BaseName).finish()
    }
}
impl ::windows::core::TypeKind for DUMP_DRIVER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DUMP_DRIVER {
    fn eq(&self, other: &Self) -> bool {
        self.DumpDriverList == other.DumpDriverList && self.DriverName == other.DriverName && self.BaseName == other.BaseName
    }
}
impl ::core::cmp::Eq for DUMP_DRIVER {}
impl ::core::default::Default for DUMP_DRIVER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct DUMP_DRIVER_EX {
    pub DumpDriverList: *mut ::core::ffi::c_void,
    pub DriverName: [u16; 15],
    pub BaseName: [u16; 15],
    pub DriverFullPath: NTSCSI_UNICODE_STRING,
}
impl ::core::marker::Copy for DUMP_DRIVER_EX {}
impl ::core::clone::Clone for DUMP_DRIVER_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DUMP_DRIVER_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DUMP_DRIVER_EX").field("DumpDriverList", &self.DumpDriverList).field("DriverName", &self.DriverName).field("BaseName", &self.BaseName).field("DriverFullPath", &self.DriverFullPath).finish()
    }
}
impl ::windows::core::TypeKind for DUMP_DRIVER_EX {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DUMP_DRIVER_EX {
    fn eq(&self, other: &Self) -> bool {
        self.DumpDriverList == other.DumpDriverList && self.DriverName == other.DriverName && self.BaseName == other.BaseName && self.DriverFullPath == other.DriverFullPath
    }
}
impl ::core::cmp::Eq for DUMP_DRIVER_EX {}
impl ::core::default::Default for DUMP_DRIVER_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DUMP_POINTERS {
    pub AdapterObject: *mut _ADAPTER_OBJECT,
    pub MappedRegisterBase: *mut ::core::ffi::c_void,
    pub DumpData: *mut ::core::ffi::c_void,
    pub CommonBufferVa: *mut ::core::ffi::c_void,
    pub CommonBufferPa: i64,
    pub CommonBufferSize: u32,
    pub AllocateCommonBuffers: super::super::Foundation::BOOLEAN,
    pub UseDiskDump: super::super::Foundation::BOOLEAN,
    pub Spare1: [u8; 2],
    pub DeviceObject: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DUMP_POINTERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DUMP_POINTERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DUMP_POINTERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DUMP_POINTERS")
            .field("AdapterObject", &self.AdapterObject)
            .field("MappedRegisterBase", &self.MappedRegisterBase)
            .field("DumpData", &self.DumpData)
            .field("CommonBufferVa", &self.CommonBufferVa)
            .field("CommonBufferPa", &self.CommonBufferPa)
            .field("CommonBufferSize", &self.CommonBufferSize)
            .field("AllocateCommonBuffers", &self.AllocateCommonBuffers)
            .field("UseDiskDump", &self.UseDiskDump)
            .field("Spare1", &self.Spare1)
            .field("DeviceObject", &self.DeviceObject)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DUMP_POINTERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DUMP_POINTERS {
    fn eq(&self, other: &Self) -> bool {
        self.AdapterObject == other.AdapterObject && self.MappedRegisterBase == other.MappedRegisterBase && self.DumpData == other.DumpData && self.CommonBufferVa == other.CommonBufferVa && self.CommonBufferPa == other.CommonBufferPa && self.CommonBufferSize == other.CommonBufferSize && self.AllocateCommonBuffers == other.AllocateCommonBuffers && self.UseDiskDump == other.UseDiskDump && self.Spare1 == other.Spare1 && self.DeviceObject == other.DeviceObject
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DUMP_POINTERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DUMP_POINTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DUMP_POINTERS_EX {
    pub Header: DUMP_POINTERS_VERSION,
    pub DumpData: *mut ::core::ffi::c_void,
    pub CommonBufferVa: *mut ::core::ffi::c_void,
    pub CommonBufferSize: u32,
    pub AllocateCommonBuffers: super::super::Foundation::BOOLEAN,
    pub DeviceObject: *mut ::core::ffi::c_void,
    pub DriverList: *mut ::core::ffi::c_void,
    pub dwPortFlags: u32,
    pub MaxDeviceDumpSectionSize: u32,
    pub MaxDeviceDumpLevel: u32,
    pub MaxTransferSize: u32,
    pub AdapterObject: *mut ::core::ffi::c_void,
    pub MappedRegisterBase: *mut ::core::ffi::c_void,
    pub DeviceReady: *mut super::super::Foundation::BOOLEAN,
    pub DumpDevicePowerOn: PDUMP_DEVICE_POWERON_ROUTINE,
    pub DumpDevicePowerOnContext: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DUMP_POINTERS_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DUMP_POINTERS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DUMP_POINTERS_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DUMP_POINTERS_EX")
            .field("Header", &self.Header)
            .field("DumpData", &self.DumpData)
            .field("CommonBufferVa", &self.CommonBufferVa)
            .field("CommonBufferSize", &self.CommonBufferSize)
            .field("AllocateCommonBuffers", &self.AllocateCommonBuffers)
            .field("DeviceObject", &self.DeviceObject)
            .field("DriverList", &self.DriverList)
            .field("dwPortFlags", &self.dwPortFlags)
            .field("MaxDeviceDumpSectionSize", &self.MaxDeviceDumpSectionSize)
            .field("MaxDeviceDumpLevel", &self.MaxDeviceDumpLevel)
            .field("MaxTransferSize", &self.MaxTransferSize)
            .field("AdapterObject", &self.AdapterObject)
            .field("MappedRegisterBase", &self.MappedRegisterBase)
            .field("DeviceReady", &self.DeviceReady)
            .field("DumpDevicePowerOnContext", &self.DumpDevicePowerOnContext)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DUMP_POINTERS_EX {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DUMP_POINTERS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct DUMP_POINTERS_VERSION {
    pub Version: u32,
    pub Size: u32,
}
impl ::core::marker::Copy for DUMP_POINTERS_VERSION {}
impl ::core::clone::Clone for DUMP_POINTERS_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DUMP_POINTERS_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DUMP_POINTERS_VERSION").field("Version", &self.Version).field("Size", &self.Size).finish()
    }
}
impl ::windows::core::TypeKind for DUMP_POINTERS_VERSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DUMP_POINTERS_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for DUMP_POINTERS_VERSION {}
impl ::core::default::Default for DUMP_POINTERS_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct FIRMWARE_REQUEST_BLOCK {
    pub Version: u32,
    pub Size: u32,
    pub Function: u32,
    pub Flags: u32,
    pub DataBufferOffset: u32,
    pub DataBufferLength: u32,
}
impl ::core::marker::Copy for FIRMWARE_REQUEST_BLOCK {}
impl ::core::clone::Clone for FIRMWARE_REQUEST_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FIRMWARE_REQUEST_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FIRMWARE_REQUEST_BLOCK").field("Version", &self.Version).field("Size", &self.Size).field("Function", &self.Function).field("Flags", &self.Flags).field("DataBufferOffset", &self.DataBufferOffset).field("DataBufferLength", &self.DataBufferLength).finish()
    }
}
impl ::windows::core::TypeKind for FIRMWARE_REQUEST_BLOCK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for FIRMWARE_REQUEST_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Function == other.Function && self.Flags == other.Flags && self.DataBufferOffset == other.DataBufferOffset && self.DataBufferLength == other.DataBufferLength
    }
}
impl ::core::cmp::Eq for FIRMWARE_REQUEST_BLOCK {}
impl ::core::default::Default for FIRMWARE_REQUEST_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct HYBRID_DEMOTE_BY_SIZE {
    pub Version: u32,
    pub Size: u32,
    pub SourcePriority: u8,
    pub TargetPriority: u8,
    pub Reserved0: u16,
    pub Reserved1: u32,
    pub LbaCount: u64,
}
impl ::core::marker::Copy for HYBRID_DEMOTE_BY_SIZE {}
impl ::core::clone::Clone for HYBRID_DEMOTE_BY_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HYBRID_DEMOTE_BY_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HYBRID_DEMOTE_BY_SIZE").field("Version", &self.Version).field("Size", &self.Size).field("SourcePriority", &self.SourcePriority).field("TargetPriority", &self.TargetPriority).field("Reserved0", &self.Reserved0).field("Reserved1", &self.Reserved1).field("LbaCount", &self.LbaCount).finish()
    }
}
impl ::windows::core::TypeKind for HYBRID_DEMOTE_BY_SIZE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HYBRID_DEMOTE_BY_SIZE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.SourcePriority == other.SourcePriority && self.TargetPriority == other.TargetPriority && self.Reserved0 == other.Reserved0 && self.Reserved1 == other.Reserved1 && self.LbaCount == other.LbaCount
    }
}
impl ::core::cmp::Eq for HYBRID_DEMOTE_BY_SIZE {}
impl ::core::default::Default for HYBRID_DEMOTE_BY_SIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct HYBRID_DIRTY_THRESHOLDS {
    pub Version: u32,
    pub Size: u32,
    pub DirtyLowThreshold: u32,
    pub DirtyHighThreshold: u32,
}
impl ::core::marker::Copy for HYBRID_DIRTY_THRESHOLDS {}
impl ::core::clone::Clone for HYBRID_DIRTY_THRESHOLDS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HYBRID_DIRTY_THRESHOLDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HYBRID_DIRTY_THRESHOLDS").field("Version", &self.Version).field("Size", &self.Size).field("DirtyLowThreshold", &self.DirtyLowThreshold).field("DirtyHighThreshold", &self.DirtyHighThreshold).finish()
    }
}
impl ::windows::core::TypeKind for HYBRID_DIRTY_THRESHOLDS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HYBRID_DIRTY_THRESHOLDS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.DirtyLowThreshold == other.DirtyLowThreshold && self.DirtyHighThreshold == other.DirtyHighThreshold
    }
}
impl ::core::cmp::Eq for HYBRID_DIRTY_THRESHOLDS {}
impl ::core::default::Default for HYBRID_DIRTY_THRESHOLDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HYBRID_INFORMATION {
    pub Version: u32,
    pub Size: u32,
    pub HybridSupported: super::super::Foundation::BOOLEAN,
    pub Status: NVCACHE_STATUS,
    pub CacheTypeEffective: NVCACHE_TYPE,
    pub CacheTypeDefault: NVCACHE_TYPE,
    pub FractionBase: u32,
    pub CacheSize: u64,
    pub Attributes: HYBRID_INFORMATION_0,
    pub Priorities: HYBRID_INFORMATION_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HYBRID_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HYBRID_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HYBRID_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HYBRID_INFORMATION").field("Version", &self.Version).field("Size", &self.Size).field("HybridSupported", &self.HybridSupported).field("Status", &self.Status).field("CacheTypeEffective", &self.CacheTypeEffective).field("CacheTypeDefault", &self.CacheTypeDefault).field("FractionBase", &self.FractionBase).field("CacheSize", &self.CacheSize).field("Attributes", &self.Attributes).field("Priorities", &self.Priorities).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for HYBRID_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HYBRID_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.HybridSupported == other.HybridSupported && self.Status == other.Status && self.CacheTypeEffective == other.CacheTypeEffective && self.CacheTypeDefault == other.CacheTypeDefault && self.FractionBase == other.FractionBase && self.CacheSize == other.CacheSize && self.Attributes == other.Attributes && self.Priorities == other.Priorities
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HYBRID_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HYBRID_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HYBRID_INFORMATION_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HYBRID_INFORMATION_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HYBRID_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HYBRID_INFORMATION_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HYBRID_INFORMATION_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for HYBRID_INFORMATION_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HYBRID_INFORMATION_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HYBRID_INFORMATION_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HYBRID_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HYBRID_INFORMATION_1 {
    pub PriorityLevelCount: u8,
    pub MaxPriorityBehavior: super::super::Foundation::BOOLEAN,
    pub OptimalWriteGranularity: u8,
    pub Reserved: u8,
    pub DirtyThresholdLow: u32,
    pub DirtyThresholdHigh: u32,
    pub SupportedCommands: HYBRID_INFORMATION_1_0,
    pub Priority: [NVCACHE_PRIORITY_LEVEL_DESCRIPTOR; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HYBRID_INFORMATION_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HYBRID_INFORMATION_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HYBRID_INFORMATION_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HYBRID_INFORMATION_1").field("PriorityLevelCount", &self.PriorityLevelCount).field("MaxPriorityBehavior", &self.MaxPriorityBehavior).field("OptimalWriteGranularity", &self.OptimalWriteGranularity).field("Reserved", &self.Reserved).field("DirtyThresholdLow", &self.DirtyThresholdLow).field("DirtyThresholdHigh", &self.DirtyThresholdHigh).field("SupportedCommands", &self.SupportedCommands).field("Priority", &self.Priority).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for HYBRID_INFORMATION_1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HYBRID_INFORMATION_1 {
    fn eq(&self, other: &Self) -> bool {
        self.PriorityLevelCount == other.PriorityLevelCount && self.MaxPriorityBehavior == other.MaxPriorityBehavior && self.OptimalWriteGranularity == other.OptimalWriteGranularity && self.Reserved == other.Reserved && self.DirtyThresholdLow == other.DirtyThresholdLow && self.DirtyThresholdHigh == other.DirtyThresholdHigh && self.SupportedCommands == other.SupportedCommands && self.Priority == other.Priority
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HYBRID_INFORMATION_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HYBRID_INFORMATION_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HYBRID_INFORMATION_1_0 {
    pub _bitfield: u32,
    pub MaxEvictCommands: u32,
    pub MaxLbaRangeCountForEvict: u32,
    pub MaxLbaRangeCountForChangeLba: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HYBRID_INFORMATION_1_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HYBRID_INFORMATION_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HYBRID_INFORMATION_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HYBRID_INFORMATION_1_0").field("_bitfield", &self._bitfield).field("MaxEvictCommands", &self.MaxEvictCommands).field("MaxLbaRangeCountForEvict", &self.MaxLbaRangeCountForEvict).field("MaxLbaRangeCountForChangeLba", &self.MaxLbaRangeCountForChangeLba).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for HYBRID_INFORMATION_1_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HYBRID_INFORMATION_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield && self.MaxEvictCommands == other.MaxEvictCommands && self.MaxLbaRangeCountForEvict == other.MaxLbaRangeCountForEvict && self.MaxLbaRangeCountForChangeLba == other.MaxLbaRangeCountForChangeLba
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HYBRID_INFORMATION_1_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HYBRID_INFORMATION_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct HYBRID_REQUEST_BLOCK {
    pub Version: u32,
    pub Size: u32,
    pub Function: u32,
    pub Flags: u32,
    pub DataBufferOffset: u32,
    pub DataBufferLength: u32,
}
impl ::core::marker::Copy for HYBRID_REQUEST_BLOCK {}
impl ::core::clone::Clone for HYBRID_REQUEST_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HYBRID_REQUEST_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HYBRID_REQUEST_BLOCK").field("Version", &self.Version).field("Size", &self.Size).field("Function", &self.Function).field("Flags", &self.Flags).field("DataBufferOffset", &self.DataBufferOffset).field("DataBufferLength", &self.DataBufferLength).finish()
    }
}
impl ::windows::core::TypeKind for HYBRID_REQUEST_BLOCK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HYBRID_REQUEST_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Function == other.Function && self.Flags == other.Flags && self.DataBufferOffset == other.DataBufferOffset && self.DataBufferLength == other.DataBufferLength
    }
}
impl ::core::cmp::Eq for HYBRID_REQUEST_BLOCK {}
impl ::core::default::Default for HYBRID_REQUEST_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct IDE_IO_CONTROL {
    pub HeaderLength: u32,
    pub Signature: [u8; 8],
    pub Timeout: u32,
    pub ControlCode: u32,
    pub ReturnStatus: u32,
    pub DataLength: u32,
}
impl ::core::marker::Copy for IDE_IO_CONTROL {}
impl ::core::clone::Clone for IDE_IO_CONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IDE_IO_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IDE_IO_CONTROL").field("HeaderLength", &self.HeaderLength).field("Signature", &self.Signature).field("Timeout", &self.Timeout).field("ControlCode", &self.ControlCode).field("ReturnStatus", &self.ReturnStatus).field("DataLength", &self.DataLength).finish()
    }
}
impl ::windows::core::TypeKind for IDE_IO_CONTROL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IDE_IO_CONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.HeaderLength == other.HeaderLength && self.Signature == other.Signature && self.Timeout == other.Timeout && self.ControlCode == other.ControlCode && self.ReturnStatus == other.ReturnStatus && self.DataLength == other.DataLength
    }
}
impl ::core::cmp::Eq for IDE_IO_CONTROL {}
impl ::core::default::Default for IDE_IO_CONTROL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct IKE_AUTHENTICATION_INFORMATION {
    pub AuthMethod: IKE_AUTHENTICATION_METHOD,
    pub Anonymous: IKE_AUTHENTICATION_INFORMATION_0,
}
impl ::core::marker::Copy for IKE_AUTHENTICATION_INFORMATION {}
impl ::core::clone::Clone for IKE_AUTHENTICATION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IKE_AUTHENTICATION_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IKE_AUTHENTICATION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub union IKE_AUTHENTICATION_INFORMATION_0 {
    pub PsKey: IKE_AUTHENTICATION_PRESHARED_KEY,
}
impl ::core::marker::Copy for IKE_AUTHENTICATION_INFORMATION_0 {}
impl ::core::clone::Clone for IKE_AUTHENTICATION_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for IKE_AUTHENTICATION_INFORMATION_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for IKE_AUTHENTICATION_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct IKE_AUTHENTICATION_PRESHARED_KEY {
    pub SecurityFlags: u64,
    pub IdType: u8,
    pub IdLengthInBytes: u32,
    pub Id: *mut u8,
    pub KeyLengthInBytes: u32,
    pub Key: *mut u8,
}
impl ::core::marker::Copy for IKE_AUTHENTICATION_PRESHARED_KEY {}
impl ::core::clone::Clone for IKE_AUTHENTICATION_PRESHARED_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKE_AUTHENTICATION_PRESHARED_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKE_AUTHENTICATION_PRESHARED_KEY").field("SecurityFlags", &self.SecurityFlags).field("IdType", &self.IdType).field("IdLengthInBytes", &self.IdLengthInBytes).field("Id", &self.Id).field("KeyLengthInBytes", &self.KeyLengthInBytes).field("Key", &self.Key).finish()
    }
}
impl ::windows::core::TypeKind for IKE_AUTHENTICATION_PRESHARED_KEY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IKE_AUTHENTICATION_PRESHARED_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityFlags == other.SecurityFlags && self.IdType == other.IdType && self.IdLengthInBytes == other.IdLengthInBytes && self.Id == other.Id && self.KeyLengthInBytes == other.KeyLengthInBytes && self.Key == other.Key
    }
}
impl ::core::cmp::Eq for IKE_AUTHENTICATION_PRESHARED_KEY {}
impl ::core::default::Default for IKE_AUTHENTICATION_PRESHARED_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IO_SCSI_CAPABILITIES {
    pub Length: u32,
    pub MaximumTransferLength: u32,
    pub MaximumPhysicalPages: u32,
    pub SupportedAsynchronousEvents: u32,
    pub AlignmentMask: u32,
    pub TaggedQueuing: super::super::Foundation::BOOLEAN,
    pub AdapterScansDown: super::super::Foundation::BOOLEAN,
    pub AdapterUsesPio: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IO_SCSI_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IO_SCSI_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IO_SCSI_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IO_SCSI_CAPABILITIES").field("Length", &self.Length).field("MaximumTransferLength", &self.MaximumTransferLength).field("MaximumPhysicalPages", &self.MaximumPhysicalPages).field("SupportedAsynchronousEvents", &self.SupportedAsynchronousEvents).field("AlignmentMask", &self.AlignmentMask).field("TaggedQueuing", &self.TaggedQueuing).field("AdapterScansDown", &self.AdapterScansDown).field("AdapterUsesPio", &self.AdapterUsesPio).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for IO_SCSI_CAPABILITIES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IO_SCSI_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.MaximumTransferLength == other.MaximumTransferLength && self.MaximumPhysicalPages == other.MaximumPhysicalPages && self.SupportedAsynchronousEvents == other.SupportedAsynchronousEvents && self.AlignmentMask == other.AlignmentMask && self.TaggedQueuing == other.TaggedQueuing && self.AdapterScansDown == other.AdapterScansDown && self.AdapterUsesPio == other.AdapterUsesPio
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IO_SCSI_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IO_SCSI_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_CONNECTION_INFOA {
    pub ConnectionId: ISCSI_UNIQUE_SESSION_ID,
    pub InitiatorAddress: ::windows::core::PSTR,
    pub TargetAddress: ::windows::core::PSTR,
    pub InitiatorSocket: u16,
    pub TargetSocket: u16,
    pub CID: [u8; 2],
}
impl ::core::marker::Copy for ISCSI_CONNECTION_INFOA {}
impl ::core::clone::Clone for ISCSI_CONNECTION_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ISCSI_CONNECTION_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_CONNECTION_INFOA").field("ConnectionId", &self.ConnectionId).field("InitiatorAddress", &self.InitiatorAddress).field("TargetAddress", &self.TargetAddress).field("InitiatorSocket", &self.InitiatorSocket).field("TargetSocket", &self.TargetSocket).field("CID", &self.CID).finish()
    }
}
impl ::windows::core::TypeKind for ISCSI_CONNECTION_INFOA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ISCSI_CONNECTION_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.ConnectionId == other.ConnectionId && self.InitiatorAddress == other.InitiatorAddress && self.TargetAddress == other.TargetAddress && self.InitiatorSocket == other.InitiatorSocket && self.TargetSocket == other.TargetSocket && self.CID == other.CID
    }
}
impl ::core::cmp::Eq for ISCSI_CONNECTION_INFOA {}
impl ::core::default::Default for ISCSI_CONNECTION_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_CONNECTION_INFOW {
    pub ConnectionId: ISCSI_UNIQUE_SESSION_ID,
    pub InitiatorAddress: ::windows::core::PWSTR,
    pub TargetAddress: ::windows::core::PWSTR,
    pub InitiatorSocket: u16,
    pub TargetSocket: u16,
    pub CID: [u8; 2],
}
impl ::core::marker::Copy for ISCSI_CONNECTION_INFOW {}
impl ::core::clone::Clone for ISCSI_CONNECTION_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ISCSI_CONNECTION_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_CONNECTION_INFOW").field("ConnectionId", &self.ConnectionId).field("InitiatorAddress", &self.InitiatorAddress).field("TargetAddress", &self.TargetAddress).field("InitiatorSocket", &self.InitiatorSocket).field("TargetSocket", &self.TargetSocket).field("CID", &self.CID).finish()
    }
}
impl ::windows::core::TypeKind for ISCSI_CONNECTION_INFOW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ISCSI_CONNECTION_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.ConnectionId == other.ConnectionId && self.InitiatorAddress == other.InitiatorAddress && self.TargetAddress == other.TargetAddress && self.InitiatorSocket == other.InitiatorSocket && self.TargetSocket == other.TargetSocket && self.CID == other.CID
    }
}
impl ::core::cmp::Eq for ISCSI_CONNECTION_INFOW {}
impl ::core::default::Default for ISCSI_CONNECTION_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_CONNECTION_INFO_EX {
    pub ConnectionId: ISCSI_UNIQUE_SESSION_ID,
    pub State: u8,
    pub Protocol: u8,
    pub HeaderDigest: u8,
    pub DataDigest: u8,
    pub MaxRecvDataSegmentLength: u32,
    pub AuthType: ISCSI_AUTH_TYPES,
    pub EstimatedThroughput: u64,
    pub MaxDatagramSize: u32,
}
impl ::core::marker::Copy for ISCSI_CONNECTION_INFO_EX {}
impl ::core::clone::Clone for ISCSI_CONNECTION_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ISCSI_CONNECTION_INFO_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_CONNECTION_INFO_EX").field("ConnectionId", &self.ConnectionId).field("State", &self.State).field("Protocol", &self.Protocol).field("HeaderDigest", &self.HeaderDigest).field("DataDigest", &self.DataDigest).field("MaxRecvDataSegmentLength", &self.MaxRecvDataSegmentLength).field("AuthType", &self.AuthType).field("EstimatedThroughput", &self.EstimatedThroughput).field("MaxDatagramSize", &self.MaxDatagramSize).finish()
    }
}
impl ::windows::core::TypeKind for ISCSI_CONNECTION_INFO_EX {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ISCSI_CONNECTION_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.ConnectionId == other.ConnectionId && self.State == other.State && self.Protocol == other.Protocol && self.HeaderDigest == other.HeaderDigest && self.DataDigest == other.DataDigest && self.MaxRecvDataSegmentLength == other.MaxRecvDataSegmentLength && self.AuthType == other.AuthType && self.EstimatedThroughput == other.EstimatedThroughput && self.MaxDatagramSize == other.MaxDatagramSize
    }
}
impl ::core::cmp::Eq for ISCSI_CONNECTION_INFO_EX {}
impl ::core::default::Default for ISCSI_CONNECTION_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_System_Ioctl\"`*"]
#[cfg(feature = "Win32_System_Ioctl")]
pub struct ISCSI_DEVICE_ON_SESSIONA {
    pub InitiatorName: [u8; 256],
    pub TargetName: [u8; 224],
    pub ScsiAddress: SCSI_ADDRESS,
    pub DeviceInterfaceType: ::windows::core::GUID,
    pub DeviceInterfaceName: [u8; 260],
    pub LegacyName: [u8; 260],
    pub StorageDeviceNumber: super::super::System::Ioctl::STORAGE_DEVICE_NUMBER,
    pub DeviceInstance: u32,
}
#[cfg(feature = "Win32_System_Ioctl")]
impl ::core::marker::Copy for ISCSI_DEVICE_ON_SESSIONA {}
#[cfg(feature = "Win32_System_Ioctl")]
impl ::core::clone::Clone for ISCSI_DEVICE_ON_SESSIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Ioctl")]
impl ::core::fmt::Debug for ISCSI_DEVICE_ON_SESSIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_DEVICE_ON_SESSIONA").field("InitiatorName", &self.InitiatorName).field("TargetName", &self.TargetName).field("ScsiAddress", &self.ScsiAddress).field("DeviceInterfaceType", &self.DeviceInterfaceType).field("DeviceInterfaceName", &self.DeviceInterfaceName).field("LegacyName", &self.LegacyName).field("StorageDeviceNumber", &self.StorageDeviceNumber).field("DeviceInstance", &self.DeviceInstance).finish()
    }
}
#[cfg(feature = "Win32_System_Ioctl")]
impl ::windows::core::TypeKind for ISCSI_DEVICE_ON_SESSIONA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Ioctl")]
impl ::core::cmp::PartialEq for ISCSI_DEVICE_ON_SESSIONA {
    fn eq(&self, other: &Self) -> bool {
        self.InitiatorName == other.InitiatorName && self.TargetName == other.TargetName && self.ScsiAddress == other.ScsiAddress && self.DeviceInterfaceType == other.DeviceInterfaceType && self.DeviceInterfaceName == other.DeviceInterfaceName && self.LegacyName == other.LegacyName && self.StorageDeviceNumber == other.StorageDeviceNumber && self.DeviceInstance == other.DeviceInstance
    }
}
#[cfg(feature = "Win32_System_Ioctl")]
impl ::core::cmp::Eq for ISCSI_DEVICE_ON_SESSIONA {}
#[cfg(feature = "Win32_System_Ioctl")]
impl ::core::default::Default for ISCSI_DEVICE_ON_SESSIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_System_Ioctl\"`*"]
#[cfg(feature = "Win32_System_Ioctl")]
pub struct ISCSI_DEVICE_ON_SESSIONW {
    pub InitiatorName: [u16; 256],
    pub TargetName: [u16; 224],
    pub ScsiAddress: SCSI_ADDRESS,
    pub DeviceInterfaceType: ::windows::core::GUID,
    pub DeviceInterfaceName: [u16; 260],
    pub LegacyName: [u16; 260],
    pub StorageDeviceNumber: super::super::System::Ioctl::STORAGE_DEVICE_NUMBER,
    pub DeviceInstance: u32,
}
#[cfg(feature = "Win32_System_Ioctl")]
impl ::core::marker::Copy for ISCSI_DEVICE_ON_SESSIONW {}
#[cfg(feature = "Win32_System_Ioctl")]
impl ::core::clone::Clone for ISCSI_DEVICE_ON_SESSIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Ioctl")]
impl ::core::fmt::Debug for ISCSI_DEVICE_ON_SESSIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_DEVICE_ON_SESSIONW").field("InitiatorName", &self.InitiatorName).field("TargetName", &self.TargetName).field("ScsiAddress", &self.ScsiAddress).field("DeviceInterfaceType", &self.DeviceInterfaceType).field("DeviceInterfaceName", &self.DeviceInterfaceName).field("LegacyName", &self.LegacyName).field("StorageDeviceNumber", &self.StorageDeviceNumber).field("DeviceInstance", &self.DeviceInstance).finish()
    }
}
#[cfg(feature = "Win32_System_Ioctl")]
impl ::windows::core::TypeKind for ISCSI_DEVICE_ON_SESSIONW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Ioctl")]
impl ::core::cmp::PartialEq for ISCSI_DEVICE_ON_SESSIONW {
    fn eq(&self, other: &Self) -> bool {
        self.InitiatorName == other.InitiatorName && self.TargetName == other.TargetName && self.ScsiAddress == other.ScsiAddress && self.DeviceInterfaceType == other.DeviceInterfaceType && self.DeviceInterfaceName == other.DeviceInterfaceName && self.LegacyName == other.LegacyName && self.StorageDeviceNumber == other.StorageDeviceNumber && self.DeviceInstance == other.DeviceInstance
    }
}
#[cfg(feature = "Win32_System_Ioctl")]
impl ::core::cmp::Eq for ISCSI_DEVICE_ON_SESSIONW {}
#[cfg(feature = "Win32_System_Ioctl")]
impl ::core::default::Default for ISCSI_DEVICE_ON_SESSIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_LOGIN_OPTIONS {
    pub Version: u32,
    pub InformationSpecified: u32,
    pub LoginFlags: u32,
    pub AuthType: ISCSI_AUTH_TYPES,
    pub HeaderDigest: ISCSI_DIGEST_TYPES,
    pub DataDigest: ISCSI_DIGEST_TYPES,
    pub MaximumConnections: u32,
    pub DefaultTime2Wait: u32,
    pub DefaultTime2Retain: u32,
    pub UsernameLength: u32,
    pub PasswordLength: u32,
    pub Username: *mut u8,
    pub Password: *mut u8,
}
impl ::core::marker::Copy for ISCSI_LOGIN_OPTIONS {}
impl ::core::clone::Clone for ISCSI_LOGIN_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ISCSI_LOGIN_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_LOGIN_OPTIONS")
            .field("Version", &self.Version)
            .field("InformationSpecified", &self.InformationSpecified)
            .field("LoginFlags", &self.LoginFlags)
            .field("AuthType", &self.AuthType)
            .field("HeaderDigest", &self.HeaderDigest)
            .field("DataDigest", &self.DataDigest)
            .field("MaximumConnections", &self.MaximumConnections)
            .field("DefaultTime2Wait", &self.DefaultTime2Wait)
            .field("DefaultTime2Retain", &self.DefaultTime2Retain)
            .field("UsernameLength", &self.UsernameLength)
            .field("PasswordLength", &self.PasswordLength)
            .field("Username", &self.Username)
            .field("Password", &self.Password)
            .finish()
    }
}
impl ::windows::core::TypeKind for ISCSI_LOGIN_OPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ISCSI_LOGIN_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.InformationSpecified == other.InformationSpecified && self.LoginFlags == other.LoginFlags && self.AuthType == other.AuthType && self.HeaderDigest == other.HeaderDigest && self.DataDigest == other.DataDigest && self.MaximumConnections == other.MaximumConnections && self.DefaultTime2Wait == other.DefaultTime2Wait && self.DefaultTime2Retain == other.DefaultTime2Retain && self.UsernameLength == other.UsernameLength && self.PasswordLength == other.PasswordLength && self.Username == other.Username && self.Password == other.Password
    }
}
impl ::core::cmp::Eq for ISCSI_LOGIN_OPTIONS {}
impl ::core::default::Default for ISCSI_LOGIN_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_SESSION_INFOA {
    pub SessionId: ISCSI_UNIQUE_SESSION_ID,
    pub InitiatorName: ::windows::core::PSTR,
    pub TargetNodeName: ::windows::core::PSTR,
    pub TargetName: ::windows::core::PSTR,
    pub ISID: [u8; 6],
    pub TSID: [u8; 2],
    pub ConnectionCount: u32,
    pub Connections: *mut ISCSI_CONNECTION_INFOA,
}
impl ::core::marker::Copy for ISCSI_SESSION_INFOA {}
impl ::core::clone::Clone for ISCSI_SESSION_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ISCSI_SESSION_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_SESSION_INFOA").field("SessionId", &self.SessionId).field("InitiatorName", &self.InitiatorName).field("TargetNodeName", &self.TargetNodeName).field("TargetName", &self.TargetName).field("ISID", &self.ISID).field("TSID", &self.TSID).field("ConnectionCount", &self.ConnectionCount).field("Connections", &self.Connections).finish()
    }
}
impl ::windows::core::TypeKind for ISCSI_SESSION_INFOA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ISCSI_SESSION_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId && self.InitiatorName == other.InitiatorName && self.TargetNodeName == other.TargetNodeName && self.TargetName == other.TargetName && self.ISID == other.ISID && self.TSID == other.TSID && self.ConnectionCount == other.ConnectionCount && self.Connections == other.Connections
    }
}
impl ::core::cmp::Eq for ISCSI_SESSION_INFOA {}
impl ::core::default::Default for ISCSI_SESSION_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_SESSION_INFOW {
    pub SessionId: ISCSI_UNIQUE_SESSION_ID,
    pub InitiatorName: ::windows::core::PWSTR,
    pub TargetNodeName: ::windows::core::PWSTR,
    pub TargetName: ::windows::core::PWSTR,
    pub ISID: [u8; 6],
    pub TSID: [u8; 2],
    pub ConnectionCount: u32,
    pub Connections: *mut ISCSI_CONNECTION_INFOW,
}
impl ::core::marker::Copy for ISCSI_SESSION_INFOW {}
impl ::core::clone::Clone for ISCSI_SESSION_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ISCSI_SESSION_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_SESSION_INFOW").field("SessionId", &self.SessionId).field("InitiatorName", &self.InitiatorName).field("TargetNodeName", &self.TargetNodeName).field("TargetName", &self.TargetName).field("ISID", &self.ISID).field("TSID", &self.TSID).field("ConnectionCount", &self.ConnectionCount).field("Connections", &self.Connections).finish()
    }
}
impl ::windows::core::TypeKind for ISCSI_SESSION_INFOW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ISCSI_SESSION_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId && self.InitiatorName == other.InitiatorName && self.TargetNodeName == other.TargetNodeName && self.TargetName == other.TargetName && self.ISID == other.ISID && self.TSID == other.TSID && self.ConnectionCount == other.ConnectionCount && self.Connections == other.Connections
    }
}
impl ::core::cmp::Eq for ISCSI_SESSION_INFOW {}
impl ::core::default::Default for ISCSI_SESSION_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ISCSI_SESSION_INFO_EX {
    pub SessionId: ISCSI_UNIQUE_SESSION_ID,
    pub InitialR2t: super::super::Foundation::BOOLEAN,
    pub ImmediateData: super::super::Foundation::BOOLEAN,
    pub Type: u8,
    pub DataSequenceInOrder: super::super::Foundation::BOOLEAN,
    pub DataPduInOrder: super::super::Foundation::BOOLEAN,
    pub ErrorRecoveryLevel: u8,
    pub MaxOutstandingR2t: u32,
    pub FirstBurstLength: u32,
    pub MaxBurstLength: u32,
    pub MaximumConnections: u32,
    pub ConnectionCount: u32,
    pub Connections: *mut ISCSI_CONNECTION_INFO_EX,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ISCSI_SESSION_INFO_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ISCSI_SESSION_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ISCSI_SESSION_INFO_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_SESSION_INFO_EX")
            .field("SessionId", &self.SessionId)
            .field("InitialR2t", &self.InitialR2t)
            .field("ImmediateData", &self.ImmediateData)
            .field("Type", &self.Type)
            .field("DataSequenceInOrder", &self.DataSequenceInOrder)
            .field("DataPduInOrder", &self.DataPduInOrder)
            .field("ErrorRecoveryLevel", &self.ErrorRecoveryLevel)
            .field("MaxOutstandingR2t", &self.MaxOutstandingR2t)
            .field("FirstBurstLength", &self.FirstBurstLength)
            .field("MaxBurstLength", &self.MaxBurstLength)
            .field("MaximumConnections", &self.MaximumConnections)
            .field("ConnectionCount", &self.ConnectionCount)
            .field("Connections", &self.Connections)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for ISCSI_SESSION_INFO_EX {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ISCSI_SESSION_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId && self.InitialR2t == other.InitialR2t && self.ImmediateData == other.ImmediateData && self.Type == other.Type && self.DataSequenceInOrder == other.DataSequenceInOrder && self.DataPduInOrder == other.DataPduInOrder && self.ErrorRecoveryLevel == other.ErrorRecoveryLevel && self.MaxOutstandingR2t == other.MaxOutstandingR2t && self.FirstBurstLength == other.FirstBurstLength && self.MaxBurstLength == other.MaxBurstLength && self.MaximumConnections == other.MaximumConnections && self.ConnectionCount == other.ConnectionCount && self.Connections == other.Connections
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ISCSI_SESSION_INFO_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ISCSI_SESSION_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_TARGET_MAPPINGA {
    pub InitiatorName: [u8; 256],
    pub TargetName: [u8; 224],
    pub OSDeviceName: [u8; 260],
    pub SessionId: ISCSI_UNIQUE_SESSION_ID,
    pub OSBusNumber: u32,
    pub OSTargetNumber: u32,
    pub LUNCount: u32,
    pub LUNList: *mut SCSI_LUN_LIST,
}
impl ::core::marker::Copy for ISCSI_TARGET_MAPPINGA {}
impl ::core::clone::Clone for ISCSI_TARGET_MAPPINGA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ISCSI_TARGET_MAPPINGA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_TARGET_MAPPINGA").field("InitiatorName", &self.InitiatorName).field("TargetName", &self.TargetName).field("OSDeviceName", &self.OSDeviceName).field("SessionId", &self.SessionId).field("OSBusNumber", &self.OSBusNumber).field("OSTargetNumber", &self.OSTargetNumber).field("LUNCount", &self.LUNCount).field("LUNList", &self.LUNList).finish()
    }
}
impl ::windows::core::TypeKind for ISCSI_TARGET_MAPPINGA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ISCSI_TARGET_MAPPINGA {
    fn eq(&self, other: &Self) -> bool {
        self.InitiatorName == other.InitiatorName && self.TargetName == other.TargetName && self.OSDeviceName == other.OSDeviceName && self.SessionId == other.SessionId && self.OSBusNumber == other.OSBusNumber && self.OSTargetNumber == other.OSTargetNumber && self.LUNCount == other.LUNCount && self.LUNList == other.LUNList
    }
}
impl ::core::cmp::Eq for ISCSI_TARGET_MAPPINGA {}
impl ::core::default::Default for ISCSI_TARGET_MAPPINGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_TARGET_MAPPINGW {
    pub InitiatorName: [u16; 256],
    pub TargetName: [u16; 224],
    pub OSDeviceName: [u16; 260],
    pub SessionId: ISCSI_UNIQUE_SESSION_ID,
    pub OSBusNumber: u32,
    pub OSTargetNumber: u32,
    pub LUNCount: u32,
    pub LUNList: *mut SCSI_LUN_LIST,
}
impl ::core::marker::Copy for ISCSI_TARGET_MAPPINGW {}
impl ::core::clone::Clone for ISCSI_TARGET_MAPPINGW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ISCSI_TARGET_MAPPINGW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_TARGET_MAPPINGW").field("InitiatorName", &self.InitiatorName).field("TargetName", &self.TargetName).field("OSDeviceName", &self.OSDeviceName).field("SessionId", &self.SessionId).field("OSBusNumber", &self.OSBusNumber).field("OSTargetNumber", &self.OSTargetNumber).field("LUNCount", &self.LUNCount).field("LUNList", &self.LUNList).finish()
    }
}
impl ::windows::core::TypeKind for ISCSI_TARGET_MAPPINGW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ISCSI_TARGET_MAPPINGW {
    fn eq(&self, other: &Self) -> bool {
        self.InitiatorName == other.InitiatorName && self.TargetName == other.TargetName && self.OSDeviceName == other.OSDeviceName && self.SessionId == other.SessionId && self.OSBusNumber == other.OSBusNumber && self.OSTargetNumber == other.OSTargetNumber && self.LUNCount == other.LUNCount && self.LUNList == other.LUNList
    }
}
impl ::core::cmp::Eq for ISCSI_TARGET_MAPPINGW {}
impl ::core::default::Default for ISCSI_TARGET_MAPPINGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_TARGET_PORTALA {
    pub SymbolicName: [u8; 256],
    pub Address: [u8; 256],
    pub Socket: u16,
}
impl ::core::marker::Copy for ISCSI_TARGET_PORTALA {}
impl ::core::clone::Clone for ISCSI_TARGET_PORTALA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ISCSI_TARGET_PORTALA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_TARGET_PORTALA").field("SymbolicName", &self.SymbolicName).field("Address", &self.Address).field("Socket", &self.Socket).finish()
    }
}
impl ::windows::core::TypeKind for ISCSI_TARGET_PORTALA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ISCSI_TARGET_PORTALA {
    fn eq(&self, other: &Self) -> bool {
        self.SymbolicName == other.SymbolicName && self.Address == other.Address && self.Socket == other.Socket
    }
}
impl ::core::cmp::Eq for ISCSI_TARGET_PORTALA {}
impl ::core::default::Default for ISCSI_TARGET_PORTALA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_TARGET_PORTALW {
    pub SymbolicName: [u16; 256],
    pub Address: [u16; 256],
    pub Socket: u16,
}
impl ::core::marker::Copy for ISCSI_TARGET_PORTALW {}
impl ::core::clone::Clone for ISCSI_TARGET_PORTALW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ISCSI_TARGET_PORTALW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_TARGET_PORTALW").field("SymbolicName", &self.SymbolicName).field("Address", &self.Address).field("Socket", &self.Socket).finish()
    }
}
impl ::windows::core::TypeKind for ISCSI_TARGET_PORTALW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ISCSI_TARGET_PORTALW {
    fn eq(&self, other: &Self) -> bool {
        self.SymbolicName == other.SymbolicName && self.Address == other.Address && self.Socket == other.Socket
    }
}
impl ::core::cmp::Eq for ISCSI_TARGET_PORTALW {}
impl ::core::default::Default for ISCSI_TARGET_PORTALW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_TARGET_PORTAL_GROUPA {
    pub Count: u32,
    pub Portals: [ISCSI_TARGET_PORTALA; 1],
}
impl ::core::marker::Copy for ISCSI_TARGET_PORTAL_GROUPA {}
impl ::core::clone::Clone for ISCSI_TARGET_PORTAL_GROUPA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ISCSI_TARGET_PORTAL_GROUPA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_TARGET_PORTAL_GROUPA").field("Count", &self.Count).field("Portals", &self.Portals).finish()
    }
}
impl ::windows::core::TypeKind for ISCSI_TARGET_PORTAL_GROUPA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ISCSI_TARGET_PORTAL_GROUPA {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Portals == other.Portals
    }
}
impl ::core::cmp::Eq for ISCSI_TARGET_PORTAL_GROUPA {}
impl ::core::default::Default for ISCSI_TARGET_PORTAL_GROUPA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_TARGET_PORTAL_GROUPW {
    pub Count: u32,
    pub Portals: [ISCSI_TARGET_PORTALW; 1],
}
impl ::core::marker::Copy for ISCSI_TARGET_PORTAL_GROUPW {}
impl ::core::clone::Clone for ISCSI_TARGET_PORTAL_GROUPW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ISCSI_TARGET_PORTAL_GROUPW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_TARGET_PORTAL_GROUPW").field("Count", &self.Count).field("Portals", &self.Portals).finish()
    }
}
impl ::windows::core::TypeKind for ISCSI_TARGET_PORTAL_GROUPW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ISCSI_TARGET_PORTAL_GROUPW {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Portals == other.Portals
    }
}
impl ::core::cmp::Eq for ISCSI_TARGET_PORTAL_GROUPW {}
impl ::core::default::Default for ISCSI_TARGET_PORTAL_GROUPW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_TARGET_PORTAL_INFOA {
    pub InitiatorName: [u8; 256],
    pub InitiatorPortNumber: u32,
    pub SymbolicName: [u8; 256],
    pub Address: [u8; 256],
    pub Socket: u16,
}
impl ::core::marker::Copy for ISCSI_TARGET_PORTAL_INFOA {}
impl ::core::clone::Clone for ISCSI_TARGET_PORTAL_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ISCSI_TARGET_PORTAL_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_TARGET_PORTAL_INFOA").field("InitiatorName", &self.InitiatorName).field("InitiatorPortNumber", &self.InitiatorPortNumber).field("SymbolicName", &self.SymbolicName).field("Address", &self.Address).field("Socket", &self.Socket).finish()
    }
}
impl ::windows::core::TypeKind for ISCSI_TARGET_PORTAL_INFOA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ISCSI_TARGET_PORTAL_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.InitiatorName == other.InitiatorName && self.InitiatorPortNumber == other.InitiatorPortNumber && self.SymbolicName == other.SymbolicName && self.Address == other.Address && self.Socket == other.Socket
    }
}
impl ::core::cmp::Eq for ISCSI_TARGET_PORTAL_INFOA {}
impl ::core::default::Default for ISCSI_TARGET_PORTAL_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_TARGET_PORTAL_INFOW {
    pub InitiatorName: [u16; 256],
    pub InitiatorPortNumber: u32,
    pub SymbolicName: [u16; 256],
    pub Address: [u16; 256],
    pub Socket: u16,
}
impl ::core::marker::Copy for ISCSI_TARGET_PORTAL_INFOW {}
impl ::core::clone::Clone for ISCSI_TARGET_PORTAL_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ISCSI_TARGET_PORTAL_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_TARGET_PORTAL_INFOW").field("InitiatorName", &self.InitiatorName).field("InitiatorPortNumber", &self.InitiatorPortNumber).field("SymbolicName", &self.SymbolicName).field("Address", &self.Address).field("Socket", &self.Socket).finish()
    }
}
impl ::windows::core::TypeKind for ISCSI_TARGET_PORTAL_INFOW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ISCSI_TARGET_PORTAL_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.InitiatorName == other.InitiatorName && self.InitiatorPortNumber == other.InitiatorPortNumber && self.SymbolicName == other.SymbolicName && self.Address == other.Address && self.Socket == other.Socket
    }
}
impl ::core::cmp::Eq for ISCSI_TARGET_PORTAL_INFOW {}
impl ::core::default::Default for ISCSI_TARGET_PORTAL_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_TARGET_PORTAL_INFO_EXA {
    pub InitiatorName: [u8; 256],
    pub InitiatorPortNumber: u32,
    pub SymbolicName: [u8; 256],
    pub Address: [u8; 256],
    pub Socket: u16,
    pub SecurityFlags: u64,
    pub LoginOptions: ISCSI_LOGIN_OPTIONS,
}
impl ::core::marker::Copy for ISCSI_TARGET_PORTAL_INFO_EXA {}
impl ::core::clone::Clone for ISCSI_TARGET_PORTAL_INFO_EXA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ISCSI_TARGET_PORTAL_INFO_EXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_TARGET_PORTAL_INFO_EXA").field("InitiatorName", &self.InitiatorName).field("InitiatorPortNumber", &self.InitiatorPortNumber).field("SymbolicName", &self.SymbolicName).field("Address", &self.Address).field("Socket", &self.Socket).field("SecurityFlags", &self.SecurityFlags).field("LoginOptions", &self.LoginOptions).finish()
    }
}
impl ::windows::core::TypeKind for ISCSI_TARGET_PORTAL_INFO_EXA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ISCSI_TARGET_PORTAL_INFO_EXA {
    fn eq(&self, other: &Self) -> bool {
        self.InitiatorName == other.InitiatorName && self.InitiatorPortNumber == other.InitiatorPortNumber && self.SymbolicName == other.SymbolicName && self.Address == other.Address && self.Socket == other.Socket && self.SecurityFlags == other.SecurityFlags && self.LoginOptions == other.LoginOptions
    }
}
impl ::core::cmp::Eq for ISCSI_TARGET_PORTAL_INFO_EXA {}
impl ::core::default::Default for ISCSI_TARGET_PORTAL_INFO_EXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_TARGET_PORTAL_INFO_EXW {
    pub InitiatorName: [u16; 256],
    pub InitiatorPortNumber: u32,
    pub SymbolicName: [u16; 256],
    pub Address: [u16; 256],
    pub Socket: u16,
    pub SecurityFlags: u64,
    pub LoginOptions: ISCSI_LOGIN_OPTIONS,
}
impl ::core::marker::Copy for ISCSI_TARGET_PORTAL_INFO_EXW {}
impl ::core::clone::Clone for ISCSI_TARGET_PORTAL_INFO_EXW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ISCSI_TARGET_PORTAL_INFO_EXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_TARGET_PORTAL_INFO_EXW").field("InitiatorName", &self.InitiatorName).field("InitiatorPortNumber", &self.InitiatorPortNumber).field("SymbolicName", &self.SymbolicName).field("Address", &self.Address).field("Socket", &self.Socket).field("SecurityFlags", &self.SecurityFlags).field("LoginOptions", &self.LoginOptions).finish()
    }
}
impl ::windows::core::TypeKind for ISCSI_TARGET_PORTAL_INFO_EXW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ISCSI_TARGET_PORTAL_INFO_EXW {
    fn eq(&self, other: &Self) -> bool {
        self.InitiatorName == other.InitiatorName && self.InitiatorPortNumber == other.InitiatorPortNumber && self.SymbolicName == other.SymbolicName && self.Address == other.Address && self.Socket == other.Socket && self.SecurityFlags == other.SecurityFlags && self.LoginOptions == other.LoginOptions
    }
}
impl ::core::cmp::Eq for ISCSI_TARGET_PORTAL_INFO_EXW {}
impl ::core::default::Default for ISCSI_TARGET_PORTAL_INFO_EXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_UNIQUE_SESSION_ID {
    pub AdapterUnique: u64,
    pub AdapterSpecific: u64,
}
impl ::core::marker::Copy for ISCSI_UNIQUE_SESSION_ID {}
impl ::core::clone::Clone for ISCSI_UNIQUE_SESSION_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ISCSI_UNIQUE_SESSION_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_UNIQUE_SESSION_ID").field("AdapterUnique", &self.AdapterUnique).field("AdapterSpecific", &self.AdapterSpecific).finish()
    }
}
impl ::windows::core::TypeKind for ISCSI_UNIQUE_SESSION_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ISCSI_UNIQUE_SESSION_ID {
    fn eq(&self, other: &Self) -> bool {
        self.AdapterUnique == other.AdapterUnique && self.AdapterSpecific == other.AdapterSpecific
    }
}
impl ::core::cmp::Eq for ISCSI_UNIQUE_SESSION_ID {}
impl ::core::default::Default for ISCSI_UNIQUE_SESSION_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct ISCSI_VERSION_INFO {
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub BuildNumber: u32,
}
impl ::core::marker::Copy for ISCSI_VERSION_INFO {}
impl ::core::clone::Clone for ISCSI_VERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ISCSI_VERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ISCSI_VERSION_INFO").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("BuildNumber", &self.BuildNumber).finish()
    }
}
impl ::windows::core::TypeKind for ISCSI_VERSION_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ISCSI_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.BuildNumber == other.BuildNumber
    }
}
impl ::core::cmp::Eq for ISCSI_VERSION_INFO {}
impl ::core::default::Default for ISCSI_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct MPIO_PASS_THROUGH_PATH {
    pub PassThrough: SCSI_PASS_THROUGH,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
impl ::core::marker::Copy for MPIO_PASS_THROUGH_PATH {}
impl ::core::clone::Clone for MPIO_PASS_THROUGH_PATH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MPIO_PASS_THROUGH_PATH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPIO_PASS_THROUGH_PATH").field("PassThrough", &self.PassThrough).field("Version", &self.Version).field("Length", &self.Length).field("Flags", &self.Flags).field("PortNumber", &self.PortNumber).field("MpioPathId", &self.MpioPathId).finish()
    }
}
impl ::windows::core::TypeKind for MPIO_PASS_THROUGH_PATH {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MPIO_PASS_THROUGH_PATH {
    fn eq(&self, other: &Self) -> bool {
        self.PassThrough == other.PassThrough && self.Version == other.Version && self.Length == other.Length && self.Flags == other.Flags && self.PortNumber == other.PortNumber && self.MpioPathId == other.MpioPathId
    }
}
impl ::core::cmp::Eq for MPIO_PASS_THROUGH_PATH {}
impl ::core::default::Default for MPIO_PASS_THROUGH_PATH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct MPIO_PASS_THROUGH_PATH32 {
    pub PassThrough: SCSI_PASS_THROUGH32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for MPIO_PASS_THROUGH_PATH32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for MPIO_PASS_THROUGH_PATH32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for MPIO_PASS_THROUGH_PATH32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPIO_PASS_THROUGH_PATH32").field("PassThrough", &self.PassThrough).field("Version", &self.Version).field("Length", &self.Length).field("Flags", &self.Flags).field("PortNumber", &self.PortNumber).field("MpioPathId", &self.MpioPathId).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows::core::TypeKind for MPIO_PASS_THROUGH_PATH32 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for MPIO_PASS_THROUGH_PATH32 {
    fn eq(&self, other: &Self) -> bool {
        self.PassThrough == other.PassThrough && self.Version == other.Version && self.Length == other.Length && self.Flags == other.Flags && self.PortNumber == other.PortNumber && self.MpioPathId == other.MpioPathId
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for MPIO_PASS_THROUGH_PATH32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for MPIO_PASS_THROUGH_PATH32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct MPIO_PASS_THROUGH_PATH32_EX {
    pub PassThroughOffset: u32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for MPIO_PASS_THROUGH_PATH32_EX {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for MPIO_PASS_THROUGH_PATH32_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for MPIO_PASS_THROUGH_PATH32_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPIO_PASS_THROUGH_PATH32_EX").field("PassThroughOffset", &self.PassThroughOffset).field("Version", &self.Version).field("Length", &self.Length).field("Flags", &self.Flags).field("PortNumber", &self.PortNumber).field("MpioPathId", &self.MpioPathId).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows::core::TypeKind for MPIO_PASS_THROUGH_PATH32_EX {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for MPIO_PASS_THROUGH_PATH32_EX {
    fn eq(&self, other: &Self) -> bool {
        self.PassThroughOffset == other.PassThroughOffset && self.Version == other.Version && self.Length == other.Length && self.Flags == other.Flags && self.PortNumber == other.PortNumber && self.MpioPathId == other.MpioPathId
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for MPIO_PASS_THROUGH_PATH32_EX {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for MPIO_PASS_THROUGH_PATH32_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct MPIO_PASS_THROUGH_PATH_DIRECT {
    pub PassThrough: SCSI_PASS_THROUGH_DIRECT,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
impl ::core::marker::Copy for MPIO_PASS_THROUGH_PATH_DIRECT {}
impl ::core::clone::Clone for MPIO_PASS_THROUGH_PATH_DIRECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MPIO_PASS_THROUGH_PATH_DIRECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPIO_PASS_THROUGH_PATH_DIRECT").field("PassThrough", &self.PassThrough).field("Version", &self.Version).field("Length", &self.Length).field("Flags", &self.Flags).field("PortNumber", &self.PortNumber).field("MpioPathId", &self.MpioPathId).finish()
    }
}
impl ::windows::core::TypeKind for MPIO_PASS_THROUGH_PATH_DIRECT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MPIO_PASS_THROUGH_PATH_DIRECT {
    fn eq(&self, other: &Self) -> bool {
        self.PassThrough == other.PassThrough && self.Version == other.Version && self.Length == other.Length && self.Flags == other.Flags && self.PortNumber == other.PortNumber && self.MpioPathId == other.MpioPathId
    }
}
impl ::core::cmp::Eq for MPIO_PASS_THROUGH_PATH_DIRECT {}
impl ::core::default::Default for MPIO_PASS_THROUGH_PATH_DIRECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct MPIO_PASS_THROUGH_PATH_DIRECT32 {
    pub PassThrough: SCSI_PASS_THROUGH_DIRECT32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for MPIO_PASS_THROUGH_PATH_DIRECT32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for MPIO_PASS_THROUGH_PATH_DIRECT32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for MPIO_PASS_THROUGH_PATH_DIRECT32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPIO_PASS_THROUGH_PATH_DIRECT32").field("PassThrough", &self.PassThrough).field("Version", &self.Version).field("Length", &self.Length).field("Flags", &self.Flags).field("PortNumber", &self.PortNumber).field("MpioPathId", &self.MpioPathId).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows::core::TypeKind for MPIO_PASS_THROUGH_PATH_DIRECT32 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for MPIO_PASS_THROUGH_PATH_DIRECT32 {
    fn eq(&self, other: &Self) -> bool {
        self.PassThrough == other.PassThrough && self.Version == other.Version && self.Length == other.Length && self.Flags == other.Flags && self.PortNumber == other.PortNumber && self.MpioPathId == other.MpioPathId
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for MPIO_PASS_THROUGH_PATH_DIRECT32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for MPIO_PASS_THROUGH_PATH_DIRECT32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct MPIO_PASS_THROUGH_PATH_DIRECT32_EX {
    pub PassThroughOffset: u32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for MPIO_PASS_THROUGH_PATH_DIRECT32_EX {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for MPIO_PASS_THROUGH_PATH_DIRECT32_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for MPIO_PASS_THROUGH_PATH_DIRECT32_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPIO_PASS_THROUGH_PATH_DIRECT32_EX").field("PassThroughOffset", &self.PassThroughOffset).field("Version", &self.Version).field("Length", &self.Length).field("Flags", &self.Flags).field("PortNumber", &self.PortNumber).field("MpioPathId", &self.MpioPathId).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows::core::TypeKind for MPIO_PASS_THROUGH_PATH_DIRECT32_EX {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for MPIO_PASS_THROUGH_PATH_DIRECT32_EX {
    fn eq(&self, other: &Self) -> bool {
        self.PassThroughOffset == other.PassThroughOffset && self.Version == other.Version && self.Length == other.Length && self.Flags == other.Flags && self.PortNumber == other.PortNumber && self.MpioPathId == other.MpioPathId
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for MPIO_PASS_THROUGH_PATH_DIRECT32_EX {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for MPIO_PASS_THROUGH_PATH_DIRECT32_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct MPIO_PASS_THROUGH_PATH_DIRECT_EX {
    pub PassThroughOffset: u32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
impl ::core::marker::Copy for MPIO_PASS_THROUGH_PATH_DIRECT_EX {}
impl ::core::clone::Clone for MPIO_PASS_THROUGH_PATH_DIRECT_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MPIO_PASS_THROUGH_PATH_DIRECT_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPIO_PASS_THROUGH_PATH_DIRECT_EX").field("PassThroughOffset", &self.PassThroughOffset).field("Version", &self.Version).field("Length", &self.Length).field("Flags", &self.Flags).field("PortNumber", &self.PortNumber).field("MpioPathId", &self.MpioPathId).finish()
    }
}
impl ::windows::core::TypeKind for MPIO_PASS_THROUGH_PATH_DIRECT_EX {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MPIO_PASS_THROUGH_PATH_DIRECT_EX {
    fn eq(&self, other: &Self) -> bool {
        self.PassThroughOffset == other.PassThroughOffset && self.Version == other.Version && self.Length == other.Length && self.Flags == other.Flags && self.PortNumber == other.PortNumber && self.MpioPathId == other.MpioPathId
    }
}
impl ::core::cmp::Eq for MPIO_PASS_THROUGH_PATH_DIRECT_EX {}
impl ::core::default::Default for MPIO_PASS_THROUGH_PATH_DIRECT_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct MPIO_PASS_THROUGH_PATH_EX {
    pub PassThroughOffset: u32,
    pub Version: u32,
    pub Length: u16,
    pub Flags: u8,
    pub PortNumber: u8,
    pub MpioPathId: u64,
}
impl ::core::marker::Copy for MPIO_PASS_THROUGH_PATH_EX {}
impl ::core::clone::Clone for MPIO_PASS_THROUGH_PATH_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MPIO_PASS_THROUGH_PATH_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPIO_PASS_THROUGH_PATH_EX").field("PassThroughOffset", &self.PassThroughOffset).field("Version", &self.Version).field("Length", &self.Length).field("Flags", &self.Flags).field("PortNumber", &self.PortNumber).field("MpioPathId", &self.MpioPathId).finish()
    }
}
impl ::windows::core::TypeKind for MPIO_PASS_THROUGH_PATH_EX {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MPIO_PASS_THROUGH_PATH_EX {
    fn eq(&self, other: &Self) -> bool {
        self.PassThroughOffset == other.PassThroughOffset && self.Version == other.Version && self.Length == other.Length && self.Flags == other.Flags && self.PortNumber == other.PortNumber && self.MpioPathId == other.MpioPathId
    }
}
impl ::core::cmp::Eq for MPIO_PASS_THROUGH_PATH_EX {}
impl ::core::default::Default for MPIO_PASS_THROUGH_PATH_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct MP_DEVICE_DATA_SET_RANGE {
    pub StartingOffset: i64,
    pub LengthInBytes: u64,
}
impl ::core::marker::Copy for MP_DEVICE_DATA_SET_RANGE {}
impl ::core::clone::Clone for MP_DEVICE_DATA_SET_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MP_DEVICE_DATA_SET_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MP_DEVICE_DATA_SET_RANGE").field("StartingOffset", &self.StartingOffset).field("LengthInBytes", &self.LengthInBytes).finish()
    }
}
impl ::windows::core::TypeKind for MP_DEVICE_DATA_SET_RANGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MP_DEVICE_DATA_SET_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.StartingOffset == other.StartingOffset && self.LengthInBytes == other.LengthInBytes
    }
}
impl ::core::cmp::Eq for MP_DEVICE_DATA_SET_RANGE {}
impl ::core::default::Default for MP_DEVICE_DATA_SET_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct NTSCSI_UNICODE_STRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for NTSCSI_UNICODE_STRING {}
impl ::core::clone::Clone for NTSCSI_UNICODE_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NTSCSI_UNICODE_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTSCSI_UNICODE_STRING").field("Length", &self.Length).field("MaximumLength", &self.MaximumLength).field("Buffer", &self.Buffer).finish()
    }
}
impl ::windows::core::TypeKind for NTSCSI_UNICODE_STRING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NTSCSI_UNICODE_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.MaximumLength == other.MaximumLength && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for NTSCSI_UNICODE_STRING {}
impl ::core::default::Default for NTSCSI_UNICODE_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct NVCACHE_HINT_PAYLOAD {
    pub Command: u8,
    pub Feature7_0: u8,
    pub Feature15_8: u8,
    pub Count15_8: u8,
    pub LBA7_0: u8,
    pub LBA15_8: u8,
    pub LBA23_16: u8,
    pub LBA31_24: u8,
    pub LBA39_32: u8,
    pub LBA47_40: u8,
    pub Auxiliary7_0: u8,
    pub Auxiliary23_16: u8,
    pub Reserved: [u8; 4],
}
impl ::core::marker::Copy for NVCACHE_HINT_PAYLOAD {}
impl ::core::clone::Clone for NVCACHE_HINT_PAYLOAD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NVCACHE_HINT_PAYLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NVCACHE_HINT_PAYLOAD")
            .field("Command", &self.Command)
            .field("Feature7_0", &self.Feature7_0)
            .field("Feature15_8", &self.Feature15_8)
            .field("Count15_8", &self.Count15_8)
            .field("LBA7_0", &self.LBA7_0)
            .field("LBA15_8", &self.LBA15_8)
            .field("LBA23_16", &self.LBA23_16)
            .field("LBA31_24", &self.LBA31_24)
            .field("LBA39_32", &self.LBA39_32)
            .field("LBA47_40", &self.LBA47_40)
            .field("Auxiliary7_0", &self.Auxiliary7_0)
            .field("Auxiliary23_16", &self.Auxiliary23_16)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::windows::core::TypeKind for NVCACHE_HINT_PAYLOAD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NVCACHE_HINT_PAYLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.Command == other.Command && self.Feature7_0 == other.Feature7_0 && self.Feature15_8 == other.Feature15_8 && self.Count15_8 == other.Count15_8 && self.LBA7_0 == other.LBA7_0 && self.LBA15_8 == other.LBA15_8 && self.LBA23_16 == other.LBA23_16 && self.LBA31_24 == other.LBA31_24 && self.LBA39_32 == other.LBA39_32 && self.LBA47_40 == other.LBA47_40 && self.Auxiliary7_0 == other.Auxiliary7_0 && self.Auxiliary23_16 == other.Auxiliary23_16 && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for NVCACHE_HINT_PAYLOAD {}
impl ::core::default::Default for NVCACHE_HINT_PAYLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {
    pub PriorityLevel: u8,
    pub Reserved0: [u8; 3],
    pub ConsumedNVMSizeFraction: u32,
    pub ConsumedMappingResourcesFraction: u32,
    pub ConsumedNVMSizeForDirtyDataFraction: u32,
    pub ConsumedMappingResourcesForDirtyDataFraction: u32,
    pub Reserved1: u32,
}
impl ::core::marker::Copy for NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {}
impl ::core::clone::Clone for NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NVCACHE_PRIORITY_LEVEL_DESCRIPTOR")
            .field("PriorityLevel", &self.PriorityLevel)
            .field("Reserved0", &self.Reserved0)
            .field("ConsumedNVMSizeFraction", &self.ConsumedNVMSizeFraction)
            .field("ConsumedMappingResourcesFraction", &self.ConsumedMappingResourcesFraction)
            .field("ConsumedNVMSizeForDirtyDataFraction", &self.ConsumedNVMSizeForDirtyDataFraction)
            .field("ConsumedMappingResourcesForDirtyDataFraction", &self.ConsumedMappingResourcesForDirtyDataFraction)
            .field("Reserved1", &self.Reserved1)
            .finish()
    }
}
impl ::windows::core::TypeKind for NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.PriorityLevel == other.PriorityLevel && self.Reserved0 == other.Reserved0 && self.ConsumedNVMSizeFraction == other.ConsumedNVMSizeFraction && self.ConsumedMappingResourcesFraction == other.ConsumedMappingResourcesFraction && self.ConsumedNVMSizeForDirtyDataFraction == other.ConsumedNVMSizeForDirtyDataFraction && self.ConsumedMappingResourcesForDirtyDataFraction == other.ConsumedMappingResourcesForDirtyDataFraction && self.Reserved1 == other.Reserved1
    }
}
impl ::core::cmp::Eq for NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {}
impl ::core::default::Default for NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct NVCACHE_REQUEST_BLOCK {
    pub NRBSize: u32,
    pub Function: u16,
    pub NRBFlags: u32,
    pub NRBStatus: u32,
    pub Count: u32,
    pub LBA: u64,
    pub DataBufSize: u32,
    pub NVCacheStatus: u32,
    pub NVCacheSubStatus: u32,
}
impl ::core::marker::Copy for NVCACHE_REQUEST_BLOCK {}
impl ::core::clone::Clone for NVCACHE_REQUEST_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NVCACHE_REQUEST_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NVCACHE_REQUEST_BLOCK").field("NRBSize", &self.NRBSize).field("Function", &self.Function).field("NRBFlags", &self.NRBFlags).field("NRBStatus", &self.NRBStatus).field("Count", &self.Count).field("LBA", &self.LBA).field("DataBufSize", &self.DataBufSize).field("NVCacheStatus", &self.NVCacheStatus).field("NVCacheSubStatus", &self.NVCacheSubStatus).finish()
    }
}
impl ::windows::core::TypeKind for NVCACHE_REQUEST_BLOCK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NVCACHE_REQUEST_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.NRBSize == other.NRBSize && self.Function == other.Function && self.NRBFlags == other.NRBFlags && self.NRBStatus == other.NRBStatus && self.Count == other.Count && self.LBA == other.LBA && self.DataBufSize == other.DataBufSize && self.NVCacheStatus == other.NVCacheStatus && self.NVCacheSubStatus == other.NVCacheSubStatus
    }
}
impl ::core::cmp::Eq for NVCACHE_REQUEST_BLOCK {}
impl ::core::default::Default for NVCACHE_REQUEST_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct NV_FEATURE_PARAMETER {
    pub NVPowerModeEnabled: u16,
    pub NVParameterReserv1: u16,
    pub NVCmdEnabled: u16,
    pub NVParameterReserv2: u16,
    pub NVPowerModeVer: u16,
    pub NVCmdVer: u16,
    pub NVSize: u32,
    pub NVReadSpeed: u16,
    pub NVWrtSpeed: u16,
    pub DeviceSpinUpTime: u32,
}
impl ::core::marker::Copy for NV_FEATURE_PARAMETER {}
impl ::core::clone::Clone for NV_FEATURE_PARAMETER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NV_FEATURE_PARAMETER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NV_FEATURE_PARAMETER")
            .field("NVPowerModeEnabled", &self.NVPowerModeEnabled)
            .field("NVParameterReserv1", &self.NVParameterReserv1)
            .field("NVCmdEnabled", &self.NVCmdEnabled)
            .field("NVParameterReserv2", &self.NVParameterReserv2)
            .field("NVPowerModeVer", &self.NVPowerModeVer)
            .field("NVCmdVer", &self.NVCmdVer)
            .field("NVSize", &self.NVSize)
            .field("NVReadSpeed", &self.NVReadSpeed)
            .field("NVWrtSpeed", &self.NVWrtSpeed)
            .field("DeviceSpinUpTime", &self.DeviceSpinUpTime)
            .finish()
    }
}
impl ::windows::core::TypeKind for NV_FEATURE_PARAMETER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NV_FEATURE_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        self.NVPowerModeEnabled == other.NVPowerModeEnabled && self.NVParameterReserv1 == other.NVParameterReserv1 && self.NVCmdEnabled == other.NVCmdEnabled && self.NVParameterReserv2 == other.NVParameterReserv2 && self.NVPowerModeVer == other.NVPowerModeVer && self.NVCmdVer == other.NVCmdVer && self.NVSize == other.NVSize && self.NVReadSpeed == other.NVReadSpeed && self.NVWrtSpeed == other.NVWrtSpeed && self.DeviceSpinUpTime == other.DeviceSpinUpTime
    }
}
impl ::core::cmp::Eq for NV_FEATURE_PARAMETER {}
impl ::core::default::Default for NV_FEATURE_PARAMETER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct NV_SEP_CACHE_PARAMETER {
    pub Version: u32,
    pub Size: u32,
    pub Flags: NV_SEP_CACHE_PARAMETER_0,
    pub WriteCacheType: u8,
    pub WriteCacheTypeEffective: u8,
    pub ParameterReserve1: [u8; 3],
}
impl ::core::marker::Copy for NV_SEP_CACHE_PARAMETER {}
impl ::core::clone::Clone for NV_SEP_CACHE_PARAMETER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for NV_SEP_CACHE_PARAMETER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for NV_SEP_CACHE_PARAMETER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub union NV_SEP_CACHE_PARAMETER_0 {
    pub CacheFlags: NV_SEP_CACHE_PARAMETER_0_0,
    pub CacheFlagsSet: u8,
}
impl ::core::marker::Copy for NV_SEP_CACHE_PARAMETER_0 {}
impl ::core::clone::Clone for NV_SEP_CACHE_PARAMETER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for NV_SEP_CACHE_PARAMETER_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for NV_SEP_CACHE_PARAMETER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct NV_SEP_CACHE_PARAMETER_0_0 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for NV_SEP_CACHE_PARAMETER_0_0 {}
impl ::core::clone::Clone for NV_SEP_CACHE_PARAMETER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NV_SEP_CACHE_PARAMETER_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NV_SEP_CACHE_PARAMETER_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for NV_SEP_CACHE_PARAMETER_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for NV_SEP_CACHE_PARAMETER_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NV_SEP_CACHE_PARAMETER_0_0 {}
impl ::core::default::Default for NV_SEP_CACHE_PARAMETER_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PERSISTENT_ISCSI_LOGIN_INFOA {
    pub TargetName: [u8; 224],
    pub IsInformationalSession: super::super::Foundation::BOOLEAN,
    pub InitiatorInstance: [u8; 256],
    pub InitiatorPortNumber: u32,
    pub TargetPortal: ISCSI_TARGET_PORTALA,
    pub SecurityFlags: u64,
    pub Mappings: *mut ISCSI_TARGET_MAPPINGA,
    pub LoginOptions: ISCSI_LOGIN_OPTIONS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PERSISTENT_ISCSI_LOGIN_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PERSISTENT_ISCSI_LOGIN_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PERSISTENT_ISCSI_LOGIN_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERSISTENT_ISCSI_LOGIN_INFOA").field("TargetName", &self.TargetName).field("IsInformationalSession", &self.IsInformationalSession).field("InitiatorInstance", &self.InitiatorInstance).field("InitiatorPortNumber", &self.InitiatorPortNumber).field("TargetPortal", &self.TargetPortal).field("SecurityFlags", &self.SecurityFlags).field("Mappings", &self.Mappings).field("LoginOptions", &self.LoginOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for PERSISTENT_ISCSI_LOGIN_INFOA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PERSISTENT_ISCSI_LOGIN_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.TargetName == other.TargetName && self.IsInformationalSession == other.IsInformationalSession && self.InitiatorInstance == other.InitiatorInstance && self.InitiatorPortNumber == other.InitiatorPortNumber && self.TargetPortal == other.TargetPortal && self.SecurityFlags == other.SecurityFlags && self.Mappings == other.Mappings && self.LoginOptions == other.LoginOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PERSISTENT_ISCSI_LOGIN_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PERSISTENT_ISCSI_LOGIN_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PERSISTENT_ISCSI_LOGIN_INFOW {
    pub TargetName: [u16; 224],
    pub IsInformationalSession: super::super::Foundation::BOOLEAN,
    pub InitiatorInstance: [u16; 256],
    pub InitiatorPortNumber: u32,
    pub TargetPortal: ISCSI_TARGET_PORTALW,
    pub SecurityFlags: u64,
    pub Mappings: *mut ISCSI_TARGET_MAPPINGW,
    pub LoginOptions: ISCSI_LOGIN_OPTIONS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PERSISTENT_ISCSI_LOGIN_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PERSISTENT_ISCSI_LOGIN_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PERSISTENT_ISCSI_LOGIN_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERSISTENT_ISCSI_LOGIN_INFOW").field("TargetName", &self.TargetName).field("IsInformationalSession", &self.IsInformationalSession).field("InitiatorInstance", &self.InitiatorInstance).field("InitiatorPortNumber", &self.InitiatorPortNumber).field("TargetPortal", &self.TargetPortal).field("SecurityFlags", &self.SecurityFlags).field("Mappings", &self.Mappings).field("LoginOptions", &self.LoginOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for PERSISTENT_ISCSI_LOGIN_INFOW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PERSISTENT_ISCSI_LOGIN_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.TargetName == other.TargetName && self.IsInformationalSession == other.IsInformationalSession && self.InitiatorInstance == other.InitiatorInstance && self.InitiatorPortNumber == other.InitiatorPortNumber && self.TargetPortal == other.TargetPortal && self.SecurityFlags == other.SecurityFlags && self.Mappings == other.Mappings && self.LoginOptions == other.LoginOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PERSISTENT_ISCSI_LOGIN_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PERSISTENT_ISCSI_LOGIN_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct SCSI_ADAPTER_BUS_INFO {
    pub NumberOfBuses: u8,
    pub BusData: [SCSI_BUS_DATA; 1],
}
impl ::core::marker::Copy for SCSI_ADAPTER_BUS_INFO {}
impl ::core::clone::Clone for SCSI_ADAPTER_BUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCSI_ADAPTER_BUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCSI_ADAPTER_BUS_INFO").field("NumberOfBuses", &self.NumberOfBuses).field("BusData", &self.BusData).finish()
    }
}
impl ::windows::core::TypeKind for SCSI_ADAPTER_BUS_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SCSI_ADAPTER_BUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfBuses == other.NumberOfBuses && self.BusData == other.BusData
    }
}
impl ::core::cmp::Eq for SCSI_ADAPTER_BUS_INFO {}
impl ::core::default::Default for SCSI_ADAPTER_BUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct SCSI_ADDRESS {
    pub Length: u32,
    pub PortNumber: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
}
impl ::core::marker::Copy for SCSI_ADDRESS {}
impl ::core::clone::Clone for SCSI_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCSI_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCSI_ADDRESS").field("Length", &self.Length).field("PortNumber", &self.PortNumber).field("PathId", &self.PathId).field("TargetId", &self.TargetId).field("Lun", &self.Lun).finish()
    }
}
impl ::windows::core::TypeKind for SCSI_ADDRESS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SCSI_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.PortNumber == other.PortNumber && self.PathId == other.PathId && self.TargetId == other.TargetId && self.Lun == other.Lun
    }
}
impl ::core::cmp::Eq for SCSI_ADDRESS {}
impl ::core::default::Default for SCSI_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct SCSI_BUS_DATA {
    pub NumberOfLogicalUnits: u8,
    pub InitiatorBusId: u8,
    pub InquiryDataOffset: u32,
}
impl ::core::marker::Copy for SCSI_BUS_DATA {}
impl ::core::clone::Clone for SCSI_BUS_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCSI_BUS_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCSI_BUS_DATA").field("NumberOfLogicalUnits", &self.NumberOfLogicalUnits).field("InitiatorBusId", &self.InitiatorBusId).field("InquiryDataOffset", &self.InquiryDataOffset).finish()
    }
}
impl ::windows::core::TypeKind for SCSI_BUS_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SCSI_BUS_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfLogicalUnits == other.NumberOfLogicalUnits && self.InitiatorBusId == other.InitiatorBusId && self.InquiryDataOffset == other.InquiryDataOffset
    }
}
impl ::core::cmp::Eq for SCSI_BUS_DATA {}
impl ::core::default::Default for SCSI_BUS_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SCSI_INQUIRY_DATA {
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub DeviceClaimed: super::super::Foundation::BOOLEAN,
    pub InquiryDataLength: u32,
    pub NextInquiryDataOffset: u32,
    pub InquiryData: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SCSI_INQUIRY_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SCSI_INQUIRY_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SCSI_INQUIRY_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCSI_INQUIRY_DATA").field("PathId", &self.PathId).field("TargetId", &self.TargetId).field("Lun", &self.Lun).field("DeviceClaimed", &self.DeviceClaimed).field("InquiryDataLength", &self.InquiryDataLength).field("NextInquiryDataOffset", &self.NextInquiryDataOffset).field("InquiryData", &self.InquiryData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SCSI_INQUIRY_DATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SCSI_INQUIRY_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.PathId == other.PathId && self.TargetId == other.TargetId && self.Lun == other.Lun && self.DeviceClaimed == other.DeviceClaimed && self.InquiryDataLength == other.InquiryDataLength && self.NextInquiryDataOffset == other.NextInquiryDataOffset && self.InquiryData == other.InquiryData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SCSI_INQUIRY_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SCSI_INQUIRY_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct SCSI_LUN_LIST {
    pub OSLUN: u32,
    pub TargetLUN: u64,
}
impl ::core::marker::Copy for SCSI_LUN_LIST {}
impl ::core::clone::Clone for SCSI_LUN_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCSI_LUN_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCSI_LUN_LIST").field("OSLUN", &self.OSLUN).field("TargetLUN", &self.TargetLUN).finish()
    }
}
impl ::windows::core::TypeKind for SCSI_LUN_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SCSI_LUN_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.OSLUN == other.OSLUN && self.TargetLUN == other.TargetLUN
    }
}
impl ::core::cmp::Eq for SCSI_LUN_LIST {}
impl ::core::default::Default for SCSI_LUN_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct SCSI_PASS_THROUGH {
    pub Length: u16,
    pub ScsiStatus: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub CdbLength: u8,
    pub SenseInfoLength: u8,
    pub DataIn: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub DataBufferOffset: usize,
    pub SenseInfoOffset: u32,
    pub Cdb: [u8; 16],
}
impl ::core::marker::Copy for SCSI_PASS_THROUGH {}
impl ::core::clone::Clone for SCSI_PASS_THROUGH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCSI_PASS_THROUGH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCSI_PASS_THROUGH")
            .field("Length", &self.Length)
            .field("ScsiStatus", &self.ScsiStatus)
            .field("PathId", &self.PathId)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .field("CdbLength", &self.CdbLength)
            .field("SenseInfoLength", &self.SenseInfoLength)
            .field("DataIn", &self.DataIn)
            .field("DataTransferLength", &self.DataTransferLength)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("DataBufferOffset", &self.DataBufferOffset)
            .field("SenseInfoOffset", &self.SenseInfoOffset)
            .field("Cdb", &self.Cdb)
            .finish()
    }
}
impl ::windows::core::TypeKind for SCSI_PASS_THROUGH {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SCSI_PASS_THROUGH {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.ScsiStatus == other.ScsiStatus && self.PathId == other.PathId && self.TargetId == other.TargetId && self.Lun == other.Lun && self.CdbLength == other.CdbLength && self.SenseInfoLength == other.SenseInfoLength && self.DataIn == other.DataIn && self.DataTransferLength == other.DataTransferLength && self.TimeOutValue == other.TimeOutValue && self.DataBufferOffset == other.DataBufferOffset && self.SenseInfoOffset == other.SenseInfoOffset && self.Cdb == other.Cdb
    }
}
impl ::core::cmp::Eq for SCSI_PASS_THROUGH {}
impl ::core::default::Default for SCSI_PASS_THROUGH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SCSI_PASS_THROUGH32 {
    pub Length: u16,
    pub ScsiStatus: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub CdbLength: u8,
    pub SenseInfoLength: u8,
    pub DataIn: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub DataBufferOffset: u32,
    pub SenseInfoOffset: u32,
    pub Cdb: [u8; 16],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SCSI_PASS_THROUGH32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SCSI_PASS_THROUGH32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for SCSI_PASS_THROUGH32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCSI_PASS_THROUGH32")
            .field("Length", &self.Length)
            .field("ScsiStatus", &self.ScsiStatus)
            .field("PathId", &self.PathId)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .field("CdbLength", &self.CdbLength)
            .field("SenseInfoLength", &self.SenseInfoLength)
            .field("DataIn", &self.DataIn)
            .field("DataTransferLength", &self.DataTransferLength)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("DataBufferOffset", &self.DataBufferOffset)
            .field("SenseInfoOffset", &self.SenseInfoOffset)
            .field("Cdb", &self.Cdb)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows::core::TypeKind for SCSI_PASS_THROUGH32 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SCSI_PASS_THROUGH32 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.ScsiStatus == other.ScsiStatus && self.PathId == other.PathId && self.TargetId == other.TargetId && self.Lun == other.Lun && self.CdbLength == other.CdbLength && self.SenseInfoLength == other.SenseInfoLength && self.DataIn == other.DataIn && self.DataTransferLength == other.DataTransferLength && self.TimeOutValue == other.TimeOutValue && self.DataBufferOffset == other.DataBufferOffset && self.SenseInfoOffset == other.SenseInfoOffset && self.Cdb == other.Cdb
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SCSI_PASS_THROUGH32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SCSI_PASS_THROUGH32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SCSI_PASS_THROUGH32_EX {
    pub Version: u32,
    pub Length: u32,
    pub CdbLength: u32,
    pub StorAddressLength: u32,
    pub ScsiStatus: u8,
    pub SenseInfoLength: u8,
    pub DataDirection: u8,
    pub Reserved: u8,
    pub TimeOutValue: u32,
    pub StorAddressOffset: u32,
    pub SenseInfoOffset: u32,
    pub DataOutTransferLength: u32,
    pub DataInTransferLength: u32,
    pub DataOutBufferOffset: u32,
    pub DataInBufferOffset: u32,
    pub Cdb: [u8; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SCSI_PASS_THROUGH32_EX {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SCSI_PASS_THROUGH32_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for SCSI_PASS_THROUGH32_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCSI_PASS_THROUGH32_EX")
            .field("Version", &self.Version)
            .field("Length", &self.Length)
            .field("CdbLength", &self.CdbLength)
            .field("StorAddressLength", &self.StorAddressLength)
            .field("ScsiStatus", &self.ScsiStatus)
            .field("SenseInfoLength", &self.SenseInfoLength)
            .field("DataDirection", &self.DataDirection)
            .field("Reserved", &self.Reserved)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("StorAddressOffset", &self.StorAddressOffset)
            .field("SenseInfoOffset", &self.SenseInfoOffset)
            .field("DataOutTransferLength", &self.DataOutTransferLength)
            .field("DataInTransferLength", &self.DataInTransferLength)
            .field("DataOutBufferOffset", &self.DataOutBufferOffset)
            .field("DataInBufferOffset", &self.DataInBufferOffset)
            .field("Cdb", &self.Cdb)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows::core::TypeKind for SCSI_PASS_THROUGH32_EX {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SCSI_PASS_THROUGH32_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Length == other.Length && self.CdbLength == other.CdbLength && self.StorAddressLength == other.StorAddressLength && self.ScsiStatus == other.ScsiStatus && self.SenseInfoLength == other.SenseInfoLength && self.DataDirection == other.DataDirection && self.Reserved == other.Reserved && self.TimeOutValue == other.TimeOutValue && self.StorAddressOffset == other.StorAddressOffset && self.SenseInfoOffset == other.SenseInfoOffset && self.DataOutTransferLength == other.DataOutTransferLength && self.DataInTransferLength == other.DataInTransferLength && self.DataOutBufferOffset == other.DataOutBufferOffset && self.DataInBufferOffset == other.DataInBufferOffset && self.Cdb == other.Cdb
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SCSI_PASS_THROUGH32_EX {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SCSI_PASS_THROUGH32_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct SCSI_PASS_THROUGH_DIRECT {
    pub Length: u16,
    pub ScsiStatus: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub CdbLength: u8,
    pub SenseInfoLength: u8,
    pub DataIn: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub DataBuffer: *mut ::core::ffi::c_void,
    pub SenseInfoOffset: u32,
    pub Cdb: [u8; 16],
}
impl ::core::marker::Copy for SCSI_PASS_THROUGH_DIRECT {}
impl ::core::clone::Clone for SCSI_PASS_THROUGH_DIRECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCSI_PASS_THROUGH_DIRECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCSI_PASS_THROUGH_DIRECT")
            .field("Length", &self.Length)
            .field("ScsiStatus", &self.ScsiStatus)
            .field("PathId", &self.PathId)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .field("CdbLength", &self.CdbLength)
            .field("SenseInfoLength", &self.SenseInfoLength)
            .field("DataIn", &self.DataIn)
            .field("DataTransferLength", &self.DataTransferLength)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("DataBuffer", &self.DataBuffer)
            .field("SenseInfoOffset", &self.SenseInfoOffset)
            .field("Cdb", &self.Cdb)
            .finish()
    }
}
impl ::windows::core::TypeKind for SCSI_PASS_THROUGH_DIRECT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SCSI_PASS_THROUGH_DIRECT {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.ScsiStatus == other.ScsiStatus && self.PathId == other.PathId && self.TargetId == other.TargetId && self.Lun == other.Lun && self.CdbLength == other.CdbLength && self.SenseInfoLength == other.SenseInfoLength && self.DataIn == other.DataIn && self.DataTransferLength == other.DataTransferLength && self.TimeOutValue == other.TimeOutValue && self.DataBuffer == other.DataBuffer && self.SenseInfoOffset == other.SenseInfoOffset && self.Cdb == other.Cdb
    }
}
impl ::core::cmp::Eq for SCSI_PASS_THROUGH_DIRECT {}
impl ::core::default::Default for SCSI_PASS_THROUGH_DIRECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SCSI_PASS_THROUGH_DIRECT32 {
    pub Length: u16,
    pub ScsiStatus: u8,
    pub PathId: u8,
    pub TargetId: u8,
    pub Lun: u8,
    pub CdbLength: u8,
    pub SenseInfoLength: u8,
    pub DataIn: u8,
    pub DataTransferLength: u32,
    pub TimeOutValue: u32,
    pub DataBuffer: *mut ::core::ffi::c_void,
    pub SenseInfoOffset: u32,
    pub Cdb: [u8; 16],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SCSI_PASS_THROUGH_DIRECT32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SCSI_PASS_THROUGH_DIRECT32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for SCSI_PASS_THROUGH_DIRECT32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCSI_PASS_THROUGH_DIRECT32")
            .field("Length", &self.Length)
            .field("ScsiStatus", &self.ScsiStatus)
            .field("PathId", &self.PathId)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .field("CdbLength", &self.CdbLength)
            .field("SenseInfoLength", &self.SenseInfoLength)
            .field("DataIn", &self.DataIn)
            .field("DataTransferLength", &self.DataTransferLength)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("DataBuffer", &self.DataBuffer)
            .field("SenseInfoOffset", &self.SenseInfoOffset)
            .field("Cdb", &self.Cdb)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows::core::TypeKind for SCSI_PASS_THROUGH_DIRECT32 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SCSI_PASS_THROUGH_DIRECT32 {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.ScsiStatus == other.ScsiStatus && self.PathId == other.PathId && self.TargetId == other.TargetId && self.Lun == other.Lun && self.CdbLength == other.CdbLength && self.SenseInfoLength == other.SenseInfoLength && self.DataIn == other.DataIn && self.DataTransferLength == other.DataTransferLength && self.TimeOutValue == other.TimeOutValue && self.DataBuffer == other.DataBuffer && self.SenseInfoOffset == other.SenseInfoOffset && self.Cdb == other.Cdb
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SCSI_PASS_THROUGH_DIRECT32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SCSI_PASS_THROUGH_DIRECT32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct SCSI_PASS_THROUGH_DIRECT32_EX {
    pub Version: u32,
    pub Length: u32,
    pub CdbLength: u32,
    pub StorAddressLength: u32,
    pub ScsiStatus: u8,
    pub SenseInfoLength: u8,
    pub DataDirection: u8,
    pub Reserved: u8,
    pub TimeOutValue: u32,
    pub StorAddressOffset: u32,
    pub SenseInfoOffset: u32,
    pub DataOutTransferLength: u32,
    pub DataInTransferLength: u32,
    pub DataOutBuffer: *mut ::core::ffi::c_void,
    pub DataInBuffer: *mut ::core::ffi::c_void,
    pub Cdb: [u8; 1],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for SCSI_PASS_THROUGH_DIRECT32_EX {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for SCSI_PASS_THROUGH_DIRECT32_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for SCSI_PASS_THROUGH_DIRECT32_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCSI_PASS_THROUGH_DIRECT32_EX")
            .field("Version", &self.Version)
            .field("Length", &self.Length)
            .field("CdbLength", &self.CdbLength)
            .field("StorAddressLength", &self.StorAddressLength)
            .field("ScsiStatus", &self.ScsiStatus)
            .field("SenseInfoLength", &self.SenseInfoLength)
            .field("DataDirection", &self.DataDirection)
            .field("Reserved", &self.Reserved)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("StorAddressOffset", &self.StorAddressOffset)
            .field("SenseInfoOffset", &self.SenseInfoOffset)
            .field("DataOutTransferLength", &self.DataOutTransferLength)
            .field("DataInTransferLength", &self.DataInTransferLength)
            .field("DataOutBuffer", &self.DataOutBuffer)
            .field("DataInBuffer", &self.DataInBuffer)
            .field("Cdb", &self.Cdb)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows::core::TypeKind for SCSI_PASS_THROUGH_DIRECT32_EX {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for SCSI_PASS_THROUGH_DIRECT32_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Length == other.Length && self.CdbLength == other.CdbLength && self.StorAddressLength == other.StorAddressLength && self.ScsiStatus == other.ScsiStatus && self.SenseInfoLength == other.SenseInfoLength && self.DataDirection == other.DataDirection && self.Reserved == other.Reserved && self.TimeOutValue == other.TimeOutValue && self.StorAddressOffset == other.StorAddressOffset && self.SenseInfoOffset == other.SenseInfoOffset && self.DataOutTransferLength == other.DataOutTransferLength && self.DataInTransferLength == other.DataInTransferLength && self.DataOutBuffer == other.DataOutBuffer && self.DataInBuffer == other.DataInBuffer && self.Cdb == other.Cdb
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for SCSI_PASS_THROUGH_DIRECT32_EX {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for SCSI_PASS_THROUGH_DIRECT32_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct SCSI_PASS_THROUGH_DIRECT_EX {
    pub Version: u32,
    pub Length: u32,
    pub CdbLength: u32,
    pub StorAddressLength: u32,
    pub ScsiStatus: u8,
    pub SenseInfoLength: u8,
    pub DataDirection: u8,
    pub Reserved: u8,
    pub TimeOutValue: u32,
    pub StorAddressOffset: u32,
    pub SenseInfoOffset: u32,
    pub DataOutTransferLength: u32,
    pub DataInTransferLength: u32,
    pub DataOutBuffer: *mut ::core::ffi::c_void,
    pub DataInBuffer: *mut ::core::ffi::c_void,
    pub Cdb: [u8; 1],
}
impl ::core::marker::Copy for SCSI_PASS_THROUGH_DIRECT_EX {}
impl ::core::clone::Clone for SCSI_PASS_THROUGH_DIRECT_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCSI_PASS_THROUGH_DIRECT_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCSI_PASS_THROUGH_DIRECT_EX")
            .field("Version", &self.Version)
            .field("Length", &self.Length)
            .field("CdbLength", &self.CdbLength)
            .field("StorAddressLength", &self.StorAddressLength)
            .field("ScsiStatus", &self.ScsiStatus)
            .field("SenseInfoLength", &self.SenseInfoLength)
            .field("DataDirection", &self.DataDirection)
            .field("Reserved", &self.Reserved)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("StorAddressOffset", &self.StorAddressOffset)
            .field("SenseInfoOffset", &self.SenseInfoOffset)
            .field("DataOutTransferLength", &self.DataOutTransferLength)
            .field("DataInTransferLength", &self.DataInTransferLength)
            .field("DataOutBuffer", &self.DataOutBuffer)
            .field("DataInBuffer", &self.DataInBuffer)
            .field("Cdb", &self.Cdb)
            .finish()
    }
}
impl ::windows::core::TypeKind for SCSI_PASS_THROUGH_DIRECT_EX {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SCSI_PASS_THROUGH_DIRECT_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Length == other.Length && self.CdbLength == other.CdbLength && self.StorAddressLength == other.StorAddressLength && self.ScsiStatus == other.ScsiStatus && self.SenseInfoLength == other.SenseInfoLength && self.DataDirection == other.DataDirection && self.Reserved == other.Reserved && self.TimeOutValue == other.TimeOutValue && self.StorAddressOffset == other.StorAddressOffset && self.SenseInfoOffset == other.SenseInfoOffset && self.DataOutTransferLength == other.DataOutTransferLength && self.DataInTransferLength == other.DataInTransferLength && self.DataOutBuffer == other.DataOutBuffer && self.DataInBuffer == other.DataInBuffer && self.Cdb == other.Cdb
    }
}
impl ::core::cmp::Eq for SCSI_PASS_THROUGH_DIRECT_EX {}
impl ::core::default::Default for SCSI_PASS_THROUGH_DIRECT_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct SCSI_PASS_THROUGH_EX {
    pub Version: u32,
    pub Length: u32,
    pub CdbLength: u32,
    pub StorAddressLength: u32,
    pub ScsiStatus: u8,
    pub SenseInfoLength: u8,
    pub DataDirection: u8,
    pub Reserved: u8,
    pub TimeOutValue: u32,
    pub StorAddressOffset: u32,
    pub SenseInfoOffset: u32,
    pub DataOutTransferLength: u32,
    pub DataInTransferLength: u32,
    pub DataOutBufferOffset: usize,
    pub DataInBufferOffset: usize,
    pub Cdb: [u8; 1],
}
impl ::core::marker::Copy for SCSI_PASS_THROUGH_EX {}
impl ::core::clone::Clone for SCSI_PASS_THROUGH_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCSI_PASS_THROUGH_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCSI_PASS_THROUGH_EX")
            .field("Version", &self.Version)
            .field("Length", &self.Length)
            .field("CdbLength", &self.CdbLength)
            .field("StorAddressLength", &self.StorAddressLength)
            .field("ScsiStatus", &self.ScsiStatus)
            .field("SenseInfoLength", &self.SenseInfoLength)
            .field("DataDirection", &self.DataDirection)
            .field("Reserved", &self.Reserved)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("StorAddressOffset", &self.StorAddressOffset)
            .field("SenseInfoOffset", &self.SenseInfoOffset)
            .field("DataOutTransferLength", &self.DataOutTransferLength)
            .field("DataInTransferLength", &self.DataInTransferLength)
            .field("DataOutBufferOffset", &self.DataOutBufferOffset)
            .field("DataInBufferOffset", &self.DataInBufferOffset)
            .field("Cdb", &self.Cdb)
            .finish()
    }
}
impl ::windows::core::TypeKind for SCSI_PASS_THROUGH_EX {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SCSI_PASS_THROUGH_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Length == other.Length && self.CdbLength == other.CdbLength && self.StorAddressLength == other.StorAddressLength && self.ScsiStatus == other.ScsiStatus && self.SenseInfoLength == other.SenseInfoLength && self.DataDirection == other.DataDirection && self.Reserved == other.Reserved && self.TimeOutValue == other.TimeOutValue && self.StorAddressOffset == other.StorAddressOffset && self.SenseInfoOffset == other.SenseInfoOffset && self.DataOutTransferLength == other.DataOutTransferLength && self.DataInTransferLength == other.DataInTransferLength && self.DataOutBufferOffset == other.DataOutBufferOffset && self.DataInBufferOffset == other.DataInBufferOffset && self.Cdb == other.Cdb
    }
}
impl ::core::cmp::Eq for SCSI_PASS_THROUGH_EX {}
impl ::core::default::Default for SCSI_PASS_THROUGH_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct SRB_IO_CONTROL {
    pub HeaderLength: u32,
    pub Signature: [u8; 8],
    pub Timeout: u32,
    pub ControlCode: u32,
    pub ReturnCode: u32,
    pub Length: u32,
}
impl ::core::marker::Copy for SRB_IO_CONTROL {}
impl ::core::clone::Clone for SRB_IO_CONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SRB_IO_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SRB_IO_CONTROL").field("HeaderLength", &self.HeaderLength).field("Signature", &self.Signature).field("Timeout", &self.Timeout).field("ControlCode", &self.ControlCode).field("ReturnCode", &self.ReturnCode).field("Length", &self.Length).finish()
    }
}
impl ::windows::core::TypeKind for SRB_IO_CONTROL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SRB_IO_CONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.HeaderLength == other.HeaderLength && self.Signature == other.Signature && self.Timeout == other.Timeout && self.ControlCode == other.ControlCode && self.ReturnCode == other.ReturnCode && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for SRB_IO_CONTROL {}
impl ::core::default::Default for SRB_IO_CONTROL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct STORAGE_DIAGNOSTIC_MP_REQUEST {
    pub Version: u32,
    pub Size: u32,
    pub TargetType: MP_STORAGE_DIAGNOSTIC_TARGET_TYPE,
    pub Level: MP_STORAGE_DIAGNOSTIC_LEVEL,
    pub ProviderId: ::windows::core::GUID,
    pub BufferSize: u32,
    pub Reserved: u32,
    pub DataBuffer: [u8; 1],
}
impl ::core::marker::Copy for STORAGE_DIAGNOSTIC_MP_REQUEST {}
impl ::core::clone::Clone for STORAGE_DIAGNOSTIC_MP_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STORAGE_DIAGNOSTIC_MP_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DIAGNOSTIC_MP_REQUEST").field("Version", &self.Version).field("Size", &self.Size).field("TargetType", &self.TargetType).field("Level", &self.Level).field("ProviderId", &self.ProviderId).field("BufferSize", &self.BufferSize).field("Reserved", &self.Reserved).field("DataBuffer", &self.DataBuffer).finish()
    }
}
impl ::windows::core::TypeKind for STORAGE_DIAGNOSTIC_MP_REQUEST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for STORAGE_DIAGNOSTIC_MP_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.TargetType == other.TargetType && self.Level == other.Level && self.ProviderId == other.ProviderId && self.BufferSize == other.BufferSize && self.Reserved == other.Reserved && self.DataBuffer == other.DataBuffer
    }
}
impl ::core::cmp::Eq for STORAGE_DIAGNOSTIC_MP_REQUEST {}
impl ::core::default::Default for STORAGE_DIAGNOSTIC_MP_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct STORAGE_ENDURANCE_DATA_DESCRIPTOR {
    pub Version: u32,
    pub Size: u32,
    pub EnduranceInfo: STORAGE_ENDURANCE_INFO,
}
impl ::core::marker::Copy for STORAGE_ENDURANCE_DATA_DESCRIPTOR {}
impl ::core::clone::Clone for STORAGE_ENDURANCE_DATA_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STORAGE_ENDURANCE_DATA_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_ENDURANCE_DATA_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("EnduranceInfo", &self.EnduranceInfo).finish()
    }
}
impl ::windows::core::TypeKind for STORAGE_ENDURANCE_DATA_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for STORAGE_ENDURANCE_DATA_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.EnduranceInfo == other.EnduranceInfo
    }
}
impl ::core::cmp::Eq for STORAGE_ENDURANCE_DATA_DESCRIPTOR {}
impl ::core::default::Default for STORAGE_ENDURANCE_DATA_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct STORAGE_ENDURANCE_INFO {
    pub ValidFields: u32,
    pub GroupId: u32,
    pub Flags: STORAGE_ENDURANCE_INFO_0,
    pub LifePercentage: u32,
    pub BytesReadCount: [u8; 16],
    pub ByteWriteCount: [u8; 16],
}
impl ::core::marker::Copy for STORAGE_ENDURANCE_INFO {}
impl ::core::clone::Clone for STORAGE_ENDURANCE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STORAGE_ENDURANCE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_ENDURANCE_INFO").field("ValidFields", &self.ValidFields).field("GroupId", &self.GroupId).field("Flags", &self.Flags).field("LifePercentage", &self.LifePercentage).field("BytesReadCount", &self.BytesReadCount).field("ByteWriteCount", &self.ByteWriteCount).finish()
    }
}
impl ::windows::core::TypeKind for STORAGE_ENDURANCE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for STORAGE_ENDURANCE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ValidFields == other.ValidFields && self.GroupId == other.GroupId && self.Flags == other.Flags && self.LifePercentage == other.LifePercentage && self.BytesReadCount == other.BytesReadCount && self.ByteWriteCount == other.ByteWriteCount
    }
}
impl ::core::cmp::Eq for STORAGE_ENDURANCE_INFO {}
impl ::core::default::Default for STORAGE_ENDURANCE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct STORAGE_ENDURANCE_INFO_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for STORAGE_ENDURANCE_INFO_0 {}
impl ::core::clone::Clone for STORAGE_ENDURANCE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STORAGE_ENDURANCE_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_ENDURANCE_INFO_0").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for STORAGE_ENDURANCE_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for STORAGE_ENDURANCE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for STORAGE_ENDURANCE_INFO_0 {}
impl ::core::default::Default for STORAGE_ENDURANCE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct STORAGE_FIRMWARE_ACTIVATE {
    pub Version: u32,
    pub Size: u32,
    pub SlotToActivate: u8,
    pub Reserved0: [u8; 3],
}
impl ::core::marker::Copy for STORAGE_FIRMWARE_ACTIVATE {}
impl ::core::clone::Clone for STORAGE_FIRMWARE_ACTIVATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STORAGE_FIRMWARE_ACTIVATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_FIRMWARE_ACTIVATE").field("Version", &self.Version).field("Size", &self.Size).field("SlotToActivate", &self.SlotToActivate).field("Reserved0", &self.Reserved0).finish()
    }
}
impl ::windows::core::TypeKind for STORAGE_FIRMWARE_ACTIVATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for STORAGE_FIRMWARE_ACTIVATE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.SlotToActivate == other.SlotToActivate && self.Reserved0 == other.Reserved0
    }
}
impl ::core::cmp::Eq for STORAGE_FIRMWARE_ACTIVATE {}
impl ::core::default::Default for STORAGE_FIRMWARE_ACTIVATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct STORAGE_FIRMWARE_DOWNLOAD {
    pub Version: u32,
    pub Size: u32,
    pub Offset: u64,
    pub BufferSize: u64,
    pub ImageBuffer: [u8; 1],
}
impl ::core::marker::Copy for STORAGE_FIRMWARE_DOWNLOAD {}
impl ::core::clone::Clone for STORAGE_FIRMWARE_DOWNLOAD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STORAGE_FIRMWARE_DOWNLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_FIRMWARE_DOWNLOAD").field("Version", &self.Version).field("Size", &self.Size).field("Offset", &self.Offset).field("BufferSize", &self.BufferSize).field("ImageBuffer", &self.ImageBuffer).finish()
    }
}
impl ::windows::core::TypeKind for STORAGE_FIRMWARE_DOWNLOAD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for STORAGE_FIRMWARE_DOWNLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Offset == other.Offset && self.BufferSize == other.BufferSize && self.ImageBuffer == other.ImageBuffer
    }
}
impl ::core::cmp::Eq for STORAGE_FIRMWARE_DOWNLOAD {}
impl ::core::default::Default for STORAGE_FIRMWARE_DOWNLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub struct STORAGE_FIRMWARE_DOWNLOAD_V2 {
    pub Version: u32,
    pub Size: u32,
    pub Offset: u64,
    pub BufferSize: u64,
    pub Slot: u8,
    pub Reserved: [u8; 3],
    pub ImageSize: u32,
    pub ImageBuffer: [u8; 1],
}
impl ::core::marker::Copy for STORAGE_FIRMWARE_DOWNLOAD_V2 {}
impl ::core::clone::Clone for STORAGE_FIRMWARE_DOWNLOAD_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for STORAGE_FIRMWARE_DOWNLOAD_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_FIRMWARE_DOWNLOAD_V2").field("Version", &self.Version).field("Size", &self.Size).field("Offset", &self.Offset).field("BufferSize", &self.BufferSize).field("Slot", &self.Slot).field("Reserved", &self.Reserved).field("ImageSize", &self.ImageSize).field("ImageBuffer", &self.ImageBuffer).finish()
    }
}
impl ::windows::core::TypeKind for STORAGE_FIRMWARE_DOWNLOAD_V2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for STORAGE_FIRMWARE_DOWNLOAD_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Offset == other.Offset && self.BufferSize == other.BufferSize && self.Slot == other.Slot && self.Reserved == other.Reserved && self.ImageSize == other.ImageSize && self.ImageBuffer == other.ImageBuffer
    }
}
impl ::core::cmp::Eq for STORAGE_FIRMWARE_DOWNLOAD_V2 {}
impl ::core::default::Default for STORAGE_FIRMWARE_DOWNLOAD_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_FIRMWARE_INFO {
    pub Version: u32,
    pub Size: u32,
    pub UpgradeSupport: super::super::Foundation::BOOLEAN,
    pub SlotCount: u8,
    pub ActiveSlot: u8,
    pub PendingActivateSlot: u8,
    pub Reserved: u32,
    pub Slot: [STORAGE_FIRMWARE_SLOT_INFO; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_FIRMWARE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_FIRMWARE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for STORAGE_FIRMWARE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_FIRMWARE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_FIRMWARE_INFO_V2 {
    pub Version: u32,
    pub Size: u32,
    pub UpgradeSupport: super::super::Foundation::BOOLEAN,
    pub SlotCount: u8,
    pub ActiveSlot: u8,
    pub PendingActivateSlot: u8,
    pub FirmwareShared: super::super::Foundation::BOOLEAN,
    pub Reserved: [u8; 3],
    pub ImagePayloadAlignment: u32,
    pub ImagePayloadMaxSize: u32,
    pub Slot: [STORAGE_FIRMWARE_SLOT_INFO_V2; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_FIRMWARE_INFO_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_FIRMWARE_INFO_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_FIRMWARE_INFO_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_FIRMWARE_INFO_V2")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("UpgradeSupport", &self.UpgradeSupport)
            .field("SlotCount", &self.SlotCount)
            .field("ActiveSlot", &self.ActiveSlot)
            .field("PendingActivateSlot", &self.PendingActivateSlot)
            .field("FirmwareShared", &self.FirmwareShared)
            .field("Reserved", &self.Reserved)
            .field("ImagePayloadAlignment", &self.ImagePayloadAlignment)
            .field("ImagePayloadMaxSize", &self.ImagePayloadMaxSize)
            .field("Slot", &self.Slot)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for STORAGE_FIRMWARE_INFO_V2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_FIRMWARE_INFO_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.UpgradeSupport == other.UpgradeSupport && self.SlotCount == other.SlotCount && self.ActiveSlot == other.ActiveSlot && self.PendingActivateSlot == other.PendingActivateSlot && self.FirmwareShared == other.FirmwareShared && self.Reserved == other.Reserved && self.ImagePayloadAlignment == other.ImagePayloadAlignment && self.ImagePayloadMaxSize == other.ImagePayloadMaxSize && self.Slot == other.Slot
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_FIRMWARE_INFO_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_FIRMWARE_INFO_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_FIRMWARE_SLOT_INFO {
    pub SlotNumber: u8,
    pub ReadOnly: super::super::Foundation::BOOLEAN,
    pub Reserved: [u8; 6],
    pub Revision: STORAGE_FIRMWARE_SLOT_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_FIRMWARE_SLOT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_FIRMWARE_SLOT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for STORAGE_FIRMWARE_SLOT_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_FIRMWARE_SLOT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union STORAGE_FIRMWARE_SLOT_INFO_0 {
    pub Info: [u8; 8],
    pub AsUlonglong: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_FIRMWARE_SLOT_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_FIRMWARE_SLOT_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for STORAGE_FIRMWARE_SLOT_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_FIRMWARE_SLOT_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_FIRMWARE_SLOT_INFO_V2 {
    pub SlotNumber: u8,
    pub ReadOnly: super::super::Foundation::BOOLEAN,
    pub Reserved: [u8; 6],
    pub Revision: [u8; 16],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STORAGE_FIRMWARE_SLOT_INFO_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STORAGE_FIRMWARE_SLOT_INFO_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_FIRMWARE_SLOT_INFO_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_FIRMWARE_SLOT_INFO_V2").field("SlotNumber", &self.SlotNumber).field("ReadOnly", &self.ReadOnly).field("Reserved", &self.Reserved).field("Revision", &self.Revision).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for STORAGE_FIRMWARE_SLOT_INFO_V2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_FIRMWARE_SLOT_INFO_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.SlotNumber == other.SlotNumber && self.ReadOnly == other.ReadOnly && self.Reserved == other.Reserved && self.Revision == other.Revision
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_FIRMWARE_SLOT_INFO_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_FIRMWARE_SLOT_INFO_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct _ADAPTER_OBJECT(pub u8);
#[doc = "*Required features: `\"Win32_Storage_IscsiDisc\"`*"]
pub type PDUMP_DEVICE_POWERON_ROUTINE = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void) -> i32>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
