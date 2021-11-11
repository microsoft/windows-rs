#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_Com_ChannelCredentials`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IChannelCredentials(pub ::windows::core::IUnknown);
impl IChannelCredentials {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_ChannelCredentials`, `Win32_Foundation`*"]
    pub unsafe fn SetWindowsCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, domain: Param0, username: Param1, password: Param2, impersonationlevel: i32, allowntlm: Param4) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), domain.into_param().abi(), username.into_param().abi(), password.into_param().abi(), ::core::mem::transmute(impersonationlevel), allowntlm.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_ChannelCredentials`, `Win32_Foundation`*"]
    pub unsafe fn SetUserNameCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, username: Param0, password: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), username.into_param().abi(), password.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_Com_ChannelCredentials`, `Win32_Foundation`, `Win32_System_Ole`*"]
    pub unsafe fn SetClientCertificateFromStore<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::VARIANT>>(&self, storelocation: Param0, storename: Param1, findyype: Param2, findvalue: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), storelocation.into_param().abi(), storename.into_param().abi(), findyype.into_param().abi(), findvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_ChannelCredentials`, `Win32_Foundation`*"]
    pub unsafe fn SetClientCertificateFromStoreByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, subjectname: Param0, storelocation: Param1, storename: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), subjectname.into_param().abi(), storelocation.into_param().abi(), storename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_ChannelCredentials`, `Win32_Foundation`*"]
    pub unsafe fn SetClientCertificateFromFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, filename: Param0, password: Param1, keystorageflags: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), filename.into_param().abi(), password.into_param().abi(), keystorageflags.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    #[doc = "*Required features: `Win32_System_Com_ChannelCredentials`, `Win32_Foundation`, `Win32_System_Ole`*"]
    pub unsafe fn SetDefaultServiceCertificateFromStore<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param3: ::windows::core::IntoParam<'a, super::VARIANT>>(&self, storelocation: Param0, storename: Param1, findtype: Param2, findvalue: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), storelocation.into_param().abi(), storename.into_param().abi(), findtype.into_param().abi(), findvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_ChannelCredentials`, `Win32_Foundation`*"]
    pub unsafe fn SetDefaultServiceCertificateFromStoreByName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, subjectname: Param0, storelocation: Param1, storename: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), subjectname.into_param().abi(), storelocation.into_param().abi(), storename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_ChannelCredentials`, `Win32_Foundation`*"]
    pub unsafe fn SetDefaultServiceCertificateFromFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, filename: Param0, password: Param1, keystorageflags: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), filename.into_param().abi(), password.into_param().abi(), keystorageflags.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_ChannelCredentials`, `Win32_Foundation`*"]
    pub unsafe fn SetServiceCertificateAuthentication<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, storelocation: Param0, revocationmode: Param1, certificatevalidationmode: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), storelocation.into_param().abi(), revocationmode.into_param().abi(), certificatevalidationmode.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Com_ChannelCredentials`, `Win32_Foundation`*"]
    pub unsafe fn SetIssuedToken<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, localissueraddres: Param0, localissuerbindingtype: Param1, localissuerbinding: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), localissueraddres.into_param().abi(), localissuerbindingtype.into_param().abi(), localissuerbinding.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IChannelCredentials {
    type Vtable = IChannelCredentials_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x181b448c_c17c_4b17_ac6d_06699b93198f);
}
impl ::core::convert::From<IChannelCredentials> for ::windows::core::IUnknown {
    fn from(value: IChannelCredentials) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IChannelCredentials> for ::windows::core::IUnknown {
    fn from(value: &IChannelCredentials) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IChannelCredentials {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IChannelCredentials {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IChannelCredentials> for super::IDispatch {
    fn from(value: IChannelCredentials) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IChannelCredentials> for super::IDispatch {
    fn from(value: &IChannelCredentials) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IDispatch> for IChannelCredentials {
    fn into_param(self) -> ::windows::core::Param<'a, super::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IDispatch> for &IChannelCredentials {
    fn into_param(self) -> ::windows::core::Param<'a, super::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IChannelCredentials_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pctinfo: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const super::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::EXCEPINFO>, puargerr: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, domain: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, username: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, impersonationlevel: i32, allowntlm: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, username: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findyype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findvalue: ::core::mem::ManuallyDrop<super::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, subjectname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, keystorageflags: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findvalue: ::core::mem::ManuallyDrop<super::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, subjectname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, filename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, keystorageflags: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, storelocation: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, revocationmode: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, certificatevalidationmode: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, localissueraddres: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, localissuerbindingtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, localissuerbinding: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
