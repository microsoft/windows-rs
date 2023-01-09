impl ::core::default::Default for DeviceDiscoveryMechanism {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DeviceDiscoveryMechanism {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceDiscoveryMechanism").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWSDAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDAddress {}
impl ::core::fmt::Debug for IWSDAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDAddress").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWSDAsyncCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDAsyncCallback {}
impl ::core::fmt::Debug for IWSDAsyncCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDAsyncCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWSDAsyncResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDAsyncResult {}
impl ::core::fmt::Debug for IWSDAsyncResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDAsyncResult").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWSDAttachment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDAttachment {}
impl ::core::fmt::Debug for IWSDAttachment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDAttachment").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWSDDeviceHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDDeviceHost {}
impl ::core::fmt::Debug for IWSDDeviceHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDDeviceHost").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWSDDeviceHostNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDDeviceHostNotify {}
impl ::core::fmt::Debug for IWSDDeviceHostNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDDeviceHostNotify").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWSDDeviceProxy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDDeviceProxy {}
impl ::core::fmt::Debug for IWSDDeviceProxy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDDeviceProxy").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWSDEndpointProxy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDEndpointProxy {}
impl ::core::fmt::Debug for IWSDEndpointProxy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDEndpointProxy").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWSDEventingStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDEventingStatus {}
impl ::core::fmt::Debug for IWSDEventingStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDEventingStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWSDHttpAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDHttpAddress {}
impl ::core::fmt::Debug for IWSDHttpAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDHttpAddress").field(&self.0).finish()
    }
}
impl IWSDHttpAddress {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Serialize<P0>(&self, pszbuffer: &mut [u16], fsafe: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Serialize)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pszbuffer.as_ptr()), pszbuffer.len() as _, fsafe.into()).ok()
    }
    pub unsafe fn Deserialize<P0>(&self, pszbuffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Deserialize)(::windows::core::Vtable::as_raw(self), pszbuffer.into().abi()).ok()
    }
    pub unsafe fn GetPort(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPort)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPort(&self, wport: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPort)(::windows::core::Vtable::as_raw(self), wport).ok()
    }
    pub unsafe fn GetTransportAddress(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTransportAddress)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTransportAddressEx<P0>(&self, fsafe: P0) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTransportAddressEx)(::windows::core::Vtable::as_raw(self), fsafe.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransportAddress<P0>(&self, pszaddress: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTransportAddress)(::windows::core::Vtable::as_raw(self), pszaddress.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IWSDHttpAuthParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDHttpAuthParameters {}
impl ::core::fmt::Debug for IWSDHttpAuthParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDHttpAuthParameters").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWSDHttpMessageParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDHttpMessageParameters {}
impl ::core::fmt::Debug for IWSDHttpMessageParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDHttpMessageParameters").field(&self.0).finish()
    }
}
impl IWSDHttpMessageParameters {
    pub unsafe fn GetLocalAddress(&self) -> ::windows::core::Result<IWSDAddress> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLocalAddress)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLocalAddress<P0>(&self, paddress: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDAddress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetLocalAddress)(::windows::core::Vtable::as_raw(self), paddress.into().abi()).ok()
    }
    pub unsafe fn GetRemoteAddress(&self) -> ::windows::core::Result<IWSDAddress> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRemoteAddress)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRemoteAddress<P0>(&self, paddress: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDAddress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetRemoteAddress)(::windows::core::Vtable::as_raw(self), paddress.into().abi()).ok()
    }
    pub unsafe fn GetLowerParameters(&self) -> ::windows::core::Result<IWSDMessageParameters> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLowerParameters)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWSDInboundAttachment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDInboundAttachment {}
impl ::core::fmt::Debug for IWSDInboundAttachment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDInboundAttachment").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWSDMessageParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDMessageParameters {}
impl ::core::fmt::Debug for IWSDMessageParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDMessageParameters").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWSDMetadataExchange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDMetadataExchange {}
impl ::core::fmt::Debug for IWSDMetadataExchange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDMetadataExchange").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWSDOutboundAttachment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDOutboundAttachment {}
impl ::core::fmt::Debug for IWSDOutboundAttachment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDOutboundAttachment").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWSDSSLClientCertificate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDSSLClientCertificate {}
impl ::core::fmt::Debug for IWSDSSLClientCertificate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDSSLClientCertificate").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWSDScopeMatchingRule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDScopeMatchingRule {}
impl ::core::fmt::Debug for IWSDScopeMatchingRule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDScopeMatchingRule").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWSDServiceMessaging {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDServiceMessaging {}
impl ::core::fmt::Debug for IWSDServiceMessaging {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDServiceMessaging").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWSDServiceProxy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDServiceProxy {}
impl ::core::fmt::Debug for IWSDServiceProxy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDServiceProxy").field(&self.0).finish()
    }
}
impl IWSDServiceProxy {
    pub unsafe fn GetMetadata(&self) -> ::windows::core::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMetadata)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWSDServiceProxyEventing {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDServiceProxyEventing {}
impl ::core::fmt::Debug for IWSDServiceProxyEventing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDServiceProxyEventing").field(&self.0).finish()
    }
}
impl IWSDServiceProxyEventing {
    pub unsafe fn GetMetadata(&self) -> ::windows::core::Result<*mut WSD_METADATA_SECTION_LIST> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetMetadata)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BeginGetMetadata(&self) -> ::windows::core::Result<IWSDAsyncResult> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.BeginGetMetadata)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EndGetMetadata<P0>(&self, presult: P0) -> ::windows::core::Result<*mut WSD_METADATA_SECTION_LIST>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDAsyncResult>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EndGetMetadata)(::windows::core::Vtable::as_raw(self), presult.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetServiceMetadata(&self) -> ::windows::core::Result<*mut WSD_SERVICE_METADATA> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetServiceMetadata)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SubscribeToOperation<P0>(&self, poperation: *const WSD_OPERATION, punknown: P0, pany: *const WSDXML_ELEMENT, ppany: ::core::option::Option<*mut *mut WSDXML_ELEMENT>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SubscribeToOperation)(::windows::core::Vtable::as_raw(self), poperation, punknown.into().abi(), pany, ::core::mem::transmute(ppany.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn UnsubscribeToOperation(&self, poperation: *const WSD_OPERATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnsubscribeToOperation)(::windows::core::Vtable::as_raw(self), poperation).ok()
    }
    pub unsafe fn SetEventingStatusCallback<P0>(&self, pstatus: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDEventingStatus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetEventingStatusCallback)(::windows::core::Vtable::as_raw(self), pstatus.into().abi()).ok()
    }
    pub unsafe fn GetEndpointProxy(&self) -> ::windows::core::Result<IWSDEndpointProxy> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEndpointProxy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWSDSignatureProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDSignatureProperty {}
impl ::core::fmt::Debug for IWSDSignatureProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDSignatureProperty").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWSDTransportAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDTransportAddress {}
impl ::core::fmt::Debug for IWSDTransportAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDTransportAddress").field(&self.0).finish()
    }
}
impl IWSDTransportAddress {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Serialize<P0>(&self, pszbuffer: &mut [u16], fsafe: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Serialize)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pszbuffer.as_ptr()), pszbuffer.len() as _, fsafe.into()).ok()
    }
    pub unsafe fn Deserialize<P0>(&self, pszbuffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Deserialize)(::windows::core::Vtable::as_raw(self), pszbuffer.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IWSDUdpAddress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDUdpAddress {}
impl ::core::fmt::Debug for IWSDUdpAddress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDUdpAddress").field(&self.0).finish()
    }
}
impl IWSDUdpAddress {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Serialize<P0>(&self, pszbuffer: &mut [u16], fsafe: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Serialize)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pszbuffer.as_ptr()), pszbuffer.len() as _, fsafe.into()).ok()
    }
    pub unsafe fn Deserialize<P0>(&self, pszbuffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Deserialize)(::windows::core::Vtable::as_raw(self), pszbuffer.into().abi()).ok()
    }
    pub unsafe fn GetPort(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPort)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPort(&self, wport: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPort)(::windows::core::Vtable::as_raw(self), wport).ok()
    }
    pub unsafe fn GetTransportAddress(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTransportAddress)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTransportAddressEx<P0>(&self, fsafe: P0) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTransportAddressEx)(::windows::core::Vtable::as_raw(self), fsafe.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransportAddress<P0>(&self, pszaddress: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTransportAddress)(::windows::core::Vtable::as_raw(self), pszaddress.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IWSDUdpMessageParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDUdpMessageParameters {}
impl ::core::fmt::Debug for IWSDUdpMessageParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDUdpMessageParameters").field(&self.0).finish()
    }
}
impl IWSDUdpMessageParameters {
    pub unsafe fn GetLocalAddress(&self) -> ::windows::core::Result<IWSDAddress> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLocalAddress)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLocalAddress<P0>(&self, paddress: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDAddress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetLocalAddress)(::windows::core::Vtable::as_raw(self), paddress.into().abi()).ok()
    }
    pub unsafe fn GetRemoteAddress(&self) -> ::windows::core::Result<IWSDAddress> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRemoteAddress)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRemoteAddress<P0>(&self, paddress: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWSDAddress>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetRemoteAddress)(::windows::core::Vtable::as_raw(self), paddress.into().abi()).ok()
    }
    pub unsafe fn GetLowerParameters(&self) -> ::windows::core::Result<IWSDMessageParameters> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLowerParameters)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWSDXMLContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDXMLContext {}
impl ::core::fmt::Debug for IWSDXMLContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDXMLContext").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWSDiscoveredService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDiscoveredService {}
impl ::core::fmt::Debug for IWSDiscoveredService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDiscoveredService").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWSDiscoveryProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDiscoveryProvider {}
impl ::core::fmt::Debug for IWSDiscoveryProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDiscoveryProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWSDiscoveryProviderNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDiscoveryProviderNotify {}
impl ::core::fmt::Debug for IWSDiscoveryProviderNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDiscoveryProviderNotify").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWSDiscoveryPublisher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDiscoveryPublisher {}
impl ::core::fmt::Debug for IWSDiscoveryPublisher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDiscoveryPublisher").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWSDiscoveryPublisherNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSDiscoveryPublisherNotify {}
impl ::core::fmt::Debug for IWSDiscoveryPublisherNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSDiscoveryPublisherNotify").field(&self.0).finish()
    }
}
impl ::core::default::Default for REQUESTBODY_GetStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REQUESTBODY_GetStatus {
    fn eq(&self, other: &Self) -> bool {
        self.Any == other.Any
    }
}
impl ::core::cmp::Eq for REQUESTBODY_GetStatus {}
impl ::core::fmt::Debug for REQUESTBODY_GetStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REQUESTBODY_GetStatus").field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REQUESTBODY_Renew {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REQUESTBODY_Renew {
    fn eq(&self, other: &Self) -> bool {
        self.Expires == other.Expires && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REQUESTBODY_Renew {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for REQUESTBODY_Renew {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REQUESTBODY_Renew").field("Expires", &self.Expires).field("Any", &self.Any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REQUESTBODY_Subscribe {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REQUESTBODY_Subscribe {
    fn eq(&self, other: &Self) -> bool {
        self.EndTo == other.EndTo && self.Delivery == other.Delivery && self.Expires == other.Expires && self.Filter == other.Filter && self.Any == other.Any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REQUESTBODY_Subscribe {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for REQUESTBODY_Subscribe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REQUESTBODY_Subscribe").field("EndTo", &self.EndTo).field("Delivery", &self.Delivery).field("Expires", &self.Expires).field("Filter", &self.Filter).field("Any", &self.Any).finish()
    }
}
impl ::core::default::Default for REQUESTBODY_Unsubscribe {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REQUESTBODY_Unsubscribe {
    fn eq(&self, other: &Self) -> bool {
        self.any == other.any
    }
}
impl ::core::cmp::Eq for REQUESTBODY_Unsubscribe {}
impl ::core::fmt::Debug for REQUESTBODY_Unsubscribe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REQUESTBODY_Unsubscribe").field("any", &self.any).finish()
    }
}
impl ::core::default::Default for RESPONSEBODY_GetMetadata {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RESPONSEBODY_GetMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.Metadata == other.Metadata
    }
}
impl ::core::cmp::Eq for RESPONSEBODY_GetMetadata {}
impl ::core::fmt::Debug for RESPONSEBODY_GetMetadata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESPONSEBODY_GetMetadata").field("Metadata", &self.Metadata).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESPONSEBODY_GetStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RESPONSEBODY_GetStatus {
    fn eq(&self, other: &Self) -> bool {
        self.expires == other.expires && self.any == other.any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RESPONSEBODY_GetStatus {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RESPONSEBODY_GetStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESPONSEBODY_GetStatus").field("expires", &self.expires).field("any", &self.any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESPONSEBODY_Renew {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RESPONSEBODY_Renew {
    fn eq(&self, other: &Self) -> bool {
        self.expires == other.expires && self.any == other.any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RESPONSEBODY_Renew {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RESPONSEBODY_Renew {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESPONSEBODY_Renew").field("expires", &self.expires).field("any", &self.any).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESPONSEBODY_Subscribe {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RESPONSEBODY_Subscribe {
    fn eq(&self, other: &Self) -> bool {
        self.SubscriptionManager == other.SubscriptionManager && self.expires == other.expires && self.any == other.any
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RESPONSEBODY_Subscribe {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RESPONSEBODY_Subscribe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESPONSEBODY_Subscribe").field("SubscriptionManager", &self.SubscriptionManager).field("expires", &self.expires).field("any", &self.any).finish()
    }
}
impl ::core::default::Default for RESPONSEBODY_SubscriptionEnd {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RESPONSEBODY_SubscriptionEnd {
    fn eq(&self, other: &Self) -> bool {
        self.SubscriptionManager == other.SubscriptionManager && self.Status == other.Status && self.Reason == other.Reason && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for RESPONSEBODY_SubscriptionEnd {}
impl ::core::fmt::Debug for RESPONSEBODY_SubscriptionEnd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESPONSEBODY_SubscriptionEnd").field("SubscriptionManager", &self.SubscriptionManager).field("Status", &self.Status).field("Reason", &self.Reason).field("Any", &self.Any).finish()
    }
}
impl ::core::default::Default for WSDEventType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSDEventType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSDEventType").field(&self.0).finish()
    }
}
impl ::core::default::Default for WSDUdpMessageType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSDUdpMessageType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSDUdpMessageType").field(&self.0).finish()
    }
}
impl ::core::default::Default for WSDUdpRetransmitParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSDUdpRetransmitParams {
    fn eq(&self, other: &Self) -> bool {
        self.ulSendDelay == other.ulSendDelay && self.ulRepeat == other.ulRepeat && self.ulRepeatMinDelay == other.ulRepeatMinDelay && self.ulRepeatMaxDelay == other.ulRepeatMaxDelay && self.ulRepeatUpperDelay == other.ulRepeatUpperDelay
    }
}
impl ::core::cmp::Eq for WSDUdpRetransmitParams {}
impl ::core::fmt::Debug for WSDUdpRetransmitParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDUdpRetransmitParams").field("ulSendDelay", &self.ulSendDelay).field("ulRepeat", &self.ulRepeat).field("ulRepeatMinDelay", &self.ulRepeatMinDelay).field("ulRepeatMaxDelay", &self.ulRepeatMaxDelay).field("ulRepeatUpperDelay", &self.ulRepeatUpperDelay).finish()
    }
}
impl ::core::default::Default for WSDXML_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSDXML_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.Element == other.Element && self.Next == other.Next && self.Name == other.Name && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for WSDXML_ATTRIBUTE {}
impl ::core::fmt::Debug for WSDXML_ATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_ATTRIBUTE").field("Element", &self.Element).field("Next", &self.Next).field("Name", &self.Name).field("Value", &self.Value).finish()
    }
}
impl ::core::default::Default for WSDXML_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSDXML_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.Node == other.Node && self.Name == other.Name && self.FirstAttribute == other.FirstAttribute && self.FirstChild == other.FirstChild && self.PrefixMappings == other.PrefixMappings
    }
}
impl ::core::cmp::Eq for WSDXML_ELEMENT {}
impl ::core::fmt::Debug for WSDXML_ELEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_ELEMENT").field("Node", &self.Node).field("Name", &self.Name).field("FirstAttribute", &self.FirstAttribute).field("FirstChild", &self.FirstChild).field("PrefixMappings", &self.PrefixMappings).finish()
    }
}
impl ::core::default::Default for WSDXML_ELEMENT_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSDXML_ELEMENT_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
impl ::core::cmp::Eq for WSDXML_ELEMENT_LIST {}
impl ::core::fmt::Debug for WSDXML_ELEMENT_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_ELEMENT_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
impl ::core::default::Default for WSDXML_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSDXML_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.Space == other.Space && self.LocalName == other.LocalName
    }
}
impl ::core::cmp::Eq for WSDXML_NAME {}
impl ::core::fmt::Debug for WSDXML_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_NAME").field("Space", &self.Space).field("LocalName", &self.LocalName).finish()
    }
}
impl ::core::default::Default for WSDXML_NAMESPACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSDXML_NAMESPACE {
    fn eq(&self, other: &Self) -> bool {
        self.Uri == other.Uri && self.PreferredPrefix == other.PreferredPrefix && self.Names == other.Names && self.NamesCount == other.NamesCount && self.Encoding == other.Encoding
    }
}
impl ::core::cmp::Eq for WSDXML_NAMESPACE {}
impl ::core::fmt::Debug for WSDXML_NAMESPACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_NAMESPACE").field("Uri", &self.Uri).field("PreferredPrefix", &self.PreferredPrefix).field("Names", &self.Names).field("NamesCount", &self.NamesCount).field("Encoding", &self.Encoding).finish()
    }
}
impl ::core::default::Default for WSDXML_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSDXML_NODE {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Parent == other.Parent && self.Next == other.Next
    }
}
impl ::core::cmp::Eq for WSDXML_NODE {}
impl ::core::fmt::Debug for WSDXML_NODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_NODE").field("Type", &self.Type).field("Parent", &self.Parent).field("Next", &self.Next).finish()
    }
}
impl ::core::default::Default for WSDXML_OP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSDXML_OP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSDXML_OP").field(&self.0).finish()
    }
}
impl ::core::default::Default for WSDXML_PREFIX_MAPPING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSDXML_PREFIX_MAPPING {
    fn eq(&self, other: &Self) -> bool {
        self.Refs == other.Refs && self.Next == other.Next && self.Space == other.Space && self.Prefix == other.Prefix
    }
}
impl ::core::cmp::Eq for WSDXML_PREFIX_MAPPING {}
impl ::core::fmt::Debug for WSDXML_PREFIX_MAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_PREFIX_MAPPING").field("Refs", &self.Refs).field("Next", &self.Next).field("Space", &self.Space).field("Prefix", &self.Prefix).finish()
    }
}
impl ::core::default::Default for WSDXML_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSDXML_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.Node == other.Node && self.Text == other.Text
    }
}
impl ::core::cmp::Eq for WSDXML_TEXT {}
impl ::core::fmt::Debug for WSDXML_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_TEXT").field("Node", &self.Node).field("Text", &self.Text).finish()
    }
}
impl ::core::default::Default for WSDXML_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSDXML_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.Uri == other.Uri && self.Table == other.Table
    }
}
impl ::core::cmp::Eq for WSDXML_TYPE {}
impl ::core::fmt::Debug for WSDXML_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSDXML_TYPE").field("Uri", &self.Uri).field("Table", &self.Table).finish()
    }
}
impl ::core::default::Default for WSD_APP_SEQUENCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_APP_SEQUENCE {
    fn eq(&self, other: &Self) -> bool {
        self.InstanceId == other.InstanceId && self.SequenceId == other.SequenceId && self.MessageNumber == other.MessageNumber
    }
}
impl ::core::cmp::Eq for WSD_APP_SEQUENCE {}
impl ::core::fmt::Debug for WSD_APP_SEQUENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_APP_SEQUENCE").field("InstanceId", &self.InstanceId).field("SequenceId", &self.SequenceId).field("MessageNumber", &self.MessageNumber).finish()
    }
}
impl ::core::default::Default for WSD_BYE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_BYE {
    fn eq(&self, other: &Self) -> bool {
        self.EndpointReference == other.EndpointReference && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_BYE {}
impl ::core::fmt::Debug for WSD_BYE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_BYE").field("EndpointReference", &self.EndpointReference).field("Any", &self.Any).finish()
    }
}
impl ::core::default::Default for WSD_CONFIG_ADDRESSES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_CONFIG_ADDRESSES {
    fn eq(&self, other: &Self) -> bool {
        self.addresses == other.addresses && self.dwAddressCount == other.dwAddressCount
    }
}
impl ::core::cmp::Eq for WSD_CONFIG_ADDRESSES {}
impl ::core::fmt::Debug for WSD_CONFIG_ADDRESSES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_CONFIG_ADDRESSES").field("addresses", &self.addresses).field("dwAddressCount", &self.dwAddressCount).finish()
    }
}
impl ::core::default::Default for WSD_CONFIG_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_CONFIG_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.configParamType == other.configParamType && self.pConfigData == other.pConfigData && self.dwConfigDataSize == other.dwConfigDataSize
    }
}
impl ::core::cmp::Eq for WSD_CONFIG_PARAM {}
impl ::core::fmt::Debug for WSD_CONFIG_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_CONFIG_PARAM").field("configParamType", &self.configParamType).field("pConfigData", &self.pConfigData).field("dwConfigDataSize", &self.dwConfigDataSize).finish()
    }
}
impl ::core::default::Default for WSD_CONFIG_PARAM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSD_CONFIG_PARAM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSD_CONFIG_PARAM_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_DATETIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_DATETIME {
    fn eq(&self, other: &Self) -> bool {
        self.isPositive == other.isPositive && self.year == other.year && self.month == other.month && self.day == other.day && self.hour == other.hour && self.minute == other.minute && self.second == other.second && self.millisecond == other.millisecond && self.TZIsLocal == other.TZIsLocal && self.TZIsPositive == other.TZIsPositive && self.TZHour == other.TZHour && self.TZMinute == other.TZMinute
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_DATETIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_DATETIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_DATETIME").field("isPositive", &self.isPositive).field("year", &self.year).field("month", &self.month).field("day", &self.day).field("hour", &self.hour).field("minute", &self.minute).field("second", &self.second).field("millisecond", &self.millisecond).field("TZIsLocal", &self.TZIsLocal).field("TZIsPositive", &self.TZIsPositive).field("TZHour", &self.TZHour).field("TZMinute", &self.TZMinute).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_DURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_DURATION {
    fn eq(&self, other: &Self) -> bool {
        self.isPositive == other.isPositive && self.year == other.year && self.month == other.month && self.day == other.day && self.hour == other.hour && self.minute == other.minute && self.second == other.second && self.millisecond == other.millisecond
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_DURATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_DURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_DURATION").field("isPositive", &self.isPositive).field("year", &self.year).field("month", &self.month).field("day", &self.day).field("hour", &self.hour).field("minute", &self.minute).field("second", &self.second).field("millisecond", &self.millisecond).finish()
    }
}
impl ::core::default::Default for WSD_ENDPOINT_REFERENCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_ENDPOINT_REFERENCE {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address && self.ReferenceProperties == other.ReferenceProperties && self.ReferenceParameters == other.ReferenceParameters && self.PortType == other.PortType && self.ServiceName == other.ServiceName && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_ENDPOINT_REFERENCE {}
impl ::core::fmt::Debug for WSD_ENDPOINT_REFERENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_ENDPOINT_REFERENCE").field("Address", &self.Address).field("ReferenceProperties", &self.ReferenceProperties).field("ReferenceParameters", &self.ReferenceParameters).field("PortType", &self.PortType).field("ServiceName", &self.ServiceName).field("Any", &self.Any).finish()
    }
}
impl ::core::default::Default for WSD_ENDPOINT_REFERENCE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_ENDPOINT_REFERENCE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
impl ::core::cmp::Eq for WSD_ENDPOINT_REFERENCE_LIST {}
impl ::core::fmt::Debug for WSD_ENDPOINT_REFERENCE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_ENDPOINT_REFERENCE_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
impl ::core::default::Default for WSD_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WSD_EVENTING_DELIVERY_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_EVENTING_DELIVERY_MODE {
    fn eq(&self, other: &Self) -> bool {
        self.Mode == other.Mode && self.Push == other.Push && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for WSD_EVENTING_DELIVERY_MODE {}
impl ::core::fmt::Debug for WSD_EVENTING_DELIVERY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_EVENTING_DELIVERY_MODE").field("Mode", &self.Mode).field("Push", &self.Push).field("Data", &self.Data).finish()
    }
}
impl ::core::default::Default for WSD_EVENTING_DELIVERY_MODE_PUSH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_EVENTING_DELIVERY_MODE_PUSH {
    fn eq(&self, other: &Self) -> bool {
        self.NotifyTo == other.NotifyTo
    }
}
impl ::core::cmp::Eq for WSD_EVENTING_DELIVERY_MODE_PUSH {}
impl ::core::fmt::Debug for WSD_EVENTING_DELIVERY_MODE_PUSH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_EVENTING_DELIVERY_MODE_PUSH").field("NotifyTo", &self.NotifyTo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_EVENTING_EXPIRES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_EVENTING_EXPIRES {
    fn eq(&self, other: &Self) -> bool {
        self.Duration == other.Duration && self.DateTime == other.DateTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_EVENTING_EXPIRES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_EVENTING_EXPIRES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_EVENTING_EXPIRES").field("Duration", &self.Duration).field("DateTime", &self.DateTime).finish()
    }
}
impl ::core::default::Default for WSD_EVENTING_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_EVENTING_FILTER {
    fn eq(&self, other: &Self) -> bool {
        self.Dialect == other.Dialect && self.FilterAction == other.FilterAction && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for WSD_EVENTING_FILTER {}
impl ::core::fmt::Debug for WSD_EVENTING_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_EVENTING_FILTER").field("Dialect", &self.Dialect).field("FilterAction", &self.FilterAction).field("Data", &self.Data).finish()
    }
}
impl ::core::default::Default for WSD_EVENTING_FILTER_ACTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_EVENTING_FILTER_ACTION {
    fn eq(&self, other: &Self) -> bool {
        self.Actions == other.Actions
    }
}
impl ::core::cmp::Eq for WSD_EVENTING_FILTER_ACTION {}
impl ::core::fmt::Debug for WSD_EVENTING_FILTER_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_EVENTING_FILTER_ACTION").field("Actions", &self.Actions).finish()
    }
}
impl ::core::default::Default for WSD_HANDLER_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WSD_HEADER_RELATESTO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_HEADER_RELATESTO {
    fn eq(&self, other: &Self) -> bool {
        self.RelationshipType == other.RelationshipType && self.MessageID == other.MessageID
    }
}
impl ::core::cmp::Eq for WSD_HEADER_RELATESTO {}
impl ::core::fmt::Debug for WSD_HEADER_RELATESTO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_HEADER_RELATESTO").field("RelationshipType", &self.RelationshipType).field("MessageID", &self.MessageID).finish()
    }
}
impl ::core::default::Default for WSD_HELLO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_HELLO {
    fn eq(&self, other: &Self) -> bool {
        self.EndpointReference == other.EndpointReference && self.Types == other.Types && self.Scopes == other.Scopes && self.XAddrs == other.XAddrs && self.MetadataVersion == other.MetadataVersion && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_HELLO {}
impl ::core::fmt::Debug for WSD_HELLO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_HELLO").field("EndpointReference", &self.EndpointReference).field("Types", &self.Types).field("Scopes", &self.Scopes).field("XAddrs", &self.XAddrs).field("MetadataVersion", &self.MetadataVersion).field("Any", &self.Any).finish()
    }
}
impl ::core::default::Default for WSD_HOST_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_HOST_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.Host == other.Host && self.Hosted == other.Hosted
    }
}
impl ::core::cmp::Eq for WSD_HOST_METADATA {}
impl ::core::fmt::Debug for WSD_HOST_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_HOST_METADATA").field("Host", &self.Host).field("Hosted", &self.Hosted).finish()
    }
}
impl ::core::default::Default for WSD_LOCALIZED_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_LOCALIZED_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.lang == other.lang && self.String == other.String
    }
}
impl ::core::cmp::Eq for WSD_LOCALIZED_STRING {}
impl ::core::fmt::Debug for WSD_LOCALIZED_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_LOCALIZED_STRING").field("lang", &self.lang).field("String", &self.String).finish()
    }
}
impl ::core::default::Default for WSD_LOCALIZED_STRING_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_LOCALIZED_STRING_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
impl ::core::cmp::Eq for WSD_LOCALIZED_STRING_LIST {}
impl ::core::fmt::Debug for WSD_LOCALIZED_STRING_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_LOCALIZED_STRING_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
impl ::core::default::Default for WSD_METADATA_SECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_METADATA_SECTION {
    fn eq(&self, other: &Self) -> bool {
        self.Dialect == other.Dialect && self.Identifier == other.Identifier && self.Data == other.Data && self.MetadataReference == other.MetadataReference && self.Location == other.Location && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_METADATA_SECTION {}
impl ::core::fmt::Debug for WSD_METADATA_SECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_METADATA_SECTION").field("Dialect", &self.Dialect).field("Identifier", &self.Identifier).field("Data", &self.Data).field("MetadataReference", &self.MetadataReference).field("Location", &self.Location).field("Any", &self.Any).finish()
    }
}
impl ::core::default::Default for WSD_METADATA_SECTION_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_METADATA_SECTION_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
impl ::core::cmp::Eq for WSD_METADATA_SECTION_LIST {}
impl ::core::fmt::Debug for WSD_METADATA_SECTION_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_METADATA_SECTION_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
impl ::core::default::Default for WSD_NAME_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_NAME_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
impl ::core::cmp::Eq for WSD_NAME_LIST {}
impl ::core::fmt::Debug for WSD_NAME_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_NAME_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
impl ::core::default::Default for WSD_OPERATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WSD_PORT_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_PORT_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.EncodedName == other.EncodedName && self.OperationCount == other.OperationCount && self.Operations == other.Operations && self.ProtocolType == other.ProtocolType
    }
}
impl ::core::cmp::Eq for WSD_PORT_TYPE {}
impl ::core::fmt::Debug for WSD_PORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_PORT_TYPE").field("EncodedName", &self.EncodedName).field("OperationCount", &self.OperationCount).field("Operations", &self.Operations).field("ProtocolType", &self.ProtocolType).finish()
    }
}
impl ::core::default::Default for WSD_PROBE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_PROBE {
    fn eq(&self, other: &Self) -> bool {
        self.Types == other.Types && self.Scopes == other.Scopes && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_PROBE {}
impl ::core::fmt::Debug for WSD_PROBE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_PROBE").field("Types", &self.Types).field("Scopes", &self.Scopes).field("Any", &self.Any).finish()
    }
}
impl ::core::default::Default for WSD_PROBE_MATCH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_PROBE_MATCH {
    fn eq(&self, other: &Self) -> bool {
        self.EndpointReference == other.EndpointReference && self.Types == other.Types && self.Scopes == other.Scopes && self.XAddrs == other.XAddrs && self.MetadataVersion == other.MetadataVersion && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_PROBE_MATCH {}
impl ::core::fmt::Debug for WSD_PROBE_MATCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_PROBE_MATCH").field("EndpointReference", &self.EndpointReference).field("Types", &self.Types).field("Scopes", &self.Scopes).field("XAddrs", &self.XAddrs).field("MetadataVersion", &self.MetadataVersion).field("Any", &self.Any).finish()
    }
}
impl ::core::default::Default for WSD_PROBE_MATCHES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_PROBE_MATCHES {
    fn eq(&self, other: &Self) -> bool {
        self.ProbeMatch == other.ProbeMatch && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_PROBE_MATCHES {}
impl ::core::fmt::Debug for WSD_PROBE_MATCHES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_PROBE_MATCHES").field("ProbeMatch", &self.ProbeMatch).field("Any", &self.Any).finish()
    }
}
impl ::core::default::Default for WSD_PROBE_MATCH_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_PROBE_MATCH_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
impl ::core::cmp::Eq for WSD_PROBE_MATCH_LIST {}
impl ::core::fmt::Debug for WSD_PROBE_MATCH_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_PROBE_MATCH_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
impl ::core::default::Default for WSD_PROTOCOL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSD_PROTOCOL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSD_PROTOCOL_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WSD_REFERENCE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_REFERENCE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_REFERENCE_PARAMETERS {}
impl ::core::fmt::Debug for WSD_REFERENCE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_REFERENCE_PARAMETERS").field("Any", &self.Any).finish()
    }
}
impl ::core::default::Default for WSD_REFERENCE_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_REFERENCE_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_REFERENCE_PROPERTIES {}
impl ::core::fmt::Debug for WSD_REFERENCE_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_REFERENCE_PROPERTIES").field("Any", &self.Any).finish()
    }
}
impl ::core::default::Default for WSD_RELATIONSHIP_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_RELATIONSHIP_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Data == other.Data && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_RELATIONSHIP_METADATA {}
impl ::core::fmt::Debug for WSD_RELATIONSHIP_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_RELATIONSHIP_METADATA").field("Type", &self.Type).field("Data", &self.Data).field("Any", &self.Any).finish()
    }
}
impl ::core::default::Default for WSD_RESOLVE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_RESOLVE {
    fn eq(&self, other: &Self) -> bool {
        self.EndpointReference == other.EndpointReference && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_RESOLVE {}
impl ::core::fmt::Debug for WSD_RESOLVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_RESOLVE").field("EndpointReference", &self.EndpointReference).field("Any", &self.Any).finish()
    }
}
impl ::core::default::Default for WSD_RESOLVE_MATCH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_RESOLVE_MATCH {
    fn eq(&self, other: &Self) -> bool {
        self.EndpointReference == other.EndpointReference && self.Types == other.Types && self.Scopes == other.Scopes && self.XAddrs == other.XAddrs && self.MetadataVersion == other.MetadataVersion && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_RESOLVE_MATCH {}
impl ::core::fmt::Debug for WSD_RESOLVE_MATCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_RESOLVE_MATCH").field("EndpointReference", &self.EndpointReference).field("Types", &self.Types).field("Scopes", &self.Scopes).field("XAddrs", &self.XAddrs).field("MetadataVersion", &self.MetadataVersion).field("Any", &self.Any).finish()
    }
}
impl ::core::default::Default for WSD_RESOLVE_MATCHES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_RESOLVE_MATCHES {
    fn eq(&self, other: &Self) -> bool {
        self.ResolveMatch == other.ResolveMatch && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_RESOLVE_MATCHES {}
impl ::core::fmt::Debug for WSD_RESOLVE_MATCHES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_RESOLVE_MATCHES").field("ResolveMatch", &self.ResolveMatch).field("Any", &self.Any).finish()
    }
}
impl ::core::default::Default for WSD_SCOPES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_SCOPES {
    fn eq(&self, other: &Self) -> bool {
        self.MatchBy == other.MatchBy && self.Scopes == other.Scopes
    }
}
impl ::core::cmp::Eq for WSD_SCOPES {}
impl ::core::fmt::Debug for WSD_SCOPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SCOPES").field("MatchBy", &self.MatchBy).field("Scopes", &self.Scopes).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WSD_SECURITY_CERT_VALIDATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for WSD_SECURITY_CERT_VALIDATION {
    fn eq(&self, other: &Self) -> bool {
        self.certMatchArray == other.certMatchArray && self.dwCertMatchArrayCount == other.dwCertMatchArrayCount && self.hCertMatchStore == other.hCertMatchStore && self.hCertIssuerStore == other.hCertIssuerStore && self.dwCertCheckOptions == other.dwCertCheckOptions && self.pszCNGHashAlgId == other.pszCNGHashAlgId && self.pbCertHash == other.pbCertHash && self.dwCertHashSize == other.dwCertHashSize
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for WSD_SECURITY_CERT_VALIDATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for WSD_SECURITY_CERT_VALIDATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SECURITY_CERT_VALIDATION").field("certMatchArray", &self.certMatchArray).field("dwCertMatchArrayCount", &self.dwCertMatchArrayCount).field("hCertMatchStore", &self.hCertMatchStore).field("hCertIssuerStore", &self.hCertIssuerStore).field("dwCertCheckOptions", &self.dwCertCheckOptions).field("pszCNGHashAlgId", &self.pszCNGHashAlgId).field("pbCertHash", &self.pbCertHash).field("dwCertHashSize", &self.dwCertHashSize).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WSD_SECURITY_CERT_VALIDATION_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for WSD_SECURITY_CERT_VALIDATION_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.certMatchArray == other.certMatchArray && self.dwCertMatchArrayCount == other.dwCertMatchArrayCount && self.hCertMatchStore == other.hCertMatchStore && self.hCertIssuerStore == other.hCertIssuerStore && self.dwCertCheckOptions == other.dwCertCheckOptions
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for WSD_SECURITY_CERT_VALIDATION_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for WSD_SECURITY_CERT_VALIDATION_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SECURITY_CERT_VALIDATION_V1").field("certMatchArray", &self.certMatchArray).field("dwCertMatchArrayCount", &self.dwCertMatchArrayCount).field("hCertMatchStore", &self.hCertMatchStore).field("hCertIssuerStore", &self.hCertIssuerStore).field("dwCertCheckOptions", &self.dwCertCheckOptions).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for WSD_SECURITY_SIGNATURE_VALIDATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for WSD_SECURITY_SIGNATURE_VALIDATION {
    fn eq(&self, other: &Self) -> bool {
        self.signingCertArray == other.signingCertArray && self.dwSigningCertArrayCount == other.dwSigningCertArrayCount && self.hSigningCertStore == other.hSigningCertStore && self.dwFlags == other.dwFlags
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for WSD_SECURITY_SIGNATURE_VALIDATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for WSD_SECURITY_SIGNATURE_VALIDATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SECURITY_SIGNATURE_VALIDATION").field("signingCertArray", &self.signingCertArray).field("dwSigningCertArrayCount", &self.dwSigningCertArrayCount).field("hSigningCertStore", &self.hSigningCertStore).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for WSD_SERVICE_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_SERVICE_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.EndpointReference == other.EndpointReference && self.Types == other.Types && self.ServiceId == other.ServiceId && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_SERVICE_METADATA {}
impl ::core::fmt::Debug for WSD_SERVICE_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SERVICE_METADATA").field("EndpointReference", &self.EndpointReference).field("Types", &self.Types).field("ServiceId", &self.ServiceId).field("Any", &self.Any).finish()
    }
}
impl ::core::default::Default for WSD_SERVICE_METADATA_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_SERVICE_METADATA_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
impl ::core::cmp::Eq for WSD_SERVICE_METADATA_LIST {}
impl ::core::fmt::Debug for WSD_SERVICE_METADATA_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SERVICE_METADATA_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
impl ::core::default::Default for WSD_SOAP_FAULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_SOAP_FAULT {
    fn eq(&self, other: &Self) -> bool {
        self.Code == other.Code && self.Reason == other.Reason && self.Node == other.Node && self.Role == other.Role && self.Detail == other.Detail
    }
}
impl ::core::cmp::Eq for WSD_SOAP_FAULT {}
impl ::core::fmt::Debug for WSD_SOAP_FAULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SOAP_FAULT").field("Code", &self.Code).field("Reason", &self.Reason).field("Node", &self.Node).field("Role", &self.Role).field("Detail", &self.Detail).finish()
    }
}
impl ::core::default::Default for WSD_SOAP_FAULT_CODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_SOAP_FAULT_CODE {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value && self.Subcode == other.Subcode
    }
}
impl ::core::cmp::Eq for WSD_SOAP_FAULT_CODE {}
impl ::core::fmt::Debug for WSD_SOAP_FAULT_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SOAP_FAULT_CODE").field("Value", &self.Value).field("Subcode", &self.Subcode).finish()
    }
}
impl ::core::default::Default for WSD_SOAP_FAULT_REASON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_SOAP_FAULT_REASON {
    fn eq(&self, other: &Self) -> bool {
        self.Text == other.Text
    }
}
impl ::core::cmp::Eq for WSD_SOAP_FAULT_REASON {}
impl ::core::fmt::Debug for WSD_SOAP_FAULT_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SOAP_FAULT_REASON").field("Text", &self.Text).finish()
    }
}
impl ::core::default::Default for WSD_SOAP_FAULT_SUBCODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_SOAP_FAULT_SUBCODE {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value && self.Subcode == other.Subcode
    }
}
impl ::core::cmp::Eq for WSD_SOAP_FAULT_SUBCODE {}
impl ::core::fmt::Debug for WSD_SOAP_FAULT_SUBCODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SOAP_FAULT_SUBCODE").field("Value", &self.Value).field("Subcode", &self.Subcode).finish()
    }
}
impl ::core::default::Default for WSD_SOAP_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_SOAP_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.To == other.To && self.Action == other.Action && self.MessageID == other.MessageID && self.RelatesTo == other.RelatesTo && self.ReplyTo == other.ReplyTo && self.From == other.From && self.FaultTo == other.FaultTo && self.AppSequence == other.AppSequence && self.AnyHeaders == other.AnyHeaders
    }
}
impl ::core::cmp::Eq for WSD_SOAP_HEADER {}
impl ::core::fmt::Debug for WSD_SOAP_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SOAP_HEADER").field("To", &self.To).field("Action", &self.Action).field("MessageID", &self.MessageID).field("RelatesTo", &self.RelatesTo).field("ReplyTo", &self.ReplyTo).field("From", &self.From).field("FaultTo", &self.FaultTo).field("AppSequence", &self.AppSequence).field("AnyHeaders", &self.AnyHeaders).finish()
    }
}
impl ::core::default::Default for WSD_SOAP_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_SOAP_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Body == other.Body && self.BodyType == other.BodyType
    }
}
impl ::core::cmp::Eq for WSD_SOAP_MESSAGE {}
impl ::core::fmt::Debug for WSD_SOAP_MESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SOAP_MESSAGE").field("Header", &self.Header).field("Body", &self.Body).field("BodyType", &self.BodyType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.hr == other.hr && self.eventHandle == other.eventHandle && self.messageParameters == other.messageParameters && self.results == other.results
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_SYNCHRONOUS_RESPONSE_CONTEXT").field("hr", &self.hr).field("eventHandle", &self.eventHandle).field("messageParameters", &self.messageParameters).field("results", &self.results).finish()
    }
}
impl ::core::default::Default for WSD_THIS_DEVICE_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_THIS_DEVICE_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.FriendlyName == other.FriendlyName && self.FirmwareVersion == other.FirmwareVersion && self.SerialNumber == other.SerialNumber && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_THIS_DEVICE_METADATA {}
impl ::core::fmt::Debug for WSD_THIS_DEVICE_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_THIS_DEVICE_METADATA").field("FriendlyName", &self.FriendlyName).field("FirmwareVersion", &self.FirmwareVersion).field("SerialNumber", &self.SerialNumber).field("Any", &self.Any).finish()
    }
}
impl ::core::default::Default for WSD_THIS_MODEL_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_THIS_MODEL_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.Manufacturer == other.Manufacturer && self.ManufacturerUrl == other.ManufacturerUrl && self.ModelName == other.ModelName && self.ModelNumber == other.ModelNumber && self.ModelUrl == other.ModelUrl && self.PresentationUrl == other.PresentationUrl && self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_THIS_MODEL_METADATA {}
impl ::core::fmt::Debug for WSD_THIS_MODEL_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_THIS_MODEL_METADATA").field("Manufacturer", &self.Manufacturer).field("ManufacturerUrl", &self.ManufacturerUrl).field("ModelName", &self.ModelName).field("ModelNumber", &self.ModelNumber).field("ModelUrl", &self.ModelUrl).field("PresentationUrl", &self.PresentationUrl).field("Any", &self.Any).finish()
    }
}
impl ::core::default::Default for WSD_UNKNOWN_LOOKUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_UNKNOWN_LOOKUP {
    fn eq(&self, other: &Self) -> bool {
        self.Any == other.Any
    }
}
impl ::core::cmp::Eq for WSD_UNKNOWN_LOOKUP {}
impl ::core::fmt::Debug for WSD_UNKNOWN_LOOKUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_UNKNOWN_LOOKUP").field("Any", &self.Any).finish()
    }
}
impl ::core::default::Default for WSD_URI_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSD_URI_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.Element == other.Element
    }
}
impl ::core::cmp::Eq for WSD_URI_LIST {}
impl ::core::fmt::Debug for WSD_URI_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSD_URI_LIST").field("Next", &self.Next).field("Element", &self.Element).finish()
    }
}
