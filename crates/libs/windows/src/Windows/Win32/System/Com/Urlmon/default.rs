impl ::core::default::Default for AUTHENTICATEF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AUTHENTICATEF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHENTICATEF").field(&self.0).finish()
    }
}
impl ::core::default::Default for BINDF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BINDF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BINDF").field(&self.0).finish()
    }
}
impl ::core::default::Default for BINDF2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BINDF2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BINDF2").field(&self.0).finish()
    }
}
impl ::core::default::Default for BINDHANDLETYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BINDHANDLETYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BINDHANDLETYPES").field(&self.0).finish()
    }
}
impl ::core::default::Default for BINDINFO_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BINDINFO_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BINDINFO_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for BINDSTATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BINDSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BINDSTATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for BINDSTRING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BINDSTRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BINDSTRING").field(&self.0).finish()
    }
}
impl ::core::default::Default for BINDVERB {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BINDVERB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BINDVERB").field(&self.0).finish()
    }
}
impl ::core::default::Default for BSCF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BSCF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BSCF").field(&self.0).finish()
    }
}
impl ::core::default::Default for CIP_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CIP_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CIP_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CODEBASEHOLD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CODEBASEHOLD {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.szDistUnit == other.szDistUnit && self.szCodeBase == other.szCodeBase && self.dwVersionMS == other.dwVersionMS && self.dwVersionLS == other.dwVersionLS && self.dwStyle == other.dwStyle
    }
}
impl ::core::cmp::Eq for CODEBASEHOLD {}
impl ::core::fmt::Debug for CODEBASEHOLD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CODEBASEHOLD").field("cbSize", &self.cbSize).field("szDistUnit", &self.szDistUnit).field("szCodeBase", &self.szCodeBase).field("dwVersionMS", &self.dwVersionMS).field("dwVersionLS", &self.dwVersionLS).field("dwStyle", &self.dwStyle).finish()
    }
}
impl ::core::default::Default for CONFIRMSAFETY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CONFIRMSAFETY {
    fn eq(&self, other: &Self) -> bool {
        self.clsid == other.clsid && self.pUnk == other.pUnk && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for CONFIRMSAFETY {}
impl ::core::fmt::Debug for CONFIRMSAFETY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONFIRMSAFETY").field("clsid", &self.clsid).field("pUnk", &self.pUnk).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for DATAINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DATAINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ulTotalSize == other.ulTotalSize && self.ulavrPacketSize == other.ulavrPacketSize && self.ulConnectSpeed == other.ulConnectSpeed && self.ulProcessorSpeed == other.ulProcessorSpeed
    }
}
impl ::core::cmp::Eq for DATAINFO {}
impl ::core::fmt::Debug for DATAINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DATAINFO").field("ulTotalSize", &self.ulTotalSize).field("ulavrPacketSize", &self.ulavrPacketSize).field("ulConnectSpeed", &self.ulConnectSpeed).field("ulProcessorSpeed", &self.ulProcessorSpeed).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HIT_LOGGING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HIT_LOGGING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwStructSize == other.dwStructSize && self.lpszLoggedUrlName == other.lpszLoggedUrlName && self.StartTime == other.StartTime && self.EndTime == other.EndTime && self.lpszExtendedInfo == other.lpszExtendedInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HIT_LOGGING_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HIT_LOGGING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HIT_LOGGING_INFO").field("dwStructSize", &self.dwStructSize).field("lpszLoggedUrlName", &self.lpszLoggedUrlName).field("StartTime", &self.StartTime).field("EndTime", &self.EndTime).field("lpszExtendedInfo", &self.lpszExtendedInfo).finish()
    }
}
impl ::core::cmp::PartialEq for IBindCallbackRedirect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindCallbackRedirect {}
impl ::core::fmt::Debug for IBindCallbackRedirect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindCallbackRedirect").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IBindHttpSecurity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindHttpSecurity {}
impl ::core::fmt::Debug for IBindHttpSecurity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindHttpSecurity").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IBindProtocol {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindProtocol {}
impl ::core::fmt::Debug for IBindProtocol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindProtocol").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICatalogFileInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICatalogFileInfo {}
impl ::core::fmt::Debug for ICatalogFileInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICatalogFileInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICodeInstall {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICodeInstall {}
impl ::core::fmt::Debug for ICodeInstall {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICodeInstall").field(&self.0).finish()
    }
}
impl ICodeInstall {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self, rguidreason: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWindow)(::windows::core::Vtable::as_raw(self), rguidreason, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDataFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataFilter {}
impl ::core::fmt::Debug for IDataFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataFilter").field(&self.0).finish()
    }
}
impl ::core::default::Default for IEObjectType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IEObjectType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEObjectType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEncodingFilterFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEncodingFilterFactory {}
impl ::core::fmt::Debug for IEncodingFilterFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEncodingFilterFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGetBindHandle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetBindHandle {}
impl ::core::fmt::Debug for IGetBindHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetBindHandle").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IHttpNegotiate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHttpNegotiate {}
impl ::core::fmt::Debug for IHttpNegotiate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHttpNegotiate").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IHttpNegotiate2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHttpNegotiate2 {}
impl ::core::fmt::Debug for IHttpNegotiate2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHttpNegotiate2").field(&self.0).finish()
    }
}
impl IHttpNegotiate2 {
    pub unsafe fn BeginningTransaction<P0, P1>(&self, szurl: P0, szheaders: P1, dwreserved: u32) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.BeginningTransaction)(::windows::core::Vtable::as_raw(self), szurl.into().abi(), szheaders.into().abi(), dwreserved, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OnResponse<P0, P1>(&self, dwresponsecode: u32, szresponseheaders: P0, szrequestheaders: P1) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.OnResponse)(::windows::core::Vtable::as_raw(self), dwresponsecode, szresponseheaders.into().abi(), szrequestheaders.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IHttpNegotiate3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHttpNegotiate3 {}
impl ::core::fmt::Debug for IHttpNegotiate3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHttpNegotiate3").field(&self.0).finish()
    }
}
impl IHttpNegotiate3 {
    pub unsafe fn BeginningTransaction<P0, P1>(&self, szurl: P0, szheaders: P1, dwreserved: u32) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.BeginningTransaction)(::windows::core::Vtable::as_raw(self), szurl.into().abi(), szheaders.into().abi(), dwreserved, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OnResponse<P0, P1>(&self, dwresponsecode: u32, szresponseheaders: P0, szrequestheaders: P1) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.OnResponse)(::windows::core::Vtable::as_raw(self), dwresponsecode, szresponseheaders.into().abi(), szrequestheaders.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRootSecurityId(&self, pbsecurityid: *mut u8, pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRootSecurityId)(::windows::core::Vtable::as_raw(self), pbsecurityid, pcbsecurityid, dwreserved).ok()
    }
}
impl ::core::cmp::PartialEq for IHttpSecurity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHttpSecurity {}
impl ::core::fmt::Debug for IHttpSecurity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHttpSecurity").field(&self.0).finish()
    }
}
impl IHttpSecurity {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindow(&self, rguidreason: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWindow)(::windows::core::Vtable::as_raw(self), rguidreason, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IInternet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternet {}
impl ::core::fmt::Debug for IInternet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternet").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInternetBindInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetBindInfo {}
impl ::core::fmt::Debug for IInternetBindInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetBindInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInternetBindInfoEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetBindInfoEx {}
impl ::core::fmt::Debug for IInternetBindInfoEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetBindInfoEx").field(&self.0).finish()
    }
}
impl IInternetBindInfoEx {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut super::BINDINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBindInfo)(::windows::core::Vtable::as_raw(self), grfbindf, pbindinfo).ok()
    }
    pub unsafe fn GetBindString(&self, ulstringtype: u32, ppwzstr: *mut ::windows::core::PWSTR, cel: u32, pcelfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBindString)(::windows::core::Vtable::as_raw(self), ulstringtype, ppwzstr, cel, pcelfetched).ok()
    }
}
impl ::core::cmp::PartialEq for IInternetHostSecurityManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetHostSecurityManager {}
impl ::core::fmt::Debug for IInternetHostSecurityManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetHostSecurityManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInternetPriority {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetPriority {}
impl ::core::fmt::Debug for IInternetPriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetPriority").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInternetProtocol {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetProtocol {}
impl ::core::fmt::Debug for IInternetProtocol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetProtocol").field(&self.0).finish()
    }
}
impl IInternetProtocol {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Start<P0, P1, P2, P3>(&self, szurl: P0, poiprotsink: P1, poibindinfo: P2, grfpi: u32, dwreserved: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IInternetProtocolSink>>,
        P2: ::std::convert::Into<::windows::core::InParam<IInternetBindInfo>>,
        P3: ::std::convert::Into<super::super::super::Foundation::HANDLE_PTR>,
    {
        (::windows::core::Vtable::vtable(self).base__.Start)(::windows::core::Vtable::as_raw(self), szurl.into().abi(), poiprotsink.into().abi(), poibindinfo.into().abi(), grfpi, dwreserved.into()).ok()
    }
    pub unsafe fn Continue(&self, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Continue)(::windows::core::Vtable::as_raw(self), pprotocoldata).ok()
    }
    pub unsafe fn Abort(&self, hrreason: ::windows::core::HRESULT, dwoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Abort)(::windows::core::Vtable::as_raw(self), hrreason, dwoptions).ok()
    }
    pub unsafe fn Terminate(&self, dwoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Terminate)(::windows::core::Vtable::as_raw(self), dwoptions).ok()
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Suspend)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Resume)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IInternetProtocolEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetProtocolEx {}
impl ::core::fmt::Debug for IInternetProtocolEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetProtocolEx").field(&self.0).finish()
    }
}
impl IInternetProtocolEx {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Start<P0, P1, P2, P3>(&self, szurl: P0, poiprotsink: P1, poibindinfo: P2, grfpi: u32, dwreserved: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IInternetProtocolSink>>,
        P2: ::std::convert::Into<::windows::core::InParam<IInternetBindInfo>>,
        P3: ::std::convert::Into<super::super::super::Foundation::HANDLE_PTR>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Start)(::windows::core::Vtable::as_raw(self), szurl.into().abi(), poiprotsink.into().abi(), poibindinfo.into().abi(), grfpi, dwreserved.into()).ok()
    }
    pub unsafe fn Continue(&self, pprotocoldata: *const PROTOCOLDATA) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Continue)(::windows::core::Vtable::as_raw(self), pprotocoldata).ok()
    }
    pub unsafe fn Abort(&self, hrreason: ::windows::core::HRESULT, dwoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Abort)(::windows::core::Vtable::as_raw(self), hrreason, dwoptions).ok()
    }
    pub unsafe fn Terminate(&self, dwoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Terminate)(::windows::core::Vtable::as_raw(self), dwoptions).ok()
    }
    pub unsafe fn Suspend(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Suspend)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Resume)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Read(&self, pv: &mut [u8], pcbread: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Read)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pv.as_ptr()), pv.len() as _, pcbread).ok()
    }
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: u32) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Seek)(::windows::core::Vtable::as_raw(self), dlibmove, dworigin, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LockRequest(&self, dwoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.LockRequest)(::windows::core::Vtable::as_raw(self), dwoptions).ok()
    }
    pub unsafe fn UnlockRequest(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnlockRequest)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IInternetProtocolInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetProtocolInfo {}
impl ::core::fmt::Debug for IInternetProtocolInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetProtocolInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInternetProtocolRoot {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetProtocolRoot {}
impl ::core::fmt::Debug for IInternetProtocolRoot {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetProtocolRoot").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInternetProtocolSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetProtocolSink {}
impl ::core::fmt::Debug for IInternetProtocolSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetProtocolSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInternetProtocolSinkStackable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetProtocolSinkStackable {}
impl ::core::fmt::Debug for IInternetProtocolSinkStackable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetProtocolSinkStackable").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInternetSecurityManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetSecurityManager {}
impl ::core::fmt::Debug for IInternetSecurityManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetSecurityManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInternetSecurityManagerEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetSecurityManagerEx {}
impl ::core::fmt::Debug for IInternetSecurityManagerEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetSecurityManagerEx").field(&self.0).finish()
    }
}
impl IInternetSecurityManagerEx {
    pub unsafe fn SetSecuritySite<P0>(&self, psite: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IInternetSecurityMgrSite>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetSecuritySite)(::windows::core::Vtable::as_raw(self), psite.into().abi()).ok()
    }
    pub unsafe fn GetSecuritySite(&self) -> ::windows::core::Result<IInternetSecurityMgrSite> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSecuritySite)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MapUrlToZone<P0>(&self, pwszurl: P0, pdwzone: *mut u32, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.MapUrlToZone)(::windows::core::Vtable::as_raw(self), pwszurl.into().abi(), pdwzone, dwflags).ok()
    }
    pub unsafe fn GetSecurityId<P0>(&self, pwszurl: P0, pbsecurityid: &mut [u8; 512], pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetSecurityId)(::windows::core::Vtable::as_raw(self), pwszurl.into().abi(), ::core::mem::transmute(pbsecurityid.as_ptr()), pcbsecurityid, dwreserved).ok()
    }
    pub unsafe fn ProcessUrlAction<P0>(&self, pwszurl: P0, dwaction: u32, ppolicy: &mut [u8], pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ProcessUrlAction)(::windows::core::Vtable::as_raw(self), pwszurl.into().abi(), dwaction, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, pcontext, cbcontext, dwflags, dwreserved).ok()
    }
    pub unsafe fn QueryCustomPolicy<P0>(&self, pwszurl: P0, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.QueryCustomPolicy)(::windows::core::Vtable::as_raw(self), pwszurl.into().abi(), guidkey, pppolicy, pcbpolicy, pcontext, cbcontext, dwreserved).ok()
    }
    pub unsafe fn SetZoneMapping<P0>(&self, dwzone: u32, lpszpattern: P0, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetZoneMapping)(::windows::core::Vtable::as_raw(self), dwzone, lpszpattern.into().abi(), dwflags).ok()
    }
    pub unsafe fn GetZoneMappings(&self, dwzone: u32, ppenumstring: *mut ::core::option::Option<super::IEnumString>, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetZoneMappings)(::windows::core::Vtable::as_raw(self), dwzone, ::core::mem::transmute(ppenumstring), dwflags).ok()
    }
}
impl ::core::cmp::PartialEq for IInternetSecurityManagerEx2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetSecurityManagerEx2 {}
impl ::core::fmt::Debug for IInternetSecurityManagerEx2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetSecurityManagerEx2").field(&self.0).finish()
    }
}
impl IInternetSecurityManagerEx2 {
    pub unsafe fn SetSecuritySite<P0>(&self, psite: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IInternetSecurityMgrSite>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetSecuritySite)(::windows::core::Vtable::as_raw(self), psite.into().abi()).ok()
    }
    pub unsafe fn GetSecuritySite(&self) -> ::windows::core::Result<IInternetSecurityMgrSite> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetSecuritySite)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MapUrlToZone<P0>(&self, pwszurl: P0, pdwzone: *mut u32, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.MapUrlToZone)(::windows::core::Vtable::as_raw(self), pwszurl.into().abi(), pdwzone, dwflags).ok()
    }
    pub unsafe fn GetSecurityId<P0>(&self, pwszurl: P0, pbsecurityid: &mut [u8; 512], pcbsecurityid: *mut u32, dwreserved: usize) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetSecurityId)(::windows::core::Vtable::as_raw(self), pwszurl.into().abi(), ::core::mem::transmute(pbsecurityid.as_ptr()), pcbsecurityid, dwreserved).ok()
    }
    pub unsafe fn ProcessUrlAction<P0>(&self, pwszurl: P0, dwaction: u32, ppolicy: &mut [u8], pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ProcessUrlAction)(::windows::core::Vtable::as_raw(self), pwszurl.into().abi(), dwaction, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, pcontext, cbcontext, dwflags, dwreserved).ok()
    }
    pub unsafe fn QueryCustomPolicy<P0>(&self, pwszurl: P0, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, pcontext: *const u8, cbcontext: u32, dwreserved: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.QueryCustomPolicy)(::windows::core::Vtable::as_raw(self), pwszurl.into().abi(), guidkey, pppolicy, pcbpolicy, pcontext, cbcontext, dwreserved).ok()
    }
    pub unsafe fn SetZoneMapping<P0>(&self, dwzone: u32, lpszpattern: P0, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetZoneMapping)(::windows::core::Vtable::as_raw(self), dwzone, lpszpattern.into().abi(), dwflags).ok()
    }
    pub unsafe fn GetZoneMappings(&self, dwzone: u32, ppenumstring: *mut ::core::option::Option<super::IEnumString>, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetZoneMappings)(::windows::core::Vtable::as_raw(self), dwzone, ::core::mem::transmute(ppenumstring), dwflags).ok()
    }
    pub unsafe fn ProcessUrlActionEx<P0>(&self, pwszurl: P0, dwaction: u32, ppolicy: &mut [u8], pcontext: *const u8, cbcontext: u32, dwflags: u32, dwreserved: u32, pdwoutflags: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ProcessUrlActionEx)(::windows::core::Vtable::as_raw(self), pwszurl.into().abi(), dwaction, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, pcontext, cbcontext, dwflags, dwreserved, pdwoutflags).ok()
    }
}
impl ::core::cmp::PartialEq for IInternetSecurityMgrSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetSecurityMgrSite {}
impl ::core::fmt::Debug for IInternetSecurityMgrSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetSecurityMgrSite").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInternetSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetSession {}
impl ::core::fmt::Debug for IInternetSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetSession").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInternetThreadSwitch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetThreadSwitch {}
impl ::core::fmt::Debug for IInternetThreadSwitch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetThreadSwitch").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInternetZoneManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetZoneManager {}
impl ::core::fmt::Debug for IInternetZoneManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetZoneManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInternetZoneManagerEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetZoneManagerEx {}
impl ::core::fmt::Debug for IInternetZoneManagerEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetZoneManagerEx").field(&self.0).finish()
    }
}
impl IInternetZoneManagerEx {
    pub unsafe fn GetZoneAttributes(&self, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetZoneAttributes)(::windows::core::Vtable::as_raw(self), dwzone, pzoneattributes).ok()
    }
    pub unsafe fn SetZoneAttributes(&self, dwzone: u32, pzoneattributes: *const ZONEATTRIBUTES) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetZoneAttributes)(::windows::core::Vtable::as_raw(self), dwzone, pzoneattributes).ok()
    }
    pub unsafe fn GetZoneCustomPolicy(&self, dwzone: u32, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetZoneCustomPolicy)(::windows::core::Vtable::as_raw(self), dwzone, guidkey, pppolicy, pcbpolicy, urlzonereg).ok()
    }
    pub unsafe fn SetZoneCustomPolicy(&self, dwzone: u32, guidkey: *const ::windows::core::GUID, ppolicy: &[u8], urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetZoneCustomPolicy)(::windows::core::Vtable::as_raw(self), dwzone, guidkey, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, urlzonereg).ok()
    }
    pub unsafe fn GetZoneActionPolicy(&self, dwzone: u32, dwaction: u32, ppolicy: &mut [u8], urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetZoneActionPolicy)(::windows::core::Vtable::as_raw(self), dwzone, dwaction, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, urlzonereg).ok()
    }
    pub unsafe fn SetZoneActionPolicy(&self, dwzone: u32, dwaction: u32, ppolicy: &[u8], urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetZoneActionPolicy)(::windows::core::Vtable::as_raw(self), dwzone, dwaction, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, urlzonereg).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PromptAction<P0, P1, P2>(&self, dwaction: u32, hwndparent: P0, pwszurl: P1, pwsztext: P2, dwpromptflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.PromptAction)(::windows::core::Vtable::as_raw(self), dwaction, hwndparent.into(), pwszurl.into().abi(), pwsztext.into().abi(), dwpromptflags).ok()
    }
    pub unsafe fn LogAction<P0, P1>(&self, dwaction: u32, pwszurl: P0, pwsztext: P1, dwlogflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.LogAction)(::windows::core::Vtable::as_raw(self), dwaction, pwszurl.into().abi(), pwsztext.into().abi(), dwlogflags).ok()
    }
    pub unsafe fn CreateZoneEnumerator(&self, pdwenum: *mut u32, pdwcount: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateZoneEnumerator)(::windows::core::Vtable::as_raw(self), pdwenum, pdwcount, dwflags).ok()
    }
    pub unsafe fn GetZoneAt(&self, dwenum: u32, dwindex: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetZoneAt)(::windows::core::Vtable::as_raw(self), dwenum, dwindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DestroyZoneEnumerator(&self, dwenum: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DestroyZoneEnumerator)(::windows::core::Vtable::as_raw(self), dwenum).ok()
    }
    pub unsafe fn CopyTemplatePoliciesToZone(&self, dwtemplate: u32, dwzone: u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CopyTemplatePoliciesToZone)(::windows::core::Vtable::as_raw(self), dwtemplate, dwzone, dwreserved).ok()
    }
}
impl ::core::cmp::PartialEq for IInternetZoneManagerEx2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternetZoneManagerEx2 {}
impl ::core::fmt::Debug for IInternetZoneManagerEx2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternetZoneManagerEx2").field(&self.0).finish()
    }
}
impl IInternetZoneManagerEx2 {
    pub unsafe fn GetZoneAttributes(&self, dwzone: u32, pzoneattributes: *mut ZONEATTRIBUTES) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetZoneAttributes)(::windows::core::Vtable::as_raw(self), dwzone, pzoneattributes).ok()
    }
    pub unsafe fn SetZoneAttributes(&self, dwzone: u32, pzoneattributes: *const ZONEATTRIBUTES) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetZoneAttributes)(::windows::core::Vtable::as_raw(self), dwzone, pzoneattributes).ok()
    }
    pub unsafe fn GetZoneCustomPolicy(&self, dwzone: u32, guidkey: *const ::windows::core::GUID, pppolicy: *mut *mut u8, pcbpolicy: *mut u32, urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetZoneCustomPolicy)(::windows::core::Vtable::as_raw(self), dwzone, guidkey, pppolicy, pcbpolicy, urlzonereg).ok()
    }
    pub unsafe fn SetZoneCustomPolicy(&self, dwzone: u32, guidkey: *const ::windows::core::GUID, ppolicy: &[u8], urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetZoneCustomPolicy)(::windows::core::Vtable::as_raw(self), dwzone, guidkey, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, urlzonereg).ok()
    }
    pub unsafe fn GetZoneActionPolicy(&self, dwzone: u32, dwaction: u32, ppolicy: &mut [u8], urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetZoneActionPolicy)(::windows::core::Vtable::as_raw(self), dwzone, dwaction, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, urlzonereg).ok()
    }
    pub unsafe fn SetZoneActionPolicy(&self, dwzone: u32, dwaction: u32, ppolicy: &[u8], urlzonereg: URLZONEREG) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetZoneActionPolicy)(::windows::core::Vtable::as_raw(self), dwzone, dwaction, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, urlzonereg).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PromptAction<P0, P1, P2>(&self, dwaction: u32, hwndparent: P0, pwszurl: P1, pwsztext: P2, dwpromptflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.PromptAction)(::windows::core::Vtable::as_raw(self), dwaction, hwndparent.into(), pwszurl.into().abi(), pwsztext.into().abi(), dwpromptflags).ok()
    }
    pub unsafe fn LogAction<P0, P1>(&self, dwaction: u32, pwszurl: P0, pwsztext: P1, dwlogflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.LogAction)(::windows::core::Vtable::as_raw(self), dwaction, pwszurl.into().abi(), pwsztext.into().abi(), dwlogflags).ok()
    }
    pub unsafe fn CreateZoneEnumerator(&self, pdwenum: *mut u32, pdwcount: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateZoneEnumerator)(::windows::core::Vtable::as_raw(self), pdwenum, pdwcount, dwflags).ok()
    }
    pub unsafe fn GetZoneAt(&self, dwenum: u32, dwindex: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetZoneAt)(::windows::core::Vtable::as_raw(self), dwenum, dwindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DestroyZoneEnumerator(&self, dwenum: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.DestroyZoneEnumerator)(::windows::core::Vtable::as_raw(self), dwenum).ok()
    }
    pub unsafe fn CopyTemplatePoliciesToZone(&self, dwtemplate: u32, dwzone: u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CopyTemplatePoliciesToZone)(::windows::core::Vtable::as_raw(self), dwtemplate, dwzone, dwreserved).ok()
    }
    pub unsafe fn GetZoneActionPolicyEx(&self, dwzone: u32, dwaction: u32, ppolicy: &mut [u8], urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetZoneActionPolicyEx)(::windows::core::Vtable::as_raw(self), dwzone, dwaction, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, urlzonereg, dwflags).ok()
    }
    pub unsafe fn SetZoneActionPolicyEx(&self, dwzone: u32, dwaction: u32, ppolicy: &[u8], urlzonereg: URLZONEREG, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetZoneActionPolicyEx)(::windows::core::Vtable::as_raw(self), dwzone, dwaction, ::core::mem::transmute(ppolicy.as_ptr()), ppolicy.len() as _, urlzonereg, dwflags).ok()
    }
}
impl ::core::cmp::PartialEq for IMonikerProp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMonikerProp {}
impl ::core::fmt::Debug for IMonikerProp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMonikerProp").field(&self.0).finish()
    }
}
impl ::core::default::Default for INET_ZONE_MANAGER_CONSTANTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for INET_ZONE_MANAGER_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INET_ZONE_MANAGER_CONSTANTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for INTERNETFEATURELIST {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for INTERNETFEATURELIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INTERNETFEATURELIST").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPersistMoniker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPersistMoniker {}
impl ::core::fmt::Debug for IPersistMoniker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPersistMoniker").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISoftDistExt {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISoftDistExt {}
impl ::core::fmt::Debug for ISoftDistExt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISoftDistExt").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUriBuilderFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUriBuilderFactory {}
impl ::core::fmt::Debug for IUriBuilderFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUriBuilderFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUriContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUriContainer {}
impl ::core::fmt::Debug for IUriContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUriContainer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWinInetCacheHints {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWinInetCacheHints {}
impl ::core::fmt::Debug for IWinInetCacheHints {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWinInetCacheHints").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWinInetCacheHints2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWinInetCacheHints2 {}
impl ::core::fmt::Debug for IWinInetCacheHints2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWinInetCacheHints2").field(&self.0).finish()
    }
}
impl IWinInetCacheHints2 {
    pub unsafe fn SetCacheExtension<P0>(&self, pwzext: P0, pszcachefile: *mut ::core::ffi::c_void, pcbcachefile: *mut u32, pdwwinineterror: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCacheExtension)(::windows::core::Vtable::as_raw(self), pwzext.into().abi(), pszcachefile, pcbcachefile, pdwwinineterror, pdwreserved).ok()
    }
}
impl ::core::cmp::PartialEq for IWinInetFileStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWinInetFileStream {}
impl ::core::fmt::Debug for IWinInetFileStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWinInetFileStream").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWinInetHttpInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWinInetHttpInfo {}
impl ::core::fmt::Debug for IWinInetHttpInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWinInetHttpInfo").field(&self.0).finish()
    }
}
impl IWinInetHttpInfo {
    pub unsafe fn QueryOption(&self, dwoption: u32, pbuffer: *mut ::core::ffi::c_void, pcbbuf: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.QueryOption)(::windows::core::Vtable::as_raw(self), dwoption, pbuffer, pcbbuf).ok()
    }
}
impl ::core::cmp::PartialEq for IWinInetHttpTimeouts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWinInetHttpTimeouts {}
impl ::core::fmt::Debug for IWinInetHttpTimeouts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWinInetHttpTimeouts").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWinInetInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWinInetInfo {}
impl ::core::fmt::Debug for IWinInetInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWinInetInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWindowForBindingUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWindowForBindingUI {}
impl ::core::fmt::Debug for IWindowForBindingUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowForBindingUI").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWrappedProtocol {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWrappedProtocol {}
impl ::core::fmt::Debug for IWrappedProtocol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWrappedProtocol").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IZoneIdentifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IZoneIdentifier {}
impl ::core::fmt::Debug for IZoneIdentifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IZoneIdentifier").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IZoneIdentifier2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IZoneIdentifier2 {}
impl ::core::fmt::Debug for IZoneIdentifier2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IZoneIdentifier2").field(&self.0).finish()
    }
}
impl IZoneIdentifier2 {
    pub unsafe fn GetId(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetId(&self, dwzone: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetId)(::windows::core::Vtable::as_raw(self), dwzone).ok()
    }
    pub unsafe fn Remove(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Remove)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::default::Default for MONIKERPROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MONIKERPROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MONIKERPROPERTY").field(&self.0).finish()
    }
}
impl ::core::default::Default for OIBDG_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OIBDG_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OIBDG_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for PARSEACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PARSEACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PARSEACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for PI_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PI_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PI_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for PROTOCOLDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROTOCOLDATA {
    fn eq(&self, other: &Self) -> bool {
        self.grfFlags == other.grfFlags && self.dwState == other.dwState && self.pData == other.pData && self.cbData == other.cbData
    }
}
impl ::core::cmp::Eq for PROTOCOLDATA {}
impl ::core::fmt::Debug for PROTOCOLDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROTOCOLDATA").field("grfFlags", &self.grfFlags).field("dwState", &self.dwState).field("pData", &self.pData).field("cbData", &self.cbData).finish()
    }
}
impl ::core::default::Default for PROTOCOLFILTERDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROTOCOLFILTERDATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pProtocolSink == other.pProtocolSink && self.pProtocol == other.pProtocol && self.pUnk == other.pUnk && self.dwFilterFlags == other.dwFilterFlags
    }
}
impl ::core::cmp::Eq for PROTOCOLFILTERDATA {}
impl ::core::fmt::Debug for PROTOCOLFILTERDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROTOCOLFILTERDATA").field("cbSize", &self.cbSize).field("pProtocolSink", &self.pProtocolSink).field("pProtocol", &self.pProtocol).field("pUnk", &self.pUnk).field("dwFilterFlags", &self.dwFilterFlags).finish()
    }
}
impl ::core::default::Default for PROTOCOL_ARGUMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROTOCOL_ARGUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.szMethod == other.szMethod && self.szTargetUrl == other.szTargetUrl
    }
}
impl ::core::cmp::Eq for PROTOCOL_ARGUMENT {}
impl ::core::fmt::Debug for PROTOCOL_ARGUMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROTOCOL_ARGUMENT").field("szMethod", &self.szMethod).field("szTargetUrl", &self.szTargetUrl).finish()
    }
}
impl ::core::default::Default for PSUACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PSUACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PSUACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for PUAF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PUAF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PUAF").field(&self.0).finish()
    }
}
impl ::core::default::Default for PUAFOUT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PUAFOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PUAFOUT").field(&self.0).finish()
    }
}
impl ::core::default::Default for QUERYOPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for QUERYOPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUERYOPTION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REMSECURITY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REMSECURITY_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.nLength == other.nLength && self.lpSecurityDescriptor == other.lpSecurityDescriptor && self.bInheritHandle == other.bInheritHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REMSECURITY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for REMSECURITY_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REMSECURITY_ATTRIBUTES").field("nLength", &self.nLength).field("lpSecurityDescriptor", &self.lpSecurityDescriptor).field("bInheritHandle", &self.bInheritHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RemBINDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RemBINDINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.szExtraInfo == other.szExtraInfo && self.grfBindInfoF == other.grfBindInfoF && self.dwBindVerb == other.dwBindVerb && self.szCustomVerb == other.szCustomVerb && self.cbstgmedData == other.cbstgmedData && self.dwOptions == other.dwOptions && self.dwOptionsFlags == other.dwOptionsFlags && self.dwCodePage == other.dwCodePage && self.securityAttributes == other.securityAttributes && self.iid == other.iid && self.pUnk == other.pUnk && self.dwReserved == other.dwReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RemBINDINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RemBINDINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RemBINDINFO")
            .field("cbSize", &self.cbSize)
            .field("szExtraInfo", &self.szExtraInfo)
            .field("grfBindInfoF", &self.grfBindInfoF)
            .field("dwBindVerb", &self.dwBindVerb)
            .field("szCustomVerb", &self.szCustomVerb)
            .field("cbstgmedData", &self.cbstgmedData)
            .field("dwOptions", &self.dwOptions)
            .field("dwOptionsFlags", &self.dwOptionsFlags)
            .field("dwCodePage", &self.dwCodePage)
            .field("securityAttributes", &self.securityAttributes)
            .field("iid", &self.iid)
            .field("pUnk", &self.pUnk)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
impl ::core::default::Default for RemFORMATETC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RemFORMATETC {
    fn eq(&self, other: &Self) -> bool {
        self.cfFormat == other.cfFormat && self.ptd == other.ptd && self.dwAspect == other.dwAspect && self.lindex == other.lindex && self.tymed == other.tymed
    }
}
impl ::core::cmp::Eq for RemFORMATETC {}
impl ::core::fmt::Debug for RemFORMATETC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RemFORMATETC").field("cfFormat", &self.cfFormat).field("ptd", &self.ptd).field("dwAspect", &self.dwAspect).field("lindex", &self.lindex).field("tymed", &self.tymed).finish()
    }
}
impl ::core::default::Default for SOFTDISTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SOFTDISTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.dwAdState == other.dwAdState && self.szTitle == other.szTitle && self.szAbstract == other.szAbstract && self.szHREF == other.szHREF && self.dwInstalledVersionMS == other.dwInstalledVersionMS && self.dwInstalledVersionLS == other.dwInstalledVersionLS && self.dwUpdateVersionMS == other.dwUpdateVersionMS && self.dwUpdateVersionLS == other.dwUpdateVersionLS && self.dwAdvertisedVersionMS == other.dwAdvertisedVersionMS && self.dwAdvertisedVersionLS == other.dwAdvertisedVersionLS && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for SOFTDISTINFO {}
impl ::core::fmt::Debug for SOFTDISTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOFTDISTINFO")
            .field("cbSize", &self.cbSize)
            .field("dwFlags", &self.dwFlags)
            .field("dwAdState", &self.dwAdState)
            .field("szTitle", &self.szTitle)
            .field("szAbstract", &self.szAbstract)
            .field("szHREF", &self.szHREF)
            .field("dwInstalledVersionMS", &self.dwInstalledVersionMS)
            .field("dwInstalledVersionLS", &self.dwInstalledVersionLS)
            .field("dwUpdateVersionMS", &self.dwUpdateVersionMS)
            .field("dwUpdateVersionLS", &self.dwUpdateVersionLS)
            .field("dwAdvertisedVersionMS", &self.dwAdvertisedVersionMS)
            .field("dwAdvertisedVersionLS", &self.dwAdvertisedVersionLS)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
impl ::core::default::Default for SZM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SZM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SZM_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for StartParam {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for StartParam {
    fn eq(&self, other: &Self) -> bool {
        self.iid == other.iid && self.pIBindCtx == other.pIBindCtx && self.pItf == other.pItf
    }
}
impl ::core::cmp::Eq for StartParam {}
impl ::core::fmt::Debug for StartParam {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("StartParam").field("iid", &self.iid).field("pIBindCtx", &self.pIBindCtx).field("pItf", &self.pItf).finish()
    }
}
impl ::core::default::Default for URLTEMPLATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for URLTEMPLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URLTEMPLATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for URLZONE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for URLZONE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URLZONE").field(&self.0).finish()
    }
}
impl ::core::default::Default for URLZONEREG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for URLZONEREG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URLZONEREG").field(&self.0).finish()
    }
}
impl ::core::default::Default for URL_ENCODING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for URL_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("URL_ENCODING").field(&self.0).finish()
    }
}
impl ::core::default::Default for Uri_HOST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for Uri_HOST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Uri_HOST_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for ZAFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ZAFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ZAFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for ZONEATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ZONEATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.szDisplayName == other.szDisplayName && self.szDescription == other.szDescription && self.szIconPath == other.szIconPath && self.dwTemplateMinLevel == other.dwTemplateMinLevel && self.dwTemplateRecommended == other.dwTemplateRecommended && self.dwTemplateCurrentLevel == other.dwTemplateCurrentLevel && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for ZONEATTRIBUTES {}
impl ::core::fmt::Debug for ZONEATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ZONEATTRIBUTES").field("cbSize", &self.cbSize).field("szDisplayName", &self.szDisplayName).field("szDescription", &self.szDescription).field("szIconPath", &self.szIconPath).field("dwTemplateMinLevel", &self.dwTemplateMinLevel).field("dwTemplateRecommended", &self.dwTemplateRecommended).field("dwTemplateCurrentLevel", &self.dwTemplateCurrentLevel).field("dwFlags", &self.dwFlags).finish()
    }
}
