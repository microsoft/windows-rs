#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWSMan {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWSMan {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWSMan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSMan").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWSManConnectionOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWSManConnectionOptions {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWSManConnectionOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSManConnectionOptions").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWSManConnectionOptionsEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWSManConnectionOptionsEx {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWSManConnectionOptionsEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSManConnectionOptionsEx").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWSManConnectionOptionsEx {
    pub unsafe fn UserName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.UserName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetUserName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUserName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name)).ok()
    }
    pub unsafe fn SetPassword(&self, password: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPassword)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(password)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWSManConnectionOptionsEx2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWSManConnectionOptionsEx2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWSManConnectionOptionsEx2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSManConnectionOptionsEx2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWSManConnectionOptionsEx2 {
    pub unsafe fn UserName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.UserName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetUserName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetUserName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name)).ok()
    }
    pub unsafe fn SetPassword(&self, password: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPassword)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(password)).ok()
    }
    pub unsafe fn CertificateThumbprint(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CertificateThumbprint)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCertificateThumbprint(&self, thumbprint: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetCertificateThumbprint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(thumbprint)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWSManEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWSManEnumerator {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWSManEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSManEnumerator").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWSManEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWSManEx {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWSManEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSManEx").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWSManEx {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateSession<P0>(&self, connection: &::windows::core::BSTR, flags: i32, connectionoptions: P0) -> ::windows::core::Result<super::Com::IDispatch>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateSession)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(connection), flags, connectionoptions.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateConnectionOptions(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateConnectionOptions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CommandLine(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CommandLine)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Error(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Error)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWSManEx2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWSManEx2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWSManEx2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSManEx2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWSManEx2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateSession<P0>(&self, connection: &::windows::core::BSTR, flags: i32, connectionoptions: P0) -> ::windows::core::Result<super::Com::IDispatch>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateSession)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(connection), flags, connectionoptions.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateConnectionOptions(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateConnectionOptions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CommandLine(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CommandLine)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Error(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Error)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateResourceLocator(&self, strresourcelocator: &::windows::core::BSTR) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateResourceLocator)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strresourcelocator), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SessionFlagUTF8(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SessionFlagUTF8)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SessionFlagCredUsernamePassword(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SessionFlagCredUsernamePassword)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SessionFlagSkipCACheck(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SessionFlagSkipCACheck)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SessionFlagSkipCNCheck(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SessionFlagSkipCNCheck)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseDigest(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SessionFlagUseDigest)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseNegotiate(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SessionFlagUseNegotiate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseBasic(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SessionFlagUseBasic)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseKerberos(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SessionFlagUseKerberos)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SessionFlagNoEncryption(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SessionFlagNoEncryption)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SessionFlagEnableSPNServerPort(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SessionFlagEnableSPNServerPort)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseNoAuthentication(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SessionFlagUseNoAuthentication)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagNonXmlText(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerationFlagNonXmlText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagReturnEPR(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerationFlagReturnEPR)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagReturnObjectAndEPR(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerationFlagReturnObjectAndEPR)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetErrorMessage(&self, errornumber: u32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetErrorMessage)(::windows::core::Vtable::as_raw(self), errornumber, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagHierarchyDeep(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerationFlagHierarchyDeep)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagHierarchyShallow(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerationFlagHierarchyShallow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagHierarchyDeepBasePropsOnly(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerationFlagHierarchyDeepBasePropsOnly)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagReturnObject(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerationFlagReturnObject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWSManEx3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWSManEx3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWSManEx3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSManEx3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWSManEx3 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateSession<P0>(&self, connection: &::windows::core::BSTR, flags: i32, connectionoptions: P0) -> ::windows::core::Result<super::Com::IDispatch>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateSession)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(connection), flags, connectionoptions.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateConnectionOptions(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateConnectionOptions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CommandLine(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CommandLine)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Error(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Error)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateResourceLocator(&self, strresourcelocator: &::windows::core::BSTR) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateResourceLocator)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strresourcelocator), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SessionFlagUTF8(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.SessionFlagUTF8)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SessionFlagCredUsernamePassword(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.SessionFlagCredUsernamePassword)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SessionFlagSkipCACheck(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.SessionFlagSkipCACheck)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SessionFlagSkipCNCheck(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.SessionFlagSkipCNCheck)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseDigest(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.SessionFlagUseDigest)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseNegotiate(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.SessionFlagUseNegotiate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseBasic(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.SessionFlagUseBasic)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseKerberos(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.SessionFlagUseKerberos)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SessionFlagNoEncryption(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.SessionFlagNoEncryption)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SessionFlagEnableSPNServerPort(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.SessionFlagEnableSPNServerPort)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseNoAuthentication(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.SessionFlagUseNoAuthentication)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagNonXmlText(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnumerationFlagNonXmlText)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagReturnEPR(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnumerationFlagReturnEPR)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagReturnObjectAndEPR(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnumerationFlagReturnObjectAndEPR)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetErrorMessage(&self, errornumber: u32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetErrorMessage)(::windows::core::Vtable::as_raw(self), errornumber, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagHierarchyDeep(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnumerationFlagHierarchyDeep)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagHierarchyShallow(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnumerationFlagHierarchyShallow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagHierarchyDeepBasePropsOnly(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnumerationFlagHierarchyDeepBasePropsOnly)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumerationFlagReturnObject(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnumerationFlagReturnObject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SessionFlagUseClientCertificate(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SessionFlagUseClientCertificate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWSManInternal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWSManInternal {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWSManInternal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSManInternal").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWSManResourceLocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWSManResourceLocator {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWSManResourceLocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSManResourceLocator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWSManResourceLocatorInternal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWSManResourceLocatorInternal {}
impl ::core::fmt::Debug for IWSManResourceLocatorInternal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSManResourceLocatorInternal").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWSManSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWSManSession {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWSManSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWSManSession").field(&self.0).finish()
    }
}
impl ::core::default::Default for WSMAN_AUTHENTICATION_CREDENTIALS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WSMAN_AUTHZ_QUOTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSMAN_AUTHZ_QUOTA {
    fn eq(&self, other: &Self) -> bool {
        self.maxAllowedConcurrentShells == other.maxAllowedConcurrentShells && self.maxAllowedConcurrentOperations == other.maxAllowedConcurrentOperations && self.timeslotSize == other.timeslotSize && self.maxAllowedOperationsPerTimeslot == other.maxAllowedOperationsPerTimeslot
    }
}
impl ::core::cmp::Eq for WSMAN_AUTHZ_QUOTA {}
impl ::core::fmt::Debug for WSMAN_AUTHZ_QUOTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_AUTHZ_QUOTA").field("maxAllowedConcurrentShells", &self.maxAllowedConcurrentShells).field("maxAllowedConcurrentOperations", &self.maxAllowedConcurrentOperations).field("timeslotSize", &self.timeslotSize).field("maxAllowedOperationsPerTimeslot", &self.maxAllowedOperationsPerTimeslot).finish()
    }
}
impl ::core::default::Default for WSMAN_CERTIFICATE_DETAILS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSMAN_CERTIFICATE_DETAILS {
    fn eq(&self, other: &Self) -> bool {
        self.subject == other.subject && self.issuerName == other.issuerName && self.issuerThumbprint == other.issuerThumbprint && self.subjectName == other.subjectName
    }
}
impl ::core::cmp::Eq for WSMAN_CERTIFICATE_DETAILS {}
impl ::core::fmt::Debug for WSMAN_CERTIFICATE_DETAILS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_CERTIFICATE_DETAILS").field("subject", &self.subject).field("issuerName", &self.issuerName).field("issuerThumbprint", &self.issuerThumbprint).field("subjectName", &self.subjectName).finish()
    }
}
impl ::core::default::Default for WSMAN_COMMAND_ARG_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSMAN_COMMAND_ARG_SET {
    fn eq(&self, other: &Self) -> bool {
        self.argsCount == other.argsCount && self.args == other.args
    }
}
impl ::core::cmp::Eq for WSMAN_COMMAND_ARG_SET {}
impl ::core::fmt::Debug for WSMAN_COMMAND_ARG_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_COMMAND_ARG_SET").field("argsCount", &self.argsCount).field("args", &self.args).finish()
    }
}
impl ::core::default::Default for WSMAN_CONNECT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WSMAN_CREATE_SHELL_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WSMAN_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WSMAN_DATA_BINARY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSMAN_DATA_BINARY {
    fn eq(&self, other: &Self) -> bool {
        self.dataLength == other.dataLength && self.data == other.data
    }
}
impl ::core::cmp::Eq for WSMAN_DATA_BINARY {}
impl ::core::fmt::Debug for WSMAN_DATA_BINARY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_DATA_BINARY").field("dataLength", &self.dataLength).field("data", &self.data).finish()
    }
}
impl ::core::default::Default for WSMAN_DATA_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSMAN_DATA_TEXT {
    fn eq(&self, other: &Self) -> bool {
        self.bufferLength == other.bufferLength && self.buffer == other.buffer
    }
}
impl ::core::cmp::Eq for WSMAN_DATA_TEXT {}
impl ::core::fmt::Debug for WSMAN_DATA_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_DATA_TEXT").field("bufferLength", &self.bufferLength).field("buffer", &self.buffer).finish()
    }
}
impl ::core::default::Default for WSMAN_ENVIRONMENT_VARIABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSMAN_ENVIRONMENT_VARIABLE {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value
    }
}
impl ::core::cmp::Eq for WSMAN_ENVIRONMENT_VARIABLE {}
impl ::core::fmt::Debug for WSMAN_ENVIRONMENT_VARIABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_ENVIRONMENT_VARIABLE").field("name", &self.name).field("value", &self.value).finish()
    }
}
impl ::core::default::Default for WSMAN_ENVIRONMENT_VARIABLE_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSMAN_ENVIRONMENT_VARIABLE_SET {
    fn eq(&self, other: &Self) -> bool {
        self.varsCount == other.varsCount && self.vars == other.vars
    }
}
impl ::core::cmp::Eq for WSMAN_ENVIRONMENT_VARIABLE_SET {}
impl ::core::fmt::Debug for WSMAN_ENVIRONMENT_VARIABLE_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_ENVIRONMENT_VARIABLE_SET").field("varsCount", &self.varsCount).field("vars", &self.vars).finish()
    }
}
impl ::core::default::Default for WSMAN_ERROR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSMAN_ERROR {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code && self.errorDetail == other.errorDetail && self.language == other.language && self.machineName == other.machineName && self.pluginName == other.pluginName
    }
}
impl ::core::cmp::Eq for WSMAN_ERROR {}
impl ::core::fmt::Debug for WSMAN_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_ERROR").field("code", &self.code).field("errorDetail", &self.errorDetail).field("language", &self.language).field("machineName", &self.machineName).field("pluginName", &self.pluginName).finish()
    }
}
impl ::core::default::Default for WSMAN_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSMAN_FILTER {
    fn eq(&self, other: &Self) -> bool {
        self.filter == other.filter && self.dialect == other.dialect
    }
}
impl ::core::cmp::Eq for WSMAN_FILTER {}
impl ::core::fmt::Debug for WSMAN_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_FILTER").field("filter", &self.filter).field("dialect", &self.dialect).finish()
    }
}
impl ::core::default::Default for WSMAN_FRAGMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSMAN_FRAGMENT {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path && self.dialect == other.dialect
    }
}
impl ::core::cmp::Eq for WSMAN_FRAGMENT {}
impl ::core::fmt::Debug for WSMAN_FRAGMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_FRAGMENT").field("path", &self.path).field("dialect", &self.dialect).finish()
    }
}
impl ::core::default::Default for WSMAN_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSMAN_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.value == other.value
    }
}
impl ::core::cmp::Eq for WSMAN_KEY {}
impl ::core::fmt::Debug for WSMAN_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_KEY").field("key", &self.key).field("value", &self.value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSMAN_OPERATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSMAN_OPERATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.fragment == other.fragment && self.filter == other.filter && self.selectorSet == other.selectorSet && self.optionSet == other.optionSet && self.reserved == other.reserved && self.version == other.version
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSMAN_OPERATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSMAN_OPERATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_OPERATION_INFO").field("fragment", &self.fragment).field("filter", &self.filter).field("selectorSet", &self.selectorSet).field("optionSet", &self.optionSet).field("reserved", &self.reserved).field("version", &self.version).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSMAN_OPERATION_INFOEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSMAN_OPERATION_INFOEX {
    fn eq(&self, other: &Self) -> bool {
        self.fragment == other.fragment && self.filter == other.filter && self.selectorSet == other.selectorSet && self.optionSet == other.optionSet && self.version == other.version && self.uiLocale == other.uiLocale && self.dataLocale == other.dataLocale
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSMAN_OPERATION_INFOEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSMAN_OPERATION_INFOEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_OPERATION_INFOEX").field("fragment", &self.fragment).field("filter", &self.filter).field("selectorSet", &self.selectorSet).field("optionSet", &self.optionSet).field("version", &self.version).field("uiLocale", &self.uiLocale).field("dataLocale", &self.dataLocale).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSMAN_OPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSMAN_OPTION {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value && self.mustComply == other.mustComply
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSMAN_OPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSMAN_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_OPTION").field("name", &self.name).field("value", &self.value).field("mustComply", &self.mustComply).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSMAN_OPTION_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSMAN_OPTION_SET {
    fn eq(&self, other: &Self) -> bool {
        self.optionsCount == other.optionsCount && self.options == other.options && self.optionsMustUnderstand == other.optionsMustUnderstand
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSMAN_OPTION_SET {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSMAN_OPTION_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_OPTION_SET").field("optionsCount", &self.optionsCount).field("options", &self.options).field("optionsMustUnderstand", &self.optionsMustUnderstand).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSMAN_OPTION_SETEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSMAN_OPTION_SETEX {
    fn eq(&self, other: &Self) -> bool {
        self.optionsCount == other.optionsCount && self.options == other.options && self.optionsMustUnderstand == other.optionsMustUnderstand && self.optionTypes == other.optionTypes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSMAN_OPTION_SETEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSMAN_OPTION_SETEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_OPTION_SETEX").field("optionsCount", &self.optionsCount).field("options", &self.options).field("optionsMustUnderstand", &self.optionsMustUnderstand).field("optionTypes", &self.optionTypes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSMAN_PLUGIN_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSMAN_PLUGIN_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.senderDetails == other.senderDetails && self.locale == other.locale && self.resourceUri == other.resourceUri && self.operationInfo == other.operationInfo && self.shutdownNotification == other.shutdownNotification && self.shutdownNotificationHandle == other.shutdownNotificationHandle && self.dataLocale == other.dataLocale
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSMAN_PLUGIN_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSMAN_PLUGIN_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_PLUGIN_REQUEST").field("senderDetails", &self.senderDetails).field("locale", &self.locale).field("resourceUri", &self.resourceUri).field("operationInfo", &self.operationInfo).field("shutdownNotification", &self.shutdownNotification).field("shutdownNotificationHandle", &self.shutdownNotificationHandle).field("dataLocale", &self.dataLocale).finish()
    }
}
impl ::core::default::Default for WSMAN_PROXY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WSMAN_RECEIVE_DATA_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WSMAN_RESPONSE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WSMAN_SELECTOR_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSMAN_SELECTOR_SET {
    fn eq(&self, other: &Self) -> bool {
        self.numberKeys == other.numberKeys && self.keys == other.keys
    }
}
impl ::core::cmp::Eq for WSMAN_SELECTOR_SET {}
impl ::core::fmt::Debug for WSMAN_SELECTOR_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_SELECTOR_SET").field("numberKeys", &self.numberKeys).field("keys", &self.keys).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WSMAN_SENDER_DETAILS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WSMAN_SENDER_DETAILS {
    fn eq(&self, other: &Self) -> bool {
        self.senderName == other.senderName && self.authenticationMechanism == other.authenticationMechanism && self.certificateDetails == other.certificateDetails && self.clientToken == other.clientToken && self.httpURL == other.httpURL
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WSMAN_SENDER_DETAILS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WSMAN_SENDER_DETAILS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_SENDER_DETAILS").field("senderName", &self.senderName).field("authenticationMechanism", &self.authenticationMechanism).field("certificateDetails", &self.certificateDetails).field("clientToken", &self.clientToken).field("httpURL", &self.httpURL).finish()
    }
}
impl ::core::default::Default for WSMAN_SHELL_ASYNC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WSMAN_SHELL_DISCONNECT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSMAN_SHELL_DISCONNECT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.idleTimeoutMs == other.idleTimeoutMs
    }
}
impl ::core::cmp::Eq for WSMAN_SHELL_DISCONNECT_INFO {}
impl ::core::fmt::Debug for WSMAN_SHELL_DISCONNECT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_SHELL_DISCONNECT_INFO").field("idleTimeoutMs", &self.idleTimeoutMs).finish()
    }
}
impl ::core::default::Default for WSMAN_SHELL_STARTUP_INFO_V10 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSMAN_SHELL_STARTUP_INFO_V10 {
    fn eq(&self, other: &Self) -> bool {
        self.inputStreamSet == other.inputStreamSet && self.outputStreamSet == other.outputStreamSet && self.idleTimeoutMs == other.idleTimeoutMs && self.workingDirectory == other.workingDirectory && self.variableSet == other.variableSet
    }
}
impl ::core::cmp::Eq for WSMAN_SHELL_STARTUP_INFO_V10 {}
impl ::core::fmt::Debug for WSMAN_SHELL_STARTUP_INFO_V10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_SHELL_STARTUP_INFO_V10").field("inputStreamSet", &self.inputStreamSet).field("outputStreamSet", &self.outputStreamSet).field("idleTimeoutMs", &self.idleTimeoutMs).field("workingDirectory", &self.workingDirectory).field("variableSet", &self.variableSet).finish()
    }
}
impl ::core::default::Default for WSMAN_SHELL_STARTUP_INFO_V11 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSMAN_SHELL_STARTUP_INFO_V11 {
    fn eq(&self, other: &Self) -> bool {
        self.Base == other.Base && self.name == other.name
    }
}
impl ::core::cmp::Eq for WSMAN_SHELL_STARTUP_INFO_V11 {}
impl ::core::fmt::Debug for WSMAN_SHELL_STARTUP_INFO_V11 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_SHELL_STARTUP_INFO_V11").field("Base", &self.Base).field("name", &self.name).finish()
    }
}
impl ::core::default::Default for WSMAN_STREAM_ID_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSMAN_STREAM_ID_SET {
    fn eq(&self, other: &Self) -> bool {
        self.streamIDsCount == other.streamIDsCount && self.streamIDs == other.streamIDs
    }
}
impl ::core::cmp::Eq for WSMAN_STREAM_ID_SET {}
impl ::core::fmt::Debug for WSMAN_STREAM_ID_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_STREAM_ID_SET").field("streamIDsCount", &self.streamIDsCount).field("streamIDs", &self.streamIDs).finish()
    }
}
impl ::core::default::Default for WSMAN_USERNAME_PASSWORD_CREDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WSMAN_USERNAME_PASSWORD_CREDS {
    fn eq(&self, other: &Self) -> bool {
        self.username == other.username && self.password == other.password
    }
}
impl ::core::cmp::Eq for WSMAN_USERNAME_PASSWORD_CREDS {}
impl ::core::fmt::Debug for WSMAN_USERNAME_PASSWORD_CREDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WSMAN_USERNAME_PASSWORD_CREDS").field("username", &self.username).field("password", &self.password).finish()
    }
}
impl ::core::default::Default for WSManAuthenticationFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSManAuthenticationFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSManAuthenticationFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for WSManCallbackFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSManCallbackFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSManCallbackFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for WSManDataType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSManDataType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSManDataType").field(&self.0).finish()
    }
}
impl ::core::default::Default for WSManEnumFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSManEnumFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSManEnumFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for WSManProxyAccessType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSManProxyAccessType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSManProxyAccessType").field(&self.0).finish()
    }
}
impl ::core::default::Default for WSManProxyAccessTypeFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSManProxyAccessTypeFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSManProxyAccessTypeFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for WSManProxyAuthenticationFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSManProxyAuthenticationFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSManProxyAuthenticationFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for WSManSessionFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSManSessionFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSManSessionFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for WSManSessionOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSManSessionOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSManSessionOption").field(&self.0).finish()
    }
}
impl ::core::default::Default for WSManShellFlag {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WSManShellFlag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WSManShellFlag").field(&self.0).finish()
    }
}
