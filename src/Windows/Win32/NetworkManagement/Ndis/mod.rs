#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct BSSID_INFO {
    pub BSSID: [u8; 6],
    pub PMKID: [u8; 16],
}
impl BSSID_INFO {}
impl ::std::default::Default for BSSID_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BSSID_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BSSID_INFO").field("BSSID", &self.BSSID).field("PMKID", &self.PMKID).finish()
    }
}
impl ::std::cmp::PartialEq for BSSID_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.BSSID == other.BSSID && self.PMKID == other.PMKID
    }
}
impl ::std::cmp::Eq for BSSID_INFO {}
unsafe impl ::windows::runtime::Abi for BSSID_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const CLOCK_NETWORK_DERIVED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const CLOCK_PRECISION: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXTIHV_ADAPTER_RESET = unsafe extern "system" fn(hihvextadapter: super::super::Foundation::HANDLE) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXTIHV_CONTROL = unsafe extern "system" fn(hihvextadapter: super::super::Foundation::HANDLE, dwinbuffersize: u32, pinbuffer: *const u8, dwoutbuffersize: u32, poutbuffer: *mut u8, pdwbytesreturned: *mut u32) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`, `Win32_NetworkManagement_WiFi`, `Win32_Security_ExtensibleAuthenticationProtocol`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
pub type DOT11EXTIHV_CREATE_DISCOVERY_PROFILES = unsafe extern "system" fn(hihvextadapter: super::super::Foundation::HANDLE, binsecure: super::super::Foundation::BOOL, pihvprofileparams: *const DOT11EXT_IHV_PROFILE_PARAMS, pconnectablebssid: *const DOT11_BSS_LIST, pihvdiscoveryprofilelist: *mut DOT11EXT_IHV_DISCOVERY_PROFILE_LIST, pdwreasoncode: *mut u32) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXTIHV_DEINIT_ADAPTER = unsafe extern "system" fn(hihvextadapter: super::super::Foundation::HANDLE);
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub type DOT11EXTIHV_DEINIT_SERVICE = unsafe extern "system" fn();
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub type DOT11EXTIHV_GET_VERSION_INFO = unsafe extern "system" fn(pdot11ihvversioninfo: *mut DOT11_IHV_VERSION_INFO) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`, `Win32_NetworkManagement_WiFi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
pub type DOT11EXTIHV_INIT_ADAPTER = unsafe extern "system" fn(pdot11adapter: *const DOT11_ADAPTER, hdot11svchandle: super::super::Foundation::HANDLE, phihvextadapter: *mut super::super::Foundation::HANDLE) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`, `Win32_NetworkManagement_WiFi`, `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_System_RemoteDesktop`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol", feature = "Win32_System_RemoteDesktop"))]
pub type DOT11EXTIHV_INIT_SERVICE = unsafe extern "system" fn(dwvernumused: u32, pdot11extapi: *const ::std::mem::ManuallyDrop<DOT11EXT_APIS>, pvreserved: *mut ::std::ffi::c_void, pdot11ihvhandlers: *mut ::std::mem::ManuallyDrop<DOT11EXT_IHV_HANDLERS>) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`, `Win32_NetworkManagement_WiFi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
pub type DOT11EXTIHV_INIT_VIRTUAL_STATION = unsafe extern "system" fn(pdot11extvsapi: *const ::std::mem::ManuallyDrop<DOT11EXT_VIRTUAL_STATION_APIS>, pvreserved: *mut ::std::ffi::c_void) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXTIHV_IS_UI_REQUEST_PENDING = unsafe extern "system" fn(guiduirequest: ::windows::runtime::GUID, pbisrequestpending: *mut super::super::Foundation::BOOL) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`, `Win32_NetworkManagement_WiFi`, `Win32_Security_ExtensibleAuthenticationProtocol`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
pub type DOT11EXTIHV_ONEX_INDICATE_RESULT = unsafe extern "system" fn(hihvextadapter: super::super::Foundation::HANDLE, msonexresult: DOT11_MSONEX_RESULT, pdot11msonexresultparams: *const DOT11_MSONEX_RESULT_PARAMS) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`, `Win32_NetworkManagement_WiFi`, `Win32_Security_ExtensibleAuthenticationProtocol`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
pub type DOT11EXTIHV_PERFORM_CAPABILITY_MATCH = unsafe extern "system" fn(hihvextadapter: super::super::Foundation::HANDLE, pihvprofileparams: *const DOT11EXT_IHV_PROFILE_PARAMS, pihvconnprofile: *const DOT11EXT_IHV_CONNECTIVITY_PROFILE, pihvsecprofile: *const DOT11EXT_IHV_SECURITY_PROFILE, pconnectablebssid: *const DOT11_BSS_LIST, pdwreasoncode: *mut u32) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`, `Win32_NetworkManagement_WiFi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
pub type DOT11EXTIHV_PERFORM_POST_ASSOCIATE = unsafe extern "system" fn(hihvextadapter: super::super::Foundation::HANDLE, hsecuritysessionid: super::super::Foundation::HANDLE, pportstate: *const DOT11_PORT_STATE, udot11assocparamsbytes: u32, pdot11assocparams: *const super::WiFi::DOT11_ASSOCIATION_COMPLETION_PARAMETERS) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`, `Win32_NetworkManagement_WiFi`, `Win32_Security_ExtensibleAuthenticationProtocol`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
pub type DOT11EXTIHV_PERFORM_PRE_ASSOCIATE = unsafe extern "system" fn(hihvextadapter: super::super::Foundation::HANDLE, hconnectsession: super::super::Foundation::HANDLE, pihvprofileparams: *const DOT11EXT_IHV_PROFILE_PARAMS, pihvconnprofile: *const DOT11EXT_IHV_CONNECTIVITY_PROFILE, pihvsecprofile: *const DOT11EXT_IHV_SECURITY_PROFILE, pconnectablebssid: *const DOT11_BSS_LIST, pdwreasoncode: *mut u32) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_System_RemoteDesktop`*"]
#[cfg(feature = "Win32_System_RemoteDesktop")]
pub type DOT11EXTIHV_PROCESS_SESSION_CHANGE = unsafe extern "system" fn(ueventtype: u32, psessionnotification: *const super::super::System::RemoteDesktop::WTSSESSION_NOTIFICATION) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub type DOT11EXTIHV_PROCESS_UI_RESPONSE = unsafe extern "system" fn(guiduirequest: ::windows::runtime::GUID, dwbytecount: u32, pvresponsebuffer: *const ::std::ffi::c_void) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXTIHV_QUERY_UI_REQUEST = unsafe extern "system" fn(hihvextadapter: super::super::Foundation::HANDLE, connectionphase: DOT11EXT_IHV_CONNECTION_PHASE, ppihvuirequest: *mut *mut DOT11EXT_IHV_UI_REQUEST) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXTIHV_RECEIVE_INDICATION = unsafe extern "system" fn(hihvextadapter: super::super::Foundation::HANDLE, indicationtype: DOT11EXT_IHV_INDICATION_TYPE, ubufferlength: u32, pvbuffer: *const ::std::ffi::c_void) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXTIHV_RECEIVE_PACKET = unsafe extern "system" fn(hihvextadapter: super::super::Foundation::HANDLE, dwinbuffersize: u32, pvinbuffer: *const ::std::ffi::c_void) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXTIHV_SEND_PACKET_COMPLETION = unsafe extern "system" fn(hsendcompletion: super::super::Foundation::HANDLE) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXTIHV_STOP_POST_ASSOCIATE = unsafe extern "system" fn(hihvextadapter: super::super::Foundation::HANDLE, ppeer: *const *const u8, dot11assocstatus: u32) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`, `Win32_NetworkManagement_WiFi`, `Win32_Security_ExtensibleAuthenticationProtocol`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
pub type DOT11EXTIHV_VALIDATE_PROFILE = unsafe extern "system" fn(hihvextadapter: super::super::Foundation::HANDLE, pihvprofileparams: *const DOT11EXT_IHV_PROFILE_PARAMS, pihvconnprofile: *const DOT11EXT_IHV_CONNECTIVITY_PROFILE, pihvsecprofile: *const DOT11EXT_IHV_SECURITY_PROFILE, pdwreasoncode: *mut u32) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub type DOT11EXT_ALLOCATE_BUFFER = unsafe extern "system" fn(dwbytecount: u32, ppvbuffer: *mut *mut ::std::ffi::c_void) -> u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`, `Win32_NetworkManagement_WiFi`, `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub struct DOT11EXT_APIS {
    pub Dot11ExtAllocateBuffer: ::std::option::Option<DOT11EXT_ALLOCATE_BUFFER>,
    pub Dot11ExtFreeBuffer: ::std::option::Option<DOT11EXT_FREE_BUFFER>,
    pub Dot11ExtSetProfileCustomUserData: ::std::option::Option<DOT11EXT_SET_PROFILE_CUSTOM_USER_DATA>,
    pub Dot11ExtGetProfileCustomUserData: ::std::option::Option<DOT11EXT_GET_PROFILE_CUSTOM_USER_DATA>,
    pub Dot11ExtSetCurrentProfile: ::std::option::Option<DOT11EXT_SET_CURRENT_PROFILE>,
    pub Dot11ExtSendUIRequest: ::std::option::Option<DOT11EXT_SEND_UI_REQUEST>,
    pub Dot11ExtPreAssociateCompletion: ::std::option::Option<DOT11EXT_PRE_ASSOCIATE_COMPLETION>,
    pub Dot11ExtPostAssociateCompletion: ::std::option::Option<DOT11EXT_POST_ASSOCIATE_COMPLETION>,
    pub Dot11ExtSendNotification: ::std::option::Option<DOT11EXT_SEND_NOTIFICATION>,
    pub Dot11ExtSendPacket: ::std::option::Option<DOT11EXT_SEND_PACKET>,
    pub Dot11ExtSetEtherTypeHandling: ::std::option::Option<DOT11EXT_SET_ETHERTYPE_HANDLING>,
    pub Dot11ExtSetAuthAlgorithm: ::std::option::Option<DOT11EXT_SET_AUTH_ALGORITHM>,
    pub Dot11ExtSetUnicastCipherAlgorithm: ::std::option::Option<DOT11EXT_SET_UNICAST_CIPHER_ALGORITHM>,
    pub Dot11ExtSetMulticastCipherAlgorithm: ::std::option::Option<DOT11EXT_SET_MULTICAST_CIPHER_ALGORITHM>,
    pub Dot11ExtSetDefaultKey: ::std::option::Option<DOT11EXT_SET_DEFAULT_KEY>,
    pub Dot11ExtSetKeyMappingKey: ::std::option::Option<DOT11EXT_SET_KEY_MAPPING_KEY>,
    pub Dot11ExtSetDefaultKeyId: ::std::option::Option<DOT11EXT_SET_DEFAULT_KEY_ID>,
    pub Dot11ExtNicSpecificExtension: ::std::option::Option<DOT11EXT_NIC_SPECIFIC_EXTENSION>,
    pub Dot11ExtSetExcludeUnencrypted: ::std::option::Option<DOT11EXT_SET_EXCLUDE_UNENCRYPTED>,
    pub Dot11ExtStartOneX: ::std::option::Option<DOT11EXT_ONEX_START>,
    pub Dot11ExtStopOneX: ::std::option::Option<DOT11EXT_ONEX_STOP>,
    pub Dot11ExtProcessSecurityPacket: ::std::option::Option<DOT11EXT_PROCESS_ONEX_PACKET>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl DOT11EXT_APIS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::std::default::Default for DOT11EXT_APIS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::std::fmt::Debug for DOT11EXT_APIS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DOT11EXT_APIS").finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::std::cmp::PartialEq for DOT11EXT_APIS {
    fn eq(&self, other: &Self) -> bool {
        self.Dot11ExtAllocateBuffer.map(|f| f as usize) == other.Dot11ExtAllocateBuffer.map(|f| f as usize)
            && self.Dot11ExtFreeBuffer.map(|f| f as usize) == other.Dot11ExtFreeBuffer.map(|f| f as usize)
            && self.Dot11ExtSetProfileCustomUserData.map(|f| f as usize) == other.Dot11ExtSetProfileCustomUserData.map(|f| f as usize)
            && self.Dot11ExtGetProfileCustomUserData.map(|f| f as usize) == other.Dot11ExtGetProfileCustomUserData.map(|f| f as usize)
            && self.Dot11ExtSetCurrentProfile.map(|f| f as usize) == other.Dot11ExtSetCurrentProfile.map(|f| f as usize)
            && self.Dot11ExtSendUIRequest.map(|f| f as usize) == other.Dot11ExtSendUIRequest.map(|f| f as usize)
            && self.Dot11ExtPreAssociateCompletion.map(|f| f as usize) == other.Dot11ExtPreAssociateCompletion.map(|f| f as usize)
            && self.Dot11ExtPostAssociateCompletion.map(|f| f as usize) == other.Dot11ExtPostAssociateCompletion.map(|f| f as usize)
            && self.Dot11ExtSendNotification.map(|f| f as usize) == other.Dot11ExtSendNotification.map(|f| f as usize)
            && self.Dot11ExtSendPacket.map(|f| f as usize) == other.Dot11ExtSendPacket.map(|f| f as usize)
            && self.Dot11ExtSetEtherTypeHandling.map(|f| f as usize) == other.Dot11ExtSetEtherTypeHandling.map(|f| f as usize)
            && self.Dot11ExtSetAuthAlgorithm.map(|f| f as usize) == other.Dot11ExtSetAuthAlgorithm.map(|f| f as usize)
            && self.Dot11ExtSetUnicastCipherAlgorithm.map(|f| f as usize) == other.Dot11ExtSetUnicastCipherAlgorithm.map(|f| f as usize)
            && self.Dot11ExtSetMulticastCipherAlgorithm.map(|f| f as usize) == other.Dot11ExtSetMulticastCipherAlgorithm.map(|f| f as usize)
            && self.Dot11ExtSetDefaultKey.map(|f| f as usize) == other.Dot11ExtSetDefaultKey.map(|f| f as usize)
            && self.Dot11ExtSetKeyMappingKey.map(|f| f as usize) == other.Dot11ExtSetKeyMappingKey.map(|f| f as usize)
            && self.Dot11ExtSetDefaultKeyId.map(|f| f as usize) == other.Dot11ExtSetDefaultKeyId.map(|f| f as usize)
            && self.Dot11ExtNicSpecificExtension.map(|f| f as usize) == other.Dot11ExtNicSpecificExtension.map(|f| f as usize)
            && self.Dot11ExtSetExcludeUnencrypted.map(|f| f as usize) == other.Dot11ExtSetExcludeUnencrypted.map(|f| f as usize)
            && self.Dot11ExtStartOneX.map(|f| f as usize) == other.Dot11ExtStartOneX.map(|f| f as usize)
            && self.Dot11ExtStopOneX.map(|f| f as usize) == other.Dot11ExtStopOneX.map(|f| f as usize)
            && self.Dot11ExtProcessSecurityPacket.map(|f| f as usize) == other.Dot11ExtProcessSecurityPacket.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::std::cmp::Eq for DOT11EXT_APIS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
unsafe impl ::windows::runtime::Abi for DOT11EXT_APIS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub type DOT11EXT_FREE_BUFFER = unsafe extern "system" fn(pvmemory: *const ::std::ffi::c_void);
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_GET_PROFILE_CUSTOM_USER_DATA = unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, hconnectsession: super::super::Foundation::HANDLE, dwsessionid: u32, pdwdatasize: *mut u32, ppvdata: *mut *mut ::std::ffi::c_void) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DOT11EXT_IHV_CONNECTION_PHASE(pub i32);
pub const connection_phase_any: DOT11EXT_IHV_CONNECTION_PHASE = DOT11EXT_IHV_CONNECTION_PHASE(0i32);
pub const connection_phase_initial_connection: DOT11EXT_IHV_CONNECTION_PHASE = DOT11EXT_IHV_CONNECTION_PHASE(1i32);
pub const connection_phase_post_l3_connection: DOT11EXT_IHV_CONNECTION_PHASE = DOT11EXT_IHV_CONNECTION_PHASE(2i32);
impl ::std::convert::From<i32> for DOT11EXT_IHV_CONNECTION_PHASE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DOT11EXT_IHV_CONNECTION_PHASE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
pub struct DOT11EXT_IHV_CONNECTIVITY_PROFILE {
    pub pszXmlFragmentIhvConnectivity: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl DOT11EXT_IHV_CONNECTIVITY_PROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DOT11EXT_IHV_CONNECTIVITY_PROFILE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DOT11EXT_IHV_CONNECTIVITY_PROFILE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DOT11EXT_IHV_CONNECTIVITY_PROFILE").field("pszXmlFragmentIhvConnectivity", &self.pszXmlFragmentIhvConnectivity).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DOT11EXT_IHV_CONNECTIVITY_PROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.pszXmlFragmentIhvConnectivity == other.pszXmlFragmentIhvConnectivity
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DOT11EXT_IHV_CONNECTIVITY_PROFILE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DOT11EXT_IHV_CONNECTIVITY_PROFILE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
pub struct DOT11EXT_IHV_DISCOVERY_PROFILE {
    pub IhvConnectivityProfile: DOT11EXT_IHV_CONNECTIVITY_PROFILE,
    pub IhvSecurityProfile: DOT11EXT_IHV_SECURITY_PROFILE,
}
#[cfg(feature = "Win32_Foundation")]
impl DOT11EXT_IHV_DISCOVERY_PROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DOT11EXT_IHV_DISCOVERY_PROFILE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DOT11EXT_IHV_DISCOVERY_PROFILE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DOT11EXT_IHV_DISCOVERY_PROFILE").field("IhvConnectivityProfile", &self.IhvConnectivityProfile).field("IhvSecurityProfile", &self.IhvSecurityProfile).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DOT11EXT_IHV_DISCOVERY_PROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.IhvConnectivityProfile == other.IhvConnectivityProfile && self.IhvSecurityProfile == other.IhvSecurityProfile
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DOT11EXT_IHV_DISCOVERY_PROFILE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DOT11EXT_IHV_DISCOVERY_PROFILE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
pub struct DOT11EXT_IHV_DISCOVERY_PROFILE_LIST {
    pub dwCount: u32,
    pub pIhvDiscoveryProfiles: *mut DOT11EXT_IHV_DISCOVERY_PROFILE,
}
#[cfg(feature = "Win32_Foundation")]
impl DOT11EXT_IHV_DISCOVERY_PROFILE_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DOT11EXT_IHV_DISCOVERY_PROFILE_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DOT11EXT_IHV_DISCOVERY_PROFILE_LIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DOT11EXT_IHV_DISCOVERY_PROFILE_LIST").field("dwCount", &self.dwCount).field("pIhvDiscoveryProfiles", &self.pIhvDiscoveryProfiles).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DOT11EXT_IHV_DISCOVERY_PROFILE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwCount == other.dwCount && self.pIhvDiscoveryProfiles == other.pIhvDiscoveryProfiles
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DOT11EXT_IHV_DISCOVERY_PROFILE_LIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DOT11EXT_IHV_DISCOVERY_PROFILE_LIST {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol", feature = "Win32_System_RemoteDesktop"))]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`, `Win32_NetworkManagement_WiFi`, `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_System_RemoteDesktop`*"]
pub struct DOT11EXT_IHV_HANDLERS {
    pub Dot11ExtIhvDeinitService: ::std::option::Option<DOT11EXTIHV_DEINIT_SERVICE>,
    pub Dot11ExtIhvInitAdapter: ::std::option::Option<DOT11EXTIHV_INIT_ADAPTER>,
    pub Dot11ExtIhvDeinitAdapter: ::std::option::Option<DOT11EXTIHV_DEINIT_ADAPTER>,
    pub Dot11ExtIhvPerformPreAssociate: ::std::option::Option<DOT11EXTIHV_PERFORM_PRE_ASSOCIATE>,
    pub Dot11ExtIhvAdapterReset: ::std::option::Option<DOT11EXTIHV_ADAPTER_RESET>,
    pub Dot11ExtIhvPerformPostAssociate: ::std::option::Option<DOT11EXTIHV_PERFORM_POST_ASSOCIATE>,
    pub Dot11ExtIhvStopPostAssociate: ::std::option::Option<DOT11EXTIHV_STOP_POST_ASSOCIATE>,
    pub Dot11ExtIhvValidateProfile: ::std::option::Option<DOT11EXTIHV_VALIDATE_PROFILE>,
    pub Dot11ExtIhvPerformCapabilityMatch: ::std::option::Option<DOT11EXTIHV_PERFORM_CAPABILITY_MATCH>,
    pub Dot11ExtIhvCreateDiscoveryProfiles: ::std::option::Option<DOT11EXTIHV_CREATE_DISCOVERY_PROFILES>,
    pub Dot11ExtIhvProcessSessionChange: ::std::option::Option<DOT11EXTIHV_PROCESS_SESSION_CHANGE>,
    pub Dot11ExtIhvReceiveIndication: ::std::option::Option<DOT11EXTIHV_RECEIVE_INDICATION>,
    pub Dot11ExtIhvReceivePacket: ::std::option::Option<DOT11EXTIHV_RECEIVE_PACKET>,
    pub Dot11ExtIhvSendPacketCompletion: ::std::option::Option<DOT11EXTIHV_SEND_PACKET_COMPLETION>,
    pub Dot11ExtIhvIsUIRequestPending: ::std::option::Option<DOT11EXTIHV_IS_UI_REQUEST_PENDING>,
    pub Dot11ExtIhvProcessUIResponse: ::std::option::Option<DOT11EXTIHV_PROCESS_UI_RESPONSE>,
    pub Dot11ExtIhvQueryUIRequest: ::std::option::Option<DOT11EXTIHV_QUERY_UI_REQUEST>,
    pub Dot11ExtIhvOnexIndicateResult: ::std::option::Option<DOT11EXTIHV_ONEX_INDICATE_RESULT>,
    pub Dot11ExtIhvControl: ::std::option::Option<DOT11EXTIHV_CONTROL>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol", feature = "Win32_System_RemoteDesktop"))]
impl DOT11EXT_IHV_HANDLERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol", feature = "Win32_System_RemoteDesktop"))]
impl ::std::default::Default for DOT11EXT_IHV_HANDLERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol", feature = "Win32_System_RemoteDesktop"))]
impl ::std::fmt::Debug for DOT11EXT_IHV_HANDLERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DOT11EXT_IHV_HANDLERS").finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol", feature = "Win32_System_RemoteDesktop"))]
impl ::std::cmp::PartialEq for DOT11EXT_IHV_HANDLERS {
    fn eq(&self, other: &Self) -> bool {
        self.Dot11ExtIhvDeinitService.map(|f| f as usize) == other.Dot11ExtIhvDeinitService.map(|f| f as usize)
            && self.Dot11ExtIhvInitAdapter.map(|f| f as usize) == other.Dot11ExtIhvInitAdapter.map(|f| f as usize)
            && self.Dot11ExtIhvDeinitAdapter.map(|f| f as usize) == other.Dot11ExtIhvDeinitAdapter.map(|f| f as usize)
            && self.Dot11ExtIhvPerformPreAssociate.map(|f| f as usize) == other.Dot11ExtIhvPerformPreAssociate.map(|f| f as usize)
            && self.Dot11ExtIhvAdapterReset.map(|f| f as usize) == other.Dot11ExtIhvAdapterReset.map(|f| f as usize)
            && self.Dot11ExtIhvPerformPostAssociate.map(|f| f as usize) == other.Dot11ExtIhvPerformPostAssociate.map(|f| f as usize)
            && self.Dot11ExtIhvStopPostAssociate.map(|f| f as usize) == other.Dot11ExtIhvStopPostAssociate.map(|f| f as usize)
            && self.Dot11ExtIhvValidateProfile.map(|f| f as usize) == other.Dot11ExtIhvValidateProfile.map(|f| f as usize)
            && self.Dot11ExtIhvPerformCapabilityMatch.map(|f| f as usize) == other.Dot11ExtIhvPerformCapabilityMatch.map(|f| f as usize)
            && self.Dot11ExtIhvCreateDiscoveryProfiles.map(|f| f as usize) == other.Dot11ExtIhvCreateDiscoveryProfiles.map(|f| f as usize)
            && self.Dot11ExtIhvProcessSessionChange.map(|f| f as usize) == other.Dot11ExtIhvProcessSessionChange.map(|f| f as usize)
            && self.Dot11ExtIhvReceiveIndication.map(|f| f as usize) == other.Dot11ExtIhvReceiveIndication.map(|f| f as usize)
            && self.Dot11ExtIhvReceivePacket.map(|f| f as usize) == other.Dot11ExtIhvReceivePacket.map(|f| f as usize)
            && self.Dot11ExtIhvSendPacketCompletion.map(|f| f as usize) == other.Dot11ExtIhvSendPacketCompletion.map(|f| f as usize)
            && self.Dot11ExtIhvIsUIRequestPending.map(|f| f as usize) == other.Dot11ExtIhvIsUIRequestPending.map(|f| f as usize)
            && self.Dot11ExtIhvProcessUIResponse.map(|f| f as usize) == other.Dot11ExtIhvProcessUIResponse.map(|f| f as usize)
            && self.Dot11ExtIhvQueryUIRequest.map(|f| f as usize) == other.Dot11ExtIhvQueryUIRequest.map(|f| f as usize)
            && self.Dot11ExtIhvOnexIndicateResult.map(|f| f as usize) == other.Dot11ExtIhvOnexIndicateResult.map(|f| f as usize)
            && self.Dot11ExtIhvControl.map(|f| f as usize) == other.Dot11ExtIhvControl.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol", feature = "Win32_System_RemoteDesktop"))]
impl ::std::cmp::Eq for DOT11EXT_IHV_HANDLERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol", feature = "Win32_System_RemoteDesktop"))]
unsafe impl ::windows::runtime::Abi for DOT11EXT_IHV_HANDLERS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DOT11EXT_IHV_INDICATION_TYPE(pub i32);
pub const IndicationTypeNicSpecificNotification: DOT11EXT_IHV_INDICATION_TYPE = DOT11EXT_IHV_INDICATION_TYPE(0i32);
pub const IndicationTypePmkidCandidateList: DOT11EXT_IHV_INDICATION_TYPE = DOT11EXT_IHV_INDICATION_TYPE(1i32);
pub const IndicationTypeTkipMicFailure: DOT11EXT_IHV_INDICATION_TYPE = DOT11EXT_IHV_INDICATION_TYPE(2i32);
pub const IndicationTypePhyStateChange: DOT11EXT_IHV_INDICATION_TYPE = DOT11EXT_IHV_INDICATION_TYPE(3i32);
pub const IndicationTypeLinkQuality: DOT11EXT_IHV_INDICATION_TYPE = DOT11EXT_IHV_INDICATION_TYPE(4i32);
impl ::std::convert::From<i32> for DOT11EXT_IHV_INDICATION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DOT11EXT_IHV_INDICATION_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`, `Win32_NetworkManagement_WiFi`, `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub struct DOT11EXT_IHV_PARAMS {
    pub dot11ExtIhvProfileParams: DOT11EXT_IHV_PROFILE_PARAMS,
    pub wstrProfileName: [u16; 256],
    pub dwProfileTypeFlags: u32,
    pub interfaceGuid: ::windows::runtime::GUID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl DOT11EXT_IHV_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::std::default::Default for DOT11EXT_IHV_PARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::std::fmt::Debug for DOT11EXT_IHV_PARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DOT11EXT_IHV_PARAMS").field("dot11ExtIhvProfileParams", &self.dot11ExtIhvProfileParams).field("wstrProfileName", &self.wstrProfileName).field("dwProfileTypeFlags", &self.dwProfileTypeFlags).field("interfaceGuid", &self.interfaceGuid).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::std::cmp::PartialEq for DOT11EXT_IHV_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.dot11ExtIhvProfileParams == other.dot11ExtIhvProfileParams && self.wstrProfileName == other.wstrProfileName && self.dwProfileTypeFlags == other.dwProfileTypeFlags && self.interfaceGuid == other.interfaceGuid
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::std::cmp::Eq for DOT11EXT_IHV_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
unsafe impl ::windows::runtime::Abi for DOT11EXT_IHV_PARAMS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`, `Win32_NetworkManagement_WiFi`, `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub struct DOT11EXT_IHV_PROFILE_PARAMS {
    pub pSsidList: *mut DOT11EXT_IHV_SSID_LIST,
    pub BssType: super::WiFi::DOT11_BSS_TYPE,
    pub pMSSecuritySettings: *mut DOT11_MSSECURITY_SETTINGS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl DOT11EXT_IHV_PROFILE_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::std::default::Default for DOT11EXT_IHV_PROFILE_PARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::std::fmt::Debug for DOT11EXT_IHV_PROFILE_PARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DOT11EXT_IHV_PROFILE_PARAMS").field("pSsidList", &self.pSsidList).field("BssType", &self.BssType).field("pMSSecuritySettings", &self.pMSSecuritySettings).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::std::cmp::PartialEq for DOT11EXT_IHV_PROFILE_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.pSsidList == other.pSsidList && self.BssType == other.BssType && self.pMSSecuritySettings == other.pMSSecuritySettings
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::std::cmp::Eq for DOT11EXT_IHV_PROFILE_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
unsafe impl ::windows::runtime::Abi for DOT11EXT_IHV_PROFILE_PARAMS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
pub struct DOT11EXT_IHV_SECURITY_PROFILE {
    pub pszXmlFragmentIhvSecurity: super::super::Foundation::PWSTR,
    pub bUseMSOnex: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl DOT11EXT_IHV_SECURITY_PROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DOT11EXT_IHV_SECURITY_PROFILE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DOT11EXT_IHV_SECURITY_PROFILE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DOT11EXT_IHV_SECURITY_PROFILE").field("pszXmlFragmentIhvSecurity", &self.pszXmlFragmentIhvSecurity).field("bUseMSOnex", &self.bUseMSOnex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DOT11EXT_IHV_SECURITY_PROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.pszXmlFragmentIhvSecurity == other.pszXmlFragmentIhvSecurity && self.bUseMSOnex == other.bUseMSOnex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DOT11EXT_IHV_SECURITY_PROFILE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DOT11EXT_IHV_SECURITY_PROFILE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_WiFi")]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_NetworkManagement_WiFi`*"]
pub struct DOT11EXT_IHV_SSID_LIST {
    pub ulCount: u32,
    pub SSIDs: [super::WiFi::DOT11_SSID; 1],
}
#[cfg(feature = "Win32_NetworkManagement_WiFi")]
impl DOT11EXT_IHV_SSID_LIST {}
#[cfg(feature = "Win32_NetworkManagement_WiFi")]
impl ::std::default::Default for DOT11EXT_IHV_SSID_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_WiFi")]
impl ::std::fmt::Debug for DOT11EXT_IHV_SSID_LIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DOT11EXT_IHV_SSID_LIST").field("ulCount", &self.ulCount).field("SSIDs", &self.SSIDs).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_WiFi")]
impl ::std::cmp::PartialEq for DOT11EXT_IHV_SSID_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.ulCount == other.ulCount && self.SSIDs == other.SSIDs
    }
}
#[cfg(feature = "Win32_NetworkManagement_WiFi")]
impl ::std::cmp::Eq for DOT11EXT_IHV_SSID_LIST {}
#[cfg(feature = "Win32_NetworkManagement_WiFi")]
unsafe impl ::windows::runtime::Abi for DOT11EXT_IHV_SSID_LIST {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct DOT11EXT_IHV_UI_REQUEST {
    pub dwSessionId: u32,
    pub guidUIRequest: ::windows::runtime::GUID,
    pub UIPageClsid: ::windows::runtime::GUID,
    pub dwByteCount: u32,
    pub pvUIRequest: *mut u8,
}
impl DOT11EXT_IHV_UI_REQUEST {}
impl ::std::default::Default for DOT11EXT_IHV_UI_REQUEST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DOT11EXT_IHV_UI_REQUEST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DOT11EXT_IHV_UI_REQUEST").field("dwSessionId", &self.dwSessionId).field("guidUIRequest", &self.guidUIRequest).field("UIPageClsid", &self.UIPageClsid).field("dwByteCount", &self.dwByteCount).field("pvUIRequest", &self.pvUIRequest).finish()
    }
}
impl ::std::cmp::PartialEq for DOT11EXT_IHV_UI_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.dwSessionId == other.dwSessionId && self.guidUIRequest == other.guidUIRequest && self.UIPageClsid == other.UIPageClsid && self.dwByteCount == other.dwByteCount && self.pvUIRequest == other.pvUIRequest
    }
}
impl ::std::cmp::Eq for DOT11EXT_IHV_UI_REQUEST {}
unsafe impl ::windows::runtime::Abi for DOT11EXT_IHV_UI_REQUEST {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_NIC_SPECIFIC_EXTENSION = unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, dwinbuffersize: u32, pvinbuffer: *const ::std::ffi::c_void, pdwoutbuffersize: *mut u32, pvoutbuffer: *mut ::std::ffi::c_void) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`, `Win32_Security_ExtensibleAuthenticationProtocol`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
pub type DOT11EXT_ONEX_START = unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, peapattributes: *const super::super::Security::ExtensibleAuthenticationProtocol::EAP_ATTRIBUTES) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_ONEX_STOP = unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_POST_ASSOCIATE_COMPLETION = unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, hsecuritysessionid: super::super::Foundation::HANDLE, ppeer: *const *const u8, dwreasoncode: u32, dwwin32error: u32) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_PRE_ASSOCIATE_COMPLETION = unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, hconnectsession: super::super::Foundation::HANDLE, dwreasoncode: u32, dwwin32error: u32) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_PROCESS_ONEX_PACKET = unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, dwinpacketsize: u32, pvinpacket: *const ::std::ffi::c_void) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const DOT11EXT_PSK_MAX_LENGTH: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_QUERY_VIRTUAL_STATION_PROPERTIES = unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, pbisvirtualstation: *mut super::super::Foundation::BOOL, pgprimary: *mut ::windows::runtime::GUID, pvreserved: *mut ::std::ffi::c_void) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_RELEASE_VIRTUAL_STATION = unsafe extern "system" fn(hdot11primaryhandle: super::super::Foundation::HANDLE, pvreserved: *mut ::std::ffi::c_void) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_REQUEST_VIRTUAL_STATION = unsafe extern "system" fn(hdot11primaryhandle: super::super::Foundation::HANDLE, pvreserved: *mut ::std::ffi::c_void) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`, `Win32_NetworkManagement_WiFi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
pub type DOT11EXT_SEND_NOTIFICATION = unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, pnotificationdata: *const super::WiFi::L2_NOTIFICATION_DATA) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_SEND_PACKET = unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, upacketlen: u32, pvpacket: *const ::std::ffi::c_void, hsendcompletion: super::super::Foundation::HANDLE) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_SEND_UI_REQUEST = unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, pihvuirequest: *const DOT11EXT_IHV_UI_REQUEST) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_SET_AUTH_ALGORITHM = unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, dwauthalgo: u32) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_SET_CURRENT_PROFILE = unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, hconnectsession: super::super::Foundation::HANDLE, pihvconnprofile: *const DOT11EXT_IHV_CONNECTIVITY_PROFILE, pihvsecprofile: *const DOT11EXT_IHV_SECURITY_PROFILE) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`, `Win32_NetworkManagement_WiFi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
pub type DOT11EXT_SET_DEFAULT_KEY = unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, pkey: *const super::WiFi::DOT11_CIPHER_DEFAULT_KEY_VALUE, dot11direction: super::WiFi::DOT11_DIRECTION) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_SET_DEFAULT_KEY_ID = unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, udefaultkeyid: u32) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`, `Win32_NetworkManagement_WiFi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
pub type DOT11EXT_SET_ETHERTYPE_HANDLING = unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, umaxbacklog: u32, unumofexemption: u32, pexemption: *const super::WiFi::DOT11_PRIVACY_EXEMPTION, unumofregistration: u32, pusregistration: *const u16) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_SET_EXCLUDE_UNENCRYPTED = unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, bexcludeunencrypted: super::super::Foundation::BOOL) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`, `Win32_NetworkManagement_WiFi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
pub type DOT11EXT_SET_KEY_MAPPING_KEY = unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, pkey: *const super::WiFi::DOT11_CIPHER_KEY_MAPPING_KEY_VALUE) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_SET_MULTICAST_CIPHER_ALGORITHM = unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, dwmulticastcipheralgo: u32) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_SET_PROFILE_CUSTOM_USER_DATA = unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, hconnectsession: super::super::Foundation::HANDLE, dwsessionid: u32, dwdatasize: u32, pvdata: *const ::std::ffi::c_void) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_SET_UNICAST_CIPHER_ALGORITHM = unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, dwunicastcipheralgo: u32) -> u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`, `Win32_NetworkManagement_WiFi`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
pub type DOT11EXT_SET_VIRTUAL_STATION_AP_PROPERTIES = unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, hconnectsession: super::super::Foundation::HANDLE, dwnumproperties: u32, pproperties: *const DOT11EXT_VIRTUAL_STATION_AP_PROPERTY, pvreserved: *mut ::std::ffi::c_void) -> u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`, `Win32_NetworkManagement_WiFi`*"]
pub struct DOT11EXT_VIRTUAL_STATION_APIS {
    pub Dot11ExtRequestVirtualStation: ::std::option::Option<DOT11EXT_REQUEST_VIRTUAL_STATION>,
    pub Dot11ExtReleaseVirtualStation: ::std::option::Option<DOT11EXT_RELEASE_VIRTUAL_STATION>,
    pub Dot11ExtQueryVirtualStationProperties: ::std::option::Option<DOT11EXT_QUERY_VIRTUAL_STATION_PROPERTIES>,
    pub Dot11ExtSetVirtualStationAPProperties: ::std::option::Option<DOT11EXT_SET_VIRTUAL_STATION_AP_PROPERTIES>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
impl DOT11EXT_VIRTUAL_STATION_APIS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
impl ::std::default::Default for DOT11EXT_VIRTUAL_STATION_APIS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
impl ::std::fmt::Debug for DOT11EXT_VIRTUAL_STATION_APIS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DOT11EXT_VIRTUAL_STATION_APIS").finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
impl ::std::cmp::PartialEq for DOT11EXT_VIRTUAL_STATION_APIS {
    fn eq(&self, other: &Self) -> bool {
        self.Dot11ExtRequestVirtualStation.map(|f| f as usize) == other.Dot11ExtRequestVirtualStation.map(|f| f as usize)
            && self.Dot11ExtReleaseVirtualStation.map(|f| f as usize) == other.Dot11ExtReleaseVirtualStation.map(|f| f as usize)
            && self.Dot11ExtQueryVirtualStationProperties.map(|f| f as usize) == other.Dot11ExtQueryVirtualStationProperties.map(|f| f as usize)
            && self.Dot11ExtSetVirtualStationAPProperties.map(|f| f as usize) == other.Dot11ExtSetVirtualStationAPProperties.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
impl ::std::cmp::Eq for DOT11EXT_VIRTUAL_STATION_APIS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
unsafe impl ::windows::runtime::Abi for DOT11EXT_VIRTUAL_STATION_APIS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`, `Win32_NetworkManagement_WiFi`*"]
pub struct DOT11EXT_VIRTUAL_STATION_AP_PROPERTY {
    pub dot11SSID: super::WiFi::DOT11_SSID,
    pub dot11AuthAlgo: super::WiFi::DOT11_AUTH_ALGORITHM,
    pub dot11CipherAlgo: super::WiFi::DOT11_CIPHER_ALGORITHM,
    pub bIsPassPhrase: super::super::Foundation::BOOL,
    pub dwKeyLength: u32,
    pub ucKeyData: [u8; 64],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
impl DOT11EXT_VIRTUAL_STATION_AP_PROPERTY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
impl ::std::default::Default for DOT11EXT_VIRTUAL_STATION_AP_PROPERTY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
impl ::std::fmt::Debug for DOT11EXT_VIRTUAL_STATION_AP_PROPERTY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DOT11EXT_VIRTUAL_STATION_AP_PROPERTY")
            .field("dot11SSID", &self.dot11SSID)
            .field("dot11AuthAlgo", &self.dot11AuthAlgo)
            .field("dot11CipherAlgo", &self.dot11CipherAlgo)
            .field("bIsPassPhrase", &self.bIsPassPhrase)
            .field("dwKeyLength", &self.dwKeyLength)
            .field("ucKeyData", &self.ucKeyData)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
impl ::std::cmp::PartialEq for DOT11EXT_VIRTUAL_STATION_AP_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.dot11SSID == other.dot11SSID && self.dot11AuthAlgo == other.dot11AuthAlgo && self.dot11CipherAlgo == other.dot11CipherAlgo && self.bIsPassPhrase == other.bIsPassPhrase && self.dwKeyLength == other.dwKeyLength && self.ucKeyData == other.ucKeyData
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
impl ::std::cmp::Eq for DOT11EXT_VIRTUAL_STATION_AP_PROPERTY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
unsafe impl ::windows::runtime::Abi for DOT11EXT_VIRTUAL_STATION_AP_PROPERTY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`, `Win32_NetworkManagement_WiFi`*"]
pub struct DOT11_ADAPTER {
    pub gAdapterId: ::windows::runtime::GUID,
    pub pszDescription: super::super::Foundation::PWSTR,
    pub Dot11CurrentOpMode: super::WiFi::DOT11_CURRENT_OPERATION_MODE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
impl DOT11_ADAPTER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
impl ::std::default::Default for DOT11_ADAPTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
impl ::std::fmt::Debug for DOT11_ADAPTER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DOT11_ADAPTER").field("gAdapterId", &self.gAdapterId).field("pszDescription", &self.pszDescription).field("Dot11CurrentOpMode", &self.Dot11CurrentOpMode).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
impl ::std::cmp::PartialEq for DOT11_ADAPTER {
    fn eq(&self, other: &Self) -> bool {
        self.gAdapterId == other.gAdapterId && self.pszDescription == other.pszDescription && self.Dot11CurrentOpMode == other.Dot11CurrentOpMode
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
impl ::std::cmp::Eq for DOT11_ADAPTER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi"))]
unsafe impl ::windows::runtime::Abi for DOT11_ADAPTER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct DOT11_BSS_LIST {
    pub uNumOfBytes: u32,
    pub pucBuffer: *mut u8,
}
impl DOT11_BSS_LIST {}
impl ::std::default::Default for DOT11_BSS_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DOT11_BSS_LIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DOT11_BSS_LIST").field("uNumOfBytes", &self.uNumOfBytes).field("pucBuffer", &self.pucBuffer).finish()
    }
}
impl ::std::cmp::PartialEq for DOT11_BSS_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.uNumOfBytes == other.uNumOfBytes && self.pucBuffer == other.pucBuffer
    }
}
impl ::std::cmp::Eq for DOT11_BSS_LIST {}
unsafe impl ::windows::runtime::Abi for DOT11_BSS_LIST {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub struct DOT11_EAP_RESULT {
    pub dwFailureReasonCode: u32,
    pub pAttribArray: *mut super::super::Security::ExtensibleAuthenticationProtocol::EAP_ATTRIBUTES,
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl DOT11_EAP_RESULT {}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::std::default::Default for DOT11_EAP_RESULT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::std::fmt::Debug for DOT11_EAP_RESULT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DOT11_EAP_RESULT").field("dwFailureReasonCode", &self.dwFailureReasonCode).field("pAttribArray", &self.pAttribArray).finish()
    }
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::std::cmp::PartialEq for DOT11_EAP_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.dwFailureReasonCode == other.dwFailureReasonCode && self.pAttribArray == other.pAttribArray
    }
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::std::cmp::Eq for DOT11_EAP_RESULT {}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
unsafe impl ::windows::runtime::Abi for DOT11_EAP_RESULT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct DOT11_IHV_VERSION_INFO {
    pub dwVerMin: u32,
    pub dwVerMax: u32,
}
impl DOT11_IHV_VERSION_INFO {}
impl ::std::default::Default for DOT11_IHV_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DOT11_IHV_VERSION_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DOT11_IHV_VERSION_INFO").field("dwVerMin", &self.dwVerMin).field("dwVerMax", &self.dwVerMax).finish()
    }
}
impl ::std::cmp::PartialEq for DOT11_IHV_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVerMin == other.dwVerMin && self.dwVerMax == other.dwVerMax
    }
}
impl ::std::cmp::Eq for DOT11_IHV_VERSION_INFO {}
unsafe impl ::windows::runtime::Abi for DOT11_IHV_VERSION_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DOT11_MSONEX_RESULT(pub i32);
pub const DOT11_MSONEX_SUCCESS: DOT11_MSONEX_RESULT = DOT11_MSONEX_RESULT(0i32);
pub const DOT11_MSONEX_FAILURE: DOT11_MSONEX_RESULT = DOT11_MSONEX_RESULT(1i32);
pub const DOT11_MSONEX_IN_PROGRESS: DOT11_MSONEX_RESULT = DOT11_MSONEX_RESULT(2i32);
impl ::std::convert::From<i32> for DOT11_MSONEX_RESULT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DOT11_MSONEX_RESULT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_NetworkManagement_WiFi`, `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub struct DOT11_MSONEX_RESULT_PARAMS {
    pub Dot11OnexAuthStatus: super::WiFi::ONEX_AUTH_STATUS,
    pub Dot11OneXReasonCode: super::WiFi::ONEX_REASON_CODE,
    pub pbMPPESendKey: *mut u8,
    pub dwMPPESendKeyLen: u32,
    pub pbMPPERecvKey: *mut u8,
    pub dwMPPERecvKeyLen: u32,
    pub pDot11EapResult: *mut DOT11_EAP_RESULT,
}
#[cfg(all(feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl DOT11_MSONEX_RESULT_PARAMS {}
#[cfg(all(feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::std::default::Default for DOT11_MSONEX_RESULT_PARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::std::fmt::Debug for DOT11_MSONEX_RESULT_PARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DOT11_MSONEX_RESULT_PARAMS")
            .field("Dot11OnexAuthStatus", &self.Dot11OnexAuthStatus)
            .field("Dot11OneXReasonCode", &self.Dot11OneXReasonCode)
            .field("pbMPPESendKey", &self.pbMPPESendKey)
            .field("dwMPPESendKeyLen", &self.dwMPPESendKeyLen)
            .field("pbMPPERecvKey", &self.pbMPPERecvKey)
            .field("dwMPPERecvKeyLen", &self.dwMPPERecvKeyLen)
            .field("pDot11EapResult", &self.pDot11EapResult)
            .finish()
    }
}
#[cfg(all(feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::std::cmp::PartialEq for DOT11_MSONEX_RESULT_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.Dot11OnexAuthStatus == other.Dot11OnexAuthStatus && self.Dot11OneXReasonCode == other.Dot11OneXReasonCode && self.pbMPPESendKey == other.pbMPPESendKey && self.dwMPPESendKeyLen == other.dwMPPESendKeyLen && self.pbMPPERecvKey == other.pbMPPERecvKey && self.dwMPPERecvKeyLen == other.dwMPPERecvKeyLen && self.pDot11EapResult == other.pDot11EapResult
    }
}
#[cfg(all(feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::std::cmp::Eq for DOT11_MSONEX_RESULT_PARAMS {}
#[cfg(all(feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
unsafe impl ::windows::runtime::Abi for DOT11_MSONEX_RESULT_PARAMS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`, `Win32_NetworkManagement_WiFi`, `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub struct DOT11_MSSECURITY_SETTINGS {
    pub dot11AuthAlgorithm: super::WiFi::DOT11_AUTH_ALGORITHM,
    pub dot11CipherAlgorithm: super::WiFi::DOT11_CIPHER_ALGORITHM,
    pub fOneXEnabled: super::super::Foundation::BOOL,
    pub eapMethodType: super::super::Security::ExtensibleAuthenticationProtocol::EAP_METHOD_TYPE,
    pub dwEapConnectionDataLen: u32,
    pub pEapConnectionData: *mut u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl DOT11_MSSECURITY_SETTINGS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::std::default::Default for DOT11_MSSECURITY_SETTINGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::std::fmt::Debug for DOT11_MSSECURITY_SETTINGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DOT11_MSSECURITY_SETTINGS")
            .field("dot11AuthAlgorithm", &self.dot11AuthAlgorithm)
            .field("dot11CipherAlgorithm", &self.dot11CipherAlgorithm)
            .field("fOneXEnabled", &self.fOneXEnabled)
            .field("eapMethodType", &self.eapMethodType)
            .field("dwEapConnectionDataLen", &self.dwEapConnectionDataLen)
            .field("pEapConnectionData", &self.pEapConnectionData)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::std::cmp::PartialEq for DOT11_MSSECURITY_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.dot11AuthAlgorithm == other.dot11AuthAlgorithm && self.dot11CipherAlgorithm == other.dot11CipherAlgorithm && self.fOneXEnabled == other.fOneXEnabled && self.eapMethodType == other.eapMethodType && self.dwEapConnectionDataLen == other.dwEapConnectionDataLen && self.pEapConnectionData == other.pEapConnectionData
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::std::cmp::Eq for DOT11_MSSECURITY_SETTINGS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WiFi", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
unsafe impl ::windows::runtime::Abi for DOT11_MSSECURITY_SETTINGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
pub struct DOT11_PORT_STATE {
    pub PeerMacAddress: [u8; 6],
    pub uSessionId: u32,
    pub bPortControlled: super::super::Foundation::BOOL,
    pub bPortAuthorized: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl DOT11_PORT_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DOT11_PORT_STATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DOT11_PORT_STATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DOT11_PORT_STATE").field("PeerMacAddress", &self.PeerMacAddress).field("uSessionId", &self.uSessionId).field("bPortControlled", &self.bPortControlled).field("bPortAuthorized", &self.bPortAuthorized).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DOT11_PORT_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.PeerMacAddress == other.PeerMacAddress && self.uSessionId == other.uSessionId && self.bPortControlled == other.bPortControlled && self.bPortAuthorized == other.bPortAuthorized
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DOT11_PORT_STATE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DOT11_PORT_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const DOT11_RSN_KCK_LENGTH: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const DOT11_RSN_KEK_LENGTH: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const DOT11_RSN_MAX_CIPHER_KEY_LENGTH: u32 = 32u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct DOT11_SECURITY_PACKET_HEADER {
    pub PeerMac: [u8; 6],
    pub usEtherType: u16,
    pub Data: [u8; 1],
}
impl DOT11_SECURITY_PACKET_HEADER {}
impl ::std::default::Default for DOT11_SECURITY_PACKET_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DOT11_SECURITY_PACKET_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DOT11_SECURITY_PACKET_HEADER {}
unsafe impl ::windows::runtime::Abi for DOT11_SECURITY_PACKET_HEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const EAPOL_REQUEST_ID_WOL_FLAG_MUST_ENCRYPT: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const ETHERNET_LENGTH_OF_ADDRESS: u32 = 6u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct GEN_GET_NETCARD_TIME {
    pub ReadTime: u64,
}
impl GEN_GET_NETCARD_TIME {}
impl ::std::default::Default for GEN_GET_NETCARD_TIME {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GEN_GET_NETCARD_TIME {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GEN_GET_NETCARD_TIME").field("ReadTime", &self.ReadTime).finish()
    }
}
impl ::std::cmp::PartialEq for GEN_GET_NETCARD_TIME {
    fn eq(&self, other: &Self) -> bool {
        self.ReadTime == other.ReadTime
    }
}
impl ::std::cmp::Eq for GEN_GET_NETCARD_TIME {}
unsafe impl ::windows::runtime::Abi for GEN_GET_NETCARD_TIME {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct GEN_GET_TIME_CAPS {
    pub Flags: u32,
    pub ClockPrecision: u32,
}
impl GEN_GET_TIME_CAPS {}
impl ::std::default::Default for GEN_GET_TIME_CAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GEN_GET_TIME_CAPS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GEN_GET_TIME_CAPS").field("Flags", &self.Flags).field("ClockPrecision", &self.ClockPrecision).finish()
    }
}
impl ::std::cmp::PartialEq for GEN_GET_TIME_CAPS {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.ClockPrecision == other.ClockPrecision
    }
}
impl ::std::cmp::Eq for GEN_GET_TIME_CAPS {}
unsafe impl ::windows::runtime::Abi for GEN_GET_TIME_CAPS {
    type Abi = Self;
}
pub const GUID_DEVINTERFACE_NET: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3402138756, 29973, 19459, [130, 230, 113, 168, 122, 186, 195, 97]);
pub const GUID_DEVINTERFACE_NETUIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(137588576, 1657, 19564, [133, 210, 174, 124, 237, 101, 255, 247]);
pub const GUID_NDIS_802_11_ADD_KEY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2878036578, 7505, 18904, [186, 92, 250, 152, 11, 224, 58, 29]);
pub const GUID_NDIS_802_11_ADD_WEP: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1124581360, 8489, 4564, [151, 235, 0, 192, 79, 121, 196, 3]);
pub const GUID_NDIS_802_11_ASSOCIATION_INFORMATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2693615056, 38414, 16573, [140, 246, 197, 56, 175, 152, 242, 227]);
pub const GUID_NDIS_802_11_AUTHENTICATION_MODE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1133644324, 8489, 4564, [151, 235, 0, 192, 79, 121, 196, 3]);
pub const GUID_NDIS_802_11_BSSID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(621065922, 8101, 4564, [151, 235, 0, 192, 79, 121, 196, 3]);
pub const GUID_NDIS_802_11_BSSID_LIST: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1767010202, 8290, 4564, [151, 235, 0, 192, 79, 121, 196, 3]);
pub const GUID_NDIS_802_11_BSSID_LIST_SCAN: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(228458977, 47728, 4564, [182, 117, 0, 32, 72, 87, 3, 55]);
pub const GUID_NDIS_802_11_CONFIGURATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1246624130, 8296, 4564, [151, 235, 0, 192, 79, 121, 196, 3]);
pub const GUID_NDIS_802_11_DESIRED_RATES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1160700046, 9526, 4564, [151, 235, 0, 192, 79, 121, 196, 3]);
pub const GUID_NDIS_802_11_DISASSOCIATE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1130831680, 8489, 4564, [151, 235, 0, 192, 79, 121, 196, 3]);
pub const GUID_NDIS_802_11_FRAGMENTATION_THRESHOLD: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1772791748, 8290, 4564, [151, 235, 0, 192, 79, 121, 196, 3]);
pub const GUID_NDIS_802_11_INFRASTRUCTURE_MODE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1769822846, 8290, 4564, [151, 235, 0, 192, 79, 121, 196, 3]);
pub const GUID_NDIS_802_11_MEDIA_STREAM_MODE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(173453158, 55371, 18923, [162, 141, 82, 130, 203, 182, 208, 205]);
pub const GUID_NDIS_802_11_NETWORK_TYPES_SUPPORTED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2234636006, 8257, 4564, [151, 235, 0, 192, 79, 121, 196, 3]);
pub const GUID_NDIS_802_11_NETWORK_TYPE_IN_USE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2239636262, 8257, 4564, [151, 235, 0, 192, 79, 121, 196, 3]);
pub const GUID_NDIS_802_11_NUMBER_OF_ANTENNAS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(24613686, 8292, 4564, [151, 235, 0, 192, 79, 121, 196, 3]);
pub const GUID_NDIS_802_11_POWER_MODE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2243855228, 8257, 4564, [151, 235, 0, 192, 79, 121, 196, 3]);
pub const GUID_NDIS_802_11_PRIVACY_FILTER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1731445993, 18322, 4564, [151, 241, 0, 192, 79, 121, 196, 3]);
pub const GUID_NDIS_802_11_RELOAD_DEFAULTS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1955271912, 13038, 17445, [185, 27, 201, 132, 140, 88, 181, 90]);
pub const GUID_NDIS_802_11_REMOVE_KEY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1942694121, 12680, 17109, [181, 83, 178, 18, 55, 230, 8, 140]);
pub const GUID_NDIS_802_11_REMOVE_WEP: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1128019036, 8489, 4564, [151, 235, 0, 192, 79, 121, 196, 3]);
pub const GUID_NDIS_802_11_RSSI: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(352836374, 8275, 4564, [151, 235, 0, 192, 79, 121, 196, 3]);
pub const GUID_NDIS_802_11_RSSI_TRIGGER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(357992888, 8275, 4564, [151, 235, 0, 192, 79, 121, 196, 3]);
pub const GUID_NDIS_802_11_RTS_THRESHOLD: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(20238462, 8292, 4564, [151, 235, 0, 192, 79, 121, 196, 3]);
pub const GUID_NDIS_802_11_RX_ANTENNA_SELECTED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(28051362, 8292, 4564, [151, 235, 0, 192, 79, 121, 196, 3]);
pub const GUID_NDIS_802_11_SSID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2099941610, 8257, 4564, [151, 235, 0, 192, 79, 121, 196, 3]);
pub const GUID_NDIS_802_11_STATISTICS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1119581104, 8489, 4564, [151, 235, 0, 192, 79, 121, 196, 3]);
pub const GUID_NDIS_802_11_SUPPORTED_RATES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1239123746, 8296, 4564, [151, 235, 0, 192, 79, 121, 196, 3]);
pub const GUID_NDIS_802_11_TEST: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1268556138, 27232, 20125, [146, 12, 99, 53, 149, 63, 160, 181]);
pub const GUID_NDIS_802_11_TX_ANTENNA_SELECTED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(31176522, 8292, 4564, [151, 235, 0, 192, 79, 121, 196, 3]);
pub const GUID_NDIS_802_11_TX_POWER_LEVEL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(300333686, 8275, 4564, [151, 235, 0, 192, 79, 121, 196, 3]);
pub const GUID_NDIS_802_11_WEP_STATUS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2955387423, 15610, 16677, [128, 11, 63, 122, 24, 253, 220, 220]);
pub const GUID_NDIS_802_3_CURRENT_ADDRESS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1148802816, 42523, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_802_3_MAC_OPTIONS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1148802819, 42523, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_802_3_MAXIMUM_LIST_SIZE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1148802818, 42523, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_802_3_MULTICAST_LIST: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1148802817, 42523, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_802_3_PERMANENT_ADDRESS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1148802815, 42523, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_802_3_RCV_ERROR_ALIGNMENT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1148802820, 42523, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_802_3_XMIT_MORE_COLLISIONS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1148802822, 42523, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_802_3_XMIT_ONE_COLLISION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1148802821, 42523, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_802_5_CURRENT_ADDRESS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1148802824, 42523, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_802_5_CURRENT_FUNCTIONAL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1148802825, 42523, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_802_5_CURRENT_GROUP: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1148802826, 42523, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_802_5_CURRENT_RING_STATE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2901491762, 42524, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_802_5_CURRENT_RING_STATUS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2299148012, 42524, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_802_5_LAST_OPEN_STATUS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1148802827, 42523, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_802_5_LINE_ERRORS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2901491763, 42524, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_802_5_LOST_FRAMES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2901491764, 42524, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_802_5_PERMANENT_ADDRESS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1148802823, 42523, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_ENUMERATE_ADAPTER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2552180095, 45555, 4560, [141, 215, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_ENUMERATE_ADAPTERS_EX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(376531223, 17158, 19428, [155, 90, 56, 9, 174, 68, 177, 37]);
pub const GUID_NDIS_ENUMERATE_VC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2552180098, 45555, 4560, [141, 215, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_CO_DRIVER_VERSION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2031800728, 58204, 4560, [150, 146, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_CO_HARDWARE_STATUS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2031800722, 58204, 4560, [150, 146, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_CO_LINK_SPEED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2031800725, 58204, 4560, [150, 146, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_CO_MAC_OPTIONS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2031800730, 58204, 4560, [150, 146, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_CO_MEDIA_CONNECT_STATUS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2031800731, 58204, 4560, [150, 146, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_CO_MEDIA_IN_USE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2031800724, 58204, 4560, [150, 146, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_CO_MEDIA_SUPPORTED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2031800723, 58204, 4560, [150, 146, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_CO_MINIMUM_LINK_SPEED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2031800733, 58204, 4560, [150, 146, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_CO_RCV_PDUS_ERROR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(169953288, 58207, 4560, [150, 146, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_CO_RCV_PDUS_NO_BUFFER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(169953289, 58207, 4560, [150, 146, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_CO_RCV_PDUS_OK: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(169953286, 58207, 4560, [150, 146, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_CO_VENDOR_DESCRIPTION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2031800727, 58204, 4560, [150, 146, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_CO_VENDOR_DRIVER_VERSION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2031800732, 58204, 4560, [150, 146, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_CO_VENDOR_ID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2031800726, 58204, 4560, [150, 146, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_CO_XMIT_PDUS_ERROR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(169953287, 58207, 4560, [150, 146, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_CO_XMIT_PDUS_OK: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(169953285, 58207, 4560, [150, 146, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_CURRENT_LOOKAHEAD: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1589707617, 42522, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_CURRENT_PACKET_FILTER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1589707616, 42522, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_DRIVER_VERSION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1589707618, 42522, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_ENUMERATE_PORTS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4057377768, 5604, 17415, [129, 183, 107, 131, 12, 119, 124, 217]);
pub const GUID_NDIS_GEN_HARDWARE_STATUS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1589707604, 42522, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_INTERRUPT_MODERATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3653824165, 61806, 18044, [132, 213, 99, 69, 162, 44, 226, 19]);
pub const GUID_NDIS_GEN_INTERRUPT_MODERATION_PARAMETERS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3616124410, 40022, 17211, [173, 1, 117, 116, 243, 206, 219, 233]);
pub const GUID_NDIS_GEN_LINK_PARAMETERS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2357015929, 9515, 17940, [130, 197, 166, 80, 218, 161, 80, 73]);
pub const GUID_NDIS_GEN_LINK_SPEED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1589707609, 42522, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_LINK_STATE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3122613268, 43333, 18274, [185, 22, 11, 85, 21, 182, 244, 58]);
pub const GUID_NDIS_GEN_MAC_OPTIONS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1589707621, 42522, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_MAXIMUM_FRAME_SIZE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1589707608, 42522, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_MAXIMUM_LOOKAHEAD: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1589707607, 42522, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_MAXIMUM_SEND_PACKETS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1589707623, 42522, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_MAXIMUM_TOTAL_SIZE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1589707619, 42522, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_MEDIA_CONNECT_STATUS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1589707622, 42522, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_MEDIA_IN_USE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1589707606, 42522, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_MEDIA_SUPPORTED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1589707605, 42522, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_PCI_DEVICE_CUSTOM_PROPERTIES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2855925163, 57952, 19713, [130, 176, 183, 55, 200, 128, 234, 5]);
pub const GUID_NDIS_GEN_PHYSICAL_MEDIUM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1099735405, 14647, 16904, [148, 10, 236, 97, 150, 39, 128, 133]);
pub const GUID_NDIS_GEN_PHYSICAL_MEDIUM_EX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2308863874, 859, 17401, [139, 182, 43, 88, 151, 22, 18, 229]);
pub const GUID_NDIS_GEN_PORT_AUTHENTICATION_PARAMETERS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2864098353, 34555, 18683, [139, 72, 99, 219, 35, 90, 206, 22]);
pub const GUID_NDIS_GEN_PORT_STATE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1874799199, 35727, 18720, [129, 67, 230, 196, 96, 245, 37, 36]);
pub const GUID_NDIS_GEN_RCV_ERROR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1148802813, 42523, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_RCV_NO_BUFFER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1148802814, 42523, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_RCV_OK: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1148802811, 42523, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_RECEIVE_BLOCK_SIZE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1589707613, 42522, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_RECEIVE_BUFFER_SPACE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1589707611, 42522, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_STATISTICS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(915162549, 49449, 17345, [147, 158, 126, 220, 45, 127, 230, 33]);
pub const GUID_NDIS_GEN_TRANSMIT_BLOCK_SIZE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1589707612, 42522, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_TRANSMIT_BUFFER_SPACE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1589707610, 42522, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_VENDOR_DESCRIPTION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1589707615, 42522, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_VENDOR_DRIVER_VERSION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1148802809, 42523, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_VENDOR_ID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1589707614, 42522, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_VLAN_ID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1985857282, 50664, 19303, [132, 59, 63, 90, 79, 242, 100, 139]);
pub const GUID_NDIS_GEN_XMIT_ERROR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1148802812, 42523, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_GEN_XMIT_OK: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1148802810, 42523, 4560, [141, 212, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_HD_SPLIT_CURRENT_CONFIG: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2177970236, 43776, 20041, [128, 177, 94, 110, 11, 249, 190, 83]);
pub const GUID_NDIS_HD_SPLIT_PARAMETERS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2349108202, 10515, 17496, [182, 142, 23, 246, 193, 229, 198, 14]);
pub const GUID_NDIS_LAN_CLASS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2907277636, 30255, 4560, [141, 203, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_NDK_CAPABILITIES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2036972109, 56704, 19399, [179, 230, 104, 4, 57, 151, 229, 25]);
pub const GUID_NDIS_NDK_STATE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1393322441, 12113, 18910, [161, 175, 8, 141, 84, 255, 164, 116]);
pub const GUID_NDIS_NOTIFY_ADAPTER_ARRIVAL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2552180097, 45555, 4560, [141, 215, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_NOTIFY_ADAPTER_REMOVAL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2552180096, 45555, 4560, [141, 215, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_NOTIFY_BIND: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1410552604, 45555, 4560, [141, 215, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_NOTIFY_DEVICE_POWER_OFF: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2176614793, 45094, 18091, [185, 100, 241, 130, 227, 66, 147, 78]);
pub const GUID_NDIS_NOTIFY_DEVICE_POWER_OFF_EX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1096365372, 23767, 17102, [143, 228, 164, 90, 35, 128, 204, 79]);
pub const GUID_NDIS_NOTIFY_DEVICE_POWER_ON: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1602342864, 61510, 17218, [175, 97, 137, 90, 206, 218, 239, 217]);
pub const GUID_NDIS_NOTIFY_DEVICE_POWER_ON_EX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(725877128, 37548, 20320, [155, 45, 32, 163, 12, 187, 107, 190]);
pub const GUID_NDIS_NOTIFY_FILTER_ARRIVAL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(191708297, 22807, 17354, [181, 120, 208, 26, 121, 103, 196, 28]);
pub const GUID_NDIS_NOTIFY_FILTER_REMOVAL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(521632985, 22869, 18209, [159, 106, 120, 235, 223, 174, 248, 137]);
pub const GUID_NDIS_NOTIFY_UNBIND: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1849483756, 45555, 4560, [141, 215, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_NOTIFY_VC_ARRIVAL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(405773836, 45555, 4560, [141, 215, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_NOTIFY_VC_REMOVAL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2552180089, 45555, 4560, [141, 215, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_PM_ACTIVE_CAPABILITIES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2999940835, 45998, 17300, [160, 31, 51, 140, 152, 112, 233, 57]);
pub const GUID_NDIS_PM_ADMIN_CONFIG: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(354996497, 28810, 19620, [146, 21, 192, 87, 113, 22, 28, 218]);
pub const GUID_NDIS_RECEIVE_FILTER_ENUM_FILTERS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1059853341, 33724, 4573, [148, 184, 0, 29, 9, 22, 43, 195]);
pub const GUID_NDIS_RECEIVE_FILTER_ENUM_QUEUES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1059853339, 33724, 4573, [148, 184, 0, 29, 9, 22, 43, 195]);
pub const GUID_NDIS_RECEIVE_FILTER_GLOBAL_PARAMETERS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1059853338, 33724, 4573, [148, 184, 0, 29, 9, 22, 43, 195]);
pub const GUID_NDIS_RECEIVE_FILTER_HARDWARE_CAPABILITIES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1059853337, 33724, 4573, [148, 184, 0, 29, 9, 22, 43, 195]);
pub const GUID_NDIS_RECEIVE_FILTER_PARAMETERS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1059853342, 33724, 4573, [148, 184, 0, 29, 9, 22, 43, 195]);
pub const GUID_NDIS_RECEIVE_FILTER_QUEUE_PARAMETERS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1059853340, 33724, 4573, [148, 184, 0, 29, 9, 22, 43, 195]);
pub const GUID_NDIS_RECEIVE_SCALE_CAPABILITIES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(650282868, 16978, 18686, [166, 16, 165, 138, 57, 140, 14, 177]);
pub const GUID_NDIS_RSS_ENABLED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2506476885, 13314, 20018, [165, 182, 47, 20, 63, 47, 44, 48]);
pub const GUID_NDIS_STATUS_DOT11_ASSOCIATION_COMPLETION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1166786215, 17828, 19170, [177, 118, 229, 31, 150, 252, 5, 104]);
pub const GUID_NDIS_STATUS_DOT11_ASSOCIATION_START: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(958891067, 27008, 19272, [177, 91, 77, 229, 9, 119, 172, 64]);
pub const GUID_NDIS_STATUS_DOT11_CONNECTION_COMPLETION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2532301257, 32539, 19081, [188, 4, 62, 158, 39, 23, 101, 241]);
pub const GUID_NDIS_STATUS_DOT11_CONNECTION_START: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2071210397, 39311, 17492, [173, 8, 197, 175, 40, 87, 109, 27]);
pub const GUID_NDIS_STATUS_DOT11_DISASSOCIATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1069463292, 4066, 17405, [178, 173, 189, 153, 181, 249, 62, 19]);
pub const GUID_NDIS_STATUS_DOT11_LINK_QUALITY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2737328516, 60057, 18669, [130, 94, 164, 38, 177, 28, 39, 84]);
pub const GUID_NDIS_STATUS_DOT11_MPDU_MAX_LENGTH_CHANGED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(493183212, 36424, 19006, [159, 213, 160, 27, 105, 141, 182, 197]);
pub const GUID_NDIS_STATUS_DOT11_PHY_STATE_CHANGED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3736359702, 29109, 18230, [189, 239, 10, 158, 159, 78, 98, 220]);
pub const GUID_NDIS_STATUS_DOT11_PMKID_CANDIDATE_LIST: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(651737334, 56194, 18923, [139, 243, 76, 19, 14, 240, 105, 80]);
pub const GUID_NDIS_STATUS_DOT11_ROAMING_COMPLETION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3718072273, 10283, 16868, [185, 36, 102, 54, 136, 23, 252, 211]);
pub const GUID_NDIS_STATUS_DOT11_ROAMING_START: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2990615821, 9928, 20302, [147, 223, 247, 183, 5, 160, 180, 51]);
pub const GUID_NDIS_STATUS_DOT11_SCAN_CONFIRM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2231392542, 41159, 20219, [147, 66, 182, 116, 176, 2, 203, 230]);
pub const GUID_NDIS_STATUS_DOT11_TKIPMIC_FAILURE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1143745252, 39877, 19344, [168, 137, 69, 94, 242, 32, 244, 238]);
pub const GUID_NDIS_STATUS_EXTERNAL_CONNECTIVITY_CHANGE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4247808372, 50208, 17459, [176, 254, 76, 246, 166, 19, 245, 159]);
pub const GUID_NDIS_STATUS_HD_SPLIT_CURRENT_CONFIG: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1819560718, 61084, 16901, [144, 162, 1, 95, 109, 101, 244, 3]);
pub const GUID_NDIS_STATUS_LINK_SPEED_CHANGE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2552180101, 45555, 4560, [141, 215, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_STATUS_LINK_STATE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1690761111, 34700, 17169, [146, 70, 101, 219, 168, 156, 58, 97]);
pub const GUID_NDIS_STATUS_MEDIA_CONNECT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2552180093, 45555, 4560, [141, 215, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_STATUS_MEDIA_DISCONNECT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2552180094, 45555, 4560, [141, 215, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_STATUS_MEDIA_SPECIFIC_INDICATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2552180100, 45555, 4560, [141, 215, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_STATUS_NETWORK_CHANGE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3398063865, 52865, 16614, [167, 15, 160, 103, 164, 118, 233, 233]);
pub const GUID_NDIS_STATUS_OPER_STATUS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4179080803, 33886, 19773, [182, 212, 21, 235, 39, 175, 129, 197]);
pub const GUID_NDIS_STATUS_PACKET_FILTER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3564917767, 11893, 18141, [129, 70, 29, 126, 210, 214, 171, 29]);
pub const GUID_NDIS_STATUS_PM_OFFLOAD_REJECTED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2916209793, 28958, 19738, [146, 202, 166, 45, 185, 50, 151, 18]);
pub const GUID_NDIS_STATUS_PM_WAKE_REASON: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(154402136, 51810, 17295, [131, 218, 223, 193, 204, 203, 129, 69]);
pub const GUID_NDIS_STATUS_PM_WOL_PATTERN_REJECTED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4146919054, 6356, 19811, [154, 25, 230, 155, 19, 145, 107, 26]);
pub const GUID_NDIS_STATUS_PORT_STATE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(497815038, 17381, 17591, [183, 89, 123, 244, 109, 227, 46, 129]);
pub const GUID_NDIS_STATUS_RESET_END: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2552180087, 45555, 4560, [141, 215, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_STATUS_RESET_START: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2552180086, 45555, 4560, [141, 215, 0, 192, 79, 195, 53, 140]);
pub const GUID_NDIS_STATUS_TASK_OFFLOAD_CURRENT_CONFIG: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1157930950, 21720, 16584, [156, 61, 176, 17, 196, 231, 21, 188]);
pub const GUID_NDIS_STATUS_TASK_OFFLOAD_HARDWARE_CAPABILITIES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3065517451, 8572, 19242, [190, 134, 106, 4, 190, 234, 101, 184]);
pub const GUID_NDIS_STATUS_TCP_CONNECTION_OFFLOAD_CURRENT_CONFIG: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4176326399, 9444, 19174, [164, 19, 11, 39, 247, 107, 36, 61]);
pub const GUID_NDIS_STATUS_TCP_CONNECTION_OFFLOAD_HARDWARE_CAPABILITIES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(957966774, 16428, 17343, [137, 34, 57, 234, 224, 218, 27, 181]);
pub const GUID_NDIS_SWITCH_MICROSOFT_VENDOR_ID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(539314174, 7324, 16569, [187, 161, 8, 173, 161, 249, 139, 60]);
pub const GUID_NDIS_SWITCH_PORT_PROPERTY_PROFILE_ID_DEFAULT_EXTERNAL_NIC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(187988038, 2572, 18186, [155, 122, 13, 150, 88, 80, 105, 143]);
pub const GUID_NDIS_TCP_CONNECTION_OFFLOAD_CURRENT_CONFIG: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(786870001, 2129, 17803, [191, 13, 121, 35, 67, 209, 205, 225]);
pub const GUID_NDIS_TCP_CONNECTION_OFFLOAD_HARDWARE_CAPABILITIES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2363957036, 54842, 17296, [164, 135, 24, 250, 71, 38, 44, 235]);
pub const GUID_NDIS_TCP_OFFLOAD_CURRENT_CONFIG: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1750347757, 23668, 17950, [137, 52, 145, 198, 249, 198, 9, 96]);
pub const GUID_NDIS_TCP_OFFLOAD_HARDWARE_CAPABILITIES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3445559554, 22799, 19162, [171, 101, 91, 49, 177, 220, 1, 114]);
pub const GUID_NDIS_TCP_OFFLOAD_PARAMETERS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2393741858, 32617, 19398, [148, 154, 200, 24, 123, 7, 78, 97]);
pub const GUID_NDIS_TCP_RSC_STATISTICS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2198881349, 39773, 20198, [162, 165, 43, 211, 251, 60, 54, 175]);
pub const GUID_NDIS_WAKE_ON_MAGIC_PACKET_ONLY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2706316439, 34873, 20362, [153, 150, 162, 137, 150, 235, 191, 29]);
pub const GUID_NIC_SWITCH_CURRENT_CAPABILITIES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3882867443, 3047, 19861, [135, 233, 90, 234, 212, 181, 144, 233]);
pub const GUID_NIC_SWITCH_HARDWARE_CAPABILITIES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(936031244, 53736, 17153, [140, 29, 88, 70, 94, 12, 76, 15]);
pub const GUID_PM_ADD_PROTOCOL_OFFLOAD: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(201769234, 3475, 17307, [158, 109, 38, 190, 19, 12, 151, 132]);
pub const GUID_PM_ADD_WOL_PATTERN: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1875393447, 21180, 20394, [172, 81, 125, 47, 254, 99, 186, 144]);
pub const GUID_PM_CURRENT_CAPABILITIES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(985513236, 54346, 19007, [154, 99, 160, 164, 42, 81, 177, 49]);
pub const GUID_PM_GET_PROTOCOL_OFFLOAD: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2789432537, 5279, 18830, [149, 27, 45, 148, 190, 163, 227, 163]);
pub const GUID_PM_HARDWARE_CAPABILITIES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3974444557, 12945, 19054, [128, 68, 0, 81, 31, 237, 39, 238]);
pub const GUID_PM_PARAMETERS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1442989522, 57937, 16540, [162, 128, 49, 25, 53, 190, 59, 40]);
pub const GUID_PM_PROTOCOL_OFFLOAD_LIST: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1936639403, 51855, 16451, [187, 88, 218, 64, 42, 72, 217, 204]);
pub const GUID_PM_REMOVE_PROTOCOL_OFFLOAD: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3738008546, 42672, 17354, [174, 69, 208, 0, 210, 14, 82, 101]);
pub const GUID_PM_REMOVE_WOL_PATTERN: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2688002325, 50890, 17186, [179, 227, 239, 117, 78, 196, 152, 220]);
pub const GUID_PM_WOL_PATTERN_LIST: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1076018743, 32482, 18366, [165, 165, 5, 15, 199, 154, 252, 117]);
pub const GUID_RECEIVE_FILTER_CURRENT_CAPABILITIES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1079306255, 11201, 19660, [176, 51, 74, 188, 12, 74, 30, 140]);
pub const GUID_STATUS_MEDIA_SPECIFIC_INDICATION_EX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2863463591, 38218, 17970, [161, 110, 168, 166, 55, 147, 169, 229]);
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const IOCTL_NDIS_RESERVED5: u32 = 1507380u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const IOCTL_NDIS_RESERVED6: u32 = 1540152u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const IPSEC_OFFLOAD_V2_AND_TCP_CHECKSUM_COEXISTENCE: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const IPSEC_OFFLOAD_V2_AND_UDP_CHECKSUM_COEXISTENCE: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const IPSEC_OFFLOAD_V2_AUTHENTICATION_AES_GCM_128: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const IPSEC_OFFLOAD_V2_AUTHENTICATION_AES_GCM_192: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const IPSEC_OFFLOAD_V2_AUTHENTICATION_AES_GCM_256: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const IPSEC_OFFLOAD_V2_AUTHENTICATION_MD5: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const IPSEC_OFFLOAD_V2_AUTHENTICATION_SHA_1: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const IPSEC_OFFLOAD_V2_AUTHENTICATION_SHA_256: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const IPSEC_OFFLOAD_V2_ENCRYPTION_3_DES_CBC: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const IPSEC_OFFLOAD_V2_ENCRYPTION_AES_CBC_128: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const IPSEC_OFFLOAD_V2_ENCRYPTION_AES_CBC_192: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const IPSEC_OFFLOAD_V2_ENCRYPTION_AES_CBC_256: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const IPSEC_OFFLOAD_V2_ENCRYPTION_AES_GCM_128: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const IPSEC_OFFLOAD_V2_ENCRYPTION_AES_GCM_192: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const IPSEC_OFFLOAD_V2_ENCRYPTION_AES_GCM_256: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const IPSEC_OFFLOAD_V2_ENCRYPTION_DES_CBC: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const IPSEC_OFFLOAD_V2_ENCRYPTION_NONE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const MAXIMUM_IP_OPER_STATUS_ADDRESS_FAMILIES_SUPPORTED: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const MS_MAX_PROFILE_NAME_LENGTH: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const MS_PROFILE_GROUP_POLICY: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const MS_PROFILE_USER: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_802_11_AI_REQFI {
    pub Capabilities: u16,
    pub ListenInterval: u16,
    pub CurrentAPAddress: [u8; 6],
}
impl NDIS_802_11_AI_REQFI {}
impl ::std::default::Default for NDIS_802_11_AI_REQFI {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_802_11_AI_REQFI {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_802_11_AI_REQFI").field("Capabilities", &self.Capabilities).field("ListenInterval", &self.ListenInterval).field("CurrentAPAddress", &self.CurrentAPAddress).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_802_11_AI_REQFI {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities && self.ListenInterval == other.ListenInterval && self.CurrentAPAddress == other.CurrentAPAddress
    }
}
impl ::std::cmp::Eq for NDIS_802_11_AI_REQFI {}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_AI_REQFI {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_802_11_AI_REQFI_CAPABILITIES: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_802_11_AI_REQFI_CURRENTAPADDRESS: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_802_11_AI_REQFI_LISTENINTERVAL: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_802_11_AI_RESFI {
    pub Capabilities: u16,
    pub StatusCode: u16,
    pub AssociationId: u16,
}
impl NDIS_802_11_AI_RESFI {}
impl ::std::default::Default for NDIS_802_11_AI_RESFI {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_802_11_AI_RESFI {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_802_11_AI_RESFI").field("Capabilities", &self.Capabilities).field("StatusCode", &self.StatusCode).field("AssociationId", &self.AssociationId).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_802_11_AI_RESFI {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities && self.StatusCode == other.StatusCode && self.AssociationId == other.AssociationId
    }
}
impl ::std::cmp::Eq for NDIS_802_11_AI_RESFI {}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_AI_RESFI {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_802_11_AI_RESFI_ASSOCIATIONID: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_802_11_AI_RESFI_CAPABILITIES: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_802_11_AI_RESFI_STATUSCODE: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_802_11_ASSOCIATION_INFORMATION {
    pub Length: u32,
    pub AvailableRequestFixedIEs: u16,
    pub RequestFixedIEs: NDIS_802_11_AI_REQFI,
    pub RequestIELength: u32,
    pub OffsetRequestIEs: u32,
    pub AvailableResponseFixedIEs: u16,
    pub ResponseFixedIEs: NDIS_802_11_AI_RESFI,
    pub ResponseIELength: u32,
    pub OffsetResponseIEs: u32,
}
impl NDIS_802_11_ASSOCIATION_INFORMATION {}
impl ::std::default::Default for NDIS_802_11_ASSOCIATION_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_802_11_ASSOCIATION_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_802_11_ASSOCIATION_INFORMATION")
            .field("Length", &self.Length)
            .field("AvailableRequestFixedIEs", &self.AvailableRequestFixedIEs)
            .field("RequestFixedIEs", &self.RequestFixedIEs)
            .field("RequestIELength", &self.RequestIELength)
            .field("OffsetRequestIEs", &self.OffsetRequestIEs)
            .field("AvailableResponseFixedIEs", &self.AvailableResponseFixedIEs)
            .field("ResponseFixedIEs", &self.ResponseFixedIEs)
            .field("ResponseIELength", &self.ResponseIELength)
            .field("OffsetResponseIEs", &self.OffsetResponseIEs)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_802_11_ASSOCIATION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.AvailableRequestFixedIEs == other.AvailableRequestFixedIEs && self.RequestFixedIEs == other.RequestFixedIEs && self.RequestIELength == other.RequestIELength && self.OffsetRequestIEs == other.OffsetRequestIEs && self.AvailableResponseFixedIEs == other.AvailableResponseFixedIEs && self.ResponseFixedIEs == other.ResponseFixedIEs && self.ResponseIELength == other.ResponseIELength && self.OffsetResponseIEs == other.OffsetResponseIEs
    }
}
impl ::std::cmp::Eq for NDIS_802_11_ASSOCIATION_INFORMATION {}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_ASSOCIATION_INFORMATION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_802_11_AUTHENTICATION_ENCRYPTION {
    pub AuthModeSupported: NDIS_802_11_AUTHENTICATION_MODE,
    pub EncryptStatusSupported: NDIS_802_11_WEP_STATUS,
}
impl NDIS_802_11_AUTHENTICATION_ENCRYPTION {}
impl ::std::default::Default for NDIS_802_11_AUTHENTICATION_ENCRYPTION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_802_11_AUTHENTICATION_ENCRYPTION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_802_11_AUTHENTICATION_ENCRYPTION").field("AuthModeSupported", &self.AuthModeSupported).field("EncryptStatusSupported", &self.EncryptStatusSupported).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_802_11_AUTHENTICATION_ENCRYPTION {
    fn eq(&self, other: &Self) -> bool {
        self.AuthModeSupported == other.AuthModeSupported && self.EncryptStatusSupported == other.EncryptStatusSupported
    }
}
impl ::std::cmp::Eq for NDIS_802_11_AUTHENTICATION_ENCRYPTION {}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_AUTHENTICATION_ENCRYPTION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_802_11_AUTHENTICATION_EVENT {
    pub Status: NDIS_802_11_STATUS_INDICATION,
    pub Request: [NDIS_802_11_AUTHENTICATION_REQUEST; 1],
}
impl NDIS_802_11_AUTHENTICATION_EVENT {}
impl ::std::default::Default for NDIS_802_11_AUTHENTICATION_EVENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_802_11_AUTHENTICATION_EVENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_802_11_AUTHENTICATION_EVENT").field("Status", &self.Status).field("Request", &self.Request).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_802_11_AUTHENTICATION_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.Status == other.Status && self.Request == other.Request
    }
}
impl ::std::cmp::Eq for NDIS_802_11_AUTHENTICATION_EVENT {}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_AUTHENTICATION_EVENT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_802_11_AUTHENTICATION_MODE(pub i32);
pub const Ndis802_11AuthModeOpen: NDIS_802_11_AUTHENTICATION_MODE = NDIS_802_11_AUTHENTICATION_MODE(0i32);
pub const Ndis802_11AuthModeShared: NDIS_802_11_AUTHENTICATION_MODE = NDIS_802_11_AUTHENTICATION_MODE(1i32);
pub const Ndis802_11AuthModeAutoSwitch: NDIS_802_11_AUTHENTICATION_MODE = NDIS_802_11_AUTHENTICATION_MODE(2i32);
pub const Ndis802_11AuthModeWPA: NDIS_802_11_AUTHENTICATION_MODE = NDIS_802_11_AUTHENTICATION_MODE(3i32);
pub const Ndis802_11AuthModeWPAPSK: NDIS_802_11_AUTHENTICATION_MODE = NDIS_802_11_AUTHENTICATION_MODE(4i32);
pub const Ndis802_11AuthModeWPANone: NDIS_802_11_AUTHENTICATION_MODE = NDIS_802_11_AUTHENTICATION_MODE(5i32);
pub const Ndis802_11AuthModeWPA2: NDIS_802_11_AUTHENTICATION_MODE = NDIS_802_11_AUTHENTICATION_MODE(6i32);
pub const Ndis802_11AuthModeWPA2PSK: NDIS_802_11_AUTHENTICATION_MODE = NDIS_802_11_AUTHENTICATION_MODE(7i32);
pub const Ndis802_11AuthModeWPA3: NDIS_802_11_AUTHENTICATION_MODE = NDIS_802_11_AUTHENTICATION_MODE(8i32);
pub const Ndis802_11AuthModeWPA3Ent192: NDIS_802_11_AUTHENTICATION_MODE = NDIS_802_11_AUTHENTICATION_MODE(8i32);
pub const Ndis802_11AuthModeWPA3SAE: NDIS_802_11_AUTHENTICATION_MODE = NDIS_802_11_AUTHENTICATION_MODE(9i32);
pub const Ndis802_11AuthModeWPA3Ent: NDIS_802_11_AUTHENTICATION_MODE = NDIS_802_11_AUTHENTICATION_MODE(10i32);
pub const Ndis802_11AuthModeMax: NDIS_802_11_AUTHENTICATION_MODE = NDIS_802_11_AUTHENTICATION_MODE(11i32);
impl ::std::convert::From<i32> for NDIS_802_11_AUTHENTICATION_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_AUTHENTICATION_MODE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_802_11_AUTHENTICATION_REQUEST {
    pub Length: u32,
    pub Bssid: [u8; 6],
    pub Flags: u32,
}
impl NDIS_802_11_AUTHENTICATION_REQUEST {}
impl ::std::default::Default for NDIS_802_11_AUTHENTICATION_REQUEST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_802_11_AUTHENTICATION_REQUEST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_802_11_AUTHENTICATION_REQUEST").field("Length", &self.Length).field("Bssid", &self.Bssid).field("Flags", &self.Flags).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_802_11_AUTHENTICATION_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.Bssid == other.Bssid && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for NDIS_802_11_AUTHENTICATION_REQUEST {}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_AUTHENTICATION_REQUEST {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_802_11_AUTH_REQUEST_AUTH_FIELDS: u32 = 15u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_802_11_AUTH_REQUEST_GROUP_ERROR: u32 = 14u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_802_11_AUTH_REQUEST_KEYUPDATE: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_802_11_AUTH_REQUEST_PAIRWISE_ERROR: u32 = 6u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_802_11_AUTH_REQUEST_REAUTH: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_802_11_BSSID_LIST {
    pub NumberOfItems: u32,
    pub Bssid: [NDIS_WLAN_BSSID; 1],
}
impl NDIS_802_11_BSSID_LIST {}
impl ::std::default::Default for NDIS_802_11_BSSID_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_802_11_BSSID_LIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_802_11_BSSID_LIST").field("NumberOfItems", &self.NumberOfItems).field("Bssid", &self.Bssid).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_802_11_BSSID_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfItems == other.NumberOfItems && self.Bssid == other.Bssid
    }
}
impl ::std::cmp::Eq for NDIS_802_11_BSSID_LIST {}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_BSSID_LIST {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_802_11_BSSID_LIST_EX {
    pub NumberOfItems: u32,
    pub Bssid: [NDIS_WLAN_BSSID_EX; 1],
}
impl NDIS_802_11_BSSID_LIST_EX {}
impl ::std::default::Default for NDIS_802_11_BSSID_LIST_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_802_11_BSSID_LIST_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_802_11_BSSID_LIST_EX").field("NumberOfItems", &self.NumberOfItems).field("Bssid", &self.Bssid).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_802_11_BSSID_LIST_EX {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfItems == other.NumberOfItems && self.Bssid == other.Bssid
    }
}
impl ::std::cmp::Eq for NDIS_802_11_BSSID_LIST_EX {}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_BSSID_LIST_EX {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_802_11_CAPABILITY {
    pub Length: u32,
    pub Version: u32,
    pub NoOfPMKIDs: u32,
    pub NoOfAuthEncryptPairsSupported: u32,
    pub AuthenticationEncryptionSupported: [NDIS_802_11_AUTHENTICATION_ENCRYPTION; 1],
}
impl NDIS_802_11_CAPABILITY {}
impl ::std::default::Default for NDIS_802_11_CAPABILITY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_802_11_CAPABILITY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_802_11_CAPABILITY")
            .field("Length", &self.Length)
            .field("Version", &self.Version)
            .field("NoOfPMKIDs", &self.NoOfPMKIDs)
            .field("NoOfAuthEncryptPairsSupported", &self.NoOfAuthEncryptPairsSupported)
            .field("AuthenticationEncryptionSupported", &self.AuthenticationEncryptionSupported)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_802_11_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.Version == other.Version && self.NoOfPMKIDs == other.NoOfPMKIDs && self.NoOfAuthEncryptPairsSupported == other.NoOfAuthEncryptPairsSupported && self.AuthenticationEncryptionSupported == other.AuthenticationEncryptionSupported
    }
}
impl ::std::cmp::Eq for NDIS_802_11_CAPABILITY {}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_CAPABILITY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_802_11_CONFIGURATION {
    pub Length: u32,
    pub BeaconPeriod: u32,
    pub ATIMWindow: u32,
    pub DSConfig: u32,
    pub FHConfig: NDIS_802_11_CONFIGURATION_FH,
}
impl NDIS_802_11_CONFIGURATION {}
impl ::std::default::Default for NDIS_802_11_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_802_11_CONFIGURATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_802_11_CONFIGURATION").field("Length", &self.Length).field("BeaconPeriod", &self.BeaconPeriod).field("ATIMWindow", &self.ATIMWindow).field("DSConfig", &self.DSConfig).field("FHConfig", &self.FHConfig).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_802_11_CONFIGURATION {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.BeaconPeriod == other.BeaconPeriod && self.ATIMWindow == other.ATIMWindow && self.DSConfig == other.DSConfig && self.FHConfig == other.FHConfig
    }
}
impl ::std::cmp::Eq for NDIS_802_11_CONFIGURATION {}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_CONFIGURATION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_802_11_CONFIGURATION_FH {
    pub Length: u32,
    pub HopPattern: u32,
    pub HopSet: u32,
    pub DwellTime: u32,
}
impl NDIS_802_11_CONFIGURATION_FH {}
impl ::std::default::Default for NDIS_802_11_CONFIGURATION_FH {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_802_11_CONFIGURATION_FH {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_802_11_CONFIGURATION_FH").field("Length", &self.Length).field("HopPattern", &self.HopPattern).field("HopSet", &self.HopSet).field("DwellTime", &self.DwellTime).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_802_11_CONFIGURATION_FH {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.HopPattern == other.HopPattern && self.HopSet == other.HopSet && self.DwellTime == other.DwellTime
    }
}
impl ::std::cmp::Eq for NDIS_802_11_CONFIGURATION_FH {}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_CONFIGURATION_FH {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_802_11_FIXED_IEs {
    pub Timestamp: [u8; 8],
    pub BeaconInterval: u16,
    pub Capabilities: u16,
}
impl NDIS_802_11_FIXED_IEs {}
impl ::std::default::Default for NDIS_802_11_FIXED_IEs {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_802_11_FIXED_IEs {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_802_11_FIXED_IEs").field("Timestamp", &self.Timestamp).field("BeaconInterval", &self.BeaconInterval).field("Capabilities", &self.Capabilities).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_802_11_FIXED_IEs {
    fn eq(&self, other: &Self) -> bool {
        self.Timestamp == other.Timestamp && self.BeaconInterval == other.BeaconInterval && self.Capabilities == other.Capabilities
    }
}
impl ::std::cmp::Eq for NDIS_802_11_FIXED_IEs {}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_FIXED_IEs {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_802_11_KEY {
    pub Length: u32,
    pub KeyIndex: u32,
    pub KeyLength: u32,
    pub BSSID: [u8; 6],
    pub KeyRSC: u64,
    pub KeyMaterial: [u8; 1],
}
impl NDIS_802_11_KEY {}
impl ::std::default::Default for NDIS_802_11_KEY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_802_11_KEY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_802_11_KEY").field("Length", &self.Length).field("KeyIndex", &self.KeyIndex).field("KeyLength", &self.KeyLength).field("BSSID", &self.BSSID).field("KeyRSC", &self.KeyRSC).field("KeyMaterial", &self.KeyMaterial).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_802_11_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.KeyIndex == other.KeyIndex && self.KeyLength == other.KeyLength && self.BSSID == other.BSSID && self.KeyRSC == other.KeyRSC && self.KeyMaterial == other.KeyMaterial
    }
}
impl ::std::cmp::Eq for NDIS_802_11_KEY {}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_KEY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_802_11_LENGTH_RATES: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_802_11_LENGTH_RATES_EX: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_802_11_LENGTH_SSID: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_802_11_MEDIA_STREAM_MODE(pub i32);
pub const Ndis802_11MediaStreamOff: NDIS_802_11_MEDIA_STREAM_MODE = NDIS_802_11_MEDIA_STREAM_MODE(0i32);
pub const Ndis802_11MediaStreamOn: NDIS_802_11_MEDIA_STREAM_MODE = NDIS_802_11_MEDIA_STREAM_MODE(1i32);
impl ::std::convert::From<i32> for NDIS_802_11_MEDIA_STREAM_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_MEDIA_STREAM_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_802_11_NETWORK_INFRASTRUCTURE(pub i32);
pub const Ndis802_11IBSS: NDIS_802_11_NETWORK_INFRASTRUCTURE = NDIS_802_11_NETWORK_INFRASTRUCTURE(0i32);
pub const Ndis802_11Infrastructure: NDIS_802_11_NETWORK_INFRASTRUCTURE = NDIS_802_11_NETWORK_INFRASTRUCTURE(1i32);
pub const Ndis802_11AutoUnknown: NDIS_802_11_NETWORK_INFRASTRUCTURE = NDIS_802_11_NETWORK_INFRASTRUCTURE(2i32);
pub const Ndis802_11InfrastructureMax: NDIS_802_11_NETWORK_INFRASTRUCTURE = NDIS_802_11_NETWORK_INFRASTRUCTURE(3i32);
impl ::std::convert::From<i32> for NDIS_802_11_NETWORK_INFRASTRUCTURE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_NETWORK_INFRASTRUCTURE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_802_11_NETWORK_TYPE(pub i32);
pub const Ndis802_11FH: NDIS_802_11_NETWORK_TYPE = NDIS_802_11_NETWORK_TYPE(0i32);
pub const Ndis802_11DS: NDIS_802_11_NETWORK_TYPE = NDIS_802_11_NETWORK_TYPE(1i32);
pub const Ndis802_11OFDM5: NDIS_802_11_NETWORK_TYPE = NDIS_802_11_NETWORK_TYPE(2i32);
pub const Ndis802_11OFDM24: NDIS_802_11_NETWORK_TYPE = NDIS_802_11_NETWORK_TYPE(3i32);
pub const Ndis802_11Automode: NDIS_802_11_NETWORK_TYPE = NDIS_802_11_NETWORK_TYPE(4i32);
pub const Ndis802_11NetworkTypeMax: NDIS_802_11_NETWORK_TYPE = NDIS_802_11_NETWORK_TYPE(5i32);
impl ::std::convert::From<i32> for NDIS_802_11_NETWORK_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_NETWORK_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_802_11_NETWORK_TYPE_LIST {
    pub NumberOfItems: u32,
    pub NetworkType: [NDIS_802_11_NETWORK_TYPE; 1],
}
impl NDIS_802_11_NETWORK_TYPE_LIST {}
impl ::std::default::Default for NDIS_802_11_NETWORK_TYPE_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_802_11_NETWORK_TYPE_LIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_802_11_NETWORK_TYPE_LIST").field("NumberOfItems", &self.NumberOfItems).field("NetworkType", &self.NetworkType).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_802_11_NETWORK_TYPE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfItems == other.NumberOfItems && self.NetworkType == other.NetworkType
    }
}
impl ::std::cmp::Eq for NDIS_802_11_NETWORK_TYPE_LIST {}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_NETWORK_TYPE_LIST {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_802_11_NON_BCAST_SSID_LIST {
    pub NumberOfItems: u32,
    pub Non_Bcast_Ssid: [NDIS_802_11_SSID; 1],
}
impl NDIS_802_11_NON_BCAST_SSID_LIST {}
impl ::std::default::Default for NDIS_802_11_NON_BCAST_SSID_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_802_11_NON_BCAST_SSID_LIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_802_11_NON_BCAST_SSID_LIST").field("NumberOfItems", &self.NumberOfItems).field("Non_Bcast_Ssid", &self.Non_Bcast_Ssid).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_802_11_NON_BCAST_SSID_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfItems == other.NumberOfItems && self.Non_Bcast_Ssid == other.Non_Bcast_Ssid
    }
}
impl ::std::cmp::Eq for NDIS_802_11_NON_BCAST_SSID_LIST {}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_NON_BCAST_SSID_LIST {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_802_11_PMKID {
    pub Length: u32,
    pub BSSIDInfoCount: u32,
    pub BSSIDInfo: [BSSID_INFO; 1],
}
impl NDIS_802_11_PMKID {}
impl ::std::default::Default for NDIS_802_11_PMKID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_802_11_PMKID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_802_11_PMKID").field("Length", &self.Length).field("BSSIDInfoCount", &self.BSSIDInfoCount).field("BSSIDInfo", &self.BSSIDInfo).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_802_11_PMKID {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.BSSIDInfoCount == other.BSSIDInfoCount && self.BSSIDInfo == other.BSSIDInfo
    }
}
impl ::std::cmp::Eq for NDIS_802_11_PMKID {}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_PMKID {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_802_11_PMKID_CANDIDATE_LIST {
    pub Version: u32,
    pub NumCandidates: u32,
    pub CandidateList: [PMKID_CANDIDATE; 1],
}
impl NDIS_802_11_PMKID_CANDIDATE_LIST {}
impl ::std::default::Default for NDIS_802_11_PMKID_CANDIDATE_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_802_11_PMKID_CANDIDATE_LIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_802_11_PMKID_CANDIDATE_LIST").field("Version", &self.Version).field("NumCandidates", &self.NumCandidates).field("CandidateList", &self.CandidateList).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_802_11_PMKID_CANDIDATE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.NumCandidates == other.NumCandidates && self.CandidateList == other.CandidateList
    }
}
impl ::std::cmp::Eq for NDIS_802_11_PMKID_CANDIDATE_LIST {}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_PMKID_CANDIDATE_LIST {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_802_11_PMKID_CANDIDATE_PREAUTH_ENABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_802_11_POWER_MODE(pub i32);
pub const Ndis802_11PowerModeCAM: NDIS_802_11_POWER_MODE = NDIS_802_11_POWER_MODE(0i32);
pub const Ndis802_11PowerModeMAX_PSP: NDIS_802_11_POWER_MODE = NDIS_802_11_POWER_MODE(1i32);
pub const Ndis802_11PowerModeFast_PSP: NDIS_802_11_POWER_MODE = NDIS_802_11_POWER_MODE(2i32);
pub const Ndis802_11PowerModeMax: NDIS_802_11_POWER_MODE = NDIS_802_11_POWER_MODE(3i32);
impl ::std::convert::From<i32> for NDIS_802_11_POWER_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_POWER_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_802_11_PRIVACY_FILTER(pub i32);
pub const Ndis802_11PrivFilterAcceptAll: NDIS_802_11_PRIVACY_FILTER = NDIS_802_11_PRIVACY_FILTER(0i32);
pub const Ndis802_11PrivFilter8021xWEP: NDIS_802_11_PRIVACY_FILTER = NDIS_802_11_PRIVACY_FILTER(1i32);
impl ::std::convert::From<i32> for NDIS_802_11_PRIVACY_FILTER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_PRIVACY_FILTER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_802_11_RADIO_STATUS(pub i32);
pub const Ndis802_11RadioStatusOn: NDIS_802_11_RADIO_STATUS = NDIS_802_11_RADIO_STATUS(0i32);
pub const Ndis802_11RadioStatusHardwareOff: NDIS_802_11_RADIO_STATUS = NDIS_802_11_RADIO_STATUS(1i32);
pub const Ndis802_11RadioStatusSoftwareOff: NDIS_802_11_RADIO_STATUS = NDIS_802_11_RADIO_STATUS(2i32);
pub const Ndis802_11RadioStatusHardwareSoftwareOff: NDIS_802_11_RADIO_STATUS = NDIS_802_11_RADIO_STATUS(3i32);
pub const Ndis802_11RadioStatusMax: NDIS_802_11_RADIO_STATUS = NDIS_802_11_RADIO_STATUS(4i32);
impl ::std::convert::From<i32> for NDIS_802_11_RADIO_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_RADIO_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_802_11_RELOAD_DEFAULTS(pub i32);
pub const Ndis802_11ReloadWEPKeys: NDIS_802_11_RELOAD_DEFAULTS = NDIS_802_11_RELOAD_DEFAULTS(0i32);
impl ::std::convert::From<i32> for NDIS_802_11_RELOAD_DEFAULTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_RELOAD_DEFAULTS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_802_11_REMOVE_KEY {
    pub Length: u32,
    pub KeyIndex: u32,
    pub BSSID: [u8; 6],
}
impl NDIS_802_11_REMOVE_KEY {}
impl ::std::default::Default for NDIS_802_11_REMOVE_KEY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_802_11_REMOVE_KEY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_802_11_REMOVE_KEY").field("Length", &self.Length).field("KeyIndex", &self.KeyIndex).field("BSSID", &self.BSSID).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_802_11_REMOVE_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.KeyIndex == other.KeyIndex && self.BSSID == other.BSSID
    }
}
impl ::std::cmp::Eq for NDIS_802_11_REMOVE_KEY {}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_REMOVE_KEY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_802_11_SSID {
    pub SsidLength: u32,
    pub Ssid: [u8; 32],
}
impl NDIS_802_11_SSID {}
impl ::std::default::Default for NDIS_802_11_SSID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_802_11_SSID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_802_11_SSID").field("SsidLength", &self.SsidLength).field("Ssid", &self.Ssid).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_802_11_SSID {
    fn eq(&self, other: &Self) -> bool {
        self.SsidLength == other.SsidLength && self.Ssid == other.Ssid
    }
}
impl ::std::cmp::Eq for NDIS_802_11_SSID {}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_SSID {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_802_11_STATISTICS {
    pub Length: u32,
    pub TransmittedFragmentCount: i64,
    pub MulticastTransmittedFrameCount: i64,
    pub FailedCount: i64,
    pub RetryCount: i64,
    pub MultipleRetryCount: i64,
    pub RTSSuccessCount: i64,
    pub RTSFailureCount: i64,
    pub ACKFailureCount: i64,
    pub FrameDuplicateCount: i64,
    pub ReceivedFragmentCount: i64,
    pub MulticastReceivedFrameCount: i64,
    pub FCSErrorCount: i64,
    pub TKIPLocalMICFailures: i64,
    pub TKIPICVErrorCount: i64,
    pub TKIPCounterMeasuresInvoked: i64,
    pub TKIPReplays: i64,
    pub CCMPFormatErrors: i64,
    pub CCMPReplays: i64,
    pub CCMPDecryptErrors: i64,
    pub FourWayHandshakeFailures: i64,
    pub WEPUndecryptableCount: i64,
    pub WEPICVErrorCount: i64,
    pub DecryptSuccessCount: i64,
    pub DecryptFailureCount: i64,
}
impl NDIS_802_11_STATISTICS {}
impl ::std::default::Default for NDIS_802_11_STATISTICS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_802_11_STATISTICS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_802_11_STATISTICS")
            .field("Length", &self.Length)
            .field("TransmittedFragmentCount", &self.TransmittedFragmentCount)
            .field("MulticastTransmittedFrameCount", &self.MulticastTransmittedFrameCount)
            .field("FailedCount", &self.FailedCount)
            .field("RetryCount", &self.RetryCount)
            .field("MultipleRetryCount", &self.MultipleRetryCount)
            .field("RTSSuccessCount", &self.RTSSuccessCount)
            .field("RTSFailureCount", &self.RTSFailureCount)
            .field("ACKFailureCount", &self.ACKFailureCount)
            .field("FrameDuplicateCount", &self.FrameDuplicateCount)
            .field("ReceivedFragmentCount", &self.ReceivedFragmentCount)
            .field("MulticastReceivedFrameCount", &self.MulticastReceivedFrameCount)
            .field("FCSErrorCount", &self.FCSErrorCount)
            .field("TKIPLocalMICFailures", &self.TKIPLocalMICFailures)
            .field("TKIPICVErrorCount", &self.TKIPICVErrorCount)
            .field("TKIPCounterMeasuresInvoked", &self.TKIPCounterMeasuresInvoked)
            .field("TKIPReplays", &self.TKIPReplays)
            .field("CCMPFormatErrors", &self.CCMPFormatErrors)
            .field("CCMPReplays", &self.CCMPReplays)
            .field("CCMPDecryptErrors", &self.CCMPDecryptErrors)
            .field("FourWayHandshakeFailures", &self.FourWayHandshakeFailures)
            .field("WEPUndecryptableCount", &self.WEPUndecryptableCount)
            .field("WEPICVErrorCount", &self.WEPICVErrorCount)
            .field("DecryptSuccessCount", &self.DecryptSuccessCount)
            .field("DecryptFailureCount", &self.DecryptFailureCount)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_802_11_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.TransmittedFragmentCount == other.TransmittedFragmentCount
            && self.MulticastTransmittedFrameCount == other.MulticastTransmittedFrameCount
            && self.FailedCount == other.FailedCount
            && self.RetryCount == other.RetryCount
            && self.MultipleRetryCount == other.MultipleRetryCount
            && self.RTSSuccessCount == other.RTSSuccessCount
            && self.RTSFailureCount == other.RTSFailureCount
            && self.ACKFailureCount == other.ACKFailureCount
            && self.FrameDuplicateCount == other.FrameDuplicateCount
            && self.ReceivedFragmentCount == other.ReceivedFragmentCount
            && self.MulticastReceivedFrameCount == other.MulticastReceivedFrameCount
            && self.FCSErrorCount == other.FCSErrorCount
            && self.TKIPLocalMICFailures == other.TKIPLocalMICFailures
            && self.TKIPICVErrorCount == other.TKIPICVErrorCount
            && self.TKIPCounterMeasuresInvoked == other.TKIPCounterMeasuresInvoked
            && self.TKIPReplays == other.TKIPReplays
            && self.CCMPFormatErrors == other.CCMPFormatErrors
            && self.CCMPReplays == other.CCMPReplays
            && self.CCMPDecryptErrors == other.CCMPDecryptErrors
            && self.FourWayHandshakeFailures == other.FourWayHandshakeFailures
            && self.WEPUndecryptableCount == other.WEPUndecryptableCount
            && self.WEPICVErrorCount == other.WEPICVErrorCount
            && self.DecryptSuccessCount == other.DecryptSuccessCount
            && self.DecryptFailureCount == other.DecryptFailureCount
    }
}
impl ::std::cmp::Eq for NDIS_802_11_STATISTICS {}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_STATISTICS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_802_11_STATUS_INDICATION {
    pub StatusType: NDIS_802_11_STATUS_TYPE,
}
impl NDIS_802_11_STATUS_INDICATION {}
impl ::std::default::Default for NDIS_802_11_STATUS_INDICATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_802_11_STATUS_INDICATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_802_11_STATUS_INDICATION").field("StatusType", &self.StatusType).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_802_11_STATUS_INDICATION {
    fn eq(&self, other: &Self) -> bool {
        self.StatusType == other.StatusType
    }
}
impl ::std::cmp::Eq for NDIS_802_11_STATUS_INDICATION {}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_STATUS_INDICATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_802_11_STATUS_TYPE(pub i32);
pub const Ndis802_11StatusType_Authentication: NDIS_802_11_STATUS_TYPE = NDIS_802_11_STATUS_TYPE(0i32);
pub const Ndis802_11StatusType_MediaStreamMode: NDIS_802_11_STATUS_TYPE = NDIS_802_11_STATUS_TYPE(1i32);
pub const Ndis802_11StatusType_PMKID_CandidateList: NDIS_802_11_STATUS_TYPE = NDIS_802_11_STATUS_TYPE(2i32);
pub const Ndis802_11StatusTypeMax: NDIS_802_11_STATUS_TYPE = NDIS_802_11_STATUS_TYPE(3i32);
impl ::std::convert::From<i32> for NDIS_802_11_STATUS_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_STATUS_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_802_11_TEST {
    pub Length: u32,
    pub Type: u32,
    pub Anonymous: NDIS_802_11_TEST_0,
}
impl NDIS_802_11_TEST {}
impl ::std::default::Default for NDIS_802_11_TEST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for NDIS_802_11_TEST {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for NDIS_802_11_TEST {}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_TEST {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub union NDIS_802_11_TEST_0 {
    pub AuthenticationEvent: NDIS_802_11_AUTHENTICATION_EVENT,
    pub RssiTrigger: i32,
}
impl NDIS_802_11_TEST_0 {}
impl ::std::default::Default for NDIS_802_11_TEST_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for NDIS_802_11_TEST_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for NDIS_802_11_TEST_0 {}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_TEST_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_802_11_VARIABLE_IEs {
    pub ElementID: u8,
    pub Length: u8,
    pub data: [u8; 1],
}
impl NDIS_802_11_VARIABLE_IEs {}
impl ::std::default::Default for NDIS_802_11_VARIABLE_IEs {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_802_11_VARIABLE_IEs {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_802_11_VARIABLE_IEs").field("ElementID", &self.ElementID).field("Length", &self.Length).field("data", &self.data).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_802_11_VARIABLE_IEs {
    fn eq(&self, other: &Self) -> bool {
        self.ElementID == other.ElementID && self.Length == other.Length && self.data == other.data
    }
}
impl ::std::cmp::Eq for NDIS_802_11_VARIABLE_IEs {}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_VARIABLE_IEs {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_802_11_WEP {
    pub Length: u32,
    pub KeyIndex: u32,
    pub KeyLength: u32,
    pub KeyMaterial: [u8; 1],
}
impl NDIS_802_11_WEP {}
impl ::std::default::Default for NDIS_802_11_WEP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_802_11_WEP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_802_11_WEP").field("Length", &self.Length).field("KeyIndex", &self.KeyIndex).field("KeyLength", &self.KeyLength).field("KeyMaterial", &self.KeyMaterial).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_802_11_WEP {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.KeyIndex == other.KeyIndex && self.KeyLength == other.KeyLength && self.KeyMaterial == other.KeyMaterial
    }
}
impl ::std::cmp::Eq for NDIS_802_11_WEP {}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_WEP {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_802_11_WEP_STATUS(pub i32);
pub const Ndis802_11WEPEnabled: NDIS_802_11_WEP_STATUS = NDIS_802_11_WEP_STATUS(0i32);
pub const Ndis802_11Encryption1Enabled: NDIS_802_11_WEP_STATUS = NDIS_802_11_WEP_STATUS(0i32);
pub const Ndis802_11WEPDisabled: NDIS_802_11_WEP_STATUS = NDIS_802_11_WEP_STATUS(1i32);
pub const Ndis802_11EncryptionDisabled: NDIS_802_11_WEP_STATUS = NDIS_802_11_WEP_STATUS(1i32);
pub const Ndis802_11WEPKeyAbsent: NDIS_802_11_WEP_STATUS = NDIS_802_11_WEP_STATUS(2i32);
pub const Ndis802_11Encryption1KeyAbsent: NDIS_802_11_WEP_STATUS = NDIS_802_11_WEP_STATUS(2i32);
pub const Ndis802_11WEPNotSupported: NDIS_802_11_WEP_STATUS = NDIS_802_11_WEP_STATUS(3i32);
pub const Ndis802_11EncryptionNotSupported: NDIS_802_11_WEP_STATUS = NDIS_802_11_WEP_STATUS(3i32);
pub const Ndis802_11Encryption2Enabled: NDIS_802_11_WEP_STATUS = NDIS_802_11_WEP_STATUS(4i32);
pub const Ndis802_11Encryption2KeyAbsent: NDIS_802_11_WEP_STATUS = NDIS_802_11_WEP_STATUS(5i32);
pub const Ndis802_11Encryption3Enabled: NDIS_802_11_WEP_STATUS = NDIS_802_11_WEP_STATUS(6i32);
pub const Ndis802_11Encryption3KeyAbsent: NDIS_802_11_WEP_STATUS = NDIS_802_11_WEP_STATUS(7i32);
impl ::std::convert::From<i32> for NDIS_802_11_WEP_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_802_11_WEP_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_802_3_MAC_OPTION_PRIORITY: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_802_5_RING_STATE(pub i32);
pub const NdisRingStateOpened: NDIS_802_5_RING_STATE = NDIS_802_5_RING_STATE(1i32);
pub const NdisRingStateClosed: NDIS_802_5_RING_STATE = NDIS_802_5_RING_STATE(2i32);
pub const NdisRingStateOpening: NDIS_802_5_RING_STATE = NDIS_802_5_RING_STATE(3i32);
pub const NdisRingStateClosing: NDIS_802_5_RING_STATE = NDIS_802_5_RING_STATE(4i32);
pub const NdisRingStateOpenFailure: NDIS_802_5_RING_STATE = NDIS_802_5_RING_STATE(5i32);
pub const NdisRingStateRingFailure: NDIS_802_5_RING_STATE = NDIS_802_5_RING_STATE(6i32);
impl ::std::convert::From<i32> for NDIS_802_5_RING_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_802_5_RING_STATE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_CO_DEVICE_PROFILE {
    pub DeviceDescription: NDIS_VAR_DATA_DESC,
    pub DevSpecificInfo: NDIS_VAR_DATA_DESC,
    pub ulTAPISupplementaryPassThru: u32,
    pub ulAddressModes: u32,
    pub ulNumAddresses: u32,
    pub ulBearerModes: u32,
    pub ulMaxTxRate: u32,
    pub ulMinTxRate: u32,
    pub ulMaxRxRate: u32,
    pub ulMinRxRate: u32,
    pub ulMediaModes: u32,
    pub ulGenerateToneModes: u32,
    pub ulGenerateToneMaxNumFreq: u32,
    pub ulGenerateDigitModes: u32,
    pub ulMonitorToneMaxNumFreq: u32,
    pub ulMonitorToneMaxNumEntries: u32,
    pub ulMonitorDigitModes: u32,
    pub ulGatherDigitsMinTimeout: u32,
    pub ulGatherDigitsMaxTimeout: u32,
    pub ulDevCapFlags: u32,
    pub ulMaxNumActiveCalls: u32,
    pub ulAnswerMode: u32,
    pub ulUUIAcceptSize: u32,
    pub ulUUIAnswerSize: u32,
    pub ulUUIMakeCallSize: u32,
    pub ulUUIDropSize: u32,
    pub ulUUISendUserUserInfoSize: u32,
    pub ulUUICallInfoSize: u32,
}
impl NDIS_CO_DEVICE_PROFILE {}
impl ::std::default::Default for NDIS_CO_DEVICE_PROFILE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_CO_DEVICE_PROFILE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_CO_DEVICE_PROFILE")
            .field("DeviceDescription", &self.DeviceDescription)
            .field("DevSpecificInfo", &self.DevSpecificInfo)
            .field("ulTAPISupplementaryPassThru", &self.ulTAPISupplementaryPassThru)
            .field("ulAddressModes", &self.ulAddressModes)
            .field("ulNumAddresses", &self.ulNumAddresses)
            .field("ulBearerModes", &self.ulBearerModes)
            .field("ulMaxTxRate", &self.ulMaxTxRate)
            .field("ulMinTxRate", &self.ulMinTxRate)
            .field("ulMaxRxRate", &self.ulMaxRxRate)
            .field("ulMinRxRate", &self.ulMinRxRate)
            .field("ulMediaModes", &self.ulMediaModes)
            .field("ulGenerateToneModes", &self.ulGenerateToneModes)
            .field("ulGenerateToneMaxNumFreq", &self.ulGenerateToneMaxNumFreq)
            .field("ulGenerateDigitModes", &self.ulGenerateDigitModes)
            .field("ulMonitorToneMaxNumFreq", &self.ulMonitorToneMaxNumFreq)
            .field("ulMonitorToneMaxNumEntries", &self.ulMonitorToneMaxNumEntries)
            .field("ulMonitorDigitModes", &self.ulMonitorDigitModes)
            .field("ulGatherDigitsMinTimeout", &self.ulGatherDigitsMinTimeout)
            .field("ulGatherDigitsMaxTimeout", &self.ulGatherDigitsMaxTimeout)
            .field("ulDevCapFlags", &self.ulDevCapFlags)
            .field("ulMaxNumActiveCalls", &self.ulMaxNumActiveCalls)
            .field("ulAnswerMode", &self.ulAnswerMode)
            .field("ulUUIAcceptSize", &self.ulUUIAcceptSize)
            .field("ulUUIAnswerSize", &self.ulUUIAnswerSize)
            .field("ulUUIMakeCallSize", &self.ulUUIMakeCallSize)
            .field("ulUUIDropSize", &self.ulUUIDropSize)
            .field("ulUUISendUserUserInfoSize", &self.ulUUISendUserUserInfoSize)
            .field("ulUUICallInfoSize", &self.ulUUICallInfoSize)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_CO_DEVICE_PROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceDescription == other.DeviceDescription
            && self.DevSpecificInfo == other.DevSpecificInfo
            && self.ulTAPISupplementaryPassThru == other.ulTAPISupplementaryPassThru
            && self.ulAddressModes == other.ulAddressModes
            && self.ulNumAddresses == other.ulNumAddresses
            && self.ulBearerModes == other.ulBearerModes
            && self.ulMaxTxRate == other.ulMaxTxRate
            && self.ulMinTxRate == other.ulMinTxRate
            && self.ulMaxRxRate == other.ulMaxRxRate
            && self.ulMinRxRate == other.ulMinRxRate
            && self.ulMediaModes == other.ulMediaModes
            && self.ulGenerateToneModes == other.ulGenerateToneModes
            && self.ulGenerateToneMaxNumFreq == other.ulGenerateToneMaxNumFreq
            && self.ulGenerateDigitModes == other.ulGenerateDigitModes
            && self.ulMonitorToneMaxNumFreq == other.ulMonitorToneMaxNumFreq
            && self.ulMonitorToneMaxNumEntries == other.ulMonitorToneMaxNumEntries
            && self.ulMonitorDigitModes == other.ulMonitorDigitModes
            && self.ulGatherDigitsMinTimeout == other.ulGatherDigitsMinTimeout
            && self.ulGatherDigitsMaxTimeout == other.ulGatherDigitsMaxTimeout
            && self.ulDevCapFlags == other.ulDevCapFlags
            && self.ulMaxNumActiveCalls == other.ulMaxNumActiveCalls
            && self.ulAnswerMode == other.ulAnswerMode
            && self.ulUUIAcceptSize == other.ulUUIAcceptSize
            && self.ulUUIAnswerSize == other.ulUUIAnswerSize
            && self.ulUUIMakeCallSize == other.ulUUIMakeCallSize
            && self.ulUUIDropSize == other.ulUUIDropSize
            && self.ulUUISendUserUserInfoSize == other.ulUUISendUserUserInfoSize
            && self.ulUUICallInfoSize == other.ulUUICallInfoSize
    }
}
impl ::std::cmp::Eq for NDIS_CO_DEVICE_PROFILE {}
unsafe impl ::windows::runtime::Abi for NDIS_CO_DEVICE_PROFILE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_CO_LINK_SPEED {
    pub Outbound: u32,
    pub Inbound: u32,
}
impl NDIS_CO_LINK_SPEED {}
impl ::std::default::Default for NDIS_CO_LINK_SPEED {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_CO_LINK_SPEED {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_CO_LINK_SPEED").field("Outbound", &self.Outbound).field("Inbound", &self.Inbound).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_CO_LINK_SPEED {
    fn eq(&self, other: &Self) -> bool {
        self.Outbound == other.Outbound && self.Inbound == other.Inbound
    }
}
impl ::std::cmp::Eq for NDIS_CO_LINK_SPEED {}
unsafe impl ::windows::runtime::Abi for NDIS_CO_LINK_SPEED {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_CO_MAC_OPTION_DYNAMIC_LINK_SPEED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_DEFAULT_RECEIVE_FILTER_ID: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_DEFAULT_RECEIVE_QUEUE_GROUP_ID: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_DEFAULT_RECEIVE_QUEUE_ID: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_DEFAULT_SWITCH_ID: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_DEFAULT_VPORT_ID: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_DEVICE_POWER_STATE(pub i32);
pub const NdisDeviceStateUnspecified: NDIS_DEVICE_POWER_STATE = NDIS_DEVICE_POWER_STATE(0i32);
pub const NdisDeviceStateD0: NDIS_DEVICE_POWER_STATE = NDIS_DEVICE_POWER_STATE(1i32);
pub const NdisDeviceStateD1: NDIS_DEVICE_POWER_STATE = NDIS_DEVICE_POWER_STATE(2i32);
pub const NdisDeviceStateD2: NDIS_DEVICE_POWER_STATE = NDIS_DEVICE_POWER_STATE(3i32);
pub const NdisDeviceStateD3: NDIS_DEVICE_POWER_STATE = NDIS_DEVICE_POWER_STATE(4i32);
pub const NdisDeviceStateMaximum: NDIS_DEVICE_POWER_STATE = NDIS_DEVICE_POWER_STATE(5i32);
impl ::std::convert::From<i32> for NDIS_DEVICE_POWER_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_DEVICE_POWER_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_DEVICE_TYPE_ENDPOINT: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_DEVICE_WAKE_ON_MAGIC_PACKET_ENABLE: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_DEVICE_WAKE_ON_PATTERN_MATCH_ENABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_DEVICE_WAKE_UP_ENABLE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_ENCAPSULATED_PACKET_TASK_OFFLOAD_INNER_IPV4: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_ENCAPSULATED_PACKET_TASK_OFFLOAD_INNER_IPV6: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_ENCAPSULATED_PACKET_TASK_OFFLOAD_NOT_SUPPORTED: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_ENCAPSULATED_PACKET_TASK_OFFLOAD_OUTER_IPV4: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_ENCAPSULATED_PACKET_TASK_OFFLOAD_OUTER_IPV6: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_ENCAPSULATION_IEEE_802_3: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_ENCAPSULATION_IEEE_802_3_P_AND_Q: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_ENCAPSULATION_IEEE_802_3_P_AND_Q_IN_OOB: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_ENCAPSULATION_IEEE_LLC_SNAP_ROUTED: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_ENCAPSULATION_NOT_SUPPORTED: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_ENCAPSULATION_NULL: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_ENCAPSULATION_TYPE_GRE_MAC: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_ENCAPSULATION_TYPE_VXLAN: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_ETH_TYPE_802_1Q: u32 = 33024u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_ETH_TYPE_802_1X: u32 = 34958u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_ETH_TYPE_ARP: u32 = 2054u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_ETH_TYPE_IPV4: u32 = 2048u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_ETH_TYPE_IPV6: u32 = 34525u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_ETH_TYPE_SLOW_PROTOCOL: u32 = 34825u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_FDDI_ATTACHMENT_TYPE(pub i32);
pub const NdisFddiTypeIsolated: NDIS_FDDI_ATTACHMENT_TYPE = NDIS_FDDI_ATTACHMENT_TYPE(1i32);
pub const NdisFddiTypeLocalA: NDIS_FDDI_ATTACHMENT_TYPE = NDIS_FDDI_ATTACHMENT_TYPE(2i32);
pub const NdisFddiTypeLocalB: NDIS_FDDI_ATTACHMENT_TYPE = NDIS_FDDI_ATTACHMENT_TYPE(3i32);
pub const NdisFddiTypeLocalAB: NDIS_FDDI_ATTACHMENT_TYPE = NDIS_FDDI_ATTACHMENT_TYPE(4i32);
pub const NdisFddiTypeLocalS: NDIS_FDDI_ATTACHMENT_TYPE = NDIS_FDDI_ATTACHMENT_TYPE(5i32);
pub const NdisFddiTypeWrapA: NDIS_FDDI_ATTACHMENT_TYPE = NDIS_FDDI_ATTACHMENT_TYPE(6i32);
pub const NdisFddiTypeWrapB: NDIS_FDDI_ATTACHMENT_TYPE = NDIS_FDDI_ATTACHMENT_TYPE(7i32);
pub const NdisFddiTypeWrapAB: NDIS_FDDI_ATTACHMENT_TYPE = NDIS_FDDI_ATTACHMENT_TYPE(8i32);
pub const NdisFddiTypeWrapS: NDIS_FDDI_ATTACHMENT_TYPE = NDIS_FDDI_ATTACHMENT_TYPE(9i32);
pub const NdisFddiTypeCWrapA: NDIS_FDDI_ATTACHMENT_TYPE = NDIS_FDDI_ATTACHMENT_TYPE(10i32);
pub const NdisFddiTypeCWrapB: NDIS_FDDI_ATTACHMENT_TYPE = NDIS_FDDI_ATTACHMENT_TYPE(11i32);
pub const NdisFddiTypeCWrapS: NDIS_FDDI_ATTACHMENT_TYPE = NDIS_FDDI_ATTACHMENT_TYPE(12i32);
pub const NdisFddiTypeThrough: NDIS_FDDI_ATTACHMENT_TYPE = NDIS_FDDI_ATTACHMENT_TYPE(13i32);
impl ::std::convert::From<i32> for NDIS_FDDI_ATTACHMENT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_FDDI_ATTACHMENT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_FDDI_LCONNECTION_STATE(pub i32);
pub const NdisFddiStateOff: NDIS_FDDI_LCONNECTION_STATE = NDIS_FDDI_LCONNECTION_STATE(1i32);
pub const NdisFddiStateBreak: NDIS_FDDI_LCONNECTION_STATE = NDIS_FDDI_LCONNECTION_STATE(2i32);
pub const NdisFddiStateTrace: NDIS_FDDI_LCONNECTION_STATE = NDIS_FDDI_LCONNECTION_STATE(3i32);
pub const NdisFddiStateConnect: NDIS_FDDI_LCONNECTION_STATE = NDIS_FDDI_LCONNECTION_STATE(4i32);
pub const NdisFddiStateNext: NDIS_FDDI_LCONNECTION_STATE = NDIS_FDDI_LCONNECTION_STATE(5i32);
pub const NdisFddiStateSignal: NDIS_FDDI_LCONNECTION_STATE = NDIS_FDDI_LCONNECTION_STATE(6i32);
pub const NdisFddiStateJoin: NDIS_FDDI_LCONNECTION_STATE = NDIS_FDDI_LCONNECTION_STATE(7i32);
pub const NdisFddiStateVerify: NDIS_FDDI_LCONNECTION_STATE = NDIS_FDDI_LCONNECTION_STATE(8i32);
pub const NdisFddiStateActive: NDIS_FDDI_LCONNECTION_STATE = NDIS_FDDI_LCONNECTION_STATE(9i32);
pub const NdisFddiStateMaintenance: NDIS_FDDI_LCONNECTION_STATE = NDIS_FDDI_LCONNECTION_STATE(10i32);
impl ::std::convert::From<i32> for NDIS_FDDI_LCONNECTION_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_FDDI_LCONNECTION_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_FDDI_RING_MGT_STATE(pub i32);
pub const NdisFddiRingIsolated: NDIS_FDDI_RING_MGT_STATE = NDIS_FDDI_RING_MGT_STATE(1i32);
pub const NdisFddiRingNonOperational: NDIS_FDDI_RING_MGT_STATE = NDIS_FDDI_RING_MGT_STATE(2i32);
pub const NdisFddiRingOperational: NDIS_FDDI_RING_MGT_STATE = NDIS_FDDI_RING_MGT_STATE(3i32);
pub const NdisFddiRingDetect: NDIS_FDDI_RING_MGT_STATE = NDIS_FDDI_RING_MGT_STATE(4i32);
pub const NdisFddiRingNonOperationalDup: NDIS_FDDI_RING_MGT_STATE = NDIS_FDDI_RING_MGT_STATE(5i32);
pub const NdisFddiRingOperationalDup: NDIS_FDDI_RING_MGT_STATE = NDIS_FDDI_RING_MGT_STATE(6i32);
pub const NdisFddiRingDirected: NDIS_FDDI_RING_MGT_STATE = NDIS_FDDI_RING_MGT_STATE(7i32);
pub const NdisFddiRingTrace: NDIS_FDDI_RING_MGT_STATE = NDIS_FDDI_RING_MGT_STATE(8i32);
impl ::std::convert::From<i32> for NDIS_FDDI_RING_MGT_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_FDDI_RING_MGT_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_ENCAPSULATION_TYPE_IP_IN_GRE: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_ENCAPSULATION_TYPE_IP_IN_IP: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_ENCAPSULATION_TYPE_NOT_ENCAPSULATED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_ENCAPSULATION_TYPE_NVGRE: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_ENCAPSULATION_TYPE_VXLAN: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_EXACT_MATCH_PROFILE_RDMA_FLOW: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_EXACT_MATCH_PROFILE_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_HEADER_GROUP_EXACT_MATCH_IS_TTL_ONE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_HEADER_GROUP_EXACT_MATCH_PROFILE_IS_TTL_ONE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_HEADER_GROUP_EXACT_MATCH_PROFILE_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_HEADER_GROUP_EXACT_MATCH_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH_IS_TTL_ONE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH_PROFILE_IS_TTL_ONE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH_PROFILE_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_HEADER_PRESENT_ESP: u32 = 2048u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_HEADER_PRESENT_ETHERNET: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_HEADER_PRESENT_ICMP: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_HEADER_PRESENT_IPV4: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_HEADER_PRESENT_IPV6: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_HEADER_PRESENT_IP_IN_GRE_ENCAP: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_HEADER_PRESENT_IP_IN_IP_ENCAP: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_HEADER_PRESENT_NO_ENCAP: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_HEADER_PRESENT_NVGRE_ENCAP: u32 = 512u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_HEADER_PRESENT_TCP: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_HEADER_PRESENT_UDP: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_HEADER_PRESENT_VXLAN_ENCAP: u32 = 1024u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_UNDEFINED_PROFILE_ID: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFP_WILDCARD_MATCH_PROFILE_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_COUNTER_INFO_ARRAY_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_COUNTER_INFO_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_COUNTER_PARAMETERS_CLIENT_SPECIFIED_ADDRESS: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_COUNTER_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_COUNTER_VALUE_ARRAY_GET_VALUES: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_COUNTER_VALUE_ARRAY_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_COUNTER_VALUE_ARRAY_UPDATE_MEMORY_MAPPED_COUNTERS: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_CUSTOM_ACTION_LAST_ACTION: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_CUSTOM_ACTION_PROFILE_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_CUSTOM_ACTION_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_DELETE_PROFILE_ALL_PROFILES: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_DELETE_PROFILE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_DELETE_TABLE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_EMFE_ADD_IN_ACTIVATED_STATE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_EMFE_ALL_VPORT_FLOW_ENTRIES: u32 = 33554432u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_EMFE_COPY_AFTER_TCP_FIN_FLAG_SET: u32 = 2097152u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_EMFE_COPY_AFTER_TCP_RST_FLAG_SET: u32 = 4194304u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_EMFE_COPY_ALL_PACKETS: u32 = 65536u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_EMFE_COPY_CONDITION_CHANGED: u32 = 16777216u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_EMFE_COPY_FIRST_PACKET: u32 = 131072u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_EMFE_COPY_WHEN_TCP_FLAG_SET: u32 = 262144u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_EMFE_COUNTER_ALLOCATE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_EMFE_COUNTER_CLIENT_SPECIFIED_ADDRESS: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_EMFE_COUNTER_MEMORY_MAPPED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_EMFE_COUNTER_TRACK_TCP_FLOW: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_EMFE_CUSTOM_ACTION_PRESENT: u32 = 524288u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_EMFE_MATCH_AND_ACTION_MUST_BE_SUPPORTED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_EMFE_META_ACTION_BEFORE_HEADER_TRANSPOSITION: u32 = 1048576u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_EMFE_RDMA_FLOW: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_EMFE_REDIRECT_TO_EGRESS_QUEUE_OF_VPORT: u32 = 8192u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_EMFE_REDIRECT_TO_EGRESS_QUEUE_OF_VPORT_IF_TTL_IS_ONE: u32 = 32768u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_EMFE_REDIRECT_TO_INGRESS_QUEUE_OF_VPORT: u32 = 4096u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_EMFE_REDIRECT_TO_INGRESS_QUEUE_OF_VPORT_IF_TTL_IS_ONE: u32 = 16384u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_EXACT_MATCH_FLOW_ENTRY_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_FLOW_ENTRY_ARRAY_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_FLOW_ENTRY_ID_ALL_NIC_SWITCH_FLOW_ENTRIES: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_FLOW_ENTRY_ID_ALL_TABLE_FLOW_ENTRIES: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_FLOW_ENTRY_ID_ALL_VPORT_FLOW_ENTRIES: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_FLOW_ENTRY_ID_ARRAY_COUNTER_VALUES: u32 = 65536u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_FLOW_ENTRY_ID_ARRAY_DEFINED: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_FLOW_ENTRY_ID_ARRAY_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_FLOW_ENTRY_ID_RANGE_DEFINED: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_FLOW_ENTRY_INFO_ALL_FLOW_ENTRIES: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_FLOW_ENTRY_INFO_ARRAY_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_FREE_COUNTER_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_HEADER_GROUP_TRANSPOSITION_DECREMENT_TTL_IF_NOT_ONE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_HEADER_GROUP_TRANSPOSITION_PROFILE_DECREMENT_TTL_IF_NOT_ONE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_HEADER_GROUP_TRANSPOSITION_PROFILE_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_HEADER_GROUP_TRANSPOSITION_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_HEADER_TRANSPOSITION_PROFILE_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_HTP_COPY_ALL_PACKETS: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_HTP_COPY_FIRST_PACKET: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_HTP_COPY_WHEN_TCP_FLAG_SET: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_HTP_CUSTOM_ACTION_PRESENT: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_HTP_META_ACTION_BEFORE_HEADER_TRANSPOSITION: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_HTP_REDIRECT_TO_EGRESS_QUEUE_OF_VPORT: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_HTP_REDIRECT_TO_EGRESS_QUEUE_OF_VPORT_IF_TTL_IS_ONE: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_HTP_REDIRECT_TO_INGRESS_QUEUE_OF_VPORT: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_HTP_REDIRECT_TO_INGRESS_QUEUE_OF_VPORT_IF_TTL_IS_ONE: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_MAX_COUNTER_OBJECTS_PER_FLOW_ENTRY: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPABILITIES_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_8021P_PRIORITY_MASK: u32 = 131072u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_ADD_FLOW_ENTRY_DEACTIVATED_PREFERRED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_ALLOW: u32 = 262144u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_CLIENT_SPECIFIED_MEMORY_MAPPED_COUNTERS: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_COMBINED_COUNTER_AND_STATE: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_COPY_ALL: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_COPY_FIRST: u32 = 512u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_COPY_WHEN_TCP_FLAG_SET: u32 = 1024u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_DESIGNATED_EXCEPTION_VPORT: u32 = 32768u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_DROP: u32 = 524288u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_DSCP_MASK: u32 = 65536u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_EGRESS_AGGREGATE_COUNTERS: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_EGRESS_EXACT_MATCH: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_EGRESS_WILDCARD_MATCH: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_EXT_VPORT_EGRESS_EXACT_MATCH: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_EXT_VPORT_EGRESS_WILDCARD_MATCH: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_EXT_VPORT_INGRESS_EXACT_MATCH: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_EXT_VPORT_INGRESS_WILDCARD_MATCH: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_IGNORE_ACTION_SUPPORTED: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_INGRESS_AGGREGATE_COUNTERS: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_INGRESS_EXACT_MATCH: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_INGRESS_WILDCARD_MATCH: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_MEMORY_MAPPED_COUNTERS: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_MEMORY_MAPPED_PAKCET_AND_BYTE_COUNTERS: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_META_ACTION_AFTER_HEADER_TRANSPOSITION: u32 = 8192u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_META_ACTION_BEFORE_HEADER_TRANSPOSITION: u32 = 4096u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_MODIFY: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_PER_FLOW_ENTRY_COUNTERS: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_PER_PACKET_COUNTER_UPDATE: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_PER_VPORT_EXCEPTION_VPORT: u32 = 16384u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_POP: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_PUSH: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_RATE_LIMITING_QUEUE_SUPPORTED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_REDIRECT_TO_EGRESS_QUEUE_OF_VPORT: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_REDIRECT_TO_EGRESS_QUEUE_OF_VPORT_IF_TTL_IS_ONE: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_REDIRECT_TO_INGRESS_QUEUE_OF_VPORT: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_REDIRECT_TO_INGRESS_QUEUE_OF_VPORT_IF_TTL_IS_ONE: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_SAMPLE: u32 = 2048u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_CAPS_TRACK_TCP_FLOW_STATE: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_PARAMETERS_CUSTOM_PROVIDER_RESERVED: u32 = 4278190080u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_PARAMETERS_ENABLE_OFFLOAD: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_OFFLOAD_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_PROFILE_INFO_ARRAY_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_PROFILE_INFO_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_RESERVED_CUSTOM_ACTIONS: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_STATISTICS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_TABLE_INCLUDE_EXTERNAL_VPPORT: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_TABLE_INFO_ARRAY_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_TABLE_INFO_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_TABLE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_UNDEFINED_COUNTER_ID: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_UNDEFINED_CUSTOM_ACTION: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_UNDEFINED_FLOW_ENTRY_ID: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_UNDEFINED_TABLE_ID: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_VPORT_DSCP_FLAGS_CHANGED: u32 = 67108864u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_VPORT_DSCP_GUARD_ENABLE_RX: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_VPORT_DSCP_GUARD_ENABLE_TX: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_VPORT_DSCP_MASK_CHANGED: u32 = 8388608u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_VPORT_DSCP_MASK_ENABLE_RX: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_VPORT_DSCP_MASK_ENABLE_TX: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_VPORT_ENABLE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_VPORT_ENABLE_STATE_CHANGED: u32 = 1048576u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_VPORT_EXCEPTION_VPORT_CHANGED: u32 = 2097152u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_VPORT_MAX_DSCP_MASK_COUNTER_OBJECTS: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_VPORT_MAX_PRIORITY_MASK_COUNTER_OBJECTS: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_VPORT_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_VPORT_PARAMS_CHANGE_MASK: u32 = 4293918720u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_VPORT_PARAMS_CUSTOM_PROVIDER_RESERVED: u32 = 1044480u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_VPORT_PARSE_VXLAN: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_VPORT_PARSE_VXLAN_NOT_IN_SRC_PORT_RANGE: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_VPORT_PRIORITY_MASK_CHANGED: u32 = 16777216u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_VPORT_SAMPLING_RATE_CHANGED: u32 = 4194304u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_VPORT_VXLAN_SETTINGS_CHANGED: u32 = 33554432u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_WCFE_ADD_IN_ACTIVATED_STATE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_WCFE_COPY_ALL_PACKETS: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_WCFE_COUNTER_ALLOCATE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_WCFE_COUNTER_CLIENT_SPECIFIED_ADDRESS: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_WCFE_COUNTER_MEMORY_MAPPED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_WCFE_CUSTOM_ACTION_PRESENT: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_WCFE_REDIRECT_TO_EGRESS_QUEUE_OF_VPORT: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_WCFE_REDIRECT_TO_EGRESS_QUEUE_OF_VPORT_IF_TTL_IS_ONE: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_WCFE_REDIRECT_TO_INGRESS_QUEUE_OF_VPORT: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_WCFE_REDIRECT_TO_INGRESS_QUEUE_OF_VPORT_IF_TTL_IS_ONE: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_GFT_WILDCARD_MATCH_FLOW_ENTRY_REVISION_1: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_GUID {
    pub Guid: ::windows::runtime::GUID,
    pub Anonymous: NDIS_GUID_0,
    pub Size: u32,
    pub Flags: u32,
}
impl NDIS_GUID {}
impl ::std::default::Default for NDIS_GUID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for NDIS_GUID {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for NDIS_GUID {}
unsafe impl ::windows::runtime::Abi for NDIS_GUID {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub union NDIS_GUID_0 {
    pub Oid: u32,
    pub Status: i32,
}
impl NDIS_GUID_0 {}
impl ::std::default::Default for NDIS_GUID_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for NDIS_GUID_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for NDIS_GUID_0 {}
unsafe impl ::windows::runtime::Abi for NDIS_GUID_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_HARDWARE_CROSSTIMESTAMP {
    pub Header: NDIS_OBJECT_HEADER,
    pub Flags: u32,
    pub SystemTimestamp1: u64,
    pub HardwareClockTimestamp: u64,
    pub SystemTimestamp2: u64,
}
impl NDIS_HARDWARE_CROSSTIMESTAMP {}
impl ::std::default::Default for NDIS_HARDWARE_CROSSTIMESTAMP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_HARDWARE_CROSSTIMESTAMP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_HARDWARE_CROSSTIMESTAMP").field("Header", &self.Header).field("Flags", &self.Flags).field("SystemTimestamp1", &self.SystemTimestamp1).field("HardwareClockTimestamp", &self.HardwareClockTimestamp).field("SystemTimestamp2", &self.SystemTimestamp2).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_HARDWARE_CROSSTIMESTAMP {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Flags == other.Flags && self.SystemTimestamp1 == other.SystemTimestamp1 && self.HardwareClockTimestamp == other.HardwareClockTimestamp && self.SystemTimestamp2 == other.SystemTimestamp2
    }
}
impl ::std::cmp::Eq for NDIS_HARDWARE_CROSSTIMESTAMP {}
unsafe impl ::windows::runtime::Abi for NDIS_HARDWARE_CROSSTIMESTAMP {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_HARDWARE_CROSSTIMESTAMP_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_HARDWARE_STATUS(pub i32);
pub const NdisHardwareStatusReady: NDIS_HARDWARE_STATUS = NDIS_HARDWARE_STATUS(0i32);
pub const NdisHardwareStatusInitializing: NDIS_HARDWARE_STATUS = NDIS_HARDWARE_STATUS(1i32);
pub const NdisHardwareStatusReset: NDIS_HARDWARE_STATUS = NDIS_HARDWARE_STATUS(2i32);
pub const NdisHardwareStatusClosing: NDIS_HARDWARE_STATUS = NDIS_HARDWARE_STATUS(3i32);
pub const NdisHardwareStatusNotReady: NDIS_HARDWARE_STATUS = NDIS_HARDWARE_STATUS(4i32);
impl ::std::convert::From<i32> for NDIS_HARDWARE_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_HARDWARE_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_HASH_FUNCTION_MASK: u32 = 255u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_HASH_IPV4: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_HASH_IPV6: u32 = 1024u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_HASH_IPV6_EX: u32 = 2048u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_HASH_TCP_IPV4: u32 = 512u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_HASH_TCP_IPV6: u32 = 4096u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_HASH_TCP_IPV6_EX: u32 = 8192u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_HASH_TYPE_MASK: u32 = 16776960u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_HASH_UDP_IPV4: u32 = 16384u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_HASH_UDP_IPV6: u32 = 32768u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_HASH_UDP_IPV6_EX: u32 = 65536u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_HD_SPLIT_CAPS_SUPPORTS_HEADER_DATA_SPLIT: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_HD_SPLIT_CAPS_SUPPORTS_IPV4_OPTIONS: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_HD_SPLIT_CAPS_SUPPORTS_IPV6_EXTENSION_HEADERS: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_HD_SPLIT_CAPS_SUPPORTS_TCP_OPTIONS: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_HD_SPLIT_COMBINE_ALL_HEADERS: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_HD_SPLIT_CURRENT_CONFIG_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_HD_SPLIT_ENABLE_HEADER_DATA_SPLIT: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_HD_SPLIT_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_HYPERVISOR_INFO_FLAG_HYPERVISOR_PRESENT: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_HYPERVISOR_INFO_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_IF_MAX_STRING_SIZE: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_INTERRUPT_MODERATION(pub i32);
pub const NdisInterruptModerationUnknown: NDIS_INTERRUPT_MODERATION = NDIS_INTERRUPT_MODERATION(0i32);
pub const NdisInterruptModerationNotSupported: NDIS_INTERRUPT_MODERATION = NDIS_INTERRUPT_MODERATION(1i32);
pub const NdisInterruptModerationEnabled: NDIS_INTERRUPT_MODERATION = NDIS_INTERRUPT_MODERATION(2i32);
pub const NdisInterruptModerationDisabled: NDIS_INTERRUPT_MODERATION = NDIS_INTERRUPT_MODERATION(3i32);
impl ::std::convert::From<i32> for NDIS_INTERRUPT_MODERATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_INTERRUPT_MODERATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_INTERRUPT_MODERATION_CHANGE_NEEDS_REINITIALIZE: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_INTERRUPT_MODERATION_CHANGE_NEEDS_RESET: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_INTERRUPT_MODERATION_PARAMETERS {
    pub Header: NDIS_OBJECT_HEADER,
    pub Flags: u32,
    pub InterruptModeration: NDIS_INTERRUPT_MODERATION,
}
impl NDIS_INTERRUPT_MODERATION_PARAMETERS {}
impl ::std::default::Default for NDIS_INTERRUPT_MODERATION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_INTERRUPT_MODERATION_PARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_INTERRUPT_MODERATION_PARAMETERS").field("Header", &self.Header).field("Flags", &self.Flags).field("InterruptModeration", &self.InterruptModeration).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_INTERRUPT_MODERATION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Flags == other.Flags && self.InterruptModeration == other.InterruptModeration
    }
}
impl ::std::cmp::Eq for NDIS_INTERRUPT_MODERATION_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for NDIS_INTERRUPT_MODERATION_PARAMETERS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_INTERRUPT_MODERATION_PARAMETERS_REVISION_1: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_IPSEC_OFFLOAD_V1 {
    pub Supported: NDIS_IPSEC_OFFLOAD_V1_2,
    pub IPv4AH: NDIS_IPSEC_OFFLOAD_V1_0,
    pub IPv4ESP: NDIS_IPSEC_OFFLOAD_V1_1,
}
impl NDIS_IPSEC_OFFLOAD_V1 {}
impl ::std::default::Default for NDIS_IPSEC_OFFLOAD_V1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_IPSEC_OFFLOAD_V1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_IPSEC_OFFLOAD_V1").field("Supported", &self.Supported).field("IPv4AH", &self.IPv4AH).field("IPv4ESP", &self.IPv4ESP).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_IPSEC_OFFLOAD_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Supported == other.Supported && self.IPv4AH == other.IPv4AH && self.IPv4ESP == other.IPv4ESP
    }
}
impl ::std::cmp::Eq for NDIS_IPSEC_OFFLOAD_V1 {}
unsafe impl ::windows::runtime::Abi for NDIS_IPSEC_OFFLOAD_V1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_IPSEC_OFFLOAD_V1_0 {
    pub _bitfield: u32,
}
impl NDIS_IPSEC_OFFLOAD_V1_0 {}
impl ::std::default::Default for NDIS_IPSEC_OFFLOAD_V1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_IPSEC_OFFLOAD_V1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_IPv4AH_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_IPSEC_OFFLOAD_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for NDIS_IPSEC_OFFLOAD_V1_0 {}
unsafe impl ::windows::runtime::Abi for NDIS_IPSEC_OFFLOAD_V1_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_IPSEC_OFFLOAD_V1_1 {
    pub _bitfield: u32,
}
impl NDIS_IPSEC_OFFLOAD_V1_1 {}
impl ::std::default::Default for NDIS_IPSEC_OFFLOAD_V1_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_IPSEC_OFFLOAD_V1_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_IPv4ESP_e__Struct").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_IPSEC_OFFLOAD_V1_1 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for NDIS_IPSEC_OFFLOAD_V1_1 {}
unsafe impl ::windows::runtime::Abi for NDIS_IPSEC_OFFLOAD_V1_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_IPSEC_OFFLOAD_V1_2 {
    pub Encapsulation: u32,
    pub AhEspCombined: u32,
    pub TransportTunnelCombined: u32,
    pub IPv4Options: u32,
    pub Flags: u32,
}
impl NDIS_IPSEC_OFFLOAD_V1_2 {}
impl ::std::default::Default for NDIS_IPSEC_OFFLOAD_V1_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_IPSEC_OFFLOAD_V1_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Supported_e__Struct").field("Encapsulation", &self.Encapsulation).field("AhEspCombined", &self.AhEspCombined).field("TransportTunnelCombined", &self.TransportTunnelCombined).field("IPv4Options", &self.IPv4Options).field("Flags", &self.Flags).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_IPSEC_OFFLOAD_V1_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.AhEspCombined == other.AhEspCombined && self.TransportTunnelCombined == other.TransportTunnelCombined && self.IPv4Options == other.IPv4Options && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for NDIS_IPSEC_OFFLOAD_V1_2 {}
unsafe impl ::windows::runtime::Abi for NDIS_IPSEC_OFFLOAD_V1_2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_NetworkManagement_IpHelper`*"]
pub struct NDIS_IP_OPER_STATE {
    pub Header: NDIS_OBJECT_HEADER,
    pub Flags: u32,
    pub IpOperationalStatus: NDIS_IP_OPER_STATUS,
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl NDIS_IP_OPER_STATE {}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::default::Default for NDIS_IP_OPER_STATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::fmt::Debug for NDIS_IP_OPER_STATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_IP_OPER_STATE").field("Header", &self.Header).field("Flags", &self.Flags).field("IpOperationalStatus", &self.IpOperationalStatus).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::cmp::PartialEq for NDIS_IP_OPER_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Flags == other.Flags && self.IpOperationalStatus == other.IpOperationalStatus
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::cmp::Eq for NDIS_IP_OPER_STATE {}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
unsafe impl ::windows::runtime::Abi for NDIS_IP_OPER_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_IP_OPER_STATE_REVISION_1: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_NetworkManagement_IpHelper`*"]
pub struct NDIS_IP_OPER_STATUS {
    pub AddressFamily: u32,
    pub OperationalStatus: super::IpHelper::NET_IF_OPER_STATUS,
    pub OperationalStatusFlags: u32,
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl NDIS_IP_OPER_STATUS {}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::default::Default for NDIS_IP_OPER_STATUS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::fmt::Debug for NDIS_IP_OPER_STATUS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_IP_OPER_STATUS").field("AddressFamily", &self.AddressFamily).field("OperationalStatus", &self.OperationalStatus).field("OperationalStatusFlags", &self.OperationalStatusFlags).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::cmp::PartialEq for NDIS_IP_OPER_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.AddressFamily == other.AddressFamily && self.OperationalStatus == other.OperationalStatus && self.OperationalStatusFlags == other.OperationalStatusFlags
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::cmp::Eq for NDIS_IP_OPER_STATUS {}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
unsafe impl ::windows::runtime::Abi for NDIS_IP_OPER_STATUS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_NetworkManagement_IpHelper`*"]
pub struct NDIS_IP_OPER_STATUS_INFO {
    pub Header: NDIS_OBJECT_HEADER,
    pub Flags: u32,
    pub NumberofAddressFamiliesReturned: u32,
    pub IpOperationalStatus: [NDIS_IP_OPER_STATUS; 32],
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl NDIS_IP_OPER_STATUS_INFO {}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::default::Default for NDIS_IP_OPER_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::fmt::Debug for NDIS_IP_OPER_STATUS_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_IP_OPER_STATUS_INFO").field("Header", &self.Header).field("Flags", &self.Flags).field("NumberofAddressFamiliesReturned", &self.NumberofAddressFamiliesReturned).field("IpOperationalStatus", &self.IpOperationalStatus).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::cmp::PartialEq for NDIS_IP_OPER_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Flags == other.Flags && self.NumberofAddressFamiliesReturned == other.NumberofAddressFamiliesReturned && self.IpOperationalStatus == other.IpOperationalStatus
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::cmp::Eq for NDIS_IP_OPER_STATUS_INFO {}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
unsafe impl ::windows::runtime::Abi for NDIS_IP_OPER_STATUS_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_IP_OPER_STATUS_INFO_REVISION_1: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_IRDA_PACKET_INFO {
    pub ExtraBOFs: u32,
    pub MinTurnAroundTime: u32,
}
impl NDIS_IRDA_PACKET_INFO {}
impl ::std::default::Default for NDIS_IRDA_PACKET_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_IRDA_PACKET_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_IRDA_PACKET_INFO").field("ExtraBOFs", &self.ExtraBOFs).field("MinTurnAroundTime", &self.MinTurnAroundTime).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_IRDA_PACKET_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ExtraBOFs == other.ExtraBOFs && self.MinTurnAroundTime == other.MinTurnAroundTime
    }
}
impl ::std::cmp::Eq for NDIS_IRDA_PACKET_INFO {}
unsafe impl ::windows::runtime::Abi for NDIS_IRDA_PACKET_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_ISOLATION_NAME_MAX_STRING_SIZE: u32 = 127u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_ISOLATION_PARAMETERS_REVISION_1: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_NetworkManagement_IpHelper`*"]
pub struct NDIS_LINK_PARAMETERS {
    pub Header: NDIS_OBJECT_HEADER,
    pub MediaDuplexState: super::IpHelper::NET_IF_MEDIA_DUPLEX_STATE,
    pub XmitLinkSpeed: u64,
    pub RcvLinkSpeed: u64,
    pub PauseFunctions: NDIS_SUPPORTED_PAUSE_FUNCTIONS,
    pub AutoNegotiationFlags: u32,
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl NDIS_LINK_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::default::Default for NDIS_LINK_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::fmt::Debug for NDIS_LINK_PARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_LINK_PARAMETERS")
            .field("Header", &self.Header)
            .field("MediaDuplexState", &self.MediaDuplexState)
            .field("XmitLinkSpeed", &self.XmitLinkSpeed)
            .field("RcvLinkSpeed", &self.RcvLinkSpeed)
            .field("PauseFunctions", &self.PauseFunctions)
            .field("AutoNegotiationFlags", &self.AutoNegotiationFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::cmp::PartialEq for NDIS_LINK_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.MediaDuplexState == other.MediaDuplexState && self.XmitLinkSpeed == other.XmitLinkSpeed && self.RcvLinkSpeed == other.RcvLinkSpeed && self.PauseFunctions == other.PauseFunctions && self.AutoNegotiationFlags == other.AutoNegotiationFlags
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::cmp::Eq for NDIS_LINK_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
unsafe impl ::windows::runtime::Abi for NDIS_LINK_PARAMETERS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_LINK_PARAMETERS_REVISION_1: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_LINK_SPEED {
    pub XmitLinkSpeed: u64,
    pub RcvLinkSpeed: u64,
}
impl NDIS_LINK_SPEED {}
impl ::std::default::Default for NDIS_LINK_SPEED {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_LINK_SPEED {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_LINK_SPEED").field("XmitLinkSpeed", &self.XmitLinkSpeed).field("RcvLinkSpeed", &self.RcvLinkSpeed).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_LINK_SPEED {
    fn eq(&self, other: &Self) -> bool {
        self.XmitLinkSpeed == other.XmitLinkSpeed && self.RcvLinkSpeed == other.RcvLinkSpeed
    }
}
impl ::std::cmp::Eq for NDIS_LINK_SPEED {}
unsafe impl ::windows::runtime::Abi for NDIS_LINK_SPEED {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_NetworkManagement_IpHelper`*"]
pub struct NDIS_LINK_STATE {
    pub Header: NDIS_OBJECT_HEADER,
    pub MediaConnectState: super::IpHelper::NET_IF_MEDIA_CONNECT_STATE,
    pub MediaDuplexState: super::IpHelper::NET_IF_MEDIA_DUPLEX_STATE,
    pub XmitLinkSpeed: u64,
    pub RcvLinkSpeed: u64,
    pub PauseFunctions: NDIS_SUPPORTED_PAUSE_FUNCTIONS,
    pub AutoNegotiationFlags: u32,
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl NDIS_LINK_STATE {}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::default::Default for NDIS_LINK_STATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::fmt::Debug for NDIS_LINK_STATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_LINK_STATE")
            .field("Header", &self.Header)
            .field("MediaConnectState", &self.MediaConnectState)
            .field("MediaDuplexState", &self.MediaDuplexState)
            .field("XmitLinkSpeed", &self.XmitLinkSpeed)
            .field("RcvLinkSpeed", &self.RcvLinkSpeed)
            .field("PauseFunctions", &self.PauseFunctions)
            .field("AutoNegotiationFlags", &self.AutoNegotiationFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::cmp::PartialEq for NDIS_LINK_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.MediaConnectState == other.MediaConnectState && self.MediaDuplexState == other.MediaDuplexState && self.XmitLinkSpeed == other.XmitLinkSpeed && self.RcvLinkSpeed == other.RcvLinkSpeed && self.PauseFunctions == other.PauseFunctions && self.AutoNegotiationFlags == other.AutoNegotiationFlags
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::cmp::Eq for NDIS_LINK_STATE {}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
unsafe impl ::windows::runtime::Abi for NDIS_LINK_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_LINK_STATE_DUPLEX_AUTO_NEGOTIATED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_LINK_STATE_PAUSE_FUNCTIONS_AUTO_NEGOTIATED: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_LINK_STATE_RCV_LINK_SPEED_AUTO_NEGOTIATED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_LINK_STATE_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_LINK_STATE_XMIT_LINK_SPEED_AUTO_NEGOTIATED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_MAC_OPTION_8021P_PRIORITY: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_MAC_OPTION_8021Q_VLAN: u32 = 512u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_MAC_OPTION_COPY_LOOKAHEAD_DATA: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_MAC_OPTION_EOTX_INDICATION: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_MAC_OPTION_FULL_DUPLEX: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_MAC_OPTION_NO_LOOPBACK: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_MAC_OPTION_RECEIVE_AT_DPC: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_MAC_OPTION_RECEIVE_SERIALIZED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_MAC_OPTION_RESERVED: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_MAC_OPTION_SUPPORTS_MAC_ADDRESS_OVERWRITE: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_MAC_OPTION_TRANSFERS_NOT_PEND: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_MAXIMUM_PORTS: u32 = 16777216u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_MAX_PHYS_ADDRESS_LENGTH: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_MEDIA_CAP_RECEIVE: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_MEDIA_CAP_TRANSMIT: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_MEDIA_STATE(pub i32);
pub const NdisMediaStateConnected: NDIS_MEDIA_STATE = NDIS_MEDIA_STATE(0i32);
pub const NdisMediaStateDisconnected: NDIS_MEDIA_STATE = NDIS_MEDIA_STATE(1i32);
impl ::std::convert::From<i32> for NDIS_MEDIA_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_MEDIA_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_MEDIUM(pub i32);
pub const NdisMedium802_3: NDIS_MEDIUM = NDIS_MEDIUM(0i32);
pub const NdisMedium802_5: NDIS_MEDIUM = NDIS_MEDIUM(1i32);
pub const NdisMediumFddi: NDIS_MEDIUM = NDIS_MEDIUM(2i32);
pub const NdisMediumWan: NDIS_MEDIUM = NDIS_MEDIUM(3i32);
pub const NdisMediumLocalTalk: NDIS_MEDIUM = NDIS_MEDIUM(4i32);
pub const NdisMediumDix: NDIS_MEDIUM = NDIS_MEDIUM(5i32);
pub const NdisMediumArcnetRaw: NDIS_MEDIUM = NDIS_MEDIUM(6i32);
pub const NdisMediumArcnet878_2: NDIS_MEDIUM = NDIS_MEDIUM(7i32);
pub const NdisMediumAtm: NDIS_MEDIUM = NDIS_MEDIUM(8i32);
pub const NdisMediumWirelessWan: NDIS_MEDIUM = NDIS_MEDIUM(9i32);
pub const NdisMediumIrda: NDIS_MEDIUM = NDIS_MEDIUM(10i32);
pub const NdisMediumBpc: NDIS_MEDIUM = NDIS_MEDIUM(11i32);
pub const NdisMediumCoWan: NDIS_MEDIUM = NDIS_MEDIUM(12i32);
pub const NdisMedium1394: NDIS_MEDIUM = NDIS_MEDIUM(13i32);
pub const NdisMediumInfiniBand: NDIS_MEDIUM = NDIS_MEDIUM(14i32);
pub const NdisMediumTunnel: NDIS_MEDIUM = NDIS_MEDIUM(15i32);
pub const NdisMediumNative802_11: NDIS_MEDIUM = NDIS_MEDIUM(16i32);
pub const NdisMediumLoopback: NDIS_MEDIUM = NDIS_MEDIUM(17i32);
pub const NdisMediumWiMAX: NDIS_MEDIUM = NDIS_MEDIUM(18i32);
pub const NdisMediumIP: NDIS_MEDIUM = NDIS_MEDIUM(19i32);
pub const NdisMediumMax: NDIS_MEDIUM = NDIS_MEDIUM(20i32);
impl ::std::convert::From<i32> for NDIS_MEDIUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_MEDIUM {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NDK_CAPABILITIES_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NDK_CONNECTIONS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NDK_LOCAL_ENDPOINTS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NDK_STATISTICS_INFO_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_NETWORK_CHANGE_TYPE(pub i32);
pub const NdisPossibleNetworkChange: NDIS_NETWORK_CHANGE_TYPE = NDIS_NETWORK_CHANGE_TYPE(1i32);
pub const NdisDefinitelyNetworkChange: NDIS_NETWORK_CHANGE_TYPE = NDIS_NETWORK_CHANGE_TYPE(2i32);
pub const NdisNetworkChangeFromMediaConnect: NDIS_NETWORK_CHANGE_TYPE = NDIS_NETWORK_CHANGE_TYPE(3i32);
pub const NdisNetworkChangeMax: NDIS_NETWORK_CHANGE_TYPE = NDIS_NETWORK_CHANGE_TYPE(4i32);
impl ::std::convert::From<i32> for NDIS_NETWORK_CHANGE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_NETWORK_CHANGE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_CAPABILITIES_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_CAPABILITIES_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_CAPABILITIES_REVISION_3: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_CAPS_ASYMMETRIC_QUEUE_PAIRS_FOR_NONDEFAULT_VPORT_SUPPORTED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_CAPS_NIC_SWITCH_WITHOUT_IOV_SUPPORTED: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_CAPS_PER_VPORT_INTERRUPT_MODERATION_SUPPORTED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_CAPS_RSS_ON_PF_VPORTS_SUPPORTED: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_CAPS_RSS_PARAMETERS_PER_PF_VPORT_SUPPORTED: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_CAPS_RSS_PER_PF_VPORT_HASH_FUNCTION_SUPPORTED: u32 = 512u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_CAPS_RSS_PER_PF_VPORT_HASH_KEY_SUPPORTED: u32 = 2048u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_CAPS_RSS_PER_PF_VPORT_HASH_TYPE_SUPPORTED: u32 = 1024u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_CAPS_RSS_PER_PF_VPORT_INDIRECTION_TABLE_SIZE_RESTRICTED: u32 = 4096u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_CAPS_RSS_PER_PF_VPORT_INDIRECTION_TABLE_SUPPORTED: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_CAPS_SINGLE_VPORT_POOL: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_CAPS_VF_RSS_SUPPORTED: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_CAPS_VLAN_SUPPORTED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_DELETE_SWITCH_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_DELETE_VPORT_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_FREE_VF_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_INFO_ARRAY_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_INFO_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_PARAMETERS_CHANGE_MASK: u32 = 4294901760u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_PARAMETERS_DEFAULT_NUMBER_OF_QUEUE_PAIRS_FOR_DEFAULT_VPORT: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_PARAMETERS_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_PARAMETERS_SWITCH_NAME_CHANGED: u32 = 65536u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_VF_INFO_ARRAY_ENUM_ON_SPECIFIC_SWITCH: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_VF_INFO_ARRAY_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_VF_INFO_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_VF_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_VPORT_INFO_ARRAY_ENUM_ON_SPECIFIC_FUNCTION: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_VPORT_INFO_ARRAY_ENUM_ON_SPECIFIC_SWITCH: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_VPORT_INFO_ARRAY_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_VPORT_INFO_GFT_ENABLED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_VPORT_INFO_LOOKAHEAD_SPLIT_ENABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_VPORT_INFO_PACKET_DIRECT_RX_ONLY: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_VPORT_INFO_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_VPORT_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_VPORT_PARAMETERS_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_VPORT_PARAMS_CHANGE_MASK: u32 = 4294901760u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_VPORT_PARAMS_ENFORCE_MAX_SG_LIST: u32 = 32768u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_VPORT_PARAMS_FLAGS_CHANGED: u32 = 65536u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_VPORT_PARAMS_INT_MOD_CHANGED: u32 = 262144u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_VPORT_PARAMS_LOOKAHEAD_SPLIT_ENABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_VPORT_PARAMS_NAME_CHANGED: u32 = 131072u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_VPORT_PARAMS_NDK_PARAMS_CHANGED: u32 = 2097152u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_VPORT_PARAMS_NUM_QUEUE_PAIRS_CHANGED: u32 = 8388608u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_VPORT_PARAMS_PACKET_DIRECT_RX_ONLY: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_VPORT_PARAMS_PROCESSOR_AFFINITY_CHANGED: u32 = 1048576u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_VPORT_PARAMS_QOS_SQ_ID_CHANGED: u32 = 4194304u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_NIC_SWITCH_VPORT_PARAMS_STATE_CHANGED: u32 = 524288u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_OBJECT_HEADER {
    pub Type: u8,
    pub Revision: u8,
    pub Size: u16,
}
impl NDIS_OBJECT_HEADER {}
impl ::std::default::Default for NDIS_OBJECT_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_OBJECT_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_OBJECT_HEADER").field("Type", &self.Type).field("Revision", &self.Revision).field("Size", &self.Size).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_OBJECT_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Revision == other.Revision && self.Size == other.Size
    }
}
impl ::std::cmp::Eq for NDIS_OBJECT_HEADER {}
unsafe impl ::windows::runtime::Abi for NDIS_OBJECT_HEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_BIND_PARAMETERS: u32 = 134u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_CLIENT_CHIMNEY_OFFLOAD_CHARACTERISTICS: u32 = 147u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_CLIENT_CHIMNEY_OFFLOAD_GENERIC_CHARACTERISTICS: u32 = 142u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_CONFIGURATION_OBJECT: u32 = 169u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_CO_CALL_MANAGER_OPTIONAL_HANDLERS: u32 = 165u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_CO_CLIENT_OPTIONAL_HANDLERS: u32 = 166u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_CO_MINIPORT_CHARACTERISTICS: u32 = 145u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_CO_PROTOCOL_CHARACTERISTICS: u32 = 144u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_DEFAULT: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_DEVICE_OBJECT_ATTRIBUTES: u32 = 133u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_DRIVER_WRAPPER_OBJECT: u32 = 170u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_FILTER_ATTACH_PARAMETERS: u32 = 153u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_FILTER_ATTRIBUTES: u32 = 141u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_FILTER_DRIVER_CHARACTERISTICS: u32 = 139u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_FILTER_PARTIAL_CHARACTERISTICS: u32 = 140u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_FILTER_PAUSE_PARAMETERS: u32 = 154u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_FILTER_RESTART_PARAMETERS: u32 = 155u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_HD_SPLIT_ATTRIBUTES: u32 = 171u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_MINIPORT_ADAPTER_GENERAL_ATTRIBUTES: u32 = 159u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_MINIPORT_ADAPTER_HARDWARE_ASSIST_ATTRIBUTES: u32 = 175u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_MINIPORT_ADAPTER_NATIVE_802_11_ATTRIBUTES: u32 = 161u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_MINIPORT_ADAPTER_NDK_ATTRIBUTES: u32 = 179u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_MINIPORT_ADAPTER_OFFLOAD_ATTRIBUTES: u32 = 160u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_MINIPORT_ADAPTER_PACKET_DIRECT_ATTRIBUTES: u32 = 197u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_MINIPORT_ADAPTER_REGISTRATION_ATTRIBUTES: u32 = 158u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_MINIPORT_ADD_DEVICE_REGISTRATION_ATTRIBUTES: u32 = 164u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_MINIPORT_DEVICE_POWER_NOTIFICATION: u32 = 198u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_MINIPORT_DRIVER_CHARACTERISTICS: u32 = 138u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_MINIPORT_INIT_PARAMETERS: u32 = 129u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_MINIPORT_INTERRUPT: u32 = 132u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_MINIPORT_PNP_CHARACTERISTICS: u32 = 146u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_MINIPORT_SS_CHARACTERISTICS: u32 = 180u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_NDK_PROVIDER_CHARACTERISTICS: u32 = 178u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_NSI_COMPARTMENT_RW_STRUCT: u32 = 173u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_NSI_INTERFACE_PERSIST_RW_STRUCT: u32 = 174u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_NSI_NETWORK_RW_STRUCT: u32 = 172u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_OFFLOAD: u32 = 167u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_OFFLOAD_ENCAPSULATION: u32 = 168u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_OID_REQUEST: u32 = 150u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_OPEN_PARAMETERS: u32 = 135u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_PCI_DEVICE_CUSTOM_PROPERTIES_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_PCI_DEVICE_CUSTOM_PROPERTIES_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_PD_RECEIVE_QUEUE: u32 = 191u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_PD_TRANSMIT_QUEUE: u32 = 190u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_PORT_CHARACTERISTICS: u32 = 156u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_PORT_STATE: u32 = 157u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_PROTOCOL_DRIVER_CHARACTERISTICS: u32 = 149u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_PROTOCOL_RESTART_PARAMETERS: u32 = 163u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_PROVIDER_CHIMNEY_OFFLOAD_CHARACTERISTICS: u32 = 148u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_PROVIDER_CHIMNEY_OFFLOAD_GENERIC_CHARACTERISTICS: u32 = 143u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_QOS_CAPABILITIES: u32 = 181u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_QOS_CLASSIFICATION_ELEMENT: u32 = 183u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_QOS_PARAMETERS: u32 = 182u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_REQUEST_EX: u32 = 150u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_RESTART_GENERAL_ATTRIBUTES: u32 = 162u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_RSS_CAPABILITIES: u32 = 136u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_RSS_PARAMETERS: u32 = 137u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_RSS_PARAMETERS_V2: u32 = 200u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_RSS_PROCESSOR_INFO: u32 = 177u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_RSS_SET_INDIRECTION_ENTRIES: u32 = 201u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_SG_DMA_DESCRIPTION: u32 = 131u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_SHARED_MEMORY_PROVIDER_CHARACTERISTICS: u32 = 176u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_STATUS_INDICATION: u32 = 152u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_SWITCH_OPTIONAL_HANDLERS: u32 = 184u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OBJECT_TYPE_TIMER_CHARACTERISTICS: u32 = 151u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_OFFLOAD {
    pub Header: NDIS_OBJECT_HEADER,
    pub Checksum: NDIS_TCP_IP_CHECKSUM_OFFLOAD,
    pub LsoV1: NDIS_TCP_LARGE_SEND_OFFLOAD_V1,
    pub IPsecV1: NDIS_IPSEC_OFFLOAD_V1,
    pub LsoV2: NDIS_TCP_LARGE_SEND_OFFLOAD_V2,
    pub Flags: u32,
}
impl NDIS_OFFLOAD {}
impl ::std::default::Default for NDIS_OFFLOAD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_OFFLOAD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_OFFLOAD").field("Header", &self.Header).field("Checksum", &self.Checksum).field("LsoV1", &self.LsoV1).field("IPsecV1", &self.IPsecV1).field("LsoV2", &self.LsoV2).field("Flags", &self.Flags).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_OFFLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Checksum == other.Checksum && self.LsoV1 == other.LsoV1 && self.IPsecV1 == other.IPsecV1 && self.LsoV2 == other.LsoV2 && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for NDIS_OFFLOAD {}
unsafe impl ::windows::runtime::Abi for NDIS_OFFLOAD {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_FLAGS_GROUP_CHECKSUM_CAPABILITIES: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_NOT_SUPPORTED: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_OFFLOAD_PARAMETERS {
    pub Header: NDIS_OBJECT_HEADER,
    pub IPv4Checksum: u8,
    pub TCPIPv4Checksum: u8,
    pub UDPIPv4Checksum: u8,
    pub TCPIPv6Checksum: u8,
    pub UDPIPv6Checksum: u8,
    pub LsoV1: u8,
    pub IPsecV1: u8,
    pub LsoV2IPv4: u8,
    pub LsoV2IPv6: u8,
    pub TcpConnectionIPv4: u8,
    pub TcpConnectionIPv6: u8,
    pub Flags: u32,
}
impl NDIS_OFFLOAD_PARAMETERS {}
impl ::std::default::Default for NDIS_OFFLOAD_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_OFFLOAD_PARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_OFFLOAD_PARAMETERS")
            .field("Header", &self.Header)
            .field("IPv4Checksum", &self.IPv4Checksum)
            .field("TCPIPv4Checksum", &self.TCPIPv4Checksum)
            .field("UDPIPv4Checksum", &self.UDPIPv4Checksum)
            .field("TCPIPv6Checksum", &self.TCPIPv6Checksum)
            .field("UDPIPv6Checksum", &self.UDPIPv6Checksum)
            .field("LsoV1", &self.LsoV1)
            .field("IPsecV1", &self.IPsecV1)
            .field("LsoV2IPv4", &self.LsoV2IPv4)
            .field("LsoV2IPv6", &self.LsoV2IPv6)
            .field("TcpConnectionIPv4", &self.TcpConnectionIPv4)
            .field("TcpConnectionIPv6", &self.TcpConnectionIPv6)
            .field("Flags", &self.Flags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_OFFLOAD_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.IPv4Checksum == other.IPv4Checksum
            && self.TCPIPv4Checksum == other.TCPIPv4Checksum
            && self.UDPIPv4Checksum == other.UDPIPv4Checksum
            && self.TCPIPv6Checksum == other.TCPIPv6Checksum
            && self.UDPIPv6Checksum == other.UDPIPv6Checksum
            && self.LsoV1 == other.LsoV1
            && self.IPsecV1 == other.IPsecV1
            && self.LsoV2IPv4 == other.LsoV2IPv4
            && self.LsoV2IPv6 == other.LsoV2IPv6
            && self.TcpConnectionIPv4 == other.TcpConnectionIPv4
            && self.TcpConnectionIPv6 == other.TcpConnectionIPv6
            && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for NDIS_OFFLOAD_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for NDIS_OFFLOAD_PARAMETERS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_CONNECTION_OFFLOAD_DISABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_CONNECTION_OFFLOAD_ENABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_IPSECV1_AH_AND_ESP_ENABLED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_IPSECV1_AH_ENABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_IPSECV1_DISABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_IPSECV1_ESP_ENABLED: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_IPSECV2_AH_AND_ESP_ENABLED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_IPSECV2_AH_ENABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_IPSECV2_DISABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_IPSECV2_ESP_ENABLED: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_LSOV1_DISABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_LSOV1_ENABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_LSOV2_DISABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_LSOV2_ENABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_NO_CHANGE: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_REVISION_3: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_REVISION_4: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_REVISION_5: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_RSC_DISABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_RSC_ENABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_RX_ENABLED_TX_DISABLED: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_SKIP_REGISTRY_UPDATE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_TX_ENABLED_RX_DISABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_TX_RX_DISABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_TX_RX_ENABLED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_USO_DISABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_PARAMETERS_USO_ENABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_REVISION_3: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_REVISION_4: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_REVISION_5: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_REVISION_6: u32 = 6u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_REVISION_7: u32 = 7u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_SET_NO_CHANGE: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_SET_OFF: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_SET_ON: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OFFLOAD_SUPPORTED: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_NetworkManagement_IpHelper`*"]
pub struct NDIS_OPER_STATE {
    pub Header: NDIS_OBJECT_HEADER,
    pub OperationalStatus: super::IpHelper::NET_IF_OPER_STATUS,
    pub OperationalStatusFlags: u32,
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl NDIS_OPER_STATE {}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::default::Default for NDIS_OPER_STATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::fmt::Debug for NDIS_OPER_STATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_OPER_STATE").field("Header", &self.Header).field("OperationalStatus", &self.OperationalStatus).field("OperationalStatusFlags", &self.OperationalStatusFlags).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::cmp::PartialEq for NDIS_OPER_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.OperationalStatus == other.OperationalStatus && self.OperationalStatusFlags == other.OperationalStatusFlags
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::cmp::Eq for NDIS_OPER_STATE {}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
unsafe impl ::windows::runtime::Abi for NDIS_OPER_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_OPER_STATE_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PACKET_TYPE_ALL_FUNCTIONAL: u32 = 8192u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PACKET_TYPE_ALL_LOCAL: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PACKET_TYPE_ALL_MULTICAST: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PACKET_TYPE_BROADCAST: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PACKET_TYPE_DIRECTED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PACKET_TYPE_FUNCTIONAL: u32 = 16384u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PACKET_TYPE_GROUP: u32 = 4096u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PACKET_TYPE_MAC_FRAME: u32 = 32768u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PACKET_TYPE_MULTICAST: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PACKET_TYPE_NO_LOCAL: u32 = 65536u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PACKET_TYPE_PROMISCUOUS: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PACKET_TYPE_SMT: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PACKET_TYPE_SOURCE_ROUTING: u32 = 16u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_PCI_DEVICE_CUSTOM_PROPERTIES {
    pub Header: NDIS_OBJECT_HEADER,
    pub DeviceType: u32,
    pub CurrentSpeedAndMode: u32,
    pub CurrentPayloadSize: u32,
    pub MaxPayloadSize: u32,
    pub MaxReadRequestSize: u32,
    pub CurrentLinkSpeed: u32,
    pub CurrentLinkWidth: u32,
    pub MaxLinkSpeed: u32,
    pub MaxLinkWidth: u32,
    pub PciExpressVersion: u32,
    pub InterruptType: u32,
    pub MaxInterruptMessages: u32,
}
impl NDIS_PCI_DEVICE_CUSTOM_PROPERTIES {}
impl ::std::default::Default for NDIS_PCI_DEVICE_CUSTOM_PROPERTIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_PCI_DEVICE_CUSTOM_PROPERTIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_PCI_DEVICE_CUSTOM_PROPERTIES")
            .field("Header", &self.Header)
            .field("DeviceType", &self.DeviceType)
            .field("CurrentSpeedAndMode", &self.CurrentSpeedAndMode)
            .field("CurrentPayloadSize", &self.CurrentPayloadSize)
            .field("MaxPayloadSize", &self.MaxPayloadSize)
            .field("MaxReadRequestSize", &self.MaxReadRequestSize)
            .field("CurrentLinkSpeed", &self.CurrentLinkSpeed)
            .field("CurrentLinkWidth", &self.CurrentLinkWidth)
            .field("MaxLinkSpeed", &self.MaxLinkSpeed)
            .field("MaxLinkWidth", &self.MaxLinkWidth)
            .field("PciExpressVersion", &self.PciExpressVersion)
            .field("InterruptType", &self.InterruptType)
            .field("MaxInterruptMessages", &self.MaxInterruptMessages)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_PCI_DEVICE_CUSTOM_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.DeviceType == other.DeviceType
            && self.CurrentSpeedAndMode == other.CurrentSpeedAndMode
            && self.CurrentPayloadSize == other.CurrentPayloadSize
            && self.MaxPayloadSize == other.MaxPayloadSize
            && self.MaxReadRequestSize == other.MaxReadRequestSize
            && self.CurrentLinkSpeed == other.CurrentLinkSpeed
            && self.CurrentLinkWidth == other.CurrentLinkWidth
            && self.MaxLinkSpeed == other.MaxLinkSpeed
            && self.MaxLinkWidth == other.MaxLinkWidth
            && self.PciExpressVersion == other.PciExpressVersion
            && self.InterruptType == other.InterruptType
            && self.MaxInterruptMessages == other.MaxInterruptMessages
    }
}
impl ::std::cmp::Eq for NDIS_PCI_DEVICE_CUSTOM_PROPERTIES {}
unsafe impl ::windows::runtime::Abi for NDIS_PCI_DEVICE_CUSTOM_PROPERTIES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PD_CAPABILITIES_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PD_CAPS_DRAIN_NOTIFICATIONS_SUPPORTED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PD_CAPS_NOTIFICATION_MODERATION_COUNT_SUPPORTED: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PD_CAPS_NOTIFICATION_MODERATION_INTERVAL_SUPPORTED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PD_CAPS_RECEIVE_FILTER_COUNTERS_SUPPORTED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PD_CONFIG_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_PHYSICAL_MEDIUM(pub i32);
pub const NdisPhysicalMediumUnspecified: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(0i32);
pub const NdisPhysicalMediumWirelessLan: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(1i32);
pub const NdisPhysicalMediumCableModem: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(2i32);
pub const NdisPhysicalMediumPhoneLine: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(3i32);
pub const NdisPhysicalMediumPowerLine: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(4i32);
pub const NdisPhysicalMediumDSL: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(5i32);
pub const NdisPhysicalMediumFibreChannel: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(6i32);
pub const NdisPhysicalMedium1394: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(7i32);
pub const NdisPhysicalMediumWirelessWan: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(8i32);
pub const NdisPhysicalMediumNative802_11: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(9i32);
pub const NdisPhysicalMediumBluetooth: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(10i32);
pub const NdisPhysicalMediumInfiniband: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(11i32);
pub const NdisPhysicalMediumWiMax: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(12i32);
pub const NdisPhysicalMediumUWB: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(13i32);
pub const NdisPhysicalMedium802_3: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(14i32);
pub const NdisPhysicalMedium802_5: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(15i32);
pub const NdisPhysicalMediumIrda: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(16i32);
pub const NdisPhysicalMediumWiredWAN: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(17i32);
pub const NdisPhysicalMediumWiredCoWan: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(18i32);
pub const NdisPhysicalMediumOther: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(19i32);
pub const NdisPhysicalMediumNative802_15_4: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(20i32);
pub const NdisPhysicalMediumMax: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(21i32);
impl ::std::convert::From<i32> for NDIS_PHYSICAL_MEDIUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_PHYSICAL_MEDIUM {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_CAPABILITIES_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_CAPABILITIES_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_MAX_PATTERN_ID: u32 = 65535u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_MAX_STRING_SIZE: u32 = 64u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_PM_PACKET_PATTERN {
    pub Priority: u32,
    pub Reserved: u32,
    pub MaskSize: u32,
    pub PatternOffset: u32,
    pub PatternSize: u32,
    pub PatternFlags: u32,
}
impl NDIS_PM_PACKET_PATTERN {}
impl ::std::default::Default for NDIS_PM_PACKET_PATTERN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_PM_PACKET_PATTERN {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_PM_PACKET_PATTERN").field("Priority", &self.Priority).field("Reserved", &self.Reserved).field("MaskSize", &self.MaskSize).field("PatternOffset", &self.PatternOffset).field("PatternSize", &self.PatternSize).field("PatternFlags", &self.PatternFlags).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_PM_PACKET_PATTERN {
    fn eq(&self, other: &Self) -> bool {
        self.Priority == other.Priority && self.Reserved == other.Reserved && self.MaskSize == other.MaskSize && self.PatternOffset == other.PatternOffset && self.PatternSize == other.PatternSize && self.PatternFlags == other.PatternFlags
    }
}
impl ::std::cmp::Eq for NDIS_PM_PACKET_PATTERN {}
unsafe impl ::windows::runtime::Abi for NDIS_PM_PACKET_PATTERN {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_PARAMETERS_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_PRIVATE_PATTERN_ID: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_PROTOCOL_OFFLOAD_80211_RSN_REKEY_ENABLED: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_PROTOCOL_OFFLOAD_80211_RSN_REKEY_SUPPORTED: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_PROTOCOL_OFFLOAD_ARP_ENABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_PROTOCOL_OFFLOAD_ARP_SUPPORTED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_PROTOCOL_OFFLOAD_NS_ENABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_PROTOCOL_OFFLOAD_NS_SUPPORTED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_PROTOCOL_OFFLOAD_PRIORITY_HIGHEST: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_PROTOCOL_OFFLOAD_PRIORITY_LOWEST: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_PROTOCOL_OFFLOAD_PRIORITY_NORMAL: u32 = 268435456u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_PROTOCOL_OFFLOAD_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_PROTOCOL_OFFLOAD_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_SELECTIVE_SUSPEND_ENABLED: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_SELECTIVE_SUSPEND_SUPPORTED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_WAKE_ON_LINK_CHANGE_ENABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_WAKE_ON_MEDIA_CONNECT_SUPPORTED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_WAKE_ON_MEDIA_DISCONNECT_ENABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_WAKE_ON_MEDIA_DISCONNECT_SUPPORTED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_WAKE_PACKET_INDICATION_SUPPORTED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_WAKE_PACKET_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_WAKE_REASON_REVISION_1: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_PM_WAKE_UP_CAPABILITIES {
    pub MinMagicPacketWakeUp: NDIS_DEVICE_POWER_STATE,
    pub MinPatternWakeUp: NDIS_DEVICE_POWER_STATE,
    pub MinLinkChangeWakeUp: NDIS_DEVICE_POWER_STATE,
}
impl NDIS_PM_WAKE_UP_CAPABILITIES {}
impl ::std::default::Default for NDIS_PM_WAKE_UP_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_PM_WAKE_UP_CAPABILITIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_PM_WAKE_UP_CAPABILITIES").field("MinMagicPacketWakeUp", &self.MinMagicPacketWakeUp).field("MinPatternWakeUp", &self.MinPatternWakeUp).field("MinLinkChangeWakeUp", &self.MinLinkChangeWakeUp).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_PM_WAKE_UP_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.MinMagicPacketWakeUp == other.MinMagicPacketWakeUp && self.MinPatternWakeUp == other.MinPatternWakeUp && self.MinLinkChangeWakeUp == other.MinLinkChangeWakeUp
    }
}
impl ::std::cmp::Eq for NDIS_PM_WAKE_UP_CAPABILITIES {}
unsafe impl ::windows::runtime::Abi for NDIS_PM_WAKE_UP_CAPABILITIES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_WOL_BITMAP_PATTERN_ENABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_WOL_BITMAP_PATTERN_SUPPORTED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_WOL_EAPOL_REQUEST_ID_MESSAGE_ENABLED: u32 = 65536u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_WOL_EAPOL_REQUEST_ID_MESSAGE_SUPPORTED: u32 = 65536u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_WOL_IPV4_DEST_ADDR_WILDCARD_ENABLED: u32 = 512u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_WOL_IPV4_DEST_ADDR_WILDCARD_SUPPORTED: u32 = 512u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_WOL_IPV4_TCP_SYN_ENABLED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_WOL_IPV4_TCP_SYN_SUPPORTED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_WOL_IPV6_DEST_ADDR_WILDCARD_ENABLED: u32 = 2048u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_WOL_IPV6_DEST_ADDR_WILDCARD_SUPPORTED: u32 = 2048u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_WOL_IPV6_TCP_SYN_ENABLED: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_WOL_IPV6_TCP_SYN_SUPPORTED: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_WOL_MAGIC_PACKET_ENABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_WOL_MAGIC_PACKET_SUPPORTED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_WOL_PATTERN_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_WOL_PATTERN_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_WOL_PRIORITY_HIGHEST: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_WOL_PRIORITY_LOWEST: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PM_WOL_PRIORITY_NORMAL: u32 = 268435456u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_PNP_CAPABILITIES {
    pub Flags: u32,
    pub WakeUpCapabilities: NDIS_PM_WAKE_UP_CAPABILITIES,
}
impl NDIS_PNP_CAPABILITIES {}
impl ::std::default::Default for NDIS_PNP_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_PNP_CAPABILITIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_PNP_CAPABILITIES").field("Flags", &self.Flags).field("WakeUpCapabilities", &self.WakeUpCapabilities).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_PNP_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.WakeUpCapabilities == other.WakeUpCapabilities
    }
}
impl ::std::cmp::Eq for NDIS_PNP_CAPABILITIES {}
unsafe impl ::windows::runtime::Abi for NDIS_PNP_CAPABILITIES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PNP_WAKE_UP_LINK_CHANGE: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PNP_WAKE_UP_MAGIC_PACKET: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PNP_WAKE_UP_PATTERN_MATCH: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_NetworkManagement_IpHelper`*"]
pub struct NDIS_PORT {
    pub Next: *mut NDIS_PORT,
    pub NdisReserved: *mut ::std::ffi::c_void,
    pub MiniportReserved: *mut ::std::ffi::c_void,
    pub ProtocolReserved: *mut ::std::ffi::c_void,
    pub PortCharacteristics: NDIS_PORT_CHARACTERISTICS,
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl NDIS_PORT {}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::default::Default for NDIS_PORT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::fmt::Debug for NDIS_PORT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_PORT").field("Next", &self.Next).field("NdisReserved", &self.NdisReserved).field("MiniportReserved", &self.MiniportReserved).field("ProtocolReserved", &self.ProtocolReserved).field("PortCharacteristics", &self.PortCharacteristics).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::cmp::PartialEq for NDIS_PORT {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.NdisReserved == other.NdisReserved && self.MiniportReserved == other.MiniportReserved && self.ProtocolReserved == other.ProtocolReserved && self.PortCharacteristics == other.PortCharacteristics
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::cmp::Eq for NDIS_PORT {}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
unsafe impl ::windows::runtime::Abi for NDIS_PORT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_NetworkManagement_IpHelper`*"]
pub struct NDIS_PORT_ARRAY {
    pub Header: NDIS_OBJECT_HEADER,
    pub NumberOfPorts: u32,
    pub OffsetFirstPort: u32,
    pub ElementSize: u32,
    pub Ports: [NDIS_PORT_CHARACTERISTICS; 1],
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl NDIS_PORT_ARRAY {}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::default::Default for NDIS_PORT_ARRAY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::fmt::Debug for NDIS_PORT_ARRAY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_PORT_ARRAY").field("Header", &self.Header).field("NumberOfPorts", &self.NumberOfPorts).field("OffsetFirstPort", &self.OffsetFirstPort).field("ElementSize", &self.ElementSize).field("Ports", &self.Ports).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::cmp::PartialEq for NDIS_PORT_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.NumberOfPorts == other.NumberOfPorts && self.OffsetFirstPort == other.OffsetFirstPort && self.ElementSize == other.ElementSize && self.Ports == other.Ports
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::cmp::Eq for NDIS_PORT_ARRAY {}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
unsafe impl ::windows::runtime::Abi for NDIS_PORT_ARRAY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PORT_ARRAY_REVISION_1: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_PORT_AUTHENTICATION_PARAMETERS {
    pub Header: NDIS_OBJECT_HEADER,
    pub SendControlState: NDIS_PORT_CONTROL_STATE,
    pub RcvControlState: NDIS_PORT_CONTROL_STATE,
    pub SendAuthorizationState: NDIS_PORT_AUTHORIZATION_STATE,
    pub RcvAuthorizationState: NDIS_PORT_AUTHORIZATION_STATE,
}
impl NDIS_PORT_AUTHENTICATION_PARAMETERS {}
impl ::std::default::Default for NDIS_PORT_AUTHENTICATION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_PORT_AUTHENTICATION_PARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_PORT_AUTHENTICATION_PARAMETERS")
            .field("Header", &self.Header)
            .field("SendControlState", &self.SendControlState)
            .field("RcvControlState", &self.RcvControlState)
            .field("SendAuthorizationState", &self.SendAuthorizationState)
            .field("RcvAuthorizationState", &self.RcvAuthorizationState)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_PORT_AUTHENTICATION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.SendControlState == other.SendControlState && self.RcvControlState == other.RcvControlState && self.SendAuthorizationState == other.SendAuthorizationState && self.RcvAuthorizationState == other.RcvAuthorizationState
    }
}
impl ::std::cmp::Eq for NDIS_PORT_AUTHENTICATION_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for NDIS_PORT_AUTHENTICATION_PARAMETERS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PORT_AUTHENTICATION_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_PORT_AUTHORIZATION_STATE(pub i32);
pub const NdisPortAuthorizationUnknown: NDIS_PORT_AUTHORIZATION_STATE = NDIS_PORT_AUTHORIZATION_STATE(0i32);
pub const NdisPortAuthorized: NDIS_PORT_AUTHORIZATION_STATE = NDIS_PORT_AUTHORIZATION_STATE(1i32);
pub const NdisPortUnauthorized: NDIS_PORT_AUTHORIZATION_STATE = NDIS_PORT_AUTHORIZATION_STATE(2i32);
pub const NdisPortReauthorizing: NDIS_PORT_AUTHORIZATION_STATE = NDIS_PORT_AUTHORIZATION_STATE(3i32);
impl ::std::convert::From<i32> for NDIS_PORT_AUTHORIZATION_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_PORT_AUTHORIZATION_STATE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_NetworkManagement_IpHelper`*"]
pub struct NDIS_PORT_CHARACTERISTICS {
    pub Header: NDIS_OBJECT_HEADER,
    pub PortNumber: u32,
    pub Flags: u32,
    pub Type: NDIS_PORT_TYPE,
    pub MediaConnectState: super::IpHelper::NET_IF_MEDIA_CONNECT_STATE,
    pub XmitLinkSpeed: u64,
    pub RcvLinkSpeed: u64,
    pub Direction: super::IpHelper::NET_IF_DIRECTION_TYPE,
    pub SendControlState: NDIS_PORT_CONTROL_STATE,
    pub RcvControlState: NDIS_PORT_CONTROL_STATE,
    pub SendAuthorizationState: NDIS_PORT_AUTHORIZATION_STATE,
    pub RcvAuthorizationState: NDIS_PORT_AUTHORIZATION_STATE,
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl NDIS_PORT_CHARACTERISTICS {}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::default::Default for NDIS_PORT_CHARACTERISTICS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::fmt::Debug for NDIS_PORT_CHARACTERISTICS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_PORT_CHARACTERISTICS")
            .field("Header", &self.Header)
            .field("PortNumber", &self.PortNumber)
            .field("Flags", &self.Flags)
            .field("Type", &self.Type)
            .field("MediaConnectState", &self.MediaConnectState)
            .field("XmitLinkSpeed", &self.XmitLinkSpeed)
            .field("RcvLinkSpeed", &self.RcvLinkSpeed)
            .field("Direction", &self.Direction)
            .field("SendControlState", &self.SendControlState)
            .field("RcvControlState", &self.RcvControlState)
            .field("SendAuthorizationState", &self.SendAuthorizationState)
            .field("RcvAuthorizationState", &self.RcvAuthorizationState)
            .finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::cmp::PartialEq for NDIS_PORT_CHARACTERISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.PortNumber == other.PortNumber
            && self.Flags == other.Flags
            && self.Type == other.Type
            && self.MediaConnectState == other.MediaConnectState
            && self.XmitLinkSpeed == other.XmitLinkSpeed
            && self.RcvLinkSpeed == other.RcvLinkSpeed
            && self.Direction == other.Direction
            && self.SendControlState == other.SendControlState
            && self.RcvControlState == other.RcvControlState
            && self.SendAuthorizationState == other.SendAuthorizationState
            && self.RcvAuthorizationState == other.RcvAuthorizationState
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::cmp::Eq for NDIS_PORT_CHARACTERISTICS {}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
unsafe impl ::windows::runtime::Abi for NDIS_PORT_CHARACTERISTICS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PORT_CHARACTERISTICS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PORT_CHAR_USE_DEFAULT_AUTH_SETTINGS: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_PORT_CONTROL_STATE(pub i32);
pub const NdisPortControlStateUnknown: NDIS_PORT_CONTROL_STATE = NDIS_PORT_CONTROL_STATE(0i32);
pub const NdisPortControlStateControlled: NDIS_PORT_CONTROL_STATE = NDIS_PORT_CONTROL_STATE(1i32);
pub const NdisPortControlStateUncontrolled: NDIS_PORT_CONTROL_STATE = NDIS_PORT_CONTROL_STATE(2i32);
impl ::std::convert::From<i32> for NDIS_PORT_CONTROL_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_PORT_CONTROL_STATE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_NetworkManagement_IpHelper`*"]
pub struct NDIS_PORT_STATE {
    pub Header: NDIS_OBJECT_HEADER,
    pub MediaConnectState: super::IpHelper::NET_IF_MEDIA_CONNECT_STATE,
    pub XmitLinkSpeed: u64,
    pub RcvLinkSpeed: u64,
    pub Direction: super::IpHelper::NET_IF_DIRECTION_TYPE,
    pub SendControlState: NDIS_PORT_CONTROL_STATE,
    pub RcvControlState: NDIS_PORT_CONTROL_STATE,
    pub SendAuthorizationState: NDIS_PORT_AUTHORIZATION_STATE,
    pub RcvAuthorizationState: NDIS_PORT_AUTHORIZATION_STATE,
    pub Flags: u32,
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl NDIS_PORT_STATE {}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::default::Default for NDIS_PORT_STATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::fmt::Debug for NDIS_PORT_STATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_PORT_STATE")
            .field("Header", &self.Header)
            .field("MediaConnectState", &self.MediaConnectState)
            .field("XmitLinkSpeed", &self.XmitLinkSpeed)
            .field("RcvLinkSpeed", &self.RcvLinkSpeed)
            .field("Direction", &self.Direction)
            .field("SendControlState", &self.SendControlState)
            .field("RcvControlState", &self.RcvControlState)
            .field("SendAuthorizationState", &self.SendAuthorizationState)
            .field("RcvAuthorizationState", &self.RcvAuthorizationState)
            .field("Flags", &self.Flags)
            .finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::cmp::PartialEq for NDIS_PORT_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.MediaConnectState == other.MediaConnectState && self.XmitLinkSpeed == other.XmitLinkSpeed && self.RcvLinkSpeed == other.RcvLinkSpeed && self.Direction == other.Direction && self.SendControlState == other.SendControlState && self.RcvControlState == other.RcvControlState && self.SendAuthorizationState == other.SendAuthorizationState && self.RcvAuthorizationState == other.RcvAuthorizationState && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::cmp::Eq for NDIS_PORT_STATE {}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
unsafe impl ::windows::runtime::Abi for NDIS_PORT_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PORT_STATE_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_PORT_TYPE(pub i32);
pub const NdisPortTypeUndefined: NDIS_PORT_TYPE = NDIS_PORT_TYPE(0i32);
pub const NdisPortTypeBridge: NDIS_PORT_TYPE = NDIS_PORT_TYPE(1i32);
pub const NdisPortTypeRasConnection: NDIS_PORT_TYPE = NDIS_PORT_TYPE(2i32);
pub const NdisPortType8021xSupplicant: NDIS_PORT_TYPE = NDIS_PORT_TYPE(3i32);
pub const NdisPortTypeMax: NDIS_PORT_TYPE = NDIS_PORT_TYPE(4i32);
impl ::std::convert::From<i32> for NDIS_PORT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_PORT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_PROCESSOR_VENDOR(pub i32);
pub const NdisProcessorVendorUnknown: NDIS_PROCESSOR_VENDOR = NDIS_PROCESSOR_VENDOR(0i32);
pub const NdisProcessorVendorGenuinIntel: NDIS_PROCESSOR_VENDOR = NDIS_PROCESSOR_VENDOR(1i32);
pub const NdisProcessorVendorGenuineIntel: NDIS_PROCESSOR_VENDOR = NDIS_PROCESSOR_VENDOR(1i32);
pub const NdisProcessorVendorAuthenticAMD: NDIS_PROCESSOR_VENDOR = NDIS_PROCESSOR_VENDOR(2i32);
impl ::std::convert::From<i32> for NDIS_PROCESSOR_VENDOR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_PROCESSOR_VENDOR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PROTOCOL_ID_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PROTOCOL_ID_IP6: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PROTOCOL_ID_IPX: u32 = 6u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PROTOCOL_ID_MASK: u32 = 15u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PROTOCOL_ID_MAX: u32 = 15u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PROTOCOL_ID_NBF: u32 = 7u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PROTOCOL_ID_TCP_IP: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PROT_OPTION_ESTIMATED_LENGTH: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PROT_OPTION_NO_LOOPBACK: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PROT_OPTION_NO_RSVD_ON_RCVPKT: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_PROT_OPTION_SEND_RESTRICTED: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_ACTION_MAXIMUM: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_ACTION_PRIORITY: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_CAPABILITIES_CEE_DCBX_SUPPORTED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_CAPABILITIES_IEEE_DCBX_SUPPORTED: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_CAPABILITIES_MACSEC_BYPASS_SUPPORTED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_CAPABILITIES_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_CAPABILITIES_STRICT_TSA_SUPPORTED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_CLASSIFICATION_ELEMENT_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_CLASSIFICATION_ENFORCED_BY_MINIPORT: u32 = 16777216u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_CLASSIFICATION_SET_BY_MINIPORT_MASK: u32 = 4278190080u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_CONDITION_DEFAULT: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_CONDITION_ETHERTYPE: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_CONDITION_MAXIMUM: u32 = 7u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_CONDITION_NETDIRECT_PORT: u32 = 6u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_CONDITION_RESERVED: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_CONDITION_TCP_OR_UDP_PORT: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_CONDITION_TCP_PORT: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_CONDITION_UDP_PORT: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_DEFAULT_SQ_ID: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_MAXIMUM_PRIORITIES: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_MAXIMUM_TRAFFIC_CLASSES: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_OFFLOAD_CAPABILITIES_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_OFFLOAD_CAPABILITIES_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_OFFLOAD_CAPS_GFT_SQ: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_OFFLOAD_CAPS_STANDARD_SQ: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_PARAMETERS_CLASSIFICATION_CHANGED: u32 = 65536u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_PARAMETERS_CLASSIFICATION_CONFIGURED: u32 = 131072u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_PARAMETERS_ETS_CHANGED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_PARAMETERS_ETS_CONFIGURED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_PARAMETERS_PFC_CHANGED: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_PARAMETERS_PFC_CONFIGURED: u32 = 512u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_PARAMETERS_WILLING: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_SQ_ARRAY_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_SQ_PARAMETERS_ARRAY_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_SQ_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_SQ_PARAMETERS_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_SQ_RECEIVE_CAP_ENABLED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_SQ_STATS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_SQ_TRANSMIT_CAP_ENABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_SQ_TRANSMIT_RESERVATION_ENABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_TSA_CBS: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_TSA_ETS: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_TSA_MAXIMUM: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_QOS_TSA_STRICT: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_ANY_VLAN_SUPPORTED: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_ARP_HEADER_OPERATION_SUPPORTED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_ARP_HEADER_SPA_SUPPORTED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_ARP_HEADER_SUPPORTED: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_ARP_HEADER_TPA_SUPPORTED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_CAPABILITIES_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_CAPABILITIES_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_CLEAR_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_DYNAMIC_PROCESSOR_AFFINITY_CHANGE_FOR_DEFAULT_QUEUE_SUPPORTED: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_DYNAMIC_PROCESSOR_AFFINITY_CHANGE_SUPPORTED: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_FIELD_MAC_HEADER_VLAN_UNTAGGED_OR_ZERO: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_FIELD_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_FIELD_PARAMETERS_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_FLAGS_RESERVED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_GLOBAL_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_IMPLAT_MIN_OF_QUEUES_MODE: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_IMPLAT_SUM_OF_QUEUES_MODE: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_INFO_ARRAY_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_INFO_ARRAY_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_INFO_ARRAY_VPORT_ID_SPECIFIED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_INFO_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_INTERRUPT_VECTOR_COALESCING_SUPPORTED: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_IPV4_HEADER_PROTOCOL_SUPPORTED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_IPV4_HEADER_SUPPORTED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_IPV6_HEADER_PROTOCOL_SUPPORTED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_IPV6_HEADER_SUPPORTED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_LOOKAHEAD_SPLIT_SUPPORTED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_MAC_HEADER_DEST_ADDR_SUPPORTED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_MAC_HEADER_PACKET_TYPE_SUPPORTED: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_MAC_HEADER_PRIORITY_SUPPORTED: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_MAC_HEADER_PROTOCOL_SUPPORTED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_MAC_HEADER_SOURCE_ADDR_SUPPORTED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_MAC_HEADER_SUPPORTED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_MAC_HEADER_VLAN_ID_SUPPORTED: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_MOVE_FILTER_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_MSI_X_SUPPORTED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_PACKET_COALESCING_FILTERS_ENABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_PACKET_COALESCING_SUPPORTED_ON_DEFAULT_QUEUE: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_PACKET_ENCAPSULATION: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_PACKET_ENCAPSULATION_GRE: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_PARAMETERS_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_RESERVED: u32 = 254u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_TEST_HEADER_FIELD_EQUAL_SUPPORTED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_TEST_HEADER_FIELD_MASK_EQUAL_SUPPORTED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_TEST_HEADER_FIELD_NOT_EQUAL_SUPPORTED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_UDP_HEADER_DEST_PORT_SUPPORTED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_UDP_HEADER_SUPPORTED: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_VMQ_FILTERS_ENABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_VM_QUEUES_ENABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_FILTER_VM_QUEUE_SUPPORTED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_HASH_FLAG_ENABLE_HASH: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_HASH_FLAG_HASH_INFO_UNCHANGED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_HASH_FLAG_HASH_KEY_UNCHANGED: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_RECEIVE_HASH_PARAMETERS {
    pub Header: NDIS_OBJECT_HEADER,
    pub Flags: u32,
    pub HashInformation: u32,
    pub HashSecretKeySize: u16,
    pub HashSecretKeyOffset: u32,
}
impl NDIS_RECEIVE_HASH_PARAMETERS {}
impl ::std::default::Default for NDIS_RECEIVE_HASH_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_RECEIVE_HASH_PARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_RECEIVE_HASH_PARAMETERS").field("Header", &self.Header).field("Flags", &self.Flags).field("HashInformation", &self.HashInformation).field("HashSecretKeySize", &self.HashSecretKeySize).field("HashSecretKeyOffset", &self.HashSecretKeyOffset).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_RECEIVE_HASH_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Flags == other.Flags && self.HashInformation == other.HashInformation && self.HashSecretKeySize == other.HashSecretKeySize && self.HashSecretKeyOffset == other.HashSecretKeyOffset
    }
}
impl ::std::cmp::Eq for NDIS_RECEIVE_HASH_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for NDIS_RECEIVE_HASH_PARAMETERS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_HASH_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_QUEUE_ALLOCATION_COMPLETE_ARRAY_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_QUEUE_ALLOCATION_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_QUEUE_FREE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_QUEUE_INFO_ARRAY_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_QUEUE_INFO_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_QUEUE_INFO_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_QUEUE_PARAMETERS_CHANGE_MASK: u32 = 4294901760u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_QUEUE_PARAMETERS_FLAGS_CHANGED: u32 = 65536u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_QUEUE_PARAMETERS_INTERRUPT_COALESCING_DOMAIN_ID_CHANGED: u32 = 1048576u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_QUEUE_PARAMETERS_LOOKAHEAD_SPLIT_REQUIRED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_QUEUE_PARAMETERS_NAME_CHANGED: u32 = 524288u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_QUEUE_PARAMETERS_PER_QUEUE_RECEIVE_INDICATION: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_QUEUE_PARAMETERS_PROCESSOR_AFFINITY_CHANGED: u32 = 131072u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_QUEUE_PARAMETERS_QOS_SQ_ID_CHANGED: u32 = 2097152u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_QUEUE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_QUEUE_PARAMETERS_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_QUEUE_PARAMETERS_REVISION_3: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_QUEUE_PARAMETERS_SUGGESTED_RECV_BUFFER_NUMBERS_CHANGED: u32 = 262144u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_RECEIVE_SCALE_CAPABILITIES {
    pub Header: NDIS_OBJECT_HEADER,
    pub CapabilitiesFlags: u32,
    pub NumberOfInterruptMessages: u32,
    pub NumberOfReceiveQueues: u32,
}
impl NDIS_RECEIVE_SCALE_CAPABILITIES {}
impl ::std::default::Default for NDIS_RECEIVE_SCALE_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_RECEIVE_SCALE_CAPABILITIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_RECEIVE_SCALE_CAPABILITIES").field("Header", &self.Header).field("CapabilitiesFlags", &self.CapabilitiesFlags).field("NumberOfInterruptMessages", &self.NumberOfInterruptMessages).field("NumberOfReceiveQueues", &self.NumberOfReceiveQueues).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_RECEIVE_SCALE_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.CapabilitiesFlags == other.CapabilitiesFlags && self.NumberOfInterruptMessages == other.NumberOfInterruptMessages && self.NumberOfReceiveQueues == other.NumberOfReceiveQueues
    }
}
impl ::std::cmp::Eq for NDIS_RECEIVE_SCALE_CAPABILITIES {}
unsafe impl ::windows::runtime::Abi for NDIS_RECEIVE_SCALE_CAPABILITIES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_SCALE_CAPABILITIES_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_SCALE_CAPABILITIES_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_SCALE_CAPABILITIES_REVISION_3: u32 = 3u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_RECEIVE_SCALE_PARAMETERS {
    pub Header: NDIS_OBJECT_HEADER,
    pub Flags: u16,
    pub BaseCpuNumber: u16,
    pub HashInformation: u32,
    pub IndirectionTableSize: u16,
    pub IndirectionTableOffset: u32,
    pub HashSecretKeySize: u16,
    pub HashSecretKeyOffset: u32,
}
impl NDIS_RECEIVE_SCALE_PARAMETERS {}
impl ::std::default::Default for NDIS_RECEIVE_SCALE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_RECEIVE_SCALE_PARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_RECEIVE_SCALE_PARAMETERS")
            .field("Header", &self.Header)
            .field("Flags", &self.Flags)
            .field("BaseCpuNumber", &self.BaseCpuNumber)
            .field("HashInformation", &self.HashInformation)
            .field("IndirectionTableSize", &self.IndirectionTableSize)
            .field("IndirectionTableOffset", &self.IndirectionTableOffset)
            .field("HashSecretKeySize", &self.HashSecretKeySize)
            .field("HashSecretKeyOffset", &self.HashSecretKeyOffset)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_RECEIVE_SCALE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Flags == other.Flags && self.BaseCpuNumber == other.BaseCpuNumber && self.HashInformation == other.HashInformation && self.IndirectionTableSize == other.IndirectionTableSize && self.IndirectionTableOffset == other.IndirectionTableOffset && self.HashSecretKeySize == other.HashSecretKeySize && self.HashSecretKeyOffset == other.HashSecretKeyOffset
    }
}
impl ::std::cmp::Eq for NDIS_RECEIVE_SCALE_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for NDIS_RECEIVE_SCALE_PARAMETERS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_SCALE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_SCALE_PARAMETERS_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_SCALE_PARAMETERS_REVISION_3: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_SCALE_PARAMETERS_V2_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_SCALE_PARAM_ENABLE_RSS: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_SCALE_PARAM_HASH_INFO_CHANGED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_SCALE_PARAM_HASH_KEY_CHANGED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_SCALE_PARAM_NUMBER_OF_ENTRIES_CHANGED: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RECEIVE_SCALE_PARAM_NUMBER_OF_QUEUES_CHANGED: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_REQUEST_TYPE(pub i32);
pub const NdisRequestQueryInformation: NDIS_REQUEST_TYPE = NDIS_REQUEST_TYPE(0i32);
pub const NdisRequestSetInformation: NDIS_REQUEST_TYPE = NDIS_REQUEST_TYPE(1i32);
pub const NdisRequestQueryStatistics: NDIS_REQUEST_TYPE = NDIS_REQUEST_TYPE(2i32);
pub const NdisRequestOpen: NDIS_REQUEST_TYPE = NDIS_REQUEST_TYPE(3i32);
pub const NdisRequestClose: NDIS_REQUEST_TYPE = NDIS_REQUEST_TYPE(4i32);
pub const NdisRequestSend: NDIS_REQUEST_TYPE = NDIS_REQUEST_TYPE(5i32);
pub const NdisRequestTransferData: NDIS_REQUEST_TYPE = NDIS_REQUEST_TYPE(6i32);
pub const NdisRequestReset: NDIS_REQUEST_TYPE = NDIS_REQUEST_TYPE(7i32);
pub const NdisRequestGeneric1: NDIS_REQUEST_TYPE = NDIS_REQUEST_TYPE(8i32);
pub const NdisRequestGeneric2: NDIS_REQUEST_TYPE = NDIS_REQUEST_TYPE(9i32);
pub const NdisRequestGeneric3: NDIS_REQUEST_TYPE = NDIS_REQUEST_TYPE(10i32);
pub const NdisRequestGeneric4: NDIS_REQUEST_TYPE = NDIS_REQUEST_TYPE(11i32);
impl ::std::convert::From<i32> for NDIS_REQUEST_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_REQUEST_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RING_AUTO_REMOVAL_ERROR: u32 = 1024u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RING_COUNTER_OVERFLOW: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RING_HARD_ERROR: u32 = 16384u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RING_LOBE_WIRE_FAULT: u32 = 2048u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RING_REMOVE_RECEIVED: u32 = 512u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RING_RING_RECOVERY: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RING_SIGNAL_LOSS: u32 = 32768u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RING_SINGLE_STATION: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RING_SOFT_ERROR: u32 = 8192u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RING_TRANSMIT_BEACON: u32 = 4096u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_ROUTING_DOMAIN_ENTRY_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_ROUTING_DOMAIN_ISOLATION_ENTRY_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSC_STATISTICS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_CAPS_CLASSIFICATION_AT_DPC: u32 = 67108864u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_CAPS_CLASSIFICATION_AT_ISR: u32 = 33554432u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_CAPS_HASH_TYPE_TCP_IPV4: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_CAPS_HASH_TYPE_TCP_IPV6: u32 = 512u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_CAPS_HASH_TYPE_TCP_IPV6_EX: u32 = 1024u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_CAPS_HASH_TYPE_UDP_IPV4: u32 = 2048u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_CAPS_HASH_TYPE_UDP_IPV6: u32 = 4096u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_CAPS_HASH_TYPE_UDP_IPV6_EX: u32 = 8192u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_CAPS_MESSAGE_SIGNALED_INTERRUPTS: u32 = 16777216u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_CAPS_RSS_AVAILABLE_ON_PORTS: u32 = 268435456u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_CAPS_SUPPORTS_INDEPENDENT_ENTRY_MOVE: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_CAPS_SUPPORTS_MSI_X: u32 = 536870912u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_CAPS_USING_MSI_X: u32 = 134217728u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_HASH_SECRET_KEY_MAX_SIZE_REVISION_1: u32 = 40u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_HASH_SECRET_KEY_MAX_SIZE_REVISION_2: u32 = 40u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_HASH_SECRET_KEY_MAX_SIZE_REVISION_3: u32 = 40u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_HASH_SECRET_KEY_SIZE_REVISION_1: u32 = 40u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_INDIRECTION_TABLE_MAX_SIZE_REVISION_1: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_INDIRECTION_TABLE_SIZE_REVISION_1: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_PARAM_FLAG_BASE_CPU_UNCHANGED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_PARAM_FLAG_DEFAULT_PROCESSOR_UNCHANGED: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_PARAM_FLAG_DISABLE_RSS: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_PARAM_FLAG_HASH_INFO_UNCHANGED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_PARAM_FLAG_HASH_KEY_UNCHANGED: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_PARAM_FLAG_ITABLE_UNCHANGED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_PROCESSOR_INFO_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_PROCESSOR_INFO_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_SET_INDIRECTION_ENTRIES_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_SET_INDIRECTION_ENTRY_FLAG_DEFAULT_PROCESSOR: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_RSS_SET_INDIRECTION_ENTRY_FLAG_PRIMARY_PROCESSOR: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SIZEOF_NDIS_PM_PROTOCOL_OFFLOAD_REVISION_1: u32 = 240u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SRIOV_BAR_RESOURCES_INFO_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SRIOV_CAPABILITIES_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SRIOV_CAPS_PF_MINIPORT: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SRIOV_CAPS_SRIOV_SUPPORTED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SRIOV_CAPS_VF_MINIPORT: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SRIOV_CONFIG_STATE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SRIOV_OVERLYING_ADAPTER_INFO_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SRIOV_PF_LUID_INFO_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SRIOV_PROBED_BARS_INFO_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SRIOV_READ_VF_CONFIG_BLOCK_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SRIOV_READ_VF_CONFIG_SPACE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SRIOV_RESET_VF_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SRIOV_SET_VF_POWER_STATE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SRIOV_VF_INVALIDATE_CONFIG_BLOCK_INFO_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SRIOV_VF_SERIAL_NUMBER_INFO_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SRIOV_VF_VENDOR_DEVICE_ID_INFO_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SRIOV_WRITE_VF_CONFIG_BLOCK_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SRIOV_WRITE_VF_CONFIG_SPACE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_STATISTICS_FLAGS_VALID_BROADCAST_BYTES_RCV: u32 = 262144u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_STATISTICS_FLAGS_VALID_BROADCAST_BYTES_XMIT: u32 = 2097152u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_STATISTICS_FLAGS_VALID_BROADCAST_FRAMES_RCV: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_STATISTICS_FLAGS_VALID_BROADCAST_FRAMES_XMIT: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_STATISTICS_FLAGS_VALID_BYTES_RCV: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_STATISTICS_FLAGS_VALID_BYTES_XMIT: u32 = 512u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_STATISTICS_FLAGS_VALID_DIRECTED_BYTES_RCV: u32 = 65536u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_STATISTICS_FLAGS_VALID_DIRECTED_BYTES_XMIT: u32 = 524288u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_STATISTICS_FLAGS_VALID_DIRECTED_FRAMES_RCV: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_STATISTICS_FLAGS_VALID_DIRECTED_FRAMES_XMIT: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_STATISTICS_FLAGS_VALID_MULTICAST_BYTES_RCV: u32 = 131072u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_STATISTICS_FLAGS_VALID_MULTICAST_BYTES_XMIT: u32 = 1048576u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_STATISTICS_FLAGS_VALID_MULTICAST_FRAMES_RCV: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_STATISTICS_FLAGS_VALID_MULTICAST_FRAMES_XMIT: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_STATISTICS_FLAGS_VALID_RCV_DISCARDS: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_STATISTICS_FLAGS_VALID_RCV_ERROR: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_STATISTICS_FLAGS_VALID_XMIT_DISCARDS: u32 = 32768u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_STATISTICS_FLAGS_VALID_XMIT_ERROR: u32 = 1024u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_STATISTICS_INFO {
    pub Header: NDIS_OBJECT_HEADER,
    pub SupportedStatistics: u32,
    pub ifInDiscards: u64,
    pub ifInErrors: u64,
    pub ifHCInOctets: u64,
    pub ifHCInUcastPkts: u64,
    pub ifHCInMulticastPkts: u64,
    pub ifHCInBroadcastPkts: u64,
    pub ifHCOutOctets: u64,
    pub ifHCOutUcastPkts: u64,
    pub ifHCOutMulticastPkts: u64,
    pub ifHCOutBroadcastPkts: u64,
    pub ifOutErrors: u64,
    pub ifOutDiscards: u64,
    pub ifHCInUcastOctets: u64,
    pub ifHCInMulticastOctets: u64,
    pub ifHCInBroadcastOctets: u64,
    pub ifHCOutUcastOctets: u64,
    pub ifHCOutMulticastOctets: u64,
    pub ifHCOutBroadcastOctets: u64,
}
impl NDIS_STATISTICS_INFO {}
impl ::std::default::Default for NDIS_STATISTICS_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_STATISTICS_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_STATISTICS_INFO")
            .field("Header", &self.Header)
            .field("SupportedStatistics", &self.SupportedStatistics)
            .field("ifInDiscards", &self.ifInDiscards)
            .field("ifInErrors", &self.ifInErrors)
            .field("ifHCInOctets", &self.ifHCInOctets)
            .field("ifHCInUcastPkts", &self.ifHCInUcastPkts)
            .field("ifHCInMulticastPkts", &self.ifHCInMulticastPkts)
            .field("ifHCInBroadcastPkts", &self.ifHCInBroadcastPkts)
            .field("ifHCOutOctets", &self.ifHCOutOctets)
            .field("ifHCOutUcastPkts", &self.ifHCOutUcastPkts)
            .field("ifHCOutMulticastPkts", &self.ifHCOutMulticastPkts)
            .field("ifHCOutBroadcastPkts", &self.ifHCOutBroadcastPkts)
            .field("ifOutErrors", &self.ifOutErrors)
            .field("ifOutDiscards", &self.ifOutDiscards)
            .field("ifHCInUcastOctets", &self.ifHCInUcastOctets)
            .field("ifHCInMulticastOctets", &self.ifHCInMulticastOctets)
            .field("ifHCInBroadcastOctets", &self.ifHCInBroadcastOctets)
            .field("ifHCOutUcastOctets", &self.ifHCOutUcastOctets)
            .field("ifHCOutMulticastOctets", &self.ifHCOutMulticastOctets)
            .field("ifHCOutBroadcastOctets", &self.ifHCOutBroadcastOctets)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_STATISTICS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.SupportedStatistics == other.SupportedStatistics
            && self.ifInDiscards == other.ifInDiscards
            && self.ifInErrors == other.ifInErrors
            && self.ifHCInOctets == other.ifHCInOctets
            && self.ifHCInUcastPkts == other.ifHCInUcastPkts
            && self.ifHCInMulticastPkts == other.ifHCInMulticastPkts
            && self.ifHCInBroadcastPkts == other.ifHCInBroadcastPkts
            && self.ifHCOutOctets == other.ifHCOutOctets
            && self.ifHCOutUcastPkts == other.ifHCOutUcastPkts
            && self.ifHCOutMulticastPkts == other.ifHCOutMulticastPkts
            && self.ifHCOutBroadcastPkts == other.ifHCOutBroadcastPkts
            && self.ifOutErrors == other.ifOutErrors
            && self.ifOutDiscards == other.ifOutDiscards
            && self.ifHCInUcastOctets == other.ifHCInUcastOctets
            && self.ifHCInMulticastOctets == other.ifHCInMulticastOctets
            && self.ifHCInBroadcastOctets == other.ifHCInBroadcastOctets
            && self.ifHCOutUcastOctets == other.ifHCOutUcastOctets
            && self.ifHCOutMulticastOctets == other.ifHCOutMulticastOctets
            && self.ifHCOutBroadcastOctets == other.ifHCOutBroadcastOctets
    }
}
impl ::std::cmp::Eq for NDIS_STATISTICS_INFO {}
unsafe impl ::windows::runtime::Abi for NDIS_STATISTICS_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_STATISTICS_INFO_REVISION_1: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_STATISTICS_VALUE {
    pub Oid: u32,
    pub DataLength: u32,
    pub Data: [u8; 1],
}
impl NDIS_STATISTICS_VALUE {}
impl ::std::default::Default for NDIS_STATISTICS_VALUE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_STATISTICS_VALUE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_STATISTICS_VALUE").field("Oid", &self.Oid).field("DataLength", &self.DataLength).field("Data", &self.Data).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_STATISTICS_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.Oid == other.Oid && self.DataLength == other.DataLength && self.Data == other.Data
    }
}
impl ::std::cmp::Eq for NDIS_STATISTICS_VALUE {}
unsafe impl ::windows::runtime::Abi for NDIS_STATISTICS_VALUE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_STATISTICS_VALUE_EX {
    pub Oid: u32,
    pub DataLength: u32,
    pub Length: u32,
    pub Data: [u8; 1],
}
impl NDIS_STATISTICS_VALUE_EX {}
impl ::std::default::Default for NDIS_STATISTICS_VALUE_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_STATISTICS_VALUE_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_STATISTICS_VALUE_EX").field("Oid", &self.Oid).field("DataLength", &self.DataLength).field("Length", &self.Length).field("Data", &self.Data).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_STATISTICS_VALUE_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Oid == other.Oid && self.DataLength == other.DataLength && self.Length == other.Length && self.Data == other.Data
    }
}
impl ::std::cmp::Eq for NDIS_STATISTICS_VALUE_EX {}
unsafe impl ::windows::runtime::Abi for NDIS_STATISTICS_VALUE_EX {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_SUPPORTED_PAUSE_FUNCTIONS(pub i32);
pub const NdisPauseFunctionsUnsupported: NDIS_SUPPORTED_PAUSE_FUNCTIONS = NDIS_SUPPORTED_PAUSE_FUNCTIONS(0i32);
pub const NdisPauseFunctionsSendOnly: NDIS_SUPPORTED_PAUSE_FUNCTIONS = NDIS_SUPPORTED_PAUSE_FUNCTIONS(1i32);
pub const NdisPauseFunctionsReceiveOnly: NDIS_SUPPORTED_PAUSE_FUNCTIONS = NDIS_SUPPORTED_PAUSE_FUNCTIONS(2i32);
pub const NdisPauseFunctionsSendAndReceive: NDIS_SUPPORTED_PAUSE_FUNCTIONS = NDIS_SUPPORTED_PAUSE_FUNCTIONS(3i32);
pub const NdisPauseFunctionsUnknown: NDIS_SUPPORTED_PAUSE_FUNCTIONS = NDIS_SUPPORTED_PAUSE_FUNCTIONS(4i32);
impl ::std::convert::From<i32> for NDIS_SUPPORTED_PAUSE_FUNCTIONS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_SUPPORTED_PAUSE_FUNCTIONS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SUPPORT_NDIS6: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SUPPORT_NDIS61: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SUPPORT_NDIS620: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SUPPORT_NDIS630: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SUPPORT_NDIS640: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SUPPORT_NDIS650: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SUPPORT_NDIS651: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SUPPORT_NDIS660: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SUPPORT_NDIS670: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SUPPORT_NDIS680: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SUPPORT_NDIS681: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SUPPORT_NDIS682: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SUPPORT_NDIS683: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SUPPORT_NDIS684: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SUPPORT_NDIS685: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SUPPORT_NDIS686: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_FEATURE_STATUS_CUSTOM_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_FEATURE_STATUS_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_NIC_ARRAY_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_NIC_FLAGS_MAPPED_NIC_UPDATED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_NIC_FLAGS_NIC_INITIALIZING: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_NIC_FLAGS_NIC_SUSPENDED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_NIC_FLAGS_NIC_SUSPENDED_LM: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_NIC_OID_REQUEST_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_NIC_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_NIC_PARAMETERS_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_NIC_SAVE_STATE_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_NIC_SAVE_STATE_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_OBJECT_SERIALIZATION_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_PORT_ARRAY_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_PORT_FEATURE_STATUS_CUSTOM_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_PORT_FEATURE_STATUS_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_PORT_PARAMETERS_FLAG_RESTORING_PORT: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_PORT_PARAMETERS_FLAG_UNTRUSTED_INTERNAL_PORT: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_PORT_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_PORT_PROPERTY_CUSTOM_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_PORT_PROPERTY_DELETE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_PORT_PROPERTY_ENUM_INFO_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_PORT_PROPERTY_ENUM_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_PORT_PROPERTY_ISOLATION_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_PORT_PROPERTY_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_PORT_PROPERTY_PROFILE_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_PORT_PROPERTY_ROUTING_DOMAIN_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_PORT_PROPERTY_SECURITY_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_PORT_PROPERTY_SECURITY_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_PORT_PROPERTY_VLAN_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_PROPERTY_CUSTOM_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_PROPERTY_DELETE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_PROPERTY_ENUM_INFO_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_PROPERTY_ENUM_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SWITCH_PROPERTY_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_SYSTEM_PROCESSOR_INFO_EX_REVISION_1: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_TCP_CONNECTION_OFFLOAD {
    pub Header: NDIS_OBJECT_HEADER,
    pub Encapsulation: u32,
    pub _bitfield: u32,
    pub TcpConnectionOffloadCapacity: u32,
    pub Flags: u32,
}
impl NDIS_TCP_CONNECTION_OFFLOAD {}
impl ::std::default::Default for NDIS_TCP_CONNECTION_OFFLOAD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_TCP_CONNECTION_OFFLOAD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_TCP_CONNECTION_OFFLOAD").field("Header", &self.Header).field("Encapsulation", &self.Encapsulation).field("_bitfield", &self._bitfield).field("TcpConnectionOffloadCapacity", &self.TcpConnectionOffloadCapacity).field("Flags", &self.Flags).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_TCP_CONNECTION_OFFLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Encapsulation == other.Encapsulation && self._bitfield == other._bitfield && self.TcpConnectionOffloadCapacity == other.TcpConnectionOffloadCapacity && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for NDIS_TCP_CONNECTION_OFFLOAD {}
unsafe impl ::windows::runtime::Abi for NDIS_TCP_CONNECTION_OFFLOAD {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_TCP_CONNECTION_OFFLOAD_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_TCP_CONNECTION_OFFLOAD_REVISION_2: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_TCP_IP_CHECKSUM_OFFLOAD {
    pub IPv4Transmit: NDIS_TCP_IP_CHECKSUM_OFFLOAD_1,
    pub IPv4Receive: NDIS_TCP_IP_CHECKSUM_OFFLOAD_0,
    pub IPv6Transmit: NDIS_TCP_IP_CHECKSUM_OFFLOAD_3,
    pub IPv6Receive: NDIS_TCP_IP_CHECKSUM_OFFLOAD_2,
}
impl NDIS_TCP_IP_CHECKSUM_OFFLOAD {}
impl ::std::default::Default for NDIS_TCP_IP_CHECKSUM_OFFLOAD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_TCP_IP_CHECKSUM_OFFLOAD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_TCP_IP_CHECKSUM_OFFLOAD").field("IPv4Transmit", &self.IPv4Transmit).field("IPv4Receive", &self.IPv4Receive).field("IPv6Transmit", &self.IPv6Transmit).field("IPv6Receive", &self.IPv6Receive).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_TCP_IP_CHECKSUM_OFFLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.IPv4Transmit == other.IPv4Transmit && self.IPv4Receive == other.IPv4Receive && self.IPv6Transmit == other.IPv6Transmit && self.IPv6Receive == other.IPv6Receive
    }
}
impl ::std::cmp::Eq for NDIS_TCP_IP_CHECKSUM_OFFLOAD {}
unsafe impl ::windows::runtime::Abi for NDIS_TCP_IP_CHECKSUM_OFFLOAD {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_TCP_IP_CHECKSUM_OFFLOAD_0 {
    pub Encapsulation: u32,
    pub _bitfield: u32,
}
impl NDIS_TCP_IP_CHECKSUM_OFFLOAD_0 {}
impl ::std::default::Default for NDIS_TCP_IP_CHECKSUM_OFFLOAD_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_TCP_IP_CHECKSUM_OFFLOAD_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_IPv4Receive_e__Struct").field("Encapsulation", &self.Encapsulation).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_TCP_IP_CHECKSUM_OFFLOAD_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for NDIS_TCP_IP_CHECKSUM_OFFLOAD_0 {}
unsafe impl ::windows::runtime::Abi for NDIS_TCP_IP_CHECKSUM_OFFLOAD_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_TCP_IP_CHECKSUM_OFFLOAD_1 {
    pub Encapsulation: u32,
    pub _bitfield: u32,
}
impl NDIS_TCP_IP_CHECKSUM_OFFLOAD_1 {}
impl ::std::default::Default for NDIS_TCP_IP_CHECKSUM_OFFLOAD_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_TCP_IP_CHECKSUM_OFFLOAD_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_IPv4Transmit_e__Struct").field("Encapsulation", &self.Encapsulation).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_TCP_IP_CHECKSUM_OFFLOAD_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for NDIS_TCP_IP_CHECKSUM_OFFLOAD_1 {}
unsafe impl ::windows::runtime::Abi for NDIS_TCP_IP_CHECKSUM_OFFLOAD_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_TCP_IP_CHECKSUM_OFFLOAD_2 {
    pub Encapsulation: u32,
    pub _bitfield: u32,
}
impl NDIS_TCP_IP_CHECKSUM_OFFLOAD_2 {}
impl ::std::default::Default for NDIS_TCP_IP_CHECKSUM_OFFLOAD_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_TCP_IP_CHECKSUM_OFFLOAD_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_IPv6Receive_e__Struct").field("Encapsulation", &self.Encapsulation).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_TCP_IP_CHECKSUM_OFFLOAD_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for NDIS_TCP_IP_CHECKSUM_OFFLOAD_2 {}
unsafe impl ::windows::runtime::Abi for NDIS_TCP_IP_CHECKSUM_OFFLOAD_2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_TCP_IP_CHECKSUM_OFFLOAD_3 {
    pub Encapsulation: u32,
    pub _bitfield: u32,
}
impl NDIS_TCP_IP_CHECKSUM_OFFLOAD_3 {}
impl ::std::default::Default for NDIS_TCP_IP_CHECKSUM_OFFLOAD_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_TCP_IP_CHECKSUM_OFFLOAD_3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_IPv6Transmit_e__Struct").field("Encapsulation", &self.Encapsulation).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_TCP_IP_CHECKSUM_OFFLOAD_3 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for NDIS_TCP_IP_CHECKSUM_OFFLOAD_3 {}
unsafe impl ::windows::runtime::Abi for NDIS_TCP_IP_CHECKSUM_OFFLOAD_3 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_TCP_LARGE_SEND_OFFLOAD_V1 {
    pub IPv4: NDIS_TCP_LARGE_SEND_OFFLOAD_V1_0,
}
impl NDIS_TCP_LARGE_SEND_OFFLOAD_V1 {}
impl ::std::default::Default for NDIS_TCP_LARGE_SEND_OFFLOAD_V1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_TCP_LARGE_SEND_OFFLOAD_V1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_TCP_LARGE_SEND_OFFLOAD_V1").field("IPv4", &self.IPv4).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_TCP_LARGE_SEND_OFFLOAD_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.IPv4 == other.IPv4
    }
}
impl ::std::cmp::Eq for NDIS_TCP_LARGE_SEND_OFFLOAD_V1 {}
unsafe impl ::windows::runtime::Abi for NDIS_TCP_LARGE_SEND_OFFLOAD_V1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    pub Encapsulation: u32,
    pub MaxOffLoadSize: u32,
    pub MinSegmentCount: u32,
    pub _bitfield: u32,
}
impl NDIS_TCP_LARGE_SEND_OFFLOAD_V1_0 {}
impl ::std::default::Default for NDIS_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_IPv4_e__Struct").field("Encapsulation", &self.Encapsulation).field("MaxOffLoadSize", &self.MaxOffLoadSize).field("MinSegmentCount", &self.MinSegmentCount).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.MaxOffLoadSize == other.MaxOffLoadSize && self.MinSegmentCount == other.MinSegmentCount && self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for NDIS_TCP_LARGE_SEND_OFFLOAD_V1_0 {}
unsafe impl ::windows::runtime::Abi for NDIS_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_TCP_LARGE_SEND_OFFLOAD_V2 {
    pub IPv4: NDIS_TCP_LARGE_SEND_OFFLOAD_V2_0,
    pub IPv6: NDIS_TCP_LARGE_SEND_OFFLOAD_V2_1,
}
impl NDIS_TCP_LARGE_SEND_OFFLOAD_V2 {}
impl ::std::default::Default for NDIS_TCP_LARGE_SEND_OFFLOAD_V2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_TCP_LARGE_SEND_OFFLOAD_V2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_TCP_LARGE_SEND_OFFLOAD_V2").field("IPv4", &self.IPv4).field("IPv6", &self.IPv6).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_TCP_LARGE_SEND_OFFLOAD_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.IPv4 == other.IPv4 && self.IPv6 == other.IPv6
    }
}
impl ::std::cmp::Eq for NDIS_TCP_LARGE_SEND_OFFLOAD_V2 {}
unsafe impl ::windows::runtime::Abi for NDIS_TCP_LARGE_SEND_OFFLOAD_V2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    pub Encapsulation: u32,
    pub MaxOffLoadSize: u32,
    pub MinSegmentCount: u32,
}
impl NDIS_TCP_LARGE_SEND_OFFLOAD_V2_0 {}
impl ::std::default::Default for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_IPv4_e__Struct").field("Encapsulation", &self.Encapsulation).field("MaxOffLoadSize", &self.MaxOffLoadSize).field("MinSegmentCount", &self.MinSegmentCount).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.MaxOffLoadSize == other.MaxOffLoadSize && self.MinSegmentCount == other.MinSegmentCount
    }
}
impl ::std::cmp::Eq for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_0 {}
unsafe impl ::windows::runtime::Abi for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    pub Encapsulation: u32,
    pub MaxOffLoadSize: u32,
    pub MinSegmentCount: u32,
    pub _bitfield: u32,
}
impl NDIS_TCP_LARGE_SEND_OFFLOAD_V2_1 {}
impl ::std::default::Default for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_IPv6_e__Struct").field("Encapsulation", &self.Encapsulation).field("MaxOffLoadSize", &self.MaxOffLoadSize).field("MinSegmentCount", &self.MinSegmentCount).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.MaxOffLoadSize == other.MaxOffLoadSize && self.MinSegmentCount == other.MinSegmentCount && self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_1 {}
unsafe impl ::windows::runtime::Abi for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_TCP_RECV_SEG_COALESC_OFFLOAD_REVISION_1: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES {
    pub Header: NDIS_OBJECT_HEADER,
    pub Flags: u32,
    pub TimeoutArrayLength: u32,
    pub TimeoutArray: [u32; 1],
}
impl NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES {}
impl ::std::default::Default for NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES").field("Header", &self.Header).field("Flags", &self.Flags).field("TimeoutArrayLength", &self.TimeoutArrayLength).field("TimeoutArray", &self.TimeoutArray).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Flags == other.Flags && self.TimeoutArrayLength == other.TimeoutArrayLength && self.TimeoutArray == other.TimeoutArray
    }
}
impl ::std::cmp::Eq for NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES {}
unsafe impl ::windows::runtime::Abi for NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES_REVISION_1: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
pub struct NDIS_TIMESTAMP_CAPABILITIES {
    pub Header: NDIS_OBJECT_HEADER,
    pub HardwareClockFrequencyHz: u64,
    pub CrossTimestamp: super::super::Foundation::BOOLEAN,
    pub Reserved1: u64,
    pub Reserved2: u64,
    pub TimestampFlags: NDIS_TIMESTAMP_CAPABILITY_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl NDIS_TIMESTAMP_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NDIS_TIMESTAMP_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NDIS_TIMESTAMP_CAPABILITIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_TIMESTAMP_CAPABILITIES")
            .field("Header", &self.Header)
            .field("HardwareClockFrequencyHz", &self.HardwareClockFrequencyHz)
            .field("CrossTimestamp", &self.CrossTimestamp)
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .field("TimestampFlags", &self.TimestampFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NDIS_TIMESTAMP_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.HardwareClockFrequencyHz == other.HardwareClockFrequencyHz && self.CrossTimestamp == other.CrossTimestamp && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.TimestampFlags == other.TimestampFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NDIS_TIMESTAMP_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NDIS_TIMESTAMP_CAPABILITIES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_TIMESTAMP_CAPABILITIES_REVISION_1: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
pub struct NDIS_TIMESTAMP_CAPABILITY_FLAGS {
    pub PtpV2OverUdpIPv4EventMsgReceiveHw: super::super::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv4AllMsgReceiveHw: super::super::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv4EventMsgTransmitHw: super::super::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv4AllMsgTransmitHw: super::super::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv6EventMsgReceiveHw: super::super::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv6AllMsgReceiveHw: super::super::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv6EventMsgTransmitHw: super::super::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv6AllMsgTransmitHw: super::super::Foundation::BOOLEAN,
    pub AllReceiveHw: super::super::Foundation::BOOLEAN,
    pub AllTransmitHw: super::super::Foundation::BOOLEAN,
    pub TaggedTransmitHw: super::super::Foundation::BOOLEAN,
    pub AllReceiveSw: super::super::Foundation::BOOLEAN,
    pub AllTransmitSw: super::super::Foundation::BOOLEAN,
    pub TaggedTransmitSw: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl NDIS_TIMESTAMP_CAPABILITY_FLAGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NDIS_TIMESTAMP_CAPABILITY_FLAGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NDIS_TIMESTAMP_CAPABILITY_FLAGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_TIMESTAMP_CAPABILITY_FLAGS")
            .field("PtpV2OverUdpIPv4EventMsgReceiveHw", &self.PtpV2OverUdpIPv4EventMsgReceiveHw)
            .field("PtpV2OverUdpIPv4AllMsgReceiveHw", &self.PtpV2OverUdpIPv4AllMsgReceiveHw)
            .field("PtpV2OverUdpIPv4EventMsgTransmitHw", &self.PtpV2OverUdpIPv4EventMsgTransmitHw)
            .field("PtpV2OverUdpIPv4AllMsgTransmitHw", &self.PtpV2OverUdpIPv4AllMsgTransmitHw)
            .field("PtpV2OverUdpIPv6EventMsgReceiveHw", &self.PtpV2OverUdpIPv6EventMsgReceiveHw)
            .field("PtpV2OverUdpIPv6AllMsgReceiveHw", &self.PtpV2OverUdpIPv6AllMsgReceiveHw)
            .field("PtpV2OverUdpIPv6EventMsgTransmitHw", &self.PtpV2OverUdpIPv6EventMsgTransmitHw)
            .field("PtpV2OverUdpIPv6AllMsgTransmitHw", &self.PtpV2OverUdpIPv6AllMsgTransmitHw)
            .field("AllReceiveHw", &self.AllReceiveHw)
            .field("AllTransmitHw", &self.AllTransmitHw)
            .field("TaggedTransmitHw", &self.TaggedTransmitHw)
            .field("AllReceiveSw", &self.AllReceiveSw)
            .field("AllTransmitSw", &self.AllTransmitSw)
            .field("TaggedTransmitSw", &self.TaggedTransmitSw)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NDIS_TIMESTAMP_CAPABILITY_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self.PtpV2OverUdpIPv4EventMsgReceiveHw == other.PtpV2OverUdpIPv4EventMsgReceiveHw
            && self.PtpV2OverUdpIPv4AllMsgReceiveHw == other.PtpV2OverUdpIPv4AllMsgReceiveHw
            && self.PtpV2OverUdpIPv4EventMsgTransmitHw == other.PtpV2OverUdpIPv4EventMsgTransmitHw
            && self.PtpV2OverUdpIPv4AllMsgTransmitHw == other.PtpV2OverUdpIPv4AllMsgTransmitHw
            && self.PtpV2OverUdpIPv6EventMsgReceiveHw == other.PtpV2OverUdpIPv6EventMsgReceiveHw
            && self.PtpV2OverUdpIPv6AllMsgReceiveHw == other.PtpV2OverUdpIPv6AllMsgReceiveHw
            && self.PtpV2OverUdpIPv6EventMsgTransmitHw == other.PtpV2OverUdpIPv6EventMsgTransmitHw
            && self.PtpV2OverUdpIPv6AllMsgTransmitHw == other.PtpV2OverUdpIPv6AllMsgTransmitHw
            && self.AllReceiveHw == other.AllReceiveHw
            && self.AllTransmitHw == other.AllTransmitHw
            && self.TaggedTransmitHw == other.TaggedTransmitHw
            && self.AllReceiveSw == other.AllReceiveSw
            && self.AllTransmitSw == other.AllTransmitSw
            && self.TaggedTransmitSw == other.TaggedTransmitSw
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NDIS_TIMESTAMP_CAPABILITY_FLAGS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NDIS_TIMESTAMP_CAPABILITY_FLAGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_VAR_DATA_DESC {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Offset: usize,
}
impl NDIS_VAR_DATA_DESC {}
impl ::std::default::Default for NDIS_VAR_DATA_DESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_VAR_DATA_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_VAR_DATA_DESC").field("Length", &self.Length).field("MaximumLength", &self.MaximumLength).field("Offset", &self.Offset).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_VAR_DATA_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.MaximumLength == other.MaximumLength && self.Offset == other.Offset
    }
}
impl ::std::cmp::Eq for NDIS_VAR_DATA_DESC {}
unsafe impl ::windows::runtime::Abi for NDIS_VAR_DATA_DESC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_WAN_HEADER_FORMAT(pub i32);
pub const NdisWanHeaderNative: NDIS_WAN_HEADER_FORMAT = NDIS_WAN_HEADER_FORMAT(0i32);
pub const NdisWanHeaderEthernet: NDIS_WAN_HEADER_FORMAT = NDIS_WAN_HEADER_FORMAT(1i32);
impl ::std::convert::From<i32> for NDIS_WAN_HEADER_FORMAT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_WAN_HEADER_FORMAT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_WAN_MEDIUM_SUBTYPE(pub i32);
pub const NdisWanMediumHub: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(0i32);
pub const NdisWanMediumX_25: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(1i32);
pub const NdisWanMediumIsdn: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(2i32);
pub const NdisWanMediumSerial: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(3i32);
pub const NdisWanMediumFrameRelay: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(4i32);
pub const NdisWanMediumAtm: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(5i32);
pub const NdisWanMediumSonet: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(6i32);
pub const NdisWanMediumSW56K: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(7i32);
pub const NdisWanMediumPPTP: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(8i32);
pub const NdisWanMediumL2TP: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(9i32);
pub const NdisWanMediumIrda: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(10i32);
pub const NdisWanMediumParallel: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(11i32);
pub const NdisWanMediumPppoe: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(12i32);
pub const NdisWanMediumSSTP: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(13i32);
pub const NdisWanMediumAgileVPN: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(14i32);
pub const NdisWanMediumGre: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(15i32);
pub const NdisWanMediumSubTypeMax: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(16i32);
impl ::std::convert::From<i32> for NDIS_WAN_MEDIUM_SUBTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_WAN_MEDIUM_SUBTYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_WAN_PROTOCOL_CAPS {
    pub Flags: u32,
    pub Reserved: u32,
}
impl NDIS_WAN_PROTOCOL_CAPS {}
impl ::std::default::Default for NDIS_WAN_PROTOCOL_CAPS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_WAN_PROTOCOL_CAPS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_WAN_PROTOCOL_CAPS").field("Flags", &self.Flags).field("Reserved", &self.Reserved).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_WAN_PROTOCOL_CAPS {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for NDIS_WAN_PROTOCOL_CAPS {}
unsafe impl ::windows::runtime::Abi for NDIS_WAN_PROTOCOL_CAPS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDIS_WAN_QUALITY(pub i32);
pub const NdisWanRaw: NDIS_WAN_QUALITY = NDIS_WAN_QUALITY(0i32);
pub const NdisWanErrorControl: NDIS_WAN_QUALITY = NDIS_WAN_QUALITY(1i32);
pub const NdisWanReliable: NDIS_WAN_QUALITY = NDIS_WAN_QUALITY(2i32);
impl ::std::convert::From<i32> for NDIS_WAN_QUALITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDIS_WAN_QUALITY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_WLAN_BSSID {
    pub Length: u32,
    pub MacAddress: [u8; 6],
    pub Reserved: [u8; 2],
    pub Ssid: NDIS_802_11_SSID,
    pub Privacy: u32,
    pub Rssi: i32,
    pub NetworkTypeInUse: NDIS_802_11_NETWORK_TYPE,
    pub Configuration: NDIS_802_11_CONFIGURATION,
    pub InfrastructureMode: NDIS_802_11_NETWORK_INFRASTRUCTURE,
    pub SupportedRates: [u8; 8],
}
impl NDIS_WLAN_BSSID {}
impl ::std::default::Default for NDIS_WLAN_BSSID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_WLAN_BSSID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_WLAN_BSSID")
            .field("Length", &self.Length)
            .field("MacAddress", &self.MacAddress)
            .field("Reserved", &self.Reserved)
            .field("Ssid", &self.Ssid)
            .field("Privacy", &self.Privacy)
            .field("Rssi", &self.Rssi)
            .field("NetworkTypeInUse", &self.NetworkTypeInUse)
            .field("Configuration", &self.Configuration)
            .field("InfrastructureMode", &self.InfrastructureMode)
            .field("SupportedRates", &self.SupportedRates)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_WLAN_BSSID {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.MacAddress == other.MacAddress && self.Reserved == other.Reserved && self.Ssid == other.Ssid && self.Privacy == other.Privacy && self.Rssi == other.Rssi && self.NetworkTypeInUse == other.NetworkTypeInUse && self.Configuration == other.Configuration && self.InfrastructureMode == other.InfrastructureMode && self.SupportedRates == other.SupportedRates
    }
}
impl ::std::cmp::Eq for NDIS_WLAN_BSSID {}
unsafe impl ::windows::runtime::Abi for NDIS_WLAN_BSSID {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_WLAN_BSSID_EX {
    pub Length: u32,
    pub MacAddress: [u8; 6],
    pub Reserved: [u8; 2],
    pub Ssid: NDIS_802_11_SSID,
    pub Privacy: u32,
    pub Rssi: i32,
    pub NetworkTypeInUse: NDIS_802_11_NETWORK_TYPE,
    pub Configuration: NDIS_802_11_CONFIGURATION,
    pub InfrastructureMode: NDIS_802_11_NETWORK_INFRASTRUCTURE,
    pub SupportedRates: [u8; 16],
    pub IELength: u32,
    pub IEs: [u8; 1],
}
impl NDIS_WLAN_BSSID_EX {}
impl ::std::default::Default for NDIS_WLAN_BSSID_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_WLAN_BSSID_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_WLAN_BSSID_EX")
            .field("Length", &self.Length)
            .field("MacAddress", &self.MacAddress)
            .field("Reserved", &self.Reserved)
            .field("Ssid", &self.Ssid)
            .field("Privacy", &self.Privacy)
            .field("Rssi", &self.Rssi)
            .field("NetworkTypeInUse", &self.NetworkTypeInUse)
            .field("Configuration", &self.Configuration)
            .field("InfrastructureMode", &self.InfrastructureMode)
            .field("SupportedRates", &self.SupportedRates)
            .field("IELength", &self.IELength)
            .field("IEs", &self.IEs)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_WLAN_BSSID_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.MacAddress == other.MacAddress && self.Reserved == other.Reserved && self.Ssid == other.Ssid && self.Privacy == other.Privacy && self.Rssi == other.Rssi && self.NetworkTypeInUse == other.NetworkTypeInUse && self.Configuration == other.Configuration && self.InfrastructureMode == other.InfrastructureMode && self.SupportedRates == other.SupportedRates && self.IELength == other.IELength && self.IEs == other.IEs
    }
}
impl ::std::cmp::Eq for NDIS_WLAN_BSSID_EX {}
unsafe impl ::windows::runtime::Abi for NDIS_WLAN_BSSID_EX {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WLAN_WAKE_ON_4WAY_HANDSHAKE_REQUEST_ENABLED: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WLAN_WAKE_ON_4WAY_HANDSHAKE_REQUEST_SUPPORTED: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WLAN_WAKE_ON_AP_ASSOCIATION_LOST_ENABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WLAN_WAKE_ON_AP_ASSOCIATION_LOST_SUPPORTED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WLAN_WAKE_ON_GTK_HANDSHAKE_ERROR_ENABLED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WLAN_WAKE_ON_GTK_HANDSHAKE_ERROR_SUPPORTED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WLAN_WAKE_ON_NLO_DISCOVERY_ENABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WLAN_WAKE_ON_NLO_DISCOVERY_SUPPORTED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WMI_DEFAULT_METHOD_ID: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_IpHelper"))]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`, `Win32_NetworkManagement_IpHelper`*"]
pub struct NDIS_WMI_ENUM_ADAPTER {
    pub Header: NDIS_OBJECT_HEADER,
    pub IfIndex: u32,
    pub NetLuid: super::IpHelper::NET_LUID_LH,
    pub DeviceNameLength: u16,
    pub DeviceName: [super::super::Foundation::CHAR; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_IpHelper"))]
impl NDIS_WMI_ENUM_ADAPTER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_IpHelper"))]
impl ::std::default::Default for NDIS_WMI_ENUM_ADAPTER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_IpHelper"))]
impl ::std::cmp::PartialEq for NDIS_WMI_ENUM_ADAPTER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_IpHelper"))]
impl ::std::cmp::Eq for NDIS_WMI_ENUM_ADAPTER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_IpHelper"))]
unsafe impl ::windows::runtime::Abi for NDIS_WMI_ENUM_ADAPTER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WMI_ENUM_ADAPTER_REVISION_1: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_NetworkManagement_IpHelper`*"]
pub struct NDIS_WMI_EVENT_HEADER {
    pub Header: NDIS_OBJECT_HEADER,
    pub IfIndex: u32,
    pub NetLuid: super::IpHelper::NET_LUID_LH,
    pub RequestId: u64,
    pub PortNumber: u32,
    pub DeviceNameLength: u32,
    pub DeviceNameOffset: u32,
    pub Padding: [u8; 4],
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl NDIS_WMI_EVENT_HEADER {}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::default::Default for NDIS_WMI_EVENT_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::cmp::PartialEq for NDIS_WMI_EVENT_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::cmp::Eq for NDIS_WMI_EVENT_HEADER {}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
unsafe impl ::windows::runtime::Abi for NDIS_WMI_EVENT_HEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WMI_EVENT_HEADER_REVISION_1: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_WMI_IPSEC_OFFLOAD_V1 {
    pub Supported: NDIS_WMI_IPSEC_OFFLOAD_V1_2,
    pub IPv4AH: NDIS_WMI_IPSEC_OFFLOAD_V1_0,
    pub IPv4ESP: NDIS_WMI_IPSEC_OFFLOAD_V1_1,
}
impl NDIS_WMI_IPSEC_OFFLOAD_V1 {}
impl ::std::default::Default for NDIS_WMI_IPSEC_OFFLOAD_V1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_WMI_IPSEC_OFFLOAD_V1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_WMI_IPSEC_OFFLOAD_V1").field("Supported", &self.Supported).field("IPv4AH", &self.IPv4AH).field("IPv4ESP", &self.IPv4ESP).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_WMI_IPSEC_OFFLOAD_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Supported == other.Supported && self.IPv4AH == other.IPv4AH && self.IPv4ESP == other.IPv4ESP
    }
}
impl ::std::cmp::Eq for NDIS_WMI_IPSEC_OFFLOAD_V1 {}
unsafe impl ::windows::runtime::Abi for NDIS_WMI_IPSEC_OFFLOAD_V1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_WMI_IPSEC_OFFLOAD_V1_0 {
    pub Md5: u32,
    pub Sha_1: u32,
    pub Transport: u32,
    pub Tunnel: u32,
    pub Send: u32,
    pub Receive: u32,
}
impl NDIS_WMI_IPSEC_OFFLOAD_V1_0 {}
impl ::std::default::Default for NDIS_WMI_IPSEC_OFFLOAD_V1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_WMI_IPSEC_OFFLOAD_V1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_IPv4AH_e__Struct").field("Md5", &self.Md5).field("Sha_1", &self.Sha_1).field("Transport", &self.Transport).field("Tunnel", &self.Tunnel).field("Send", &self.Send).field("Receive", &self.Receive).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_WMI_IPSEC_OFFLOAD_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Md5 == other.Md5 && self.Sha_1 == other.Sha_1 && self.Transport == other.Transport && self.Tunnel == other.Tunnel && self.Send == other.Send && self.Receive == other.Receive
    }
}
impl ::std::cmp::Eq for NDIS_WMI_IPSEC_OFFLOAD_V1_0 {}
unsafe impl ::windows::runtime::Abi for NDIS_WMI_IPSEC_OFFLOAD_V1_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_WMI_IPSEC_OFFLOAD_V1_1 {
    pub Des: u32,
    pub Reserved: u32,
    pub TripleDes: u32,
    pub NullEsp: u32,
    pub Transport: u32,
    pub Tunnel: u32,
    pub Send: u32,
    pub Receive: u32,
}
impl NDIS_WMI_IPSEC_OFFLOAD_V1_1 {}
impl ::std::default::Default for NDIS_WMI_IPSEC_OFFLOAD_V1_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_WMI_IPSEC_OFFLOAD_V1_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_IPv4ESP_e__Struct").field("Des", &self.Des).field("Reserved", &self.Reserved).field("TripleDes", &self.TripleDes).field("NullEsp", &self.NullEsp).field("Transport", &self.Transport).field("Tunnel", &self.Tunnel).field("Send", &self.Send).field("Receive", &self.Receive).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_WMI_IPSEC_OFFLOAD_V1_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Des == other.Des && self.Reserved == other.Reserved && self.TripleDes == other.TripleDes && self.NullEsp == other.NullEsp && self.Transport == other.Transport && self.Tunnel == other.Tunnel && self.Send == other.Send && self.Receive == other.Receive
    }
}
impl ::std::cmp::Eq for NDIS_WMI_IPSEC_OFFLOAD_V1_1 {}
unsafe impl ::windows::runtime::Abi for NDIS_WMI_IPSEC_OFFLOAD_V1_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_WMI_IPSEC_OFFLOAD_V1_2 {
    pub Encapsulation: u32,
    pub AhEspCombined: u32,
    pub TransportTunnelCombined: u32,
    pub IPv4Options: u32,
    pub Flags: u32,
}
impl NDIS_WMI_IPSEC_OFFLOAD_V1_2 {}
impl ::std::default::Default for NDIS_WMI_IPSEC_OFFLOAD_V1_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_WMI_IPSEC_OFFLOAD_V1_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Supported_e__Struct").field("Encapsulation", &self.Encapsulation).field("AhEspCombined", &self.AhEspCombined).field("TransportTunnelCombined", &self.TransportTunnelCombined).field("IPv4Options", &self.IPv4Options).field("Flags", &self.Flags).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_WMI_IPSEC_OFFLOAD_V1_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.AhEspCombined == other.AhEspCombined && self.TransportTunnelCombined == other.TransportTunnelCombined && self.IPv4Options == other.IPv4Options && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for NDIS_WMI_IPSEC_OFFLOAD_V1_2 {}
unsafe impl ::windows::runtime::Abi for NDIS_WMI_IPSEC_OFFLOAD_V1_2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_NetworkManagement_IpHelper`*"]
pub struct NDIS_WMI_METHOD_HEADER {
    pub Header: NDIS_OBJECT_HEADER,
    pub PortNumber: u32,
    pub NetLuid: super::IpHelper::NET_LUID_LH,
    pub RequestId: u64,
    pub Timeout: u32,
    pub Padding: [u8; 4],
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl NDIS_WMI_METHOD_HEADER {}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::default::Default for NDIS_WMI_METHOD_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::cmp::PartialEq for NDIS_WMI_METHOD_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::cmp::Eq for NDIS_WMI_METHOD_HEADER {}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
unsafe impl ::windows::runtime::Abi for NDIS_WMI_METHOD_HEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WMI_METHOD_HEADER_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WMI_OBJECT_TYPE_ENUM_ADAPTER: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WMI_OBJECT_TYPE_EVENT: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WMI_OBJECT_TYPE_METHOD: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WMI_OBJECT_TYPE_OUTPUT_INFO: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WMI_OBJECT_TYPE_SET: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_WMI_OFFLOAD {
    pub Header: NDIS_OBJECT_HEADER,
    pub Checksum: NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD,
    pub LsoV1: NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1,
    pub IPsecV1: NDIS_WMI_IPSEC_OFFLOAD_V1,
    pub LsoV2: NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2,
    pub Flags: u32,
}
impl NDIS_WMI_OFFLOAD {}
impl ::std::default::Default for NDIS_WMI_OFFLOAD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_WMI_OFFLOAD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_WMI_OFFLOAD").field("Header", &self.Header).field("Checksum", &self.Checksum).field("LsoV1", &self.LsoV1).field("IPsecV1", &self.IPsecV1).field("LsoV2", &self.LsoV2).field("Flags", &self.Flags).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_WMI_OFFLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Checksum == other.Checksum && self.LsoV1 == other.LsoV1 && self.IPsecV1 == other.IPsecV1 && self.LsoV2 == other.LsoV2 && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for NDIS_WMI_OFFLOAD {}
unsafe impl ::windows::runtime::Abi for NDIS_WMI_OFFLOAD {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_WMI_OUTPUT_INFO {
    pub Header: NDIS_OBJECT_HEADER,
    pub Flags: u32,
    pub SupportedRevision: u8,
    pub DataOffset: u32,
}
impl NDIS_WMI_OUTPUT_INFO {}
impl ::std::default::Default for NDIS_WMI_OUTPUT_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_WMI_OUTPUT_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_WMI_OUTPUT_INFO").field("Header", &self.Header).field("Flags", &self.Flags).field("SupportedRevision", &self.SupportedRevision).field("DataOffset", &self.DataOffset).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_WMI_OUTPUT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Flags == other.Flags && self.SupportedRevision == other.SupportedRevision && self.DataOffset == other.DataOffset
    }
}
impl ::std::cmp::Eq for NDIS_WMI_OUTPUT_INFO {}
unsafe impl ::windows::runtime::Abi for NDIS_WMI_OUTPUT_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WMI_PM_ACTIVE_CAPABILITIES_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WMI_PM_ADMIN_CONFIG_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WMI_RECEIVE_QUEUE_INFO_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WMI_RECEIVE_QUEUE_PARAMETERS_REVISION_1: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_NetworkManagement_IpHelper`*"]
pub struct NDIS_WMI_SET_HEADER {
    pub Header: NDIS_OBJECT_HEADER,
    pub PortNumber: u32,
    pub NetLuid: super::IpHelper::NET_LUID_LH,
    pub RequestId: u64,
    pub Timeout: u32,
    pub Padding: [u8; 4],
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl NDIS_WMI_SET_HEADER {}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::default::Default for NDIS_WMI_SET_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::cmp::PartialEq for NDIS_WMI_SET_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
impl ::std::cmp::Eq for NDIS_WMI_SET_HEADER {}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
unsafe impl ::windows::runtime::Abi for NDIS_WMI_SET_HEADER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WMI_SET_HEADER_REVISION_1: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_WMI_TCP_CONNECTION_OFFLOAD {
    pub Header: NDIS_OBJECT_HEADER,
    pub Encapsulation: u32,
    pub SupportIPv4: u32,
    pub SupportIPv6: u32,
    pub SupportIPv6ExtensionHeaders: u32,
    pub SupportSack: u32,
    pub TcpConnectionOffloadCapacity: u32,
    pub Flags: u32,
}
impl NDIS_WMI_TCP_CONNECTION_OFFLOAD {}
impl ::std::default::Default for NDIS_WMI_TCP_CONNECTION_OFFLOAD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_WMI_TCP_CONNECTION_OFFLOAD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_WMI_TCP_CONNECTION_OFFLOAD")
            .field("Header", &self.Header)
            .field("Encapsulation", &self.Encapsulation)
            .field("SupportIPv4", &self.SupportIPv4)
            .field("SupportIPv6", &self.SupportIPv6)
            .field("SupportIPv6ExtensionHeaders", &self.SupportIPv6ExtensionHeaders)
            .field("SupportSack", &self.SupportSack)
            .field("TcpConnectionOffloadCapacity", &self.TcpConnectionOffloadCapacity)
            .field("Flags", &self.Flags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_WMI_TCP_CONNECTION_OFFLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Encapsulation == other.Encapsulation && self.SupportIPv4 == other.SupportIPv4 && self.SupportIPv6 == other.SupportIPv6 && self.SupportIPv6ExtensionHeaders == other.SupportIPv6ExtensionHeaders && self.SupportSack == other.SupportSack && self.TcpConnectionOffloadCapacity == other.TcpConnectionOffloadCapacity && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for NDIS_WMI_TCP_CONNECTION_OFFLOAD {}
unsafe impl ::windows::runtime::Abi for NDIS_WMI_TCP_CONNECTION_OFFLOAD {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD {
    pub IPv4Transmit: NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_1,
    pub IPv4Receive: NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_0,
    pub IPv6Transmit: NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_3,
    pub IPv6Receive: NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_2,
}
impl NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD {}
impl ::std::default::Default for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD").field("IPv4Transmit", &self.IPv4Transmit).field("IPv4Receive", &self.IPv4Receive).field("IPv6Transmit", &self.IPv6Transmit).field("IPv6Receive", &self.IPv6Receive).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.IPv4Transmit == other.IPv4Transmit && self.IPv4Receive == other.IPv4Receive && self.IPv6Transmit == other.IPv6Transmit && self.IPv6Receive == other.IPv6Receive
    }
}
impl ::std::cmp::Eq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD {}
unsafe impl ::windows::runtime::Abi for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_0 {
    pub Encapsulation: u32,
    pub IpOptionsSupported: u32,
    pub TcpOptionsSupported: u32,
    pub TcpChecksum: u32,
    pub UdpChecksum: u32,
    pub IpChecksum: u32,
}
impl NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_0 {}
impl ::std::default::Default for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_IPv4Receive_e__Struct")
            .field("Encapsulation", &self.Encapsulation)
            .field("IpOptionsSupported", &self.IpOptionsSupported)
            .field("TcpOptionsSupported", &self.TcpOptionsSupported)
            .field("TcpChecksum", &self.TcpChecksum)
            .field("UdpChecksum", &self.UdpChecksum)
            .field("IpChecksum", &self.IpChecksum)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.IpOptionsSupported == other.IpOptionsSupported && self.TcpOptionsSupported == other.TcpOptionsSupported && self.TcpChecksum == other.TcpChecksum && self.UdpChecksum == other.UdpChecksum && self.IpChecksum == other.IpChecksum
    }
}
impl ::std::cmp::Eq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_0 {}
unsafe impl ::windows::runtime::Abi for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_1 {
    pub Encapsulation: u32,
    pub IpOptionsSupported: u32,
    pub TcpOptionsSupported: u32,
    pub TcpChecksum: u32,
    pub UdpChecksum: u32,
    pub IpChecksum: u32,
}
impl NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_1 {}
impl ::std::default::Default for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_IPv4Transmit_e__Struct")
            .field("Encapsulation", &self.Encapsulation)
            .field("IpOptionsSupported", &self.IpOptionsSupported)
            .field("TcpOptionsSupported", &self.TcpOptionsSupported)
            .field("TcpChecksum", &self.TcpChecksum)
            .field("UdpChecksum", &self.UdpChecksum)
            .field("IpChecksum", &self.IpChecksum)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.IpOptionsSupported == other.IpOptionsSupported && self.TcpOptionsSupported == other.TcpOptionsSupported && self.TcpChecksum == other.TcpChecksum && self.UdpChecksum == other.UdpChecksum && self.IpChecksum == other.IpChecksum
    }
}
impl ::std::cmp::Eq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_1 {}
unsafe impl ::windows::runtime::Abi for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_2 {
    pub Encapsulation: u32,
    pub IpExtensionHeadersSupported: u32,
    pub TcpOptionsSupported: u32,
    pub TcpChecksum: u32,
    pub UdpChecksum: u32,
}
impl NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_2 {}
impl ::std::default::Default for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_IPv6Receive_e__Struct")
            .field("Encapsulation", &self.Encapsulation)
            .field("IpExtensionHeadersSupported", &self.IpExtensionHeadersSupported)
            .field("TcpOptionsSupported", &self.TcpOptionsSupported)
            .field("TcpChecksum", &self.TcpChecksum)
            .field("UdpChecksum", &self.UdpChecksum)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.IpExtensionHeadersSupported == other.IpExtensionHeadersSupported && self.TcpOptionsSupported == other.TcpOptionsSupported && self.TcpChecksum == other.TcpChecksum && self.UdpChecksum == other.UdpChecksum
    }
}
impl ::std::cmp::Eq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_2 {}
unsafe impl ::windows::runtime::Abi for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_3 {
    pub Encapsulation: u32,
    pub IpExtensionHeadersSupported: u32,
    pub TcpOptionsSupported: u32,
    pub TcpChecksum: u32,
    pub UdpChecksum: u32,
}
impl NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_3 {}
impl ::std::default::Default for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_IPv6Transmit_e__Struct")
            .field("Encapsulation", &self.Encapsulation)
            .field("IpExtensionHeadersSupported", &self.IpExtensionHeadersSupported)
            .field("TcpOptionsSupported", &self.TcpOptionsSupported)
            .field("TcpChecksum", &self.TcpChecksum)
            .field("UdpChecksum", &self.UdpChecksum)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_3 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.IpExtensionHeadersSupported == other.IpExtensionHeadersSupported && self.TcpOptionsSupported == other.TcpOptionsSupported && self.TcpChecksum == other.TcpChecksum && self.UdpChecksum == other.UdpChecksum
    }
}
impl ::std::cmp::Eq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_3 {}
unsafe impl ::windows::runtime::Abi for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_3 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1 {
    pub IPv4: NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1_0,
}
impl NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1 {}
impl ::std::default::Default for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1").field("IPv4", &self.IPv4).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.IPv4 == other.IPv4
    }
}
impl ::std::cmp::Eq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1 {}
unsafe impl ::windows::runtime::Abi for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    pub Encapsulation: u32,
    pub MaxOffLoadSize: u32,
    pub MinSegmentCount: u32,
    pub TcpOptions: u32,
    pub IpOptions: u32,
}
impl NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1_0 {}
impl ::std::default::Default for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_IPv4_e__Struct").field("Encapsulation", &self.Encapsulation).field("MaxOffLoadSize", &self.MaxOffLoadSize).field("MinSegmentCount", &self.MinSegmentCount).field("TcpOptions", &self.TcpOptions).field("IpOptions", &self.IpOptions).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.MaxOffLoadSize == other.MaxOffLoadSize && self.MinSegmentCount == other.MinSegmentCount && self.TcpOptions == other.TcpOptions && self.IpOptions == other.IpOptions
    }
}
impl ::std::cmp::Eq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1_0 {}
unsafe impl ::windows::runtime::Abi for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2 {
    pub IPv4: NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_0,
    pub IPv6: NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_1,
}
impl NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2 {}
impl ::std::default::Default for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2").field("IPv4", &self.IPv4).field("IPv6", &self.IPv6).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.IPv4 == other.IPv4 && self.IPv6 == other.IPv6
    }
}
impl ::std::cmp::Eq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2 {}
unsafe impl ::windows::runtime::Abi for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    pub Encapsulation: u32,
    pub MaxOffLoadSize: u32,
    pub MinSegmentCount: u32,
}
impl NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_0 {}
impl ::std::default::Default for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_IPv4_e__Struct").field("Encapsulation", &self.Encapsulation).field("MaxOffLoadSize", &self.MaxOffLoadSize).field("MinSegmentCount", &self.MinSegmentCount).finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.MaxOffLoadSize == other.MaxOffLoadSize && self.MinSegmentCount == other.MinSegmentCount
    }
}
impl ::std::cmp::Eq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_0 {}
unsafe impl ::windows::runtime::Abi for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    pub Encapsulation: u32,
    pub MaxOffLoadSize: u32,
    pub MinSegmentCount: u32,
    pub IpExtensionHeadersSupported: u32,
    pub TcpOptionsSupported: u32,
}
impl NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_1 {}
impl ::std::default::Default for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_IPv6_e__Struct")
            .field("Encapsulation", &self.Encapsulation)
            .field("MaxOffLoadSize", &self.MaxOffLoadSize)
            .field("MinSegmentCount", &self.MinSegmentCount)
            .field("IpExtensionHeadersSupported", &self.IpExtensionHeadersSupported)
            .field("TcpOptionsSupported", &self.TcpOptionsSupported)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.MaxOffLoadSize == other.MaxOffLoadSize && self.MinSegmentCount == other.MinSegmentCount && self.IpExtensionHeadersSupported == other.IpExtensionHeadersSupported && self.TcpOptionsSupported == other.TcpOptionsSupported
    }
}
impl ::std::cmp::Eq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_1 {}
unsafe impl ::windows::runtime::Abi for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WWAN_WAKE_ON_PACKET_STATE_ENABLED: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WWAN_WAKE_ON_PACKET_STATE_SUPPORTED: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WWAN_WAKE_ON_REGISTER_STATE_ENABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WWAN_WAKE_ON_REGISTER_STATE_SUPPORTED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WWAN_WAKE_ON_SMS_RECEIVE_ENABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WWAN_WAKE_ON_SMS_RECEIVE_SUPPORTED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WWAN_WAKE_ON_UICC_CHANGE_ENABLED: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WWAN_WAKE_ON_UICC_CHANGE_SUPPORTED: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WWAN_WAKE_ON_USSD_RECEIVE_ENABLED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDIS_WWAN_WAKE_ON_USSD_RECEIVE_SUPPORTED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDK_ADAPTER_FLAG_CQ_INTERRUPT_MODERATION_SUPPORTED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDK_ADAPTER_FLAG_CQ_RESIZE_SUPPORTED: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDK_ADAPTER_FLAG_IN_ORDER_DMA_SUPPORTED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDK_ADAPTER_FLAG_LOOPBACK_CONNECTIONS_SUPPORTED: u32 = 65536u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDK_ADAPTER_FLAG_MULTI_ENGINE_SUPPORTED: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDK_ADAPTER_FLAG_RDMA_READ_LOCAL_INVALIDATE_SUPPORTED: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NDK_ADAPTER_FLAG_RDMA_READ_SINK_NOT_REQUIRED: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDK_ADAPTER_INFO {
    pub Version: NDK_VERSION,
    pub VendorId: u32,
    pub DeviceId: u32,
    pub MaxRegistrationSize: usize,
    pub MaxWindowSize: usize,
    pub FRMRPageCount: u32,
    pub MaxInitiatorRequestSge: u32,
    pub MaxReceiveRequestSge: u32,
    pub MaxReadRequestSge: u32,
    pub MaxTransferLength: u32,
    pub MaxInlineDataSize: u32,
    pub MaxInboundReadLimit: u32,
    pub MaxOutboundReadLimit: u32,
    pub MaxReceiveQueueDepth: u32,
    pub MaxInitiatorQueueDepth: u32,
    pub MaxSrqDepth: u32,
    pub MaxCqDepth: u32,
    pub LargeRequestThreshold: u32,
    pub MaxCallerData: u32,
    pub MaxCalleeData: u32,
    pub AdapterFlags: u32,
    pub RdmaTechnology: NDK_RDMA_TECHNOLOGY,
}
impl NDK_ADAPTER_INFO {}
impl ::std::default::Default for NDK_ADAPTER_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDK_ADAPTER_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDK_ADAPTER_INFO")
            .field("Version", &self.Version)
            .field("VendorId", &self.VendorId)
            .field("DeviceId", &self.DeviceId)
            .field("MaxRegistrationSize", &self.MaxRegistrationSize)
            .field("MaxWindowSize", &self.MaxWindowSize)
            .field("FRMRPageCount", &self.FRMRPageCount)
            .field("MaxInitiatorRequestSge", &self.MaxInitiatorRequestSge)
            .field("MaxReceiveRequestSge", &self.MaxReceiveRequestSge)
            .field("MaxReadRequestSge", &self.MaxReadRequestSge)
            .field("MaxTransferLength", &self.MaxTransferLength)
            .field("MaxInlineDataSize", &self.MaxInlineDataSize)
            .field("MaxInboundReadLimit", &self.MaxInboundReadLimit)
            .field("MaxOutboundReadLimit", &self.MaxOutboundReadLimit)
            .field("MaxReceiveQueueDepth", &self.MaxReceiveQueueDepth)
            .field("MaxInitiatorQueueDepth", &self.MaxInitiatorQueueDepth)
            .field("MaxSrqDepth", &self.MaxSrqDepth)
            .field("MaxCqDepth", &self.MaxCqDepth)
            .field("LargeRequestThreshold", &self.LargeRequestThreshold)
            .field("MaxCallerData", &self.MaxCallerData)
            .field("MaxCalleeData", &self.MaxCalleeData)
            .field("AdapterFlags", &self.AdapterFlags)
            .field("RdmaTechnology", &self.RdmaTechnology)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NDK_ADAPTER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.VendorId == other.VendorId
            && self.DeviceId == other.DeviceId
            && self.MaxRegistrationSize == other.MaxRegistrationSize
            && self.MaxWindowSize == other.MaxWindowSize
            && self.FRMRPageCount == other.FRMRPageCount
            && self.MaxInitiatorRequestSge == other.MaxInitiatorRequestSge
            && self.MaxReceiveRequestSge == other.MaxReceiveRequestSge
            && self.MaxReadRequestSge == other.MaxReadRequestSge
            && self.MaxTransferLength == other.MaxTransferLength
            && self.MaxInlineDataSize == other.MaxInlineDataSize
            && self.MaxInboundReadLimit == other.MaxInboundReadLimit
            && self.MaxOutboundReadLimit == other.MaxOutboundReadLimit
            && self.MaxReceiveQueueDepth == other.MaxReceiveQueueDepth
            && self.MaxInitiatorQueueDepth == other.MaxInitiatorQueueDepth
            && self.MaxSrqDepth == other.MaxSrqDepth
            && self.MaxCqDepth == other.MaxCqDepth
            && self.LargeRequestThreshold == other.LargeRequestThreshold
            && self.MaxCallerData == other.MaxCallerData
            && self.MaxCalleeData == other.MaxCalleeData
            && self.AdapterFlags == other.AdapterFlags
            && self.RdmaTechnology == other.RdmaTechnology
    }
}
impl ::std::cmp::Eq for NDK_ADAPTER_INFO {}
unsafe impl ::windows::runtime::Abi for NDK_ADAPTER_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NDK_RDMA_TECHNOLOGY(pub i32);
pub const NdkUndefined: NDK_RDMA_TECHNOLOGY = NDK_RDMA_TECHNOLOGY(0i32);
pub const NdkiWarp: NDK_RDMA_TECHNOLOGY = NDK_RDMA_TECHNOLOGY(1i32);
pub const NdkInfiniBand: NDK_RDMA_TECHNOLOGY = NDK_RDMA_TECHNOLOGY(2i32);
pub const NdkRoCE: NDK_RDMA_TECHNOLOGY = NDK_RDMA_TECHNOLOGY(3i32);
pub const NdkRoCEv2: NDK_RDMA_TECHNOLOGY = NDK_RDMA_TECHNOLOGY(4i32);
pub const NdkMaxTechnology: NDK_RDMA_TECHNOLOGY = NDK_RDMA_TECHNOLOGY(5i32);
impl ::std::convert::From<i32> for NDK_RDMA_TECHNOLOGY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NDK_RDMA_TECHNOLOGY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NDK_VERSION {
    pub Major: u16,
    pub Minor: u16,
}
impl NDK_VERSION {}
impl ::std::default::Default for NDK_VERSION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDK_VERSION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDK_VERSION").field("Major", &self.Major).field("Minor", &self.Minor).finish()
    }
}
impl ::std::cmp::PartialEq for NDK_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.Major == other.Major && self.Minor == other.Minor
    }
}
impl ::std::cmp::Eq for NDK_VERSION {}
unsafe impl ::windows::runtime::Abi for NDK_VERSION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NETWORK_ADDRESS {
    pub AddressLength: u16,
    pub AddressType: u16,
    pub Address: [u8; 1],
}
impl NETWORK_ADDRESS {}
impl ::std::default::Default for NETWORK_ADDRESS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NETWORK_ADDRESS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NETWORK_ADDRESS").field("AddressLength", &self.AddressLength).field("AddressType", &self.AddressType).field("Address", &self.Address).finish()
    }
}
impl ::std::cmp::PartialEq for NETWORK_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.AddressLength == other.AddressLength && self.AddressType == other.AddressType && self.Address == other.Address
    }
}
impl ::std::cmp::Eq for NETWORK_ADDRESS {}
unsafe impl ::windows::runtime::Abi for NETWORK_ADDRESS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NETWORK_ADDRESS_IP {
    pub sin_port: u16,
    pub IN_ADDR: u32,
    pub sin_zero: [u8; 8],
}
impl NETWORK_ADDRESS_IP {}
impl ::std::default::Default for NETWORK_ADDRESS_IP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NETWORK_ADDRESS_IP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NETWORK_ADDRESS_IP").field("sin_port", &self.sin_port).field("IN_ADDR", &self.IN_ADDR).field("sin_zero", &self.sin_zero).finish()
    }
}
impl ::std::cmp::PartialEq for NETWORK_ADDRESS_IP {
    fn eq(&self, other: &Self) -> bool {
        self.sin_port == other.sin_port && self.IN_ADDR == other.IN_ADDR && self.sin_zero == other.sin_zero
    }
}
impl ::std::cmp::Eq for NETWORK_ADDRESS_IP {}
unsafe impl ::windows::runtime::Abi for NETWORK_ADDRESS_IP {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NETWORK_ADDRESS_IP6 {
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: [u16; 8],
    pub sin6_scope_id: u32,
}
impl NETWORK_ADDRESS_IP6 {}
impl ::std::default::Default for NETWORK_ADDRESS_IP6 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NETWORK_ADDRESS_IP6 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NETWORK_ADDRESS_IP6").field("sin6_port", &self.sin6_port).field("sin6_flowinfo", &self.sin6_flowinfo).field("sin6_addr", &self.sin6_addr).field("sin6_scope_id", &self.sin6_scope_id).finish()
    }
}
impl ::std::cmp::PartialEq for NETWORK_ADDRESS_IP6 {
    fn eq(&self, other: &Self) -> bool {
        self.sin6_port == other.sin6_port && self.sin6_flowinfo == other.sin6_flowinfo && self.sin6_addr == other.sin6_addr && self.sin6_scope_id == other.sin6_scope_id
    }
}
impl ::std::cmp::Eq for NETWORK_ADDRESS_IP6 {}
unsafe impl ::windows::runtime::Abi for NETWORK_ADDRESS_IP6 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NETWORK_ADDRESS_IPX {
    pub NetworkAddress: u32,
    pub NodeAddress: [u8; 6],
    pub Socket: u16,
}
impl NETWORK_ADDRESS_IPX {}
impl ::std::default::Default for NETWORK_ADDRESS_IPX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NETWORK_ADDRESS_IPX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NETWORK_ADDRESS_IPX").field("NetworkAddress", &self.NetworkAddress).field("NodeAddress", &self.NodeAddress).field("Socket", &self.Socket).finish()
    }
}
impl ::std::cmp::PartialEq for NETWORK_ADDRESS_IPX {
    fn eq(&self, other: &Self) -> bool {
        self.NetworkAddress == other.NetworkAddress && self.NodeAddress == other.NodeAddress && self.Socket == other.Socket
    }
}
impl ::std::cmp::Eq for NETWORK_ADDRESS_IPX {}
unsafe impl ::windows::runtime::Abi for NETWORK_ADDRESS_IPX {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct NETWORK_ADDRESS_LIST {
    pub AddressCount: i32,
    pub AddressType: u16,
    pub Address: [NETWORK_ADDRESS; 1],
}
impl NETWORK_ADDRESS_LIST {}
impl ::std::default::Default for NETWORK_ADDRESS_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NETWORK_ADDRESS_LIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NETWORK_ADDRESS_LIST").field("AddressCount", &self.AddressCount).field("AddressType", &self.AddressType).field("Address", &self.Address).finish()
    }
}
impl ::std::cmp::PartialEq for NETWORK_ADDRESS_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.AddressCount == other.AddressCount && self.AddressType == other.AddressType && self.Address == other.Address
    }
}
impl ::std::cmp::Eq for NETWORK_ADDRESS_LIST {}
unsafe impl ::windows::runtime::Abi for NETWORK_ADDRESS_LIST {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NdisHashFunctionReserved1: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NdisHashFunctionReserved2: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NdisHashFunctionReserved3: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const NdisHashFunctionToeplitz: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct OFFLOAD_ALGO_INFO {
    pub algoIdentifier: u32,
    pub algoKeylen: u32,
    pub algoRounds: u32,
}
impl OFFLOAD_ALGO_INFO {}
impl ::std::default::Default for OFFLOAD_ALGO_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for OFFLOAD_ALGO_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OFFLOAD_ALGO_INFO").field("algoIdentifier", &self.algoIdentifier).field("algoKeylen", &self.algoKeylen).field("algoRounds", &self.algoRounds).finish()
    }
}
impl ::std::cmp::PartialEq for OFFLOAD_ALGO_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.algoIdentifier == other.algoIdentifier && self.algoKeylen == other.algoKeylen && self.algoRounds == other.algoRounds
    }
}
impl ::std::cmp::Eq for OFFLOAD_ALGO_INFO {}
unsafe impl ::windows::runtime::Abi for OFFLOAD_ALGO_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OFFLOAD_CONF_ALGO(pub i32);
pub const OFFLOAD_IPSEC_CONF_NONE: OFFLOAD_CONF_ALGO = OFFLOAD_CONF_ALGO(0i32);
pub const OFFLOAD_IPSEC_CONF_DES: OFFLOAD_CONF_ALGO = OFFLOAD_CONF_ALGO(1i32);
pub const OFFLOAD_IPSEC_CONF_RESERVED: OFFLOAD_CONF_ALGO = OFFLOAD_CONF_ALGO(2i32);
pub const OFFLOAD_IPSEC_CONF_3_DES: OFFLOAD_CONF_ALGO = OFFLOAD_CONF_ALGO(3i32);
pub const OFFLOAD_IPSEC_CONF_MAX: OFFLOAD_CONF_ALGO = OFFLOAD_CONF_ALGO(4i32);
impl ::std::convert::From<i32> for OFFLOAD_CONF_ALGO {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OFFLOAD_CONF_ALGO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OFFLOAD_INBOUND_SA: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OFFLOAD_INTEGRITY_ALGO(pub i32);
pub const OFFLOAD_IPSEC_INTEGRITY_NONE: OFFLOAD_INTEGRITY_ALGO = OFFLOAD_INTEGRITY_ALGO(0i32);
pub const OFFLOAD_IPSEC_INTEGRITY_MD5: OFFLOAD_INTEGRITY_ALGO = OFFLOAD_INTEGRITY_ALGO(1i32);
pub const OFFLOAD_IPSEC_INTEGRITY_SHA: OFFLOAD_INTEGRITY_ALGO = OFFLOAD_INTEGRITY_ALGO(2i32);
pub const OFFLOAD_IPSEC_INTEGRITY_MAX: OFFLOAD_INTEGRITY_ALGO = OFFLOAD_INTEGRITY_ALGO(3i32);
impl ::std::convert::From<i32> for OFFLOAD_INTEGRITY_ALGO {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OFFLOAD_INTEGRITY_ALGO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
pub struct OFFLOAD_IPSEC_ADD_SA {
    pub SrcAddr: u32,
    pub SrcMask: u32,
    pub DestAddr: u32,
    pub DestMask: u32,
    pub Protocol: u32,
    pub SrcPort: u16,
    pub DestPort: u16,
    pub SrcTunnelAddr: u32,
    pub DestTunnelAddr: u32,
    pub Flags: u16,
    pub NumSAs: i16,
    pub SecAssoc: [OFFLOAD_SECURITY_ASSOCIATION; 3],
    pub OffloadHandle: super::super::Foundation::HANDLE,
    pub KeyLen: u32,
    pub KeyMat: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl OFFLOAD_IPSEC_ADD_SA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OFFLOAD_IPSEC_ADD_SA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for OFFLOAD_IPSEC_ADD_SA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OFFLOAD_IPSEC_ADD_SA")
            .field("SrcAddr", &self.SrcAddr)
            .field("SrcMask", &self.SrcMask)
            .field("DestAddr", &self.DestAddr)
            .field("DestMask", &self.DestMask)
            .field("Protocol", &self.Protocol)
            .field("SrcPort", &self.SrcPort)
            .field("DestPort", &self.DestPort)
            .field("SrcTunnelAddr", &self.SrcTunnelAddr)
            .field("DestTunnelAddr", &self.DestTunnelAddr)
            .field("Flags", &self.Flags)
            .field("NumSAs", &self.NumSAs)
            .field("SecAssoc", &self.SecAssoc)
            .field("OffloadHandle", &self.OffloadHandle)
            .field("KeyLen", &self.KeyLen)
            .field("KeyMat", &self.KeyMat)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OFFLOAD_IPSEC_ADD_SA {
    fn eq(&self, other: &Self) -> bool {
        self.SrcAddr == other.SrcAddr
            && self.SrcMask == other.SrcMask
            && self.DestAddr == other.DestAddr
            && self.DestMask == other.DestMask
            && self.Protocol == other.Protocol
            && self.SrcPort == other.SrcPort
            && self.DestPort == other.DestPort
            && self.SrcTunnelAddr == other.SrcTunnelAddr
            && self.DestTunnelAddr == other.DestTunnelAddr
            && self.Flags == other.Flags
            && self.NumSAs == other.NumSAs
            && self.SecAssoc == other.SecAssoc
            && self.OffloadHandle == other.OffloadHandle
            && self.KeyLen == other.KeyLen
            && self.KeyMat == other.KeyMat
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OFFLOAD_IPSEC_ADD_SA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OFFLOAD_IPSEC_ADD_SA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
pub struct OFFLOAD_IPSEC_ADD_UDPESP_SA {
    pub SrcAddr: u32,
    pub SrcMask: u32,
    pub DstAddr: u32,
    pub DstMask: u32,
    pub Protocol: u32,
    pub SrcPort: u16,
    pub DstPort: u16,
    pub SrcTunnelAddr: u32,
    pub DstTunnelAddr: u32,
    pub Flags: u16,
    pub NumSAs: i16,
    pub SecAssoc: [OFFLOAD_SECURITY_ASSOCIATION; 3],
    pub OffloadHandle: super::super::Foundation::HANDLE,
    pub EncapTypeEntry: OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY,
    pub EncapTypeEntryOffldHandle: super::super::Foundation::HANDLE,
    pub KeyLen: u32,
    pub KeyMat: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl OFFLOAD_IPSEC_ADD_UDPESP_SA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OFFLOAD_IPSEC_ADD_UDPESP_SA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for OFFLOAD_IPSEC_ADD_UDPESP_SA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OFFLOAD_IPSEC_ADD_UDPESP_SA")
            .field("SrcAddr", &self.SrcAddr)
            .field("SrcMask", &self.SrcMask)
            .field("DstAddr", &self.DstAddr)
            .field("DstMask", &self.DstMask)
            .field("Protocol", &self.Protocol)
            .field("SrcPort", &self.SrcPort)
            .field("DstPort", &self.DstPort)
            .field("SrcTunnelAddr", &self.SrcTunnelAddr)
            .field("DstTunnelAddr", &self.DstTunnelAddr)
            .field("Flags", &self.Flags)
            .field("NumSAs", &self.NumSAs)
            .field("SecAssoc", &self.SecAssoc)
            .field("OffloadHandle", &self.OffloadHandle)
            .field("EncapTypeEntry", &self.EncapTypeEntry)
            .field("EncapTypeEntryOffldHandle", &self.EncapTypeEntryOffldHandle)
            .field("KeyLen", &self.KeyLen)
            .field("KeyMat", &self.KeyMat)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OFFLOAD_IPSEC_ADD_UDPESP_SA {
    fn eq(&self, other: &Self) -> bool {
        self.SrcAddr == other.SrcAddr
            && self.SrcMask == other.SrcMask
            && self.DstAddr == other.DstAddr
            && self.DstMask == other.DstMask
            && self.Protocol == other.Protocol
            && self.SrcPort == other.SrcPort
            && self.DstPort == other.DstPort
            && self.SrcTunnelAddr == other.SrcTunnelAddr
            && self.DstTunnelAddr == other.DstTunnelAddr
            && self.Flags == other.Flags
            && self.NumSAs == other.NumSAs
            && self.SecAssoc == other.SecAssoc
            && self.OffloadHandle == other.OffloadHandle
            && self.EncapTypeEntry == other.EncapTypeEntry
            && self.EncapTypeEntryOffldHandle == other.EncapTypeEntryOffldHandle
            && self.KeyLen == other.KeyLen
            && self.KeyMat == other.KeyMat
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OFFLOAD_IPSEC_ADD_UDPESP_SA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OFFLOAD_IPSEC_ADD_UDPESP_SA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
pub struct OFFLOAD_IPSEC_DELETE_SA {
    pub OffloadHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl OFFLOAD_IPSEC_DELETE_SA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OFFLOAD_IPSEC_DELETE_SA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for OFFLOAD_IPSEC_DELETE_SA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OFFLOAD_IPSEC_DELETE_SA").field("OffloadHandle", &self.OffloadHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OFFLOAD_IPSEC_DELETE_SA {
    fn eq(&self, other: &Self) -> bool {
        self.OffloadHandle == other.OffloadHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OFFLOAD_IPSEC_DELETE_SA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OFFLOAD_IPSEC_DELETE_SA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_Foundation`*"]
pub struct OFFLOAD_IPSEC_DELETE_UDPESP_SA {
    pub OffloadHandle: super::super::Foundation::HANDLE,
    pub EncapTypeEntryOffldHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl OFFLOAD_IPSEC_DELETE_UDPESP_SA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OFFLOAD_IPSEC_DELETE_UDPESP_SA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for OFFLOAD_IPSEC_DELETE_UDPESP_SA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OFFLOAD_IPSEC_DELETE_UDPESP_SA").field("OffloadHandle", &self.OffloadHandle).field("EncapTypeEntryOffldHandle", &self.EncapTypeEntryOffldHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OFFLOAD_IPSEC_DELETE_UDPESP_SA {
    fn eq(&self, other: &Self) -> bool {
        self.OffloadHandle == other.OffloadHandle && self.EncapTypeEntryOffldHandle == other.EncapTypeEntryOffldHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OFFLOAD_IPSEC_DELETE_UDPESP_SA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OFFLOAD_IPSEC_DELETE_UDPESP_SA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY {
    pub UdpEncapType: UDP_ENCAP_TYPE,
    pub DstEncapPort: u16,
}
impl OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY {}
impl ::std::default::Default for OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY").field("UdpEncapType", &self.UdpEncapType).field("DstEncapPort", &self.DstEncapPort).finish()
    }
}
impl ::std::cmp::PartialEq for OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.UdpEncapType == other.UdpEncapType && self.DstEncapPort == other.DstEncapPort
    }
}
impl ::std::cmp::Eq for OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY {}
unsafe impl ::windows::runtime::Abi for OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OFFLOAD_MAX_SAS: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OFFLOAD_OPERATION_E(pub i32);
pub const AUTHENTICATE: OFFLOAD_OPERATION_E = OFFLOAD_OPERATION_E(1i32);
pub const ENCRYPT: OFFLOAD_OPERATION_E = OFFLOAD_OPERATION_E(2i32);
impl ::std::convert::From<i32> for OFFLOAD_OPERATION_E {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OFFLOAD_OPERATION_E {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OFFLOAD_OUTBOUND_SA: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct OFFLOAD_SECURITY_ASSOCIATION {
    pub Operation: OFFLOAD_OPERATION_E,
    pub SPI: u32,
    pub IntegrityAlgo: OFFLOAD_ALGO_INFO,
    pub ConfAlgo: OFFLOAD_ALGO_INFO,
    pub Reserved: OFFLOAD_ALGO_INFO,
}
impl OFFLOAD_SECURITY_ASSOCIATION {}
impl ::std::default::Default for OFFLOAD_SECURITY_ASSOCIATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for OFFLOAD_SECURITY_ASSOCIATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OFFLOAD_SECURITY_ASSOCIATION").field("Operation", &self.Operation).field("SPI", &self.SPI).field("IntegrityAlgo", &self.IntegrityAlgo).field("ConfAlgo", &self.ConfAlgo).field("Reserved", &self.Reserved).finish()
    }
}
impl ::std::cmp::PartialEq for OFFLOAD_SECURITY_ASSOCIATION {
    fn eq(&self, other: &Self) -> bool {
        self.Operation == other.Operation && self.SPI == other.SPI && self.IntegrityAlgo == other.IntegrityAlgo && self.ConfAlgo == other.ConfAlgo && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for OFFLOAD_SECURITY_ASSOCIATION {}
unsafe impl ::windows::runtime::Abi for OFFLOAD_SECURITY_ASSOCIATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_1394_LOCAL_NODE_INFO: u32 = 201392385u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_1394_VC_INFO: u32 = 201392386u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_ADD_KEY: u32 = 218169629u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_ADD_WEP: u32 = 218169619u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_ASSOCIATION_INFORMATION: u32 = 218169631u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_AUTHENTICATION_MODE: u32 = 218169624u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_BSSID: u32 = 218169601u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_BSSID_LIST: u32 = 218169879u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_BSSID_LIST_SCAN: u32 = 218169626u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_CAPABILITY: u32 = 218169634u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_CONFIGURATION: u32 = 218169873u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_DESIRED_RATES: u32 = 218169872u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_DISASSOCIATE: u32 = 218169621u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_ENCRYPTION_STATUS: u32 = 218169627u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_FRAGMENTATION_THRESHOLD: u32 = 218169865u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_INFRASTRUCTURE_MODE: u32 = 218169608u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_MEDIA_STREAM_MODE: u32 = 218169633u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_NETWORK_TYPES_SUPPORTED: u32 = 218169859u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_NETWORK_TYPE_IN_USE: u32 = 218169860u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_NON_BCAST_SSID_LIST: u32 = 218169636u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_NUMBER_OF_ANTENNAS: u32 = 218169867u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_PMKID: u32 = 218169635u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_POWER_MODE: u32 = 218169878u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_PRIVACY_FILTER: u32 = 218169625u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_RADIO_STATUS: u32 = 218169637u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_RELOAD_DEFAULTS: u32 = 218169628u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_REMOVE_KEY: u32 = 218169630u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_REMOVE_WEP: u32 = 218169620u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_RSSI: u32 = 218169862u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_RSSI_TRIGGER: u32 = 218169863u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_RTS_THRESHOLD: u32 = 218169866u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_RX_ANTENNA_SELECTED: u32 = 218169868u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_SSID: u32 = 218169602u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_STATISTICS: u32 = 218235410u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_SUPPORTED_RATES: u32 = 218169870u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_TEST: u32 = 218169632u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_TX_ANTENNA_SELECTED: u32 = 218169869u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_TX_POWER_LEVEL: u32 = 218169861u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_11_WEP_STATUS: u32 = 218169627u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_3_ADD_MULTICAST_ADDRESS: u32 = 16843272u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_3_CURRENT_ADDRESS: u32 = 16843010u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_3_DELETE_MULTICAST_ADDRESS: u32 = 16843273u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_3_MAC_OPTIONS: u32 = 16843013u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_3_MAXIMUM_LIST_SIZE: u32 = 16843012u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_3_MULTICAST_LIST: u32 = 16843011u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_3_PERMANENT_ADDRESS: u32 = 16843009u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_3_RCV_ERROR_ALIGNMENT: u32 = 16908545u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_3_RCV_OVERRUN: u32 = 16908803u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_3_XMIT_DEFERRED: u32 = 16908801u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_3_XMIT_HEARTBEAT_FAILURE: u32 = 16908805u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_3_XMIT_LATE_COLLISIONS: u32 = 16908807u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_3_XMIT_MAX_COLLISIONS: u32 = 16908802u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_3_XMIT_MORE_COLLISIONS: u32 = 16908547u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_3_XMIT_ONE_COLLISION: u32 = 16908546u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_3_XMIT_TIMES_CRS_LOST: u32 = 16908806u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_3_XMIT_UNDERRUN: u32 = 16908804u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_5_ABORT_DELIMETERS: u32 = 33686019u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_5_AC_ERRORS: u32 = 33686018u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_5_BURST_ERRORS: u32 = 33686017u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_5_CURRENT_ADDRESS: u32 = 33620226u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_5_CURRENT_FUNCTIONAL: u32 = 33620227u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_5_CURRENT_GROUP: u32 = 33620228u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_5_CURRENT_RING_STATE: u32 = 33620231u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_5_CURRENT_RING_STATUS: u32 = 33620230u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_5_FRAME_COPIED_ERRORS: u32 = 33686020u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_5_FREQUENCY_ERRORS: u32 = 33686021u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_5_INTERNAL_ERRORS: u32 = 33686023u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_5_LAST_OPEN_STATUS: u32 = 33620229u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_5_LINE_ERRORS: u32 = 33685761u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_5_LOST_FRAMES: u32 = 33685762u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_5_PERMANENT_ADDRESS: u32 = 33620225u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_802_5_TOKEN_ERRORS: u32 = 33686022u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ARCNET_CURRENT_ADDRESS: u32 = 100729090u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ARCNET_PERMANENT_ADDRESS: u32 = 100729089u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ARCNET_RECONFIGURATIONS: u32 = 100794881u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_ACQUIRE_ACCESS_NET_RESOURCES: u32 = 134283779u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_ALIGNMENT_REQUIRED: u32 = 134283784u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_ASSIGNED_VPI: u32 = 134283778u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_CALL_ALERTING: u32 = 134283788u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_CALL_NOTIFY: u32 = 134283790u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_CALL_PROCEEDING: u32 = 134283787u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_CELLS_HEC_ERROR: u32 = 134349314u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_DIGITAL_BROADCAST_VPIVCI: u32 = 134283782u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_GET_NEAREST_FLOW: u32 = 134283783u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_HW_CURRENT_ADDRESS: u32 = 134283524u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_ILMI_VPIVCI: u32 = 134283781u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_LECS_ADDRESS: u32 = 134283785u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_MAX_AAL0_PACKET_SIZE: u32 = 134283528u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_MAX_AAL1_PACKET_SIZE: u32 = 134283529u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_MAX_AAL34_PACKET_SIZE: u32 = 134283530u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_MAX_AAL5_PACKET_SIZE: u32 = 134283531u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_MAX_ACTIVE_VCI_BITS: u32 = 134283526u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_MAX_ACTIVE_VCS: u32 = 134283525u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_MAX_ACTIVE_VPI_BITS: u32 = 134283527u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_MY_IP_NM_ADDRESS: u32 = 134283791u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_PARTY_ALERTING: u32 = 134283789u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_RCV_CELLS_DROPPED: u32 = 134349059u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_RCV_CELLS_OK: u32 = 134349057u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_RCV_INVALID_VPI_VCI: u32 = 134349313u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_RCV_REASSEMBLY_ERROR: u32 = 134349315u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_RELEASE_ACCESS_NET_RESOURCES: u32 = 134283780u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_SERVICE_ADDRESS: u32 = 134283786u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_SIGNALING_VPIVCI: u32 = 134283777u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_SUPPORTED_AAL_TYPES: u32 = 134283523u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_SUPPORTED_SERVICE_CATEGORY: u32 = 134283522u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_SUPPORTED_VC_RATES: u32 = 134283521u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_ATM_XMIT_CELLS_OK: u32 = 134349058u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_CO_ADDRESS_CHANGE: u32 = 4261412871u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_CO_ADD_ADDRESS: u32 = 4261412868u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_CO_ADD_PVC: u32 = 4261412865u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_CO_AF_CLOSE: u32 = 4261412874u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_CO_DELETE_ADDRESS: u32 = 4261412869u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_CO_DELETE_PVC: u32 = 4261412866u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_CO_GET_ADDRESSES: u32 = 4261412870u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_CO_GET_CALL_INFORMATION: u32 = 4261412867u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_CO_SIGNALING_DISABLED: u32 = 4261412873u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_CO_SIGNALING_ENABLED: u32 = 4261412872u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_CO_TAPI_ADDRESS_CAPS: u32 = 4261416963u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_CO_TAPI_CM_CAPS: u32 = 4261416961u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_CO_TAPI_DONT_REPORT_DIGITS: u32 = 4261416969u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_CO_TAPI_GET_CALL_DIAGNOSTICS: u32 = 4261416967u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_CO_TAPI_LINE_CAPS: u32 = 4261416962u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_CO_TAPI_REPORT_DIGITS: u32 = 4261416968u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_CO_TAPI_TRANSLATE_NDIS_CALLPARAMS: u32 = 4261416965u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_CO_TAPI_TRANSLATE_TAPI_CALLPARAMS: u32 = 4261416964u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_CO_TAPI_TRANSLATE_TAPI_SAP: u32 = 4261416966u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_ATTACHMENT_TYPE: u32 = 50462977u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_DOWNSTREAM_NODE_LONG: u32 = 50462979u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_FRAMES_LOST: u32 = 50462981u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_FRAME_ERRORS: u32 = 50462980u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_IF_ADMIN_STATUS: u32 = 50528894u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_IF_DESCR: u32 = 50528889u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_IF_IN_DISCARDS: u32 = 50528900u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_IF_IN_ERRORS: u32 = 50528901u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_IF_IN_NUCAST_PKTS: u32 = 50528899u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_IF_IN_OCTETS: u32 = 50528897u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_IF_IN_UCAST_PKTS: u32 = 50528898u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_IF_IN_UNKNOWN_PROTOS: u32 = 50528902u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_IF_LAST_CHANGE: u32 = 50528896u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_IF_MTU: u32 = 50528891u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_IF_OPER_STATUS: u32 = 50528895u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_IF_OUT_DISCARDS: u32 = 50528906u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_IF_OUT_ERRORS: u32 = 50528907u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_IF_OUT_NUCAST_PKTS: u32 = 50528905u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_IF_OUT_OCTETS: u32 = 50528903u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_IF_OUT_QLEN: u32 = 50528908u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_IF_OUT_UCAST_PKTS: u32 = 50528904u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_IF_PHYS_ADDRESS: u32 = 50528893u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_IF_SPECIFIC: u32 = 50528909u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_IF_SPEED: u32 = 50528892u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_IF_TYPE: u32 = 50528890u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_LCONNECTION_STATE: u32 = 50462985u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_LCT_FAILURES: u32 = 50462983u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_LEM_REJECTS: u32 = 50462984u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_LONG_CURRENT_ADDR: u32 = 50397442u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_LONG_MAX_LIST_SIZE: u32 = 50397444u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_LONG_MULTICAST_LIST: u32 = 50397443u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_LONG_PERMANENT_ADDR: u32 = 50397441u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_AVAILABLE_PATHS: u32 = 50528803u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_BRIDGE_FUNCTIONS: u32 = 50528800u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_COPIED_CT: u32 = 50528828u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_CURRENT_PATH: u32 = 50528804u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_DA_FLAG: u32 = 50528842u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_DOWNSTREAM_NBR: u32 = 50528806u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_DOWNSTREAM_PORT_TYPE: u32 = 50528811u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_DUP_ADDRESS_TEST: u32 = 50528809u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_ERROR_CT: u32 = 50528831u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_FRAME_CT: u32 = 50528827u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_FRAME_ERROR_FLAG: u32 = 50528844u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_FRAME_ERROR_RATIO: u32 = 50528838u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_FRAME_ERROR_THRESHOLD: u32 = 50528837u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_FRAME_STATUS_FUNCTIONS: u32 = 50528799u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_HARDWARE_PRESENT: u32 = 50528847u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_INDEX: u32 = 50528812u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_LATE_CT: u32 = 50528835u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_LONG_GRP_ADDRESS: u32 = 50528814u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_LOST_CT: u32 = 50528832u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_MA_UNITDATA_AVAILABLE: u32 = 50528846u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_MA_UNITDATA_ENABLE: u32 = 50528848u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_NOT_COPIED_CT: u32 = 50528834u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_NOT_COPIED_FLAG: u32 = 50528845u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_NOT_COPIED_RATIO: u32 = 50528840u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_NOT_COPIED_THRESHOLD: u32 = 50528839u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_OLD_DOWNSTREAM_NBR: u32 = 50528808u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_OLD_UPSTREAM_NBR: u32 = 50528807u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_REQUESTED_PATHS: u32 = 50528810u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_RING_OP_CT: u32 = 50528836u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_RMT_STATE: u32 = 50528841u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_SHORT_GRP_ADDRESS: u32 = 50528815u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_SMT_ADDRESS: u32 = 50528813u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_TOKEN_CT: u32 = 50528830u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_TRANSMIT_CT: u32 = 50528829u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_TVX_CAPABILITY: u32 = 50528802u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_TVX_EXPIRED_CT: u32 = 50528833u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_TVX_VALUE: u32 = 50528819u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_T_MAX: u32 = 50528818u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_T_MAX_CAPABILITY: u32 = 50528801u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_T_NEG: u32 = 50528817u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_T_PRI0: u32 = 50528820u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_T_PRI1: u32 = 50528821u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_T_PRI2: u32 = 50528822u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_T_PRI3: u32 = 50528823u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_T_PRI4: u32 = 50528824u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_T_PRI5: u32 = 50528825u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_T_PRI6: u32 = 50528826u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_T_REQ: u32 = 50528816u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_UNDA_FLAG: u32 = 50528843u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_MAC_UPSTREAM_NBR: u32 = 50528805u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PATH_CONFIGURATION: u32 = 50528854u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PATH_INDEX: u32 = 50528849u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PATH_MAX_T_REQ: u32 = 50528859u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PATH_RING_LATENCY: u32 = 50528850u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PATH_SBA_AVAILABLE: u32 = 50528856u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PATH_SBA_OVERHEAD: u32 = 50528853u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PATH_SBA_PAYLOAD: u32 = 50528852u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PATH_TRACE_STATUS: u32 = 50528851u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PATH_TVX_LOWER_BOUND: u32 = 50528857u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PATH_T_MAX_LOWER_BOUND: u32 = 50528858u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PATH_T_R_MODE: u32 = 50528855u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_ACTION: u32 = 50528888u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_AVAILABLE_PATHS: u32 = 50528867u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_BS_FLAG: u32 = 50528873u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_CONNECTION_CAPABILITIES: u32 = 50528870u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_CONNECTION_POLICIES: u32 = 50528862u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_CONNNECT_STATE: u32 = 50528882u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_CURRENT_PATH: u32 = 50528864u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_EB_ERROR_CT: u32 = 50528875u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_HARDWARE_PRESENT: u32 = 50528886u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_INDEX: u32 = 50528871u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_LCT_FAIL_CT: u32 = 50528876u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_LEM_CT: u32 = 50528879u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_LEM_REJECT_CT: u32 = 50528878u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_LER_ALARM: u32 = 50528881u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_LER_CUTOFF: u32 = 50528880u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_LER_ESTIMATE: u32 = 50528877u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_LER_FLAG: u32 = 50528885u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_MAC_INDICATED: u32 = 50528863u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_MAC_LOOP_TIME: u32 = 50528868u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_MAC_PLACEMENT: u32 = 50528866u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_MAINT_LS: u32 = 50528872u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_MY_TYPE: u32 = 50528860u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_NEIGHBOR_TYPE: u32 = 50528861u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_PCM_STATE: u32 = 50528883u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_PC_LS: u32 = 50528874u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_PC_WITHHOLD: u32 = 50528884u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_PMD_CLASS: u32 = 50528869u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_PORT_REQUESTED_PATHS: u32 = 50528865u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_RING_MGT_STATE: u32 = 50462982u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SHORT_CURRENT_ADDR: u32 = 50397446u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SHORT_MAX_LIST_SIZE: u32 = 50397448u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SHORT_MULTICAST_LIST: u32 = 50397447u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SHORT_PERMANENT_ADDR: u32 = 50397445u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_AVAILABLE_PATHS: u32 = 50528779u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_BYPASS_PRESENT: u32 = 50528788u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_CF_STATE: u32 = 50528790u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_CONFIG_CAPABILITIES: u32 = 50528780u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_CONFIG_POLICY: u32 = 50528781u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_CONNECTION_POLICY: u32 = 50528782u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_ECM_STATE: u32 = 50528789u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_HI_VERSION_ID: u32 = 50528771u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_HOLD_STATE: u32 = 50528791u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_LAST_SET_STATION_ID: u32 = 50528798u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_LO_VERSION_ID: u32 = 50528772u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_MAC_CT: u32 = 50528776u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_MAC_INDEXES: u32 = 50528787u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_MANUFACTURER_DATA: u32 = 50528773u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_MASTER_CT: u32 = 50528778u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_MIB_VERSION_ID: u32 = 50528775u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_MSG_TIME_STAMP: u32 = 50528795u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_NON_MASTER_CT: u32 = 50528777u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_OP_VERSION_ID: u32 = 50528770u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_PEER_WRAP_FLAG: u32 = 50528794u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_PORT_INDEXES: u32 = 50528786u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_REMOTE_DISCONNECT_FLAG: u32 = 50528792u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_SET_COUNT: u32 = 50528797u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_STATION_ACTION: u32 = 50528887u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_STATION_ID: u32 = 50528769u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_STATION_STATUS: u32 = 50528793u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_STAT_RPT_POLICY: u32 = 50528784u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_TRACE_MAX_EXPIRATION: u32 = 50528785u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_TRANSITION_TIME_STAMP: u32 = 50528796u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_T_NOTIFY: u32 = 50528783u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_SMT_USER_DATA: u32 = 50528774u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FDDI_UPSTREAM_NODE_LONG: u32 = 50462978u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FFP_ADAPTER_STATS: u32 = 4227990033u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FFP_CONTROL: u32 = 4227924498u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FFP_DATA: u32 = 4227924500u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FFP_DRIVER_STATS: u32 = 4227990032u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FFP_FLUSH: u32 = 4227924497u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FFP_PARAMS: u32 = 4227924499u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_FFP_SUPPORT: u32 = 4227924496u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_ADMIN_STATUS: u32 = 66184u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_ALIAS: u32 = 66185u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_BROADCAST_BYTES_RCV: u32 = 131595u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_BROADCAST_BYTES_XMIT: u32 = 131589u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_BROADCAST_FRAMES_RCV: u32 = 131596u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_BROADCAST_FRAMES_XMIT: u32 = 131590u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_BYTES_RCV: u32 = 131609u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_BYTES_XMIT: u32 = 131610u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_BYTES_RCV: u32 = 131591u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_BYTES_XMIT: u32 = 131585u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_BYTES_XMIT_OUTSTANDING: u32 = 131617u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_DEVICE_PROFILE: u32 = 131602u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_DRIVER_VERSION: u32 = 65808u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_GET_NETCARD_TIME: u32 = 131600u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_GET_TIME_CAPS: u32 = 131599u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_HARDWARE_STATUS: u32 = 65794u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_LINK_SPEED: u32 = 65799u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_MAC_OPTIONS: u32 = 65811u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_MEDIA_CONNECT_STATUS: u32 = 65812u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_MEDIA_IN_USE: u32 = 65796u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_MEDIA_SUPPORTED: u32 = 65795u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_MINIMUM_LINK_SPEED: u32 = 131360u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_NETCARD_LOAD: u32 = 131601u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_PROTOCOL_OPTIONS: u32 = 65810u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_RCV_CRC_ERROR: u32 = 131597u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_RCV_PDUS_ERROR: u32 = 131332u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_RCV_PDUS_NO_BUFFER: u32 = 131333u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_RCV_PDUS_OK: u32 = 131330u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_SUPPORTED_GUIDS: u32 = 65815u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_SUPPORTED_LIST: u32 = 65793u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_TRANSMIT_QUEUE_LENGTH: u32 = 131598u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_VENDOR_DESCRIPTION: u32 = 65805u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_VENDOR_DRIVER_VERSION: u32 = 65814u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_VENDOR_ID: u32 = 65804u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_XMIT_PDUS_ERROR: u32 = 131331u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CO_XMIT_PDUS_OK: u32 = 131329u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CURRENT_LOOKAHEAD: u32 = 65807u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_CURRENT_PACKET_FILTER: u32 = 65806u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_DEVICE_PROFILE: u32 = 131602u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_DIRECTED_BYTES_RCV: u32 = 131591u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_DIRECTED_BYTES_XMIT: u32 = 131585u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_DIRECTED_FRAMES_RCV: u32 = 131592u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_DIRECTED_FRAMES_XMIT: u32 = 131586u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_DISCONTINUITY_TIME: u32 = 66178u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_DRIVER_VERSION: u32 = 65808u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_ENUMERATE_PORTS: u32 = 66061u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_FRIENDLY_NAME: u32 = 131606u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_GET_NETCARD_TIME: u32 = 131600u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_GET_TIME_CAPS: u32 = 131599u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_HARDWARE_STATUS: u32 = 65794u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_HD_SPLIT_CURRENT_CONFIG: u32 = 66080u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_HD_SPLIT_PARAMETERS: u32 = 66078u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_INIT_TIME_MS: u32 = 131603u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_INTERFACE_INFO: u32 = 66183u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_INTERRUPT_MODERATION: u32 = 66057u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_IP_OPER_STATUS: u32 = 66189u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_ISOLATION_PARAMETERS: u32 = 66304u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_LAST_CHANGE: u32 = 66177u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_LINK_PARAMETERS: u32 = 66056u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_LINK_SPEED: u32 = 65799u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_LINK_SPEED_EX: u32 = 66187u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_LINK_STATE: u32 = 66055u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_MACHINE_NAME: u32 = 66074u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_MAC_ADDRESS: u32 = 66053u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_MAC_OPTIONS: u32 = 65811u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_MAXIMUM_FRAME_SIZE: u32 = 65798u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_MAXIMUM_LOOKAHEAD: u32 = 65797u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_MAXIMUM_SEND_PACKETS: u32 = 65813u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_MAXIMUM_TOTAL_SIZE: u32 = 65809u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_MAX_LINK_SPEED: u32 = 66054u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_MEDIA_CAPABILITIES: u32 = 66049u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_MEDIA_CONNECT_STATUS: u32 = 65812u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_MEDIA_CONNECT_STATUS_EX: u32 = 66186u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_MEDIA_DUPLEX_STATE: u32 = 66188u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_MEDIA_IN_USE: u32 = 65796u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_MEDIA_SENSE_COUNTS: u32 = 131605u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_MEDIA_SUPPORTED: u32 = 65795u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_MINIPORT_RESTART_ATTRIBUTES: u32 = 66077u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_MULTICAST_BYTES_RCV: u32 = 131593u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_MULTICAST_BYTES_XMIT: u32 = 131587u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_MULTICAST_FRAMES_RCV: u32 = 131594u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_MULTICAST_FRAMES_XMIT: u32 = 131588u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_NDIS_RESERVED_1: u32 = 131607u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_NDIS_RESERVED_2: u32 = 131608u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_NDIS_RESERVED_3: u32 = 66058u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_NDIS_RESERVED_4: u32 = 66059u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_NDIS_RESERVED_5: u32 = 66060u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_NDIS_RESERVED_6: u32 = 66066u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_NDIS_RESERVED_7: u32 = 131614u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_NETCARD_LOAD: u32 = 131601u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_NETWORK_LAYER_ADDRESSES: u32 = 65816u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_OPERATIONAL_STATUS: u32 = 66179u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_PCI_DEVICE_CUSTOM_PROPERTIES: u32 = 66065u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_PHYSICAL_MEDIUM: u32 = 66050u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_PHYSICAL_MEDIUM_EX: u32 = 66067u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_PORT_AUTHENTICATION_PARAMETERS: u32 = 66063u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_PORT_STATE: u32 = 66062u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_PROMISCUOUS_MODE: u32 = 66176u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_PROTOCOL_OPTIONS: u32 = 65810u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_RCV_CRC_ERROR: u32 = 131597u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_RCV_DISCARDS: u32 = 131611u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_RCV_ERROR: u32 = 131332u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_RCV_LINK_SPEED: u32 = 66181u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_RCV_NO_BUFFER: u32 = 131333u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_RCV_OK: u32 = 131330u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_RECEIVE_BLOCK_SIZE: u32 = 65803u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_RECEIVE_BUFFER_SPACE: u32 = 65801u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_RECEIVE_HASH: u32 = 66079u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_RECEIVE_SCALE_CAPABILITIES: u32 = 66051u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_RECEIVE_SCALE_PARAMETERS: u32 = 66052u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_RECEIVE_SCALE_PARAMETERS_V2: u32 = 66068u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_RESET_COUNTS: u32 = 131604u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_RNDIS_CONFIG_PARAMETER: u32 = 66075u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_RSS_SET_INDIRECTION_TABLE_ENTRIES: u32 = 66240u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_STATISTICS: u32 = 131334u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_SUPPORTED_GUIDS: u32 = 65815u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_SUPPORTED_LIST: u32 = 65793u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_TIMEOUT_DPC_REQUEST_CAPABILITIES: u32 = 66064u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_TRANSMIT_BLOCK_SIZE: u32 = 65802u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_TRANSMIT_BUFFER_SPACE: u32 = 65800u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_TRANSMIT_QUEUE_LENGTH: u32 = 131598u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_TRANSPORT_HEADER_OFFSET: u32 = 65817u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_UNKNOWN_PROTOS: u32 = 66182u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_VENDOR_DESCRIPTION: u32 = 65805u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_VENDOR_DRIVER_VERSION: u32 = 65814u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_VENDOR_ID: u32 = 65804u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_VLAN_ID: u32 = 66076u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_XMIT_DISCARDS: u32 = 131612u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_XMIT_ERROR: u32 = 131331u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_XMIT_LINK_SPEED: u32 = 66180u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GEN_XMIT_OK: u32 = 131329u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GFT_ACTIVATE_FLOW_ENTRIES: u32 = 66575u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GFT_ADD_FLOW_ENTRIES: u32 = 66572u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GFT_ALLOCATE_COUNTERS: u32 = 66567u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GFT_COUNTER_VALUES: u32 = 66570u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GFT_CREATE_LOGICAL_VPORT: u32 = 66584u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GFT_CREATE_TABLE: u32 = 66564u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GFT_CURRENT_CAPABILITIES: u32 = 66562u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GFT_DEACTIVATE_FLOW_ENTRIES: u32 = 66576u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GFT_DELETE_FLOW_ENTRIES: u32 = 66573u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GFT_DELETE_LOGICAL_VPORT: u32 = 66585u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GFT_DELETE_PROFILE: u32 = 66582u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GFT_DELETE_TABLE: u32 = 66565u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GFT_ENUM_COUNTERS: u32 = 66569u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GFT_ENUM_FLOW_ENTRIES: u32 = 66574u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GFT_ENUM_LOGICAL_VPORTS: u32 = 66586u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GFT_ENUM_PROFILES: u32 = 66581u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GFT_ENUM_TABLES: u32 = 66566u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GFT_EXACT_MATCH_PROFILE: u32 = 66578u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GFT_FLOW_ENTRY_PARAMETERS: u32 = 66577u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GFT_FREE_COUNTERS: u32 = 66568u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GFT_GLOBAL_PARAMETERS: u32 = 66563u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GFT_HARDWARE_CAPABILITIES: u32 = 66561u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GFT_HEADER_TRANSPOSITION_PROFILE: u32 = 66579u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GFT_STATISTICS: u32 = 66571u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GFT_VPORT_PARAMETERS: u32 = 66583u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_GFT_WILDCARD_MATCH_PROFILE: u32 = 66580u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_IP4_OFFLOAD_STATS: u32 = 4227924489u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_IP6_OFFLOAD_STATS: u32 = 4227924490u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_IRDA_EXTRA_RCV_BOFS: u32 = 167838208u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_IRDA_LINK_SPEED: u32 = 167837955u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_IRDA_MAX_RECEIVE_WINDOW_SIZE: u32 = 167838212u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_IRDA_MAX_SEND_WINDOW_SIZE: u32 = 167838213u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_IRDA_MAX_UNICAST_LIST_SIZE: u32 = 167838211u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_IRDA_MEDIA_BUSY: u32 = 167837956u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_IRDA_RATE_SNIFF: u32 = 167838209u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_IRDA_RECEIVING: u32 = 167837952u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_IRDA_RESERVED1: u32 = 167838218u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_IRDA_RESERVED2: u32 = 167838223u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_IRDA_SUPPORTED_SPEEDS: u32 = 167837954u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_IRDA_TURNAROUND_TIME: u32 = 167837953u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_IRDA_UNICAST_LIST: u32 = 167838210u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_KDNET_ADD_PF: u32 = 131619u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_KDNET_ENUMERATE_PFS: u32 = 131618u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_KDNET_QUERY_PF_INFORMATION: u32 = 131621u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_KDNET_REMOVE_PF: u32 = 131620u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_LTALK_COLLISIONS: u32 = 84017666u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_LTALK_CURRENT_NODE_ID: u32 = 83951874u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_LTALK_DEFERS: u32 = 84017667u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_LTALK_FCS_ERRORS: u32 = 84017670u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_LTALK_IN_BROADCASTS: u32 = 84017409u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_LTALK_IN_LENGTH_ERRORS: u32 = 84017410u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_LTALK_NO_DATA_ERRORS: u32 = 84017668u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_LTALK_OUT_NO_HANDLERS: u32 = 84017665u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_LTALK_RANDOM_CTS_ERRORS: u32 = 84017669u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_NDK_CONNECTIONS: u32 = 4228121091u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_NDK_LOCAL_ENDPOINTS: u32 = 4228121092u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_NDK_SET_STATE: u32 = 4228121089u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_NDK_STATISTICS: u32 = 4228121090u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_NIC_SWITCH_ALLOCATE_VF: u32 = 66117u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_NIC_SWITCH_CREATE_SWITCH: u32 = 66103u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_NIC_SWITCH_CREATE_VPORT: u32 = 66113u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_NIC_SWITCH_CURRENT_CAPABILITIES: u32 = 66095u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_NIC_SWITCH_DELETE_SWITCH: u32 = 66105u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_NIC_SWITCH_DELETE_VPORT: u32 = 66116u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_NIC_SWITCH_ENUM_SWITCHES: u32 = 66112u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_NIC_SWITCH_ENUM_VFS: u32 = 66120u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_NIC_SWITCH_ENUM_VPORTS: u32 = 66115u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_NIC_SWITCH_FREE_VF: u32 = 66118u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_NIC_SWITCH_HARDWARE_CAPABILITIES: u32 = 66094u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_NIC_SWITCH_PARAMETERS: u32 = 66104u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_NIC_SWITCH_VF_PARAMETERS: u32 = 66119u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_NIC_SWITCH_VPORT_PARAMETERS: u32 = 66114u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_OFFLOAD_ENCAPSULATION: u32 = 16843018u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_PACKET_COALESCING_FILTER_MATCH_COUNT: u32 = 66101u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_PD_CLOSE_PROVIDER: u32 = 66818u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_PD_OPEN_PROVIDER: u32 = 66817u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_PD_QUERY_CURRENT_CONFIG: u32 = 66819u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_PM_ADD_PROTOCOL_OFFLOAD: u32 = 4244701453u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_PM_ADD_WOL_PATTERN: u32 = 4244701450u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_PM_CURRENT_CAPABILITIES: u32 = 4244701447u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_PM_GET_PROTOCOL_OFFLOAD: u32 = 4244701454u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_PM_HARDWARE_CAPABILITIES: u32 = 4244701448u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_PM_PARAMETERS: u32 = 4244701449u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_PM_PROTOCOL_OFFLOAD_LIST: u32 = 4244701456u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_PM_REMOVE_PROTOCOL_OFFLOAD: u32 = 4244701455u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_PM_REMOVE_WOL_PATTERN: u32 = 4244701451u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_PM_RESERVED_1: u32 = 4244701457u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_PM_WOL_PATTERN_LIST: u32 = 4244701452u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_PNP_ADD_WAKE_UP_PATTERN: u32 = 4244701443u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_PNP_CAPABILITIES: u32 = 4244701440u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_PNP_ENABLE_WAKE_UP: u32 = 4244701446u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_PNP_QUERY_POWER: u32 = 4244701442u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_PNP_REMOVE_WAKE_UP_PATTERN: u32 = 4244701444u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_PNP_SET_POWER: u32 = 4244701441u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_PNP_WAKE_UP_ERROR: u32 = 4244767233u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_PNP_WAKE_UP_OK: u32 = 4244767232u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_PNP_WAKE_UP_PATTERN_LIST: u32 = 4244701445u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_CURRENT_CAPABILITIES: u32 = 4228186114u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_HARDWARE_CAPABILITIES: u32 = 4228186113u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_OFFLOAD_CREATE_SQ: u32 = 67075u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_OFFLOAD_CURRENT_CAPABILITIES: u32 = 67074u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_OFFLOAD_DELETE_SQ: u32 = 67076u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_OFFLOAD_ENUM_SQS: u32 = 67078u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_OFFLOAD_HARDWARE_CAPABILITIES: u32 = 67073u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_OFFLOAD_SQ_STATS: u32 = 67079u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_OFFLOAD_UPDATE_SQ: u32 = 67077u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_OPERATIONAL_PARAMETERS: u32 = 4228186116u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_PARAMETERS: u32 = 4228186115u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_REMOTE_PARAMETERS: u32 = 4228186117u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_RESERVED1: u32 = 4211147008u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_RESERVED10: u32 = 4211147017u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_RESERVED11: u32 = 4211147018u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_RESERVED12: u32 = 4211147019u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_RESERVED13: u32 = 4211147020u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_RESERVED14: u32 = 4211147021u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_RESERVED15: u32 = 4211147022u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_RESERVED16: u32 = 4211147023u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_RESERVED17: u32 = 4211147024u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_RESERVED18: u32 = 4211147025u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_RESERVED19: u32 = 4211147026u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_RESERVED2: u32 = 4211147009u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_RESERVED20: u32 = 4211147027u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_RESERVED3: u32 = 4211147010u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_RESERVED4: u32 = 4211147011u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_RESERVED5: u32 = 4211147012u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_RESERVED6: u32 = 4211147013u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_RESERVED7: u32 = 4211147014u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_RESERVED8: u32 = 4211147015u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_QOS_RESERVED9: u32 = 4211147016u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_RECEIVE_FILTER_ALLOCATE_QUEUE: u32 = 66083u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_RECEIVE_FILTER_CLEAR_FILTER: u32 = 66088u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_RECEIVE_FILTER_CURRENT_CAPABILITIES: u32 = 66093u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_RECEIVE_FILTER_ENUM_FILTERS: u32 = 66089u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_RECEIVE_FILTER_ENUM_QUEUES: u32 = 66085u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_RECEIVE_FILTER_FREE_QUEUE: u32 = 66084u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_RECEIVE_FILTER_GLOBAL_PARAMETERS: u32 = 66082u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_RECEIVE_FILTER_HARDWARE_CAPABILITIES: u32 = 66081u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_RECEIVE_FILTER_MOVE_FILTER: u32 = 66096u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_RECEIVE_FILTER_PARAMETERS: u32 = 66090u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_RECEIVE_FILTER_QUEUE_ALLOCATION_COMPLETE: u32 = 66091u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_RECEIVE_FILTER_QUEUE_PARAMETERS: u32 = 66086u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_RECEIVE_FILTER_SET_FILTER: u32 = 66087u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SRIOV_BAR_RESOURCES: u32 = 66137u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SRIOV_CONFIG_STATE: u32 = 66145u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SRIOV_CURRENT_CAPABILITIES: u32 = 66128u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SRIOV_HARDWARE_CAPABILITIES: u32 = 66121u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SRIOV_OVERLYING_ADAPTER_INFO: u32 = 66152u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SRIOV_PF_LUID: u32 = 66144u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SRIOV_PROBED_BARS: u32 = 66136u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SRIOV_READ_VF_CONFIG_BLOCK: u32 = 66131u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SRIOV_READ_VF_CONFIG_SPACE: u32 = 66129u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SRIOV_RESET_VF: u32 = 66133u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SRIOV_SET_VF_POWER_STATE: u32 = 66134u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SRIOV_VF_INVALIDATE_CONFIG_BLOCK: u32 = 66153u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SRIOV_VF_SERIAL_NUMBER: u32 = 66146u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SRIOV_VF_VENDOR_DEVICE_ID: u32 = 66135u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SRIOV_WRITE_VF_CONFIG_BLOCK: u32 = 66132u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SRIOV_WRITE_VF_CONFIG_SPACE: u32 = 66130u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_FEATURE_STATUS_QUERY: u32 = 66151u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_NIC_ARRAY: u32 = 66167u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_NIC_CONNECT: u32 = 66171u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_NIC_CREATE: u32 = 66170u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_NIC_DELETE: u32 = 66173u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_NIC_DIRECT_REQUEST: u32 = 66198u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_NIC_DISCONNECT: u32 = 66172u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_NIC_REQUEST: u32 = 66160u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_NIC_RESTORE: u32 = 66194u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_NIC_RESTORE_COMPLETE: u32 = 66195u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_NIC_RESUME: u32 = 66200u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_NIC_SAVE: u32 = 66192u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_NIC_SAVE_COMPLETE: u32 = 66193u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_NIC_SUSPEND: u32 = 66199u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_NIC_SUSPENDED_LM_SOURCE_FINISHED: u32 = 66202u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_NIC_SUSPENDED_LM_SOURCE_STARTED: u32 = 66201u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_NIC_UPDATED: u32 = 66196u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_PARAMETERS: u32 = 66165u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_PORT_ARRAY: u32 = 66166u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_PORT_CREATE: u32 = 66168u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_PORT_DELETE: u32 = 66169u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_PORT_FEATURE_STATUS_QUERY: u32 = 66174u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_PORT_PROPERTY_ADD: u32 = 66161u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_PORT_PROPERTY_DELETE: u32 = 66163u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_PORT_PROPERTY_ENUM: u32 = 66164u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_PORT_PROPERTY_UPDATE: u32 = 66162u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_PORT_TEARDOWN: u32 = 66175u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_PORT_UPDATED: u32 = 66197u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_PROPERTY_ADD: u32 = 66147u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_PROPERTY_DELETE: u32 = 66149u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_PROPERTY_ENUM: u32 = 66150u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_SWITCH_PROPERTY_UPDATE: u32 = 66148u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_ACCEPT: u32 = 117637377u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_ANSWER: u32 = 117637378u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_CLOSE: u32 = 117637379u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_CLOSE_CALL: u32 = 117637380u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_CONDITIONAL_MEDIA_DETECTION: u32 = 117637381u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_CONFIG_DIALOG: u32 = 117637382u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_DEV_SPECIFIC: u32 = 117637383u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_DIAL: u32 = 117637384u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_DROP: u32 = 117637385u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_GATHER_DIGITS: u32 = 117637411u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_GET_ADDRESS_CAPS: u32 = 117637386u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_GET_ADDRESS_ID: u32 = 117637387u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_GET_ADDRESS_STATUS: u32 = 117637388u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_GET_CALL_ADDRESS_ID: u32 = 117637389u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_GET_CALL_INFO: u32 = 117637390u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_GET_CALL_STATUS: u32 = 117637391u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_GET_DEV_CAPS: u32 = 117637392u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_GET_DEV_CONFIG: u32 = 117637393u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_GET_EXTENSION_ID: u32 = 117637394u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_GET_ID: u32 = 117637395u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_GET_LINE_DEV_STATUS: u32 = 117637396u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_MAKE_CALL: u32 = 117637397u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_MONITOR_DIGITS: u32 = 117637412u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_NEGOTIATE_EXT_VERSION: u32 = 117637398u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_OPEN: u32 = 117637399u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_PROVIDER_INITIALIZE: u32 = 117637400u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_PROVIDER_SHUTDOWN: u32 = 117637401u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_SECURE_CALL: u32 = 117637402u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_SELECT_EXT_VERSION: u32 = 117637403u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_SEND_USER_USER_INFO: u32 = 117637404u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_SET_APP_SPECIFIC: u32 = 117637405u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_SET_CALL_PARAMS: u32 = 117637406u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_SET_DEFAULT_MEDIA_DETECTION: u32 = 117637407u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_SET_DEV_CONFIG: u32 = 117637408u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_SET_MEDIA_MODE: u32 = 117637409u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TAPI_SET_STATUS_MESSAGES: u32 = 117637410u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TCP4_OFFLOAD_STATS: u32 = 4227924487u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TCP6_OFFLOAD_STATS: u32 = 4227924488u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TCP_CONNECTION_OFFLOAD_CURRENT_CONFIG: u32 = 4227924494u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TCP_CONNECTION_OFFLOAD_HARDWARE_CAPABILITIES: u32 = 4227924495u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TCP_CONNECTION_OFFLOAD_PARAMETERS: u32 = 4228055553u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TCP_OFFLOAD_CURRENT_CONFIG: u32 = 4227924491u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TCP_OFFLOAD_HARDWARE_CAPABILITIES: u32 = 4227924493u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TCP_OFFLOAD_PARAMETERS: u32 = 4227924492u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TCP_RSC_STATISTICS: u32 = 131613u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TCP_SAN_SUPPORT: u32 = 4227924484u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TCP_TASK_IPSEC_ADD_SA: u32 = 4227924482u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TCP_TASK_IPSEC_ADD_UDPESP_SA: u32 = 4227924485u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TCP_TASK_IPSEC_DELETE_SA: u32 = 4227924483u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TCP_TASK_IPSEC_DELETE_UDPESP_SA: u32 = 4227924486u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TCP_TASK_IPSEC_OFFLOAD_V2_ADD_SA: u32 = 4228055554u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TCP_TASK_IPSEC_OFFLOAD_V2_ADD_SA_EX: u32 = 4228055557u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TCP_TASK_IPSEC_OFFLOAD_V2_DELETE_SA: u32 = 4228055555u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TCP_TASK_IPSEC_OFFLOAD_V2_UPDATE_SA: u32 = 4228055556u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TCP_TASK_OFFLOAD: u32 = 4227924481u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TIMESTAMP_CAPABILITY: u32 = 10485761u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TIMESTAMP_CURRENT_CONFIG: u32 = 10485762u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TIMESTAMP_GET_CROSSTIMESTAMP: u32 = 10485763u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TUNNEL_INTERFACE_RELEASE_OID: u32 = 251724039u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_TUNNEL_INTERFACE_SET_OID: u32 = 251724038u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_VLAN_RESERVED1: u32 = 66097u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_VLAN_RESERVED2: u32 = 66098u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_VLAN_RESERVED3: u32 = 66099u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_VLAN_RESERVED4: u32 = 66100u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WAN_CO_GET_COMP_INFO: u32 = 67175040u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WAN_CO_GET_INFO: u32 = 67174784u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WAN_CO_GET_LINK_INFO: u32 = 67174786u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WAN_CO_GET_STATS_INFO: u32 = 67175042u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WAN_CO_SET_COMP_INFO: u32 = 67175041u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WAN_CO_SET_LINK_INFO: u32 = 67174785u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WAN_CURRENT_ADDRESS: u32 = 67174658u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WAN_GET_BRIDGE_INFO: u32 = 67174922u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WAN_GET_COMP_INFO: u32 = 67174924u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WAN_GET_INFO: u32 = 67174663u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WAN_GET_LINK_INFO: u32 = 67174665u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WAN_GET_STATS_INFO: u32 = 67174926u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WAN_HEADER_FORMAT: u32 = 67174662u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WAN_LINE_COUNT: u32 = 67174666u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WAN_MEDIUM_SUBTYPE: u32 = 67174661u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WAN_PERMANENT_ADDRESS: u32 = 67174657u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WAN_PROTOCOL_CAPS: u32 = 67174667u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WAN_PROTOCOL_TYPE: u32 = 67174660u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WAN_QUALITY_OF_SERVICE: u32 = 67174659u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WAN_SET_BRIDGE_INFO: u32 = 67174923u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WAN_SET_COMP_INFO: u32 = 67174925u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WAN_SET_LINK_INFO: u32 = 67174664u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_AUTH_CHALLENGE: u32 = 234946837u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_BASE_STATIONS_INFO: u32 = 234946888u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_CONNECT: u32 = 234946828u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_CREATE_MAC: u32 = 234946854u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_DELETE_MAC: u32 = 234946855u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_DEVICE_BINDINGS: u32 = 234946865u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_DEVICE_CAPS: u32 = 234946817u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_DEVICE_CAPS_EX: u32 = 234946862u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_DEVICE_RESET: u32 = 234946887u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_DEVICE_SERVICE_COMMAND: u32 = 234946840u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_DEVICE_SERVICE_SESSION: u32 = 234946851u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_DEVICE_SERVICE_SESSION_WRITE: u32 = 234946852u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_DRIVER_CAPS: u32 = 234946816u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_ENUMERATE_DEVICE_SERVICES: u32 = 234946838u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_ENUMERATE_DEVICE_SERVICE_COMMANDS: u32 = 234946850u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_HOME_PROVIDER: u32 = 234946822u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_IMS_VOICE_STATE: u32 = 234946867u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_LOCATION_STATE: u32 = 234946869u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_LTE_ATTACH_CONFIG: u32 = 234946882u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_LTE_ATTACH_STATUS: u32 = 234946883u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_MBIM_VERSION: u32 = 234946860u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_MODEM_CONFIG_INFO: u32 = 234946884u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_MODEM_LOGGING_CONFIG: u32 = 234946891u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_MPDP: u32 = 234946889u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_NETWORK_BLACKLIST: u32 = 234946881u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_NETWORK_IDLE_HINT: u32 = 234946871u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_NETWORK_PARAMS: u32 = 234946893u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_NITZ: u32 = 234946870u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_PACKET_SERVICE: u32 = 234946826u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_PCO: u32 = 234946885u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_PIN: u32 = 234946820u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_PIN_EX: u32 = 234946849u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_PIN_EX2: u32 = 234946859u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_PIN_LIST: u32 = 234946821u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_PREFERRED_MULTICARRIER_PROVIDERS: u32 = 234946853u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_PREFERRED_PROVIDERS: u32 = 234946823u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_PRESHUTDOWN: u32 = 234946872u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_PROVISIONED_CONTEXTS: u32 = 234946829u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_PS_MEDIA_CONFIG: u32 = 234946878u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_RADIO_STATE: u32 = 234946819u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_READY_INFO: u32 = 234946818u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_REGISTER_PARAMS: u32 = 234946892u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_REGISTER_STATE: u32 = 234946825u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_REGISTER_STATE_EX: u32 = 234946866u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_SAR_CONFIG: u32 = 234946879u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_SAR_TRANSMISSION_STATUS: u32 = 234946880u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_SERVICE_ACTIVATION: u32 = 234946830u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_SIGNAL_STATE: u32 = 234946827u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_SIGNAL_STATE_EX: u32 = 234946868u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_SLOT_INFO_STATUS: u32 = 234946864u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_SMS_CONFIGURATION: u32 = 234946831u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_SMS_DELETE: u32 = 234946834u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_SMS_READ: u32 = 234946832u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_SMS_SEND: u32 = 234946833u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_SMS_STATUS: u32 = 234946835u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_SUBSCRIBE_DEVICE_SERVICE_EVENTS: u32 = 234946839u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_SYS_CAPS: u32 = 234946861u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_SYS_SLOTMAPPINGS: u32 = 234946863u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_UICC_ACCESS_BINARY: u32 = 234946857u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_UICC_ACCESS_RECORD: u32 = 234946858u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_UICC_APDU: u32 = 234946876u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_UICC_APP_LIST: u32 = 234946890u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_UICC_ATR: u32 = 234946873u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_UICC_CLOSE_CHANNEL: u32 = 234946875u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_UICC_FILE_STATUS: u32 = 234946856u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_UICC_OPEN_CHANNEL: u32 = 234946874u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_UICC_RESET: u32 = 234946886u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_UICC_TERMINAL_CAPABILITY: u32 = 234946877u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_USSD: u32 = 234946841u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_VENDOR_SPECIFIC: u32 = 234946836u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_WWAN_VISIBLE_PROVIDERS: u32 = 234946824u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const OID_XBOX_ACC_RESERVED0: u32 = 4194304000u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct PMKID_CANDIDATE {
    pub BSSID: [u8; 6],
    pub Flags: u32,
}
impl PMKID_CANDIDATE {}
impl ::std::default::Default for PMKID_CANDIDATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PMKID_CANDIDATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PMKID_CANDIDATE").field("BSSID", &self.BSSID).field("Flags", &self.Flags).finish()
    }
}
impl ::std::cmp::PartialEq for PMKID_CANDIDATE {
    fn eq(&self, other: &Self) -> bool {
        self.BSSID == other.BSSID && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for PMKID_CANDIDATE {}
unsafe impl ::windows::runtime::Abi for PMKID_CANDIDATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const READABLE_LOCAL_CLOCK: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const RECEIVE_TIME_INDICATION_CAPABLE: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const TIMED_SEND_CAPABLE: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const TIME_STAMP_CAPABLE: u32 = 32u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub struct TRANSPORT_HEADER_OFFSET {
    pub ProtocolType: u16,
    pub HeaderOffset: u16,
}
impl TRANSPORT_HEADER_OFFSET {}
impl ::std::default::Default for TRANSPORT_HEADER_OFFSET {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TRANSPORT_HEADER_OFFSET {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TRANSPORT_HEADER_OFFSET").field("ProtocolType", &self.ProtocolType).field("HeaderOffset", &self.HeaderOffset).finish()
    }
}
impl ::std::cmp::PartialEq for TRANSPORT_HEADER_OFFSET {
    fn eq(&self, other: &Self) -> bool {
        self.ProtocolType == other.ProtocolType && self.HeaderOffset == other.HeaderOffset
    }
}
impl ::std::cmp::Eq for TRANSPORT_HEADER_OFFSET {}
unsafe impl ::windows::runtime::Abi for TRANSPORT_HEADER_OFFSET {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UDP_ENCAP_TYPE(pub i32);
pub const OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_IKE: UDP_ENCAP_TYPE = UDP_ENCAP_TYPE(0i32);
pub const OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_OTHER: UDP_ENCAP_TYPE = UDP_ENCAP_TYPE(1i32);
impl ::std::convert::From<i32> for UDP_ENCAP_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UDP_ENCAP_TYPE {
    type Abi = Self;
}
pub const UNSPECIFIED_NETWORK_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(314203102, 5182, 19469, [182, 109, 35, 121, 187, 20, 25, 19]);
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const WAN_PROTOCOL_KEEPS_STATS: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_WiFi")]
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`, `Win32_NetworkManagement_WiFi`*"]
pub struct WDIAG_IHV_WLAN_ID {
    pub strProfileName: [u16; 256],
    pub Ssid: super::WiFi::DOT11_SSID,
    pub BssType: super::WiFi::DOT11_BSS_TYPE,
    pub dwFlags: u32,
    pub dwReasonCode: u32,
}
#[cfg(feature = "Win32_NetworkManagement_WiFi")]
impl WDIAG_IHV_WLAN_ID {}
#[cfg(feature = "Win32_NetworkManagement_WiFi")]
impl ::std::default::Default for WDIAG_IHV_WLAN_ID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_WiFi")]
impl ::std::fmt::Debug for WDIAG_IHV_WLAN_ID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WDIAG_IHV_WLAN_ID").field("strProfileName", &self.strProfileName).field("Ssid", &self.Ssid).field("BssType", &self.BssType).field("dwFlags", &self.dwFlags).field("dwReasonCode", &self.dwReasonCode).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_WiFi")]
impl ::std::cmp::PartialEq for WDIAG_IHV_WLAN_ID {
    fn eq(&self, other: &Self) -> bool {
        self.strProfileName == other.strProfileName && self.Ssid == other.Ssid && self.BssType == other.BssType && self.dwFlags == other.dwFlags && self.dwReasonCode == other.dwReasonCode
    }
}
#[cfg(feature = "Win32_NetworkManagement_WiFi")]
impl ::std::cmp::Eq for WDIAG_IHV_WLAN_ID {}
#[cfg(feature = "Win32_NetworkManagement_WiFi")]
unsafe impl ::windows::runtime::Abi for WDIAG_IHV_WLAN_ID {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_Ndis`*"]
pub const WDIAG_IHV_WLAN_ID_FLAG_SECURITY_ENABLED: u32 = 1u32;
