pub trait IVpnChannelStatics_Impl: Sized {
    fn ProcessEventAsync(&mut self, thirdpartyplugin: &::core::option::Option<::windows::core::IInspectable>, event: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IVpnChannelStatics {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannelStatics";
}
impl IVpnChannelStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnChannelStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnChannelStatics_Vtbl {
        unsafe extern "system" fn ProcessEventAsync<Impl: IVpnChannelStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, thirdpartyplugin: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessEventAsync(&*(&thirdpartyplugin as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&event as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnChannelStatics, BASE_OFFSET>(),
            ProcessEventAsync: ProcessEventAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnChannelStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
pub trait IVpnCredential_Impl: Sized {
    fn PasskeyCredential(&mut self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn CertificateCredential(&mut self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn AdditionalPin(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OldPasswordCredential(&mut self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
}
#[cfg(all(feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
impl ::windows::core::RuntimeName for IVpnCredential {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCredential";
}
#[cfg(all(feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
impl IVpnCredential_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCredential_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnCredential_Vtbl {
        unsafe extern "system" fn PasskeyCredential<Impl: IVpnCredential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PasskeyCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CertificateCredential<Impl: IVpnCredential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CertificateCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdditionalPin<Impl: IVpnCredential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdditionalPin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OldPasswordCredential<Impl: IVpnCredential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnCredential, BASE_OFFSET>(),
            PasskeyCredential: PasskeyCredential::<Impl, IMPL_OFFSET>,
            CertificateCredential: CertificateCredential::<Impl, IMPL_OFFSET>,
            AdditionalPin: AdditionalPin::<Impl, IMPL_OFFSET>,
            OldPasswordCredential: OldPasswordCredential::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnCredential as ::windows::core::Interface>::IID
    }
}
pub trait IVpnCustomPrompt_Impl: Sized {
    fn SetLabel(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Label(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCompulsory(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Compulsory(&mut self) -> ::windows::core::Result<bool>;
    fn SetBordered(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Bordered(&mut self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IVpnCustomPrompt {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomPrompt";
}
impl IVpnCustomPrompt_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPrompt_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnCustomPrompt_Vtbl {
        unsafe extern "system" fn SetLabel<Impl: IVpnCustomPrompt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabel(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Label<Impl: IVpnCustomPrompt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompulsory<Impl: IVpnCustomPrompt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompulsory(value).into()
        }
        unsafe extern "system" fn Compulsory<Impl: IVpnCustomPrompt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Compulsory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBordered<Impl: IVpnCustomPrompt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBordered(value).into()
        }
        unsafe extern "system" fn Bordered<Impl: IVpnCustomPrompt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnCustomPrompt, BASE_OFFSET>(),
            SetLabel: SetLabel::<Impl, IMPL_OFFSET>,
            Label: Label::<Impl, IMPL_OFFSET>,
            SetCompulsory: SetCompulsory::<Impl, IMPL_OFFSET>,
            Compulsory: Compulsory::<Impl, IMPL_OFFSET>,
            SetBordered: SetBordered::<Impl, IMPL_OFFSET>,
            Bordered: Bordered::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnCustomPrompt as ::windows::core::Interface>::IID
    }
}
pub trait IVpnCustomPromptElement_Impl: Sized {
    fn SetDisplayName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCompulsory(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Compulsory(&mut self) -> ::windows::core::Result<bool>;
    fn SetEmphasized(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Emphasized(&mut self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IVpnCustomPromptElement {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomPromptElement";
}
impl IVpnCustomPromptElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPromptElement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnCustomPromptElement_Vtbl {
        unsafe extern "system" fn SetDisplayName<Impl: IVpnCustomPromptElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayName<Impl: IVpnCustomPromptElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompulsory<Impl: IVpnCustomPromptElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompulsory(value).into()
        }
        unsafe extern "system" fn Compulsory<Impl: IVpnCustomPromptElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Compulsory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmphasized<Impl: IVpnCustomPromptElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEmphasized(value).into()
        }
        unsafe extern "system" fn Emphasized<Impl: IVpnCustomPromptElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnCustomPromptElement, BASE_OFFSET>(),
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            SetCompulsory: SetCompulsory::<Impl, IMPL_OFFSET>,
            Compulsory: Compulsory::<Impl, IMPL_OFFSET>,
            SetEmphasized: SetEmphasized::<Impl, IMPL_OFFSET>,
            Emphasized: Emphasized::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnCustomPromptElement as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IVpnDomainNameInfoFactory_Impl: Sized {
    fn CreateVpnDomainNameInfo(&mut self, name: &::windows::core::HSTRING, nametype: VpnDomainNameType, dnsserverlist: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::HostName>>, proxyserverlist: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::HostName>>) -> ::windows::core::Result<VpnDomainNameInfo>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IVpnDomainNameInfoFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnDomainNameInfoFactory";
}
#[cfg(feature = "Foundation_Collections")]
impl IVpnDomainNameInfoFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnDomainNameInfoFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnDomainNameInfoFactory_Vtbl {
        unsafe extern "system" fn CreateVpnDomainNameInfo<Impl: IVpnDomainNameInfoFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, nametype: VpnDomainNameType, dnsserverlist: ::windows::core::RawPtr, proxyserverlist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVpnDomainNameInfo(
                &*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                nametype,
                &*(&dnsserverlist as *const <super::super::Foundation::Collections::IIterable<super::HostName> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::HostName> as ::windows::core::DefaultType>::DefaultType),
                &*(&proxyserverlist as *const <super::super::Foundation::Collections::IIterable<super::HostName> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::HostName> as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnDomainNameInfoFactory, BASE_OFFSET>(),
            CreateVpnDomainNameInfo: CreateVpnDomainNameInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnDomainNameInfoFactory as ::windows::core::Interface>::IID
    }
}
pub trait IVpnInterfaceIdFactory_Impl: Sized {
    fn CreateVpnInterfaceId(&mut self, address: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<VpnInterfaceId>;
}
impl ::windows::core::RuntimeName for IVpnInterfaceIdFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnInterfaceIdFactory";
}
impl IVpnInterfaceIdFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnInterfaceIdFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnInterfaceIdFactory_Vtbl {
        unsafe extern "system" fn CreateVpnInterfaceId<Impl: IVpnInterfaceIdFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address_array_size: u32, address: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnInterfaceIdFactory, BASE_OFFSET>(),
            CreateVpnInterfaceId: CreateVpnInterfaceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnInterfaceIdFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IVpnNamespaceInfoFactory_Impl: Sized {
    fn CreateVpnNamespaceInfo(&mut self, name: &::windows::core::HSTRING, dnsserverlist: &::core::option::Option<super::super::Foundation::Collections::IVector<super::HostName>>, proxyserverlist: &::core::option::Option<super::super::Foundation::Collections::IVector<super::HostName>>) -> ::windows::core::Result<VpnNamespaceInfo>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IVpnNamespaceInfoFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnNamespaceInfoFactory";
}
#[cfg(feature = "Foundation_Collections")]
impl IVpnNamespaceInfoFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnNamespaceInfoFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnNamespaceInfoFactory_Vtbl {
        unsafe extern "system" fn CreateVpnNamespaceInfo<Impl: IVpnNamespaceInfoFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, dnsserverlist: ::windows::core::RawPtr, proxyserverlist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVpnNamespaceInfo(
                &*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&dnsserverlist as *const <super::super::Foundation::Collections::IVector<super::HostName> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<super::HostName> as ::windows::core::DefaultType>::DefaultType),
                &*(&proxyserverlist as *const <super::super::Foundation::Collections::IVector<super::HostName> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<super::HostName> as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnNamespaceInfoFactory, BASE_OFFSET>(),
            CreateVpnNamespaceInfo: CreateVpnNamespaceInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnNamespaceInfoFactory as ::windows::core::Interface>::IID
    }
}
pub trait IVpnPacketBufferFactory_Impl: Sized {
    fn CreateVpnPacketBuffer(&mut self, parentbuffer: &::core::option::Option<VpnPacketBuffer>, offset: u32, length: u32) -> ::windows::core::Result<VpnPacketBuffer>;
}
impl ::windows::core::RuntimeName for IVpnPacketBufferFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPacketBufferFactory";
}
impl IVpnPacketBufferFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPacketBufferFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnPacketBufferFactory_Vtbl {
        unsafe extern "system" fn CreateVpnPacketBuffer<Impl: IVpnPacketBufferFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parentbuffer: ::windows::core::RawPtr, offset: u32, length: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVpnPacketBuffer(&*(&parentbuffer as *const <VpnPacketBuffer as ::windows::core::Abi>::Abi as *const <VpnPacketBuffer as ::windows::core::DefaultType>::DefaultType), offset, length) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnPacketBufferFactory, BASE_OFFSET>(),
            CreateVpnPacketBuffer: CreateVpnPacketBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnPacketBufferFactory as ::windows::core::Interface>::IID
    }
}
pub trait IVpnPlugIn_Impl: Sized {
    fn Connect(&mut self, channel: &::core::option::Option<VpnChannel>) -> ::windows::core::Result<()>;
    fn Disconnect(&mut self, channel: &::core::option::Option<VpnChannel>) -> ::windows::core::Result<()>;
    fn GetKeepAlivePayload(&mut self, channel: &::core::option::Option<VpnChannel>, keepalivepacket: &mut ::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()>;
    fn Encapsulate(&mut self, channel: &::core::option::Option<VpnChannel>, packets: &::core::option::Option<VpnPacketBufferList>, encapulatedpackets: &::core::option::Option<VpnPacketBufferList>) -> ::windows::core::Result<()>;
    fn Decapsulate(&mut self, channel: &::core::option::Option<VpnChannel>, encapbuffer: &::core::option::Option<VpnPacketBuffer>, decapsulatedpackets: &::core::option::Option<VpnPacketBufferList>, controlpacketstosend: &::core::option::Option<VpnPacketBufferList>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IVpnPlugIn {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPlugIn";
}
impl IVpnPlugIn_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPlugIn_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnPlugIn_Vtbl {
        unsafe extern "system" fn Connect<Impl: IVpnPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Connect(&*(&channel as *const <VpnChannel as ::windows::core::Abi>::Abi as *const <VpnChannel as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Disconnect<Impl: IVpnPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect(&*(&channel as *const <VpnChannel as ::windows::core::Abi>::Abi as *const <VpnChannel as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetKeepAlivePayload<Impl: IVpnPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: ::windows::core::RawPtr, keepalivepacket: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetKeepAlivePayload(&*(&channel as *const <VpnChannel as ::windows::core::Abi>::Abi as *const <VpnChannel as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&keepalivepacket)).into()
        }
        unsafe extern "system" fn Encapsulate<Impl: IVpnPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: ::windows::core::RawPtr, packets: ::windows::core::RawPtr, encapulatedpackets: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Encapsulate(&*(&channel as *const <VpnChannel as ::windows::core::Abi>::Abi as *const <VpnChannel as ::windows::core::DefaultType>::DefaultType), &*(&packets as *const <VpnPacketBufferList as ::windows::core::Abi>::Abi as *const <VpnPacketBufferList as ::windows::core::DefaultType>::DefaultType), &*(&encapulatedpackets as *const <VpnPacketBufferList as ::windows::core::Abi>::Abi as *const <VpnPacketBufferList as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Decapsulate<Impl: IVpnPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: ::windows::core::RawPtr, encapbuffer: ::windows::core::RawPtr, decapsulatedpackets: ::windows::core::RawPtr, controlpacketstosend: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .Decapsulate(
                    &*(&channel as *const <VpnChannel as ::windows::core::Abi>::Abi as *const <VpnChannel as ::windows::core::DefaultType>::DefaultType),
                    &*(&encapbuffer as *const <VpnPacketBuffer as ::windows::core::Abi>::Abi as *const <VpnPacketBuffer as ::windows::core::DefaultType>::DefaultType),
                    &*(&decapsulatedpackets as *const <VpnPacketBufferList as ::windows::core::Abi>::Abi as *const <VpnPacketBufferList as ::windows::core::DefaultType>::DefaultType),
                    &*(&controlpacketstosend as *const <VpnPacketBufferList as ::windows::core::Abi>::Abi as *const <VpnPacketBufferList as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnPlugIn, BASE_OFFSET>(),
            Connect: Connect::<Impl, IMPL_OFFSET>,
            Disconnect: Disconnect::<Impl, IMPL_OFFSET>,
            GetKeepAlivePayload: GetKeepAlivePayload::<Impl, IMPL_OFFSET>,
            Encapsulate: Encapsulate::<Impl, IMPL_OFFSET>,
            Decapsulate: Decapsulate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnPlugIn as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IVpnProfile_Impl: Sized {
    fn ProfileName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetProfileName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AppTriggers(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnAppId>>;
    fn Routes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnRoute>>;
    fn DomainNameInfoList(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnDomainNameInfo>>;
    fn TrafficFilters(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnTrafficFilter>>;
    fn RememberCredentials(&mut self) -> ::windows::core::Result<bool>;
    fn SetRememberCredentials(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AlwaysOn(&mut self) -> ::windows::core::Result<bool>;
    fn SetAlwaysOn(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IVpnProfile {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnProfile";
}
#[cfg(feature = "Foundation_Collections")]
impl IVpnProfile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnProfile_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnProfile_Vtbl {
        unsafe extern "system" fn ProfileName<Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProfileName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProfileName<Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProfileName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppTriggers<Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppTriggers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Routes<Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Routes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainNameInfoList<Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DomainNameInfoList() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrafficFilters<Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrafficFilters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RememberCredentials<Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RememberCredentials() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRememberCredentials<Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRememberCredentials(value).into()
        }
        unsafe extern "system" fn AlwaysOn<Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlwaysOn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlwaysOn<Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlwaysOn(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnProfile, BASE_OFFSET>(),
            ProfileName: ProfileName::<Impl, IMPL_OFFSET>,
            SetProfileName: SetProfileName::<Impl, IMPL_OFFSET>,
            AppTriggers: AppTriggers::<Impl, IMPL_OFFSET>,
            Routes: Routes::<Impl, IMPL_OFFSET>,
            DomainNameInfoList: DomainNameInfoList::<Impl, IMPL_OFFSET>,
            TrafficFilters: TrafficFilters::<Impl, IMPL_OFFSET>,
            RememberCredentials: RememberCredentials::<Impl, IMPL_OFFSET>,
            SetRememberCredentials: SetRememberCredentials::<Impl, IMPL_OFFSET>,
            AlwaysOn: AlwaysOn::<Impl, IMPL_OFFSET>,
            SetAlwaysOn: SetAlwaysOn::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnProfile as ::windows::core::Interface>::IID
    }
}
pub trait IVpnRouteFactory_Impl: Sized {
    fn CreateVpnRoute(&mut self, address: &::core::option::Option<super::HostName>, prefixsize: u8) -> ::windows::core::Result<VpnRoute>;
}
impl ::windows::core::RuntimeName for IVpnRouteFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnRouteFactory";
}
impl IVpnRouteFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnRouteFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnRouteFactory_Vtbl {
        unsafe extern "system" fn CreateVpnRoute<Impl: IVpnRouteFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: ::windows::core::RawPtr, prefixsize: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVpnRoute(&*(&address as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType), prefixsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnRouteFactory, BASE_OFFSET>(), CreateVpnRoute: CreateVpnRoute::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnRouteFactory as ::windows::core::Interface>::IID
    }
}
