impl ::core::default::Default for APPLICATIONTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for APPLICATIONTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPLICATIONTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for AUTHENTICATION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AUTHENTICATION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHENTICATION_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for BOID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BOID {
    fn eq(&self, other: &Self) -> bool {
        self.rgb == other.rgb
    }
}
impl ::core::cmp::Eq for BOID {}
impl ::core::fmt::Debug for BOID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BOID").field("rgb", &self.rgb).finish()
    }
}
impl ::core::default::Default for DTCINITIATEDRECOVERYWORK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DTCINITIATEDRECOVERYWORK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DTCINITIATEDRECOVERYWORK").field(&self.0).finish()
    }
}
impl ::core::default::Default for DTCLUCOMPARESTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DTCLUCOMPARESTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DTCLUCOMPARESTATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DTCLUCOMPARESTATESCONFIRMATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DTCLUCOMPARESTATESCONFIRMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DTCLUCOMPARESTATESCONFIRMATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for DTCLUCOMPARESTATESERROR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DTCLUCOMPARESTATESERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DTCLUCOMPARESTATESERROR").field(&self.0).finish()
    }
}
impl ::core::default::Default for DTCLUCOMPARESTATESRESPONSE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DTCLUCOMPARESTATESRESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DTCLUCOMPARESTATESRESPONSE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DTCLUXLN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DTCLUXLN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DTCLUXLN").field(&self.0).finish()
    }
}
impl ::core::default::Default for DTCLUXLNCONFIRMATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DTCLUXLNCONFIRMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DTCLUXLNCONFIRMATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for DTCLUXLNERROR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DTCLUXLNERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DTCLUXLNERROR").field(&self.0).finish()
    }
}
impl ::core::default::Default for DTCLUXLNRESPONSE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DTCLUXLNRESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DTCLUXLNRESPONSE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DTC_STATUS_ {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DTC_STATUS_ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DTC_STATUS_").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDtcLuConfigure {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuConfigure {}
impl ::core::fmt::Debug for IDtcLuConfigure {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuConfigure").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDtcLuRecovery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRecovery {}
impl ::core::fmt::Debug for IDtcLuRecovery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRecovery").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDtcLuRecoveryFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRecoveryFactory {}
impl ::core::fmt::Debug for IDtcLuRecoveryFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRecoveryFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDtcLuRecoveryInitiatedByDtc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRecoveryInitiatedByDtc {}
impl ::core::fmt::Debug for IDtcLuRecoveryInitiatedByDtc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRecoveryInitiatedByDtc").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDtcLuRecoveryInitiatedByDtcStatusWork {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRecoveryInitiatedByDtcStatusWork {}
impl ::core::fmt::Debug for IDtcLuRecoveryInitiatedByDtcStatusWork {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRecoveryInitiatedByDtcStatusWork").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDtcLuRecoveryInitiatedByDtcTransWork {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRecoveryInitiatedByDtcTransWork {}
impl ::core::fmt::Debug for IDtcLuRecoveryInitiatedByDtcTransWork {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRecoveryInitiatedByDtcTransWork").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDtcLuRecoveryInitiatedByLu {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRecoveryInitiatedByLu {}
impl ::core::fmt::Debug for IDtcLuRecoveryInitiatedByLu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRecoveryInitiatedByLu").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDtcLuRecoveryInitiatedByLuWork {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRecoveryInitiatedByLuWork {}
impl ::core::fmt::Debug for IDtcLuRecoveryInitiatedByLuWork {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRecoveryInitiatedByLuWork").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDtcLuRmEnlistment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRmEnlistment {}
impl ::core::fmt::Debug for IDtcLuRmEnlistment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRmEnlistment").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDtcLuRmEnlistmentFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRmEnlistmentFactory {}
impl ::core::fmt::Debug for IDtcLuRmEnlistmentFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRmEnlistmentFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDtcLuRmEnlistmentSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuRmEnlistmentSink {}
impl ::core::fmt::Debug for IDtcLuRmEnlistmentSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuRmEnlistmentSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDtcLuSubordinateDtc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuSubordinateDtc {}
impl ::core::fmt::Debug for IDtcLuSubordinateDtc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuSubordinateDtc").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDtcLuSubordinateDtcFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuSubordinateDtcFactory {}
impl ::core::fmt::Debug for IDtcLuSubordinateDtcFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuSubordinateDtcFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDtcLuSubordinateDtcSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcLuSubordinateDtcSink {}
impl ::core::fmt::Debug for IDtcLuSubordinateDtcSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcLuSubordinateDtcSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDtcNetworkAccessConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcNetworkAccessConfig {}
impl ::core::fmt::Debug for IDtcNetworkAccessConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcNetworkAccessConfig").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDtcNetworkAccessConfig2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcNetworkAccessConfig2 {}
impl ::core::fmt::Debug for IDtcNetworkAccessConfig2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcNetworkAccessConfig2").field(&self.0).finish()
    }
}
impl IDtcNetworkAccessConfig2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnyNetworkAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAnyNetworkAccess)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAnyNetworkAccess<P0>(&self, banynetworkaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetAnyNetworkAccess)(::windows::core::Vtable::as_raw(self), banynetworkaccess.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkAdministrationAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetNetworkAdministrationAccess)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkAdministrationAccess<P0>(&self, bnetworkadministrationaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetNetworkAdministrationAccess)(::windows::core::Vtable::as_raw(self), bnetworkadministrationaccess.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTransactionAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetNetworkTransactionAccess)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTransactionAccess<P0>(&self, bnetworktransactionaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetNetworkTransactionAccess)(::windows::core::Vtable::as_raw(self), bnetworktransactionaccess.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkClientAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetNetworkClientAccess)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkClientAccess<P0>(&self, bnetworkclientaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetNetworkClientAccess)(::windows::core::Vtable::as_raw(self), bnetworkclientaccess.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTIPAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetNetworkTIPAccess)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTIPAccess<P0>(&self, bnetworktipaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetNetworkTIPAccess)(::windows::core::Vtable::as_raw(self), bnetworktipaccess.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetXAAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetXAAccess)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXAAccess<P0>(&self, bxaaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetXAAccess)(::windows::core::Vtable::as_raw(self), bxaaccess.into()).ok()
    }
    pub unsafe fn RestartDtcService(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RestartDtcService)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IDtcNetworkAccessConfig3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcNetworkAccessConfig3 {}
impl ::core::fmt::Debug for IDtcNetworkAccessConfig3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcNetworkAccessConfig3").field(&self.0).finish()
    }
}
impl IDtcNetworkAccessConfig3 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnyNetworkAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetAnyNetworkAccess)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAnyNetworkAccess<P0>(&self, banynetworkaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetAnyNetworkAccess)(::windows::core::Vtable::as_raw(self), banynetworkaccess.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkAdministrationAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetNetworkAdministrationAccess)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkAdministrationAccess<P0>(&self, bnetworkadministrationaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetNetworkAdministrationAccess)(::windows::core::Vtable::as_raw(self), bnetworkadministrationaccess.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTransactionAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetNetworkTransactionAccess)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTransactionAccess<P0>(&self, bnetworktransactionaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetNetworkTransactionAccess)(::windows::core::Vtable::as_raw(self), bnetworktransactionaccess.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkClientAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetNetworkClientAccess)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkClientAccess<P0>(&self, bnetworkclientaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetNetworkClientAccess)(::windows::core::Vtable::as_raw(self), bnetworkclientaccess.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkTIPAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetNetworkTIPAccess)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkTIPAccess<P0>(&self, bnetworktipaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetNetworkTIPAccess)(::windows::core::Vtable::as_raw(self), bnetworktipaccess.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetXAAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetXAAccess)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetXAAccess<P0>(&self, bxaaccess: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetXAAccess)(::windows::core::Vtable::as_raw(self), bxaaccess.into()).ok()
    }
    pub unsafe fn RestartDtcService(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.RestartDtcService)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkInboundAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetNetworkInboundAccess)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNetworkOutboundAccess(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetNetworkOutboundAccess)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkInboundAccess<P0>(&self, binbound: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetNetworkInboundAccess)(::windows::core::Vtable::as_raw(self), binbound.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNetworkOutboundAccess<P0>(&self, boutbound: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetNetworkOutboundAccess)(::windows::core::Vtable::as_raw(self), boutbound.into()).ok()
    }
    pub unsafe fn GetAuthenticationLevel(&self) -> ::windows::core::Result<AUTHENTICATION_LEVEL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAuthenticationLevel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAuthenticationLevel(&self, authlevel: AUTHENTICATION_LEVEL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetAuthenticationLevel)(::windows::core::Vtable::as_raw(self), authlevel).ok()
    }
}
impl ::core::cmp::PartialEq for IDtcToXaHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcToXaHelper {}
impl ::core::fmt::Debug for IDtcToXaHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcToXaHelper").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDtcToXaHelperFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcToXaHelperFactory {}
impl ::core::fmt::Debug for IDtcToXaHelperFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcToXaHelperFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDtcToXaHelperSinglePipe {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcToXaHelperSinglePipe {}
impl ::core::fmt::Debug for IDtcToXaHelperSinglePipe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcToXaHelperSinglePipe").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDtcToXaMapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDtcToXaMapper {}
impl ::core::fmt::Debug for IDtcToXaMapper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDtcToXaMapper").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGetDispenser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetDispenser {}
impl ::core::fmt::Debug for IGetDispenser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetDispenser").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IKernelTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKernelTransaction {}
impl ::core::fmt::Debug for IKernelTransaction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKernelTransaction").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ILastResourceManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILastResourceManager {}
impl ::core::fmt::Debug for ILastResourceManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILastResourceManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrepareInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrepareInfo {}
impl ::core::fmt::Debug for IPrepareInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrepareInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPrepareInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrepareInfo2 {}
impl ::core::fmt::Debug for IPrepareInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrepareInfo2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRMHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRMHelper {}
impl ::core::fmt::Debug for IRMHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRMHelper").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IResourceManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResourceManager {}
impl ::core::fmt::Debug for IResourceManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResourceManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IResourceManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResourceManager2 {}
impl ::core::fmt::Debug for IResourceManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResourceManager2").field(&self.0).finish()
    }
}
impl IResourceManager2 {
    pub unsafe fn Enlist<P0, P1>(&self, ptransaction: P0, pres: P1, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut ::core::option::Option<ITransactionEnlistmentAsync>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITransaction>>,
        P1: ::std::convert::Into<::windows::core::InParam<ITransactionResourceAsync>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Enlist)(::windows::core::Vtable::as_raw(self), ptransaction.into().abi(), pres.into().abi(), puow, pisolevel, ::core::mem::transmute(ppenlist)).ok()
    }
    pub unsafe fn Reenlist(&self, pprepinfo: &[u8], ltimeout: u32) -> ::windows::core::Result<XACTSTAT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Reenlist)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pprepinfo.as_ptr()), pprepinfo.len() as _, ltimeout, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ReenlistmentComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ReenlistmentComplete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetDistributedTransactionManager(&self, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDistributedTransactionManager)(::windows::core::Vtable::as_raw(self), iid, ppvobject).ok()
    }
}
impl ::core::cmp::PartialEq for IResourceManagerFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResourceManagerFactory {}
impl ::core::fmt::Debug for IResourceManagerFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResourceManagerFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IResourceManagerFactory2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResourceManagerFactory2 {}
impl ::core::fmt::Debug for IResourceManagerFactory2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResourceManagerFactory2").field(&self.0).finish()
    }
}
impl IResourceManagerFactory2 {
    pub unsafe fn Create<P0, P1>(&self, pguidrm: *const ::windows::core::GUID, pszrmname: P0, piresmgrsink: P1) -> ::windows::core::Result<IResourceManager>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IResourceManagerSink>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Create)(::windows::core::Vtable::as_raw(self), pguidrm, pszrmname.into().abi(), piresmgrsink.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IResourceManagerRejoinable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResourceManagerRejoinable {}
impl ::core::fmt::Debug for IResourceManagerRejoinable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResourceManagerRejoinable").field(&self.0).finish()
    }
}
impl IResourceManagerRejoinable {
    pub unsafe fn Enlist<P0, P1>(&self, ptransaction: P0, pres: P1, puow: *mut BOID, pisolevel: *mut i32, ppenlist: *mut ::core::option::Option<ITransactionEnlistmentAsync>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITransaction>>,
        P1: ::std::convert::Into<::windows::core::InParam<ITransactionResourceAsync>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Enlist)(::windows::core::Vtable::as_raw(self), ptransaction.into().abi(), pres.into().abi(), puow, pisolevel, ::core::mem::transmute(ppenlist)).ok()
    }
    pub unsafe fn Reenlist(&self, pprepinfo: &[u8], ltimeout: u32) -> ::windows::core::Result<XACTSTAT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Reenlist)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pprepinfo.as_ptr()), pprepinfo.len() as _, ltimeout, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ReenlistmentComplete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ReenlistmentComplete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetDistributedTransactionManager(&self, iid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDistributedTransactionManager)(::windows::core::Vtable::as_raw(self), iid, ppvobject).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enlist2<P0, P1>(&self, ptransaction: P0, presasync: P1, puow: *mut BOID, pisolevel: *mut i32, pxid: *mut XID, ppenlist: *mut ::core::option::Option<ITransactionEnlistmentAsync>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ITransaction>>,
        P1: ::std::convert::Into<::windows::core::InParam<ITransactionResourceAsync>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Enlist2)(::windows::core::Vtable::as_raw(self), ptransaction.into().abi(), presasync.into().abi(), puow, pisolevel, pxid, ::core::mem::transmute(ppenlist)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Reenlist2(&self, pxid: *const XID, dwtimeout: u32) -> ::windows::core::Result<XACTSTAT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Reenlist2)(::windows::core::Vtable::as_raw(self), pxid, dwtimeout, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IResourceManagerSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResourceManagerSink {}
impl ::core::fmt::Debug for IResourceManagerSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResourceManagerSink").field(&self.0).finish()
    }
}
impl ::core::default::Default for ISOFLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ISOFLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISOFLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for ISOLATIONLEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ISOLATIONLEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISOLATIONLEVEL").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITipHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITipHelper {}
impl ::core::fmt::Debug for ITipHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITipHelper").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITipPullSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITipPullSink {}
impl ::core::fmt::Debug for ITipPullSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITipPullSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITipTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITipTransaction {}
impl ::core::fmt::Debug for ITipTransaction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITipTransaction").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITmNodeName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITmNodeName {}
impl ::core::fmt::Debug for ITmNodeName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITmNodeName").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransaction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransaction {}
impl ::core::fmt::Debug for ITransaction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransaction").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransaction2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransaction2 {}
impl ::core::fmt::Debug for ITransaction2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransaction2").field(&self.0).finish()
    }
}
impl ITransaction2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Commit<P0>(&self, fretaining: P0, grftc: u32, grfrm: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Commit)(::windows::core::Vtable::as_raw(self), fretaining.into(), grftc, grfrm).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Abort<P0, P1>(&self, pboidreason: *const BOID, fretaining: P0, fasync: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Abort)(::windows::core::Vtable::as_raw(self), pboidreason, fretaining.into(), fasync.into()).ok()
    }
    pub unsafe fn GetTransactionInfo(&self, pinfo: *mut XACTTRANSINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetTransactionInfo)(::windows::core::Vtable::as_raw(self), pinfo).ok()
    }
    pub unsafe fn CloneWithCommitDisabled(&self) -> ::windows::core::Result<ITransaction> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CloneWithCommitDisabled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ITransactionCloner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionCloner {}
impl ::core::fmt::Debug for ITransactionCloner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionCloner").field(&self.0).finish()
    }
}
impl ITransactionCloner {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Commit<P0>(&self, fretaining: P0, grftc: u32, grfrm: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Commit)(::windows::core::Vtable::as_raw(self), fretaining.into(), grftc, grfrm).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Abort<P0, P1>(&self, pboidreason: *const BOID, fretaining: P0, fasync: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Abort)(::windows::core::Vtable::as_raw(self), pboidreason, fretaining.into(), fasync.into()).ok()
    }
    pub unsafe fn GetTransactionInfo(&self, pinfo: *mut XACTTRANSINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetTransactionInfo)(::windows::core::Vtable::as_raw(self), pinfo).ok()
    }
}
impl ::core::cmp::PartialEq for ITransactionDispenser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionDispenser {}
impl ::core::fmt::Debug for ITransactionDispenser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionDispenser").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransactionEnlistmentAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionEnlistmentAsync {}
impl ::core::fmt::Debug for ITransactionEnlistmentAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionEnlistmentAsync").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransactionExport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionExport {}
impl ::core::fmt::Debug for ITransactionExport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionExport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransactionExportFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionExportFactory {}
impl ::core::fmt::Debug for ITransactionExportFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionExportFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransactionImport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionImport {}
impl ::core::fmt::Debug for ITransactionImport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionImport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransactionImportWhereabouts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionImportWhereabouts {}
impl ::core::fmt::Debug for ITransactionImportWhereabouts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionImportWhereabouts").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransactionLastEnlistmentAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionLastEnlistmentAsync {}
impl ::core::fmt::Debug for ITransactionLastEnlistmentAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionLastEnlistmentAsync").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransactionLastResourceAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionLastResourceAsync {}
impl ::core::fmt::Debug for ITransactionLastResourceAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionLastResourceAsync").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransactionOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionOptions {}
impl ::core::fmt::Debug for ITransactionOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionOptions").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransactionOutcomeEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionOutcomeEvents {}
impl ::core::fmt::Debug for ITransactionOutcomeEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionOutcomeEvents").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransactionPhase0EnlistmentAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionPhase0EnlistmentAsync {}
impl ::core::fmt::Debug for ITransactionPhase0EnlistmentAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionPhase0EnlistmentAsync").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransactionPhase0Factory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionPhase0Factory {}
impl ::core::fmt::Debug for ITransactionPhase0Factory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionPhase0Factory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransactionPhase0NotifyAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionPhase0NotifyAsync {}
impl ::core::fmt::Debug for ITransactionPhase0NotifyAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionPhase0NotifyAsync").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransactionReceiver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionReceiver {}
impl ::core::fmt::Debug for ITransactionReceiver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionReceiver").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransactionReceiverFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionReceiverFactory {}
impl ::core::fmt::Debug for ITransactionReceiverFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionReceiverFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransactionResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionResource {}
impl ::core::fmt::Debug for ITransactionResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionResource").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransactionResourceAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionResourceAsync {}
impl ::core::fmt::Debug for ITransactionResourceAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionResourceAsync").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransactionTransmitter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionTransmitter {}
impl ::core::fmt::Debug for ITransactionTransmitter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionTransmitter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransactionTransmitterFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionTransmitterFactory {}
impl ::core::fmt::Debug for ITransactionTransmitterFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionTransmitterFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransactionVoterBallotAsync2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionVoterBallotAsync2 {}
impl ::core::fmt::Debug for ITransactionVoterBallotAsync2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionVoterBallotAsync2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransactionVoterFactory2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionVoterFactory2 {}
impl ::core::fmt::Debug for ITransactionVoterFactory2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionVoterFactory2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransactionVoterNotifyAsync2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionVoterNotifyAsync2 {}
impl ::core::fmt::Debug for ITransactionVoterNotifyAsync2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionVoterNotifyAsync2").field(&self.0).finish()
    }
}
impl ITransactionVoterNotifyAsync2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Committed<P0>(&self, fretaining: P0, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Committed)(::windows::core::Vtable::as_raw(self), fretaining.into(), pnewuow, hr).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Aborted<P0>(&self, pboidreason: *const BOID, fretaining: P0, pnewuow: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Aborted)(::windows::core::Vtable::as_raw(self), pboidreason, fretaining.into(), pnewuow, hr).ok()
    }
    pub unsafe fn HeuristicDecision(&self, dwdecision: u32, pboidreason: *const BOID, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.HeuristicDecision)(::windows::core::Vtable::as_raw(self), dwdecision, pboidreason, hr).ok()
    }
    pub unsafe fn Indoubt(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Indoubt)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IXAConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXAConfig {}
impl ::core::fmt::Debug for IXAConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXAConfig").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXAObtainRMInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXAObtainRMInfo {}
impl ::core::fmt::Debug for IXAObtainRMInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXAObtainRMInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXATransLookup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXATransLookup {}
impl ::core::fmt::Debug for IXATransLookup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXATransLookup").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXATransLookup2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXATransLookup2 {}
impl ::core::fmt::Debug for IXATransLookup2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXATransLookup2").field(&self.0).finish()
    }
}
impl ::core::default::Default for OLE_TM_CONFIG_PARAMS_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OLE_TM_CONFIG_PARAMS_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwcConcurrencyHint == other.dwcConcurrencyHint
    }
}
impl ::core::cmp::Eq for OLE_TM_CONFIG_PARAMS_V1 {}
impl ::core::fmt::Debug for OLE_TM_CONFIG_PARAMS_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLE_TM_CONFIG_PARAMS_V1").field("dwVersion", &self.dwVersion).field("dwcConcurrencyHint", &self.dwcConcurrencyHint).finish()
    }
}
impl ::core::default::Default for OLE_TM_CONFIG_PARAMS_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OLE_TM_CONFIG_PARAMS_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwcConcurrencyHint == other.dwcConcurrencyHint && self.applicationType == other.applicationType && self.clusterResourceId == other.clusterResourceId
    }
}
impl ::core::cmp::Eq for OLE_TM_CONFIG_PARAMS_V2 {}
impl ::core::fmt::Debug for OLE_TM_CONFIG_PARAMS_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OLE_TM_CONFIG_PARAMS_V2").field("dwVersion", &self.dwVersion).field("dwcConcurrencyHint", &self.dwcConcurrencyHint).field("applicationType", &self.applicationType).field("clusterResourceId", &self.clusterResourceId).finish()
    }
}
impl ::core::default::Default for PROXY_CONFIG_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROXY_CONFIG_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.wcThreadsMax == other.wcThreadsMax
    }
}
impl ::core::cmp::Eq for PROXY_CONFIG_PARAMS {}
impl ::core::fmt::Debug for PROXY_CONFIG_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROXY_CONFIG_PARAMS").field("wcThreadsMax", &self.wcThreadsMax).finish()
    }
}
impl ::core::default::Default for TX_MISC_CONSTANTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TX_MISC_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TX_MISC_CONSTANTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for XACTCONST {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XACTCONST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XACTCONST").field(&self.0).finish()
    }
}
impl ::core::default::Default for XACTHEURISTIC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XACTHEURISTIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XACTHEURISTIC").field(&self.0).finish()
    }
}
impl ::core::default::Default for XACTOPT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for XACTOPT {
    fn eq(&self, other: &Self) -> bool {
        self.ulTimeout == other.ulTimeout && self.szDescription == other.szDescription
    }
}
impl ::core::cmp::Eq for XACTOPT {}
impl ::core::fmt::Debug for XACTOPT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XACTOPT").field("ulTimeout", &self.ulTimeout).field("szDescription", &self.szDescription).finish()
    }
}
impl ::core::default::Default for XACTRM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XACTRM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XACTRM").field(&self.0).finish()
    }
}
impl ::core::default::Default for XACTSTAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XACTSTAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XACTSTAT").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for XACTSTATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for XACTSTATS {
    fn eq(&self, other: &Self) -> bool {
        self.cOpen == other.cOpen && self.cCommitting == other.cCommitting && self.cCommitted == other.cCommitted && self.cAborting == other.cAborting && self.cAborted == other.cAborted && self.cInDoubt == other.cInDoubt && self.cHeuristicDecision == other.cHeuristicDecision && self.timeTransactionsUp == other.timeTransactionsUp
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for XACTSTATS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for XACTSTATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XACTSTATS").field("cOpen", &self.cOpen).field("cCommitting", &self.cCommitting).field("cCommitted", &self.cCommitted).field("cAborting", &self.cAborting).field("cAborted", &self.cAborted).field("cInDoubt", &self.cInDoubt).field("cHeuristicDecision", &self.cHeuristicDecision).field("timeTransactionsUp", &self.timeTransactionsUp).finish()
    }
}
impl ::core::default::Default for XACTTC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XACTTC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XACTTC").field(&self.0).finish()
    }
}
impl ::core::default::Default for XACTTRANSINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for XACTTRANSINFO {
    fn eq(&self, other: &Self) -> bool {
        self.uow == other.uow && self.isoLevel == other.isoLevel && self.isoFlags == other.isoFlags && self.grfTCSupported == other.grfTCSupported && self.grfRMSupported == other.grfRMSupported && self.grfTCSupportedRetaining == other.grfTCSupportedRetaining && self.grfRMSupportedRetaining == other.grfRMSupportedRetaining
    }
}
impl ::core::cmp::Eq for XACTTRANSINFO {}
impl ::core::fmt::Debug for XACTTRANSINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XACTTRANSINFO").field("uow", &self.uow).field("isoLevel", &self.isoLevel).field("isoFlags", &self.isoFlags).field("grfTCSupported", &self.grfTCSupported).field("grfRMSupported", &self.grfRMSupported).field("grfTCSupportedRetaining", &self.grfTCSupportedRetaining).field("grfRMSupportedRetaining", &self.grfRMSupportedRetaining).finish()
    }
}
impl ::core::default::Default for XACT_DTC_CONSTANTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XACT_DTC_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XACT_DTC_CONSTANTS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for XID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for XID {
    fn eq(&self, other: &Self) -> bool {
        self.formatID == other.formatID && self.gtrid_length == other.gtrid_length && self.bqual_length == other.bqual_length && self.data == other.data
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for XID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for XID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XID").field("formatID", &self.formatID).field("gtrid_length", &self.gtrid_length).field("bqual_length", &self.bqual_length).field("data", &self.data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for xa_switch_t {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for xa_switch_t {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.flags == other.flags && self.version == other.version && self.xa_open_entry == other.xa_open_entry && self.xa_close_entry == other.xa_close_entry && self.xa_start_entry == other.xa_start_entry && self.xa_end_entry == other.xa_end_entry && self.xa_rollback_entry == other.xa_rollback_entry && self.xa_prepare_entry == other.xa_prepare_entry && self.xa_commit_entry == other.xa_commit_entry && self.xa_recover_entry == other.xa_recover_entry && self.xa_forget_entry == other.xa_forget_entry && self.xa_complete_entry == other.xa_complete_entry
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for xa_switch_t {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for xa_switch_t {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("xa_switch_t")
            .field("name", &self.name)
            .field("flags", &self.flags)
            .field("version", &self.version)
            .field("xa_open_entry", &self.xa_open_entry)
            .field("xa_close_entry", &self.xa_close_entry)
            .field("xa_start_entry", &self.xa_start_entry)
            .field("xa_end_entry", &self.xa_end_entry)
            .field("xa_rollback_entry", &self.xa_rollback_entry)
            .field("xa_prepare_entry", &self.xa_prepare_entry)
            .field("xa_commit_entry", &self.xa_commit_entry)
            .field("xa_recover_entry", &self.xa_recover_entry)
            .field("xa_forget_entry", &self.xa_forget_entry)
            .field("xa_complete_entry", &self.xa_complete_entry)
            .finish()
    }
}
