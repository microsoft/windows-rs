#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for CONVCONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for CONVCONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.wFlags == other.wFlags && self.wCountryID == other.wCountryID && self.iCodePage == other.iCodePage && self.dwLangID == other.dwLangID && self.dwSecurity == other.dwSecurity && self.qos == other.qos
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for CONVCONTEXT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for CONVCONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONVCONTEXT").field("cb", &self.cb).field("wFlags", &self.wFlags).field("wCountryID", &self.wCountryID).field("iCodePage", &self.iCodePage).field("dwLangID", &self.dwLangID).field("dwSecurity", &self.dwSecurity).field("qos", &self.qos).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for CONVINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for CONVINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.hUser == other.hUser && self.hConvPartner == other.hConvPartner && self.hszSvcPartner == other.hszSvcPartner && self.hszServiceReq == other.hszServiceReq && self.hszTopic == other.hszTopic && self.hszItem == other.hszItem && self.wFmt == other.wFmt && self.wType == other.wType && self.wStatus == other.wStatus && self.wConvst == other.wConvst && self.wLastError == other.wLastError && self.hConvList == other.hConvList && self.ConvCtxt == other.ConvCtxt && self.hwnd == other.hwnd && self.hwndPartner == other.hwndPartner
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for CONVINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for CONVINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONVINFO")
            .field("cb", &self.cb)
            .field("hUser", &self.hUser)
            .field("hConvPartner", &self.hConvPartner)
            .field("hszSvcPartner", &self.hszSvcPartner)
            .field("hszServiceReq", &self.hszServiceReq)
            .field("hszTopic", &self.hszTopic)
            .field("hszItem", &self.hszItem)
            .field("wFmt", &self.wFmt)
            .field("wType", &self.wType)
            .field("wStatus", &self.wStatus)
            .field("wConvst", &self.wConvst)
            .field("wLastError", &self.wLastError)
            .field("hConvList", &self.hConvList)
            .field("ConvCtxt", &self.ConvCtxt)
            .field("hwnd", &self.hwnd)
            .field("hwndPartner", &self.hwndPartner)
            .finish()
    }
}
impl ::core::default::Default for CONVINFO_CONVERSATION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CONVINFO_CONVERSATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONVINFO_CONVERSATION_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CONVINFO_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CONVINFO_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONVINFO_STATUS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CONVINFO_STATUS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CONVINFO_STATUS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CONVINFO_STATUS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CONVINFO_STATUS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CONVINFO_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for COPYDATASTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for COPYDATASTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.dwData == other.dwData && self.cbData == other.cbData && self.lpData == other.lpData
    }
}
impl ::core::cmp::Eq for COPYDATASTRUCT {}
impl ::core::fmt::Debug for COPYDATASTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COPYDATASTRUCT").field("dwData", &self.dwData).field("cbData", &self.cbData).field("lpData", &self.lpData).finish()
    }
}
impl ::core::default::Default for DDEACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDEACK {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for DDEACK {}
impl ::core::fmt::Debug for DDEACK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDEACK").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for DDEADVISE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDEADVISE {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield && self.cfFormat == other.cfFormat
    }
}
impl ::core::cmp::Eq for DDEADVISE {}
impl ::core::fmt::Debug for DDEADVISE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDEADVISE").field("_bitfield", &self._bitfield).field("cfFormat", &self.cfFormat).finish()
    }
}
impl ::core::default::Default for DDEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDEDATA {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield && self.cfFormat == other.cfFormat && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for DDEDATA {}
impl ::core::fmt::Debug for DDEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDEDATA").field("_bitfield", &self._bitfield).field("cfFormat", &self.cfFormat).field("Value", &self.Value).finish()
    }
}
impl ::core::default::Default for DDELN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDELN {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield && self.cfFormat == other.cfFormat
    }
}
impl ::core::cmp::Eq for DDELN {}
impl ::core::fmt::Debug for DDELN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDELN").field("_bitfield", &self._bitfield).field("cfFormat", &self.cfFormat).finish()
    }
}
impl ::core::default::Default for DDEML_MSG_HOOK_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDEML_MSG_HOOK_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.uiLo == other.uiLo && self.uiHi == other.uiHi && self.cbData == other.cbData && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for DDEML_MSG_HOOK_DATA {}
impl ::core::fmt::Debug for DDEML_MSG_HOOK_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDEML_MSG_HOOK_DATA").field("uiLo", &self.uiLo).field("uiHi", &self.uiHi).field("cbData", &self.cbData).field("Data", &self.Data).finish()
    }
}
impl ::core::default::Default for DDEPOKE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDEPOKE {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield && self.cfFormat == other.cfFormat && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for DDEPOKE {}
impl ::core::fmt::Debug for DDEPOKE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDEPOKE").field("_bitfield", &self._bitfield).field("cfFormat", &self.cfFormat).field("Value", &self.Value).finish()
    }
}
impl ::core::default::Default for DDEUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DDEUP {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield && self.cfFormat == other.cfFormat && self.rgb == other.rgb
    }
}
impl ::core::cmp::Eq for DDEUP {}
impl ::core::fmt::Debug for DDEUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDEUP").field("_bitfield", &self._bitfield).field("cfFormat", &self.cfFormat).field("rgb", &self.rgb).finish()
    }
}
impl ::core::default::Default for DDE_CLIENT_TRANSACTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DDE_CLIENT_TRANSACTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DDE_CLIENT_TRANSACTION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DDE_ENABLE_CALLBACK_CMD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DDE_ENABLE_CALLBACK_CMD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DDE_ENABLE_CALLBACK_CMD").field(&self.0).finish()
    }
}
impl ::core::default::Default for DDE_INITIALIZE_COMMAND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DDE_INITIALIZE_COMMAND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DDE_INITIALIZE_COMMAND").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DDE_INITIALIZE_COMMAND {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DDE_INITIALIZE_COMMAND {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DDE_INITIALIZE_COMMAND {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DDE_INITIALIZE_COMMAND {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DDE_INITIALIZE_COMMAND {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for DDE_NAME_SERVICE_CMD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DDE_NAME_SERVICE_CMD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DDE_NAME_SERVICE_CMD").field(&self.0).finish()
    }
}
impl ::core::default::Default for HSZPAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HSZPAIR {
    fn eq(&self, other: &Self) -> bool {
        self.hszSvc == other.hszSvc && self.hszTopic == other.hszTopic
    }
}
impl ::core::cmp::Eq for HSZPAIR {}
impl ::core::fmt::Debug for HSZPAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HSZPAIR").field("hszSvc", &self.hszSvc).field("hszTopic", &self.hszTopic).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for METAFILEPICT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for METAFILEPICT {
    fn eq(&self, other: &Self) -> bool {
        self.mm == other.mm && self.xExt == other.xExt && self.yExt == other.yExt && self.hMF == other.hMF
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for METAFILEPICT {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for METAFILEPICT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("METAFILEPICT").field("mm", &self.mm).field("xExt", &self.xExt).field("yExt", &self.yExt).field("hMF", &self.hMF).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for MONCBSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for MONCBSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.dwTime == other.dwTime && self.hTask == other.hTask && self.dwRet == other.dwRet && self.wType == other.wType && self.wFmt == other.wFmt && self.hConv == other.hConv && self.hsz1 == other.hsz1 && self.hsz2 == other.hsz2 && self.hData == other.hData && self.dwData1 == other.dwData1 && self.dwData2 == other.dwData2 && self.cc == other.cc && self.cbData == other.cbData && self.Data == other.Data
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for MONCBSTRUCT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for MONCBSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONCBSTRUCT").field("cb", &self.cb).field("dwTime", &self.dwTime).field("hTask", &self.hTask).field("dwRet", &self.dwRet).field("wType", &self.wType).field("wFmt", &self.wFmt).field("hConv", &self.hConv).field("hsz1", &self.hsz1).field("hsz2", &self.hsz2).field("hData", &self.hData).field("dwData1", &self.dwData1).field("dwData2", &self.dwData2).field("cc", &self.cc).field("cbData", &self.cbData).field("Data", &self.Data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MONCONVSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MONCONVSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.fConnect == other.fConnect && self.dwTime == other.dwTime && self.hTask == other.hTask && self.hszSvc == other.hszSvc && self.hszTopic == other.hszTopic && self.hConvClient == other.hConvClient && self.hConvServer == other.hConvServer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MONCONVSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MONCONVSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONCONVSTRUCT").field("cb", &self.cb).field("fConnect", &self.fConnect).field("dwTime", &self.dwTime).field("hTask", &self.hTask).field("hszSvc", &self.hszSvc).field("hszTopic", &self.hszTopic).field("hConvClient", &self.hConvClient).field("hConvServer", &self.hConvServer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MONERRSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MONERRSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.wLastError == other.wLastError && self.dwTime == other.dwTime && self.hTask == other.hTask
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MONERRSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MONERRSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONERRSTRUCT").field("cb", &self.cb).field("wLastError", &self.wLastError).field("dwTime", &self.dwTime).field("hTask", &self.hTask).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MONHSZSTRUCTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MONHSZSTRUCTA {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.fsAction == other.fsAction && self.dwTime == other.dwTime && self.hsz == other.hsz && self.hTask == other.hTask && self.str == other.str
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MONHSZSTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MONHSZSTRUCTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONHSZSTRUCTA").field("cb", &self.cb).field("fsAction", &self.fsAction).field("dwTime", &self.dwTime).field("hsz", &self.hsz).field("hTask", &self.hTask).field("str", &self.str).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MONHSZSTRUCTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MONHSZSTRUCTW {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.fsAction == other.fsAction && self.dwTime == other.dwTime && self.hsz == other.hsz && self.hTask == other.hTask && self.str == other.str
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MONHSZSTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MONHSZSTRUCTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONHSZSTRUCTW").field("cb", &self.cb).field("fsAction", &self.fsAction).field("dwTime", &self.dwTime).field("hsz", &self.hsz).field("hTask", &self.hTask).field("str", &self.str).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MONLINKSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MONLINKSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.dwTime == other.dwTime && self.hTask == other.hTask && self.fEstablished == other.fEstablished && self.fNoData == other.fNoData && self.hszSvc == other.hszSvc && self.hszTopic == other.hszTopic && self.hszItem == other.hszItem && self.wFmt == other.wFmt && self.fServer == other.fServer && self.hConvServer == other.hConvServer && self.hConvClient == other.hConvClient
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MONLINKSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MONLINKSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONLINKSTRUCT").field("cb", &self.cb).field("dwTime", &self.dwTime).field("hTask", &self.hTask).field("fEstablished", &self.fEstablished).field("fNoData", &self.fNoData).field("hszSvc", &self.hszSvc).field("hszTopic", &self.hszTopic).field("hszItem", &self.hszItem).field("wFmt", &self.wFmt).field("fServer", &self.fServer).field("hConvServer", &self.hConvServer).field("hConvClient", &self.hConvClient).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MONMSGSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MONMSGSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.hwndTo == other.hwndTo && self.dwTime == other.dwTime && self.hTask == other.hTask && self.wMsg == other.wMsg && self.wParam == other.wParam && self.lParam == other.lParam && self.dmhd == other.dmhd
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MONMSGSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MONMSGSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONMSGSTRUCT").field("cb", &self.cb).field("hwndTo", &self.hwndTo).field("dwTime", &self.dwTime).field("hTask", &self.hTask).field("wMsg", &self.wMsg).field("wParam", &self.wParam).field("lParam", &self.lParam).field("dmhd", &self.dmhd).finish()
    }
}
