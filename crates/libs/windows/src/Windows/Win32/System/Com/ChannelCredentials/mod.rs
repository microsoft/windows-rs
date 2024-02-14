::windows_core::imp::com_interface!(IChannelCredentials, IChannelCredentials_Vtbl, 0x181b448c_c17c_4b17_ac6d_06699b93198f);
::windows_core::imp::interface_hierarchy!(IChannelCredentials, ::windows_core::IUnknown, super::IDispatch);
impl IChannelCredentials {
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
    pub unsafe fn SetClientCertificateFromStore<P0, P1, P2, P3>(&self, storelocation: P0, storename: P1, findyype: P2, findvalue: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<::windows_core::VARIANT>,
    {
        (::windows_core::Interface::vtable(self).SetClientCertificateFromStore)(::windows_core::Interface::as_raw(self), storelocation.into_param().abi(), storename.into_param().abi(), findyype.into_param().abi(), findvalue.into_param().abi()).ok()
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
    pub unsafe fn SetDefaultServiceCertificateFromStore<P0, P1, P2, P3>(&self, storelocation: P0, storename: P1, findtype: P2, findvalue: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
        P3: ::windows_core::IntoParam<::windows_core::VARIANT>,
    {
        (::windows_core::Interface::vtable(self).SetDefaultServiceCertificateFromStore)(::windows_core::Interface::as_raw(self), storelocation.into_param().abi(), storename.into_param().abi(), findtype.into_param().abi(), findvalue.into_param().abi()).ok()
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
#[repr(C)]
pub struct IChannelCredentials_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub SetWindowsCredential: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>, i32, super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetUserNameCredential: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetClientCertificateFromStore: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT,
    pub SetClientCertificateFromStoreByName: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetClientCertificateFromFile: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDefaultServiceCertificateFromStore: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::VARIANT>) -> ::windows_core::HRESULT,
    pub SetDefaultServiceCertificateFromStoreByName: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDefaultServiceCertificateFromFile: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetServiceCertificateAuthentication: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetIssuedToken: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>, ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
