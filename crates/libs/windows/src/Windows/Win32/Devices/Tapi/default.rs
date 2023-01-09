impl ::core::default::Default for ACDGROUP_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ACDGROUP_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACDGROUP_EVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for ACDQUEUE_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ACDQUEUE_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACDQUEUE_EVENT").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ADDRALIAS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ADDRALIAS {
    fn eq(&self, other: &Self) -> bool {
        self.rgchName == other.rgchName && self.rgchEName == other.rgchEName && self.rgchSrvr == other.rgchSrvr && self.dibDetail == other.dibDetail && self.r#type == other.r#type
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ADDRALIAS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ADDRALIAS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ADDRALIAS").field("rgchName", &self.rgchName).field("rgchEName", &self.rgchEName).field("rgchSrvr", &self.rgchSrvr).field("dibDetail", &self.dibDetail).field("type", &self.r#type).finish()
    }
}
impl ::core::default::Default for ADDRESS_CAPABILITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADDRESS_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADDRESS_CAPABILITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADDRESS_CAPABILITY_STRING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADDRESS_CAPABILITY_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADDRESS_CAPABILITY_STRING").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADDRESS_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADDRESS_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADDRESS_EVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for ADDRESS_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADDRESS_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADDRESS_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for AGENTHANDLER_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AGENTHANDLER_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AGENTHANDLER_EVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for AGENT_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AGENT_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AGENT_EVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for AGENT_SESSION_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AGENT_SESSION_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AGENT_SESSION_EVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for AGENT_SESSION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AGENT_SESSION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AGENT_SESSION_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for AGENT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AGENT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AGENT_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CALLHUB_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CALLHUB_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALLHUB_EVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for CALLHUB_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CALLHUB_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALLHUB_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CALLINFOCHANGE_CAUSE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CALLINFOCHANGE_CAUSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALLINFOCHANGE_CAUSE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CALLINFO_BUFFER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CALLINFO_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALLINFO_BUFFER").field(&self.0).finish()
    }
}
impl ::core::default::Default for CALLINFO_LONG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CALLINFO_LONG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALLINFO_LONG").field(&self.0).finish()
    }
}
impl ::core::default::Default for CALLINFO_STRING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CALLINFO_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALLINFO_STRING").field(&self.0).finish()
    }
}
impl ::core::default::Default for CALL_MEDIA_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CALL_MEDIA_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALL_MEDIA_EVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for CALL_MEDIA_EVENT_CAUSE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CALL_MEDIA_EVENT_CAUSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALL_MEDIA_EVENT_CAUSE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CALL_NOTIFICATION_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CALL_NOTIFICATION_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALL_NOTIFICATION_EVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for CALL_PRIVILEGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CALL_PRIVILEGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALL_PRIVILEGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CALL_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CALL_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALL_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CALL_STATE_EVENT_CAUSE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CALL_STATE_EVENT_CAUSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CALL_STATE_EVENT_CAUSE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DIRECTORY_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DIRECTORY_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTORY_OBJECT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DIRECTORY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DIRECTORY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIRECTORY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISCONNECT_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISCONNECT_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISCONNECT_CODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DTR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FINISH_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FINISH_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FINISH_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FT_STATE_EVENT_CAUSE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FT_STATE_EVENT_CAUSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FT_STATE_EVENT_CAUSE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FULLDUPLEX_SUPPORT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FULLDUPLEX_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FULLDUPLEX_SUPPORT").field(&self.0).finish()
    }
}
impl ::core::default::Default for HDRVCALL__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HDRVCALL__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for HDRVCALL__ {}
impl ::core::fmt::Debug for HDRVCALL__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HDRVCALL__").field("unused", &self.unused).finish()
    }
}
impl ::core::default::Default for HDRVDIALOGINSTANCE__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HDRVDIALOGINSTANCE__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for HDRVDIALOGINSTANCE__ {}
impl ::core::fmt::Debug for HDRVDIALOGINSTANCE__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HDRVDIALOGINSTANCE__").field("unused", &self.unused).finish()
    }
}
impl ::core::default::Default for HDRVLINE__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HDRVLINE__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for HDRVLINE__ {}
impl ::core::fmt::Debug for HDRVLINE__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HDRVLINE__").field("unused", &self.unused).finish()
    }
}
impl ::core::default::Default for HDRVMSPLINE__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HDRVMSPLINE__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for HDRVMSPLINE__ {}
impl ::core::fmt::Debug for HDRVMSPLINE__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HDRVMSPLINE__").field("unused", &self.unused).finish()
    }
}
impl ::core::default::Default for HDRVPHONE__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HDRVPHONE__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for HDRVPHONE__ {}
impl ::core::fmt::Debug for HDRVPHONE__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HDRVPHONE__").field("unused", &self.unused).finish()
    }
}
impl ::core::default::Default for HPROVIDER__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HPROVIDER__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for HPROVIDER__ {}
impl ::core::fmt::Debug for HPROVIDER__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HPROVIDER__").field("unused", &self.unused).finish()
    }
}
impl ::core::default::Default for HTAPICALL__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTAPICALL__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for HTAPICALL__ {}
impl ::core::fmt::Debug for HTAPICALL__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTAPICALL__").field("unused", &self.unused).finish()
    }
}
impl ::core::default::Default for HTAPILINE__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTAPILINE__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for HTAPILINE__ {}
impl ::core::fmt::Debug for HTAPILINE__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTAPILINE__").field("unused", &self.unused).finish()
    }
}
impl ::core::default::Default for HTAPIPHONE__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HTAPIPHONE__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for HTAPIPHONE__ {}
impl ::core::fmt::Debug for HTAPIPHONE__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HTAPIPHONE__").field("unused", &self.unused).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumACDGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumACDGroup {}
impl ::core::fmt::Debug for IEnumACDGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumACDGroup").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumAddress {}
impl ::core::fmt::Debug for IEnumAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumAddress").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumAgent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumAgent {}
impl ::core::fmt::Debug for IEnumAgent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumAgent").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumAgentHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumAgentHandler {}
impl ::core::fmt::Debug for IEnumAgentHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumAgentHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumAgentSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumAgentSession {}
impl ::core::fmt::Debug for IEnumAgentSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumAgentSession").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumBstr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumBstr {}
impl ::core::fmt::Debug for IEnumBstr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumBstr").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumCall {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumCall {}
impl ::core::fmt::Debug for IEnumCall {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumCall").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumCallHub {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumCallHub {}
impl ::core::fmt::Debug for IEnumCallHub {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumCallHub").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumCallingCard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumCallingCard {}
impl ::core::fmt::Debug for IEnumCallingCard {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumCallingCard").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumDialableAddrs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDialableAddrs {}
impl ::core::fmt::Debug for IEnumDialableAddrs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDialableAddrs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumDirectory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDirectory {}
impl ::core::fmt::Debug for IEnumDirectory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDirectory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumDirectoryObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDirectoryObject {}
impl ::core::fmt::Debug for IEnumDirectoryObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDirectoryObject").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumLocation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumLocation {}
impl ::core::fmt::Debug for IEnumLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumLocation").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumMcastScope {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumMcastScope {}
impl ::core::fmt::Debug for IEnumMcastScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumMcastScope").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumPhone {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumPhone {}
impl ::core::fmt::Debug for IEnumPhone {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumPhone").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumPluggableSuperclassInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumPluggableSuperclassInfo {}
impl ::core::fmt::Debug for IEnumPluggableSuperclassInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumPluggableSuperclassInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumPluggableTerminalClassInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumPluggableTerminalClassInfo {}
impl ::core::fmt::Debug for IEnumPluggableTerminalClassInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumPluggableTerminalClassInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumQueue {}
impl ::core::fmt::Debug for IEnumQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumQueue").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumStream {}
impl ::core::fmt::Debug for IEnumStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumStream").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumSubStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSubStream {}
impl ::core::fmt::Debug for IEnumSubStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSubStream").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumTerminal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTerminal {}
impl ::core::fmt::Debug for IEnumTerminal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTerminal").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumTerminalClass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTerminalClass {}
impl ::core::fmt::Debug for IEnumTerminalClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTerminalClass").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMcastAddressAllocation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMcastAddressAllocation {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMcastAddressAllocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMcastAddressAllocation").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMcastLeaseInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMcastLeaseInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMcastLeaseInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMcastLeaseInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IMcastScope {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IMcastScope {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IMcastScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMcastScope").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITACDGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITACDGroup {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITACDGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITACDGroup").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITACDGroupEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITACDGroupEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITACDGroupEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITACDGroupEvent").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITAMMediaFormat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITAMMediaFormat {}
impl ::core::fmt::Debug for ITAMMediaFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITAMMediaFormat").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITASRTerminalEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITASRTerminalEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITASRTerminalEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITASRTerminalEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITAddress {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITAddress").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITAddress2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITAddress2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITAddress2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITAddress2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ITAddress2 {
    pub unsafe fn State(&self) -> ::windows::core::Result<ADDRESS_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.State)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddressName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.AddressName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ServiceProviderName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ServiceProviderName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TAPIObject(&self) -> ::windows::core::Result<ITTAPI> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.TAPIObject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateCall(&self, pdestaddress: &::windows::core::BSTR, laddresstype: i32, lmediatypes: i32) -> ::windows::core::Result<ITBasicCallControl> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateCall)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pdestaddress), laddresstype, lmediatypes, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Calls(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Calls)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateCalls(&self) -> ::windows::core::Result<IEnumCall> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerateCalls)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DialableAddress(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DialableAddress)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateForwardInfoObject(&self) -> ::windows::core::Result<ITForwardInformation> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateForwardInfoObject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Forward<P0, P1>(&self, pforwardinfo: P0, pcall: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITForwardInformation>>,
        P1: ::std::convert::Into<::windows::core::InParam<ITBasicCallControl>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Forward)(::windows::core::Vtable::as_raw(self), pforwardinfo.into().abi(), pcall.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CurrentForwardInfo(&self) -> ::windows::core::Result<ITForwardInformation> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CurrentForwardInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMessageWaiting<P0>(&self, fmessagewaiting: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetMessageWaiting)(::windows::core::Vtable::as_raw(self), fmessagewaiting.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MessageWaiting(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MessageWaiting)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDoNotDisturb<P0>(&self, fdonotdisturb: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDoNotDisturb)(::windows::core::Vtable::as_raw(self), fdonotdisturb.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoNotDisturb(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DoNotDisturb)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITAddressCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITAddressCapabilities {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITAddressCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITAddressCapabilities").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITAddressDeviceSpecificEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITAddressDeviceSpecificEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITAddressDeviceSpecificEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITAddressDeviceSpecificEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITAddressEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITAddressEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITAddressEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITAddressEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITAddressTranslation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITAddressTranslation {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITAddressTranslation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITAddressTranslation").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITAddressTranslationInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITAddressTranslationInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITAddressTranslationInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITAddressTranslationInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITAgent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITAgent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITAgent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITAgent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITAgentEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITAgentEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITAgentEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITAgentEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITAgentHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITAgentHandler {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITAgentHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITAgentHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITAgentHandlerEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITAgentHandlerEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITAgentHandlerEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITAgentHandlerEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITAgentSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITAgentSession {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITAgentSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITAgentSession").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITAgentSessionEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITAgentSessionEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITAgentSessionEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITAgentSessionEvent").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITAllocatorProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITAllocatorProperties {}
impl ::core::fmt::Debug for ITAllocatorProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITAllocatorProperties").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITAutomatedPhoneControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITAutomatedPhoneControl {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITAutomatedPhoneControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITAutomatedPhoneControl").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITBasicAudioTerminal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITBasicAudioTerminal {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITBasicAudioTerminal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITBasicAudioTerminal").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITBasicCallControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITBasicCallControl {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITBasicCallControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITBasicCallControl").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITBasicCallControl2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITBasicCallControl2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITBasicCallControl2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITBasicCallControl2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ITBasicCallControl2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Connect<P0>(&self, fsync: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Connect)(::windows::core::Vtable::as_raw(self), fsync.into()).ok()
    }
    pub unsafe fn Answer(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Answer)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Disconnect(&self, code: DISCONNECT_CODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Disconnect)(::windows::core::Vtable::as_raw(self), code).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Hold<P0>(&self, fhold: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Hold)(::windows::core::Vtable::as_raw(self), fhold.into()).ok()
    }
    pub unsafe fn HandoffDirect(&self, papplicationname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.HandoffDirect)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(papplicationname)).ok()
    }
    pub unsafe fn HandoffIndirect(&self, lmediatype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.HandoffIndirect)(::windows::core::Vtable::as_raw(self), lmediatype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Conference<P0, P1>(&self, pcall: P0, fsync: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITBasicCallControl>>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Conference)(::windows::core::Vtable::as_raw(self), pcall.into().abi(), fsync.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Transfer<P0, P1>(&self, pcall: P0, fsync: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITBasicCallControl>>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Transfer)(::windows::core::Vtable::as_raw(self), pcall.into().abi(), fsync.into()).ok()
    }
    pub unsafe fn BlindTransfer(&self, pdestaddress: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.BlindTransfer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pdestaddress)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SwapHold<P0>(&self, pcall: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITBasicCallControl>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SwapHold)(::windows::core::Vtable::as_raw(self), pcall.into().abi()).ok()
    }
    pub unsafe fn ParkDirect(&self, pparkaddress: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ParkDirect)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pparkaddress)).ok()
    }
    pub unsafe fn ParkIndirect(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ParkIndirect)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Unpark(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Unpark)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetQOS(&self, lmediatype: i32, servicelevel: QOS_SERVICE_LEVEL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetQOS)(::windows::core::Vtable::as_raw(self), lmediatype, servicelevel).ok()
    }
    pub unsafe fn Pickup(&self, pgroupid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Pickup)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pgroupid)).ok()
    }
    pub unsafe fn Dial(&self, pdestaddress: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Dial)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pdestaddress)).ok()
    }
    pub unsafe fn Finish(&self, finishmode: FINISH_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Finish)(::windows::core::Vtable::as_raw(self), finishmode).ok()
    }
    pub unsafe fn RemoveFromConference(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RemoveFromConference)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITCallHub {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITCallHub {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITCallHub {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITCallHub").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITCallHubEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITCallHubEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITCallHubEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITCallHubEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITCallInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITCallInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITCallInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITCallInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITCallInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITCallInfo2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITCallInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITCallInfo2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ITCallInfo2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Address(&self) -> ::windows::core::Result<ITAddress> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Address)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CallState(&self) -> ::windows::core::Result<CALL_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CallState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Privilege(&self) -> ::windows::core::Result<CALL_PRIVILEGE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Privilege)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CallHub(&self) -> ::windows::core::Result<ITCallHub> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CallHub)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_CallInfoLong(&self, callinfolong: CALLINFO_LONG) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_CallInfoLong)(::windows::core::Vtable::as_raw(self), callinfolong, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_CallInfoLong(&self, callinfolong: CALLINFO_LONG, lcallinfolongval: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.put_CallInfoLong)(::windows::core::Vtable::as_raw(self), callinfolong, lcallinfolongval).ok()
    }
    pub unsafe fn get_CallInfoString(&self, callinfostring: CALLINFO_STRING) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_CallInfoString)(::windows::core::Vtable::as_raw(self), callinfostring, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_CallInfoString(&self, callinfostring: CALLINFO_STRING, pcallinfostring: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.put_CallInfoString)(::windows::core::Vtable::as_raw(self), callinfostring, ::core::mem::transmute_copy(pcallinfostring)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_CallInfoBuffer(&self, callinfobuffer: CALLINFO_BUFFER) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_CallInfoBuffer)(::windows::core::Vtable::as_raw(self), callinfobuffer, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn put_CallInfoBuffer(&self, callinfobuffer: CALLINFO_BUFFER, pcallinfobuffer: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.put_CallInfoBuffer)(::windows::core::Vtable::as_raw(self), callinfobuffer, ::core::mem::transmute(pcallinfobuffer)).ok()
    }
    pub unsafe fn GetCallInfoBuffer(&self, callinfobuffer: CALLINFO_BUFFER, pdwsize: *mut u32, ppcallinfobuffer: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCallInfoBuffer)(::windows::core::Vtable::as_raw(self), callinfobuffer, pdwsize, ppcallinfobuffer).ok()
    }
    pub unsafe fn SetCallInfoBuffer(&self, callinfobuffer: CALLINFO_BUFFER, pcallinfobuffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetCallInfoBuffer)(::windows::core::Vtable::as_raw(self), callinfobuffer, pcallinfobuffer.len() as _, ::core::mem::transmute(pcallinfobuffer.as_ptr())).ok()
    }
    pub unsafe fn ReleaseUserUserInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ReleaseUserUserInfo)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITCallInfoChangeEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITCallInfoChangeEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITCallInfoChangeEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITCallInfoChangeEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITCallMediaEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITCallMediaEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITCallMediaEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITCallMediaEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITCallNotificationEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITCallNotificationEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITCallNotificationEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITCallNotificationEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITCallStateEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITCallStateEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITCallStateEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITCallStateEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITCallingCard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITCallingCard {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITCallingCard {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITCallingCard").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITCollection2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITCollection2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITCollection2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITCollection2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ITCollection2 {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Count)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_Item)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__._NewEnum)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITCustomTone {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITCustomTone {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITCustomTone {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITCustomTone").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITDetectTone {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITDetectTone {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITDetectTone {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITDetectTone").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITDigitDetectionEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITDigitDetectionEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITDigitDetectionEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITDigitDetectionEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITDigitGenerationEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITDigitGenerationEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITDigitGenerationEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITDigitGenerationEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITDigitsGatheredEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITDigitsGatheredEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITDigitsGatheredEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITDigitsGatheredEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITDirectory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITDirectory {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITDirectory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITDirectory").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITDirectoryObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITDirectoryObject {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITDirectoryObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITDirectoryObject").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITDirectoryObjectConference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITDirectoryObjectConference {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITDirectoryObjectConference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITDirectoryObjectConference").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITDirectoryObjectUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITDirectoryObjectUser {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITDirectoryObjectUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITDirectoryObjectUser").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITDispatchMapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITDispatchMapper {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITDispatchMapper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITDispatchMapper").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITFileTerminalEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITFileTerminalEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITFileTerminalEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITFileTerminalEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITFileTrack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITFileTrack {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITFileTrack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITFileTrack").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITForwardInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITForwardInformation {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITForwardInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITForwardInformation").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITForwardInformation2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITForwardInformation2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITForwardInformation2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITForwardInformation2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ITForwardInformation2 {
    pub unsafe fn SetNumRingsNoAnswer(&self, lnumrings: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetNumRingsNoAnswer)(::windows::core::Vtable::as_raw(self), lnumrings).ok()
    }
    pub unsafe fn NumRingsNoAnswer(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.NumRingsNoAnswer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetForwardType(&self, forwardtype: i32, pdestaddress: &::windows::core::BSTR, pcalleraddress: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetForwardType)(::windows::core::Vtable::as_raw(self), forwardtype, ::core::mem::transmute_copy(pdestaddress), ::core::mem::transmute_copy(pcalleraddress)).ok()
    }
    pub unsafe fn get_ForwardTypeDestination(&self, forwardtype: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_ForwardTypeDestination)(::windows::core::Vtable::as_raw(self), forwardtype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_ForwardTypeCaller(&self, forwardtype: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_ForwardTypeCaller)(::windows::core::Vtable::as_raw(self), forwardtype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetForwardType(&self, forwardtype: i32, ppdestinationaddress: *mut ::windows::core::BSTR, ppcalleraddress: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetForwardType)(::windows::core::Vtable::as_raw(self), forwardtype, ::core::mem::transmute(ppdestinationaddress), ::core::mem::transmute(ppcalleraddress)).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Clear)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITILSConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITILSConfig {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITILSConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITILSConfig").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITLegacyAddressMediaControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITLegacyAddressMediaControl {}
impl ::core::fmt::Debug for ITLegacyAddressMediaControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITLegacyAddressMediaControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITLegacyAddressMediaControl2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITLegacyAddressMediaControl2 {}
impl ::core::fmt::Debug for ITLegacyAddressMediaControl2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITLegacyAddressMediaControl2").field(&self.0).finish()
    }
}
impl ITLegacyAddressMediaControl2 {
    pub unsafe fn GetID(&self, pdeviceclass: &::windows::core::BSTR, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pdeviceclass), pdwsize, ppdeviceid).ok()
    }
    pub unsafe fn GetDevConfig(&self, pdeviceclass: &::windows::core::BSTR, pdwsize: *mut u32, ppdeviceconfig: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDevConfig)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pdeviceclass), pdwsize, ppdeviceconfig).ok()
    }
    pub unsafe fn SetDevConfig(&self, pdeviceclass: &::windows::core::BSTR, pdeviceconfig: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDevConfig)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pdeviceclass), pdeviceconfig.len() as _, ::core::mem::transmute(pdeviceconfig.as_ptr())).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITLegacyCallMediaControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITLegacyCallMediaControl {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITLegacyCallMediaControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITLegacyCallMediaControl").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITLegacyCallMediaControl2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITLegacyCallMediaControl2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITLegacyCallMediaControl2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITLegacyCallMediaControl2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ITLegacyCallMediaControl2 {
    pub unsafe fn DetectDigits(&self, digitmode: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DetectDigits)(::windows::core::Vtable::as_raw(self), digitmode).ok()
    }
    pub unsafe fn GenerateDigits(&self, pdigits: &::windows::core::BSTR, digitmode: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GenerateDigits)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pdigits), digitmode).ok()
    }
    pub unsafe fn GetID(&self, pdeviceclass: &::windows::core::BSTR, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pdeviceclass), pdwsize, ppdeviceid).ok()
    }
    pub unsafe fn SetMediaType(&self, lmediatype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMediaType)(::windows::core::Vtable::as_raw(self), lmediatype).ok()
    }
    pub unsafe fn MonitorMedia(&self, lmediatype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MonitorMedia)(::windows::core::Vtable::as_raw(self), lmediatype).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITLegacyWaveSupport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITLegacyWaveSupport {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITLegacyWaveSupport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITLegacyWaveSupport").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITLocationInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITLocationInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITLocationInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITLocationInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITMSPAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITMSPAddress {}
impl ::core::fmt::Debug for ITMSPAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITMSPAddress").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITMediaControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITMediaControl {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITMediaControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITMediaControl").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITMediaPlayback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITMediaPlayback {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITMediaPlayback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITMediaPlayback").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITMediaRecord {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITMediaRecord {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITMediaRecord {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITMediaRecord").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITMediaSupport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITMediaSupport {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITMediaSupport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITMediaSupport").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITMultiTrackTerminal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITMultiTrackTerminal {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITMultiTrackTerminal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITMultiTrackTerminal").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITPhone {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITPhone {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITPhone {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITPhone").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITPhoneDeviceSpecificEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITPhoneDeviceSpecificEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITPhoneDeviceSpecificEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITPhoneDeviceSpecificEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITPhoneEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITPhoneEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITPhoneEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITPhoneEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITPluggableTerminalClassInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITPluggableTerminalClassInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITPluggableTerminalClassInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITPluggableTerminalClassInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITPluggableTerminalEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITPluggableTerminalEventSink {}
impl ::core::fmt::Debug for ITPluggableTerminalEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITPluggableTerminalEventSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITPluggableTerminalEventSinkRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITPluggableTerminalEventSinkRegistration {}
impl ::core::fmt::Debug for ITPluggableTerminalEventSinkRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITPluggableTerminalEventSinkRegistration").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITPluggableTerminalSuperclassInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITPluggableTerminalSuperclassInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITPluggableTerminalSuperclassInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITPluggableTerminalSuperclassInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITPrivateEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITPrivateEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITPrivateEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITPrivateEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITQOSEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITQOSEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITQOSEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITQOSEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITQueue {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITQueue").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITQueueEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITQueueEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITQueueEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITQueueEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITRendezvous {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITRendezvous {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITRendezvous {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITRendezvous").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITRequest {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITRequest").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITRequestEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITRequestEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITRequestEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITRequestEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITScriptableAudioFormat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITScriptableAudioFormat {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITScriptableAudioFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITScriptableAudioFormat").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITStaticAudioTerminal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITStaticAudioTerminal {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITStaticAudioTerminal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITStaticAudioTerminal").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITStream {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITStream").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITStreamControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITStreamControl {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITStreamControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITStreamControl").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITSubStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITSubStream {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITSubStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITSubStream").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITSubStreamControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITSubStreamControl {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITSubStreamControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITSubStreamControl").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITTAPI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITTAPI {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITTAPI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITTAPI").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITTAPI2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITTAPI2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITTAPI2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITTAPI2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ITTAPI2 {
    pub unsafe fn Initialize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Shutdown)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Addresses(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Addresses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateAddresses(&self) -> ::windows::core::Result<IEnumAddress> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerateAddresses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn RegisterCallNotifications<P0, P1, P2>(&self, paddress: P0, fmonitor: P1, fowner: P2, lmediatypes: i32, lcallbackinstance: i32) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITAddress>>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RegisterCallNotifications)(::windows::core::Vtable::as_raw(self), paddress.into().abi(), fmonitor.into(), fowner.into(), lmediatypes, lcallbackinstance, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UnregisterNotifications(&self, lregister: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnregisterNotifications)(::windows::core::Vtable::as_raw(self), lregister).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CallHubs(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CallHubs)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateCallHubs(&self) -> ::windows::core::Result<IEnumCallHub> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerateCallHubs)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetCallHubTracking<P0>(&self, paddresses: super::super::System::Com::VARIANT, btracking: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCallHubTracking)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(paddresses), btracking.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumeratePrivateTAPIObjects(&self) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumeratePrivateTAPIObjects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PrivateTAPIObjects(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PrivateTAPIObjects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterRequestRecipient<P0>(&self, lregistrationinstance: i32, lrequestmode: i32, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.RegisterRequestRecipient)(::windows::core::Vtable::as_raw(self), lregistrationinstance, lrequestmode, fenable.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAssistedTelephonyPriority<P0>(&self, pappfilename: &::windows::core::BSTR, fpriority: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetAssistedTelephonyPriority)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pappfilename), fpriority.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetApplicationPriority<P0>(&self, pappfilename: &::windows::core::BSTR, lmediatype: i32, fpriority: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetApplicationPriority)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pappfilename), lmediatype, fpriority.into()).ok()
    }
    pub unsafe fn SetEventFilter(&self, lfiltermask: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetEventFilter)(::windows::core::Vtable::as_raw(self), lfiltermask).ok()
    }
    pub unsafe fn EventFilter(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EventFilter)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITTAPICallCenter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITTAPICallCenter {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITTAPICallCenter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITTAPICallCenter").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITTAPIDispatchEventNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITTAPIDispatchEventNotification {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITTAPIDispatchEventNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITTAPIDispatchEventNotification").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITTAPIEventNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITTAPIEventNotification {}
impl ::core::fmt::Debug for ITTAPIEventNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITTAPIEventNotification").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITTAPIObjectEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITTAPIObjectEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITTAPIObjectEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITTAPIObjectEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITTAPIObjectEvent2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITTAPIObjectEvent2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITTAPIObjectEvent2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITTAPIObjectEvent2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ITTAPIObjectEvent2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TAPIObject(&self) -> ::windows::core::Result<ITTAPI> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.TAPIObject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Event(&self) -> ::windows::core::Result<TAPIOBJECT_EVENT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Event)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Address(&self) -> ::windows::core::Result<ITAddress> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Address)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CallbackInstance(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CallbackInstance)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITTTSTerminalEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITTTSTerminalEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITTTSTerminalEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITTTSTerminalEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITTerminal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITTerminal {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITTerminal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITTerminal").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITTerminalSupport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITTerminalSupport {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITTerminalSupport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITTerminalSupport").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITTerminalSupport2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITTerminalSupport2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITTerminalSupport2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITTerminalSupport2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ITTerminalSupport2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn StaticTerminals(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.StaticTerminals)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateStaticTerminals(&self) -> ::windows::core::Result<IEnumTerminal> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerateStaticTerminals)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DynamicTerminalClasses(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DynamicTerminalClasses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerateDynamicTerminalClasses(&self) -> ::windows::core::Result<IEnumTerminalClass> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerateDynamicTerminalClasses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateTerminal(&self, pterminalclass: &::windows::core::BSTR, lmediatype: i32, direction: TERMINAL_DIRECTION) -> ::windows::core::Result<ITTerminal> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateTerminal)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pterminalclass), lmediatype, direction, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDefaultStaticTerminal(&self, lmediatype: i32, direction: TERMINAL_DIRECTION) -> ::windows::core::Result<ITTerminal> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDefaultStaticTerminal)(::windows::core::Vtable::as_raw(self), lmediatype, direction, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITToneDetectionEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITToneDetectionEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITToneDetectionEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITToneDetectionEvent").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITToneTerminalEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITToneTerminalEvent {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITToneTerminalEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITToneTerminalEvent").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITnef {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITnef {}
impl ::core::fmt::Debug for ITnef {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITnef").field(&self.0).finish()
    }
}
impl ::core::default::Default for LINEADDRESSCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEADDRESSSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEAGENTACTIVITYENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEAGENTACTIVITYLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEAGENTCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEAGENTENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEAGENTGROUPENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEAGENTGROUPLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for LINEAGENTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEAGENTLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEAGENTSESSIONENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for LINEAGENTSESSIONINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEAGENTSESSIONLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEAGENTSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEAPPINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINECALLINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINECALLLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINECALLPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LINECALLSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINECALLTREATMENTENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINECARDENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINECOUNTRYENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINECOUNTRYLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEDEVCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEDEVSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEDIALPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEEXTENSIONID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEFORWARD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEFORWARDLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEGENERATETONE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LINEINITIALIZEEXPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINELOCATIONENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEMEDIACONTROLCALLSTATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEMEDIACONTROLDIGIT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEMEDIACONTROLMEDIA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEMEDIACONTROLTONE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEMESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEMONITORTONE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEPROVIDERENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEPROVIDERLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for LINEPROXYREQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEPROXYREQUESTLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEQUEUEENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEQUEUEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINEQUEUELIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LINEREQMAKECALL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LINEREQMAKECALL {
    fn eq(&self, other: &Self) -> bool {
        self.szDestAddress == other.szDestAddress && self.szAppName == other.szAppName && self.szCalledParty == other.szCalledParty && self.szComment == other.szComment
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LINEREQMAKECALL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LINEREQMAKECALL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LINEREQMAKECALL").field("szDestAddress", &self.szDestAddress).field("szAppName", &self.szAppName).field("szCalledParty", &self.szCalledParty).field("szComment", &self.szComment).finish()
    }
}
impl ::core::default::Default for LINEREQMAKECALLW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LINEREQMEDIACALL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LINEREQMEDIACALLW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINETERMCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINETRANSLATECAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LINETRANSLATEOUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MSP_ADDRESS_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MSP_ADDRESS_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSP_ADDRESS_EVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for MSP_CALL_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MSP_CALL_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSP_CALL_EVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for MSP_CALL_EVENT_CAUSE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MSP_CALL_EVENT_CAUSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSP_CALL_EVENT_CAUSE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MSP_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MSP_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSP_EVENT").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for MSP_EVENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NSID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PHONEBUTTONINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PHONECAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PHONECAPS_BUFFER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PHONECAPS_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PHONECAPS_BUFFER").field(&self.0).finish()
    }
}
impl ::core::default::Default for PHONECAPS_LONG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PHONECAPS_LONG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PHONECAPS_LONG").field(&self.0).finish()
    }
}
impl ::core::default::Default for PHONECAPS_STRING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PHONECAPS_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PHONECAPS_STRING").field(&self.0).finish()
    }
}
impl ::core::default::Default for PHONEEXTENSIONID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PHONEINITIALIZEEXPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PHONEMESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PHONESTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PHONE_BUTTON_FUNCTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PHONE_BUTTON_FUNCTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PHONE_BUTTON_FUNCTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for PHONE_BUTTON_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PHONE_BUTTON_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PHONE_BUTTON_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PHONE_BUTTON_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PHONE_BUTTON_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PHONE_BUTTON_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PHONE_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PHONE_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PHONE_EVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for PHONE_HOOK_SWITCH_DEVICE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PHONE_HOOK_SWITCH_DEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PHONE_HOOK_SWITCH_DEVICE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PHONE_HOOK_SWITCH_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PHONE_HOOK_SWITCH_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PHONE_HOOK_SWITCH_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PHONE_LAMP_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PHONE_LAMP_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PHONE_LAMP_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PHONE_PRIVILEGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PHONE_PRIVILEGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PHONE_PRIVILEGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PHONE_TONE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PHONE_TONE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PHONE_TONE").field(&self.0).finish()
    }
}
impl ::core::default::Default for QOS_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for QOS_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QOS_EVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for QOS_SERVICE_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for QOS_SERVICE_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QOS_SERVICE_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for RENDDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RND_ADVERTISING_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RND_ADVERTISING_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RND_ADVERTISING_SCOPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for STnefProblem {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STnefProblem {
    fn eq(&self, other: &Self) -> bool {
        self.ulComponent == other.ulComponent && self.ulAttribute == other.ulAttribute && self.ulPropTag == other.ulPropTag && self.scode == other.scode
    }
}
impl ::core::cmp::Eq for STnefProblem {}
impl ::core::fmt::Debug for STnefProblem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STnefProblem").field("ulComponent", &self.ulComponent).field("ulAttribute", &self.ulAttribute).field("ulPropTag", &self.ulPropTag).field("scode", &self.scode).finish()
    }
}
impl ::core::default::Default for STnefProblemArray {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STnefProblemArray {
    fn eq(&self, other: &Self) -> bool {
        self.cProblem == other.cProblem && self.aProblem == other.aProblem
    }
}
impl ::core::cmp::Eq for STnefProblemArray {}
impl ::core::fmt::Debug for STnefProblemArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STnefProblemArray").field("cProblem", &self.cProblem).field("aProblem", &self.aProblem).finish()
    }
}
impl ::core::default::Default for TAPIOBJECT_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TAPIOBJECT_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TAPIOBJECT_EVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for TAPI_CUSTOMTONE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TAPI_CUSTOMTONE {
    fn eq(&self, other: &Self) -> bool {
        self.dwFrequency == other.dwFrequency && self.dwCadenceOn == other.dwCadenceOn && self.dwCadenceOff == other.dwCadenceOff && self.dwVolume == other.dwVolume
    }
}
impl ::core::cmp::Eq for TAPI_CUSTOMTONE {}
impl ::core::fmt::Debug for TAPI_CUSTOMTONE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAPI_CUSTOMTONE").field("dwFrequency", &self.dwFrequency).field("dwCadenceOn", &self.dwCadenceOn).field("dwCadenceOff", &self.dwCadenceOff).field("dwVolume", &self.dwVolume).finish()
    }
}
impl ::core::default::Default for TAPI_DETECTTONE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TAPI_DETECTTONE {
    fn eq(&self, other: &Self) -> bool {
        self.dwAppSpecific == other.dwAppSpecific && self.dwDuration == other.dwDuration && self.dwFrequency1 == other.dwFrequency1 && self.dwFrequency2 == other.dwFrequency2 && self.dwFrequency3 == other.dwFrequency3
    }
}
impl ::core::cmp::Eq for TAPI_DETECTTONE {}
impl ::core::fmt::Debug for TAPI_DETECTTONE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAPI_DETECTTONE").field("dwAppSpecific", &self.dwAppSpecific).field("dwDuration", &self.dwDuration).field("dwFrequency1", &self.dwFrequency1).field("dwFrequency2", &self.dwFrequency2).field("dwFrequency3", &self.dwFrequency3).finish()
    }
}
impl ::core::default::Default for TAPI_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TAPI_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TAPI_EVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for TAPI_GATHERTERM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TAPI_GATHERTERM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TAPI_GATHERTERM").field(&self.0).finish()
    }
}
impl ::core::default::Default for TAPI_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TAPI_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TAPI_OBJECT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TAPI_TONEMODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TAPI_TONEMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TAPI_TONEMODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TERMINAL_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TERMINAL_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TERMINAL_DIRECTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for TERMINAL_MEDIA_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TERMINAL_MEDIA_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TERMINAL_MEDIA_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TERMINAL_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TERMINAL_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TERMINAL_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TERMINAL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TERMINAL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TERMINAL_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TRP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRP {
    fn eq(&self, other: &Self) -> bool {
        self.trpid == other.trpid && self.cbgrtrp == other.cbgrtrp && self.cch == other.cch && self.cbRgb == other.cbRgb
    }
}
impl ::core::cmp::Eq for TRP {}
impl ::core::fmt::Debug for TRP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRP").field("trpid", &self.trpid).field("cbgrtrp", &self.cbgrtrp).field("cch", &self.cch).field("cbRgb", &self.cbRgb).finish()
    }
}
impl ::core::default::Default for TUISPICREATEDIALOGINSTANCEPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TUISPICREATEDIALOGINSTANCEPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwRequestID == other.dwRequestID && self.hdDlgInst == other.hdDlgInst && self.htDlgInst == other.htDlgInst && self.lpszUIDLLName == other.lpszUIDLLName && self.lpParams == other.lpParams && self.dwSize == other.dwSize
    }
}
impl ::core::cmp::Eq for TUISPICREATEDIALOGINSTANCEPARAMS {}
impl ::core::fmt::Debug for TUISPICREATEDIALOGINSTANCEPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TUISPICREATEDIALOGINSTANCEPARAMS").field("dwRequestID", &self.dwRequestID).field("hdDlgInst", &self.hdDlgInst).field("htDlgInst", &self.htDlgInst).field("lpszUIDLLName", &self.lpszUIDLLName).field("lpParams", &self.lpParams).field("dwSize", &self.dwSize).finish()
    }
}
impl ::core::default::Default for VARSTRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
