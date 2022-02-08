pub trait IVpnChannelStatics_Impl: Sized {
    fn ProcessEventAsync(&self, thirdpartyplugin: &::core::option::Option<::windows::core::IInspectable>, event: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IVpnChannelStatics {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannelStatics";
}
impl IVpnChannelStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnChannelStatics_Impl, const OFFSET: isize>() -> IVpnChannelStatics_Vtbl {
        unsafe extern "system" fn ProcessEventAsync<Identity: ::windows::core::IUnknownImpl, Impl: IVpnChannelStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, thirdpartyplugin: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ProcessEventAsync(::core::mem::transmute(&thirdpartyplugin), ::core::mem::transmute(&event)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnChannelStatics, OFFSET>(),
            ProcessEventAsync: ProcessEventAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnChannelStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
pub trait IVpnCredential_Impl: Sized {
    fn PasskeyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn CertificateCredential(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn AdditionalPin(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OldPasswordCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
}
#[cfg(all(feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
impl ::windows::core::RuntimeName for IVpnCredential {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCredential";
}
#[cfg(all(feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
impl IVpnCredential_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCredential_Impl, const OFFSET: isize>() -> IVpnCredential_Vtbl {
        unsafe extern "system" fn PasskeyCredential<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCredential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PasskeyCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CertificateCredential<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCredential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CertificateCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdditionalPin<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCredential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AdditionalPin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OldPasswordCredential<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCredential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OldPasswordCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnCredential, OFFSET>(),
            PasskeyCredential: PasskeyCredential::<Identity, Impl, OFFSET>,
            CertificateCredential: CertificateCredential::<Identity, Impl, OFFSET>,
            AdditionalPin: AdditionalPin::<Identity, Impl, OFFSET>,
            OldPasswordCredential: OldPasswordCredential::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnCredential as ::windows::core::Interface>::IID
    }
}
pub trait IVpnCustomPrompt_Impl: Sized {
    fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCompulsory(&self, value: bool) -> ::windows::core::Result<()>;
    fn Compulsory(&self) -> ::windows::core::Result<bool>;
    fn SetBordered(&self, value: bool) -> ::windows::core::Result<()>;
    fn Bordered(&self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IVpnCustomPrompt {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomPrompt";
}
impl IVpnCustomPrompt_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPrompt_Impl, const OFFSET: isize>() -> IVpnCustomPrompt_Vtbl {
        unsafe extern "system" fn SetLabel<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPrompt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLabel(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Label<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPrompt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompulsory<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPrompt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCompulsory(value).into()
        }
        unsafe extern "system" fn Compulsory<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPrompt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Compulsory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBordered<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPrompt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBordered(value).into()
        }
        unsafe extern "system" fn Bordered<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPrompt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Bordered() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnCustomPrompt, OFFSET>(),
            SetLabel: SetLabel::<Identity, Impl, OFFSET>,
            Label: Label::<Identity, Impl, OFFSET>,
            SetCompulsory: SetCompulsory::<Identity, Impl, OFFSET>,
            Compulsory: Compulsory::<Identity, Impl, OFFSET>,
            SetBordered: SetBordered::<Identity, Impl, OFFSET>,
            Bordered: Bordered::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnCustomPrompt as ::windows::core::Interface>::IID
    }
}
pub trait IVpnCustomPromptElement_Impl: Sized {
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCompulsory(&self, value: bool) -> ::windows::core::Result<()>;
    fn Compulsory(&self) -> ::windows::core::Result<bool>;
    fn SetEmphasized(&self, value: bool) -> ::windows::core::Result<()>;
    fn Emphasized(&self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IVpnCustomPromptElement {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomPromptElement";
}
impl IVpnCustomPromptElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPromptElement_Impl, const OFFSET: isize>() -> IVpnCustomPromptElement_Vtbl {
        unsafe extern "system" fn SetDisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPromptElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisplayName(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPromptElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompulsory<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPromptElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCompulsory(value).into()
        }
        unsafe extern "system" fn Compulsory<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPromptElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Compulsory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmphasized<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPromptElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEmphasized(value).into()
        }
        unsafe extern "system" fn Emphasized<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPromptElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Emphasized() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnCustomPromptElement, OFFSET>(),
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            SetCompulsory: SetCompulsory::<Identity, Impl, OFFSET>,
            Compulsory: Compulsory::<Identity, Impl, OFFSET>,
            SetEmphasized: SetEmphasized::<Identity, Impl, OFFSET>,
            Emphasized: Emphasized::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnCustomPromptElement as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IVpnDomainNameInfoFactory_Impl: Sized {
    fn CreateVpnDomainNameInfo(&self, name: &::windows::core::HSTRING, nametype: VpnDomainNameType, dnsserverlist: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::HostName>>, proxyserverlist: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::HostName>>) -> ::windows::core::Result<VpnDomainNameInfo>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IVpnDomainNameInfoFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnDomainNameInfoFactory";
}
#[cfg(feature = "Foundation_Collections")]
impl IVpnDomainNameInfoFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnDomainNameInfoFactory_Impl, const OFFSET: isize>() -> IVpnDomainNameInfoFactory_Vtbl {
        unsafe extern "system" fn CreateVpnDomainNameInfo<Identity: ::windows::core::IUnknownImpl, Impl: IVpnDomainNameInfoFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, nametype: VpnDomainNameType, dnsserverlist: ::windows::core::RawPtr, proxyserverlist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateVpnDomainNameInfo(::core::mem::transmute(&name), nametype, ::core::mem::transmute(&dnsserverlist), ::core::mem::transmute(&proxyserverlist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnDomainNameInfoFactory, OFFSET>(),
            CreateVpnDomainNameInfo: CreateVpnDomainNameInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnDomainNameInfoFactory as ::windows::core::Interface>::IID
    }
}
pub trait IVpnInterfaceIdFactory_Impl: Sized {
    fn CreateVpnInterfaceId(&self, address: &[u8]) -> ::windows::core::Result<VpnInterfaceId>;
}
impl ::windows::core::RuntimeName for IVpnInterfaceIdFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnInterfaceIdFactory";
}
impl IVpnInterfaceIdFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnInterfaceIdFactory_Impl, const OFFSET: isize>() -> IVpnInterfaceIdFactory_Vtbl {
        unsafe extern "system" fn CreateVpnInterfaceId<Identity: ::windows::core::IUnknownImpl, Impl: IVpnInterfaceIdFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address_array_size: u32, address: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateVpnInterfaceId(::core::slice::from_raw_parts(::core::mem::transmute_copy(&address), address_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnInterfaceIdFactory, OFFSET>(),
            CreateVpnInterfaceId: CreateVpnInterfaceId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnInterfaceIdFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IVpnNamespaceInfoFactory_Impl: Sized {
    fn CreateVpnNamespaceInfo(&self, name: &::windows::core::HSTRING, dnsserverlist: &::core::option::Option<super::super::Foundation::Collections::IVector<super::HostName>>, proxyserverlist: &::core::option::Option<super::super::Foundation::Collections::IVector<super::HostName>>) -> ::windows::core::Result<VpnNamespaceInfo>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IVpnNamespaceInfoFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnNamespaceInfoFactory";
}
#[cfg(feature = "Foundation_Collections")]
impl IVpnNamespaceInfoFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnNamespaceInfoFactory_Impl, const OFFSET: isize>() -> IVpnNamespaceInfoFactory_Vtbl {
        unsafe extern "system" fn CreateVpnNamespaceInfo<Identity: ::windows::core::IUnknownImpl, Impl: IVpnNamespaceInfoFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, dnsserverlist: ::windows::core::RawPtr, proxyserverlist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateVpnNamespaceInfo(::core::mem::transmute(&name), ::core::mem::transmute(&dnsserverlist), ::core::mem::transmute(&proxyserverlist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnNamespaceInfoFactory, OFFSET>(),
            CreateVpnNamespaceInfo: CreateVpnNamespaceInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnNamespaceInfoFactory as ::windows::core::Interface>::IID
    }
}
pub trait IVpnPacketBufferFactory_Impl: Sized {
    fn CreateVpnPacketBuffer(&self, parentbuffer: &::core::option::Option<VpnPacketBuffer>, offset: u32, length: u32) -> ::windows::core::Result<VpnPacketBuffer>;
}
impl ::windows::core::RuntimeName for IVpnPacketBufferFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPacketBufferFactory";
}
impl IVpnPacketBufferFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPacketBufferFactory_Impl, const OFFSET: isize>() -> IVpnPacketBufferFactory_Vtbl {
        unsafe extern "system" fn CreateVpnPacketBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPacketBufferFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parentbuffer: ::windows::core::RawPtr, offset: u32, length: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateVpnPacketBuffer(::core::mem::transmute(&parentbuffer), offset, length) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnPacketBufferFactory, OFFSET>(),
            CreateVpnPacketBuffer: CreateVpnPacketBuffer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnPacketBufferFactory as ::windows::core::Interface>::IID
    }
}
pub trait IVpnPlugIn_Impl: Sized {
    fn Connect(&self, channel: &::core::option::Option<VpnChannel>) -> ::windows::core::Result<()>;
    fn Disconnect(&self, channel: &::core::option::Option<VpnChannel>) -> ::windows::core::Result<()>;
    fn GetKeepAlivePayload(&self, channel: &::core::option::Option<VpnChannel>, keepalivepacket: &mut ::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()>;
    fn Encapsulate(&self, channel: &::core::option::Option<VpnChannel>, packets: &::core::option::Option<VpnPacketBufferList>, encapulatedpackets: &::core::option::Option<VpnPacketBufferList>) -> ::windows::core::Result<()>;
    fn Decapsulate(&self, channel: &::core::option::Option<VpnChannel>, encapbuffer: &::core::option::Option<VpnPacketBuffer>, decapsulatedpackets: &::core::option::Option<VpnPacketBufferList>, controlpacketstosend: &::core::option::Option<VpnPacketBufferList>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IVpnPlugIn {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPlugIn";
}
impl IVpnPlugIn_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPlugIn_Impl, const OFFSET: isize>() -> IVpnPlugIn_Vtbl {
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Connect(::core::mem::transmute(&channel)).into()
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Disconnect(::core::mem::transmute(&channel)).into()
        }
        unsafe extern "system" fn GetKeepAlivePayload<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: ::windows::core::RawPtr, keepalivepacket: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetKeepAlivePayload(::core::mem::transmute(&channel), ::core::mem::transmute_copy(&keepalivepacket)).into()
        }
        unsafe extern "system" fn Encapsulate<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: ::windows::core::RawPtr, packets: ::windows::core::RawPtr, encapulatedpackets: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Encapsulate(::core::mem::transmute(&channel), ::core::mem::transmute(&packets), ::core::mem::transmute(&encapulatedpackets)).into()
        }
        unsafe extern "system" fn Decapsulate<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: ::windows::core::RawPtr, encapbuffer: ::windows::core::RawPtr, decapsulatedpackets: ::windows::core::RawPtr, controlpacketstosend: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Decapsulate(::core::mem::transmute(&channel), ::core::mem::transmute(&encapbuffer), ::core::mem::transmute(&decapsulatedpackets), ::core::mem::transmute(&controlpacketstosend)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnPlugIn, OFFSET>(),
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            GetKeepAlivePayload: GetKeepAlivePayload::<Identity, Impl, OFFSET>,
            Encapsulate: Encapsulate::<Identity, Impl, OFFSET>,
            Decapsulate: Decapsulate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnPlugIn as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IVpnProfile_Impl: Sized {
    fn ProfileName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetProfileName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AppTriggers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnAppId>>;
    fn Routes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnRoute>>;
    fn DomainNameInfoList(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnDomainNameInfo>>;
    fn TrafficFilters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnTrafficFilter>>;
    fn RememberCredentials(&self) -> ::windows::core::Result<bool>;
    fn SetRememberCredentials(&self, value: bool) -> ::windows::core::Result<()>;
    fn AlwaysOn(&self) -> ::windows::core::Result<bool>;
    fn SetAlwaysOn(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IVpnProfile {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnProfile";
}
#[cfg(feature = "Foundation_Collections")]
impl IVpnProfile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnProfile_Impl, const OFFSET: isize>() -> IVpnProfile_Vtbl {
        unsafe extern "system" fn ProfileName<Identity: ::windows::core::IUnknownImpl, Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProfileName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProfileName<Identity: ::windows::core::IUnknownImpl, Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProfileName(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn AppTriggers<Identity: ::windows::core::IUnknownImpl, Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AppTriggers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Routes<Identity: ::windows::core::IUnknownImpl, Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Routes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainNameInfoList<Identity: ::windows::core::IUnknownImpl, Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DomainNameInfoList() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrafficFilters<Identity: ::windows::core::IUnknownImpl, Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TrafficFilters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RememberCredentials<Identity: ::windows::core::IUnknownImpl, Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RememberCredentials() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRememberCredentials<Identity: ::windows::core::IUnknownImpl, Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRememberCredentials(value).into()
        }
        unsafe extern "system" fn AlwaysOn<Identity: ::windows::core::IUnknownImpl, Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AlwaysOn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlwaysOn<Identity: ::windows::core::IUnknownImpl, Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAlwaysOn(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnProfile, OFFSET>(),
            ProfileName: ProfileName::<Identity, Impl, OFFSET>,
            SetProfileName: SetProfileName::<Identity, Impl, OFFSET>,
            AppTriggers: AppTriggers::<Identity, Impl, OFFSET>,
            Routes: Routes::<Identity, Impl, OFFSET>,
            DomainNameInfoList: DomainNameInfoList::<Identity, Impl, OFFSET>,
            TrafficFilters: TrafficFilters::<Identity, Impl, OFFSET>,
            RememberCredentials: RememberCredentials::<Identity, Impl, OFFSET>,
            SetRememberCredentials: SetRememberCredentials::<Identity, Impl, OFFSET>,
            AlwaysOn: AlwaysOn::<Identity, Impl, OFFSET>,
            SetAlwaysOn: SetAlwaysOn::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnProfile as ::windows::core::Interface>::IID
    }
}
pub trait IVpnRouteFactory_Impl: Sized {
    fn CreateVpnRoute(&self, address: &::core::option::Option<super::HostName>, prefixsize: u8) -> ::windows::core::Result<VpnRoute>;
}
impl ::windows::core::RuntimeName for IVpnRouteFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnRouteFactory";
}
impl IVpnRouteFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnRouteFactory_Impl, const OFFSET: isize>() -> IVpnRouteFactory_Vtbl {
        unsafe extern "system" fn CreateVpnRoute<Identity: ::windows::core::IUnknownImpl, Impl: IVpnRouteFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: ::windows::core::RawPtr, prefixsize: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateVpnRoute(::core::mem::transmute(&address), prefixsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnRouteFactory, OFFSET>(), CreateVpnRoute: CreateVpnRoute::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnRouteFactory as ::windows::core::Interface>::IID
    }
}
