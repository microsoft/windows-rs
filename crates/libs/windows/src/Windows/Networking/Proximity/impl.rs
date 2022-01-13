#[cfg(feature = "implement_exclusive")]
pub trait IConnectionRequestedEventArgsImpl: Sized {
    fn PeerInformation(&mut self) -> ::windows::core::Result<PeerInformation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConnectionRequestedEventArgs {
    const NAME: &'static str = "Windows.Networking.Proximity.IConnectionRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IConnectionRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectionRequestedEventArgsVtbl {
        unsafe extern "system" fn PeerInformation<Impl: IConnectionRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeerInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConnectionRequestedEventArgs, BASE_OFFSET>(),
            PeerInformation: PeerInformation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectionRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPeerFinderStaticsImpl: Sized {
    fn AllowBluetooth(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowBluetooth(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AllowInfrastructure(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowInfrastructure(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AllowWiFiDirect(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowWiFiDirect(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SupportedDiscoveryTypes(&mut self) -> ::windows::core::Result<PeerDiscoveryTypes>;
    fn AlternateIdentities(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn StartWithMessage(&mut self, peermessage: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn TriggeredConnectionStateChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, TriggeredConnectionStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTriggeredConnectionStateChanged(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ConnectionRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ConnectionRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionRequested(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FindAllPeersAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PeerInformation>>>;
    fn ConnectAsync(&mut self, peerinformation: &::core::option::Option<PeerInformation>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Sockets::StreamSocket>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPeerFinderStatics {
    const NAME: &'static str = "Windows.Networking.Proximity.IPeerFinderStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPeerFinderStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPeerFinderStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPeerFinderStaticsVtbl {
        unsafe extern "system" fn AllowBluetooth<Impl: IPeerFinderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowBluetooth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowBluetooth<Impl: IPeerFinderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowBluetooth(value).into()
        }
        unsafe extern "system" fn AllowInfrastructure<Impl: IPeerFinderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowInfrastructure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInfrastructure<Impl: IPeerFinderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowInfrastructure(value).into()
        }
        unsafe extern "system" fn AllowWiFiDirect<Impl: IPeerFinderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowWiFiDirect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowWiFiDirect<Impl: IPeerFinderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowWiFiDirect(value).into()
        }
        unsafe extern "system" fn DisplayName<Impl: IPeerFinderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDisplayName<Impl: IPeerFinderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SupportedDiscoveryTypes<Impl: IPeerFinderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PeerDiscoveryTypes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedDiscoveryTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlternateIdentities<Impl: IPeerFinderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlternateIdentities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IPeerFinderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn StartWithMessage<Impl: IPeerFinderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peermessage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartWithMessage(&*(&peermessage as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stop<Impl: IPeerFinderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn TriggeredConnectionStateChanged<Impl: IPeerFinderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TriggeredConnectionStateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, TriggeredConnectionStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, TriggeredConnectionStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTriggeredConnectionStateChanged<Impl: IPeerFinderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTriggeredConnectionStateChanged(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConnectionRequested<Impl: IPeerFinderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ConnectionRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ConnectionRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveConnectionRequested<Impl: IPeerFinderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveConnectionRequested(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FindAllPeersAsync<Impl: IPeerFinderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllPeersAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectAsync<Impl: IPeerFinderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peerinformation: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectAsync(&*(&peerinformation as *const <PeerInformation as ::windows::core::Abi>::Abi as *const <PeerInformation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPeerFinderStatics, BASE_OFFSET>(),
            AllowBluetooth: AllowBluetooth::<Impl, IMPL_OFFSET>,
            SetAllowBluetooth: SetAllowBluetooth::<Impl, IMPL_OFFSET>,
            AllowInfrastructure: AllowInfrastructure::<Impl, IMPL_OFFSET>,
            SetAllowInfrastructure: SetAllowInfrastructure::<Impl, IMPL_OFFSET>,
            AllowWiFiDirect: AllowWiFiDirect::<Impl, IMPL_OFFSET>,
            SetAllowWiFiDirect: SetAllowWiFiDirect::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            SupportedDiscoveryTypes: SupportedDiscoveryTypes::<Impl, IMPL_OFFSET>,
            AlternateIdentities: AlternateIdentities::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            StartWithMessage: StartWithMessage::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            TriggeredConnectionStateChanged: TriggeredConnectionStateChanged::<Impl, IMPL_OFFSET>,
            RemoveTriggeredConnectionStateChanged: RemoveTriggeredConnectionStateChanged::<Impl, IMPL_OFFSET>,
            ConnectionRequested: ConnectionRequested::<Impl, IMPL_OFFSET>,
            RemoveConnectionRequested: RemoveConnectionRequested::<Impl, IMPL_OFFSET>,
            FindAllPeersAsync: FindAllPeersAsync::<Impl, IMPL_OFFSET>,
            ConnectAsync: ConnectAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPeerFinderStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IPeerFinderStatics2Impl: Sized {
    fn Role(&mut self) -> ::windows::core::Result<PeerRole>;
    fn SetRole(&mut self, value: PeerRole) -> ::windows::core::Result<()>;
    fn DiscoveryData(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetDiscoveryData(&mut self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn CreateWatcher(&mut self) -> ::windows::core::Result<PeerWatcher>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPeerFinderStatics2 {
    const NAME: &'static str = "Windows.Networking.Proximity.IPeerFinderStatics2";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IPeerFinderStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPeerFinderStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPeerFinderStatics2Vtbl {
        unsafe extern "system" fn Role<Impl: IPeerFinderStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PeerRole) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Role() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRole<Impl: IPeerFinderStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PeerRole) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRole(value).into()
        }
        unsafe extern "system" fn DiscoveryData<Impl: IPeerFinderStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiscoveryData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiscoveryData<Impl: IPeerFinderStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDiscoveryData(&*(&value as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateWatcher<Impl: IPeerFinderStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWatcher() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPeerFinderStatics2, BASE_OFFSET>(),
            Role: Role::<Impl, IMPL_OFFSET>,
            SetRole: SetRole::<Impl, IMPL_OFFSET>,
            DiscoveryData: DiscoveryData::<Impl, IMPL_OFFSET>,
            SetDiscoveryData: SetDiscoveryData::<Impl, IMPL_OFFSET>,
            CreateWatcher: CreateWatcher::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPeerFinderStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPeerInformationImpl: Sized {
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPeerInformation {
    const NAME: &'static str = "Windows.Networking.Proximity.IPeerInformation";
}
#[cfg(feature = "implement_exclusive")]
impl IPeerInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPeerInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPeerInformationVtbl {
        unsafe extern "system" fn DisplayName<Impl: IPeerInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPeerInformation, BASE_OFFSET>(), DisplayName: DisplayName::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPeerInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IPeerInformation3Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DiscoveryData(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPeerInformation3 {
    const NAME: &'static str = "Windows.Networking.Proximity.IPeerInformation3";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IPeerInformation3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPeerInformation3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPeerInformation3Vtbl {
        unsafe extern "system" fn Id<Impl: IPeerInformation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscoveryData<Impl: IPeerInformation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiscoveryData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPeerInformation3, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            DiscoveryData: DiscoveryData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPeerInformation3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPeerInformationWithHostAndServiceImpl: Sized {
    fn HostName(&mut self) -> ::windows::core::Result<super::HostName>;
    fn ServiceName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPeerInformationWithHostAndService {
    const NAME: &'static str = "Windows.Networking.Proximity.IPeerInformationWithHostAndService";
}
#[cfg(feature = "implement_exclusive")]
impl IPeerInformationWithHostAndServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPeerInformationWithHostAndServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPeerInformationWithHostAndServiceVtbl {
        unsafe extern "system" fn HostName<Impl: IPeerInformationWithHostAndServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HostName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceName<Impl: IPeerInformationWithHostAndServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPeerInformationWithHostAndService, BASE_OFFSET>(),
            HostName: HostName::<Impl, IMPL_OFFSET>,
            ServiceName: ServiceName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPeerInformationWithHostAndService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPeerWatcherImpl: Sized {
    fn Added(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PeerWatcher, PeerInformation>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PeerWatcher, PeerInformation>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Updated(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PeerWatcher, PeerInformation>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PeerWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PeerWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&mut self) -> ::windows::core::Result<PeerWatcherStatus>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPeerWatcher {
    const NAME: &'static str = "Windows.Networking.Proximity.IPeerWatcher";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPeerWatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPeerWatcherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPeerWatcherVtbl {
        unsafe extern "system" fn Added<Impl: IPeerWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Added(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PeerWatcher, PeerInformation> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PeerWatcher, PeerInformation> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAdded<Impl: IPeerWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Removed<Impl: IPeerWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Removed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PeerWatcher, PeerInformation> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PeerWatcher, PeerInformation> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRemoved<Impl: IPeerWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Updated<Impl: IPeerWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Updated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PeerWatcher, PeerInformation> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PeerWatcher, PeerInformation> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUpdated<Impl: IPeerWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUpdated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnumerationCompleted<Impl: IPeerWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerationCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PeerWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PeerWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnumerationCompleted<Impl: IPeerWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnumerationCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stopped<Impl: IPeerWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stopped(&*(&handler as *const <super::super::Foundation::TypedEventHandler<PeerWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<PeerWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStopped<Impl: IPeerWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStopped(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Status<Impl: IPeerWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PeerWatcherStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IPeerWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IPeerWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPeerWatcher, BASE_OFFSET>(),
            Added: Added::<Impl, IMPL_OFFSET>,
            RemoveAdded: RemoveAdded::<Impl, IMPL_OFFSET>,
            Removed: Removed::<Impl, IMPL_OFFSET>,
            RemoveRemoved: RemoveRemoved::<Impl, IMPL_OFFSET>,
            Updated: Updated::<Impl, IMPL_OFFSET>,
            RemoveUpdated: RemoveUpdated::<Impl, IMPL_OFFSET>,
            EnumerationCompleted: EnumerationCompleted::<Impl, IMPL_OFFSET>,
            RemoveEnumerationCompleted: RemoveEnumerationCompleted::<Impl, IMPL_OFFSET>,
            Stopped: Stopped::<Impl, IMPL_OFFSET>,
            RemoveStopped: RemoveStopped::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPeerWatcher as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IProximityDeviceImpl: Sized {
    fn SubscribeForMessage(&mut self, messagetype: &::windows::core::HSTRING, messagereceivedhandler: &::core::option::Option<MessageReceivedHandler>) -> ::windows::core::Result<i64>;
    fn PublishMessage(&mut self, messagetype: &::windows::core::HSTRING, message: &::windows::core::HSTRING) -> ::windows::core::Result<i64>;
    fn PublishMessageWithCallback(&mut self, messagetype: &::windows::core::HSTRING, message: &::windows::core::HSTRING, messagetransmittedhandler: &::core::option::Option<MessageTransmittedHandler>) -> ::windows::core::Result<i64>;
    fn PublishBinaryMessage(&mut self, messagetype: &::windows::core::HSTRING, message: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<i64>;
    fn PublishBinaryMessageWithCallback(&mut self, messagetype: &::windows::core::HSTRING, message: &::core::option::Option<super::super::Storage::Streams::IBuffer>, messagetransmittedhandler: &::core::option::Option<MessageTransmittedHandler>) -> ::windows::core::Result<i64>;
    fn PublishUriMessage(&mut self, message: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<i64>;
    fn PublishUriMessageWithCallback(&mut self, message: &::core::option::Option<super::super::Foundation::Uri>, messagetransmittedhandler: &::core::option::Option<MessageTransmittedHandler>) -> ::windows::core::Result<i64>;
    fn StopSubscribingForMessage(&mut self, subscriptionid: i64) -> ::windows::core::Result<()>;
    fn StopPublishingMessage(&mut self, messageid: i64) -> ::windows::core::Result<()>;
    fn DeviceArrived(&mut self, arrivedhandler: &::core::option::Option<DeviceArrivedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDeviceArrived(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DeviceDeparted(&mut self, departedhandler: &::core::option::Option<DeviceDepartedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDeviceDeparted(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MaxMessageBytes(&mut self) -> ::windows::core::Result<u32>;
    fn BitsPerSecond(&mut self) -> ::windows::core::Result<u64>;
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProximityDevice {
    const NAME: &'static str = "Windows.Networking.Proximity.IProximityDevice";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IProximityDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProximityDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProximityDeviceVtbl {
        unsafe extern "system" fn SubscribeForMessage<Impl: IProximityDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, messagereceivedhandler: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubscribeForMessage(&*(&messagetype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&messagereceivedhandler as *const <MessageReceivedHandler as ::windows::core::Abi>::Abi as *const <MessageReceivedHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublishMessage<Impl: IProximityDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublishMessage(&*(&messagetype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&message as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublishMessageWithCallback<Impl: IProximityDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, messagetransmittedhandler: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublishMessageWithCallback(
                &*(&messagetype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&message as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&messagetransmittedhandler as *const <MessageTransmittedHandler as ::windows::core::Abi>::Abi as *const <MessageTransmittedHandler as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublishBinaryMessage<Impl: IProximityDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublishBinaryMessage(&*(&messagetype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&message as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublishBinaryMessageWithCallback<Impl: IProximityDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: ::windows::core::RawPtr, messagetransmittedhandler: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublishBinaryMessageWithCallback(
                &*(&messagetype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&message as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&messagetransmittedhandler as *const <MessageTransmittedHandler as ::windows::core::Abi>::Abi as *const <MessageTransmittedHandler as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublishUriMessage<Impl: IProximityDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublishUriMessage(&*(&message as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublishUriMessageWithCallback<Impl: IProximityDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, messagetransmittedhandler: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublishUriMessageWithCallback(&*(&message as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&messagetransmittedhandler as *const <MessageTransmittedHandler as ::windows::core::Abi>::Abi as *const <MessageTransmittedHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopSubscribingForMessage<Impl: IProximityDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subscriptionid: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopSubscribingForMessage(subscriptionid).into()
        }
        unsafe extern "system" fn StopPublishingMessage<Impl: IProximityDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messageid: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopPublishingMessage(messageid).into()
        }
        unsafe extern "system" fn DeviceArrived<Impl: IProximityDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, arrivedhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceArrived(&*(&arrivedhandler as *const <DeviceArrivedEventHandler as ::windows::core::Abi>::Abi as *const <DeviceArrivedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDeviceArrived<Impl: IProximityDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDeviceArrived(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeviceDeparted<Impl: IProximityDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, departedhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceDeparted(&*(&departedhandler as *const <DeviceDepartedEventHandler as ::windows::core::Abi>::Abi as *const <DeviceDepartedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDeviceDeparted<Impl: IProximityDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDeviceDeparted(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxMessageBytes<Impl: IProximityDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxMessageBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitsPerSecond<Impl: IProximityDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BitsPerSecond() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Impl: IProximityDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProximityDevice, BASE_OFFSET>(),
            SubscribeForMessage: SubscribeForMessage::<Impl, IMPL_OFFSET>,
            PublishMessage: PublishMessage::<Impl, IMPL_OFFSET>,
            PublishMessageWithCallback: PublishMessageWithCallback::<Impl, IMPL_OFFSET>,
            PublishBinaryMessage: PublishBinaryMessage::<Impl, IMPL_OFFSET>,
            PublishBinaryMessageWithCallback: PublishBinaryMessageWithCallback::<Impl, IMPL_OFFSET>,
            PublishUriMessage: PublishUriMessage::<Impl, IMPL_OFFSET>,
            PublishUriMessageWithCallback: PublishUriMessageWithCallback::<Impl, IMPL_OFFSET>,
            StopSubscribingForMessage: StopSubscribingForMessage::<Impl, IMPL_OFFSET>,
            StopPublishingMessage: StopPublishingMessage::<Impl, IMPL_OFFSET>,
            DeviceArrived: DeviceArrived::<Impl, IMPL_OFFSET>,
            RemoveDeviceArrived: RemoveDeviceArrived::<Impl, IMPL_OFFSET>,
            DeviceDeparted: DeviceDeparted::<Impl, IMPL_OFFSET>,
            RemoveDeviceDeparted: RemoveDeviceDeparted::<Impl, IMPL_OFFSET>,
            MaxMessageBytes: MaxMessageBytes::<Impl, IMPL_OFFSET>,
            BitsPerSecond: BitsPerSecond::<Impl, IMPL_OFFSET>,
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProximityDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProximityDeviceStaticsImpl: Sized {
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDefault(&mut self) -> ::windows::core::Result<ProximityDevice>;
    fn FromId(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<ProximityDevice>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProximityDeviceStatics {
    const NAME: &'static str = "Windows.Networking.Proximity.IProximityDeviceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IProximityDeviceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProximityDeviceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProximityDeviceStaticsVtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IProximityDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefault<Impl: IProximityDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromId<Impl: IProximityDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromId(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProximityDeviceStatics, BASE_OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
            FromId: FromId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProximityDeviceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IProximityMessageImpl: Sized {
    fn MessageType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SubscriptionId(&mut self) -> ::windows::core::Result<i64>;
    fn Data(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn DataAsString(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProximityMessage {
    const NAME: &'static str = "Windows.Networking.Proximity.IProximityMessage";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IProximityMessageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProximityMessageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProximityMessageVtbl {
        unsafe extern "system" fn MessageType<Impl: IProximityMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubscriptionId<Impl: IProximityMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubscriptionId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Impl: IProximityMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataAsString<Impl: IProximityMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataAsString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProximityMessage, BASE_OFFSET>(),
            MessageType: MessageType::<Impl, IMPL_OFFSET>,
            SubscriptionId: SubscriptionId::<Impl, IMPL_OFFSET>,
            Data: Data::<Impl, IMPL_OFFSET>,
            DataAsString: DataAsString::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProximityMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Networking_Sockets", feature = "implement_exclusive"))]
pub trait ITriggeredConnectionStateChangedEventArgsImpl: Sized {
    fn State(&mut self) -> ::windows::core::Result<TriggeredConnectState>;
    fn Id(&mut self) -> ::windows::core::Result<u32>;
    fn Socket(&mut self) -> ::windows::core::Result<super::Sockets::StreamSocket>;
}
#[cfg(all(feature = "Networking_Sockets", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITriggeredConnectionStateChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.Proximity.ITriggeredConnectionStateChangedEventArgs";
}
#[cfg(all(feature = "Networking_Sockets", feature = "implement_exclusive"))]
impl ITriggeredConnectionStateChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITriggeredConnectionStateChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITriggeredConnectionStateChangedEventArgsVtbl {
        unsafe extern "system" fn State<Impl: ITriggeredConnectionStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TriggeredConnectState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: ITriggeredConnectionStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Socket<Impl: ITriggeredConnectionStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Socket() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITriggeredConnectionStateChangedEventArgs, BASE_OFFSET>(),
            State: State::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            Socket: Socket::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITriggeredConnectionStateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
