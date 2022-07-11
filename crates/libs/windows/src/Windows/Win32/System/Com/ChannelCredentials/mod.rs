#[doc = "*Required features: `\"Win32_System_Com_ChannelCredentials\"`*"]
#[repr(transparent)]
pub struct IChannelCredentials(::windows::core::IUnknown);
impl IChannelCredentials {
    #[doc = "*Required features: `\"Win32_System_Com_ChannelCredentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWindowsCredential<'a, Param0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>, Param1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>, Param2: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>, Param4: ::std::convert::Into<super::super::super::Foundation::BOOL>>(&self, domain: Param0, username: Param1, password: Param2, impersonationlevel: i32, allowntlm: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWindowsCredential)(::windows::core::Interface::as_raw(self), domain.into().abi(), username.into().abi(), password.into().abi(), ::core::mem::transmute(impersonationlevel), allowntlm.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_ChannelCredentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserNameCredential<'a, Param0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>, Param1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>>(&self, username: Param0, password: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetUserNameCredential)(::windows::core::Interface::as_raw(self), username.into().abi(), password.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_ChannelCredentials\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn SetClientCertificateFromStore<'a, Param0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>, Param1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>, Param2: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>, Param3: ::std::convert::Into<::windows::core::InParam<'a, super::VARIANT>>>(&self, storelocation: Param0, storename: Param1, findyype: Param2, findvalue: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClientCertificateFromStore)(::windows::core::Interface::as_raw(self), storelocation.into().abi(), storename.into().abi(), findyype.into().abi(), findvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_ChannelCredentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientCertificateFromStoreByName<'a, Param0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>, Param1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>, Param2: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>>(&self, subjectname: Param0, storelocation: Param1, storename: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClientCertificateFromStoreByName)(::windows::core::Interface::as_raw(self), subjectname.into().abi(), storelocation.into().abi(), storename.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_ChannelCredentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientCertificateFromFile<'a, Param0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>, Param1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>, Param2: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>>(&self, filename: Param0, password: Param1, keystorageflags: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClientCertificateFromFile)(::windows::core::Interface::as_raw(self), filename.into().abi(), password.into().abi(), keystorageflags.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_ChannelCredentials\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn SetDefaultServiceCertificateFromStore<'a, Param0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>, Param1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>, Param2: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>, Param3: ::std::convert::Into<::windows::core::InParam<'a, super::VARIANT>>>(&self, storelocation: Param0, storename: Param1, findtype: Param2, findvalue: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDefaultServiceCertificateFromStore)(::windows::core::Interface::as_raw(self), storelocation.into().abi(), storename.into().abi(), findtype.into().abi(), findvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_ChannelCredentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDefaultServiceCertificateFromStoreByName<'a, Param0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>, Param1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>, Param2: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>>(&self, subjectname: Param0, storelocation: Param1, storename: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDefaultServiceCertificateFromStoreByName)(::windows::core::Interface::as_raw(self), subjectname.into().abi(), storelocation.into().abi(), storename.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_ChannelCredentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDefaultServiceCertificateFromFile<'a, Param0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>, Param1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>, Param2: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>>(&self, filename: Param0, password: Param1, keystorageflags: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDefaultServiceCertificateFromFile)(::windows::core::Interface::as_raw(self), filename.into().abi(), password.into().abi(), keystorageflags.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_ChannelCredentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceCertificateAuthentication<'a, Param0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>, Param1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>, Param2: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>>(&self, storelocation: Param0, revocationmode: Param1, certificatevalidationmode: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetServiceCertificateAuthentication)(::windows::core::Interface::as_raw(self), storelocation.into().abi(), revocationmode.into().abi(), certificatevalidationmode.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_ChannelCredentials\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIssuedToken<'a, Param0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>, Param1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>, Param2: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>>(&self, localissueraddres: Param0, localissuerbindingtype: Param1, localissuerbinding: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIssuedToken)(::windows::core::Interface::as_raw(self), localissueraddres.into().abi(), localissuerbindingtype.into().abi(), localissuerbinding.into().abi()).ok()
    }
}
impl ::core::convert::From<IChannelCredentials> for ::windows::core::IUnknown {
    fn from(value: IChannelCredentials) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IChannelCredentials> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IChannelCredentials) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IChannelCredentials> for ::windows::core::IUnknown {
    fn from(value: &IChannelCredentials) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IChannelCredentials> for super::IDispatch {
    fn from(value: IChannelCredentials) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IChannelCredentials> for &'a super::IDispatch {
    fn from(value: &'a IChannelCredentials) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IChannelCredentials> for super::IDispatch {
    fn from(value: &IChannelCredentials) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IChannelCredentials {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IChannelCredentials {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IChannelCredentials {}
impl ::core::fmt::Debug for IChannelCredentials {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IChannelCredentials").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IChannelCredentials {
    type Vtable = IChannelCredentials_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x181b448c_c17c_4b17_ac6d_06699b93198f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChannelCredentials_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWindowsCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, domain: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, impersonationlevel: i32, allowntlm: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWindowsCredential: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUserNameCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUserNameCredential: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub SetClientCertificateFromStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findyype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findvalue: ::core::mem::ManuallyDrop<super::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    SetClientCertificateFromStore: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientCertificateFromStoreByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subjectname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientCertificateFromStoreByName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClientCertificateFromFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, keystorageflags: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClientCertificateFromFile: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub SetDefaultServiceCertificateFromStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findvalue: ::core::mem::ManuallyDrop<super::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    SetDefaultServiceCertificateFromStore: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDefaultServiceCertificateFromStoreByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subjectname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDefaultServiceCertificateFromStoreByName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDefaultServiceCertificateFromFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, keystorageflags: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDefaultServiceCertificateFromFile: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetServiceCertificateAuthentication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, revocationmode: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, certificatevalidationmode: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetServiceCertificateAuthentication: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIssuedToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localissueraddres: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, localissuerbindingtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, localissuerbinding: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIssuedToken: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
