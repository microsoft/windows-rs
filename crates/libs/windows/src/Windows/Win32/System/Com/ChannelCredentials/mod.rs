#[doc = "*Required features: `\"Win32_System_Com_ChannelCredentials\"`*"]
#[repr(transparent)]
pub struct IChannelCredentials(::windows_core::IUnknown);
impl IChannelCredentials {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWindowsCredential<P0, P1, P2, P3>(&self, domain: P0, username: P1, password: P2, impersonationlevel: i32, allowntlm: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetWindowsCredential)(::windows_core::Interface::as_raw(self), domain.into_param().abi(), username.into_param().abi(), password.into_param().abi(), impersonationlevel, allowntlm.into_param().abi()).ok()
    }
    pub unsafe fn SetUserNameCredential<P0, P1>(&self, username: P0, password: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetUserNameCredential)(::windows_core::Interface::as_raw(self), username.into_param().abi(), password.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetClientCertificateFromStore<P0, P1, P2>(&self, storelocation: P0, storename: P1, findyype: P2, findvalue: super::super::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetClientCertificateFromStore)(::windows_core::Interface::as_raw(self), storelocation.into_param().abi(), storename.into_param().abi(), findyype.into_param().abi(), ::core::mem::transmute(findvalue)).ok()
    }
    pub unsafe fn SetClientCertificateFromStoreByName<P0, P1, P2>(&self, subjectname: P0, storelocation: P1, storename: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetClientCertificateFromStoreByName)(::windows_core::Interface::as_raw(self), subjectname.into_param().abi(), storelocation.into_param().abi(), storename.into_param().abi()).ok()
    }
    pub unsafe fn SetClientCertificateFromFile<P0, P1, P2>(&self, filename: P0, password: P1, keystorageflags: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetClientCertificateFromFile)(::windows_core::Interface::as_raw(self), filename.into_param().abi(), password.into_param().abi(), keystorageflags.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetDefaultServiceCertificateFromStore<P0, P1, P2>(&self, storelocation: P0, storename: P1, findtype: P2, findvalue: super::super::Variant::VARIANT) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDefaultServiceCertificateFromStore)(::windows_core::Interface::as_raw(self), storelocation.into_param().abi(), storename.into_param().abi(), findtype.into_param().abi(), ::core::mem::transmute(findvalue)).ok()
    }
    pub unsafe fn SetDefaultServiceCertificateFromStoreByName<P0, P1, P2>(&self, subjectname: P0, storelocation: P1, storename: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDefaultServiceCertificateFromStoreByName)(::windows_core::Interface::as_raw(self), subjectname.into_param().abi(), storelocation.into_param().abi(), storename.into_param().abi()).ok()
    }
    pub unsafe fn SetDefaultServiceCertificateFromFile<P0, P1, P2>(&self, filename: P0, password: P1, keystorageflags: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDefaultServiceCertificateFromFile)(::windows_core::Interface::as_raw(self), filename.into_param().abi(), password.into_param().abi(), keystorageflags.into_param().abi()).ok()
    }
    pub unsafe fn SetServiceCertificateAuthentication<P0, P1, P2>(&self, storelocation: P0, revocationmode: P1, certificatevalidationmode: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetServiceCertificateAuthentication)(::windows_core::Interface::as_raw(self), storelocation.into_param().abi(), revocationmode.into_param().abi(), certificatevalidationmode.into_param().abi()).ok()
    }
    pub unsafe fn SetIssuedToken<P0, P1, P2>(&self, localissueraddres: P0, localissuerbindingtype: P1, localissuerbinding: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetIssuedToken)(::windows_core::Interface::as_raw(self), localissueraddres.into_param().abi(), localissuerbindingtype.into_param().abi(), localissuerbinding.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IChannelCredentials, ::windows_core::IUnknown, super::IDispatch);
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
unsafe impl ::windows_core::Interface for IChannelCredentials {
    type Vtable = IChannelCredentials_Vtbl;
}
impl ::core::clone::Clone for IChannelCredentials {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChannelCredentials {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x181b448c_c17c_4b17_ac6d_06699b93198f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChannelCredentials_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetWindowsCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, domain: ::std::mem::MaybeUninit<::windows_core::BSTR>, username: ::std::mem::MaybeUninit<::windows_core::BSTR>, password: ::std::mem::MaybeUninit<::windows_core::BSTR>, impersonationlevel: i32, allowntlm: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetWindowsCredential: usize,
    pub SetUserNameCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::std::mem::MaybeUninit<::windows_core::BSTR>, password: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetClientCertificateFromStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storelocation: ::std::mem::MaybeUninit<::windows_core::BSTR>, storename: ::std::mem::MaybeUninit<::windows_core::BSTR>, findyype: ::std::mem::MaybeUninit<::windows_core::BSTR>, findvalue: super::super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetClientCertificateFromStore: usize,
    pub SetClientCertificateFromStoreByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subjectname: ::std::mem::MaybeUninit<::windows_core::BSTR>, storelocation: ::std::mem::MaybeUninit<::windows_core::BSTR>, storename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetClientCertificateFromFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows_core::BSTR>, password: ::std::mem::MaybeUninit<::windows_core::BSTR>, keystorageflags: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetDefaultServiceCertificateFromStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storelocation: ::std::mem::MaybeUninit<::windows_core::BSTR>, storename: ::std::mem::MaybeUninit<::windows_core::BSTR>, findtype: ::std::mem::MaybeUninit<::windows_core::BSTR>, findvalue: super::super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetDefaultServiceCertificateFromStore: usize,
    pub SetDefaultServiceCertificateFromStoreByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subjectname: ::std::mem::MaybeUninit<::windows_core::BSTR>, storelocation: ::std::mem::MaybeUninit<::windows_core::BSTR>, storename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDefaultServiceCertificateFromFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows_core::BSTR>, password: ::std::mem::MaybeUninit<::windows_core::BSTR>, keystorageflags: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetServiceCertificateAuthentication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storelocation: ::std::mem::MaybeUninit<::windows_core::BSTR>, revocationmode: ::std::mem::MaybeUninit<::windows_core::BSTR>, certificatevalidationmode: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetIssuedToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localissueraddres: ::std::mem::MaybeUninit<::windows_core::BSTR>, localissuerbindingtype: ::std::mem::MaybeUninit<::windows_core::BSTR>, localissuerbinding: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
