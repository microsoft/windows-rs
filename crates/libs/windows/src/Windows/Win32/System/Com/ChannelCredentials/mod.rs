#[doc = "*Required features: `\"Win32_System_Com_ChannelCredentials\"`*"]
#[repr(transparent)]
pub struct IChannelCredentials(::windows::core::IUnknown);
impl IChannelCredentials {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWindowsCredential<P0>(&self, domain: &::windows::core::BSTR, username: &::windows::core::BSTR, password: &::windows::core::BSTR, impersonationlevel: i32, allowntlm: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetWindowsCredential)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(domain), ::core::mem::transmute_copy(username), ::core::mem::transmute_copy(password), impersonationlevel, allowntlm.into()).ok()
    }
    pub unsafe fn SetUserNameCredential(&self, username: &::windows::core::BSTR, password: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetUserNameCredential)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(username), ::core::mem::transmute_copy(password)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn SetClientCertificateFromStore(&self, storelocation: &::windows::core::BSTR, storename: &::windows::core::BSTR, findyype: &::windows::core::BSTR, findvalue: super::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetClientCertificateFromStore)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(storelocation), ::core::mem::transmute_copy(storename), ::core::mem::transmute_copy(findyype), ::core::mem::transmute(findvalue)).ok()
    }
    pub unsafe fn SetClientCertificateFromStoreByName(&self, subjectname: &::windows::core::BSTR, storelocation: &::windows::core::BSTR, storename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetClientCertificateFromStoreByName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(subjectname), ::core::mem::transmute_copy(storelocation), ::core::mem::transmute_copy(storename)).ok()
    }
    pub unsafe fn SetClientCertificateFromFile(&self, filename: &::windows::core::BSTR, password: &::windows::core::BSTR, keystorageflags: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetClientCertificateFromFile)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(filename), ::core::mem::transmute_copy(password), ::core::mem::transmute_copy(keystorageflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn SetDefaultServiceCertificateFromStore(&self, storelocation: &::windows::core::BSTR, storename: &::windows::core::BSTR, findtype: &::windows::core::BSTR, findvalue: super::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDefaultServiceCertificateFromStore)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(storelocation), ::core::mem::transmute_copy(storename), ::core::mem::transmute_copy(findtype), ::core::mem::transmute(findvalue)).ok()
    }
    pub unsafe fn SetDefaultServiceCertificateFromStoreByName(&self, subjectname: &::windows::core::BSTR, storelocation: &::windows::core::BSTR, storename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDefaultServiceCertificateFromStoreByName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(subjectname), ::core::mem::transmute_copy(storelocation), ::core::mem::transmute_copy(storename)).ok()
    }
    pub unsafe fn SetDefaultServiceCertificateFromFile(&self, filename: &::windows::core::BSTR, password: &::windows::core::BSTR, keystorageflags: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetDefaultServiceCertificateFromFile)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(filename), ::core::mem::transmute_copy(password), ::core::mem::transmute_copy(keystorageflags)).ok()
    }
    pub unsafe fn SetServiceCertificateAuthentication(&self, storelocation: &::windows::core::BSTR, revocationmode: &::windows::core::BSTR, certificatevalidationmode: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetServiceCertificateAuthentication)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(storelocation), ::core::mem::transmute_copy(revocationmode), ::core::mem::transmute_copy(certificatevalidationmode)).ok()
    }
    pub unsafe fn SetIssuedToken(&self, localissueraddres: &::windows::core::BSTR, localissuerbindingtype: &::windows::core::BSTR, localissuerbinding: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetIssuedToken)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(localissueraddres), ::core::mem::transmute_copy(localissuerbindingtype), ::core::mem::transmute_copy(localissuerbinding)).ok()
    }
}
::windows::imp::interface_hierarchy!(IChannelCredentials, ::windows::core::IUnknown, super::IDispatch);
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
unsafe impl ::windows::core::Vtable for IChannelCredentials {
    type Vtable = IChannelCredentials_Vtbl;
}
impl ::core::clone::Clone for IChannelCredentials {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for IChannelCredentials {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x181b448c_c17c_4b17_ac6d_06699b93198f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChannelCredentials_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWindowsCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, domain: ::std::mem::MaybeUninit<::windows::core::BSTR>, username: ::std::mem::MaybeUninit<::windows::core::BSTR>, password: ::std::mem::MaybeUninit<::windows::core::BSTR>, impersonationlevel: i32, allowntlm: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWindowsCredential: usize,
    pub SetUserNameCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::std::mem::MaybeUninit<::windows::core::BSTR>, password: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub SetClientCertificateFromStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storelocation: ::std::mem::MaybeUninit<::windows::core::BSTR>, storename: ::std::mem::MaybeUninit<::windows::core::BSTR>, findyype: ::std::mem::MaybeUninit<::windows::core::BSTR>, findvalue: super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    SetClientCertificateFromStore: usize,
    pub SetClientCertificateFromStoreByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subjectname: ::std::mem::MaybeUninit<::windows::core::BSTR>, storelocation: ::std::mem::MaybeUninit<::windows::core::BSTR>, storename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetClientCertificateFromFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows::core::BSTR>, password: ::std::mem::MaybeUninit<::windows::core::BSTR>, keystorageflags: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub SetDefaultServiceCertificateFromStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storelocation: ::std::mem::MaybeUninit<::windows::core::BSTR>, storename: ::std::mem::MaybeUninit<::windows::core::BSTR>, findtype: ::std::mem::MaybeUninit<::windows::core::BSTR>, findvalue: super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    SetDefaultServiceCertificateFromStore: usize,
    pub SetDefaultServiceCertificateFromStoreByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subjectname: ::std::mem::MaybeUninit<::windows::core::BSTR>, storelocation: ::std::mem::MaybeUninit<::windows::core::BSTR>, storename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetDefaultServiceCertificateFromFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows::core::BSTR>, password: ::std::mem::MaybeUninit<::windows::core::BSTR>, keystorageflags: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetServiceCertificateAuthentication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storelocation: ::std::mem::MaybeUninit<::windows::core::BSTR>, revocationmode: ::std::mem::MaybeUninit<::windows::core::BSTR>, certificatevalidationmode: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetIssuedToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localissueraddres: ::std::mem::MaybeUninit<::windows::core::BSTR>, localissuerbindingtype: ::std::mem::MaybeUninit<::windows::core::BSTR>, localissuerbinding: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
