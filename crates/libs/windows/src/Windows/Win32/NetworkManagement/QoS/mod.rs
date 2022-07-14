#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ABLE_TO_RECV_RSVP: u32 = 50002u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct ADDRESS_LIST_DESCRIPTOR {
    pub MediaType: u32,
    pub AddressList: super::Ndis::NETWORK_ADDRESS_LIST,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for ADDRESS_LIST_DESCRIPTOR {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for ADDRESS_LIST_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for ADDRESS_LIST_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADDRESS_LIST_DESCRIPTOR").field("MediaType", &self.MediaType).field("AddressList", &self.AddressList).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for ADDRESS_LIST_DESCRIPTOR {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for ADDRESS_LIST_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ADDRESS_LIST_DESCRIPTOR>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for ADDRESS_LIST_DESCRIPTOR {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for ADDRESS_LIST_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ADM_CTRL_FAILED: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct ADSPEC {
    pub adspec_header: RsvpObjHdr,
    pub adspec_body: IS_ADSPEC_BODY,
}
impl ::core::marker::Copy for ADSPEC {}
impl ::core::clone::Clone for ADSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ADSPEC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADSPEC").field("adspec_header", &self.adspec_header).field("adspec_body", &self.adspec_body).finish()
    }
}
unsafe impl ::windows::core::Abi for ADSPEC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ADSPEC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ADSPEC>()) == 0 }
    }
}
impl ::core::cmp::Eq for ADSPEC {}
impl ::core::default::Default for ADSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const AD_FLAG_BREAK_BIT: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct AD_GENERAL_PARAMS {
    pub IntServAwareHopCount: u32,
    pub PathBandwidthEstimate: u32,
    pub MinimumLatency: u32,
    pub PathMTU: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for AD_GENERAL_PARAMS {}
impl ::core::clone::Clone for AD_GENERAL_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AD_GENERAL_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AD_GENERAL_PARAMS").field("IntServAwareHopCount", &self.IntServAwareHopCount).field("PathBandwidthEstimate", &self.PathBandwidthEstimate).field("MinimumLatency", &self.MinimumLatency).field("PathMTU", &self.PathMTU).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for AD_GENERAL_PARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AD_GENERAL_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AD_GENERAL_PARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for AD_GENERAL_PARAMS {}
impl ::core::default::Default for AD_GENERAL_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct AD_GUARANTEED {
    pub CTotal: u32,
    pub DTotal: u32,
    pub CSum: u32,
    pub DSum: u32,
}
impl ::core::marker::Copy for AD_GUARANTEED {}
impl ::core::clone::Clone for AD_GUARANTEED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for AD_GUARANTEED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AD_GUARANTEED").field("CTotal", &self.CTotal).field("DTotal", &self.DTotal).field("CSum", &self.CSum).field("DSum", &self.DSum).finish()
    }
}
unsafe impl ::windows::core::Abi for AD_GUARANTEED {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AD_GUARANTEED {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AD_GUARANTEED>()) == 0 }
    }
}
impl ::core::cmp::Eq for AD_GUARANTEED {}
impl ::core::default::Default for AD_GUARANTEED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ALLOWED_TO_SEND_DATA: u32 = 50001u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ANY_DEST_ADDR: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub type CBADMITRESULT = ::core::option::Option<unsafe extern "system" fn(lpmhandle: LPM_HANDLE, requesthandle: RHANDLE, ulpcmactionflags: u32, lpmerror: i32, policydecisionscount: i32, ppolicydecisions: *mut policy_decision) -> *mut u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub type CBGETRSVPOBJECTS = ::core::option::Option<unsafe extern "system" fn(lpmhandle: LPM_HANDLE, requesthandle: RHANDLE, lpmerror: i32, rsvpobjectscount: i32, pprsvpobjects: *mut *mut RsvpObjHdr) -> *mut u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const CONTROLLED_DELAY_SERV: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const CONTROLLED_LOAD_SERV: u32 = 5u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct CONTROL_SERVICE {
    pub Length: u32,
    pub Service: u32,
    pub Overrides: AD_GENERAL_PARAMS,
    pub Anonymous: CONTROL_SERVICE_0,
}
impl ::core::marker::Copy for CONTROL_SERVICE {}
impl ::core::clone::Clone for CONTROL_SERVICE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CONTROL_SERVICE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONTROL_SERVICE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONTROL_SERVICE>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONTROL_SERVICE {}
impl ::core::default::Default for CONTROL_SERVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub union CONTROL_SERVICE_0 {
    pub Guaranteed: AD_GUARANTEED,
    pub ParamBuffer: [PARAM_BUFFER; 1],
}
impl ::core::marker::Copy for CONTROL_SERVICE_0 {}
impl ::core::clone::Clone for CONTROL_SERVICE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CONTROL_SERVICE_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CONTROL_SERVICE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONTROL_SERVICE_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for CONTROL_SERVICE_0 {}
impl ::core::default::Default for CONTROL_SERVICE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const CREDENTIAL_SUB_TYPE_ASCII_ID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const CREDENTIAL_SUB_TYPE_KERBEROS_TKT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const CREDENTIAL_SUB_TYPE_PGP_CERT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const CREDENTIAL_SUB_TYPE_UNICODE_ID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const CREDENTIAL_SUB_TYPE_X509_V3_CERT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const CURRENT_TCI_VERSION: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct CtrlLoadFlowspec {
    pub CL_spec_serv_hdr: IntServServiceHdr,
    pub CL_spec_parm_hdr: IntServParmHdr,
    pub CL_spec_parms: GenTspecParms,
}
impl ::core::marker::Copy for CtrlLoadFlowspec {}
impl ::core::clone::Clone for CtrlLoadFlowspec {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CtrlLoadFlowspec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CtrlLoadFlowspec").field("CL_spec_serv_hdr", &self.CL_spec_serv_hdr).field("CL_spec_parm_hdr", &self.CL_spec_parm_hdr).field("CL_spec_parms", &self.CL_spec_parms).finish()
    }
}
unsafe impl ::windows::core::Abi for CtrlLoadFlowspec {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CtrlLoadFlowspec {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CtrlLoadFlowspec>()) == 0 }
    }
}
impl ::core::cmp::Eq for CtrlLoadFlowspec {}
impl ::core::default::Default for CtrlLoadFlowspec {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const DD_TCP_DEVICE_NAME: &str = "\\Device\\Tcp";
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const DUP_RESULTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const END_TO_END_QOSABILITY: u32 = 50006u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct ENUMERATION_BUFFER {
    pub Length: u32,
    pub OwnerProcessId: u32,
    pub FlowNameLength: u16,
    pub FlowName: [u16; 256],
    pub pFlow: *mut TC_GEN_FLOW,
    pub NumberOfFilters: u32,
    pub GenericFilter: [TC_GEN_FILTER; 1],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for ENUMERATION_BUFFER {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for ENUMERATION_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for ENUMERATION_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMERATION_BUFFER").field("Length", &self.Length).field("OwnerProcessId", &self.OwnerProcessId).field("FlowNameLength", &self.FlowNameLength).field("FlowName", &self.FlowName).field("pFlow", &self.pFlow).field("NumberOfFilters", &self.NumberOfFilters).field("GenericFilter", &self.GenericFilter).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for ENUMERATION_BUFFER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for ENUMERATION_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENUMERATION_BUFFER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for ENUMERATION_BUFFER {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for ENUMERATION_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERROR_ADDRESS_TYPE_NOT_SUPPORTED: u32 = 7511u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERROR_DS_MAPPING_EXISTS: u32 = 7518u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERROR_DUPLICATE_FILTER: u32 = 7509u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERROR_FILTER_CONFLICT: u32 = 7510u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERROR_INCOMPATABLE_QOS: u32 = 7513u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERROR_INCOMPATIBLE_TCI_VERSION: u32 = 7501u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERROR_INVALID_ADDRESS_TYPE: u32 = 7508u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERROR_INVALID_DIFFSERV_FLOW: u32 = 7517u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERROR_INVALID_DS_CLASS: u32 = 7520u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERROR_INVALID_FLOW_MODE: u32 = 7516u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERROR_INVALID_PEAK_RATE: u32 = 7504u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERROR_INVALID_QOS_PRIORITY: u32 = 7506u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERROR_INVALID_SD_MODE: u32 = 7505u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERROR_INVALID_SERVICE_TYPE: u32 = 7502u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERROR_INVALID_SHAPE_RATE: u32 = 7519u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERROR_INVALID_TOKEN_RATE: u32 = 7503u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERROR_INVALID_TRAFFIC_CLASS: u32 = 7507u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERROR_NO_MORE_INFO: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct ERROR_SPEC {
    pub errs_header: RsvpObjHdr,
    pub errs_u: ERROR_SPEC_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for ERROR_SPEC {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for ERROR_SPEC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for ERROR_SPEC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for ERROR_SPEC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ERROR_SPEC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for ERROR_SPEC {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for ERROR_SPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union ERROR_SPEC_0 {
    pub errs_ipv4: Error_Spec_IPv4,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for ERROR_SPEC_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for ERROR_SPEC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for ERROR_SPEC_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for ERROR_SPEC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ERROR_SPEC_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for ERROR_SPEC_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for ERROR_SPEC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERROR_SPECF_InPlace: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERROR_SPECF_NotGuilty: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERROR_TC_NOT_SUPPORTED: u32 = 7514u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERROR_TC_OBJECT_LENGTH_INVALID: u32 = 7515u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERROR_TC_SUPPORTED_OBJECTS_EXIST: u32 = 7512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERROR_TOO_MANY_CLIENTS: u32 = 7521u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERR_FORWARD_OK: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERR_Usage_globl: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERR_Usage_local: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERR_Usage_serv: u32 = 17u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ERR_global_mask: u32 = 4095u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const EXPIRED_CREDENTIAL: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct Error_Spec_IPv4 {
    pub errs_errnode: super::super::Networking::WinSock::IN_ADDR,
    pub errs_flags: u8,
    pub errs_code: u8,
    pub errs_value: u16,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for Error_Spec_IPv4 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for Error_Spec_IPv4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for Error_Spec_IPv4 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for Error_Spec_IPv4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<Error_Spec_IPv4>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for Error_Spec_IPv4 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for Error_Spec_IPv4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct FILTER_SPEC {
    pub filt_header: RsvpObjHdr,
    pub filt_u: FILTER_SPEC_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for FILTER_SPEC {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for FILTER_SPEC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for FILTER_SPEC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for FILTER_SPEC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILTER_SPEC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for FILTER_SPEC {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for FILTER_SPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union FILTER_SPEC_0 {
    pub filt_ipv4: Filter_Spec_IPv4,
    pub filt_ipv4gpi: Filter_Spec_IPv4GPI,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for FILTER_SPEC_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for FILTER_SPEC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for FILTER_SPEC_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for FILTER_SPEC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILTER_SPEC_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for FILTER_SPEC_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for FILTER_SPEC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct FLOWDESCRIPTOR {
    pub FlowSpec: super::super::Networking::WinSock::FLOWSPEC,
    pub NumFilters: u32,
    pub FilterList: *mut RSVP_FILTERSPEC,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for FLOWDESCRIPTOR {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for FLOWDESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for FLOWDESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FLOWDESCRIPTOR").field("FlowSpec", &self.FlowSpec).field("NumFilters", &self.NumFilters).field("FilterList", &self.FilterList).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for FLOWDESCRIPTOR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for FLOWDESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FLOWDESCRIPTOR>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for FLOWDESCRIPTOR {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for FLOWDESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const FLOW_DURATION: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const FORCE_IMMEDIATE_REFRESH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const FSCTL_TCP_BASE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const FVEB_UNLOCK_FLAG_AUK_OSFVEINFO: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const FVEB_UNLOCK_FLAG_CACHED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const FVEB_UNLOCK_FLAG_EXTERNAL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const FVEB_UNLOCK_FLAG_MEDIA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const FVEB_UNLOCK_FLAG_NBP: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const FVEB_UNLOCK_FLAG_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const FVEB_UNLOCK_FLAG_PASSPHRASE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const FVEB_UNLOCK_FLAG_PIN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const FVEB_UNLOCK_FLAG_RECOVERY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const FVEB_UNLOCK_FLAG_TPM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FilterType(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const FILTERSPECV4: FilterType = FilterType(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const FILTERSPECV6: FilterType = FilterType(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const FILTERSPECV6_FLOW: FilterType = FilterType(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const FILTERSPECV4_GPI: FilterType = FilterType(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const FILTERSPECV6_GPI: FilterType = FilterType(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const FILTERSPEC_END: FilterType = FilterType(6i32);
impl ::core::marker::Copy for FilterType {}
impl ::core::clone::Clone for FilterType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FilterType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FilterType {
    type Abi = Self;
}
impl ::core::fmt::Debug for FilterType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FilterType").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct Filter_Spec_IPv4 {
    pub filt_ipaddr: super::super::Networking::WinSock::IN_ADDR,
    pub filt_unused: u16,
    pub filt_port: u16,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for Filter_Spec_IPv4 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for Filter_Spec_IPv4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for Filter_Spec_IPv4 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for Filter_Spec_IPv4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<Filter_Spec_IPv4>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for Filter_Spec_IPv4 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for Filter_Spec_IPv4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct Filter_Spec_IPv4GPI {
    pub filt_ipaddr: super::super::Networking::WinSock::IN_ADDR,
    pub filt_gpi: u32,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for Filter_Spec_IPv4GPI {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for Filter_Spec_IPv4GPI {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for Filter_Spec_IPv4GPI {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for Filter_Spec_IPv4GPI {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<Filter_Spec_IPv4GPI>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for Filter_Spec_IPv4GPI {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for Filter_Spec_IPv4GPI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const GENERAL_INFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const GQOS_API: u32 = 56400u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const GQOS_ERRORCODE_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const GQOS_ERRORVALUE_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const GQOS_KERNEL_TC: u32 = 56700u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const GQOS_KERNEL_TC_SYS: u32 = 56500u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const GQOS_NET_ADMISSION: u32 = 56100u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const GQOS_NET_POLICY: u32 = 56200u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const GQOS_NO_ERRORCODE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const GQOS_NO_ERRORVALUE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const GQOS_RSVP: u32 = 56300u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const GQOS_RSVP_SYS: u32 = 56600u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const GUARANTEED_SERV: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const GUAR_ADSPARM_C: i32 = 131i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const GUAR_ADSPARM_Csum: i32 = 135i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const GUAR_ADSPARM_Ctot: i32 = 133i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const GUAR_ADSPARM_D: i32 = 132i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const GUAR_ADSPARM_Dsum: i32 = 136i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const GUAR_ADSPARM_Dtot: i32 = 134i32;
pub const GUID_QOS_BESTEFFORT_BANDWIDTH: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed885290_40ec_11d1_2c91_00aa00574915);
pub const GUID_QOS_ENABLE_AVG_STATS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbafb6d11_27c4_4801_a46f_ef8080c188c8);
pub const GUID_QOS_ENABLE_WINDOW_ADJUSTMENT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa966725_d3e9_4c55_b335_2a00279a1e64);
pub const GUID_QOS_FLOW_8021P_CONFORMING: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08c1e013_fcd2_11d2_be1e_00a0c99ee63b);
pub const GUID_QOS_FLOW_8021P_NONCONFORMING: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09023f91_fcd2_11d2_be1e_00a0c99ee63b);
pub const GUID_QOS_FLOW_COUNT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1147f880_40ed_11d1_2c91_00aa00574915);
pub const GUID_QOS_FLOW_IP_CONFORMING: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07f99a8b_fcd2_11d2_be1e_00a0c99ee63b);
pub const GUID_QOS_FLOW_IP_NONCONFORMING: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x087a5987_fcd2_11d2_be1e_00a0c99ee63b);
pub const GUID_QOS_FLOW_MODE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c82290a_515a_11d2_8e58_00c04fc9bfcb);
pub const GUID_QOS_ISSLOW_FLOW: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xabf273a4_ee07_11d2_be1b_00a0c99ee63b);
pub const GUID_QOS_LATENCY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc408ef0_40ec_11d1_2c91_00aa00574915);
pub const GUID_QOS_MAX_OUTSTANDING_SENDS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x161ffa86_6120_11d1_2c91_00aa00574915);
pub const GUID_QOS_NON_BESTEFFORT_LIMIT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x185c44e0_40ed_11d1_2c91_00aa00574915);
pub const GUID_QOS_REMAINING_BANDWIDTH: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4c51720_40ec_11d1_2c91_00aa00574915);
pub const GUID_QOS_STATISTICS_BUFFER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb2c0980_e900_11d1_b07e_0080c71382bf);
pub const GUID_QOS_TIMER_RESOLUTION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba10cc88_f13e_11d2_be1b_00a0c99ee63b);
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct Gads_parms_t {
    pub Gads_serv_hdr: IntServServiceHdr,
    pub Gads_Ctot_hdr: IntServParmHdr,
    pub Gads_Ctot: u32,
    pub Gads_Dtot_hdr: IntServParmHdr,
    pub Gads_Dtot: u32,
    pub Gads_Csum_hdr: IntServParmHdr,
    pub Gads_Csum: u32,
    pub Gads_Dsum_hdr: IntServParmHdr,
    pub Gads_Dsum: u32,
}
impl ::core::marker::Copy for Gads_parms_t {}
impl ::core::clone::Clone for Gads_parms_t {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Gads_parms_t {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Gads_parms_t").field("Gads_serv_hdr", &self.Gads_serv_hdr).field("Gads_Ctot_hdr", &self.Gads_Ctot_hdr).field("Gads_Ctot", &self.Gads_Ctot).field("Gads_Dtot_hdr", &self.Gads_Dtot_hdr).field("Gads_Dtot", &self.Gads_Dtot).field("Gads_Csum_hdr", &self.Gads_Csum_hdr).field("Gads_Csum", &self.Gads_Csum).field("Gads_Dsum_hdr", &self.Gads_Dsum_hdr).field("Gads_Dsum", &self.Gads_Dsum).finish()
    }
}
unsafe impl ::windows::core::Abi for Gads_parms_t {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for Gads_parms_t {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<Gads_parms_t>()) == 0 }
    }
}
impl ::core::cmp::Eq for Gads_parms_t {}
impl ::core::default::Default for Gads_parms_t {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct GenAdspecParams {
    pub gen_parm_hdr: IntServServiceHdr,
    pub gen_parm_hopcnt_hdr: IntServParmHdr,
    pub gen_parm_hopcnt: u32,
    pub gen_parm_pathbw_hdr: IntServParmHdr,
    pub gen_parm_path_bw: f32,
    pub gen_parm_minlat_hdr: IntServParmHdr,
    pub gen_parm_min_latency: u32,
    pub gen_parm_compmtu_hdr: IntServParmHdr,
    pub gen_parm_composed_MTU: u32,
}
impl ::core::marker::Copy for GenAdspecParams {}
impl ::core::clone::Clone for GenAdspecParams {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GenAdspecParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GenAdspecParams")
            .field("gen_parm_hdr", &self.gen_parm_hdr)
            .field("gen_parm_hopcnt_hdr", &self.gen_parm_hopcnt_hdr)
            .field("gen_parm_hopcnt", &self.gen_parm_hopcnt)
            .field("gen_parm_pathbw_hdr", &self.gen_parm_pathbw_hdr)
            .field("gen_parm_path_bw", &self.gen_parm_path_bw)
            .field("gen_parm_minlat_hdr", &self.gen_parm_minlat_hdr)
            .field("gen_parm_min_latency", &self.gen_parm_min_latency)
            .field("gen_parm_compmtu_hdr", &self.gen_parm_compmtu_hdr)
            .field("gen_parm_composed_MTU", &self.gen_parm_composed_MTU)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for GenAdspecParams {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GenAdspecParams {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GenAdspecParams>()) == 0 }
    }
}
impl ::core::cmp::Eq for GenAdspecParams {}
impl ::core::default::Default for GenAdspecParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct GenTspec {
    pub gen_Tspec_serv_hdr: IntServServiceHdr,
    pub gen_Tspec_parm_hdr: IntServParmHdr,
    pub gen_Tspec_parms: GenTspecParms,
}
impl ::core::marker::Copy for GenTspec {}
impl ::core::clone::Clone for GenTspec {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GenTspec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GenTspec").field("gen_Tspec_serv_hdr", &self.gen_Tspec_serv_hdr).field("gen_Tspec_parm_hdr", &self.gen_Tspec_parm_hdr).field("gen_Tspec_parms", &self.gen_Tspec_parms).finish()
    }
}
unsafe impl ::windows::core::Abi for GenTspec {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GenTspec {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GenTspec>()) == 0 }
    }
}
impl ::core::cmp::Eq for GenTspec {}
impl ::core::default::Default for GenTspec {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct GenTspecParms {
    pub TB_Tspec_r: f32,
    pub TB_Tspec_b: f32,
    pub TB_Tspec_p: f32,
    pub TB_Tspec_m: u32,
    pub TB_Tspec_M: u32,
}
impl ::core::marker::Copy for GenTspecParms {}
impl ::core::clone::Clone for GenTspecParms {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GenTspecParms {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GenTspecParms").field("TB_Tspec_r", &self.TB_Tspec_r).field("TB_Tspec_b", &self.TB_Tspec_b).field("TB_Tspec_p", &self.TB_Tspec_p).field("TB_Tspec_m", &self.TB_Tspec_m).field("TB_Tspec_M", &self.TB_Tspec_M).finish()
    }
}
unsafe impl ::windows::core::Abi for GenTspecParms {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GenTspecParms {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GenTspecParms>()) == 0 }
    }
}
impl ::core::cmp::Eq for GenTspecParms {}
impl ::core::default::Default for GenTspecParms {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct GuarFlowSpec {
    pub Guar_serv_hdr: IntServServiceHdr,
    pub Guar_Tspec_hdr: IntServParmHdr,
    pub Guar_Tspec_parms: GenTspecParms,
    pub Guar_Rspec_hdr: IntServParmHdr,
    pub Guar_Rspec: GuarRspec,
}
impl ::core::marker::Copy for GuarFlowSpec {}
impl ::core::clone::Clone for GuarFlowSpec {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GuarFlowSpec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GuarFlowSpec").field("Guar_serv_hdr", &self.Guar_serv_hdr).field("Guar_Tspec_hdr", &self.Guar_Tspec_hdr).field("Guar_Tspec_parms", &self.Guar_Tspec_parms).field("Guar_Rspec_hdr", &self.Guar_Rspec_hdr).field("Guar_Rspec", &self.Guar_Rspec).finish()
    }
}
unsafe impl ::windows::core::Abi for GuarFlowSpec {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GuarFlowSpec {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GuarFlowSpec>()) == 0 }
    }
}
impl ::core::cmp::Eq for GuarFlowSpec {}
impl ::core::default::Default for GuarFlowSpec {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct GuarRspec {
    pub Guar_R: f32,
    pub Guar_S: u32,
}
impl ::core::marker::Copy for GuarRspec {}
impl ::core::clone::Clone for GuarRspec {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GuarRspec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GuarRspec").field("Guar_R", &self.Guar_R).field("Guar_S", &self.Guar_S).finish()
    }
}
unsafe impl ::windows::core::Abi for GuarRspec {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GuarRspec {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GuarRspec>()) == 0 }
    }
}
impl ::core::cmp::Eq for GuarRspec {}
impl ::core::default::Default for GuarRspec {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const HIGHLY_DELAY_SENSITIVE: u32 = 4294967294u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const IDENTITY_CHANGED: u32 = 5u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct IDPE_ATTR {
    pub PeAttribLength: u16,
    pub PeAttribType: u8,
    pub PeAttribSubType: u8,
    pub PeAttribValue: [u8; 4],
}
impl ::core::marker::Copy for IDPE_ATTR {}
impl ::core::clone::Clone for IDPE_ATTR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IDPE_ATTR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IDPE_ATTR").field("PeAttribLength", &self.PeAttribLength).field("PeAttribType", &self.PeAttribType).field("PeAttribSubType", &self.PeAttribSubType).field("PeAttribValue", &self.PeAttribValue).finish()
    }
}
unsafe impl ::windows::core::Abi for IDPE_ATTR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IDPE_ATTR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IDPE_ATTR>()) == 0 }
    }
}
impl ::core::cmp::Eq for IDPE_ATTR {}
impl ::core::default::Default for IDPE_ATTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct ID_ERROR_OBJECT {
    pub usIdErrLength: u16,
    pub ucAType: u8,
    pub ucSubType: u8,
    pub usReserved: u16,
    pub usIdErrorValue: u16,
    pub ucIdErrData: [u8; 4],
}
impl ::core::marker::Copy for ID_ERROR_OBJECT {}
impl ::core::clone::Clone for ID_ERROR_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ID_ERROR_OBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ID_ERROR_OBJECT").field("usIdErrLength", &self.usIdErrLength).field("ucAType", &self.ucAType).field("ucSubType", &self.ucSubType).field("usReserved", &self.usReserved).field("usIdErrorValue", &self.usIdErrorValue).field("ucIdErrData", &self.ucIdErrData).finish()
    }
}
unsafe impl ::windows::core::Abi for ID_ERROR_OBJECT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ID_ERROR_OBJECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ID_ERROR_OBJECT>()) == 0 }
    }
}
impl ::core::cmp::Eq for ID_ERROR_OBJECT {}
impl ::core::default::Default for ID_ERROR_OBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const IF_MIB_STATS_ID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const INFO_NOT_AVAILABLE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const INSUFFICIENT_PRIVILEGES: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const INTSERV_VERSION0: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const INTSERV_VERS_MASK: u32 = 240u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const INV_LPM_HANDLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const INV_REQ_HANDLE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const INV_RESULTS: u32 = 5u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub union IN_ADDR_IPV4 {
    pub Addr: u32,
    pub AddrBytes: [u8; 4],
}
impl ::core::marker::Copy for IN_ADDR_IPV4 {}
impl ::core::clone::Clone for IN_ADDR_IPV4 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IN_ADDR_IPV4 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IN_ADDR_IPV4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IN_ADDR_IPV4>()) == 0 }
    }
}
impl ::core::cmp::Eq for IN_ADDR_IPV4 {}
impl ::core::default::Default for IN_ADDR_IPV4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct IN_ADDR_IPV6 {
    pub Addr: [u8; 16],
}
impl ::core::marker::Copy for IN_ADDR_IPV6 {}
impl ::core::clone::Clone for IN_ADDR_IPV6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IN_ADDR_IPV6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IN_ADDR_IPV6").field("Addr", &self.Addr).finish()
    }
}
unsafe impl ::windows::core::Abi for IN_ADDR_IPV6 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IN_ADDR_IPV6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IN_ADDR_IPV6>()) == 0 }
    }
}
impl ::core::cmp::Eq for IN_ADDR_IPV6 {}
impl ::core::default::Default for IN_ADDR_IPV6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct IPX_PATTERN {
    pub Src: IPX_PATTERN_0,
    pub Dest: IPX_PATTERN_0,
}
impl ::core::marker::Copy for IPX_PATTERN {}
impl ::core::clone::Clone for IPX_PATTERN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPX_PATTERN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPX_PATTERN").field("Src", &self.Src).field("Dest", &self.Dest).finish()
    }
}
unsafe impl ::windows::core::Abi for IPX_PATTERN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPX_PATTERN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPX_PATTERN>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPX_PATTERN {}
impl ::core::default::Default for IPX_PATTERN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct IPX_PATTERN_0 {
    pub NetworkAddress: u32,
    pub NodeAddress: [u8; 6],
    pub Socket: u16,
}
impl ::core::marker::Copy for IPX_PATTERN_0 {}
impl ::core::clone::Clone for IPX_PATTERN_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IPX_PATTERN_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPX_PATTERN_0").field("NetworkAddress", &self.NetworkAddress).field("NodeAddress", &self.NodeAddress).field("Socket", &self.Socket).finish()
    }
}
unsafe impl ::windows::core::Abi for IPX_PATTERN_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IPX_PATTERN_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IPX_PATTERN_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IPX_PATTERN_0 {}
impl ::core::default::Default for IPX_PATTERN_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const IP_INTFC_INFO_ID: u32 = 259u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const IP_MIB_ADDRTABLE_ENTRY_ID: u32 = 258u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const IP_MIB_STATS_ID: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct IP_PATTERN {
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub SrcAddr: u32,
    pub DstAddr: u32,
    pub S_un: IP_PATTERN_0,
    pub ProtocolId: u8,
    pub Reserved3: [u8; 3],
}
impl ::core::marker::Copy for IP_PATTERN {}
impl ::core::clone::Clone for IP_PATTERN {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IP_PATTERN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IP_PATTERN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_PATTERN>()) == 0 }
    }
}
impl ::core::cmp::Eq for IP_PATTERN {}
impl ::core::default::Default for IP_PATTERN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub union IP_PATTERN_0 {
    pub S_un_ports: IP_PATTERN_0_1,
    pub S_un_icmp: IP_PATTERN_0_0,
    pub S_Spi: u32,
}
impl ::core::marker::Copy for IP_PATTERN_0 {}
impl ::core::clone::Clone for IP_PATTERN_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IP_PATTERN_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IP_PATTERN_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_PATTERN_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IP_PATTERN_0 {}
impl ::core::default::Default for IP_PATTERN_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct IP_PATTERN_0_0 {
    pub s_type: u8,
    pub s_code: u8,
    pub filler: u16,
}
impl ::core::marker::Copy for IP_PATTERN_0_0 {}
impl ::core::clone::Clone for IP_PATTERN_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IP_PATTERN_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_PATTERN_0_0").field("s_type", &self.s_type).field("s_code", &self.s_code).field("filler", &self.filler).finish()
    }
}
unsafe impl ::windows::core::Abi for IP_PATTERN_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IP_PATTERN_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_PATTERN_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IP_PATTERN_0_0 {}
impl ::core::default::Default for IP_PATTERN_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct IP_PATTERN_0_1 {
    pub s_srcport: u16,
    pub s_dstport: u16,
}
impl ::core::marker::Copy for IP_PATTERN_0_1 {}
impl ::core::clone::Clone for IP_PATTERN_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IP_PATTERN_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IP_PATTERN_0_1").field("s_srcport", &self.s_srcport).field("s_dstport", &self.s_dstport).finish()
    }
}
unsafe impl ::windows::core::Abi for IP_PATTERN_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IP_PATTERN_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IP_PATTERN_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for IP_PATTERN_0_1 {}
impl ::core::default::Default for IP_PATTERN_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ISPH_FLG_INV: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ISSH_BREAK_BIT: u32 = 128u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct IS_ADSPEC_BODY {
    pub adspec_mh: IntServMainHdr,
    pub adspec_genparms: GenAdspecParams,
}
impl ::core::marker::Copy for IS_ADSPEC_BODY {}
impl ::core::clone::Clone for IS_ADSPEC_BODY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IS_ADSPEC_BODY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IS_ADSPEC_BODY").field("adspec_mh", &self.adspec_mh).field("adspec_genparms", &self.adspec_genparms).finish()
    }
}
unsafe impl ::windows::core::Abi for IS_ADSPEC_BODY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IS_ADSPEC_BODY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IS_ADSPEC_BODY>()) == 0 }
    }
}
impl ::core::cmp::Eq for IS_ADSPEC_BODY {}
impl ::core::default::Default for IS_ADSPEC_BODY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct IS_FLOWSPEC {
    pub flow_header: RsvpObjHdr,
    pub flow_body: IntServFlowSpec,
}
impl ::core::marker::Copy for IS_FLOWSPEC {}
impl ::core::clone::Clone for IS_FLOWSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IS_FLOWSPEC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IS_FLOWSPEC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IS_FLOWSPEC>()) == 0 }
    }
}
impl ::core::cmp::Eq for IS_FLOWSPEC {}
impl ::core::default::Default for IS_FLOWSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const IS_GUAR_RSPEC: i32 = 130i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct IntServFlowSpec {
    pub spec_mh: IntServMainHdr,
    pub spec_u: IntServFlowSpec_0,
}
impl ::core::marker::Copy for IntServFlowSpec {}
impl ::core::clone::Clone for IntServFlowSpec {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IntServFlowSpec {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IntServFlowSpec {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IntServFlowSpec>()) == 0 }
    }
}
impl ::core::cmp::Eq for IntServFlowSpec {}
impl ::core::default::Default for IntServFlowSpec {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub union IntServFlowSpec_0 {
    pub CL_spec: CtrlLoadFlowspec,
    pub G_spec: GuarFlowSpec,
    pub Q_spec: QualAppFlowSpec,
}
impl ::core::marker::Copy for IntServFlowSpec_0 {}
impl ::core::clone::Clone for IntServFlowSpec_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IntServFlowSpec_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IntServFlowSpec_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IntServFlowSpec_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IntServFlowSpec_0 {}
impl ::core::default::Default for IntServFlowSpec_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct IntServMainHdr {
    pub ismh_version: u8,
    pub ismh_unused: u8,
    pub ismh_len32b: u16,
}
impl ::core::marker::Copy for IntServMainHdr {}
impl ::core::clone::Clone for IntServMainHdr {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IntServMainHdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IntServMainHdr").field("ismh_version", &self.ismh_version).field("ismh_unused", &self.ismh_unused).field("ismh_len32b", &self.ismh_len32b).finish()
    }
}
unsafe impl ::windows::core::Abi for IntServMainHdr {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IntServMainHdr {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IntServMainHdr>()) == 0 }
    }
}
impl ::core::cmp::Eq for IntServMainHdr {}
impl ::core::default::Default for IntServMainHdr {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct IntServParmHdr {
    pub isph_parm_num: u8,
    pub isph_flags: u8,
    pub isph_len32b: u16,
}
impl ::core::marker::Copy for IntServParmHdr {}
impl ::core::clone::Clone for IntServParmHdr {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IntServParmHdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IntServParmHdr").field("isph_parm_num", &self.isph_parm_num).field("isph_flags", &self.isph_flags).field("isph_len32b", &self.isph_len32b).finish()
    }
}
unsafe impl ::windows::core::Abi for IntServParmHdr {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IntServParmHdr {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IntServParmHdr>()) == 0 }
    }
}
impl ::core::cmp::Eq for IntServParmHdr {}
impl ::core::default::Default for IntServParmHdr {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct IntServServiceHdr {
    pub issh_service: u8,
    pub issh_flags: u8,
    pub issh_len32b: u16,
}
impl ::core::marker::Copy for IntServServiceHdr {}
impl ::core::clone::Clone for IntServServiceHdr {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IntServServiceHdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IntServServiceHdr").field("issh_service", &self.issh_service).field("issh_flags", &self.issh_flags).field("issh_len32b", &self.issh_len32b).finish()
    }
}
unsafe impl ::windows::core::Abi for IntServServiceHdr {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IntServServiceHdr {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IntServServiceHdr>()) == 0 }
    }
}
impl ::core::cmp::Eq for IntServServiceHdr {}
impl ::core::default::Default for IntServServiceHdr {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct IntServTspecBody {
    pub st_mh: IntServMainHdr,
    pub tspec_u: IntServTspecBody_0,
}
impl ::core::marker::Copy for IntServTspecBody {}
impl ::core::clone::Clone for IntServTspecBody {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IntServTspecBody {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IntServTspecBody {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IntServTspecBody>()) == 0 }
    }
}
impl ::core::cmp::Eq for IntServTspecBody {}
impl ::core::default::Default for IntServTspecBody {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub union IntServTspecBody_0 {
    pub gen_stspec: GenTspec,
    pub qual_stspec: QualTspec,
}
impl ::core::marker::Copy for IntServTspecBody_0 {}
impl ::core::clone::Clone for IntServTspecBody_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for IntServTspecBody_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IntServTspecBody_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IntServTspecBody_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for IntServTspecBody_0 {}
impl ::core::default::Default for IntServTspecBody_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const LINE_RATE: u32 = 50003u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const LOCAL_QOSABILITY: u32 = 50005u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const LOCAL_TRAFFIC_CONTROL: u32 = 50004u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const LPM_API_VERSION_1: u32 = 1u32;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LPM_HANDLE(pub isize);
impl LPM_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for LPM_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for LPM_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for LPM_HANDLE {}
impl ::core::fmt::Debug for LPM_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LPM_HANDLE").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<LPM_HANDLE>> for LPM_HANDLE {
    fn from(optional: ::core::option::Option<LPM_HANDLE>) -> LPM_HANDLE {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for LPM_HANDLE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct LPM_INIT_INFO {
    pub PcmVersionNumber: u32,
    pub ResultTimeLimit: u32,
    pub ConfiguredLpmCount: i32,
    pub AllocMemory: PALLOCMEM,
    pub FreeMemory: PFREEMEM,
    pub PcmAdmitResultCallback: CBADMITRESULT,
    pub GetRsvpObjectsCallback: CBGETRSVPOBJECTS,
}
impl ::core::marker::Copy for LPM_INIT_INFO {}
impl ::core::clone::Clone for LPM_INIT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LPM_INIT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LPM_INIT_INFO").field("PcmVersionNumber", &self.PcmVersionNumber).field("ResultTimeLimit", &self.ResultTimeLimit).field("ConfiguredLpmCount", &self.ConfiguredLpmCount).field("AllocMemory", &self.AllocMemory.map(|f| f as usize)).field("FreeMemory", &self.FreeMemory.map(|f| f as usize)).field("PcmAdmitResultCallback", &self.PcmAdmitResultCallback.map(|f| f as usize)).field("GetRsvpObjectsCallback", &self.GetRsvpObjectsCallback.map(|f| f as usize)).finish()
    }
}
unsafe impl ::windows::core::Abi for LPM_INIT_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LPM_INIT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LPM_INIT_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for LPM_INIT_INFO {}
impl ::core::default::Default for LPM_INIT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const LPM_OK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const LPM_PE_ALL_TYPES: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const LPM_PE_APP_IDENTITY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const LPM_PE_USER_IDENTITY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const LPM_RESULT_DEFER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const LPM_RESULT_READY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const LPM_TIME_OUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const LPV_DONT_CARE: u32 = 65534u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const LPV_DROP_MSG: u32 = 65533u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const LPV_MAX_PRIORITY: u32 = 65280u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const LPV_MIN_PRIORITY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const LPV_REJECT: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const LPV_RESERVED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const MAX_PHYSADDR_SIZE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const MAX_STRING_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const MODERATELY_DELAY_SENSITIVE: u32 = 4294967293u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const OSDEVICE_TYPE_BLOCKIO_CDROM: u32 = 65539u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const OSDEVICE_TYPE_BLOCKIO_FILE: u32 = 65541u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const OSDEVICE_TYPE_BLOCKIO_HARDDISK: u32 = 65537u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const OSDEVICE_TYPE_BLOCKIO_PARTITION: u32 = 65540u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const OSDEVICE_TYPE_BLOCKIO_RAMDISK: u32 = 65542u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const OSDEVICE_TYPE_BLOCKIO_REMOVABLEDISK: u32 = 65538u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const OSDEVICE_TYPE_BLOCKIO_VIRTUALHARDDISK: u32 = 65543u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const OSDEVICE_TYPE_COMPOSITE: u32 = 327680u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const OSDEVICE_TYPE_SERIAL: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const OSDEVICE_TYPE_UDP: u32 = 196608u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const OSDEVICE_TYPE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const OSDEVICE_TYPE_VMBUS: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const Opt_Distinct: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const Opt_Explicit: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const Opt_Share_mask: u32 = 24u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const Opt_Shared: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const Opt_SndSel_mask: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const Opt_Wildcard: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub type PALLOCMEM = ::core::option::Option<unsafe extern "system" fn(size: u32) -> *mut ::core::ffi::c_void>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct PARAM_BUFFER {
    pub ParameterId: u32,
    pub Length: u32,
    pub Buffer: [u8; 1],
}
impl ::core::marker::Copy for PARAM_BUFFER {}
impl ::core::clone::Clone for PARAM_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PARAM_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PARAM_BUFFER").field("ParameterId", &self.ParameterId).field("Length", &self.Length).field("Buffer", &self.Buffer).finish()
    }
}
unsafe impl ::windows::core::Abi for PARAM_BUFFER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PARAM_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PARAM_BUFFER>()) == 0 }
    }
}
impl ::core::cmp::Eq for PARAM_BUFFER {}
impl ::core::default::Default for PARAM_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const PCM_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const PE_ATTRIB_TYPE_CREDENTIAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const PE_ATTRIB_TYPE_POLICY_LOCATOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const PE_TYPE_APPID: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub type PFREEMEM = ::core::option::Option<unsafe extern "system" fn(pv: *mut ::core::ffi::c_void)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct POLICY_DATA {
    pub PolicyObjHdr: RsvpObjHdr,
    pub usPeOffset: u16,
    pub usReserved: u16,
}
impl ::core::marker::Copy for POLICY_DATA {}
impl ::core::clone::Clone for POLICY_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for POLICY_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLICY_DATA").field("PolicyObjHdr", &self.PolicyObjHdr).field("usPeOffset", &self.usPeOffset).field("usReserved", &self.usReserved).finish()
    }
}
unsafe impl ::windows::core::Abi for POLICY_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for POLICY_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<POLICY_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for POLICY_DATA {}
impl ::core::default::Default for POLICY_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct POLICY_ELEMENT {
    pub usPeLength: u16,
    pub usPeType: u16,
    pub ucPeData: [u8; 4],
}
impl ::core::marker::Copy for POLICY_ELEMENT {}
impl ::core::clone::Clone for POLICY_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for POLICY_ELEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLICY_ELEMENT").field("usPeLength", &self.usPeLength).field("usPeType", &self.usPeType).field("ucPeData", &self.ucPeData).finish()
    }
}
unsafe impl ::windows::core::Abi for POLICY_ELEMENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for POLICY_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<POLICY_ELEMENT>()) == 0 }
    }
}
impl ::core::cmp::Eq for POLICY_ELEMENT {}
impl ::core::default::Default for POLICY_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_CRAZY_FLOWSPEC: u32 = 57u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_EXPIRED_CREDENTIALS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_EXPIRED_USER_TOKEN: u32 = 51u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_GLOBAL_DEF_FLOW_COUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_GLOBAL_DEF_FLOW_DURATION: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_GLOBAL_DEF_FLOW_RATE: u32 = 17u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_GLOBAL_DEF_PEAK_RATE: u32 = 25u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_GLOBAL_DEF_SUM_FLOW_RATE: u32 = 33u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_GLOBAL_DEF_SUM_PEAK_RATE: u32 = 41u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_GLOBAL_GRP_FLOW_COUNT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_GLOBAL_GRP_FLOW_DURATION: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_GLOBAL_GRP_FLOW_RATE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_GLOBAL_GRP_PEAK_RATE: u32 = 26u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_GLOBAL_GRP_SUM_FLOW_RATE: u32 = 34u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_GLOBAL_GRP_SUM_PEAK_RATE: u32 = 42u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_GLOBAL_UNAUTH_USER_FLOW_COUNT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_GLOBAL_UNAUTH_USER_FLOW_DURATION: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_GLOBAL_UNAUTH_USER_FLOW_RATE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_GLOBAL_UNAUTH_USER_PEAK_RATE: u32 = 28u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_GLOBAL_UNAUTH_USER_SUM_FLOW_RATE: u32 = 36u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_GLOBAL_UNAUTH_USER_SUM_PEAK_RATE: u32 = 44u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_GLOBAL_USER_FLOW_COUNT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_GLOBAL_USER_FLOW_DURATION: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_GLOBAL_USER_FLOW_RATE: u32 = 19u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_GLOBAL_USER_PEAK_RATE: u32 = 27u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_GLOBAL_USER_SUM_FLOW_RATE: u32 = 35u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_GLOBAL_USER_SUM_PEAK_RATE: u32 = 43u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_IDENTITY_CHANGED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_INSUFFICIENT_PRIVILEGES: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_NO_ACCEPTS: u32 = 55u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_NO_MEMORY: u32 = 56u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_NO_MORE_INFO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_NO_PRIVILEGES: u32 = 50u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_NO_RESOURCES: u32 = 52u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_PRE_EMPTED: u32 = 53u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_SUBNET_DEF_FLOW_COUNT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_SUBNET_DEF_FLOW_DURATION: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_SUBNET_DEF_FLOW_RATE: u32 = 21u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_SUBNET_DEF_PEAK_RATE: u32 = 29u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_SUBNET_DEF_SUM_FLOW_RATE: u32 = 37u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_SUBNET_DEF_SUM_PEAK_RATE: u32 = 45u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_SUBNET_GRP_FLOW_COUNT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_SUBNET_GRP_FLOW_DURATION: u32 = 14u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_SUBNET_GRP_FLOW_RATE: u32 = 22u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_SUBNET_GRP_PEAK_RATE: u32 = 30u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_SUBNET_GRP_SUM_FLOW_RATE: u32 = 38u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_SUBNET_GRP_SUM_PEAK_RATE: u32 = 46u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_SUBNET_UNAUTH_USER_FLOW_COUNT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_SUBNET_UNAUTH_USER_FLOW_DURATION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_SUBNET_UNAUTH_USER_FLOW_RATE: u32 = 24u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_SUBNET_UNAUTH_USER_PEAK_RATE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_SUBNET_UNAUTH_USER_SUM_FLOW_RATE: u32 = 40u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_SUBNET_UNAUTH_USER_SUM_PEAK_RATE: u32 = 48u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_SUBNET_USER_FLOW_COUNT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_SUBNET_USER_FLOW_DURATION: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_SUBNET_USER_FLOW_RATE: u32 = 23u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_SUBNET_USER_PEAK_RATE: u32 = 31u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_SUBNET_USER_SUM_FLOW_RATE: u32 = 39u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_SUBNET_USER_SUM_PEAK_RATE: u32 = 47u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_UNKNOWN_USER: u32 = 49u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_UNSUPPORTED_CREDENTIAL_TYPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_ERRV_USER_CHANGED: u32 = 54u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_LOCATOR_SUB_TYPE_ASCII_DN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_LOCATOR_SUB_TYPE_ASCII_DN_ENC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_LOCATOR_SUB_TYPE_UNICODE_DN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POLICY_LOCATOR_SUB_TYPE_UNICODE_DN_ENC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const POSITIVE_INFINITY_RATE: u32 = 4294967294u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const PREDICTIVE_SERV: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn QOSAddSocketToFlow<'a, P0, P1>(qoshandle: P0, socket: P1, destaddr: *const super::super::Networking::WinSock::SOCKADDR, traffictype: QOS_TRAFFIC_TYPE, flags: u32, flowid: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Networking::WinSock::SOCKET>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QOSAddSocketToFlow(qoshandle: super::super::Foundation::HANDLE, socket: super::super::Networking::WinSock::SOCKET, destaddr: *const super::super::Networking::WinSock::SOCKADDR, traffictype: QOS_TRAFFIC_TYPE, flags: u32, flowid: *mut u32) -> super::super::Foundation::BOOL;
    }
    QOSAddSocketToFlow(qoshandle.into(), socket.into(), ::core::mem::transmute(destaddr), traffictype, flags, ::core::mem::transmute(flowid))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn QOSCancel<'a, P0>(qoshandle: P0, overlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QOSCancel(qoshandle: super::super::Foundation::HANDLE, overlapped: *const super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    }
    QOSCancel(qoshandle.into(), ::core::mem::transmute(overlapped))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QOSCloseHandle<'a, P0>(qoshandle: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QOSCloseHandle(qoshandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    }
    QOSCloseHandle(qoshandle.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QOSCreateHandle(version: *const QOS_VERSION, qoshandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QOSCreateHandle(version: *const QOS_VERSION, qoshandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    }
    QOSCreateHandle(::core::mem::transmute(version), ::core::mem::transmute(qoshandle))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QOSEnumerateFlows<'a, P0>(qoshandle: P0, size: *mut u32, buffer: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QOSEnumerateFlows(qoshandle: super::super::Foundation::HANDLE, size: *mut u32, buffer: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    QOSEnumerateFlows(qoshandle.into(), ::core::mem::transmute(size), ::core::mem::transmute(buffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn QOSNotifyFlow<'a, P0>(qoshandle: P0, flowid: u32, operation: QOS_NOTIFY_FLOW, size: *mut u32, buffer: *mut ::core::ffi::c_void, flags: u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QOSNotifyFlow(qoshandle: super::super::Foundation::HANDLE, flowid: u32, operation: QOS_NOTIFY_FLOW, size: *mut u32, buffer: *mut ::core::ffi::c_void, flags: u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    }
    QOSNotifyFlow(qoshandle.into(), flowid, operation, ::core::mem::transmute(size), ::core::mem::transmute(buffer), flags, ::core::mem::transmute(overlapped))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn QOSQueryFlow<'a, P0>(qoshandle: P0, flowid: u32, operation: QOS_QUERY_FLOW, size: *mut u32, buffer: *mut ::core::ffi::c_void, flags: u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QOSQueryFlow(qoshandle: super::super::Foundation::HANDLE, flowid: u32, operation: QOS_QUERY_FLOW, size: *mut u32, buffer: *mut ::core::ffi::c_void, flags: u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    }
    QOSQueryFlow(qoshandle.into(), flowid, operation, ::core::mem::transmute(size), ::core::mem::transmute(buffer), flags, ::core::mem::transmute(overlapped))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn QOSRemoveSocketFromFlow<'a, P0, P1>(qoshandle: P0, socket: P1, flowid: u32, flags: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Networking::WinSock::SOCKET>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QOSRemoveSocketFromFlow(qoshandle: super::super::Foundation::HANDLE, socket: super::super::Networking::WinSock::SOCKET, flowid: u32, flags: u32) -> super::super::Foundation::BOOL;
    }
    QOSRemoveSocketFromFlow(qoshandle.into(), socket.into(), flowid, flags)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOSSPBASE: u32 = 50000u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOSSP_ERR_BASE: u32 = 56000u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn QOSSetFlow<'a, P0>(qoshandle: P0, flowid: u32, operation: QOS_SET_FLOW, size: u32, buffer: *const ::core::ffi::c_void, flags: u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QOSSetFlow(qoshandle: super::super::Foundation::HANDLE, flowid: u32, operation: QOS_SET_FLOW, size: u32, buffer: *const ::core::ffi::c_void, flags: u32, overlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    }
    QOSSetFlow(qoshandle.into(), flowid, operation, size, ::core::mem::transmute(buffer), flags, ::core::mem::transmute(overlapped))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn QOSStartTrackingClient<'a, P0>(qoshandle: P0, destaddr: *const super::super::Networking::WinSock::SOCKADDR, flags: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QOSStartTrackingClient(qoshandle: super::super::Foundation::HANDLE, destaddr: *const super::super::Networking::WinSock::SOCKADDR, flags: u32) -> super::super::Foundation::BOOL;
    }
    QOSStartTrackingClient(qoshandle.into(), ::core::mem::transmute(destaddr), flags)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn QOSStopTrackingClient<'a, P0>(qoshandle: P0, destaddr: *const super::super::Networking::WinSock::SOCKADDR, flags: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QOSStopTrackingClient(qoshandle: super::super::Foundation::HANDLE, destaddr: *const super::super::Networking::WinSock::SOCKADDR, flags: u32) -> super::super::Foundation::BOOL;
    }
    QOSStopTrackingClient(qoshandle.into(), ::core::mem::transmute(destaddr), flags)
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct QOS_DESTADDR {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub SocketAddress: *const super::super::Networking::WinSock::SOCKADDR,
    pub SocketAddressLength: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for QOS_DESTADDR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for QOS_DESTADDR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for QOS_DESTADDR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_DESTADDR").field("ObjectHdr", &self.ObjectHdr).field("SocketAddress", &self.SocketAddress).field("SocketAddressLength", &self.SocketAddressLength).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for QOS_DESTADDR {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for QOS_DESTADDR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QOS_DESTADDR>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for QOS_DESTADDR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for QOS_DESTADDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct QOS_DIFFSERV {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub DSFieldCount: u32,
    pub DiffservRule: [u8; 1],
}
impl ::core::marker::Copy for QOS_DIFFSERV {}
impl ::core::clone::Clone for QOS_DIFFSERV {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QOS_DIFFSERV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_DIFFSERV").field("ObjectHdr", &self.ObjectHdr).field("DSFieldCount", &self.DSFieldCount).field("DiffservRule", &self.DiffservRule).finish()
    }
}
unsafe impl ::windows::core::Abi for QOS_DIFFSERV {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QOS_DIFFSERV {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QOS_DIFFSERV>()) == 0 }
    }
}
impl ::core::cmp::Eq for QOS_DIFFSERV {}
impl ::core::default::Default for QOS_DIFFSERV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct QOS_DIFFSERV_RULE {
    pub InboundDSField: u8,
    pub ConformingOutboundDSField: u8,
    pub NonConformingOutboundDSField: u8,
    pub ConformingUserPriority: u8,
    pub NonConformingUserPriority: u8,
}
impl ::core::marker::Copy for QOS_DIFFSERV_RULE {}
impl ::core::clone::Clone for QOS_DIFFSERV_RULE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QOS_DIFFSERV_RULE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_DIFFSERV_RULE").field("InboundDSField", &self.InboundDSField).field("ConformingOutboundDSField", &self.ConformingOutboundDSField).field("NonConformingOutboundDSField", &self.NonConformingOutboundDSField).field("ConformingUserPriority", &self.ConformingUserPriority).field("NonConformingUserPriority", &self.NonConformingUserPriority).finish()
    }
}
unsafe impl ::windows::core::Abi for QOS_DIFFSERV_RULE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QOS_DIFFSERV_RULE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QOS_DIFFSERV_RULE>()) == 0 }
    }
}
impl ::core::cmp::Eq for QOS_DIFFSERV_RULE {}
impl ::core::default::Default for QOS_DIFFSERV_RULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct QOS_DS_CLASS {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub DSField: u32,
}
impl ::core::marker::Copy for QOS_DS_CLASS {}
impl ::core::clone::Clone for QOS_DS_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QOS_DS_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_DS_CLASS").field("ObjectHdr", &self.ObjectHdr).field("DSField", &self.DSField).finish()
    }
}
unsafe impl ::windows::core::Abi for QOS_DS_CLASS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QOS_DS_CLASS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QOS_DS_CLASS>()) == 0 }
    }
}
impl ::core::cmp::Eq for QOS_DS_CLASS {}
impl ::core::default::Default for QOS_DS_CLASS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct QOS_FLOWRATE_OUTGOING {
    pub Bandwidth: u64,
    pub ShapingBehavior: QOS_SHAPING,
    pub Reason: QOS_FLOWRATE_REASON,
}
impl ::core::marker::Copy for QOS_FLOWRATE_OUTGOING {}
impl ::core::clone::Clone for QOS_FLOWRATE_OUTGOING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QOS_FLOWRATE_OUTGOING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_FLOWRATE_OUTGOING").field("Bandwidth", &self.Bandwidth).field("ShapingBehavior", &self.ShapingBehavior).field("Reason", &self.Reason).finish()
    }
}
unsafe impl ::windows::core::Abi for QOS_FLOWRATE_OUTGOING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QOS_FLOWRATE_OUTGOING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QOS_FLOWRATE_OUTGOING>()) == 0 }
    }
}
impl ::core::cmp::Eq for QOS_FLOWRATE_OUTGOING {}
impl ::core::default::Default for QOS_FLOWRATE_OUTGOING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct QOS_FLOWRATE_REASON(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOSFlowRateNotApplicable: QOS_FLOWRATE_REASON = QOS_FLOWRATE_REASON(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOSFlowRateContentChange: QOS_FLOWRATE_REASON = QOS_FLOWRATE_REASON(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOSFlowRateCongestion: QOS_FLOWRATE_REASON = QOS_FLOWRATE_REASON(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOSFlowRateHigherContentEncoding: QOS_FLOWRATE_REASON = QOS_FLOWRATE_REASON(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOSFlowRateUserCaused: QOS_FLOWRATE_REASON = QOS_FLOWRATE_REASON(4i32);
impl ::core::marker::Copy for QOS_FLOWRATE_REASON {}
impl ::core::clone::Clone for QOS_FLOWRATE_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QOS_FLOWRATE_REASON {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for QOS_FLOWRATE_REASON {
    type Abi = Self;
}
impl ::core::fmt::Debug for QOS_FLOWRATE_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QOS_FLOWRATE_REASON").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct QOS_FLOW_FUNDAMENTALS {
    pub BottleneckBandwidthSet: super::super::Foundation::BOOL,
    pub BottleneckBandwidth: u64,
    pub AvailableBandwidthSet: super::super::Foundation::BOOL,
    pub AvailableBandwidth: u64,
    pub RTTSet: super::super::Foundation::BOOL,
    pub RTT: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for QOS_FLOW_FUNDAMENTALS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for QOS_FLOW_FUNDAMENTALS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for QOS_FLOW_FUNDAMENTALS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_FLOW_FUNDAMENTALS").field("BottleneckBandwidthSet", &self.BottleneckBandwidthSet).field("BottleneckBandwidth", &self.BottleneckBandwidth).field("AvailableBandwidthSet", &self.AvailableBandwidthSet).field("AvailableBandwidth", &self.AvailableBandwidth).field("RTTSet", &self.RTTSet).field("RTT", &self.RTT).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for QOS_FLOW_FUNDAMENTALS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for QOS_FLOW_FUNDAMENTALS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QOS_FLOW_FUNDAMENTALS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for QOS_FLOW_FUNDAMENTALS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for QOS_FLOW_FUNDAMENTALS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct QOS_FRIENDLY_NAME {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub FriendlyName: [u16; 256],
}
impl ::core::marker::Copy for QOS_FRIENDLY_NAME {}
impl ::core::clone::Clone for QOS_FRIENDLY_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QOS_FRIENDLY_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_FRIENDLY_NAME").field("ObjectHdr", &self.ObjectHdr).field("FriendlyName", &self.FriendlyName).finish()
    }
}
unsafe impl ::windows::core::Abi for QOS_FRIENDLY_NAME {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QOS_FRIENDLY_NAME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QOS_FRIENDLY_NAME>()) == 0 }
    }
}
impl ::core::cmp::Eq for QOS_FRIENDLY_NAME {}
impl ::core::default::Default for QOS_FRIENDLY_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOS_GENERAL_ID_BASE: u32 = 2000u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOS_MAX_OBJECT_STRING_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOS_NON_ADAPTIVE_FLOW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct QOS_NOTIFY_FLOW(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOSNotifyCongested: QOS_NOTIFY_FLOW = QOS_NOTIFY_FLOW(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOSNotifyUncongested: QOS_NOTIFY_FLOW = QOS_NOTIFY_FLOW(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOSNotifyAvailable: QOS_NOTIFY_FLOW = QOS_NOTIFY_FLOW(2i32);
impl ::core::marker::Copy for QOS_NOTIFY_FLOW {}
impl ::core::clone::Clone for QOS_NOTIFY_FLOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QOS_NOTIFY_FLOW {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for QOS_NOTIFY_FLOW {
    type Abi = Self;
}
impl ::core::fmt::Debug for QOS_NOTIFY_FLOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QOS_NOTIFY_FLOW").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOS_NOT_SPECIFIED: u32 = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct QOS_OBJECT_HDR {
    pub ObjectType: u32,
    pub ObjectLength: u32,
}
impl ::core::marker::Copy for QOS_OBJECT_HDR {}
impl ::core::clone::Clone for QOS_OBJECT_HDR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QOS_OBJECT_HDR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_OBJECT_HDR").field("ObjectType", &self.ObjectType).field("ObjectLength", &self.ObjectLength).finish()
    }
}
unsafe impl ::windows::core::Abi for QOS_OBJECT_HDR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QOS_OBJECT_HDR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QOS_OBJECT_HDR>()) == 0 }
    }
}
impl ::core::cmp::Eq for QOS_OBJECT_HDR {}
impl ::core::default::Default for QOS_OBJECT_HDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOS_OUTGOING_DEFAULT_MINIMUM_BANDWIDTH: u32 = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct QOS_PACKET_PRIORITY {
    pub ConformantDSCPValue: u32,
    pub NonConformantDSCPValue: u32,
    pub ConformantL2Value: u32,
    pub NonConformantL2Value: u32,
}
impl ::core::marker::Copy for QOS_PACKET_PRIORITY {}
impl ::core::clone::Clone for QOS_PACKET_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QOS_PACKET_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_PACKET_PRIORITY").field("ConformantDSCPValue", &self.ConformantDSCPValue).field("NonConformantDSCPValue", &self.NonConformantDSCPValue).field("ConformantL2Value", &self.ConformantL2Value).field("NonConformantL2Value", &self.NonConformantL2Value).finish()
    }
}
unsafe impl ::windows::core::Abi for QOS_PACKET_PRIORITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QOS_PACKET_PRIORITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QOS_PACKET_PRIORITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for QOS_PACKET_PRIORITY {}
impl ::core::default::Default for QOS_PACKET_PRIORITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOS_QUERYFLOW_FRESH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct QOS_QUERY_FLOW(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOSQueryFlowFundamentals: QOS_QUERY_FLOW = QOS_QUERY_FLOW(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOSQueryPacketPriority: QOS_QUERY_FLOW = QOS_QUERY_FLOW(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOSQueryOutgoingRate: QOS_QUERY_FLOW = QOS_QUERY_FLOW(2i32);
impl ::core::marker::Copy for QOS_QUERY_FLOW {}
impl ::core::clone::Clone for QOS_QUERY_FLOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QOS_QUERY_FLOW {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for QOS_QUERY_FLOW {
    type Abi = Self;
}
impl ::core::fmt::Debug for QOS_QUERY_FLOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QOS_QUERY_FLOW").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct QOS_SD_MODE {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub ShapeDiscardMode: u32,
}
impl ::core::marker::Copy for QOS_SD_MODE {}
impl ::core::clone::Clone for QOS_SD_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QOS_SD_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_SD_MODE").field("ObjectHdr", &self.ObjectHdr).field("ShapeDiscardMode", &self.ShapeDiscardMode).finish()
    }
}
unsafe impl ::windows::core::Abi for QOS_SD_MODE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QOS_SD_MODE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QOS_SD_MODE>()) == 0 }
    }
}
impl ::core::cmp::Eq for QOS_SD_MODE {}
impl ::core::default::Default for QOS_SD_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct QOS_SET_FLOW(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOSSetTrafficType: QOS_SET_FLOW = QOS_SET_FLOW(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOSSetOutgoingRate: QOS_SET_FLOW = QOS_SET_FLOW(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOSSetOutgoingDSCPValue: QOS_SET_FLOW = QOS_SET_FLOW(2i32);
impl ::core::marker::Copy for QOS_SET_FLOW {}
impl ::core::clone::Clone for QOS_SET_FLOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QOS_SET_FLOW {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for QOS_SET_FLOW {
    type Abi = Self;
}
impl ::core::fmt::Debug for QOS_SET_FLOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QOS_SET_FLOW").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct QOS_SHAPING(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOSShapeOnly: QOS_SHAPING = QOS_SHAPING(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOSShapeAndMark: QOS_SHAPING = QOS_SHAPING(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOSUseNonConformantMarkings: QOS_SHAPING = QOS_SHAPING(2i32);
impl ::core::marker::Copy for QOS_SHAPING {}
impl ::core::clone::Clone for QOS_SHAPING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QOS_SHAPING {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for QOS_SHAPING {
    type Abi = Self;
}
impl ::core::fmt::Debug for QOS_SHAPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QOS_SHAPING").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct QOS_SHAPING_RATE {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub ShapingRate: u32,
}
impl ::core::marker::Copy for QOS_SHAPING_RATE {}
impl ::core::clone::Clone for QOS_SHAPING_RATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QOS_SHAPING_RATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_SHAPING_RATE").field("ObjectHdr", &self.ObjectHdr).field("ShapingRate", &self.ShapingRate).finish()
    }
}
unsafe impl ::windows::core::Abi for QOS_SHAPING_RATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QOS_SHAPING_RATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QOS_SHAPING_RATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for QOS_SHAPING_RATE {}
impl ::core::default::Default for QOS_SHAPING_RATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct QOS_TCP_TRAFFIC {
    pub ObjectHdr: QOS_OBJECT_HDR,
}
impl ::core::marker::Copy for QOS_TCP_TRAFFIC {}
impl ::core::clone::Clone for QOS_TCP_TRAFFIC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QOS_TCP_TRAFFIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_TCP_TRAFFIC").field("ObjectHdr", &self.ObjectHdr).finish()
    }
}
unsafe impl ::windows::core::Abi for QOS_TCP_TRAFFIC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QOS_TCP_TRAFFIC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QOS_TCP_TRAFFIC>()) == 0 }
    }
}
impl ::core::cmp::Eq for QOS_TCP_TRAFFIC {}
impl ::core::default::Default for QOS_TCP_TRAFFIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct QOS_TRAFFIC_CLASS {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub TrafficClass: u32,
}
impl ::core::marker::Copy for QOS_TRAFFIC_CLASS {}
impl ::core::clone::Clone for QOS_TRAFFIC_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QOS_TRAFFIC_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_TRAFFIC_CLASS").field("ObjectHdr", &self.ObjectHdr).field("TrafficClass", &self.TrafficClass).finish()
    }
}
unsafe impl ::windows::core::Abi for QOS_TRAFFIC_CLASS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QOS_TRAFFIC_CLASS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QOS_TRAFFIC_CLASS>()) == 0 }
    }
}
impl ::core::cmp::Eq for QOS_TRAFFIC_CLASS {}
impl ::core::default::Default for QOS_TRAFFIC_CLASS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOS_TRAFFIC_GENERAL_ID_BASE: u32 = 4000u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct QOS_TRAFFIC_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOSTrafficTypeBestEffort: QOS_TRAFFIC_TYPE = QOS_TRAFFIC_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOSTrafficTypeBackground: QOS_TRAFFIC_TYPE = QOS_TRAFFIC_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOSTrafficTypeExcellentEffort: QOS_TRAFFIC_TYPE = QOS_TRAFFIC_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOSTrafficTypeAudioVideo: QOS_TRAFFIC_TYPE = QOS_TRAFFIC_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOSTrafficTypeVoice: QOS_TRAFFIC_TYPE = QOS_TRAFFIC_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QOSTrafficTypeControl: QOS_TRAFFIC_TYPE = QOS_TRAFFIC_TYPE(5i32);
impl ::core::marker::Copy for QOS_TRAFFIC_TYPE {}
impl ::core::clone::Clone for QOS_TRAFFIC_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QOS_TRAFFIC_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for QOS_TRAFFIC_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for QOS_TRAFFIC_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QOS_TRAFFIC_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct QOS_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
impl ::core::marker::Copy for QOS_VERSION {}
impl ::core::clone::Clone for QOS_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QOS_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QOS_VERSION").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).finish()
    }
}
unsafe impl ::windows::core::Abi for QOS_VERSION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QOS_VERSION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QOS_VERSION>()) == 0 }
    }
}
impl ::core::cmp::Eq for QOS_VERSION {}
impl ::core::default::Default for QOS_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const QUALITATIVE_SERV: u32 = 6u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct QualAppFlowSpec {
    pub Q_spec_serv_hdr: IntServServiceHdr,
    pub Q_spec_parm_hdr: IntServParmHdr,
    pub Q_spec_parms: QualTspecParms,
}
impl ::core::marker::Copy for QualAppFlowSpec {}
impl ::core::clone::Clone for QualAppFlowSpec {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QualAppFlowSpec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QualAppFlowSpec").field("Q_spec_serv_hdr", &self.Q_spec_serv_hdr).field("Q_spec_parm_hdr", &self.Q_spec_parm_hdr).field("Q_spec_parms", &self.Q_spec_parms).finish()
    }
}
unsafe impl ::windows::core::Abi for QualAppFlowSpec {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QualAppFlowSpec {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QualAppFlowSpec>()) == 0 }
    }
}
impl ::core::cmp::Eq for QualAppFlowSpec {}
impl ::core::default::Default for QualAppFlowSpec {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct QualTspec {
    pub qual_Tspec_serv_hdr: IntServServiceHdr,
    pub qual_Tspec_parm_hdr: IntServParmHdr,
    pub qual_Tspec_parms: QualTspecParms,
}
impl ::core::marker::Copy for QualTspec {}
impl ::core::clone::Clone for QualTspec {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QualTspec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QualTspec").field("qual_Tspec_serv_hdr", &self.qual_Tspec_serv_hdr).field("qual_Tspec_parm_hdr", &self.qual_Tspec_parm_hdr).field("qual_Tspec_parms", &self.qual_Tspec_parms).finish()
    }
}
unsafe impl ::windows::core::Abi for QualTspec {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QualTspec {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QualTspec>()) == 0 }
    }
}
impl ::core::cmp::Eq for QualTspec {}
impl ::core::default::Default for QualTspec {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct QualTspecParms {
    pub TB_Tspec_M: u32,
}
impl ::core::marker::Copy for QualTspecParms {}
impl ::core::clone::Clone for QualTspecParms {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QualTspecParms {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QualTspecParms").field("TB_Tspec_M", &self.TB_Tspec_M).finish()
    }
}
unsafe impl ::windows::core::Abi for QualTspecParms {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QualTspecParms {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QualTspecParms>()) == 0 }
    }
}
impl ::core::cmp::Eq for QualTspecParms {}
impl ::core::default::Default for QualTspecParms {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RCVD_PATH_TEAR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RCVD_RESV_TEAR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RESOURCES_ALLOCATED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RESOURCES_MODIFIED: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct RESV_STYLE {
    pub style_header: RsvpObjHdr,
    pub style_word: u32,
}
impl ::core::marker::Copy for RESV_STYLE {}
impl ::core::clone::Clone for RESV_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RESV_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESV_STYLE").field("style_header", &self.style_header).field("style_word", &self.style_word).finish()
    }
}
unsafe impl ::windows::core::Abi for RESV_STYLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RESV_STYLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RESV_STYLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for RESV_STYLE {}
impl ::core::default::Default for RESV_STYLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RHANDLE(pub isize);
impl RHANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for RHANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for RHANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for RHANDLE {}
impl ::core::fmt::Debug for RHANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RHANDLE").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<RHANDLE>> for RHANDLE {
    fn from(optional: ::core::option::Option<RHANDLE>) -> RHANDLE {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for RHANDLE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct RSVP_ADSPEC {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub GeneralParams: AD_GENERAL_PARAMS,
    pub NumberOfServices: u32,
    pub Services: [CONTROL_SERVICE; 1],
}
impl ::core::marker::Copy for RSVP_ADSPEC {}
impl ::core::clone::Clone for RSVP_ADSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RSVP_ADSPEC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RSVP_ADSPEC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RSVP_ADSPEC>()) == 0 }
    }
}
impl ::core::cmp::Eq for RSVP_ADSPEC {}
impl ::core::default::Default for RSVP_ADSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_DEFAULT_STYLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Err_ADMISSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Err_AMBIG_FILTER: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Err_API_ERROR: u32 = 20u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Err_BAD_DSTPORT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Err_BAD_SNDPORT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Err_BAD_STYLE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Err_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Err_NO_PATH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Err_NO_SENDER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Err_POLICY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Err_PREEMPTED: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Err_RSVP_SYS_ERROR: u32 = 23u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Err_TC_ERROR: u32 = 21u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Err_TC_SYS_ERROR: u32 = 22u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Err_UNKNOWN_CTYPE: u32 = 14u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Err_UNKNOWN_STYLE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Err_UNKN_OBJ_CLASS: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Erv_API: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Erv_Bandwidth: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Erv_Bucket_szie: u32 = 32770u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Erv_Conflict_Serv: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Erv_Crazy_Flowspec: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Erv_Crazy_Tspec: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Erv_DelayBnd: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Erv_Flow_Rate: u32 = 32769u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Erv_MEMORY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Erv_MTU: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Erv_Min_Policied_size: u32 = 32772u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Erv_No_Serv: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Erv_Nonev: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Erv_Other: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_Erv_Peak_Rate: u32 = 32771u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct RSVP_FILTERSPEC {
    pub Type: FilterType,
    pub Anonymous: RSVP_FILTERSPEC_0,
}
impl ::core::marker::Copy for RSVP_FILTERSPEC {}
impl ::core::clone::Clone for RSVP_FILTERSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RSVP_FILTERSPEC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RSVP_FILTERSPEC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RSVP_FILTERSPEC>()) == 0 }
    }
}
impl ::core::cmp::Eq for RSVP_FILTERSPEC {}
impl ::core::default::Default for RSVP_FILTERSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub union RSVP_FILTERSPEC_0 {
    pub FilterSpecV4: RSVP_FILTERSPEC_V4,
    pub FilterSpecV6: RSVP_FILTERSPEC_V6,
    pub FilterSpecV6Flow: RSVP_FILTERSPEC_V6_FLOW,
    pub FilterSpecV4Gpi: RSVP_FILTERSPEC_V4_GPI,
    pub FilterSpecV6Gpi: RSVP_FILTERSPEC_V6_GPI,
}
impl ::core::marker::Copy for RSVP_FILTERSPEC_0 {}
impl ::core::clone::Clone for RSVP_FILTERSPEC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RSVP_FILTERSPEC_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RSVP_FILTERSPEC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RSVP_FILTERSPEC_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for RSVP_FILTERSPEC_0 {}
impl ::core::default::Default for RSVP_FILTERSPEC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct RSVP_FILTERSPEC_V4 {
    pub Address: IN_ADDR_IPV4,
    pub Unused: u16,
    pub Port: u16,
}
impl ::core::marker::Copy for RSVP_FILTERSPEC_V4 {}
impl ::core::clone::Clone for RSVP_FILTERSPEC_V4 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RSVP_FILTERSPEC_V4 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RSVP_FILTERSPEC_V4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RSVP_FILTERSPEC_V4>()) == 0 }
    }
}
impl ::core::cmp::Eq for RSVP_FILTERSPEC_V4 {}
impl ::core::default::Default for RSVP_FILTERSPEC_V4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct RSVP_FILTERSPEC_V4_GPI {
    pub Address: IN_ADDR_IPV4,
    pub GeneralPortId: u32,
}
impl ::core::marker::Copy for RSVP_FILTERSPEC_V4_GPI {}
impl ::core::clone::Clone for RSVP_FILTERSPEC_V4_GPI {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RSVP_FILTERSPEC_V4_GPI {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RSVP_FILTERSPEC_V4_GPI {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RSVP_FILTERSPEC_V4_GPI>()) == 0 }
    }
}
impl ::core::cmp::Eq for RSVP_FILTERSPEC_V4_GPI {}
impl ::core::default::Default for RSVP_FILTERSPEC_V4_GPI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct RSVP_FILTERSPEC_V6 {
    pub Address: IN_ADDR_IPV6,
    pub UnUsed: u16,
    pub Port: u16,
}
impl ::core::marker::Copy for RSVP_FILTERSPEC_V6 {}
impl ::core::clone::Clone for RSVP_FILTERSPEC_V6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RSVP_FILTERSPEC_V6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RSVP_FILTERSPEC_V6").field("Address", &self.Address).field("UnUsed", &self.UnUsed).field("Port", &self.Port).finish()
    }
}
unsafe impl ::windows::core::Abi for RSVP_FILTERSPEC_V6 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RSVP_FILTERSPEC_V6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RSVP_FILTERSPEC_V6>()) == 0 }
    }
}
impl ::core::cmp::Eq for RSVP_FILTERSPEC_V6 {}
impl ::core::default::Default for RSVP_FILTERSPEC_V6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct RSVP_FILTERSPEC_V6_FLOW {
    pub Address: IN_ADDR_IPV6,
    pub UnUsed: u8,
    pub FlowLabel: [u8; 3],
}
impl ::core::marker::Copy for RSVP_FILTERSPEC_V6_FLOW {}
impl ::core::clone::Clone for RSVP_FILTERSPEC_V6_FLOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RSVP_FILTERSPEC_V6_FLOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RSVP_FILTERSPEC_V6_FLOW").field("Address", &self.Address).field("UnUsed", &self.UnUsed).field("FlowLabel", &self.FlowLabel).finish()
    }
}
unsafe impl ::windows::core::Abi for RSVP_FILTERSPEC_V6_FLOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RSVP_FILTERSPEC_V6_FLOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RSVP_FILTERSPEC_V6_FLOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for RSVP_FILTERSPEC_V6_FLOW {}
impl ::core::default::Default for RSVP_FILTERSPEC_V6_FLOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct RSVP_FILTERSPEC_V6_GPI {
    pub Address: IN_ADDR_IPV6,
    pub GeneralPortId: u32,
}
impl ::core::marker::Copy for RSVP_FILTERSPEC_V6_GPI {}
impl ::core::clone::Clone for RSVP_FILTERSPEC_V6_GPI {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RSVP_FILTERSPEC_V6_GPI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RSVP_FILTERSPEC_V6_GPI").field("Address", &self.Address).field("GeneralPortId", &self.GeneralPortId).finish()
    }
}
unsafe impl ::windows::core::Abi for RSVP_FILTERSPEC_V6_GPI {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RSVP_FILTERSPEC_V6_GPI {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RSVP_FILTERSPEC_V6_GPI>()) == 0 }
    }
}
impl ::core::cmp::Eq for RSVP_FILTERSPEC_V6_GPI {}
impl ::core::default::Default for RSVP_FILTERSPEC_V6_GPI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_FIXED_FILTER_STYLE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct RSVP_HOP {
    pub hop_header: RsvpObjHdr,
    pub hop_u: RSVP_HOP_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for RSVP_HOP {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for RSVP_HOP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for RSVP_HOP {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for RSVP_HOP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RSVP_HOP>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for RSVP_HOP {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for RSVP_HOP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union RSVP_HOP_0 {
    pub hop_ipv4: Rsvp_Hop_IPv4,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for RSVP_HOP_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for RSVP_HOP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for RSVP_HOP_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for RSVP_HOP_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RSVP_HOP_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for RSVP_HOP_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for RSVP_HOP_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct RSVP_MSG_OBJS {
    pub RsvpMsgType: i32,
    pub pRsvpSession: *mut RSVP_SESSION,
    pub pRsvpFromHop: *mut RSVP_HOP,
    pub pRsvpToHop: *mut RSVP_HOP,
    pub pResvStyle: *mut RESV_STYLE,
    pub pRsvpScope: *mut RSVP_SCOPE,
    pub FlowDescCount: i32,
    pub pFlowDescs: *mut flow_desc,
    pub PdObjectCount: i32,
    pub ppPdObjects: *mut *mut POLICY_DATA,
    pub pErrorSpec: *mut ERROR_SPEC,
    pub pAdspec: *mut ADSPEC,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for RSVP_MSG_OBJS {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for RSVP_MSG_OBJS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for RSVP_MSG_OBJS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RSVP_MSG_OBJS")
            .field("RsvpMsgType", &self.RsvpMsgType)
            .field("pRsvpSession", &self.pRsvpSession)
            .field("pRsvpFromHop", &self.pRsvpFromHop)
            .field("pRsvpToHop", &self.pRsvpToHop)
            .field("pResvStyle", &self.pResvStyle)
            .field("pRsvpScope", &self.pRsvpScope)
            .field("FlowDescCount", &self.FlowDescCount)
            .field("pFlowDescs", &self.pFlowDescs)
            .field("PdObjectCount", &self.PdObjectCount)
            .field("ppPdObjects", &self.ppPdObjects)
            .field("pErrorSpec", &self.pErrorSpec)
            .field("pAdspec", &self.pAdspec)
            .finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for RSVP_MSG_OBJS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for RSVP_MSG_OBJS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RSVP_MSG_OBJS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for RSVP_MSG_OBJS {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for RSVP_MSG_OBJS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_OBJECT_ID_BASE: u32 = 1000u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_PATH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_PATH_ERR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_PATH_TEAR: u32 = 5u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct RSVP_POLICY {
    pub Len: u16,
    pub Type: u16,
    pub Info: [u8; 4],
}
impl ::core::marker::Copy for RSVP_POLICY {}
impl ::core::clone::Clone for RSVP_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RSVP_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RSVP_POLICY").field("Len", &self.Len).field("Type", &self.Type).field("Info", &self.Info).finish()
    }
}
unsafe impl ::windows::core::Abi for RSVP_POLICY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RSVP_POLICY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RSVP_POLICY>()) == 0 }
    }
}
impl ::core::cmp::Eq for RSVP_POLICY {}
impl ::core::default::Default for RSVP_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct RSVP_POLICY_INFO {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub NumPolicyElement: u32,
    pub PolicyElement: [RSVP_POLICY; 1],
}
impl ::core::marker::Copy for RSVP_POLICY_INFO {}
impl ::core::clone::Clone for RSVP_POLICY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RSVP_POLICY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RSVP_POLICY_INFO").field("ObjectHdr", &self.ObjectHdr).field("NumPolicyElement", &self.NumPolicyElement).field("PolicyElement", &self.PolicyElement).finish()
    }
}
unsafe impl ::windows::core::Abi for RSVP_POLICY_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RSVP_POLICY_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RSVP_POLICY_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for RSVP_POLICY_INFO {}
impl ::core::default::Default for RSVP_POLICY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct RSVP_RESERVE_INFO {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub Style: u32,
    pub ConfirmRequest: u32,
    pub PolicyElementList: *mut RSVP_POLICY_INFO,
    pub NumFlowDesc: u32,
    pub FlowDescList: *mut FLOWDESCRIPTOR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for RSVP_RESERVE_INFO {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for RSVP_RESERVE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for RSVP_RESERVE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RSVP_RESERVE_INFO").field("ObjectHdr", &self.ObjectHdr).field("Style", &self.Style).field("ConfirmRequest", &self.ConfirmRequest).field("PolicyElementList", &self.PolicyElementList).field("NumFlowDesc", &self.NumFlowDesc).field("FlowDescList", &self.FlowDescList).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for RSVP_RESERVE_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for RSVP_RESERVE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RSVP_RESERVE_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for RSVP_RESERVE_INFO {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for RSVP_RESERVE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_RESV: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_RESV_ERR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_RESV_TEAR: u32 = 6u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct RSVP_SCOPE {
    pub scopl_header: RsvpObjHdr,
    pub scope_u: RSVP_SCOPE_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for RSVP_SCOPE {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for RSVP_SCOPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for RSVP_SCOPE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for RSVP_SCOPE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RSVP_SCOPE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for RSVP_SCOPE {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for RSVP_SCOPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union RSVP_SCOPE_0 {
    pub scopl_ipv4: Scope_list_ipv4,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for RSVP_SCOPE_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for RSVP_SCOPE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for RSVP_SCOPE_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for RSVP_SCOPE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RSVP_SCOPE_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for RSVP_SCOPE_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for RSVP_SCOPE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct RSVP_SESSION {
    pub sess_header: RsvpObjHdr,
    pub sess_u: RSVP_SESSION_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for RSVP_SESSION {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for RSVP_SESSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for RSVP_SESSION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for RSVP_SESSION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RSVP_SESSION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for RSVP_SESSION {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for RSVP_SESSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union RSVP_SESSION_0 {
    pub sess_ipv4: Session_IPv4,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for RSVP_SESSION_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for RSVP_SESSION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for RSVP_SESSION_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for RSVP_SESSION_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RSVP_SESSION_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for RSVP_SESSION_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for RSVP_SESSION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_SHARED_EXPLICIT_STYLE: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct RSVP_STATUS_INFO {
    pub ObjectHdr: QOS_OBJECT_HDR,
    pub StatusCode: u32,
    pub ExtendedStatus1: u32,
    pub ExtendedStatus2: u32,
}
impl ::core::marker::Copy for RSVP_STATUS_INFO {}
impl ::core::clone::Clone for RSVP_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RSVP_STATUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RSVP_STATUS_INFO").field("ObjectHdr", &self.ObjectHdr).field("StatusCode", &self.StatusCode).field("ExtendedStatus1", &self.ExtendedStatus1).field("ExtendedStatus2", &self.ExtendedStatus2).finish()
    }
}
unsafe impl ::windows::core::Abi for RSVP_STATUS_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RSVP_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RSVP_STATUS_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for RSVP_STATUS_INFO {}
impl ::core::default::Default for RSVP_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const RSVP_WILDCARD_STYLE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct RsvpObjHdr {
    pub obj_length: u16,
    pub obj_class: u8,
    pub obj_ctype: u8,
}
impl ::core::marker::Copy for RsvpObjHdr {}
impl ::core::clone::Clone for RsvpObjHdr {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RsvpObjHdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RsvpObjHdr").field("obj_length", &self.obj_length).field("obj_class", &self.obj_class).field("obj_ctype", &self.obj_ctype).finish()
    }
}
unsafe impl ::windows::core::Abi for RsvpObjHdr {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RsvpObjHdr {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RsvpObjHdr>()) == 0 }
    }
}
impl ::core::cmp::Eq for RsvpObjHdr {}
impl ::core::default::Default for RsvpObjHdr {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct Rsvp_Hop_IPv4 {
    pub hop_ipaddr: super::super::Networking::WinSock::IN_ADDR,
    pub hop_LIH: u32,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for Rsvp_Hop_IPv4 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for Rsvp_Hop_IPv4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for Rsvp_Hop_IPv4 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for Rsvp_Hop_IPv4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<Rsvp_Hop_IPv4>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for Rsvp_Hop_IPv4 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for Rsvp_Hop_IPv4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct SENDER_TSPEC {
    pub stspec_header: RsvpObjHdr,
    pub stspec_body: IntServTspecBody,
}
impl ::core::marker::Copy for SENDER_TSPEC {}
impl ::core::clone::Clone for SENDER_TSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SENDER_TSPEC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SENDER_TSPEC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SENDER_TSPEC>()) == 0 }
    }
}
impl ::core::cmp::Eq for SENDER_TSPEC {}
impl ::core::default::Default for SENDER_TSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SERVICETYPE_BESTEFFORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SERVICETYPE_CONTROLLEDLOAD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SERVICETYPE_GENERAL_INFORMATION: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SERVICETYPE_GUARANTEED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SERVICETYPE_NETWORK_CONTROL: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SERVICETYPE_NETWORK_UNAVAILABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SERVICETYPE_NOCHANGE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SERVICETYPE_NONCONFORMING: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SERVICETYPE_NOTRAFFIC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SERVICETYPE_QUALITATIVE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SERVICE_BESTEFFORT: u32 = 2147549184u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SERVICE_CONTROLLEDLOAD: u32 = 2147614720u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SERVICE_GUARANTEED: u32 = 2147745792u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SERVICE_NO_QOS_SIGNALING: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SERVICE_NO_TRAFFIC_CONTROL: u32 = 2164260864u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SERVICE_QUALITATIVE: u32 = 2149580800u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SESSFLG_E_Police: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAERROR_FIRMWAREFAILURE: u32 = 196609u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAERROR_INTERNALFAILURE: u32 = 196611u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENTTYPE_AGGREGATION: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENTTYPE_AUTHORITY: u32 = 393216u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENTTYPE_CONTAINER: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENTTYPE_DRTM: u32 = 786432u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENTTYPE_ELAM: u32 = 589824u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENTTYPE_ERROR: u32 = 196608u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENTTYPE_INFORMATION: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENTTYPE_KSR: u32 = 720896u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENTTYPE_LOADEDMODULE: u32 = 458752u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENTTYPE_NONMEASURED: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENTTYPE_OSPARAMETER: u32 = 327680u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENTTYPE_PREOSPARAMETER: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENTTYPE_TRUSTPOINT: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENTTYPE_VBS: u32 = 655360u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_APPLICATION_RETURN: u32 = 131076u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_APPLICATION_SVN: u32 = 131081u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_AUTHENTICODEHASH: u32 = 458756u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_AUTHORITYISSUER: u32 = 458757u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_AUTHORITYPUBKEY: u32 = 393218u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_AUTHORITYPUBLISHER: u32 = 458760u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_AUTHORITYSERIAL: u32 = 458758u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_AUTHORITYSHA1THUMBPRINT: u32 = 458761u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_BITLOCKER_UNLOCK: u32 = 131077u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_BOOTCOUNTER: u32 = 131074u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_BOOTDEBUGGING: u32 = 262145u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_BOOT_REVOCATION_LIST: u32 = 262146u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_CODEINTEGRITY: u32 = 327682u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_COUNTERID: u32 = 131079u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_DATAEXECUTIONPREVENTION: u32 = 327684u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_DRIVER_LOAD_POLICY: u32 = 327694u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_DRTM_AMD_SMM_HASH: u32 = 786435u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_DRTM_AMD_SMM_SIGNER_KEY: u32 = 786436u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_DRTM_SMM_LEVEL: u32 = 786434u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_DRTM_STATE_AUTH: u32 = 786433u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_DUMPS_DISABLED: u32 = 327717u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_DUMP_ENCRYPTION_ENABLED: u32 = 327718u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_DUMP_ENCRYPTION_KEY_DIGEST: u32 = 327719u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_ELAM_CONFIGURATION: u32 = 589826u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_ELAM_KEYNAME: u32 = 589825u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_ELAM_MEASURED: u32 = 589828u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_ELAM_POLICY: u32 = 589827u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_EVENTCOUNTER: u32 = 131078u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_FILEPATH: u32 = 458753u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_FLIGHTSIGNING: u32 = 327713u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_HASHALGORITHMID: u32 = 458755u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_HIBERNATION_DISABLED: u32 = 327716u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_HYPERVISOR_BOOT_DMA_PROTECTION: u32 = 327728u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_HYPERVISOR_DEBUG: u32 = 327693u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_HYPERVISOR_IOMMU_POLICY: u32 = 327692u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_HYPERVISOR_LAUNCH_TYPE: u32 = 327690u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_HYPERVISOR_MMIO_NX_POLICY: u32 = 327696u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_HYPERVISOR_MSR_FILTER_POLICY: u32 = 327697u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_HYPERVISOR_PATH: u32 = 327691u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_IMAGEBASE: u32 = 458759u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_IMAGESIZE: u32 = 458754u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_IMAGEVALIDATED: u32 = 458762u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_INFORMATION: u32 = 131073u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_KSR_SIGNATURE: u32 = 720897u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_LSAISO_CONFIG: u32 = 327720u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_MODULE_SVN: u32 = 458763u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_MORBIT_API_STATUS: u32 = 131083u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_MORBIT_NOT_CANCELABLE: u32 = 131080u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_NOAUTHORITY: u32 = 393217u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_OSDEVICE: u32 = 327688u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_OSKERNELDEBUG: u32 = 327681u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_OS_REVOCATION_LIST: u32 = 327699u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_PAGEFILE_ENCRYPTION_ENABLED: u32 = 327714u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_PHYSICALADDRESSEXTENSION: u32 = 327687u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_SAFEMODE: u32 = 327685u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_SBCP_INFO: u32 = 327721u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_SI_POLICY: u32 = 327695u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_SMT_STATUS: u32 = 327700u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_SVN_CHAIN_STATUS: u32 = 131082u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_SYSTEMROOT: u32 = 327689u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_TESTSIGNING: u32 = 327683u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_TRANSFER_CONTROL: u32 = 131075u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_VBS_DUMP_USES_AMEROOT: u32 = 655369u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_VBS_HVCI_POLICY: u32 = 655367u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_VBS_IOMMU_REQUIRED: u32 = 655363u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_VBS_MANDATORY_ENFORCEMENT: u32 = 655366u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_VBS_MICROSOFT_BOOT_CHAIN_REQUIRED: u32 = 655368u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_VBS_MMIO_NX_REQUIRED: u32 = 655364u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_VBS_MSR_FILTERING_REQUIRED: u32 = 655365u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_VBS_SECUREBOOT_REQUIRED: u32 = 655362u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_VBS_VSM_NOSECRETS_ENFORCED: u32 = 655370u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_VBS_VSM_REQUIRED: u32 = 655361u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_VSM_IDKS_INFO: u32 = 327715u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_VSM_IDK_INFO: u32 = 327712u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_VSM_LAUNCH_TYPE: u32 = 327698u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEVENT_WINPE: u32 = 327686u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_ACTION: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_AMD_SL_EVENT_BASE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_AMD_SL_LOAD: u32 = 32769u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_AMD_SL_LOAD_1: u32 = 32774u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_AMD_SL_PSP_FW_SPLT: u32 = 32770u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_AMD_SL_PUB_KEY: u32 = 32772u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_AMD_SL_SEPARATOR: u32 = 32775u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_AMD_SL_SVN: u32 = 32773u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_AMD_SL_TSME_RB_FUSE: u32 = 32771u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_COMPACT_HASH: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_CPU_MICROCODE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_EFI_ACTION: u32 = 2147483655u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_EFI_BOOT_SERVICES_APPLICATION: u32 = 2147483651u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_EFI_BOOT_SERVICES_DRIVER: u32 = 2147483652u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_EFI_EVENT_BASE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_EFI_GPT_EVENT: u32 = 2147483654u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_EFI_HANDOFF_TABLES: u32 = 2147483657u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_EFI_HANDOFF_TABLES2: u32 = 2147483659u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_EFI_HCRTM_EVENT: u32 = 2147483664u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_EFI_PLATFORM_FIRMWARE_BLOB: u32 = 2147483656u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_EFI_PLATFORM_FIRMWARE_BLOB2: u32 = 2147483658u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_EFI_RUNTIME_SERVICES_DRIVER: u32 = 2147483653u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_EFI_SPDM_FIRMWARE_BLOB: u32 = 2147483873u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_EFI_SPDM_FIRMWARE_CONFIG: u32 = 2147483874u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_EFI_VARIABLE_AUTHORITY: u32 = 2147483872u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_EFI_VARIABLE_BOOT: u32 = 2147483650u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_EFI_VARIABLE_DRIVER_CONFIG: u32 = 2147483649u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_EVENT_TAG: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_IPL: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_IPL_PARTITION_DATA: u32 = 14u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_NONHOST_CODE: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_NONHOST_CONFIG: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_NONHOST_INFO: u32 = 17u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_NO_ACTION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_OMIT_BOOT_DEVICE_EVENTS: u32 = 18u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_PLATFORM_CONFIG_FLAGS: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_POST_CODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_PREBOOT_CERT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_SEPARATOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_S_CRTM_CONTENTS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_S_CRTM_VERSION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_TABLE_OF_DEVICES: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_TXT_BIOSAC_REG_DATA: u32 = 1034u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_TXT_BOOT_POL_HASH: u32 = 1050u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_TXT_BPM_HASH: u32 = 1047u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_TXT_BPM_INFO_HASH: u32 = 1049u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_TXT_CAP_VALUE: u32 = 1279u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_TXT_COLD_BOOT_BIOS_HASH: u32 = 1045u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_TXT_COMBINED_HASH: u32 = 1027u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_TXT_CPU_SCRTM_STAT: u32 = 1035u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_TXT_ELEMENTS_HASH: u32 = 1037u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_TXT_EVENT_BASE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_TXT_HASH_START: u32 = 1026u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_TXT_KM_HASH: u32 = 1046u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_TXT_KM_INFO_HASH: u32 = 1048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_TXT_LCP_AUTHORITIES_HASH: u32 = 1043u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_TXT_LCP_CONTROL_HASH: u32 = 1036u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_TXT_LCP_DETAILS_HASH: u32 = 1042u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_TXT_LCP_HASH: u32 = 1041u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_TXT_MLE_HASH: u32 = 1028u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_TXT_NV_INFO_HASH: u32 = 1044u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_TXT_OSSINITDATA_CAP_HASH: u32 = 1039u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_TXT_PCR_MAPPING: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_TXT_RANDOM_VALUE: u32 = 1278u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_TXT_SINIT_PUBKEY_HASH: u32 = 1040u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_TXT_STM_HASH: u32 = 1038u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAEV_UNUSED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAHDRSIGNATURE: u32 = 1279476311u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPAKSRHDRSIGNATURE: u32 = 1297240907u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const SIPALOGVERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const STATE_TIMEOUT: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct Scope_list_ipv4 {
    pub scopl_ipaddr: [super::super::Networking::WinSock::IN_ADDR; 1],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for Scope_list_ipv4 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for Scope_list_ipv4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for Scope_list_ipv4 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for Scope_list_ipv4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<Scope_list_ipv4>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for Scope_list_ipv4 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for Scope_list_ipv4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct Session_IPv4 {
    pub sess_destaddr: super::super::Networking::WinSock::IN_ADDR,
    pub sess_protid: u8,
    pub sess_flags: u8,
    pub sess_destport: u16,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for Session_IPv4 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for Session_IPv4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for Session_IPv4 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for Session_IPv4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<Session_IPv4>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for Session_IPv4 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for Session_IPv4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const TCBASE: u32 = 7500u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct TCG_PCClientPCREventStruct {
    pub pcrIndex: u32,
    pub eventType: u32,
    pub digest: [u8; 20],
    pub eventDataSize: u32,
    pub event: [u8; 1],
}
impl ::core::marker::Copy for TCG_PCClientPCREventStruct {}
impl ::core::clone::Clone for TCG_PCClientPCREventStruct {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TCG_PCClientPCREventStruct {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TCG_PCClientPCREventStruct {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TCG_PCClientPCREventStruct>()) == 0 }
    }
}
impl ::core::cmp::Eq for TCG_PCClientPCREventStruct {}
impl ::core::default::Default for TCG_PCClientPCREventStruct {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct TCG_PCClientTaggedEventStruct {
    pub EventID: u32,
    pub EventDataSize: u32,
    pub EventData: [u8; 1],
}
impl ::core::marker::Copy for TCG_PCClientTaggedEventStruct {}
impl ::core::clone::Clone for TCG_PCClientTaggedEventStruct {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TCG_PCClientTaggedEventStruct {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TCG_PCClientTaggedEventStruct {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TCG_PCClientTaggedEventStruct>()) == 0 }
    }
}
impl ::core::cmp::Eq for TCG_PCClientTaggedEventStruct {}
impl ::core::default::Default for TCG_PCClientTaggedEventStruct {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type TCI_ADD_FLOW_COMPLETE_HANDLER = ::core::option::Option<unsafe extern "system" fn(clflowctx: super::super::Foundation::HANDLE, status: u32)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TCI_CLIENT_FUNC_LIST {
    pub ClNotifyHandler: TCI_NOTIFY_HANDLER,
    pub ClAddFlowCompleteHandler: TCI_ADD_FLOW_COMPLETE_HANDLER,
    pub ClModifyFlowCompleteHandler: TCI_MOD_FLOW_COMPLETE_HANDLER,
    pub ClDeleteFlowCompleteHandler: TCI_DEL_FLOW_COMPLETE_HANDLER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TCI_CLIENT_FUNC_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TCI_CLIENT_FUNC_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TCI_CLIENT_FUNC_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TCI_CLIENT_FUNC_LIST").field("ClNotifyHandler", &self.ClNotifyHandler.map(|f| f as usize)).field("ClAddFlowCompleteHandler", &self.ClAddFlowCompleteHandler.map(|f| f as usize)).field("ClModifyFlowCompleteHandler", &self.ClModifyFlowCompleteHandler.map(|f| f as usize)).field("ClDeleteFlowCompleteHandler", &self.ClDeleteFlowCompleteHandler.map(|f| f as usize)).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for TCI_CLIENT_FUNC_LIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TCI_CLIENT_FUNC_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TCI_CLIENT_FUNC_LIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TCI_CLIENT_FUNC_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TCI_CLIENT_FUNC_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type TCI_DEL_FLOW_COMPLETE_HANDLER = ::core::option::Option<unsafe extern "system" fn(clflowctx: super::super::Foundation::HANDLE, status: u32)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type TCI_MOD_FLOW_COMPLETE_HANDLER = ::core::option::Option<unsafe extern "system" fn(clflowctx: super::super::Foundation::HANDLE, status: u32)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type TCI_NOTIFY_HANDLER = ::core::option::Option<unsafe extern "system" fn(clregctx: super::super::Foundation::HANDLE, clifcctx: super::super::Foundation::HANDLE, event: u32, subcode: super::super::Foundation::HANDLE, bufsize: u32, buffer: *const ::core::ffi::c_void)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct TC_GEN_FILTER {
    pub AddressType: u16,
    pub PatternSize: u32,
    pub Pattern: *mut ::core::ffi::c_void,
    pub Mask: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for TC_GEN_FILTER {}
impl ::core::clone::Clone for TC_GEN_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TC_GEN_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TC_GEN_FILTER").field("AddressType", &self.AddressType).field("PatternSize", &self.PatternSize).field("Pattern", &self.Pattern).field("Mask", &self.Mask).finish()
    }
}
unsafe impl ::windows::core::Abi for TC_GEN_FILTER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TC_GEN_FILTER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TC_GEN_FILTER>()) == 0 }
    }
}
impl ::core::cmp::Eq for TC_GEN_FILTER {}
impl ::core::default::Default for TC_GEN_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct TC_GEN_FLOW {
    pub SendingFlowspec: super::super::Networking::WinSock::FLOWSPEC,
    pub ReceivingFlowspec: super::super::Networking::WinSock::FLOWSPEC,
    pub TcObjectsLength: u32,
    pub TcObjects: [QOS_OBJECT_HDR; 1],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for TC_GEN_FLOW {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for TC_GEN_FLOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for TC_GEN_FLOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TC_GEN_FLOW").field("SendingFlowspec", &self.SendingFlowspec).field("ReceivingFlowspec", &self.ReceivingFlowspec).field("TcObjectsLength", &self.TcObjectsLength).field("TcObjects", &self.TcObjects).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for TC_GEN_FLOW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for TC_GEN_FLOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TC_GEN_FLOW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for TC_GEN_FLOW {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for TC_GEN_FLOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct TC_IFC_DESCRIPTOR {
    pub Length: u32,
    pub pInterfaceName: ::windows::core::PWSTR,
    pub pInterfaceID: ::windows::core::PWSTR,
    pub AddressListDesc: ADDRESS_LIST_DESCRIPTOR,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for TC_IFC_DESCRIPTOR {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for TC_IFC_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for TC_IFC_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TC_IFC_DESCRIPTOR").field("Length", &self.Length).field("pInterfaceName", &self.pInterfaceName).field("pInterfaceID", &self.pInterfaceID).field("AddressListDesc", &self.AddressListDesc).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for TC_IFC_DESCRIPTOR {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for TC_IFC_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TC_IFC_DESCRIPTOR>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for TC_IFC_DESCRIPTOR {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for TC_IFC_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const TC_NONCONF_BORROW: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const TC_NONCONF_BORROW_PLUS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const TC_NONCONF_DISCARD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const TC_NONCONF_SHAPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const TC_NOTIFY_FLOW_CLOSE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const TC_NOTIFY_IFC_CHANGE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const TC_NOTIFY_IFC_CLOSE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const TC_NOTIFY_IFC_UP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const TC_NOTIFY_PARAM_CHANGED: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct TC_SUPPORTED_INFO_BUFFER {
    pub InstanceIDLength: u16,
    pub InstanceID: [u16; 256],
    pub InterfaceLuid: u64,
    pub AddrListDesc: ADDRESS_LIST_DESCRIPTOR,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for TC_SUPPORTED_INFO_BUFFER {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for TC_SUPPORTED_INFO_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for TC_SUPPORTED_INFO_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TC_SUPPORTED_INFO_BUFFER").field("InstanceIDLength", &self.InstanceIDLength).field("InstanceID", &self.InstanceID).field("InterfaceLuid", &self.InterfaceLuid).field("AddrListDesc", &self.AddrListDesc).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for TC_SUPPORTED_INFO_BUFFER {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for TC_SUPPORTED_INFO_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TC_SUPPORTED_INFO_BUFFER>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for TC_SUPPORTED_INFO_BUFFER {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for TC_SUPPORTED_INFO_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TcAddFilter<'a, P0>(flowhandle: P0, pgenericfilter: *const TC_GEN_FILTER, pfilterhandle: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TcAddFilter(flowhandle: super::super::Foundation::HANDLE, pgenericfilter: *const TC_GEN_FILTER, pfilterhandle: *mut super::super::Foundation::HANDLE) -> u32;
    }
    TcAddFilter(flowhandle.into(), ::core::mem::transmute(pgenericfilter), ::core::mem::transmute(pfilterhandle))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn TcAddFlow<'a, P0, P1>(ifchandle: P0, clflowctx: P1, flags: u32, pgenericflow: *const TC_GEN_FLOW, pflowhandle: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TcAddFlow(ifchandle: super::super::Foundation::HANDLE, clflowctx: super::super::Foundation::HANDLE, flags: u32, pgenericflow: *const TC_GEN_FLOW, pflowhandle: *mut super::super::Foundation::HANDLE) -> u32;
    }
    TcAddFlow(ifchandle.into(), clflowctx.into(), flags, ::core::mem::transmute(pgenericflow), ::core::mem::transmute(pflowhandle))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TcCloseInterface<'a, P0>(ifchandle: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TcCloseInterface(ifchandle: super::super::Foundation::HANDLE) -> u32;
    }
    TcCloseInterface(ifchandle.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TcDeleteFilter<'a, P0>(filterhandle: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TcDeleteFilter(filterhandle: super::super::Foundation::HANDLE) -> u32;
    }
    TcDeleteFilter(filterhandle.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TcDeleteFlow<'a, P0>(flowhandle: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TcDeleteFlow(flowhandle: super::super::Foundation::HANDLE) -> u32;
    }
    TcDeleteFlow(flowhandle.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TcDeregisterClient<'a, P0>(clienthandle: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TcDeregisterClient(clienthandle: super::super::Foundation::HANDLE) -> u32;
    }
    TcDeregisterClient(clienthandle.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn TcEnumerateFlows<'a, P0>(ifchandle: P0, penumhandle: *mut super::super::Foundation::HANDLE, pflowcount: *mut u32, pbufsize: *mut u32, buffer: *mut ENUMERATION_BUFFER) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TcEnumerateFlows(ifchandle: super::super::Foundation::HANDLE, penumhandle: *mut super::super::Foundation::HANDLE, pflowcount: *mut u32, pbufsize: *mut u32, buffer: *mut ENUMERATION_BUFFER) -> u32;
    }
    TcEnumerateFlows(ifchandle.into(), ::core::mem::transmute(penumhandle), ::core::mem::transmute(pflowcount), ::core::mem::transmute(pbufsize), ::core::mem::transmute(buffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[inline]
pub unsafe fn TcEnumerateInterfaces<'a, P0>(clienthandle: P0, pbuffersize: *mut u32, interfacebuffer: *mut TC_IFC_DESCRIPTOR) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TcEnumerateInterfaces(clienthandle: super::super::Foundation::HANDLE, pbuffersize: *mut u32, interfacebuffer: *mut TC_IFC_DESCRIPTOR) -> u32;
    }
    TcEnumerateInterfaces(clienthandle.into(), ::core::mem::transmute(pbuffersize), ::core::mem::transmute(interfacebuffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TcGetFlowNameA<'a, P0>(flowhandle: P0, pflowname: &mut [u8]) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TcGetFlowNameA(flowhandle: super::super::Foundation::HANDLE, strsize: u32, pflowname: ::windows::core::PSTR) -> u32;
    }
    TcGetFlowNameA(flowhandle.into(), pflowname.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pflowname)))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TcGetFlowNameW<'a, P0>(flowhandle: P0, pflowname: &mut [u16]) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TcGetFlowNameW(flowhandle: super::super::Foundation::HANDLE, strsize: u32, pflowname: ::windows::core::PWSTR) -> u32;
    }
    TcGetFlowNameW(flowhandle.into(), pflowname.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pflowname)))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn TcModifyFlow<'a, P0>(flowhandle: P0, pgenericflow: *const TC_GEN_FLOW) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TcModifyFlow(flowhandle: super::super::Foundation::HANDLE, pgenericflow: *const TC_GEN_FLOW) -> u32;
    }
    TcModifyFlow(flowhandle.into(), ::core::mem::transmute(pgenericflow))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TcOpenInterfaceA<'a, P0, P1, P2>(pinterfacename: P0, clienthandle: P1, clifcctx: P2, pifchandle: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P2: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TcOpenInterfaceA(pinterfacename: ::windows::core::PCSTR, clienthandle: super::super::Foundation::HANDLE, clifcctx: super::super::Foundation::HANDLE, pifchandle: *mut super::super::Foundation::HANDLE) -> u32;
    }
    TcOpenInterfaceA(pinterfacename.into(), clienthandle.into(), clifcctx.into(), ::core::mem::transmute(pifchandle))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TcOpenInterfaceW<'a, P0, P1, P2>(pinterfacename: P0, clienthandle: P1, clifcctx: P2, pifchandle: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P2: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TcOpenInterfaceW(pinterfacename: ::windows::core::PCWSTR, clienthandle: super::super::Foundation::HANDLE, clifcctx: super::super::Foundation::HANDLE, pifchandle: *mut super::super::Foundation::HANDLE) -> u32;
    }
    TcOpenInterfaceW(pinterfacename.into(), clienthandle.into(), clifcctx.into(), ::core::mem::transmute(pifchandle))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
#[inline]
pub unsafe fn TcQueryFlowA<'a, P0>(pflowname: P0, pguidparam: *const ::windows::core::GUID, pbuffersize: *mut u32, buffer: *mut ::core::ffi::c_void) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TcQueryFlowA(pflowname: ::windows::core::PCSTR, pguidparam: *const ::windows::core::GUID, pbuffersize: *mut u32, buffer: *mut ::core::ffi::c_void) -> u32;
    }
    TcQueryFlowA(pflowname.into(), ::core::mem::transmute(pguidparam), ::core::mem::transmute(pbuffersize), ::core::mem::transmute(buffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
#[inline]
pub unsafe fn TcQueryFlowW<'a, P0>(pflowname: P0, pguidparam: *const ::windows::core::GUID, pbuffersize: *mut u32, buffer: *mut ::core::ffi::c_void) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TcQueryFlowW(pflowname: ::windows::core::PCWSTR, pguidparam: *const ::windows::core::GUID, pbuffersize: *mut u32, buffer: *mut ::core::ffi::c_void) -> u32;
    }
    TcQueryFlowW(pflowname.into(), ::core::mem::transmute(pguidparam), ::core::mem::transmute(pbuffersize), ::core::mem::transmute(buffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TcQueryInterface<'a, P0, P1>(ifchandle: P0, pguidparam: *const ::windows::core::GUID, notifychange: P1, pbuffersize: *mut u32, buffer: *mut ::core::ffi::c_void) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::BOOLEAN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TcQueryInterface(ifchandle: super::super::Foundation::HANDLE, pguidparam: *const ::windows::core::GUID, notifychange: super::super::Foundation::BOOLEAN, pbuffersize: *mut u32, buffer: *mut ::core::ffi::c_void) -> u32;
    }
    TcQueryInterface(ifchandle.into(), ::core::mem::transmute(pguidparam), notifychange.into(), ::core::mem::transmute(pbuffersize), ::core::mem::transmute(buffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TcRegisterClient<'a, P0>(tciversion: u32, clregctx: P0, clienthandlerlist: *const TCI_CLIENT_FUNC_LIST, pclienthandle: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TcRegisterClient(tciversion: u32, clregctx: super::super::Foundation::HANDLE, clienthandlerlist: *const TCI_CLIENT_FUNC_LIST, pclienthandle: *mut super::super::Foundation::HANDLE) -> u32;
    }
    TcRegisterClient(tciversion, clregctx.into(), ::core::mem::transmute(clienthandlerlist), ::core::mem::transmute(pclienthandle))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
#[inline]
pub unsafe fn TcSetFlowA<'a, P0>(pflowname: P0, pguidparam: *const ::windows::core::GUID, buffersize: u32, buffer: *const ::core::ffi::c_void) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TcSetFlowA(pflowname: ::windows::core::PCSTR, pguidparam: *const ::windows::core::GUID, buffersize: u32, buffer: *const ::core::ffi::c_void) -> u32;
    }
    TcSetFlowA(pflowname.into(), ::core::mem::transmute(pguidparam), buffersize, ::core::mem::transmute(buffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
#[inline]
pub unsafe fn TcSetFlowW<'a, P0>(pflowname: P0, pguidparam: *const ::windows::core::GUID, buffersize: u32, buffer: *const ::core::ffi::c_void) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TcSetFlowW(pflowname: ::windows::core::PCWSTR, pguidparam: *const ::windows::core::GUID, buffersize: u32, buffer: *const ::core::ffi::c_void) -> u32;
    }
    TcSetFlowW(pflowname.into(), ::core::mem::transmute(pguidparam), buffersize, ::core::mem::transmute(buffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TcSetInterface<'a, P0>(ifchandle: P0, pguidparam: *const ::windows::core::GUID, buffersize: u32, buffer: *const ::core::ffi::c_void) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn TcSetInterface(ifchandle: super::super::Foundation::HANDLE, pguidparam: *const ::windows::core::GUID, buffersize: u32, buffer: *const ::core::ffi::c_void) -> u32;
    }
    TcSetInterface(ifchandle.into(), ::core::mem::transmute(pguidparam), buffersize, ::core::mem::transmute(buffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const UNSUPPORTED_CREDENTIAL_TYPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const WBCL_DIGEST_ALG_BITMAP_SHA3_256: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const WBCL_DIGEST_ALG_BITMAP_SHA3_384: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const WBCL_DIGEST_ALG_BITMAP_SHA3_512: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const WBCL_DIGEST_ALG_BITMAP_SHA_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const WBCL_DIGEST_ALG_BITMAP_SHA_2_256: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const WBCL_DIGEST_ALG_BITMAP_SHA_2_384: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const WBCL_DIGEST_ALG_BITMAP_SHA_2_512: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const WBCL_DIGEST_ALG_BITMAP_SM3_256: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const WBCL_DIGEST_ALG_ID_SHA3_256: u32 = 39u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const WBCL_DIGEST_ALG_ID_SHA3_384: u32 = 40u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const WBCL_DIGEST_ALG_ID_SHA3_512: u32 = 41u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const WBCL_DIGEST_ALG_ID_SHA_1: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const WBCL_DIGEST_ALG_ID_SHA_2_256: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const WBCL_DIGEST_ALG_ID_SHA_2_384: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const WBCL_DIGEST_ALG_ID_SHA_2_512: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const WBCL_DIGEST_ALG_ID_SM3_256: u32 = 18u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const WBCL_HASH_LEN_SHA1: u32 = 20u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct WBCL_Iterator {
    pub firstElementPtr: *mut ::core::ffi::c_void,
    pub logSize: u32,
    pub currentElementPtr: *mut ::core::ffi::c_void,
    pub currentElementSize: u32,
    pub digestSize: u16,
    pub logFormat: u16,
    pub numberOfDigests: u32,
    pub digestSizes: *mut ::core::ffi::c_void,
    pub supportedAlgorithms: u32,
    pub hashAlgorithm: u16,
}
impl ::core::marker::Copy for WBCL_Iterator {}
impl ::core::clone::Clone for WBCL_Iterator {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WBCL_Iterator {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WBCL_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WBCL_Iterator>()) == 0 }
    }
}
impl ::core::cmp::Eq for WBCL_Iterator {}
impl ::core::default::Default for WBCL_Iterator {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct WBCL_LogHdr {
    pub signature: u32,
    pub version: u32,
    pub entries: u32,
    pub length: u32,
}
impl ::core::marker::Copy for WBCL_LogHdr {}
impl ::core::clone::Clone for WBCL_LogHdr {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WBCL_LogHdr {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WBCL_LogHdr {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WBCL_LogHdr>()) == 0 }
    }
}
impl ::core::cmp::Eq for WBCL_LogHdr {}
impl ::core::default::Default for WBCL_LogHdr {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const class_ADSPEC: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const class_CONFIRM: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const class_ERROR_SPEC: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const class_FILTER_SPEC: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const class_FLOWSPEC: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const class_INTEGRITY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const class_IS_FLOWSPEC: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const class_MAX: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const class_NULL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const class_POLICY_DATA: u32 = 14u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const class_RSVP_HOP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const class_SCOPE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const class_SENDER_TEMPLATE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const class_SENDER_TSPEC: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const class_SESSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const class_SESSION_GROUP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const class_STYLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const class_TIME_VALUES: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ctype_ADSPEC_INTSERV: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ctype_ERROR_SPEC_ipv4: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ctype_FILTER_SPEC_ipv4: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ctype_FILTER_SPEC_ipv4GPI: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ctype_FLOWSPEC_Intserv0: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ctype_POLICY_DATA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ctype_RSVP_HOP_ipv4: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ctype_SCOPE_list_ipv4: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ctype_SENDER_TEMPLATE_ipv4: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ctype_SENDER_TEMPLATE_ipv4GPI: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ctype_SENDER_TSPEC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ctype_SESSION_ipv4: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ctype_SESSION_ipv4GPI: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ctype_STYLE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct flow_desc {
    pub u1: flow_desc_0,
    pub u2: flow_desc_1,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for flow_desc {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for flow_desc {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for flow_desc {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for flow_desc {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<flow_desc>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for flow_desc {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for flow_desc {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union flow_desc_0 {
    pub stspec: *mut SENDER_TSPEC,
    pub isflow: *mut IS_FLOWSPEC,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for flow_desc_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for flow_desc_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for flow_desc_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for flow_desc_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<flow_desc_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for flow_desc_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for flow_desc_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union flow_desc_1 {
    pub stemp: *mut FILTER_SPEC,
    pub fspec: *mut FILTER_SPEC,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for flow_desc_1 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for flow_desc_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for flow_desc_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for flow_desc_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<flow_desc_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for flow_desc_1 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for flow_desc_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct int_serv_wkp(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const IS_WKP_HOP_CNT: int_serv_wkp = int_serv_wkp(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const IS_WKP_PATH_BW: int_serv_wkp = int_serv_wkp(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const IS_WKP_MIN_LATENCY: int_serv_wkp = int_serv_wkp(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const IS_WKP_COMPOSED_MTU: int_serv_wkp = int_serv_wkp(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const IS_WKP_TB_TSPEC: int_serv_wkp = int_serv_wkp(127i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const IS_WKP_Q_TSPEC: int_serv_wkp = int_serv_wkp(128i32);
impl ::core::marker::Copy for int_serv_wkp {}
impl ::core::clone::Clone for int_serv_wkp {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for int_serv_wkp {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for int_serv_wkp {
    type Abi = Self;
}
impl ::core::fmt::Debug for int_serv_wkp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("int_serv_wkp").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const ioctl_code: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct lpmiptable {
    pub ulIfIndex: u32,
    pub MediaType: u32,
    pub IfIpAddr: super::super::Networking::WinSock::IN_ADDR,
    pub IfNetMask: super::super::Networking::WinSock::IN_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for lpmiptable {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for lpmiptable {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for lpmiptable {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for lpmiptable {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<lpmiptable>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for lpmiptable {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for lpmiptable {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const mCOMPANY: u32 = 402653184u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const mIOC_IN: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const mIOC_OUT: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub const mIOC_VENDOR: u32 = 67108864u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct policy_decision {
    pub lpvResult: u32,
    pub wPolicyErrCode: u16,
    pub wPolicyErrValue: u16,
}
impl ::core::marker::Copy for policy_decision {}
impl ::core::clone::Clone for policy_decision {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for policy_decision {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("policy_decision").field("lpvResult", &self.lpvResult).field("wPolicyErrCode", &self.wPolicyErrCode).field("wPolicyErrValue", &self.wPolicyErrValue).finish()
    }
}
unsafe impl ::windows::core::Abi for policy_decision {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for policy_decision {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<policy_decision>()) == 0 }
    }
}
impl ::core::cmp::Eq for policy_decision {}
impl ::core::default::Default for policy_decision {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct tag_SIPAEVENT_KSR_SIGNATURE_PAYLOAD {
    pub SignAlgID: u32,
    pub SignatureLength: u32,
    pub Signature: [u8; 1],
}
impl ::core::marker::Copy for tag_SIPAEVENT_KSR_SIGNATURE_PAYLOAD {}
impl ::core::clone::Clone for tag_SIPAEVENT_KSR_SIGNATURE_PAYLOAD {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for tag_SIPAEVENT_KSR_SIGNATURE_PAYLOAD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tag_SIPAEVENT_KSR_SIGNATURE_PAYLOAD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tag_SIPAEVENT_KSR_SIGNATURE_PAYLOAD>()) == 0 }
    }
}
impl ::core::cmp::Eq for tag_SIPAEVENT_KSR_SIGNATURE_PAYLOAD {}
impl ::core::default::Default for tag_SIPAEVENT_KSR_SIGNATURE_PAYLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct tag_SIPAEVENT_REVOCATION_LIST_PAYLOAD {
    pub CreationTime: i64,
    pub DigestLength: u32,
    pub HashAlgID: u16,
    pub Digest: [u8; 1],
}
impl ::core::marker::Copy for tag_SIPAEVENT_REVOCATION_LIST_PAYLOAD {}
impl ::core::clone::Clone for tag_SIPAEVENT_REVOCATION_LIST_PAYLOAD {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for tag_SIPAEVENT_REVOCATION_LIST_PAYLOAD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tag_SIPAEVENT_REVOCATION_LIST_PAYLOAD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tag_SIPAEVENT_REVOCATION_LIST_PAYLOAD>()) == 0 }
    }
}
impl ::core::cmp::Eq for tag_SIPAEVENT_REVOCATION_LIST_PAYLOAD {}
impl ::core::default::Default for tag_SIPAEVENT_REVOCATION_LIST_PAYLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct tag_SIPAEVENT_SBCP_INFO_PAYLOAD_V1 {
    pub PayloadVersion: u32,
    pub VarDataOffset: u32,
    pub HashAlgID: u16,
    pub DigestLength: u16,
    pub Options: u32,
    pub SignersCount: u32,
    pub VarData: [u8; 1],
}
impl ::core::marker::Copy for tag_SIPAEVENT_SBCP_INFO_PAYLOAD_V1 {}
impl ::core::clone::Clone for tag_SIPAEVENT_SBCP_INFO_PAYLOAD_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for tag_SIPAEVENT_SBCP_INFO_PAYLOAD_V1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tag_SIPAEVENT_SBCP_INFO_PAYLOAD_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tag_SIPAEVENT_SBCP_INFO_PAYLOAD_V1>()) == 0 }
    }
}
impl ::core::cmp::Eq for tag_SIPAEVENT_SBCP_INFO_PAYLOAD_V1 {}
impl ::core::default::Default for tag_SIPAEVENT_SBCP_INFO_PAYLOAD_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct tag_SIPAEVENT_SI_POLICY_PAYLOAD {
    pub PolicyVersion: u64,
    pub PolicyNameLength: u16,
    pub HashAlgID: u16,
    pub DigestLength: u32,
    pub VarLengthData: [u8; 1],
}
impl ::core::marker::Copy for tag_SIPAEVENT_SI_POLICY_PAYLOAD {}
impl ::core::clone::Clone for tag_SIPAEVENT_SI_POLICY_PAYLOAD {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for tag_SIPAEVENT_SI_POLICY_PAYLOAD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tag_SIPAEVENT_SI_POLICY_PAYLOAD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tag_SIPAEVENT_SI_POLICY_PAYLOAD>()) == 0 }
    }
}
impl ::core::cmp::Eq for tag_SIPAEVENT_SI_POLICY_PAYLOAD {}
impl ::core::default::Default for tag_SIPAEVENT_SI_POLICY_PAYLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct tag_SIPAEVENT_VSM_IDK_INFO_PAYLOAD {
    pub KeyAlgID: u32,
    pub Anonymous: tag_SIPAEVENT_VSM_IDK_INFO_PAYLOAD_0,
}
impl ::core::marker::Copy for tag_SIPAEVENT_VSM_IDK_INFO_PAYLOAD {}
impl ::core::clone::Clone for tag_SIPAEVENT_VSM_IDK_INFO_PAYLOAD {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for tag_SIPAEVENT_VSM_IDK_INFO_PAYLOAD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tag_SIPAEVENT_VSM_IDK_INFO_PAYLOAD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tag_SIPAEVENT_VSM_IDK_INFO_PAYLOAD>()) == 0 }
    }
}
impl ::core::cmp::Eq for tag_SIPAEVENT_VSM_IDK_INFO_PAYLOAD {}
impl ::core::default::Default for tag_SIPAEVENT_VSM_IDK_INFO_PAYLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub union tag_SIPAEVENT_VSM_IDK_INFO_PAYLOAD_0 {
    pub RsaKeyInfo: tag_SIPAEVENT_VSM_IDK_RSA_INFO,
}
impl ::core::marker::Copy for tag_SIPAEVENT_VSM_IDK_INFO_PAYLOAD_0 {}
impl ::core::clone::Clone for tag_SIPAEVENT_VSM_IDK_INFO_PAYLOAD_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for tag_SIPAEVENT_VSM_IDK_INFO_PAYLOAD_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tag_SIPAEVENT_VSM_IDK_INFO_PAYLOAD_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tag_SIPAEVENT_VSM_IDK_INFO_PAYLOAD_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for tag_SIPAEVENT_VSM_IDK_INFO_PAYLOAD_0 {}
impl ::core::default::Default for tag_SIPAEVENT_VSM_IDK_INFO_PAYLOAD_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_QoS\"`*"]
pub struct tag_SIPAEVENT_VSM_IDK_RSA_INFO {
    pub KeyBitLength: u32,
    pub PublicExpLengthBytes: u32,
    pub ModulusSizeBytes: u32,
    pub PublicKeyData: [u8; 1],
}
impl ::core::marker::Copy for tag_SIPAEVENT_VSM_IDK_RSA_INFO {}
impl ::core::clone::Clone for tag_SIPAEVENT_VSM_IDK_RSA_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for tag_SIPAEVENT_VSM_IDK_RSA_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for tag_SIPAEVENT_VSM_IDK_RSA_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<tag_SIPAEVENT_VSM_IDK_RSA_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for tag_SIPAEVENT_VSM_IDK_RSA_INFO {}
impl ::core::default::Default for tag_SIPAEVENT_VSM_IDK_RSA_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
