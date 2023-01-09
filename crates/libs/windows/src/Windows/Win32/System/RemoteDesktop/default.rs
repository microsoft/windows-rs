impl ::core::default::Default for AAAccountingData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AAAccountingData {
    fn eq(&self, other: &Self) -> bool {
        self.userName == other.userName && self.clientName == other.clientName && self.authType == other.authType && self.resourceName == other.resourceName && self.portNumber == other.portNumber && self.protocolName == other.protocolName && self.numberOfBytesReceived == other.numberOfBytesReceived && self.numberOfBytesTransfered == other.numberOfBytesTransfered && self.reasonForDisconnect == other.reasonForDisconnect && self.mainSessionId == other.mainSessionId && self.subSessionId == other.subSessionId
    }
}
impl ::core::cmp::Eq for AAAccountingData {}
impl ::core::fmt::Debug for AAAccountingData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AAAccountingData")
            .field("userName", &self.userName)
            .field("clientName", &self.clientName)
            .field("authType", &self.authType)
            .field("resourceName", &self.resourceName)
            .field("portNumber", &self.portNumber)
            .field("protocolName", &self.protocolName)
            .field("numberOfBytesReceived", &self.numberOfBytesReceived)
            .field("numberOfBytesTransfered", &self.numberOfBytesTransfered)
            .field("reasonForDisconnect", &self.reasonForDisconnect)
            .field("mainSessionId", &self.mainSessionId)
            .field("subSessionId", &self.subSessionId)
            .finish()
    }
}
impl ::core::default::Default for AAAccountingDataType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AAAccountingDataType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AAAccountingDataType").field(&self.0).finish()
    }
}
impl ::core::default::Default for AAAuthSchemes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AAAuthSchemes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AAAuthSchemes").field(&self.0).finish()
    }
}
impl ::core::default::Default for AATrustClassID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AATrustClassID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AATrustClassID").field(&self.0).finish()
    }
}
impl ::core::default::Default for AE_CURRENT_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AE_CURRENT_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.u64DevicePosition == other.u64DevicePosition && self.u64StreamPosition == other.u64StreamPosition && self.u64PaddingFrames == other.u64PaddingFrames && self.hnsQPCPosition == other.hnsQPCPosition && self.f32FramesPerSecond == other.f32FramesPerSecond && self.Flag == other.Flag
    }
}
impl ::core::cmp::Eq for AE_CURRENT_POSITION {}
impl ::core::fmt::Debug for AE_CURRENT_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AE_CURRENT_POSITION").field("u64DevicePosition", &self.u64DevicePosition).field("u64StreamPosition", &self.u64StreamPosition).field("u64PaddingFrames", &self.u64PaddingFrames).field("hnsQPCPosition", &self.hnsQPCPosition).field("f32FramesPerSecond", &self.f32FramesPerSecond).field("Flag", &self.Flag).finish()
    }
}
impl ::core::default::Default for AE_POSITION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AE_POSITION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AE_POSITION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for BITMAP_RENDERER_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BITMAP_RENDERER_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.dwFramesDelivered == other.dwFramesDelivered && self.dwFramesDropped == other.dwFramesDropped
    }
}
impl ::core::cmp::Eq for BITMAP_RENDERER_STATISTICS {}
impl ::core::fmt::Debug for BITMAP_RENDERER_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BITMAP_RENDERER_STATISTICS").field("dwFramesDelivered", &self.dwFramesDelivered).field("dwFramesDropped", &self.dwFramesDropped).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHANNEL_DEF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHANNEL_ENTRY_POINTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CHANNEL_PDU_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CHANNEL_PDU_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for CHANNEL_PDU_HEADER {}
impl ::core::fmt::Debug for CHANNEL_PDU_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANNEL_PDU_HEADER").field("length", &self.length).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for CLIENT_DISPLAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLIENT_DISPLAY {
    fn eq(&self, other: &Self) -> bool {
        self.HorizontalResolution == other.HorizontalResolution && self.VerticalResolution == other.VerticalResolution && self.ColorDepth == other.ColorDepth
    }
}
impl ::core::cmp::Eq for CLIENT_DISPLAY {}
impl ::core::fmt::Debug for CLIENT_DISPLAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLIENT_DISPLAY").field("HorizontalResolution", &self.HorizontalResolution).field("VerticalResolution", &self.VerticalResolution).field("ColorDepth", &self.ColorDepth).finish()
    }
}
impl ::core::default::Default for CLIENT_MESSAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLIENT_MESSAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLIENT_MESSAGE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CONNECTION_CHANGE_NOTIFICATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CONNECTION_CHANGE_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONNECTION_CHANGE_NOTIFICATION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IADsTSUserEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IADsTSUserEx {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IADsTSUserEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADsTSUserEx").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioDeviceEndpoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioDeviceEndpoint {}
impl ::core::fmt::Debug for IAudioDeviceEndpoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioDeviceEndpoint").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioEndpoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEndpoint {}
impl ::core::fmt::Debug for IAudioEndpoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEndpoint").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioEndpointControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEndpointControl {}
impl ::core::fmt::Debug for IAudioEndpointControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEndpointControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioEndpointRT {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioEndpointRT {}
impl ::core::fmt::Debug for IAudioEndpointRT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioEndpointRT").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioInputEndpointRT {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioInputEndpointRT {}
impl ::core::fmt::Debug for IAudioInputEndpointRT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioInputEndpointRT").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioOutputEndpointRT {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioOutputEndpointRT {}
impl ::core::fmt::Debug for IAudioOutputEndpointRT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioOutputEndpointRT").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRemoteDesktopClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRemoteDesktopClient {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRemoteDesktopClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRemoteDesktopClient").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRemoteDesktopClientActions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRemoteDesktopClientActions {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRemoteDesktopClientActions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRemoteDesktopClientActions").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRemoteDesktopClientSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRemoteDesktopClientSettings {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRemoteDesktopClientSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRemoteDesktopClientSettings").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IRemoteDesktopClientTouchPointer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IRemoteDesktopClientTouchPointer {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IRemoteDesktopClientTouchPointer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRemoteDesktopClientTouchPointer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRemoteSystemAdditionalInfoProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRemoteSystemAdditionalInfoProvider {}
impl ::core::fmt::Debug for IRemoteSystemAdditionalInfoProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRemoteSystemAdditionalInfoProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITSGAccountingEngine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITSGAccountingEngine {}
impl ::core::fmt::Debug for ITSGAccountingEngine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITSGAccountingEngine").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITSGAuthenticateUserSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITSGAuthenticateUserSink {}
impl ::core::fmt::Debug for ITSGAuthenticateUserSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITSGAuthenticateUserSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITSGAuthenticationEngine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITSGAuthenticationEngine {}
impl ::core::fmt::Debug for ITSGAuthenticationEngine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITSGAuthenticationEngine").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITSGAuthorizeConnectionSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITSGAuthorizeConnectionSink {}
impl ::core::fmt::Debug for ITSGAuthorizeConnectionSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITSGAuthorizeConnectionSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITSGAuthorizeResourceSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITSGAuthorizeResourceSink {}
impl ::core::fmt::Debug for ITSGAuthorizeResourceSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITSGAuthorizeResourceSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITSGPolicyEngine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITSGPolicyEngine {}
impl ::core::fmt::Debug for ITSGPolicyEngine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITSGPolicyEngine").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITsSbBaseNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbBaseNotifySink {}
impl ::core::fmt::Debug for ITsSbBaseNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbBaseNotifySink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITsSbClientConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbClientConnection {}
impl ::core::fmt::Debug for ITsSbClientConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbClientConnection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::cmp::PartialEq for ITsSbClientConnectionPropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::cmp::Eq for ITsSbClientConnectionPropertySet {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::fmt::Debug for ITsSbClientConnectionPropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbClientConnectionPropertySet").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbClientConnectionPropertySet {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn Read<P0, P1>(&self, pszpropname: P0, pvar: *mut super::Com::VARIANT, perrorlog: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IErrorLog>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Read)(::windows::core::Vtable::as_raw(self), pszpropname.into().abi(), pvar, perrorlog.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn Write<P0>(&self, pszpropname: P0, pvar: *const super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Write)(::windows::core::Vtable::as_raw(self), pszpropname.into().abi(), pvar).ok()
    }
}
impl ::core::cmp::PartialEq for ITsSbEnvironment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbEnvironment {}
impl ::core::fmt::Debug for ITsSbEnvironment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbEnvironment").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::cmp::PartialEq for ITsSbEnvironmentPropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::cmp::Eq for ITsSbEnvironmentPropertySet {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::fmt::Debug for ITsSbEnvironmentPropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbEnvironmentPropertySet").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbEnvironmentPropertySet {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn Read<P0, P1>(&self, pszpropname: P0, pvar: *mut super::Com::VARIANT, perrorlog: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IErrorLog>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Read)(::windows::core::Vtable::as_raw(self), pszpropname.into().abi(), pvar, perrorlog.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn Write<P0>(&self, pszpropname: P0, pvar: *const super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Write)(::windows::core::Vtable::as_raw(self), pszpropname.into().abi(), pvar).ok()
    }
}
impl ::core::cmp::PartialEq for ITsSbFilterPluginStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbFilterPluginStore {}
impl ::core::fmt::Debug for ITsSbFilterPluginStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbFilterPluginStore").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITsSbGenericNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbGenericNotifySink {}
impl ::core::fmt::Debug for ITsSbGenericNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbGenericNotifySink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITsSbGlobalStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbGlobalStore {}
impl ::core::fmt::Debug for ITsSbGlobalStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbGlobalStore").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITsSbLoadBalanceResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbLoadBalanceResult {}
impl ::core::fmt::Debug for ITsSbLoadBalanceResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbLoadBalanceResult").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITsSbLoadBalancing {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbLoadBalancing {}
impl ::core::fmt::Debug for ITsSbLoadBalancing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbLoadBalancing").field(&self.0).finish()
    }
}
impl ITsSbLoadBalancing {
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Initialize<P0, P1, P2>(&self, pprovider: P0, pnotifysink: P1, ppropertyset: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITsSbProvider>>,
        P1: ::std::convert::Into<::windows::core::InParam<ITsSbPluginNotifySink>>,
        P2: ::std::convert::Into<::windows::core::InParam<ITsSbPluginPropertySet>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pprovider.into().abi(), pnotifysink.into().abi(), ppropertyset.into().abi()).ok()
    }
    pub unsafe fn Terminate(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Terminate)(::windows::core::Vtable::as_raw(self), hr).ok()
    }
}
impl ::core::cmp::PartialEq for ITsSbLoadBalancingNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbLoadBalancingNotifySink {}
impl ::core::fmt::Debug for ITsSbLoadBalancingNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbLoadBalancingNotifySink").field(&self.0).finish()
    }
}
impl ITsSbLoadBalancingNotifySink {
    pub unsafe fn OnError(&self, hrerror: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnError)(::windows::core::Vtable::as_raw(self), hrerror).ok()
    }
    pub unsafe fn OnReportStatus(&self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnReportStatus)(::windows::core::Vtable::as_raw(self), messagetype, messageid).ok()
    }
}
impl ::core::cmp::PartialEq for ITsSbOrchestration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbOrchestration {}
impl ::core::fmt::Debug for ITsSbOrchestration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbOrchestration").field(&self.0).finish()
    }
}
impl ITsSbOrchestration {
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Initialize<P0, P1, P2>(&self, pprovider: P0, pnotifysink: P1, ppropertyset: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITsSbProvider>>,
        P1: ::std::convert::Into<::windows::core::InParam<ITsSbPluginNotifySink>>,
        P2: ::std::convert::Into<::windows::core::InParam<ITsSbPluginPropertySet>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pprovider.into().abi(), pnotifysink.into().abi(), ppropertyset.into().abi()).ok()
    }
    pub unsafe fn Terminate(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Terminate)(::windows::core::Vtable::as_raw(self), hr).ok()
    }
}
impl ::core::cmp::PartialEq for ITsSbOrchestrationNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbOrchestrationNotifySink {}
impl ::core::fmt::Debug for ITsSbOrchestrationNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbOrchestrationNotifySink").field(&self.0).finish()
    }
}
impl ITsSbOrchestrationNotifySink {
    pub unsafe fn OnError(&self, hrerror: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnError)(::windows::core::Vtable::as_raw(self), hrerror).ok()
    }
    pub unsafe fn OnReportStatus(&self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnReportStatus)(::windows::core::Vtable::as_raw(self), messagetype, messageid).ok()
    }
}
impl ::core::cmp::PartialEq for ITsSbPlacement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbPlacement {}
impl ::core::fmt::Debug for ITsSbPlacement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbPlacement").field(&self.0).finish()
    }
}
impl ITsSbPlacement {
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Initialize<P0, P1, P2>(&self, pprovider: P0, pnotifysink: P1, ppropertyset: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITsSbProvider>>,
        P1: ::std::convert::Into<::windows::core::InParam<ITsSbPluginNotifySink>>,
        P2: ::std::convert::Into<::windows::core::InParam<ITsSbPluginPropertySet>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pprovider.into().abi(), pnotifysink.into().abi(), ppropertyset.into().abi()).ok()
    }
    pub unsafe fn Terminate(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Terminate)(::windows::core::Vtable::as_raw(self), hr).ok()
    }
}
impl ::core::cmp::PartialEq for ITsSbPlacementNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbPlacementNotifySink {}
impl ::core::fmt::Debug for ITsSbPlacementNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbPlacementNotifySink").field(&self.0).finish()
    }
}
impl ITsSbPlacementNotifySink {
    pub unsafe fn OnError(&self, hrerror: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnError)(::windows::core::Vtable::as_raw(self), hrerror).ok()
    }
    pub unsafe fn OnReportStatus(&self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnReportStatus)(::windows::core::Vtable::as_raw(self), messagetype, messageid).ok()
    }
}
impl ::core::cmp::PartialEq for ITsSbPlugin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbPlugin {}
impl ::core::fmt::Debug for ITsSbPlugin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbPlugin").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITsSbPluginNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbPluginNotifySink {}
impl ::core::fmt::Debug for ITsSbPluginNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbPluginNotifySink").field(&self.0).finish()
    }
}
impl ITsSbPluginNotifySink {
    pub unsafe fn OnError(&self, hrerror: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnError)(::windows::core::Vtable::as_raw(self), hrerror).ok()
    }
    pub unsafe fn OnReportStatus(&self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnReportStatus)(::windows::core::Vtable::as_raw(self), messagetype, messageid).ok()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::cmp::PartialEq for ITsSbPluginPropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::cmp::Eq for ITsSbPluginPropertySet {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::fmt::Debug for ITsSbPluginPropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbPluginPropertySet").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbPluginPropertySet {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn Read<P0, P1>(&self, pszpropname: P0, pvar: *mut super::Com::VARIANT, perrorlog: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IErrorLog>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Read)(::windows::core::Vtable::as_raw(self), pszpropname.into().abi(), pvar, perrorlog.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn Write<P0>(&self, pszpropname: P0, pvar: *const super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Write)(::windows::core::Vtable::as_raw(self), pszpropname.into().abi(), pvar).ok()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::cmp::PartialEq for ITsSbPropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::cmp::Eq for ITsSbPropertySet {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::fmt::Debug for ITsSbPropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbPropertySet").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbPropertySet {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn Read<P0, P1>(&self, pszpropname: P0, pvar: *mut super::Com::VARIANT, perrorlog: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IErrorLog>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Read)(::windows::core::Vtable::as_raw(self), pszpropname.into().abi(), pvar, perrorlog.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn Write<P0>(&self, pszpropname: P0, pvar: *const super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Write)(::windows::core::Vtable::as_raw(self), pszpropname.into().abi(), pvar).ok()
    }
}
impl ::core::cmp::PartialEq for ITsSbProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbProvider {}
impl ::core::fmt::Debug for ITsSbProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITsSbProvisioning {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbProvisioning {}
impl ::core::fmt::Debug for ITsSbProvisioning {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbProvisioning").field(&self.0).finish()
    }
}
impl ITsSbProvisioning {
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Initialize<P0, P1, P2>(&self, pprovider: P0, pnotifysink: P1, ppropertyset: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITsSbProvider>>,
        P1: ::std::convert::Into<::windows::core::InParam<ITsSbPluginNotifySink>>,
        P2: ::std::convert::Into<::windows::core::InParam<ITsSbPluginPropertySet>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pprovider.into().abi(), pnotifysink.into().abi(), ppropertyset.into().abi()).ok()
    }
    pub unsafe fn Terminate(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Terminate)(::windows::core::Vtable::as_raw(self), hr).ok()
    }
}
impl ::core::cmp::PartialEq for ITsSbProvisioningPluginNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbProvisioningPluginNotifySink {}
impl ::core::fmt::Debug for ITsSbProvisioningPluginNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbProvisioningPluginNotifySink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITsSbResourceNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbResourceNotification {}
impl ::core::fmt::Debug for ITsSbResourceNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbResourceNotification").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITsSbResourceNotificationEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbResourceNotificationEx {}
impl ::core::fmt::Debug for ITsSbResourceNotificationEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbResourceNotificationEx").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITsSbResourcePlugin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbResourcePlugin {}
impl ::core::fmt::Debug for ITsSbResourcePlugin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbResourcePlugin").field(&self.0).finish()
    }
}
impl ITsSbResourcePlugin {
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Initialize<P0, P1, P2>(&self, pprovider: P0, pnotifysink: P1, ppropertyset: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITsSbProvider>>,
        P1: ::std::convert::Into<::windows::core::InParam<ITsSbPluginNotifySink>>,
        P2: ::std::convert::Into<::windows::core::InParam<ITsSbPluginPropertySet>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pprovider.into().abi(), pnotifysink.into().abi(), ppropertyset.into().abi()).ok()
    }
    pub unsafe fn Terminate(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Terminate)(::windows::core::Vtable::as_raw(self), hr).ok()
    }
}
impl ::core::cmp::PartialEq for ITsSbResourcePluginStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbResourcePluginStore {}
impl ::core::fmt::Debug for ITsSbResourcePluginStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbResourcePluginStore").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITsSbServiceNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbServiceNotification {}
impl ::core::fmt::Debug for ITsSbServiceNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbServiceNotification").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITsSbSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbSession {}
impl ::core::fmt::Debug for ITsSbSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbSession").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITsSbTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbTarget {}
impl ::core::fmt::Debug for ITsSbTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbTarget").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::cmp::PartialEq for ITsSbTargetPropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::cmp::Eq for ITsSbTargetPropertySet {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::fmt::Debug for ITsSbTargetPropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbTargetPropertySet").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ITsSbTargetPropertySet {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn Read<P0, P1>(&self, pszpropname: P0, pvar: *mut super::Com::VARIANT, perrorlog: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IErrorLog>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Read)(::windows::core::Vtable::as_raw(self), pszpropname.into().abi(), pvar, perrorlog.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub unsafe fn Write<P0>(&self, pszpropname: P0, pvar: *const super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Write)(::windows::core::Vtable::as_raw(self), pszpropname.into().abi(), pvar).ok()
    }
}
impl ::core::cmp::PartialEq for ITsSbTaskInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbTaskInfo {}
impl ::core::fmt::Debug for ITsSbTaskInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbTaskInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITsSbTaskPlugin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbTaskPlugin {}
impl ::core::fmt::Debug for ITsSbTaskPlugin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbTaskPlugin").field(&self.0).finish()
    }
}
impl ITsSbTaskPlugin {
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Initialize<P0, P1, P2>(&self, pprovider: P0, pnotifysink: P1, ppropertyset: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITsSbProvider>>,
        P1: ::std::convert::Into<::windows::core::InParam<ITsSbPluginNotifySink>>,
        P2: ::std::convert::Into<::windows::core::InParam<ITsSbPluginPropertySet>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pprovider.into().abi(), pnotifysink.into().abi(), ppropertyset.into().abi()).ok()
    }
    pub unsafe fn Terminate(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Terminate)(::windows::core::Vtable::as_raw(self), hr).ok()
    }
}
impl ::core::cmp::PartialEq for ITsSbTaskPluginNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITsSbTaskPluginNotifySink {}
impl ::core::fmt::Debug for ITsSbTaskPluginNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITsSbTaskPluginNotifySink").field(&self.0).finish()
    }
}
impl ITsSbTaskPluginNotifySink {
    pub unsafe fn OnError(&self, hrerror: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnError)(::windows::core::Vtable::as_raw(self), hrerror).ok()
    }
    pub unsafe fn OnReportStatus(&self, messagetype: CLIENT_MESSAGE_TYPE, messageid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnReportStatus)(::windows::core::Vtable::as_raw(self), messagetype, messageid).ok()
    }
}
impl ::core::cmp::PartialEq for IWRdsEnhancedFastReconnectArbitrator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsEnhancedFastReconnectArbitrator {}
impl ::core::fmt::Debug for IWRdsEnhancedFastReconnectArbitrator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsEnhancedFastReconnectArbitrator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWRdsGraphicsChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsGraphicsChannel {}
impl ::core::fmt::Debug for IWRdsGraphicsChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsGraphicsChannel").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWRdsGraphicsChannelEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsGraphicsChannelEvents {}
impl ::core::fmt::Debug for IWRdsGraphicsChannelEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsGraphicsChannelEvents").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWRdsGraphicsChannelManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsGraphicsChannelManager {}
impl ::core::fmt::Debug for IWRdsGraphicsChannelManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsGraphicsChannelManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWRdsProtocolConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsProtocolConnection {}
impl ::core::fmt::Debug for IWRdsProtocolConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsProtocolConnection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWRdsProtocolConnectionCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsProtocolConnectionCallback {}
impl ::core::fmt::Debug for IWRdsProtocolConnectionCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsProtocolConnectionCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWRdsProtocolConnectionSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsProtocolConnectionSettings {}
impl ::core::fmt::Debug for IWRdsProtocolConnectionSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsProtocolConnectionSettings").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWRdsProtocolLicenseConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsProtocolLicenseConnection {}
impl ::core::fmt::Debug for IWRdsProtocolLicenseConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsProtocolLicenseConnection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWRdsProtocolListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsProtocolListener {}
impl ::core::fmt::Debug for IWRdsProtocolListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsProtocolListener").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWRdsProtocolListenerCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsProtocolListenerCallback {}
impl ::core::fmt::Debug for IWRdsProtocolListenerCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsProtocolListenerCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWRdsProtocolLogonErrorRedirector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsProtocolLogonErrorRedirector {}
impl ::core::fmt::Debug for IWRdsProtocolLogonErrorRedirector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsProtocolLogonErrorRedirector").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWRdsProtocolManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsProtocolManager {}
impl ::core::fmt::Debug for IWRdsProtocolManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsProtocolManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWRdsProtocolSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsProtocolSettings {}
impl ::core::fmt::Debug for IWRdsProtocolSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsProtocolSettings").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWRdsProtocolShadowCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsProtocolShadowCallback {}
impl ::core::fmt::Debug for IWRdsProtocolShadowCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsProtocolShadowCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWRdsProtocolShadowConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsProtocolShadowConnection {}
impl ::core::fmt::Debug for IWRdsProtocolShadowConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsProtocolShadowConnection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWRdsWddmIddProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWRdsWddmIddProps {}
impl ::core::fmt::Debug for IWRdsWddmIddProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWRdsWddmIddProps").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWTSBitmapRenderService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSBitmapRenderService {}
impl ::core::fmt::Debug for IWTSBitmapRenderService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSBitmapRenderService").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWTSBitmapRenderer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSBitmapRenderer {}
impl ::core::fmt::Debug for IWTSBitmapRenderer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSBitmapRenderer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWTSBitmapRendererCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSBitmapRendererCallback {}
impl ::core::fmt::Debug for IWTSBitmapRendererCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSBitmapRendererCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWTSListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSListener {}
impl ::core::fmt::Debug for IWTSListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSListener").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWTSListenerCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSListenerCallback {}
impl ::core::fmt::Debug for IWTSListenerCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSListenerCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWTSPlugin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSPlugin {}
impl ::core::fmt::Debug for IWTSPlugin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSPlugin").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWTSPluginServiceProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSPluginServiceProvider {}
impl ::core::fmt::Debug for IWTSPluginServiceProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSPluginServiceProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWTSProtocolConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSProtocolConnection {}
impl ::core::fmt::Debug for IWTSProtocolConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSProtocolConnection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWTSProtocolConnectionCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSProtocolConnectionCallback {}
impl ::core::fmt::Debug for IWTSProtocolConnectionCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSProtocolConnectionCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWTSProtocolLicenseConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSProtocolLicenseConnection {}
impl ::core::fmt::Debug for IWTSProtocolLicenseConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSProtocolLicenseConnection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWTSProtocolListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSProtocolListener {}
impl ::core::fmt::Debug for IWTSProtocolListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSProtocolListener").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWTSProtocolListenerCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSProtocolListenerCallback {}
impl ::core::fmt::Debug for IWTSProtocolListenerCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSProtocolListenerCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWTSProtocolLogonErrorRedirector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSProtocolLogonErrorRedirector {}
impl ::core::fmt::Debug for IWTSProtocolLogonErrorRedirector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSProtocolLogonErrorRedirector").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWTSProtocolManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSProtocolManager {}
impl ::core::fmt::Debug for IWTSProtocolManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSProtocolManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWTSProtocolShadowCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSProtocolShadowCallback {}
impl ::core::fmt::Debug for IWTSProtocolShadowCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSProtocolShadowCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWTSProtocolShadowConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSProtocolShadowConnection {}
impl ::core::fmt::Debug for IWTSProtocolShadowConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSProtocolShadowConnection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWTSSBPlugin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSSBPlugin {}
impl ::core::fmt::Debug for IWTSSBPlugin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSSBPlugin").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWTSVirtualChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSVirtualChannel {}
impl ::core::fmt::Debug for IWTSVirtualChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSVirtualChannel").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWTSVirtualChannelCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSVirtualChannelCallback {}
impl ::core::fmt::Debug for IWTSVirtualChannelCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSVirtualChannelCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWTSVirtualChannelManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWTSVirtualChannelManager {}
impl ::core::fmt::Debug for IWTSVirtualChannelManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWTSVirtualChannelManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWorkspace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWorkspace {}
impl ::core::fmt::Debug for IWorkspace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWorkspace").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWorkspace2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWorkspace2 {}
impl ::core::fmt::Debug for IWorkspace2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWorkspace2").field(&self.0).finish()
    }
}
impl IWorkspace2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWorkspaceNames(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWorkspaceNames)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn StartRemoteApplication(&self, bstrworkspaceid: &::windows::core::BSTR, psaparams: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.StartRemoteApplication)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrworkspaceid), psaparams).ok()
    }
    pub unsafe fn GetProcessId(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetProcessId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWorkspace3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWorkspace3 {}
impl ::core::fmt::Debug for IWorkspace3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWorkspace3").field(&self.0).finish()
    }
}
impl IWorkspace3 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWorkspaceNames(&self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetWorkspaceNames)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn StartRemoteApplication(&self, bstrworkspaceid: &::windows::core::BSTR, psaparams: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.StartRemoteApplication)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrworkspaceid), psaparams).ok()
    }
    pub unsafe fn GetProcessId(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetProcessId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn StartRemoteApplicationEx<P0>(&self, bstrworkspaceid: &::windows::core::BSTR, bstrrequestingappid: &::windows::core::BSTR, bstrrequestingappfamilyname: &::windows::core::BSTR, blaunchintoimmersiveclient: P0, bstrimmersiveclientactivationcontext: &::windows::core::BSTR, psaparams: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.StartRemoteApplicationEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrworkspaceid), ::core::mem::transmute_copy(bstrrequestingappid), ::core::mem::transmute_copy(bstrrequestingappfamilyname), blaunchintoimmersiveclient.into(), ::core::mem::transmute_copy(bstrimmersiveclientactivationcontext), psaparams).ok()
    }
}
impl ::core::cmp::PartialEq for IWorkspaceClientExt {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWorkspaceClientExt {}
impl ::core::fmt::Debug for IWorkspaceClientExt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWorkspaceClientExt").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWorkspaceRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWorkspaceRegistration {}
impl ::core::fmt::Debug for IWorkspaceRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWorkspaceRegistration").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWorkspaceRegistration2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWorkspaceRegistration2 {}
impl ::core::fmt::Debug for IWorkspaceRegistration2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWorkspaceRegistration2").field(&self.0).finish()
    }
}
impl IWorkspaceRegistration2 {
    pub unsafe fn AddResource<P0>(&self, punk: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWorkspaceClientExt>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.AddResource)(::windows::core::Vtable::as_raw(self), punk.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RemoveResource(&self, dwcookieconnection: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RemoveResource)(::windows::core::Vtable::as_raw(self), dwcookieconnection).ok()
    }
}
impl ::core::cmp::PartialEq for IWorkspaceReportMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWorkspaceReportMessage {}
impl ::core::fmt::Debug for IWorkspaceReportMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWorkspaceReportMessage").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWorkspaceResTypeRegistry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWorkspaceResTypeRegistry {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWorkspaceResTypeRegistry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWorkspaceResTypeRegistry").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWorkspaceScriptable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWorkspaceScriptable {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWorkspaceScriptable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWorkspaceScriptable").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWorkspaceScriptable2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWorkspaceScriptable2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWorkspaceScriptable2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWorkspaceScriptable2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWorkspaceScriptable2 {
    pub unsafe fn DisconnectWorkspace(&self, bstrworkspaceid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DisconnectWorkspace)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrworkspaceid)).ok()
    }
    pub unsafe fn StartWorkspace(&self, bstrworkspaceid: &::windows::core::BSTR, bstrusername: &::windows::core::BSTR, bstrpassword: &::windows::core::BSTR, bstrworkspaceparams: &::windows::core::BSTR, ltimeout: i32, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.StartWorkspace)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrworkspaceid), ::core::mem::transmute_copy(bstrusername), ::core::mem::transmute_copy(bstrpassword), ::core::mem::transmute_copy(bstrworkspaceparams), ltimeout, lflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWorkspaceCredentialSpecified<P0>(&self, bstrworkspaceid: &::windows::core::BSTR, bcountunauthenticatedcredentials: P0) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsWorkspaceCredentialSpecified)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrworkspaceid), bcountunauthenticatedcredentials.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWorkspaceSSOEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsWorkspaceSSOEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ClearWorkspaceCredential(&self, bstrworkspaceid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ClearWorkspaceCredential)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrworkspaceid)).ok()
    }
    pub unsafe fn OnAuthenticated(&self, bstrworkspaceid: &::windows::core::BSTR, bstrusername: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnAuthenticated)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrworkspaceid), ::core::mem::transmute_copy(bstrusername)).ok()
    }
    pub unsafe fn DisconnectWorkspaceByFriendlyName(&self, bstrworkspacefriendlyname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DisconnectWorkspaceByFriendlyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrworkspacefriendlyname)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWorkspaceScriptable3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWorkspaceScriptable3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWorkspaceScriptable3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWorkspaceScriptable3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWorkspaceScriptable3 {
    pub unsafe fn DisconnectWorkspace(&self, bstrworkspaceid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.DisconnectWorkspace)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrworkspaceid)).ok()
    }
    pub unsafe fn StartWorkspace(&self, bstrworkspaceid: &::windows::core::BSTR, bstrusername: &::windows::core::BSTR, bstrpassword: &::windows::core::BSTR, bstrworkspaceparams: &::windows::core::BSTR, ltimeout: i32, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.StartWorkspace)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrworkspaceid), ::core::mem::transmute_copy(bstrusername), ::core::mem::transmute_copy(bstrpassword), ::core::mem::transmute_copy(bstrworkspaceparams), ltimeout, lflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWorkspaceCredentialSpecified<P0>(&self, bstrworkspaceid: &::windows::core::BSTR, bcountunauthenticatedcredentials: P0) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.IsWorkspaceCredentialSpecified)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrworkspaceid), bcountunauthenticatedcredentials.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWorkspaceSSOEnabled(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.IsWorkspaceSSOEnabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ClearWorkspaceCredential(&self, bstrworkspaceid: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ClearWorkspaceCredential)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrworkspaceid)).ok()
    }
    pub unsafe fn OnAuthenticated(&self, bstrworkspaceid: &::windows::core::BSTR, bstrusername: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.OnAuthenticated)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrworkspaceid), ::core::mem::transmute_copy(bstrusername)).ok()
    }
    pub unsafe fn DisconnectWorkspaceByFriendlyName(&self, bstrworkspacefriendlyname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.DisconnectWorkspaceByFriendlyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrworkspacefriendlyname)).ok()
    }
    pub unsafe fn StartWorkspaceEx(&self, bstrworkspaceid: &::windows::core::BSTR, bstrworkspacefriendlyname: &::windows::core::BSTR, bstrredirectorname: &::windows::core::BSTR, bstrusername: &::windows::core::BSTR, bstrpassword: &::windows::core::BSTR, bstrappcontainer: &::windows::core::BSTR, bstrworkspaceparams: &::windows::core::BSTR, ltimeout: i32, lflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.StartWorkspaceEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrworkspaceid), ::core::mem::transmute_copy(bstrworkspacefriendlyname), ::core::mem::transmute_copy(bstrredirectorname), ::core::mem::transmute_copy(bstrusername), ::core::mem::transmute_copy(bstrpassword), ::core::mem::transmute_copy(bstrappcontainer), ::core::mem::transmute_copy(bstrworkspaceparams), ltimeout, lflags).ok()
    }
    pub unsafe fn ResourceDismissed(&self, bstrworkspaceid: &::windows::core::BSTR, bstrworkspacefriendlyname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ResourceDismissed)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrworkspaceid), ::core::mem::transmute_copy(bstrworkspacefriendlyname)).ok()
    }
}
impl ::core::cmp::PartialEq for ItsPubPlugin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ItsPubPlugin {}
impl ::core::fmt::Debug for ItsPubPlugin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ItsPubPlugin").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ItsPubPlugin2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ItsPubPlugin2 {}
impl ::core::fmt::Debug for ItsPubPlugin2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ItsPubPlugin2").field(&self.0).finish()
    }
}
impl ItsPubPlugin2 {
    pub unsafe fn GetResourceList<P0>(&self, userid: P0, pceapplistsize: *mut i32, resourcelist: *mut *mut pluginResource) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetResourceList)(::windows::core::Vtable::as_raw(self), userid.into().abi(), pceapplistsize, resourcelist).ok()
    }
    pub unsafe fn GetResource<P0>(&self, alias: P0, flags: i32, resource: *mut pluginResource) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetResource)(::windows::core::Vtable::as_raw(self), alias.into().abi(), flags, resource).ok()
    }
    pub unsafe fn GetCacheLastUpdateTime(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCacheLastUpdateTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn pluginName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.pluginName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn pluginVersion(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.pluginVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ResolveResource<P0, P1>(&self, resourcetype: *mut u32, resourcelocation: ::windows::core::PWSTR, endpointname: ::windows::core::PWSTR, userid: P0, alias: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ResolveResource)(::windows::core::Vtable::as_raw(self), resourcetype, ::core::mem::transmute(resourcelocation), ::core::mem::transmute(endpointname), userid.into().abi(), alias.into().abi()).ok()
    }
}
impl ::core::default::Default for KeyCombinationType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KeyCombinationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyCombinationType").field(&self.0).finish()
    }
}
impl ::core::default::Default for PLUGIN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PLUGIN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PLUGIN_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PRODUCT_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PRODUCT_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.CompanyName == other.CompanyName && self.ProductID == other.ProductID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PRODUCT_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PRODUCT_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRODUCT_INFOA").field("CompanyName", &self.CompanyName).field("ProductID", &self.ProductID).finish()
    }
}
impl ::core::default::Default for PRODUCT_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PRODUCT_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.CompanyName == other.CompanyName && self.ProductID == other.ProductID
    }
}
impl ::core::cmp::Eq for PRODUCT_INFOW {}
impl ::core::fmt::Debug for PRODUCT_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PRODUCT_INFOW").field("CompanyName", &self.CompanyName).field("ProductID", &self.ProductID).finish()
    }
}
impl ::core::default::Default for PasswordEncodingType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PasswordEncodingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PasswordEncodingType").field(&self.0).finish()
    }
}
impl ::core::default::Default for PolicyAttributeType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PolicyAttributeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PolicyAttributeType").field(&self.0).finish()
    }
}
impl ::core::default::Default for RDV_TASK_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RDV_TASK_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RDV_TASK_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for RD_FARM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RD_FARM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RD_FARM_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RFX_GFX_MONITOR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RFX_GFX_MSG_CLIENT_DESKTOP_INFO_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_CONFIRM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RFX_GFX_MSG_DESKTOP_CONFIG_CHANGE_NOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RFX_GFX_MSG_DESKTOP_INPUT_RESET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RFX_GFX_MSG_DESKTOP_RESEND_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RFX_GFX_MSG_DISCONNECT_NOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RFX_GFX_MSG_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RFX_GFX_MSG_RDP_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RFX_GFX_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RemoteActionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RemoteActionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteActionType").field(&self.0).finish()
    }
}
impl ::core::default::Default for SESSION_TIMEOUT_ACTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SESSION_TIMEOUT_ACTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SESSION_TIMEOUT_ACTION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SnapshotEncodingType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SnapshotEncodingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SnapshotEncodingType").field(&self.0).finish()
    }
}
impl ::core::default::Default for SnapshotFormatType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SnapshotFormatType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SnapshotFormatType").field(&self.0).finish()
    }
}
impl ::core::default::Default for TARGET_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TARGET_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TARGET_CHANGE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TARGET_OWNER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TARGET_OWNER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TARGET_OWNER").field(&self.0).finish()
    }
}
impl ::core::default::Default for TARGET_PATCH_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TARGET_PATCH_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TARGET_PATCH_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TARGET_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TARGET_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TARGET_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TARGET_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TARGET_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TARGET_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TSPUB_PLUGIN_PD_RESOLUTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TSPUB_PLUGIN_PD_RESOLUTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TSPUB_PLUGIN_PD_RESOLUTION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TSSB_NOTIFICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TSSB_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TSSB_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TSSD_AddrV46Type {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TSSD_AddrV46Type {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TSSD_AddrV46Type").field(&self.0).finish()
    }
}
impl ::core::default::Default for TSSD_ConnectionPoint {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TSSD_ConnectionPoint {
    fn eq(&self, other: &Self) -> bool {
        self.ServerAddressB == other.ServerAddressB && self.AddressType == other.AddressType && self.PortNumber == other.PortNumber && self.AddressScope == other.AddressScope
    }
}
impl ::core::cmp::Eq for TSSD_ConnectionPoint {}
impl ::core::fmt::Debug for TSSD_ConnectionPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TSSD_ConnectionPoint").field("ServerAddressB", &self.ServerAddressB).field("AddressType", &self.AddressType).field("PortNumber", &self.PortNumber).field("AddressScope", &self.AddressScope).finish()
    }
}
impl ::core::default::Default for TSSESSION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TSSESSION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TSSESSION_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TS_SB_SORT_BY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TS_SB_SORT_BY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TS_SB_SORT_BY").field(&self.0).finish()
    }
}
impl ::core::default::Default for VM_HOST_NOTIFY_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VM_HOST_NOTIFY_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VM_HOST_NOTIFY_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for VM_NOTIFY_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VM_NOTIFY_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.VmName == other.VmName && self.VmHost == other.VmHost
    }
}
impl ::core::cmp::Eq for VM_NOTIFY_ENTRY {}
impl ::core::fmt::Debug for VM_NOTIFY_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VM_NOTIFY_ENTRY").field("VmName", &self.VmName).field("VmHost", &self.VmHost).finish()
    }
}
impl ::core::default::Default for VM_NOTIFY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VM_NOTIFY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumEntries == other.dwNumEntries && self.ppVmEntries == other.ppVmEntries
    }
}
impl ::core::cmp::Eq for VM_NOTIFY_INFO {}
impl ::core::fmt::Debug for VM_NOTIFY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VM_NOTIFY_INFO").field("dwNumEntries", &self.dwNumEntries).field("ppVmEntries", &self.ppVmEntries).finish()
    }
}
impl ::core::default::Default for VM_NOTIFY_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VM_NOTIFY_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VM_NOTIFY_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for VM_PATCH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VM_PATCH_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumEntries == other.dwNumEntries && self.pVmNames == other.pVmNames
    }
}
impl ::core::cmp::Eq for VM_PATCH_INFO {}
impl ::core::fmt::Debug for VM_PATCH_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VM_PATCH_INFO").field("dwNumEntries", &self.dwNumEntries).field("pVmNames", &self.pVmNames).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WRDS_CONNECTION_SETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WRDS_CONNECTION_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WRDS_CONNECTION_SETTINGS_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WRDS_CONNECTION_SETTING_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WRDS_CONNECTION_SETTING_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WRDS_CONNECTION_SETTING_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Bias == other.Bias && self.StandardName == other.StandardName && self.StandardDate == other.StandardDate && self.StandardBias == other.StandardBias && self.DaylightName == other.DaylightName && self.DaylightDate == other.DaylightDate && self.DaylightBias == other.DaylightBias && self.TimeZoneKeyName == other.TimeZoneKeyName && self.DynamicDaylightTimeDisabled == other.DynamicDaylightTimeDisabled
    }
}
impl ::core::cmp::Eq for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {}
impl ::core::fmt::Debug for WRDS_DYNAMIC_TIME_ZONE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WRDS_DYNAMIC_TIME_ZONE_INFORMATION").field("Bias", &self.Bias).field("StandardName", &self.StandardName).field("StandardDate", &self.StandardDate).field("StandardBias", &self.StandardBias).field("DaylightName", &self.DaylightName).field("DaylightDate", &self.DaylightDate).field("DaylightBias", &self.DaylightBias).field("TimeZoneKeyName", &self.TimeZoneKeyName).field("DynamicDaylightTimeDisabled", &self.DynamicDaylightTimeDisabled).finish()
    }
}
impl ::core::default::Default for WRDS_LISTENER_SETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WRDS_LISTENER_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WRDS_LISTENER_SETTINGS_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WRDS_LISTENER_SETTINGS_1 {
    fn eq(&self, other: &Self) -> bool {
        self.MaxProtocolListenerConnectionCount == other.MaxProtocolListenerConnectionCount && self.SecurityDescriptorSize == other.SecurityDescriptorSize && self.pSecurityDescriptor == other.pSecurityDescriptor
    }
}
impl ::core::cmp::Eq for WRDS_LISTENER_SETTINGS_1 {}
impl ::core::fmt::Debug for WRDS_LISTENER_SETTINGS_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WRDS_LISTENER_SETTINGS_1").field("MaxProtocolListenerConnectionCount", &self.MaxProtocolListenerConnectionCount).field("SecurityDescriptorSize", &self.SecurityDescriptorSize).field("pSecurityDescriptor", &self.pSecurityDescriptor).finish()
    }
}
impl ::core::default::Default for WRDS_LISTENER_SETTING_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WRDS_LISTENER_SETTING_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WRDS_LISTENER_SETTING_LEVEL").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WRDS_SETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WRDS_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WRDS_SETTINGS_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WRDS_SETTINGS_1 {
    fn eq(&self, other: &Self) -> bool {
        self.WRdsDisableClipStatus == other.WRdsDisableClipStatus
            && self.WRdsDisableClipValue == other.WRdsDisableClipValue
            && self.WRdsDisableLPTStatus == other.WRdsDisableLPTStatus
            && self.WRdsDisableLPTValue == other.WRdsDisableLPTValue
            && self.WRdsDisableCcmStatus == other.WRdsDisableCcmStatus
            && self.WRdsDisableCcmValue == other.WRdsDisableCcmValue
            && self.WRdsDisableCdmStatus == other.WRdsDisableCdmStatus
            && self.WRdsDisableCdmValue == other.WRdsDisableCdmValue
            && self.WRdsDisableCpmStatus == other.WRdsDisableCpmStatus
            && self.WRdsDisableCpmValue == other.WRdsDisableCpmValue
            && self.WRdsDisablePnpStatus == other.WRdsDisablePnpStatus
            && self.WRdsDisablePnpValue == other.WRdsDisablePnpValue
            && self.WRdsEncryptionLevelStatus == other.WRdsEncryptionLevelStatus
            && self.WRdsEncryptionValue == other.WRdsEncryptionValue
            && self.WRdsColorDepthStatus == other.WRdsColorDepthStatus
            && self.WRdsColorDepthValue == other.WRdsColorDepthValue
            && self.WRdsDisableAutoReconnecetStatus == other.WRdsDisableAutoReconnecetStatus
            && self.WRdsDisableAutoReconnecetValue == other.WRdsDisableAutoReconnecetValue
            && self.WRdsDisableEncryptionStatus == other.WRdsDisableEncryptionStatus
            && self.WRdsDisableEncryptionValue == other.WRdsDisableEncryptionValue
            && self.WRdsResetBrokenStatus == other.WRdsResetBrokenStatus
            && self.WRdsResetBrokenValue == other.WRdsResetBrokenValue
            && self.WRdsMaxIdleTimeStatus == other.WRdsMaxIdleTimeStatus
            && self.WRdsMaxIdleTimeValue == other.WRdsMaxIdleTimeValue
            && self.WRdsMaxDisconnectTimeStatus == other.WRdsMaxDisconnectTimeStatus
            && self.WRdsMaxDisconnectTimeValue == other.WRdsMaxDisconnectTimeValue
            && self.WRdsMaxConnectTimeStatus == other.WRdsMaxConnectTimeStatus
            && self.WRdsMaxConnectTimeValue == other.WRdsMaxConnectTimeValue
            && self.WRdsKeepAliveStatus == other.WRdsKeepAliveStatus
            && self.WRdsKeepAliveStartValue == other.WRdsKeepAliveStartValue
            && self.WRdsKeepAliveIntervalValue == other.WRdsKeepAliveIntervalValue
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WRDS_SETTINGS_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WRDS_SETTINGS_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WRDS_SETTINGS_1")
            .field("WRdsDisableClipStatus", &self.WRdsDisableClipStatus)
            .field("WRdsDisableClipValue", &self.WRdsDisableClipValue)
            .field("WRdsDisableLPTStatus", &self.WRdsDisableLPTStatus)
            .field("WRdsDisableLPTValue", &self.WRdsDisableLPTValue)
            .field("WRdsDisableCcmStatus", &self.WRdsDisableCcmStatus)
            .field("WRdsDisableCcmValue", &self.WRdsDisableCcmValue)
            .field("WRdsDisableCdmStatus", &self.WRdsDisableCdmStatus)
            .field("WRdsDisableCdmValue", &self.WRdsDisableCdmValue)
            .field("WRdsDisableCpmStatus", &self.WRdsDisableCpmStatus)
            .field("WRdsDisableCpmValue", &self.WRdsDisableCpmValue)
            .field("WRdsDisablePnpStatus", &self.WRdsDisablePnpStatus)
            .field("WRdsDisablePnpValue", &self.WRdsDisablePnpValue)
            .field("WRdsEncryptionLevelStatus", &self.WRdsEncryptionLevelStatus)
            .field("WRdsEncryptionValue", &self.WRdsEncryptionValue)
            .field("WRdsColorDepthStatus", &self.WRdsColorDepthStatus)
            .field("WRdsColorDepthValue", &self.WRdsColorDepthValue)
            .field("WRdsDisableAutoReconnecetStatus", &self.WRdsDisableAutoReconnecetStatus)
            .field("WRdsDisableAutoReconnecetValue", &self.WRdsDisableAutoReconnecetValue)
            .field("WRdsDisableEncryptionStatus", &self.WRdsDisableEncryptionStatus)
            .field("WRdsDisableEncryptionValue", &self.WRdsDisableEncryptionValue)
            .field("WRdsResetBrokenStatus", &self.WRdsResetBrokenStatus)
            .field("WRdsResetBrokenValue", &self.WRdsResetBrokenValue)
            .field("WRdsMaxIdleTimeStatus", &self.WRdsMaxIdleTimeStatus)
            .field("WRdsMaxIdleTimeValue", &self.WRdsMaxIdleTimeValue)
            .field("WRdsMaxDisconnectTimeStatus", &self.WRdsMaxDisconnectTimeStatus)
            .field("WRdsMaxDisconnectTimeValue", &self.WRdsMaxDisconnectTimeValue)
            .field("WRdsMaxConnectTimeStatus", &self.WRdsMaxConnectTimeStatus)
            .field("WRdsMaxConnectTimeValue", &self.WRdsMaxConnectTimeValue)
            .field("WRdsKeepAliveStatus", &self.WRdsKeepAliveStatus)
            .field("WRdsKeepAliveStartValue", &self.WRdsKeepAliveStartValue)
            .field("WRdsKeepAliveIntervalValue", &self.WRdsKeepAliveIntervalValue)
            .finish()
    }
}
impl ::core::default::Default for WRDS_SETTING_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WRDS_SETTING_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WRDS_SETTING_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for WRDS_SETTING_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WRDS_SETTING_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WRDS_SETTING_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WRDS_SETTING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WRDS_SETTING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WRDS_SETTING_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WRdsGraphicsChannelType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WRdsGraphicsChannelType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WRdsGraphicsChannelType").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSCLIENTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTSCLIENTA {
    fn eq(&self, other: &Self) -> bool {
        self.ClientName == other.ClientName
            && self.Domain == other.Domain
            && self.UserName == other.UserName
            && self.WorkDirectory == other.WorkDirectory
            && self.InitialProgram == other.InitialProgram
            && self.EncryptionLevel == other.EncryptionLevel
            && self.ClientAddressFamily == other.ClientAddressFamily
            && self.ClientAddress == other.ClientAddress
            && self.HRes == other.HRes
            && self.VRes == other.VRes
            && self.ColorDepth == other.ColorDepth
            && self.ClientDirectory == other.ClientDirectory
            && self.ClientBuildNumber == other.ClientBuildNumber
            && self.ClientHardwareId == other.ClientHardwareId
            && self.ClientProductId == other.ClientProductId
            && self.OutBufCountHost == other.OutBufCountHost
            && self.OutBufCountClient == other.OutBufCountClient
            && self.OutBufLength == other.OutBufLength
            && self.DeviceId == other.DeviceId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTSCLIENTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTSCLIENTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSCLIENTA")
            .field("ClientName", &self.ClientName)
            .field("Domain", &self.Domain)
            .field("UserName", &self.UserName)
            .field("WorkDirectory", &self.WorkDirectory)
            .field("InitialProgram", &self.InitialProgram)
            .field("EncryptionLevel", &self.EncryptionLevel)
            .field("ClientAddressFamily", &self.ClientAddressFamily)
            .field("ClientAddress", &self.ClientAddress)
            .field("HRes", &self.HRes)
            .field("VRes", &self.VRes)
            .field("ColorDepth", &self.ColorDepth)
            .field("ClientDirectory", &self.ClientDirectory)
            .field("ClientBuildNumber", &self.ClientBuildNumber)
            .field("ClientHardwareId", &self.ClientHardwareId)
            .field("ClientProductId", &self.ClientProductId)
            .field("OutBufCountHost", &self.OutBufCountHost)
            .field("OutBufCountClient", &self.OutBufCountClient)
            .field("OutBufLength", &self.OutBufLength)
            .field("DeviceId", &self.DeviceId)
            .finish()
    }
}
impl ::core::default::Default for WTSCLIENTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTSCLIENTW {
    fn eq(&self, other: &Self) -> bool {
        self.ClientName == other.ClientName
            && self.Domain == other.Domain
            && self.UserName == other.UserName
            && self.WorkDirectory == other.WorkDirectory
            && self.InitialProgram == other.InitialProgram
            && self.EncryptionLevel == other.EncryptionLevel
            && self.ClientAddressFamily == other.ClientAddressFamily
            && self.ClientAddress == other.ClientAddress
            && self.HRes == other.HRes
            && self.VRes == other.VRes
            && self.ColorDepth == other.ColorDepth
            && self.ClientDirectory == other.ClientDirectory
            && self.ClientBuildNumber == other.ClientBuildNumber
            && self.ClientHardwareId == other.ClientHardwareId
            && self.ClientProductId == other.ClientProductId
            && self.OutBufCountHost == other.OutBufCountHost
            && self.OutBufCountClient == other.OutBufCountClient
            && self.OutBufLength == other.OutBufLength
            && self.DeviceId == other.DeviceId
    }
}
impl ::core::cmp::Eq for WTSCLIENTW {}
impl ::core::fmt::Debug for WTSCLIENTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSCLIENTW")
            .field("ClientName", &self.ClientName)
            .field("Domain", &self.Domain)
            .field("UserName", &self.UserName)
            .field("WorkDirectory", &self.WorkDirectory)
            .field("InitialProgram", &self.InitialProgram)
            .field("EncryptionLevel", &self.EncryptionLevel)
            .field("ClientAddressFamily", &self.ClientAddressFamily)
            .field("ClientAddress", &self.ClientAddress)
            .field("HRes", &self.HRes)
            .field("VRes", &self.VRes)
            .field("ColorDepth", &self.ColorDepth)
            .field("ClientDirectory", &self.ClientDirectory)
            .field("ClientBuildNumber", &self.ClientBuildNumber)
            .field("ClientHardwareId", &self.ClientHardwareId)
            .field("ClientProductId", &self.ClientProductId)
            .field("OutBufCountHost", &self.OutBufCountHost)
            .field("OutBufCountClient", &self.OutBufCountClient)
            .field("OutBufLength", &self.OutBufLength)
            .field("DeviceId", &self.DeviceId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSCONFIGINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTSCONFIGINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version && self.fConnectClientDrivesAtLogon == other.fConnectClientDrivesAtLogon && self.fConnectPrinterAtLogon == other.fConnectPrinterAtLogon && self.fDisablePrinterRedirection == other.fDisablePrinterRedirection && self.fDisableDefaultMainClientPrinter == other.fDisableDefaultMainClientPrinter && self.ShadowSettings == other.ShadowSettings && self.LogonUserName == other.LogonUserName && self.LogonDomain == other.LogonDomain && self.WorkDirectory == other.WorkDirectory && self.InitialProgram == other.InitialProgram && self.ApplicationName == other.ApplicationName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTSCONFIGINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTSCONFIGINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSCONFIGINFOA")
            .field("version", &self.version)
            .field("fConnectClientDrivesAtLogon", &self.fConnectClientDrivesAtLogon)
            .field("fConnectPrinterAtLogon", &self.fConnectPrinterAtLogon)
            .field("fDisablePrinterRedirection", &self.fDisablePrinterRedirection)
            .field("fDisableDefaultMainClientPrinter", &self.fDisableDefaultMainClientPrinter)
            .field("ShadowSettings", &self.ShadowSettings)
            .field("LogonUserName", &self.LogonUserName)
            .field("LogonDomain", &self.LogonDomain)
            .field("WorkDirectory", &self.WorkDirectory)
            .field("InitialProgram", &self.InitialProgram)
            .field("ApplicationName", &self.ApplicationName)
            .finish()
    }
}
impl ::core::default::Default for WTSCONFIGINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTSCONFIGINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version && self.fConnectClientDrivesAtLogon == other.fConnectClientDrivesAtLogon && self.fConnectPrinterAtLogon == other.fConnectPrinterAtLogon && self.fDisablePrinterRedirection == other.fDisablePrinterRedirection && self.fDisableDefaultMainClientPrinter == other.fDisableDefaultMainClientPrinter && self.ShadowSettings == other.ShadowSettings && self.LogonUserName == other.LogonUserName && self.LogonDomain == other.LogonDomain && self.WorkDirectory == other.WorkDirectory && self.InitialProgram == other.InitialProgram && self.ApplicationName == other.ApplicationName
    }
}
impl ::core::cmp::Eq for WTSCONFIGINFOW {}
impl ::core::fmt::Debug for WTSCONFIGINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSCONFIGINFOW")
            .field("version", &self.version)
            .field("fConnectClientDrivesAtLogon", &self.fConnectClientDrivesAtLogon)
            .field("fConnectPrinterAtLogon", &self.fConnectPrinterAtLogon)
            .field("fDisablePrinterRedirection", &self.fDisablePrinterRedirection)
            .field("fDisableDefaultMainClientPrinter", &self.fDisableDefaultMainClientPrinter)
            .field("ShadowSettings", &self.ShadowSettings)
            .field("LogonUserName", &self.LogonUserName)
            .field("LogonDomain", &self.LogonDomain)
            .field("WorkDirectory", &self.WorkDirectory)
            .field("InitialProgram", &self.InitialProgram)
            .field("ApplicationName", &self.ApplicationName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTSINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.State == other.State && self.SessionId == other.SessionId && self.IncomingBytes == other.IncomingBytes && self.OutgoingBytes == other.OutgoingBytes && self.IncomingFrames == other.IncomingFrames && self.OutgoingFrames == other.OutgoingFrames && self.IncomingCompressedBytes == other.IncomingCompressedBytes && self.OutgoingCompressedBy == other.OutgoingCompressedBy && self.WinStationName == other.WinStationName && self.Domain == other.Domain && self.UserName == other.UserName && self.ConnectTime == other.ConnectTime && self.DisconnectTime == other.DisconnectTime && self.LastInputTime == other.LastInputTime && self.LogonTime == other.LogonTime && self.CurrentTime == other.CurrentTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTSINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTSINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSINFOA")
            .field("State", &self.State)
            .field("SessionId", &self.SessionId)
            .field("IncomingBytes", &self.IncomingBytes)
            .field("OutgoingBytes", &self.OutgoingBytes)
            .field("IncomingFrames", &self.IncomingFrames)
            .field("OutgoingFrames", &self.OutgoingFrames)
            .field("IncomingCompressedBytes", &self.IncomingCompressedBytes)
            .field("OutgoingCompressedBy", &self.OutgoingCompressedBy)
            .field("WinStationName", &self.WinStationName)
            .field("Domain", &self.Domain)
            .field("UserName", &self.UserName)
            .field("ConnectTime", &self.ConnectTime)
            .field("DisconnectTime", &self.DisconnectTime)
            .field("LastInputTime", &self.LastInputTime)
            .field("LogonTime", &self.LogonTime)
            .field("CurrentTime", &self.CurrentTime)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSINFOEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WTSINFOEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSINFOEX_LEVEL1_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTSINFOEX_LEVEL1_A {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId && self.SessionState == other.SessionState && self.SessionFlags == other.SessionFlags && self.WinStationName == other.WinStationName && self.UserName == other.UserName && self.DomainName == other.DomainName && self.LogonTime == other.LogonTime && self.ConnectTime == other.ConnectTime && self.DisconnectTime == other.DisconnectTime && self.LastInputTime == other.LastInputTime && self.CurrentTime == other.CurrentTime && self.IncomingBytes == other.IncomingBytes && self.OutgoingBytes == other.OutgoingBytes && self.IncomingFrames == other.IncomingFrames && self.OutgoingFrames == other.OutgoingFrames && self.IncomingCompressedBytes == other.IncomingCompressedBytes && self.OutgoingCompressedBytes == other.OutgoingCompressedBytes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTSINFOEX_LEVEL1_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTSINFOEX_LEVEL1_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSINFOEX_LEVEL1_A")
            .field("SessionId", &self.SessionId)
            .field("SessionState", &self.SessionState)
            .field("SessionFlags", &self.SessionFlags)
            .field("WinStationName", &self.WinStationName)
            .field("UserName", &self.UserName)
            .field("DomainName", &self.DomainName)
            .field("LogonTime", &self.LogonTime)
            .field("ConnectTime", &self.ConnectTime)
            .field("DisconnectTime", &self.DisconnectTime)
            .field("LastInputTime", &self.LastInputTime)
            .field("CurrentTime", &self.CurrentTime)
            .field("IncomingBytes", &self.IncomingBytes)
            .field("OutgoingBytes", &self.OutgoingBytes)
            .field("IncomingFrames", &self.IncomingFrames)
            .field("OutgoingFrames", &self.OutgoingFrames)
            .field("IncomingCompressedBytes", &self.IncomingCompressedBytes)
            .field("OutgoingCompressedBytes", &self.OutgoingCompressedBytes)
            .finish()
    }
}
impl ::core::default::Default for WTSINFOEX_LEVEL1_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTSINFOEX_LEVEL1_W {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId && self.SessionState == other.SessionState && self.SessionFlags == other.SessionFlags && self.WinStationName == other.WinStationName && self.UserName == other.UserName && self.DomainName == other.DomainName && self.LogonTime == other.LogonTime && self.ConnectTime == other.ConnectTime && self.DisconnectTime == other.DisconnectTime && self.LastInputTime == other.LastInputTime && self.CurrentTime == other.CurrentTime && self.IncomingBytes == other.IncomingBytes && self.OutgoingBytes == other.OutgoingBytes && self.IncomingFrames == other.IncomingFrames && self.OutgoingFrames == other.OutgoingFrames && self.IncomingCompressedBytes == other.IncomingCompressedBytes && self.OutgoingCompressedBytes == other.OutgoingCompressedBytes
    }
}
impl ::core::cmp::Eq for WTSINFOEX_LEVEL1_W {}
impl ::core::fmt::Debug for WTSINFOEX_LEVEL1_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSINFOEX_LEVEL1_W")
            .field("SessionId", &self.SessionId)
            .field("SessionState", &self.SessionState)
            .field("SessionFlags", &self.SessionFlags)
            .field("WinStationName", &self.WinStationName)
            .field("UserName", &self.UserName)
            .field("DomainName", &self.DomainName)
            .field("LogonTime", &self.LogonTime)
            .field("ConnectTime", &self.ConnectTime)
            .field("DisconnectTime", &self.DisconnectTime)
            .field("LastInputTime", &self.LastInputTime)
            .field("CurrentTime", &self.CurrentTime)
            .field("IncomingBytes", &self.IncomingBytes)
            .field("OutgoingBytes", &self.OutgoingBytes)
            .field("IncomingFrames", &self.IncomingFrames)
            .field("OutgoingFrames", &self.OutgoingFrames)
            .field("IncomingCompressedBytes", &self.IncomingCompressedBytes)
            .field("OutgoingCompressedBytes", &self.OutgoingCompressedBytes)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSINFOEX_LEVEL_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WTSINFOEX_LEVEL_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WTSINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTSINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.State == other.State && self.SessionId == other.SessionId && self.IncomingBytes == other.IncomingBytes && self.OutgoingBytes == other.OutgoingBytes && self.IncomingFrames == other.IncomingFrames && self.OutgoingFrames == other.OutgoingFrames && self.IncomingCompressedBytes == other.IncomingCompressedBytes && self.OutgoingCompressedBytes == other.OutgoingCompressedBytes && self.WinStationName == other.WinStationName && self.Domain == other.Domain && self.UserName == other.UserName && self.ConnectTime == other.ConnectTime && self.DisconnectTime == other.DisconnectTime && self.LastInputTime == other.LastInputTime && self.LogonTime == other.LogonTime && self.CurrentTime == other.CurrentTime
    }
}
impl ::core::cmp::Eq for WTSINFOW {}
impl ::core::fmt::Debug for WTSINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSINFOW")
            .field("State", &self.State)
            .field("SessionId", &self.SessionId)
            .field("IncomingBytes", &self.IncomingBytes)
            .field("OutgoingBytes", &self.OutgoingBytes)
            .field("IncomingFrames", &self.IncomingFrames)
            .field("OutgoingFrames", &self.OutgoingFrames)
            .field("IncomingCompressedBytes", &self.IncomingCompressedBytes)
            .field("OutgoingCompressedBytes", &self.OutgoingCompressedBytes)
            .field("WinStationName", &self.WinStationName)
            .field("Domain", &self.Domain)
            .field("UserName", &self.UserName)
            .field("ConnectTime", &self.ConnectTime)
            .field("DisconnectTime", &self.DisconnectTime)
            .field("LastInputTime", &self.LastInputTime)
            .field("LogonTime", &self.LogonTime)
            .field("CurrentTime", &self.CurrentTime)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSLISTENERCONFIGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTSLISTENERCONFIGA {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
            && self.fEnableListener == other.fEnableListener
            && self.MaxConnectionCount == other.MaxConnectionCount
            && self.fPromptForPassword == other.fPromptForPassword
            && self.fInheritColorDepth == other.fInheritColorDepth
            && self.ColorDepth == other.ColorDepth
            && self.fInheritBrokenTimeoutSettings == other.fInheritBrokenTimeoutSettings
            && self.BrokenTimeoutSettings == other.BrokenTimeoutSettings
            && self.fDisablePrinterRedirection == other.fDisablePrinterRedirection
            && self.fDisableDriveRedirection == other.fDisableDriveRedirection
            && self.fDisableComPortRedirection == other.fDisableComPortRedirection
            && self.fDisableLPTPortRedirection == other.fDisableLPTPortRedirection
            && self.fDisableClipboardRedirection == other.fDisableClipboardRedirection
            && self.fDisableAudioRedirection == other.fDisableAudioRedirection
            && self.fDisablePNPRedirection == other.fDisablePNPRedirection
            && self.fDisableDefaultMainClientPrinter == other.fDisableDefaultMainClientPrinter
            && self.LanAdapter == other.LanAdapter
            && self.PortNumber == other.PortNumber
            && self.fInheritShadowSettings == other.fInheritShadowSettings
            && self.ShadowSettings == other.ShadowSettings
            && self.TimeoutSettingsConnection == other.TimeoutSettingsConnection
            && self.TimeoutSettingsDisconnection == other.TimeoutSettingsDisconnection
            && self.TimeoutSettingsIdle == other.TimeoutSettingsIdle
            && self.SecurityLayer == other.SecurityLayer
            && self.MinEncryptionLevel == other.MinEncryptionLevel
            && self.UserAuthentication == other.UserAuthentication
            && self.Comment == other.Comment
            && self.LogonUserName == other.LogonUserName
            && self.LogonDomain == other.LogonDomain
            && self.WorkDirectory == other.WorkDirectory
            && self.InitialProgram == other.InitialProgram
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTSLISTENERCONFIGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTSLISTENERCONFIGA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSLISTENERCONFIGA")
            .field("version", &self.version)
            .field("fEnableListener", &self.fEnableListener)
            .field("MaxConnectionCount", &self.MaxConnectionCount)
            .field("fPromptForPassword", &self.fPromptForPassword)
            .field("fInheritColorDepth", &self.fInheritColorDepth)
            .field("ColorDepth", &self.ColorDepth)
            .field("fInheritBrokenTimeoutSettings", &self.fInheritBrokenTimeoutSettings)
            .field("BrokenTimeoutSettings", &self.BrokenTimeoutSettings)
            .field("fDisablePrinterRedirection", &self.fDisablePrinterRedirection)
            .field("fDisableDriveRedirection", &self.fDisableDriveRedirection)
            .field("fDisableComPortRedirection", &self.fDisableComPortRedirection)
            .field("fDisableLPTPortRedirection", &self.fDisableLPTPortRedirection)
            .field("fDisableClipboardRedirection", &self.fDisableClipboardRedirection)
            .field("fDisableAudioRedirection", &self.fDisableAudioRedirection)
            .field("fDisablePNPRedirection", &self.fDisablePNPRedirection)
            .field("fDisableDefaultMainClientPrinter", &self.fDisableDefaultMainClientPrinter)
            .field("LanAdapter", &self.LanAdapter)
            .field("PortNumber", &self.PortNumber)
            .field("fInheritShadowSettings", &self.fInheritShadowSettings)
            .field("ShadowSettings", &self.ShadowSettings)
            .field("TimeoutSettingsConnection", &self.TimeoutSettingsConnection)
            .field("TimeoutSettingsDisconnection", &self.TimeoutSettingsDisconnection)
            .field("TimeoutSettingsIdle", &self.TimeoutSettingsIdle)
            .field("SecurityLayer", &self.SecurityLayer)
            .field("MinEncryptionLevel", &self.MinEncryptionLevel)
            .field("UserAuthentication", &self.UserAuthentication)
            .field("Comment", &self.Comment)
            .field("LogonUserName", &self.LogonUserName)
            .field("LogonDomain", &self.LogonDomain)
            .field("WorkDirectory", &self.WorkDirectory)
            .field("InitialProgram", &self.InitialProgram)
            .finish()
    }
}
impl ::core::default::Default for WTSLISTENERCONFIGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTSLISTENERCONFIGW {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
            && self.fEnableListener == other.fEnableListener
            && self.MaxConnectionCount == other.MaxConnectionCount
            && self.fPromptForPassword == other.fPromptForPassword
            && self.fInheritColorDepth == other.fInheritColorDepth
            && self.ColorDepth == other.ColorDepth
            && self.fInheritBrokenTimeoutSettings == other.fInheritBrokenTimeoutSettings
            && self.BrokenTimeoutSettings == other.BrokenTimeoutSettings
            && self.fDisablePrinterRedirection == other.fDisablePrinterRedirection
            && self.fDisableDriveRedirection == other.fDisableDriveRedirection
            && self.fDisableComPortRedirection == other.fDisableComPortRedirection
            && self.fDisableLPTPortRedirection == other.fDisableLPTPortRedirection
            && self.fDisableClipboardRedirection == other.fDisableClipboardRedirection
            && self.fDisableAudioRedirection == other.fDisableAudioRedirection
            && self.fDisablePNPRedirection == other.fDisablePNPRedirection
            && self.fDisableDefaultMainClientPrinter == other.fDisableDefaultMainClientPrinter
            && self.LanAdapter == other.LanAdapter
            && self.PortNumber == other.PortNumber
            && self.fInheritShadowSettings == other.fInheritShadowSettings
            && self.ShadowSettings == other.ShadowSettings
            && self.TimeoutSettingsConnection == other.TimeoutSettingsConnection
            && self.TimeoutSettingsDisconnection == other.TimeoutSettingsDisconnection
            && self.TimeoutSettingsIdle == other.TimeoutSettingsIdle
            && self.SecurityLayer == other.SecurityLayer
            && self.MinEncryptionLevel == other.MinEncryptionLevel
            && self.UserAuthentication == other.UserAuthentication
            && self.Comment == other.Comment
            && self.LogonUserName == other.LogonUserName
            && self.LogonDomain == other.LogonDomain
            && self.WorkDirectory == other.WorkDirectory
            && self.InitialProgram == other.InitialProgram
    }
}
impl ::core::cmp::Eq for WTSLISTENERCONFIGW {}
impl ::core::fmt::Debug for WTSLISTENERCONFIGW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSLISTENERCONFIGW")
            .field("version", &self.version)
            .field("fEnableListener", &self.fEnableListener)
            .field("MaxConnectionCount", &self.MaxConnectionCount)
            .field("fPromptForPassword", &self.fPromptForPassword)
            .field("fInheritColorDepth", &self.fInheritColorDepth)
            .field("ColorDepth", &self.ColorDepth)
            .field("fInheritBrokenTimeoutSettings", &self.fInheritBrokenTimeoutSettings)
            .field("BrokenTimeoutSettings", &self.BrokenTimeoutSettings)
            .field("fDisablePrinterRedirection", &self.fDisablePrinterRedirection)
            .field("fDisableDriveRedirection", &self.fDisableDriveRedirection)
            .field("fDisableComPortRedirection", &self.fDisableComPortRedirection)
            .field("fDisableLPTPortRedirection", &self.fDisableLPTPortRedirection)
            .field("fDisableClipboardRedirection", &self.fDisableClipboardRedirection)
            .field("fDisableAudioRedirection", &self.fDisableAudioRedirection)
            .field("fDisablePNPRedirection", &self.fDisablePNPRedirection)
            .field("fDisableDefaultMainClientPrinter", &self.fDisableDefaultMainClientPrinter)
            .field("LanAdapter", &self.LanAdapter)
            .field("PortNumber", &self.PortNumber)
            .field("fInheritShadowSettings", &self.fInheritShadowSettings)
            .field("ShadowSettings", &self.ShadowSettings)
            .field("TimeoutSettingsConnection", &self.TimeoutSettingsConnection)
            .field("TimeoutSettingsDisconnection", &self.TimeoutSettingsDisconnection)
            .field("TimeoutSettingsIdle", &self.TimeoutSettingsIdle)
            .field("SecurityLayer", &self.SecurityLayer)
            .field("MinEncryptionLevel", &self.MinEncryptionLevel)
            .field("UserAuthentication", &self.UserAuthentication)
            .field("Comment", &self.Comment)
            .field("LogonUserName", &self.LogonUserName)
            .field("LogonDomain", &self.LogonDomain)
            .field("WorkDirectory", &self.WorkDirectory)
            .field("InitialProgram", &self.InitialProgram)
            .finish()
    }
}
impl ::core::default::Default for WTSSBX_ADDRESS_FAMILY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WTSSBX_ADDRESS_FAMILY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTSSBX_ADDRESS_FAMILY").field(&self.0).finish()
    }
}
impl ::core::default::Default for WTSSBX_IP_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTSSBX_IP_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.AddressFamily == other.AddressFamily && self.Address == other.Address && self.PortNumber == other.PortNumber && self.dwScope == other.dwScope
    }
}
impl ::core::cmp::Eq for WTSSBX_IP_ADDRESS {}
impl ::core::fmt::Debug for WTSSBX_IP_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSSBX_IP_ADDRESS").field("AddressFamily", &self.AddressFamily).field("Address", &self.Address).field("PortNumber", &self.PortNumber).field("dwScope", &self.dwScope).finish()
    }
}
impl ::core::default::Default for WTSSBX_MACHINE_CONNECT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTSSBX_MACHINE_CONNECT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.wczMachineFQDN == other.wczMachineFQDN && self.wczMachineNetBiosName == other.wczMachineNetBiosName && self.dwNumOfIPAddr == other.dwNumOfIPAddr && self.IPaddr == other.IPaddr
    }
}
impl ::core::cmp::Eq for WTSSBX_MACHINE_CONNECT_INFO {}
impl ::core::fmt::Debug for WTSSBX_MACHINE_CONNECT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSSBX_MACHINE_CONNECT_INFO").field("wczMachineFQDN", &self.wczMachineFQDN).field("wczMachineNetBiosName", &self.wczMachineNetBiosName).field("dwNumOfIPAddr", &self.dwNumOfIPAddr).field("IPaddr", &self.IPaddr).finish()
    }
}
impl ::core::default::Default for WTSSBX_MACHINE_DRAIN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WTSSBX_MACHINE_DRAIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTSSBX_MACHINE_DRAIN").field(&self.0).finish()
    }
}
impl ::core::default::Default for WTSSBX_MACHINE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTSSBX_MACHINE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ClientConnectInfo == other.ClientConnectInfo && self.wczFarmName == other.wczFarmName && self.InternalIPAddress == other.InternalIPAddress && self.dwMaxSessionsLimit == other.dwMaxSessionsLimit && self.ServerWeight == other.ServerWeight && self.SingleSessionMode == other.SingleSessionMode && self.InDrain == other.InDrain && self.MachineState == other.MachineState
    }
}
impl ::core::cmp::Eq for WTSSBX_MACHINE_INFO {}
impl ::core::fmt::Debug for WTSSBX_MACHINE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSSBX_MACHINE_INFO").field("ClientConnectInfo", &self.ClientConnectInfo).field("wczFarmName", &self.wczFarmName).field("InternalIPAddress", &self.InternalIPAddress).field("dwMaxSessionsLimit", &self.dwMaxSessionsLimit).field("ServerWeight", &self.ServerWeight).field("SingleSessionMode", &self.SingleSessionMode).field("InDrain", &self.InDrain).field("MachineState", &self.MachineState).finish()
    }
}
impl ::core::default::Default for WTSSBX_MACHINE_SESSION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WTSSBX_MACHINE_SESSION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTSSBX_MACHINE_SESSION_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WTSSBX_MACHINE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WTSSBX_MACHINE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTSSBX_MACHINE_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WTSSBX_NOTIFICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WTSSBX_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTSSBX_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSSBX_SESSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTSSBX_SESSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.wszUserName == other.wszUserName && self.wszDomainName == other.wszDomainName && self.ApplicationType == other.ApplicationType && self.dwSessionId == other.dwSessionId && self.CreateTime == other.CreateTime && self.DisconnectTime == other.DisconnectTime && self.SessionState == other.SessionState
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTSSBX_SESSION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTSSBX_SESSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSSBX_SESSION_INFO").field("wszUserName", &self.wszUserName).field("wszDomainName", &self.wszDomainName).field("ApplicationType", &self.ApplicationType).field("dwSessionId", &self.dwSessionId).field("CreateTime", &self.CreateTime).field("DisconnectTime", &self.DisconnectTime).field("SessionState", &self.SessionState).finish()
    }
}
impl ::core::default::Default for WTSSBX_SESSION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WTSSBX_SESSION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTSSBX_SESSION_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WTSSESSION_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTSSESSION_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwSessionId == other.dwSessionId
    }
}
impl ::core::cmp::Eq for WTSSESSION_NOTIFICATION {}
impl ::core::fmt::Debug for WTSSESSION_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSSESSION_NOTIFICATION").field("cbSize", &self.cbSize).field("dwSessionId", &self.dwSessionId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTSUSERCONFIGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTSUSERCONFIGA {
    fn eq(&self, other: &Self) -> bool {
        self.Source == other.Source
            && self.InheritInitialProgram == other.InheritInitialProgram
            && self.AllowLogonTerminalServer == other.AllowLogonTerminalServer
            && self.TimeoutSettingsConnections == other.TimeoutSettingsConnections
            && self.TimeoutSettingsDisconnections == other.TimeoutSettingsDisconnections
            && self.TimeoutSettingsIdle == other.TimeoutSettingsIdle
            && self.DeviceClientDrives == other.DeviceClientDrives
            && self.DeviceClientPrinters == other.DeviceClientPrinters
            && self.ClientDefaultPrinter == other.ClientDefaultPrinter
            && self.BrokenTimeoutSettings == other.BrokenTimeoutSettings
            && self.ReconnectSettings == other.ReconnectSettings
            && self.ShadowingSettings == other.ShadowingSettings
            && self.TerminalServerRemoteHomeDir == other.TerminalServerRemoteHomeDir
            && self.InitialProgram == other.InitialProgram
            && self.WorkDirectory == other.WorkDirectory
            && self.TerminalServerProfilePath == other.TerminalServerProfilePath
            && self.TerminalServerHomeDir == other.TerminalServerHomeDir
            && self.TerminalServerHomeDirDrive == other.TerminalServerHomeDirDrive
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTSUSERCONFIGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTSUSERCONFIGA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSUSERCONFIGA")
            .field("Source", &self.Source)
            .field("InheritInitialProgram", &self.InheritInitialProgram)
            .field("AllowLogonTerminalServer", &self.AllowLogonTerminalServer)
            .field("TimeoutSettingsConnections", &self.TimeoutSettingsConnections)
            .field("TimeoutSettingsDisconnections", &self.TimeoutSettingsDisconnections)
            .field("TimeoutSettingsIdle", &self.TimeoutSettingsIdle)
            .field("DeviceClientDrives", &self.DeviceClientDrives)
            .field("DeviceClientPrinters", &self.DeviceClientPrinters)
            .field("ClientDefaultPrinter", &self.ClientDefaultPrinter)
            .field("BrokenTimeoutSettings", &self.BrokenTimeoutSettings)
            .field("ReconnectSettings", &self.ReconnectSettings)
            .field("ShadowingSettings", &self.ShadowingSettings)
            .field("TerminalServerRemoteHomeDir", &self.TerminalServerRemoteHomeDir)
            .field("InitialProgram", &self.InitialProgram)
            .field("WorkDirectory", &self.WorkDirectory)
            .field("TerminalServerProfilePath", &self.TerminalServerProfilePath)
            .field("TerminalServerHomeDir", &self.TerminalServerHomeDir)
            .field("TerminalServerHomeDirDrive", &self.TerminalServerHomeDirDrive)
            .finish()
    }
}
impl ::core::default::Default for WTSUSERCONFIGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTSUSERCONFIGW {
    fn eq(&self, other: &Self) -> bool {
        self.Source == other.Source
            && self.InheritInitialProgram == other.InheritInitialProgram
            && self.AllowLogonTerminalServer == other.AllowLogonTerminalServer
            && self.TimeoutSettingsConnections == other.TimeoutSettingsConnections
            && self.TimeoutSettingsDisconnections == other.TimeoutSettingsDisconnections
            && self.TimeoutSettingsIdle == other.TimeoutSettingsIdle
            && self.DeviceClientDrives == other.DeviceClientDrives
            && self.DeviceClientPrinters == other.DeviceClientPrinters
            && self.ClientDefaultPrinter == other.ClientDefaultPrinter
            && self.BrokenTimeoutSettings == other.BrokenTimeoutSettings
            && self.ReconnectSettings == other.ReconnectSettings
            && self.ShadowingSettings == other.ShadowingSettings
            && self.TerminalServerRemoteHomeDir == other.TerminalServerRemoteHomeDir
            && self.InitialProgram == other.InitialProgram
            && self.WorkDirectory == other.WorkDirectory
            && self.TerminalServerProfilePath == other.TerminalServerProfilePath
            && self.TerminalServerHomeDir == other.TerminalServerHomeDir
            && self.TerminalServerHomeDirDrive == other.TerminalServerHomeDirDrive
    }
}
impl ::core::cmp::Eq for WTSUSERCONFIGW {}
impl ::core::fmt::Debug for WTSUSERCONFIGW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTSUSERCONFIGW")
            .field("Source", &self.Source)
            .field("InheritInitialProgram", &self.InheritInitialProgram)
            .field("AllowLogonTerminalServer", &self.AllowLogonTerminalServer)
            .field("TimeoutSettingsConnections", &self.TimeoutSettingsConnections)
            .field("TimeoutSettingsDisconnections", &self.TimeoutSettingsDisconnections)
            .field("TimeoutSettingsIdle", &self.TimeoutSettingsIdle)
            .field("DeviceClientDrives", &self.DeviceClientDrives)
            .field("DeviceClientPrinters", &self.DeviceClientPrinters)
            .field("ClientDefaultPrinter", &self.ClientDefaultPrinter)
            .field("BrokenTimeoutSettings", &self.BrokenTimeoutSettings)
            .field("ReconnectSettings", &self.ReconnectSettings)
            .field("ShadowingSettings", &self.ShadowingSettings)
            .field("TerminalServerRemoteHomeDir", &self.TerminalServerRemoteHomeDir)
            .field("InitialProgram", &self.InitialProgram)
            .field("WorkDirectory", &self.WorkDirectory)
            .field("TerminalServerProfilePath", &self.TerminalServerProfilePath)
            .field("TerminalServerHomeDir", &self.TerminalServerHomeDir)
            .field("TerminalServerHomeDirDrive", &self.TerminalServerHomeDirDrive)
            .finish()
    }
}
impl ::core::default::Default for WTS_CACHE_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WTS_CACHE_STATS_UN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WTS_CERT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WTS_CERT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_CERT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WTS_CLIENT_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTS_CLIENT_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.AddressFamily == other.AddressFamily && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for WTS_CLIENT_ADDRESS {}
impl ::core::fmt::Debug for WTS_CLIENT_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_CLIENT_ADDRESS").field("AddressFamily", &self.AddressFamily).field("Address", &self.Address).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_CLIENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WTS_CLIENT_DISPLAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTS_CLIENT_DISPLAY {
    fn eq(&self, other: &Self) -> bool {
        self.HorizontalResolution == other.HorizontalResolution && self.VerticalResolution == other.VerticalResolution && self.ColorDepth == other.ColorDepth
    }
}
impl ::core::cmp::Eq for WTS_CLIENT_DISPLAY {}
impl ::core::fmt::Debug for WTS_CLIENT_DISPLAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_CLIENT_DISPLAY").field("HorizontalResolution", &self.HorizontalResolution).field("VerticalResolution", &self.VerticalResolution).field("ColorDepth", &self.ColorDepth).finish()
    }
}
impl ::core::default::Default for WTS_CONFIG_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WTS_CONFIG_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_CONFIG_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WTS_CONFIG_SOURCE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WTS_CONFIG_SOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_CONFIG_SOURCE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WTS_CONNECTSTATE_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WTS_CONNECTSTATE_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_CONNECTSTATE_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WTS_DISPLAY_IOCTL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTS_DISPLAY_IOCTL {
    fn eq(&self, other: &Self) -> bool {
        self.pDisplayIOCtlData == other.pDisplayIOCtlData && self.cbDisplayIOCtlData == other.cbDisplayIOCtlData
    }
}
impl ::core::cmp::Eq for WTS_DISPLAY_IOCTL {}
impl ::core::fmt::Debug for WTS_DISPLAY_IOCTL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_DISPLAY_IOCTL").field("pDisplayIOCtlData", &self.pDisplayIOCtlData).field("cbDisplayIOCtlData", &self.cbDisplayIOCtlData).finish()
    }
}
impl ::core::default::Default for WTS_INFO_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WTS_INFO_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_INFO_CLASS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_LICENSE_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_LICENSE_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.KeyExchangeAlg == other.KeyExchangeAlg && self.ProtocolVer == other.ProtocolVer && self.fAuthenticateServer == other.fAuthenticateServer && self.CertType == other.CertType && self.cbClientName == other.cbClientName && self.rgbClientName == other.rgbClientName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_LICENSE_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_LICENSE_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_LICENSE_CAPABILITIES").field("KeyExchangeAlg", &self.KeyExchangeAlg).field("ProtocolVer", &self.ProtocolVer).field("fAuthenticateServer", &self.fAuthenticateServer).field("CertType", &self.CertType).field("cbClientName", &self.cbClientName).field("rgbClientName", &self.rgbClientName).finish()
    }
}
impl ::core::default::Default for WTS_LOGON_ERROR_REDIRECTOR_RESPONSE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WTS_LOGON_ERROR_REDIRECTOR_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_LOGON_ERROR_REDIRECTOR_RESPONSE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_POLICY_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_POLICY_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.fDisableEncryption == other.fDisableEncryption && self.fDisableAutoReconnect == other.fDisableAutoReconnect && self.ColorDepth == other.ColorDepth && self.MinEncryptionLevel == other.MinEncryptionLevel && self.fDisableCpm == other.fDisableCpm && self.fDisableCdm == other.fDisableCdm && self.fDisableCcm == other.fDisableCcm && self.fDisableLPT == other.fDisableLPT && self.fDisableClip == other.fDisableClip && self.fDisablePNPRedir == other.fDisablePNPRedir
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_POLICY_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_POLICY_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_POLICY_DATA")
            .field("fDisableEncryption", &self.fDisableEncryption)
            .field("fDisableAutoReconnect", &self.fDisableAutoReconnect)
            .field("ColorDepth", &self.ColorDepth)
            .field("MinEncryptionLevel", &self.MinEncryptionLevel)
            .field("fDisableCpm", &self.fDisableCpm)
            .field("fDisableCdm", &self.fDisableCdm)
            .field("fDisableCcm", &self.fDisableCcm)
            .field("fDisableLPT", &self.fDisableLPT)
            .field("fDisableClip", &self.fDisableClip)
            .field("fDisablePNPRedir", &self.fDisablePNPRedir)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_PROCESS_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_PROCESS_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId && self.ProcessId == other.ProcessId && self.pProcessName == other.pProcessName && self.pUserSid == other.pUserSid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_PROCESS_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_PROCESS_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_PROCESS_INFOA").field("SessionId", &self.SessionId).field("ProcessId", &self.ProcessId).field("pProcessName", &self.pProcessName).field("pUserSid", &self.pUserSid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_PROCESS_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_PROCESS_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId && self.ProcessId == other.ProcessId && self.pProcessName == other.pProcessName && self.pUserSid == other.pUserSid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_PROCESS_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_PROCESS_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_PROCESS_INFOW").field("SessionId", &self.SessionId).field("ProcessId", &self.ProcessId).field("pProcessName", &self.pProcessName).field("pUserSid", &self.pUserSid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_PROCESS_INFO_EXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_PROCESS_INFO_EXA {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId && self.ProcessId == other.ProcessId && self.pProcessName == other.pProcessName && self.pUserSid == other.pUserSid && self.NumberOfThreads == other.NumberOfThreads && self.HandleCount == other.HandleCount && self.PagefileUsage == other.PagefileUsage && self.PeakPagefileUsage == other.PeakPagefileUsage && self.WorkingSetSize == other.WorkingSetSize && self.PeakWorkingSetSize == other.PeakWorkingSetSize && self.UserTime == other.UserTime && self.KernelTime == other.KernelTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_PROCESS_INFO_EXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_PROCESS_INFO_EXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_PROCESS_INFO_EXA")
            .field("SessionId", &self.SessionId)
            .field("ProcessId", &self.ProcessId)
            .field("pProcessName", &self.pProcessName)
            .field("pUserSid", &self.pUserSid)
            .field("NumberOfThreads", &self.NumberOfThreads)
            .field("HandleCount", &self.HandleCount)
            .field("PagefileUsage", &self.PagefileUsage)
            .field("PeakPagefileUsage", &self.PeakPagefileUsage)
            .field("WorkingSetSize", &self.WorkingSetSize)
            .field("PeakWorkingSetSize", &self.PeakWorkingSetSize)
            .field("UserTime", &self.UserTime)
            .field("KernelTime", &self.KernelTime)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_PROCESS_INFO_EXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_PROCESS_INFO_EXW {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId && self.ProcessId == other.ProcessId && self.pProcessName == other.pProcessName && self.pUserSid == other.pUserSid && self.NumberOfThreads == other.NumberOfThreads && self.HandleCount == other.HandleCount && self.PagefileUsage == other.PagefileUsage && self.PeakPagefileUsage == other.PeakPagefileUsage && self.WorkingSetSize == other.WorkingSetSize && self.PeakWorkingSetSize == other.PeakWorkingSetSize && self.UserTime == other.UserTime && self.KernelTime == other.KernelTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_PROCESS_INFO_EXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_PROCESS_INFO_EXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_PROCESS_INFO_EXW")
            .field("SessionId", &self.SessionId)
            .field("ProcessId", &self.ProcessId)
            .field("pProcessName", &self.pProcessName)
            .field("pUserSid", &self.pUserSid)
            .field("NumberOfThreads", &self.NumberOfThreads)
            .field("HandleCount", &self.HandleCount)
            .field("PagefileUsage", &self.PagefileUsage)
            .field("PeakPagefileUsage", &self.PeakPagefileUsage)
            .field("WorkingSetSize", &self.WorkingSetSize)
            .field("PeakWorkingSetSize", &self.PeakWorkingSetSize)
            .field("UserTime", &self.UserTime)
            .field("KernelTime", &self.KernelTime)
            .finish()
    }
}
impl ::core::default::Default for WTS_PROPERTY_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WTS_PROTOCOL_CACHE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTS_PROTOCOL_CACHE {
    fn eq(&self, other: &Self) -> bool {
        self.CacheReads == other.CacheReads && self.CacheHits == other.CacheHits
    }
}
impl ::core::cmp::Eq for WTS_PROTOCOL_CACHE {}
impl ::core::fmt::Debug for WTS_PROTOCOL_CACHE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_PROTOCOL_CACHE").field("CacheReads", &self.CacheReads).field("CacheHits", &self.CacheHits).finish()
    }
}
impl ::core::default::Default for WTS_PROTOCOL_COUNTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTS_PROTOCOL_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.WdBytes == other.WdBytes && self.WdFrames == other.WdFrames && self.WaitForOutBuf == other.WaitForOutBuf && self.Frames == other.Frames && self.Bytes == other.Bytes && self.CompressedBytes == other.CompressedBytes && self.CompressFlushes == other.CompressFlushes && self.Errors == other.Errors && self.Timeouts == other.Timeouts && self.AsyncFramingError == other.AsyncFramingError && self.AsyncOverrunError == other.AsyncOverrunError && self.AsyncOverflowError == other.AsyncOverflowError && self.AsyncParityError == other.AsyncParityError && self.TdErrors == other.TdErrors && self.ProtocolType == other.ProtocolType && self.Length == other.Length && self.Specific == other.Specific && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for WTS_PROTOCOL_COUNTERS {}
impl ::core::fmt::Debug for WTS_PROTOCOL_COUNTERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_PROTOCOL_COUNTERS")
            .field("WdBytes", &self.WdBytes)
            .field("WdFrames", &self.WdFrames)
            .field("WaitForOutBuf", &self.WaitForOutBuf)
            .field("Frames", &self.Frames)
            .field("Bytes", &self.Bytes)
            .field("CompressedBytes", &self.CompressedBytes)
            .field("CompressFlushes", &self.CompressFlushes)
            .field("Errors", &self.Errors)
            .field("Timeouts", &self.Timeouts)
            .field("AsyncFramingError", &self.AsyncFramingError)
            .field("AsyncOverrunError", &self.AsyncOverrunError)
            .field("AsyncOverflowError", &self.AsyncOverflowError)
            .field("AsyncParityError", &self.AsyncParityError)
            .field("TdErrors", &self.TdErrors)
            .field("ProtocolType", &self.ProtocolType)
            .field("Length", &self.Length)
            .field("Specific", &self.Specific)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::default::Default for WTS_PROTOCOL_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WTS_RCM_DRAIN_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WTS_RCM_DRAIN_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_RCM_DRAIN_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WTS_RCM_SERVICE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WTS_RCM_SERVICE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_RCM_SERVICE_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WTS_SERVER_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTS_SERVER_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.pServerName == other.pServerName
    }
}
impl ::core::cmp::Eq for WTS_SERVER_INFOA {}
impl ::core::fmt::Debug for WTS_SERVER_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SERVER_INFOA").field("pServerName", &self.pServerName).finish()
    }
}
impl ::core::default::Default for WTS_SERVER_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTS_SERVER_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.pServerName == other.pServerName
    }
}
impl ::core::cmp::Eq for WTS_SERVER_INFOW {}
impl ::core::fmt::Debug for WTS_SERVER_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SERVER_INFOW").field("pServerName", &self.pServerName).finish()
    }
}
impl ::core::default::Default for WTS_SERVICE_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTS_SERVICE_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.RcmServiceState == other.RcmServiceState && self.RcmDrainState == other.RcmDrainState
    }
}
impl ::core::cmp::Eq for WTS_SERVICE_STATE {}
impl ::core::fmt::Debug for WTS_SERVICE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SERVICE_STATE").field("RcmServiceState", &self.RcmServiceState).field("RcmDrainState", &self.RcmDrainState).finish()
    }
}
impl ::core::default::Default for WTS_SESSION_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTS_SESSION_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.AddressFamily == other.AddressFamily && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for WTS_SESSION_ADDRESS {}
impl ::core::fmt::Debug for WTS_SESSION_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SESSION_ADDRESS").field("AddressFamily", &self.AddressFamily).field("Address", &self.Address).finish()
    }
}
impl ::core::default::Default for WTS_SESSION_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTS_SESSION_ID {
    fn eq(&self, other: &Self) -> bool {
        self.SessionUniqueGuid == other.SessionUniqueGuid && self.SessionId == other.SessionId
    }
}
impl ::core::cmp::Eq for WTS_SESSION_ID {}
impl ::core::fmt::Debug for WTS_SESSION_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SESSION_ID").field("SessionUniqueGuid", &self.SessionUniqueGuid).field("SessionId", &self.SessionId).finish()
    }
}
impl ::core::default::Default for WTS_SESSION_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTS_SESSION_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId && self.pWinStationName == other.pWinStationName && self.State == other.State
    }
}
impl ::core::cmp::Eq for WTS_SESSION_INFOA {}
impl ::core::fmt::Debug for WTS_SESSION_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SESSION_INFOA").field("SessionId", &self.SessionId).field("pWinStationName", &self.pWinStationName).field("State", &self.State).finish()
    }
}
impl ::core::default::Default for WTS_SESSION_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTS_SESSION_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.SessionId == other.SessionId && self.pWinStationName == other.pWinStationName && self.State == other.State
    }
}
impl ::core::cmp::Eq for WTS_SESSION_INFOW {}
impl ::core::fmt::Debug for WTS_SESSION_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SESSION_INFOW").field("SessionId", &self.SessionId).field("pWinStationName", &self.pWinStationName).field("State", &self.State).finish()
    }
}
impl ::core::default::Default for WTS_SESSION_INFO_1A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTS_SESSION_INFO_1A {
    fn eq(&self, other: &Self) -> bool {
        self.ExecEnvId == other.ExecEnvId && self.State == other.State && self.SessionId == other.SessionId && self.pSessionName == other.pSessionName && self.pHostName == other.pHostName && self.pUserName == other.pUserName && self.pDomainName == other.pDomainName && self.pFarmName == other.pFarmName
    }
}
impl ::core::cmp::Eq for WTS_SESSION_INFO_1A {}
impl ::core::fmt::Debug for WTS_SESSION_INFO_1A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SESSION_INFO_1A").field("ExecEnvId", &self.ExecEnvId).field("State", &self.State).field("SessionId", &self.SessionId).field("pSessionName", &self.pSessionName).field("pHostName", &self.pHostName).field("pUserName", &self.pUserName).field("pDomainName", &self.pDomainName).field("pFarmName", &self.pFarmName).finish()
    }
}
impl ::core::default::Default for WTS_SESSION_INFO_1W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTS_SESSION_INFO_1W {
    fn eq(&self, other: &Self) -> bool {
        self.ExecEnvId == other.ExecEnvId && self.State == other.State && self.SessionId == other.SessionId && self.pSessionName == other.pSessionName && self.pHostName == other.pHostName && self.pUserName == other.pUserName && self.pDomainName == other.pDomainName && self.pFarmName == other.pFarmName
    }
}
impl ::core::cmp::Eq for WTS_SESSION_INFO_1W {}
impl ::core::fmt::Debug for WTS_SESSION_INFO_1W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SESSION_INFO_1W").field("ExecEnvId", &self.ExecEnvId).field("State", &self.State).field("SessionId", &self.SessionId).field("pSessionName", &self.pSessionName).field("pHostName", &self.pHostName).field("pUserName", &self.pUserName).field("pDomainName", &self.pDomainName).field("pFarmName", &self.pFarmName).finish()
    }
}
impl ::core::default::Default for WTS_SMALL_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTS_SMALL_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.Left == other.Left && self.Top == other.Top && self.Right == other.Right && self.Bottom == other.Bottom
    }
}
impl ::core::cmp::Eq for WTS_SMALL_RECT {}
impl ::core::fmt::Debug for WTS_SMALL_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SMALL_RECT").field("Left", &self.Left).field("Top", &self.Top).field("Right", &self.Right).field("Bottom", &self.Bottom).finish()
    }
}
impl ::core::default::Default for WTS_SOCKADDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WTS_SYSTEMTIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTS_SYSTEMTIME {
    fn eq(&self, other: &Self) -> bool {
        self.wYear == other.wYear && self.wMonth == other.wMonth && self.wDayOfWeek == other.wDayOfWeek && self.wDay == other.wDay && self.wHour == other.wHour && self.wMinute == other.wMinute && self.wSecond == other.wSecond && self.wMilliseconds == other.wMilliseconds
    }
}
impl ::core::cmp::Eq for WTS_SYSTEMTIME {}
impl ::core::fmt::Debug for WTS_SYSTEMTIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_SYSTEMTIME").field("wYear", &self.wYear).field("wMonth", &self.wMonth).field("wDayOfWeek", &self.wDayOfWeek).field("wDay", &self.wDay).field("wHour", &self.wHour).field("wMinute", &self.wMinute).field("wSecond", &self.wSecond).field("wMilliseconds", &self.wMilliseconds).finish()
    }
}
impl ::core::default::Default for WTS_TIME_ZONE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTS_TIME_ZONE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Bias == other.Bias && self.StandardName == other.StandardName && self.StandardDate == other.StandardDate && self.StandardBias == other.StandardBias && self.DaylightName == other.DaylightName && self.DaylightDate == other.DaylightDate && self.DaylightBias == other.DaylightBias
    }
}
impl ::core::cmp::Eq for WTS_TIME_ZONE_INFORMATION {}
impl ::core::fmt::Debug for WTS_TIME_ZONE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_TIME_ZONE_INFORMATION").field("Bias", &self.Bias).field("StandardName", &self.StandardName).field("StandardDate", &self.StandardDate).field("StandardBias", &self.StandardBias).field("DaylightName", &self.DaylightName).field("DaylightDate", &self.DaylightDate).field("DaylightBias", &self.DaylightBias).finish()
    }
}
impl ::core::default::Default for WTS_TYPE_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WTS_TYPE_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_TYPE_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WTS_USER_CREDENTIAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTS_USER_CREDENTIAL {
    fn eq(&self, other: &Self) -> bool {
        self.UserName == other.UserName && self.Password == other.Password && self.Domain == other.Domain
    }
}
impl ::core::cmp::Eq for WTS_USER_CREDENTIAL {}
impl ::core::fmt::Debug for WTS_USER_CREDENTIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_USER_CREDENTIAL").field("UserName", &self.UserName).field("Password", &self.Password).field("Domain", &self.Domain).finish()
    }
}
impl ::core::default::Default for WTS_USER_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTS_USER_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.WorkDirectory == other.WorkDirectory && self.InitialProgram == other.InitialProgram && self.UserTimeZone == other.UserTimeZone
    }
}
impl ::core::cmp::Eq for WTS_USER_DATA {}
impl ::core::fmt::Debug for WTS_USER_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_USER_DATA").field("WorkDirectory", &self.WorkDirectory).field("InitialProgram", &self.InitialProgram).field("UserTimeZone", &self.UserTimeZone).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WTS_VALIDATION_INFORMATIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WTS_VALIDATION_INFORMATIONA {
    fn eq(&self, other: &Self) -> bool {
        self.ProductInfo == other.ProductInfo && self.License == other.License && self.LicenseLength == other.LicenseLength && self.HardwareID == other.HardwareID && self.HardwareIDLength == other.HardwareIDLength
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WTS_VALIDATION_INFORMATIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WTS_VALIDATION_INFORMATIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_VALIDATION_INFORMATIONA").field("ProductInfo", &self.ProductInfo).field("License", &self.License).field("LicenseLength", &self.LicenseLength).field("HardwareID", &self.HardwareID).field("HardwareIDLength", &self.HardwareIDLength).finish()
    }
}
impl ::core::default::Default for WTS_VALIDATION_INFORMATIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WTS_VALIDATION_INFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.ProductInfo == other.ProductInfo && self.License == other.License && self.LicenseLength == other.LicenseLength && self.HardwareID == other.HardwareID && self.HardwareIDLength == other.HardwareIDLength
    }
}
impl ::core::cmp::Eq for WTS_VALIDATION_INFORMATIONW {}
impl ::core::fmt::Debug for WTS_VALIDATION_INFORMATIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WTS_VALIDATION_INFORMATIONW").field("ProductInfo", &self.ProductInfo).field("License", &self.License).field("LicenseLength", &self.LicenseLength).field("HardwareID", &self.HardwareID).field("HardwareIDLength", &self.HardwareIDLength).finish()
    }
}
impl ::core::default::Default for WTS_VIRTUAL_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WTS_VIRTUAL_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WTS_VIRTUAL_CLASS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for _ITSWkspEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for _ITSWkspEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for _ITSWkspEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_ITSWkspEvents").field(&self.0).finish()
    }
}
impl ::core::default::Default for pluginResource {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for pluginResource {
    fn eq(&self, other: &Self) -> bool {
        self.alias == other.alias && self.name == other.name && self.resourceFileContents == other.resourceFileContents && self.fileExtension == other.fileExtension && self.resourcePluginType == other.resourcePluginType && self.isDiscoverable == other.isDiscoverable && self.resourceType == other.resourceType && self.pceIconSize == other.pceIconSize && self.iconContents == other.iconContents && self.pcePluginBlobSize == other.pcePluginBlobSize && self.blobContents == other.blobContents
    }
}
impl ::core::cmp::Eq for pluginResource {}
impl ::core::fmt::Debug for pluginResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("pluginResource")
            .field("alias", &self.alias)
            .field("name", &self.name)
            .field("resourceFileContents", &self.resourceFileContents)
            .field("fileExtension", &self.fileExtension)
            .field("resourcePluginType", &self.resourcePluginType)
            .field("isDiscoverable", &self.isDiscoverable)
            .field("resourceType", &self.resourceType)
            .field("pceIconSize", &self.pceIconSize)
            .field("iconContents", &self.iconContents)
            .field("pcePluginBlobSize", &self.pcePluginBlobSize)
            .field("blobContents", &self.blobContents)
            .finish()
    }
}
impl ::core::default::Default for pluginResource2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for pluginResource2 {
    fn eq(&self, other: &Self) -> bool {
        self.resourceV1 == other.resourceV1 && self.pceFileAssocListSize == other.pceFileAssocListSize && self.fileAssocList == other.fileAssocList && self.securityDescriptor == other.securityDescriptor && self.pceFolderListSize == other.pceFolderListSize && self.folderList == other.folderList
    }
}
impl ::core::cmp::Eq for pluginResource2 {}
impl ::core::fmt::Debug for pluginResource2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("pluginResource2").field("resourceV1", &self.resourceV1).field("pceFileAssocListSize", &self.pceFileAssocListSize).field("fileAssocList", &self.fileAssocList).field("securityDescriptor", &self.securityDescriptor).field("pceFolderListSize", &self.pceFolderListSize).field("folderList", &self.folderList).finish()
    }
}
impl ::core::default::Default for pluginResource2FileAssociation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for pluginResource2FileAssociation {
    fn eq(&self, other: &Self) -> bool {
        self.extName == other.extName && self.primaryHandler == other.primaryHandler && self.pceIconSize == other.pceIconSize && self.iconContents == other.iconContents
    }
}
impl ::core::cmp::Eq for pluginResource2FileAssociation {}
impl ::core::fmt::Debug for pluginResource2FileAssociation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("pluginResource2FileAssociation").field("extName", &self.extName).field("primaryHandler", &self.primaryHandler).field("pceIconSize", &self.pceIconSize).field("iconContents", &self.iconContents).finish()
    }
}
