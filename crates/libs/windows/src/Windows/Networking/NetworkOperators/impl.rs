#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IESim_Impl: Sized {
    fn AvailableMemoryInBytes(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn Eid(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FirmwareVersion(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MobileBroadbandModemDeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Policy(&mut self) -> ::windows::core::Result<ESimPolicy>;
    fn State(&mut self) -> ::windows::core::Result<ESimState>;
    fn GetProfiles(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ESimProfile>>;
    fn DeleteProfileAsync(&mut self, profileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>>;
    fn DownloadProfileMetadataAsync(&mut self, activationcode: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimDownloadProfileMetadataResult>>;
    fn ResetAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>>;
    fn ProfileChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ESim, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProfileChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IESim {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESim";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IESim_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESim_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESim_Vtbl {
        unsafe extern "system" fn AvailableMemoryInBytes<Impl: IESim_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailableMemoryInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Eid<Impl: IESim_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Eid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirmwareVersion<Impl: IESim_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirmwareVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MobileBroadbandModemDeviceId<Impl: IESim_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MobileBroadbandModemDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Policy<Impl: IESim_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Policy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IESim_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ESimState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetProfiles<Impl: IESim_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProfiles() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteProfileAsync<Impl: IESim_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteProfileAsync(&*(&profileid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadProfileMetadataAsync<Impl: IESim_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activationcode: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadProfileMetadataAsync(&*(&activationcode as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetAsync<Impl: IESim_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResetAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProfileChanged<Impl: IESim_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProfileChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ESim, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ESim, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveProfileChanged<Impl: IESim_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveProfileChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IESim, BASE_OFFSET>(),
            AvailableMemoryInBytes: AvailableMemoryInBytes::<Impl, IMPL_OFFSET>,
            Eid: Eid::<Impl, IMPL_OFFSET>,
            FirmwareVersion: FirmwareVersion::<Impl, IMPL_OFFSET>,
            MobileBroadbandModemDeviceId: MobileBroadbandModemDeviceId::<Impl, IMPL_OFFSET>,
            Policy: Policy::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            GetProfiles: GetProfiles::<Impl, IMPL_OFFSET>,
            DeleteProfileAsync: DeleteProfileAsync::<Impl, IMPL_OFFSET>,
            DownloadProfileMetadataAsync: DownloadProfileMetadataAsync::<Impl, IMPL_OFFSET>,
            ResetAsync: ResetAsync::<Impl, IMPL_OFFSET>,
            ProfileChanged: ProfileChanged::<Impl, IMPL_OFFSET>,
            RemoveProfileChanged: RemoveProfileChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESim as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IESim2_Impl: Sized {
    fn Discover(&mut self) -> ::windows::core::Result<ESimDiscoverResult>;
    fn DiscoverWithServerAddressAndMatchingId(&mut self, serveraddress: &::windows::core::HSTRING, matchingid: &::windows::core::HSTRING) -> ::windows::core::Result<ESimDiscoverResult>;
    fn DiscoverAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimDiscoverResult>>;
    fn DiscoverWithServerAddressAndMatchingIdAsync(&mut self, serveraddress: &::windows::core::HSTRING, matchingid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimDiscoverResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IESim2 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESim2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IESim2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESim2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESim2_Vtbl {
        unsafe extern "system" fn Discover<Impl: IESim2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Discover() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscoverWithServerAddressAndMatchingId<Impl: IESim2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serveraddress: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, matchingid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiscoverWithServerAddressAndMatchingId(&*(&serveraddress as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&matchingid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscoverAsync<Impl: IESim2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiscoverAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscoverWithServerAddressAndMatchingIdAsync<Impl: IESim2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serveraddress: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, matchingid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiscoverWithServerAddressAndMatchingIdAsync(&*(&serveraddress as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&matchingid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IESim2, BASE_OFFSET>(),
            Discover: Discover::<Impl, IMPL_OFFSET>,
            DiscoverWithServerAddressAndMatchingId: DiscoverWithServerAddressAndMatchingId::<Impl, IMPL_OFFSET>,
            DiscoverAsync: DiscoverAsync::<Impl, IMPL_OFFSET>,
            DiscoverWithServerAddressAndMatchingIdAsync: DiscoverWithServerAddressAndMatchingIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESim2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimAddedEventArgs_Impl: Sized {
    fn ESim(&mut self) -> ::windows::core::Result<ESim>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IESimAddedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimAddedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IESimAddedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimAddedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimAddedEventArgs_Vtbl {
        unsafe extern "system" fn ESim<Impl: IESimAddedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ESim() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IESimAddedEventArgs, BASE_OFFSET>(), ESim: ESim::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimAddedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimDiscoverEvent_Impl: Sized {
    fn MatchingId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RspServerAddress(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IESimDiscoverEvent {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimDiscoverEvent";
}
#[cfg(feature = "implement_exclusive")]
impl IESimDiscoverEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimDiscoverEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimDiscoverEvent_Vtbl {
        unsafe extern "system" fn MatchingId<Impl: IESimDiscoverEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MatchingId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RspServerAddress<Impl: IESimDiscoverEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RspServerAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IESimDiscoverEvent, BASE_OFFSET>(),
            MatchingId: MatchingId::<Impl, IMPL_OFFSET>,
            RspServerAddress: RspServerAddress::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimDiscoverEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IESimDiscoverResult_Impl: Sized {
    fn Events(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ESimDiscoverEvent>>;
    fn Kind(&mut self) -> ::windows::core::Result<ESimDiscoverResultKind>;
    fn ProfileMetadata(&mut self) -> ::windows::core::Result<ESimProfileMetadata>;
    fn Result(&mut self) -> ::windows::core::Result<ESimOperationResult>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IESimDiscoverResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimDiscoverResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IESimDiscoverResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimDiscoverResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimDiscoverResult_Vtbl {
        unsafe extern "system" fn Events<Impl: IESimDiscoverResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Events() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Kind<Impl: IESimDiscoverResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ESimDiscoverResultKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProfileMetadata<Impl: IESimDiscoverResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProfileMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Result<Impl: IESimDiscoverResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Result() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IESimDiscoverResult, BASE_OFFSET>(),
            Events: Events::<Impl, IMPL_OFFSET>,
            Kind: Kind::<Impl, IMPL_OFFSET>,
            ProfileMetadata: ProfileMetadata::<Impl, IMPL_OFFSET>,
            Result: Result::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimDiscoverResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimDownloadProfileMetadataResult_Impl: Sized {
    fn Result(&mut self) -> ::windows::core::Result<ESimOperationResult>;
    fn ProfileMetadata(&mut self) -> ::windows::core::Result<ESimProfileMetadata>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IESimDownloadProfileMetadataResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimDownloadProfileMetadataResult";
}
#[cfg(feature = "implement_exclusive")]
impl IESimDownloadProfileMetadataResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimDownloadProfileMetadataResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimDownloadProfileMetadataResult_Vtbl {
        unsafe extern "system" fn Result<Impl: IESimDownloadProfileMetadataResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Result() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProfileMetadata<Impl: IESimDownloadProfileMetadataResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProfileMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IESimDownloadProfileMetadataResult, BASE_OFFSET>(),
            Result: Result::<Impl, IMPL_OFFSET>,
            ProfileMetadata: ProfileMetadata::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimDownloadProfileMetadataResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IESimManagerStatics_Impl: Sized {
    fn ServiceInfo(&mut self) -> ::windows::core::Result<ESimServiceInfo>;
    fn TryCreateESimWatcher(&mut self) -> ::windows::core::Result<ESimWatcher>;
    fn ServiceInfoChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveServiceInfoChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IESimManagerStatics {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IESimManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimManagerStatics_Vtbl {
        unsafe extern "system" fn ServiceInfo<Impl: IESimManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryCreateESimWatcher<Impl: IESimManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCreateESimWatcher() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceInfoChanged<Impl: IESimManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceInfoChanged(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveServiceInfoChanged<Impl: IESimManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveServiceInfoChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IESimManagerStatics, BASE_OFFSET>(),
            ServiceInfo: ServiceInfo::<Impl, IMPL_OFFSET>,
            TryCreateESimWatcher: TryCreateESimWatcher::<Impl, IMPL_OFFSET>,
            ServiceInfoChanged: ServiceInfoChanged::<Impl, IMPL_OFFSET>,
            RemoveServiceInfoChanged: RemoveServiceInfoChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimOperationResult_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<ESimOperationStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IESimOperationResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimOperationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IESimOperationResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimOperationResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimOperationResult_Vtbl {
        unsafe extern "system" fn Status<Impl: IESimOperationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ESimOperationStatus) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IESimOperationResult, BASE_OFFSET>(), Status: Status::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimOperationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimPolicy_Impl: Sized {
    fn ShouldEnableManagingUi(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IESimPolicy {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimPolicy";
}
#[cfg(feature = "implement_exclusive")]
impl IESimPolicy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimPolicy_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimPolicy_Vtbl {
        unsafe extern "system" fn ShouldEnableManagingUi<Impl: IESimPolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShouldEnableManagingUi() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IESimPolicy, BASE_OFFSET>(),
            ShouldEnableManagingUi: ShouldEnableManagingUi::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimPolicy as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IESimProfile_Impl: Sized {
    fn Class(&mut self) -> ::windows::core::Result<ESimProfileClass>;
    fn Nickname(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Policy(&mut self) -> ::windows::core::Result<ESimProfilePolicy>;
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProviderIcon(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn ProviderId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProviderName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn State(&mut self) -> ::windows::core::Result<ESimProfileState>;
    fn DisableAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>>;
    fn EnableAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>>;
    fn SetNicknameAsync(&mut self, newnickname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IESimProfile {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimProfile";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IESimProfile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimProfile_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimProfile_Vtbl {
        unsafe extern "system" fn Class<Impl: IESimProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ESimProfileClass) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Class() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Nickname<Impl: IESimProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Nickname() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Policy<Impl: IESimProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Policy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IESimProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProviderIcon<Impl: IESimProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderIcon() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderId<Impl: IESimProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderName<Impl: IESimProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IESimProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ESimProfileState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisableAsync<Impl: IESimProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableAsync<Impl: IESimProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNicknameAsync<Impl: IESimProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newnickname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNicknameAsync(&*(&newnickname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IESimProfile, BASE_OFFSET>(),
            Class: Class::<Impl, IMPL_OFFSET>,
            Nickname: Nickname::<Impl, IMPL_OFFSET>,
            Policy: Policy::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            ProviderIcon: ProviderIcon::<Impl, IMPL_OFFSET>,
            ProviderId: ProviderId::<Impl, IMPL_OFFSET>,
            ProviderName: ProviderName::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            DisableAsync: DisableAsync::<Impl, IMPL_OFFSET>,
            EnableAsync: EnableAsync::<Impl, IMPL_OFFSET>,
            SetNicknameAsync: SetNicknameAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimProfile as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IESimProfileMetadata_Impl: Sized {
    fn IsConfirmationCodeRequired(&mut self) -> ::windows::core::Result<bool>;
    fn Policy(&mut self) -> ::windows::core::Result<ESimProfilePolicy>;
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProviderIcon(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn ProviderId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProviderName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn State(&mut self) -> ::windows::core::Result<ESimProfileMetadataState>;
    fn DenyInstallAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>>;
    fn ConfirmInstallAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<ESimOperationResult, ESimProfileInstallProgress>>;
    fn ConfirmInstallWithConfirmationCodeAsync(&mut self, confirmationcode: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<ESimOperationResult, ESimProfileInstallProgress>>;
    fn PostponeInstallAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>>;
    fn StateChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ESimProfileMetadata, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IESimProfileMetadata {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimProfileMetadata";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IESimProfileMetadata_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimProfileMetadata_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimProfileMetadata_Vtbl {
        unsafe extern "system" fn IsConfirmationCodeRequired<Impl: IESimProfileMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConfirmationCodeRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Policy<Impl: IESimProfileMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Policy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IESimProfileMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProviderIcon<Impl: IESimProfileMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderIcon() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderId<Impl: IESimProfileMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderName<Impl: IESimProfileMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IESimProfileMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ESimProfileMetadataState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DenyInstallAsync<Impl: IESimProfileMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DenyInstallAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfirmInstallAsync<Impl: IESimProfileMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConfirmInstallAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfirmInstallWithConfirmationCodeAsync<Impl: IESimProfileMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confirmationcode: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConfirmInstallWithConfirmationCodeAsync(&*(&confirmationcode as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostponeInstallAsync<Impl: IESimProfileMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostponeInstallAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StateChanged<Impl: IESimProfileMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ESimProfileMetadata, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ESimProfileMetadata, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStateChanged<Impl: IESimProfileMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IESimProfileMetadata, BASE_OFFSET>(),
            IsConfirmationCodeRequired: IsConfirmationCodeRequired::<Impl, IMPL_OFFSET>,
            Policy: Policy::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            ProviderIcon: ProviderIcon::<Impl, IMPL_OFFSET>,
            ProviderId: ProviderId::<Impl, IMPL_OFFSET>,
            ProviderName: ProviderName::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            DenyInstallAsync: DenyInstallAsync::<Impl, IMPL_OFFSET>,
            ConfirmInstallAsync: ConfirmInstallAsync::<Impl, IMPL_OFFSET>,
            ConfirmInstallWithConfirmationCodeAsync: ConfirmInstallWithConfirmationCodeAsync::<Impl, IMPL_OFFSET>,
            PostponeInstallAsync: PostponeInstallAsync::<Impl, IMPL_OFFSET>,
            StateChanged: StateChanged::<Impl, IMPL_OFFSET>,
            RemoveStateChanged: RemoveStateChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimProfileMetadata as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimProfilePolicy_Impl: Sized {
    fn CanDelete(&mut self) -> ::windows::core::Result<bool>;
    fn CanDisable(&mut self) -> ::windows::core::Result<bool>;
    fn IsManagedByEnterprise(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IESimProfilePolicy {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimProfilePolicy";
}
#[cfg(feature = "implement_exclusive")]
impl IESimProfilePolicy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimProfilePolicy_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimProfilePolicy_Vtbl {
        unsafe extern "system" fn CanDelete<Impl: IESimProfilePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanDelete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanDisable<Impl: IESimProfilePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanDisable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsManagedByEnterprise<Impl: IESimProfilePolicy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsManagedByEnterprise() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IESimProfilePolicy, BASE_OFFSET>(),
            CanDelete: CanDelete::<Impl, IMPL_OFFSET>,
            CanDisable: CanDisable::<Impl, IMPL_OFFSET>,
            IsManagedByEnterprise: IsManagedByEnterprise::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimProfilePolicy as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimRemovedEventArgs_Impl: Sized {
    fn ESim(&mut self) -> ::windows::core::Result<ESim>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IESimRemovedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimRemovedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IESimRemovedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimRemovedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimRemovedEventArgs_Vtbl {
        unsafe extern "system" fn ESim<Impl: IESimRemovedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ESim() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IESimRemovedEventArgs, BASE_OFFSET>(), ESim: ESim::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimRemovedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimServiceInfo_Impl: Sized {
    fn AuthenticationPreference(&mut self) -> ::windows::core::Result<ESimAuthenticationPreference>;
    fn IsESimUiEnabled(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IESimServiceInfo {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimServiceInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IESimServiceInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimServiceInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimServiceInfo_Vtbl {
        unsafe extern "system" fn AuthenticationPreference<Impl: IESimServiceInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ESimAuthenticationPreference) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationPreference() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsESimUiEnabled<Impl: IESimServiceInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsESimUiEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IESimServiceInfo, BASE_OFFSET>(),
            AuthenticationPreference: AuthenticationPreference::<Impl, IMPL_OFFSET>,
            IsESimUiEnabled: IsESimUiEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimServiceInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimUpdatedEventArgs_Impl: Sized {
    fn ESim(&mut self) -> ::windows::core::Result<ESim>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IESimUpdatedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimUpdatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IESimUpdatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimUpdatedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimUpdatedEventArgs_Vtbl {
        unsafe extern "system" fn ESim<Impl: IESimUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ESim() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IESimUpdatedEventArgs, BASE_OFFSET>(), ESim: ESim::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimUpdatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IESimWatcher_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<ESimWatcherStatus>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn Added(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ESimWatcher, ESimAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ESimWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ESimWatcher, ESimRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ESimWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Updated(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ESimWatcher, ESimUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IESimWatcher {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimWatcher";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IESimWatcher_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimWatcher_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimWatcher_Vtbl {
        unsafe extern "system" fn Status<Impl: IESimWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ESimWatcherStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Start<Impl: IESimWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IESimWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Added<Impl: IESimWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Added(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ESimWatcher, ESimAddedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ESimWatcher, ESimAddedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAdded<Impl: IESimWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnumerationCompleted<Impl: IESimWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerationCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ESimWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ESimWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnumerationCompleted<Impl: IESimWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnumerationCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Removed<Impl: IESimWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Removed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ESimWatcher, ESimRemovedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ESimWatcher, ESimRemovedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRemoved<Impl: IESimWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stopped<Impl: IESimWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stopped(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ESimWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ESimWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStopped<Impl: IESimWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStopped(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Updated<Impl: IESimWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Updated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ESimWatcher, ESimUpdatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ESimWatcher, ESimUpdatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUpdated<Impl: IESimWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUpdated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IESimWatcher, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            Added: Added::<Impl, IMPL_OFFSET>,
            RemoveAdded: RemoveAdded::<Impl, IMPL_OFFSET>,
            EnumerationCompleted: EnumerationCompleted::<Impl, IMPL_OFFSET>,
            RemoveEnumerationCompleted: RemoveEnumerationCompleted::<Impl, IMPL_OFFSET>,
            Removed: Removed::<Impl, IMPL_OFFSET>,
            RemoveRemoved: RemoveRemoved::<Impl, IMPL_OFFSET>,
            Stopped: Stopped::<Impl, IMPL_OFFSET>,
            RemoveStopped: RemoveStopped::<Impl, IMPL_OFFSET>,
            Updated: Updated::<Impl, IMPL_OFFSET>,
            RemoveUpdated: RemoveUpdated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimWatcher as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IFdnAccessManagerStatics_Impl: Sized {
    fn RequestUnlockAsync(&mut self, contactlistid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFdnAccessManagerStatics {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IFdnAccessManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IFdnAccessManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFdnAccessManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFdnAccessManagerStatics_Vtbl {
        unsafe extern "system" fn RequestUnlockAsync<Impl: IFdnAccessManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contactlistid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestUnlockAsync(&*(&contactlistid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFdnAccessManagerStatics, BASE_OFFSET>(),
            RequestUnlockAsync: RequestUnlockAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFdnAccessManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
pub trait IHotspotAuthenticationContext_Impl: Sized {
    fn WirelessNetworkId(&mut self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn NetworkAdapter(&mut self) -> ::windows::core::Result<super::Connectivity::NetworkAdapter>;
    fn RedirectMessageUrl(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn RedirectMessageXml(&mut self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
    fn AuthenticationUrl(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn IssueCredentials(&mut self, username: &::windows::core::HSTRING, password: &::windows::core::HSTRING, extraparameters: &::windows::core::HSTRING, markasmanualconnectonfailure: bool) -> ::windows::core::Result<()>;
    fn AbortAuthentication(&mut self, markasmanual: bool) -> ::windows::core::Result<()>;
    fn SkipAuthentication(&mut self) -> ::windows::core::Result<()>;
    fn TriggerAttentionRequired(&mut self, packagerelativeapplicationid: &::windows::core::HSTRING, applicationparameters: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHotspotAuthenticationContext {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IHotspotAuthenticationContext";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl IHotspotAuthenticationContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHotspotAuthenticationContext_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHotspotAuthenticationContext_Vtbl {
        unsafe extern "system" fn WirelessNetworkId<Impl: IHotspotAuthenticationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WirelessNetworkId() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkAdapter<Impl: IHotspotAuthenticationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkAdapter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectMessageUrl<Impl: IHotspotAuthenticationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedirectMessageUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectMessageXml<Impl: IHotspotAuthenticationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedirectMessageXml() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticationUrl<Impl: IHotspotAuthenticationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IssueCredentials<Impl: IHotspotAuthenticationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, extraparameters: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, markasmanualconnectonfailure: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .IssueCredentials(
                    &*(&username as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&password as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&extraparameters as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    markasmanualconnectonfailure,
                )
                .into()
        }
        unsafe extern "system" fn AbortAuthentication<Impl: IHotspotAuthenticationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, markasmanual: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AbortAuthentication(markasmanual).into()
        }
        unsafe extern "system" fn SkipAuthentication<Impl: IHotspotAuthenticationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SkipAuthentication().into()
        }
        unsafe extern "system" fn TriggerAttentionRequired<Impl: IHotspotAuthenticationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagerelativeapplicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, applicationparameters: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TriggerAttentionRequired(&*(&packagerelativeapplicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&applicationparameters as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHotspotAuthenticationContext, BASE_OFFSET>(),
            WirelessNetworkId: WirelessNetworkId::<Impl, IMPL_OFFSET>,
            NetworkAdapter: NetworkAdapter::<Impl, IMPL_OFFSET>,
            RedirectMessageUrl: RedirectMessageUrl::<Impl, IMPL_OFFSET>,
            RedirectMessageXml: RedirectMessageXml::<Impl, IMPL_OFFSET>,
            AuthenticationUrl: AuthenticationUrl::<Impl, IMPL_OFFSET>,
            IssueCredentials: IssueCredentials::<Impl, IMPL_OFFSET>,
            AbortAuthentication: AbortAuthentication::<Impl, IMPL_OFFSET>,
            SkipAuthentication: SkipAuthentication::<Impl, IMPL_OFFSET>,
            TriggerAttentionRequired: TriggerAttentionRequired::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHotspotAuthenticationContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHotspotAuthenticationContext2_Impl: Sized {
    fn IssueCredentialsAsync(&mut self, username: &::windows::core::HSTRING, password: &::windows::core::HSTRING, extraparameters: &::windows::core::HSTRING, markasmanualconnectonfailure: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HotspotCredentialsAuthenticationResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHotspotAuthenticationContext2 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IHotspotAuthenticationContext2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHotspotAuthenticationContext2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHotspotAuthenticationContext2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHotspotAuthenticationContext2_Vtbl {
        unsafe extern "system" fn IssueCredentialsAsync<Impl: IHotspotAuthenticationContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, extraparameters: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, markasmanualconnectonfailure: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IssueCredentialsAsync(
                &*(&username as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&password as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&extraparameters as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                markasmanualconnectonfailure,
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHotspotAuthenticationContext2, BASE_OFFSET>(),
            IssueCredentialsAsync: IssueCredentialsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHotspotAuthenticationContext2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHotspotAuthenticationContextStatics_Impl: Sized {
    fn TryGetAuthenticationContext(&mut self, eventoken: &::windows::core::HSTRING, context: &mut ::core::option::Option<HotspotAuthenticationContext>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHotspotAuthenticationContextStatics {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IHotspotAuthenticationContextStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHotspotAuthenticationContextStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHotspotAuthenticationContextStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHotspotAuthenticationContextStatics_Vtbl {
        unsafe extern "system" fn TryGetAuthenticationContext<Impl: IHotspotAuthenticationContextStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventoken: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, context: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetAuthenticationContext(&*(&eventoken as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHotspotAuthenticationContextStatics, BASE_OFFSET>(),
            TryGetAuthenticationContext: TryGetAuthenticationContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHotspotAuthenticationContextStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHotspotAuthenticationEventDetails_Impl: Sized {
    fn EventToken(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHotspotAuthenticationEventDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IHotspotAuthenticationEventDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IHotspotAuthenticationEventDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHotspotAuthenticationEventDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHotspotAuthenticationEventDetails_Vtbl {
        unsafe extern "system" fn EventToken<Impl: IHotspotAuthenticationEventDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventToken() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHotspotAuthenticationEventDetails, BASE_OFFSET>(),
            EventToken: EventToken::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHotspotAuthenticationEventDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHotspotCredentialsAuthenticationResult_Impl: Sized {
    fn HasNetworkErrorOccurred(&mut self) -> ::windows::core::Result<bool>;
    fn ResponseCode(&mut self) -> ::windows::core::Result<HotspotAuthenticationResponseCode>;
    fn LogoffUrl(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn AuthenticationReplyXml(&mut self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHotspotCredentialsAuthenticationResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IHotspotCredentialsAuthenticationResult";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
impl IHotspotCredentialsAuthenticationResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHotspotCredentialsAuthenticationResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHotspotCredentialsAuthenticationResult_Vtbl {
        unsafe extern "system" fn HasNetworkErrorOccurred<Impl: IHotspotCredentialsAuthenticationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasNetworkErrorOccurred() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseCode<Impl: IHotspotCredentialsAuthenticationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HotspotAuthenticationResponseCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogoffUrl<Impl: IHotspotCredentialsAuthenticationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LogoffUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticationReplyXml<Impl: IHotspotCredentialsAuthenticationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationReplyXml() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHotspotCredentialsAuthenticationResult, BASE_OFFSET>(),
            HasNetworkErrorOccurred: HasNetworkErrorOccurred::<Impl, IMPL_OFFSET>,
            ResponseCode: ResponseCode::<Impl, IMPL_OFFSET>,
            LogoffUrl: LogoffUrl::<Impl, IMPL_OFFSET>,
            AuthenticationReplyXml: AuthenticationReplyXml::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHotspotCredentialsAuthenticationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IKnownCSimFilePathsStatics_Impl: Sized {
    fn EFSpn(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn Gid1(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn Gid2(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKnownCSimFilePathsStatics {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IKnownCSimFilePathsStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IKnownCSimFilePathsStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownCSimFilePathsStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownCSimFilePathsStatics_Vtbl {
        unsafe extern "system" fn EFSpn<Impl: IKnownCSimFilePathsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EFSpn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gid1<Impl: IKnownCSimFilePathsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Gid1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gid2<Impl: IKnownCSimFilePathsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Gid2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownCSimFilePathsStatics, BASE_OFFSET>(),
            EFSpn: EFSpn::<Impl, IMPL_OFFSET>,
            Gid1: Gid1::<Impl, IMPL_OFFSET>,
            Gid2: Gid2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownCSimFilePathsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IKnownRuimFilePathsStatics_Impl: Sized {
    fn EFSpn(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn Gid1(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn Gid2(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKnownRuimFilePathsStatics {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IKnownRuimFilePathsStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IKnownRuimFilePathsStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownRuimFilePathsStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownRuimFilePathsStatics_Vtbl {
        unsafe extern "system" fn EFSpn<Impl: IKnownRuimFilePathsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EFSpn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gid1<Impl: IKnownRuimFilePathsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Gid1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gid2<Impl: IKnownRuimFilePathsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Gid2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownRuimFilePathsStatics, BASE_OFFSET>(),
            EFSpn: EFSpn::<Impl, IMPL_OFFSET>,
            Gid1: Gid1::<Impl, IMPL_OFFSET>,
            Gid2: Gid2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownRuimFilePathsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IKnownSimFilePathsStatics_Impl: Sized {
    fn EFOns(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn EFSpn(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn Gid1(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn Gid2(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKnownSimFilePathsStatics {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IKnownSimFilePathsStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IKnownSimFilePathsStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownSimFilePathsStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownSimFilePathsStatics_Vtbl {
        unsafe extern "system" fn EFOns<Impl: IKnownSimFilePathsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EFOns() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EFSpn<Impl: IKnownSimFilePathsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EFSpn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gid1<Impl: IKnownSimFilePathsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Gid1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gid2<Impl: IKnownSimFilePathsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Gid2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownSimFilePathsStatics, BASE_OFFSET>(),
            EFOns: EFOns::<Impl, IMPL_OFFSET>,
            EFSpn: EFSpn::<Impl, IMPL_OFFSET>,
            Gid1: Gid1::<Impl, IMPL_OFFSET>,
            Gid2: Gid2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownSimFilePathsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IKnownUSimFilePathsStatics_Impl: Sized {
    fn EFSpn(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn EFOpl(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn EFPnn(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn Gid1(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn Gid2(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKnownUSimFilePathsStatics {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IKnownUSimFilePathsStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IKnownUSimFilePathsStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownUSimFilePathsStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownUSimFilePathsStatics_Vtbl {
        unsafe extern "system" fn EFSpn<Impl: IKnownUSimFilePathsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EFSpn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EFOpl<Impl: IKnownUSimFilePathsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EFOpl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EFPnn<Impl: IKnownUSimFilePathsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EFPnn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gid1<Impl: IKnownUSimFilePathsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Gid1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gid2<Impl: IKnownUSimFilePathsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Gid2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownUSimFilePathsStatics, BASE_OFFSET>(),
            EFSpn: EFSpn::<Impl, IMPL_OFFSET>,
            EFOpl: EFOpl::<Impl, IMPL_OFFSET>,
            EFPnn: EFPnn::<Impl, IMPL_OFFSET>,
            Gid1: Gid1::<Impl, IMPL_OFFSET>,
            Gid2: Gid2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownUSimFilePathsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandAccount_Impl: Sized {
    fn NetworkAccountId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceProviderGuid(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ServiceProviderName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CurrentNetwork(&mut self) -> ::windows::core::Result<MobileBroadbandNetwork>;
    fn CurrentDeviceInformation(&mut self) -> ::windows::core::Result<MobileBroadbandDeviceInformation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandAccount {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandAccount";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandAccount_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandAccount_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandAccount_Vtbl {
        unsafe extern "system" fn NetworkAccountId<Impl: IMobileBroadbandAccount_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkAccountId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceProviderGuid<Impl: IMobileBroadbandAccount_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceProviderGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceProviderName<Impl: IMobileBroadbandAccount_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentNetwork<Impl: IMobileBroadbandAccount_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentNetwork() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDeviceInformation<Impl: IMobileBroadbandAccount_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentDeviceInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandAccount, BASE_OFFSET>(),
            NetworkAccountId: NetworkAccountId::<Impl, IMPL_OFFSET>,
            ServiceProviderGuid: ServiceProviderGuid::<Impl, IMPL_OFFSET>,
            ServiceProviderName: ServiceProviderName::<Impl, IMPL_OFFSET>,
            CurrentNetwork: CurrentNetwork::<Impl, IMPL_OFFSET>,
            CurrentDeviceInformation: CurrentDeviceInformation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandAccount as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
pub trait IMobileBroadbandAccount2_Impl: Sized {
    fn GetConnectionProfiles(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::Connectivity::ConnectionProfile>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandAccount2 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandAccount2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl IMobileBroadbandAccount2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandAccount2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandAccount2_Vtbl {
        unsafe extern "system" fn GetConnectionProfiles<Impl: IMobileBroadbandAccount2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectionProfiles() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandAccount2, BASE_OFFSET>(),
            GetConnectionProfiles: GetConnectionProfiles::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandAccount2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandAccount3_Impl: Sized {
    fn AccountExperienceUrl(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandAccount3 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandAccount3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandAccount3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandAccount3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandAccount3_Vtbl {
        unsafe extern "system" fn AccountExperienceUrl<Impl: IMobileBroadbandAccount3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccountExperienceUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandAccount3, BASE_OFFSET>(),
            AccountExperienceUrl: AccountExperienceUrl::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandAccount3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandAccountEventArgs_Impl: Sized {
    fn NetworkAccountId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandAccountEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandAccountEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandAccountEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandAccountEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandAccountEventArgs_Vtbl {
        unsafe extern "system" fn NetworkAccountId<Impl: IMobileBroadbandAccountEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkAccountId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandAccountEventArgs, BASE_OFFSET>(),
            NetworkAccountId: NetworkAccountId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandAccountEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandAccountStatics_Impl: Sized {
    fn AvailableNetworkAccountIds(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn CreateFromNetworkAccountId(&mut self, networkaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<MobileBroadbandAccount>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandAccountStatics {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandAccountStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandAccountStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandAccountStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandAccountStatics_Vtbl {
        unsafe extern "system" fn AvailableNetworkAccountIds<Impl: IMobileBroadbandAccountStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailableNetworkAccountIds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromNetworkAccountId<Impl: IMobileBroadbandAccountStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromNetworkAccountId(&*(&networkaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandAccountStatics, BASE_OFFSET>(),
            AvailableNetworkAccountIds: AvailableNetworkAccountIds::<Impl, IMPL_OFFSET>,
            CreateFromNetworkAccountId: CreateFromNetworkAccountId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandAccountStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandAccountUpdatedEventArgs_Impl: Sized {
    fn NetworkAccountId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HasDeviceInformationChanged(&mut self) -> ::windows::core::Result<bool>;
    fn HasNetworkChanged(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandAccountUpdatedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandAccountUpdatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandAccountUpdatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandAccountUpdatedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandAccountUpdatedEventArgs_Vtbl {
        unsafe extern "system" fn NetworkAccountId<Impl: IMobileBroadbandAccountUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkAccountId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasDeviceInformationChanged<Impl: IMobileBroadbandAccountUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasDeviceInformationChanged() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasNetworkChanged<Impl: IMobileBroadbandAccountUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasNetworkChanged() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandAccountUpdatedEventArgs, BASE_OFFSET>(),
            NetworkAccountId: NetworkAccountId::<Impl, IMPL_OFFSET>,
            HasDeviceInformationChanged: HasDeviceInformationChanged::<Impl, IMPL_OFFSET>,
            HasNetworkChanged: HasNetworkChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandAccountUpdatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandAccountWatcher_Impl: Sized {
    fn AccountAdded(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccountAdded(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AccountUpdated(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccountUpdated(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AccountRemoved(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccountRemoved(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&mut self) -> ::windows::core::Result<MobileBroadbandAccountWatcherStatus>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandAccountWatcher {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandAccountWatcher";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandAccountWatcher_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandAccountWatcher_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandAccountWatcher_Vtbl {
        unsafe extern "system" fn AccountAdded<Impl: IMobileBroadbandAccountWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccountAdded(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAccountAdded<Impl: IMobileBroadbandAccountWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAccountAdded(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AccountUpdated<Impl: IMobileBroadbandAccountWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccountUpdated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountUpdatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountUpdatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAccountUpdated<Impl: IMobileBroadbandAccountWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAccountUpdated(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AccountRemoved<Impl: IMobileBroadbandAccountWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccountRemoved(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAccountRemoved<Impl: IMobileBroadbandAccountWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAccountRemoved(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnumerationCompleted<Impl: IMobileBroadbandAccountWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerationCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnumerationCompleted<Impl: IMobileBroadbandAccountWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnumerationCompleted(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stopped<Impl: IMobileBroadbandAccountWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stopped(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStopped<Impl: IMobileBroadbandAccountWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStopped(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Status<Impl: IMobileBroadbandAccountWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandAccountWatcherStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Start<Impl: IMobileBroadbandAccountWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IMobileBroadbandAccountWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandAccountWatcher, BASE_OFFSET>(),
            AccountAdded: AccountAdded::<Impl, IMPL_OFFSET>,
            RemoveAccountAdded: RemoveAccountAdded::<Impl, IMPL_OFFSET>,
            AccountUpdated: AccountUpdated::<Impl, IMPL_OFFSET>,
            RemoveAccountUpdated: RemoveAccountUpdated::<Impl, IMPL_OFFSET>,
            AccountRemoved: AccountRemoved::<Impl, IMPL_OFFSET>,
            RemoveAccountRemoved: RemoveAccountRemoved::<Impl, IMPL_OFFSET>,
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
        iid == &<IMobileBroadbandAccountWatcher as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandAntennaSar_Impl: Sized {
    fn AntennaIndex(&mut self) -> ::windows::core::Result<i32>;
    fn SarBackoffIndex(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandAntennaSar {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandAntennaSar";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandAntennaSar_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandAntennaSar_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandAntennaSar_Vtbl {
        unsafe extern "system" fn AntennaIndex<Impl: IMobileBroadbandAntennaSar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AntennaIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SarBackoffIndex<Impl: IMobileBroadbandAntennaSar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SarBackoffIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandAntennaSar, BASE_OFFSET>(),
            AntennaIndex: AntennaIndex::<Impl, IMPL_OFFSET>,
            SarBackoffIndex: SarBackoffIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandAntennaSar as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandAntennaSarFactory_Impl: Sized {
    fn CreateWithIndex(&mut self, antennaindex: i32, sarbackoffindex: i32) -> ::windows::core::Result<MobileBroadbandAntennaSar>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandAntennaSarFactory {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandAntennaSarFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandAntennaSarFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandAntennaSarFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandAntennaSarFactory_Vtbl {
        unsafe extern "system" fn CreateWithIndex<Impl: IMobileBroadbandAntennaSarFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, antennaindex: i32, sarbackoffindex: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithIndex(antennaindex, sarbackoffindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandAntennaSarFactory, BASE_OFFSET>(),
            CreateWithIndex: CreateWithIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandAntennaSarFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandCellCdma_Impl: Sized {
    fn BaseStationId(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn BaseStationPNCode(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn BaseStationLatitude(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn BaseStationLongitude(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn BaseStationLastBroadcastGpsTime(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn NetworkId(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn PilotSignalStrengthInDB(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn SystemId(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandCellCdma {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandCellCdma";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandCellCdma_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandCellCdma_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandCellCdma_Vtbl {
        unsafe extern "system" fn BaseStationId<Impl: IMobileBroadbandCellCdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BaseStationId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BaseStationPNCode<Impl: IMobileBroadbandCellCdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BaseStationPNCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BaseStationLatitude<Impl: IMobileBroadbandCellCdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BaseStationLatitude() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BaseStationLongitude<Impl: IMobileBroadbandCellCdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BaseStationLongitude() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BaseStationLastBroadcastGpsTime<Impl: IMobileBroadbandCellCdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BaseStationLastBroadcastGpsTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkId<Impl: IMobileBroadbandCellCdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PilotSignalStrengthInDB<Impl: IMobileBroadbandCellCdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PilotSignalStrengthInDB() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemId<Impl: IMobileBroadbandCellCdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandCellCdma, BASE_OFFSET>(),
            BaseStationId: BaseStationId::<Impl, IMPL_OFFSET>,
            BaseStationPNCode: BaseStationPNCode::<Impl, IMPL_OFFSET>,
            BaseStationLatitude: BaseStationLatitude::<Impl, IMPL_OFFSET>,
            BaseStationLongitude: BaseStationLongitude::<Impl, IMPL_OFFSET>,
            BaseStationLastBroadcastGpsTime: BaseStationLastBroadcastGpsTime::<Impl, IMPL_OFFSET>,
            NetworkId: NetworkId::<Impl, IMPL_OFFSET>,
            PilotSignalStrengthInDB: PilotSignalStrengthInDB::<Impl, IMPL_OFFSET>,
            SystemId: SystemId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandCellCdma as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandCellGsm_Impl: Sized {
    fn BaseStationId(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn CellId(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn ChannelNumber(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn LocationAreaCode(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn ProviderId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReceivedSignalStrengthInDBm(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn TimingAdvanceInBitPeriods(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandCellGsm {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandCellGsm";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandCellGsm_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandCellGsm_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandCellGsm_Vtbl {
        unsafe extern "system" fn BaseStationId<Impl: IMobileBroadbandCellGsm_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BaseStationId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CellId<Impl: IMobileBroadbandCellGsm_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CellId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChannelNumber<Impl: IMobileBroadbandCellGsm_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChannelNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocationAreaCode<Impl: IMobileBroadbandCellGsm_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocationAreaCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderId<Impl: IMobileBroadbandCellGsm_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceivedSignalStrengthInDBm<Impl: IMobileBroadbandCellGsm_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceivedSignalStrengthInDBm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimingAdvanceInBitPeriods<Impl: IMobileBroadbandCellGsm_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimingAdvanceInBitPeriods() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandCellGsm, BASE_OFFSET>(),
            BaseStationId: BaseStationId::<Impl, IMPL_OFFSET>,
            CellId: CellId::<Impl, IMPL_OFFSET>,
            ChannelNumber: ChannelNumber::<Impl, IMPL_OFFSET>,
            LocationAreaCode: LocationAreaCode::<Impl, IMPL_OFFSET>,
            ProviderId: ProviderId::<Impl, IMPL_OFFSET>,
            ReceivedSignalStrengthInDBm: ReceivedSignalStrengthInDBm::<Impl, IMPL_OFFSET>,
            TimingAdvanceInBitPeriods: TimingAdvanceInBitPeriods::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandCellGsm as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandCellLte_Impl: Sized {
    fn CellId(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn ChannelNumber(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn PhysicalCellId(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn ProviderId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReferenceSignalReceivedPowerInDBm(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn ReferenceSignalReceivedQualityInDBm(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn TimingAdvanceInBitPeriods(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn TrackingAreaCode(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandCellLte {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandCellLte";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandCellLte_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandCellLte_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandCellLte_Vtbl {
        unsafe extern "system" fn CellId<Impl: IMobileBroadbandCellLte_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CellId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChannelNumber<Impl: IMobileBroadbandCellLte_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChannelNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhysicalCellId<Impl: IMobileBroadbandCellLte_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhysicalCellId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderId<Impl: IMobileBroadbandCellLte_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferenceSignalReceivedPowerInDBm<Impl: IMobileBroadbandCellLte_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReferenceSignalReceivedPowerInDBm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferenceSignalReceivedQualityInDBm<Impl: IMobileBroadbandCellLte_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReferenceSignalReceivedQualityInDBm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimingAdvanceInBitPeriods<Impl: IMobileBroadbandCellLte_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimingAdvanceInBitPeriods() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrackingAreaCode<Impl: IMobileBroadbandCellLte_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrackingAreaCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandCellLte, BASE_OFFSET>(),
            CellId: CellId::<Impl, IMPL_OFFSET>,
            ChannelNumber: ChannelNumber::<Impl, IMPL_OFFSET>,
            PhysicalCellId: PhysicalCellId::<Impl, IMPL_OFFSET>,
            ProviderId: ProviderId::<Impl, IMPL_OFFSET>,
            ReferenceSignalReceivedPowerInDBm: ReferenceSignalReceivedPowerInDBm::<Impl, IMPL_OFFSET>,
            ReferenceSignalReceivedQualityInDBm: ReferenceSignalReceivedQualityInDBm::<Impl, IMPL_OFFSET>,
            TimingAdvanceInBitPeriods: TimingAdvanceInBitPeriods::<Impl, IMPL_OFFSET>,
            TrackingAreaCode: TrackingAreaCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandCellLte as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandCellNR_Impl: Sized {
    fn CellId(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i64>>;
    fn ChannelNumber(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn PhysicalCellId(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn ProviderId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReferenceSignalReceivedPowerInDBm(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn ReferenceSignalReceivedQualityInDBm(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn TimingAdvanceInNanoseconds(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn TrackingAreaCode(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SignalToNoiseRatioInDB(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandCellNR {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandCellNR";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandCellNR_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandCellNR_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandCellNR_Vtbl {
        unsafe extern "system" fn CellId<Impl: IMobileBroadbandCellNR_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CellId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChannelNumber<Impl: IMobileBroadbandCellNR_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChannelNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhysicalCellId<Impl: IMobileBroadbandCellNR_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhysicalCellId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderId<Impl: IMobileBroadbandCellNR_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferenceSignalReceivedPowerInDBm<Impl: IMobileBroadbandCellNR_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReferenceSignalReceivedPowerInDBm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferenceSignalReceivedQualityInDBm<Impl: IMobileBroadbandCellNR_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReferenceSignalReceivedQualityInDBm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimingAdvanceInNanoseconds<Impl: IMobileBroadbandCellNR_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimingAdvanceInNanoseconds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrackingAreaCode<Impl: IMobileBroadbandCellNR_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrackingAreaCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignalToNoiseRatioInDB<Impl: IMobileBroadbandCellNR_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignalToNoiseRatioInDB() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandCellNR, BASE_OFFSET>(),
            CellId: CellId::<Impl, IMPL_OFFSET>,
            ChannelNumber: ChannelNumber::<Impl, IMPL_OFFSET>,
            PhysicalCellId: PhysicalCellId::<Impl, IMPL_OFFSET>,
            ProviderId: ProviderId::<Impl, IMPL_OFFSET>,
            ReferenceSignalReceivedPowerInDBm: ReferenceSignalReceivedPowerInDBm::<Impl, IMPL_OFFSET>,
            ReferenceSignalReceivedQualityInDBm: ReferenceSignalReceivedQualityInDBm::<Impl, IMPL_OFFSET>,
            TimingAdvanceInNanoseconds: TimingAdvanceInNanoseconds::<Impl, IMPL_OFFSET>,
            TrackingAreaCode: TrackingAreaCode::<Impl, IMPL_OFFSET>,
            SignalToNoiseRatioInDB: SignalToNoiseRatioInDB::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandCellNR as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandCellTdscdma_Impl: Sized {
    fn CellId(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn CellParameterId(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn ChannelNumber(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn LocationAreaCode(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn PathLossInDB(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn ProviderId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReceivedSignalCodePowerInDBm(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn TimingAdvanceInBitPeriods(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandCellTdscdma {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandCellTdscdma";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandCellTdscdma_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandCellTdscdma_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandCellTdscdma_Vtbl {
        unsafe extern "system" fn CellId<Impl: IMobileBroadbandCellTdscdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CellId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CellParameterId<Impl: IMobileBroadbandCellTdscdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CellParameterId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChannelNumber<Impl: IMobileBroadbandCellTdscdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChannelNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocationAreaCode<Impl: IMobileBroadbandCellTdscdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocationAreaCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PathLossInDB<Impl: IMobileBroadbandCellTdscdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathLossInDB() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderId<Impl: IMobileBroadbandCellTdscdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceivedSignalCodePowerInDBm<Impl: IMobileBroadbandCellTdscdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceivedSignalCodePowerInDBm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimingAdvanceInBitPeriods<Impl: IMobileBroadbandCellTdscdma_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimingAdvanceInBitPeriods() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandCellTdscdma, BASE_OFFSET>(),
            CellId: CellId::<Impl, IMPL_OFFSET>,
            CellParameterId: CellParameterId::<Impl, IMPL_OFFSET>,
            ChannelNumber: ChannelNumber::<Impl, IMPL_OFFSET>,
            LocationAreaCode: LocationAreaCode::<Impl, IMPL_OFFSET>,
            PathLossInDB: PathLossInDB::<Impl, IMPL_OFFSET>,
            ProviderId: ProviderId::<Impl, IMPL_OFFSET>,
            ReceivedSignalCodePowerInDBm: ReceivedSignalCodePowerInDBm::<Impl, IMPL_OFFSET>,
            TimingAdvanceInBitPeriods: TimingAdvanceInBitPeriods::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandCellTdscdma as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandCellUmts_Impl: Sized {
    fn CellId(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn ChannelNumber(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn LocationAreaCode(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn PathLossInDB(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn PrimaryScramblingCode(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn ProviderId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReceivedSignalCodePowerInDBm(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn SignalToNoiseRatioInDB(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandCellUmts {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandCellUmts";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandCellUmts_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandCellUmts_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandCellUmts_Vtbl {
        unsafe extern "system" fn CellId<Impl: IMobileBroadbandCellUmts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CellId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChannelNumber<Impl: IMobileBroadbandCellUmts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChannelNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocationAreaCode<Impl: IMobileBroadbandCellUmts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocationAreaCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PathLossInDB<Impl: IMobileBroadbandCellUmts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PathLossInDB() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrimaryScramblingCode<Impl: IMobileBroadbandCellUmts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrimaryScramblingCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderId<Impl: IMobileBroadbandCellUmts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceivedSignalCodePowerInDBm<Impl: IMobileBroadbandCellUmts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceivedSignalCodePowerInDBm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignalToNoiseRatioInDB<Impl: IMobileBroadbandCellUmts_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignalToNoiseRatioInDB() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandCellUmts, BASE_OFFSET>(),
            CellId: CellId::<Impl, IMPL_OFFSET>,
            ChannelNumber: ChannelNumber::<Impl, IMPL_OFFSET>,
            LocationAreaCode: LocationAreaCode::<Impl, IMPL_OFFSET>,
            PathLossInDB: PathLossInDB::<Impl, IMPL_OFFSET>,
            PrimaryScramblingCode: PrimaryScramblingCode::<Impl, IMPL_OFFSET>,
            ProviderId: ProviderId::<Impl, IMPL_OFFSET>,
            ReceivedSignalCodePowerInDBm: ReceivedSignalCodePowerInDBm::<Impl, IMPL_OFFSET>,
            SignalToNoiseRatioInDB: SignalToNoiseRatioInDB::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandCellUmts as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandCellsInfo_Impl: Sized {
    fn NeighboringCellsCdma(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellCdma>>;
    fn NeighboringCellsGsm(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellGsm>>;
    fn NeighboringCellsLte(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellLte>>;
    fn NeighboringCellsTdscdma(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellTdscdma>>;
    fn NeighboringCellsUmts(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellUmts>>;
    fn ServingCellsCdma(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellCdma>>;
    fn ServingCellsGsm(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellGsm>>;
    fn ServingCellsLte(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellLte>>;
    fn ServingCellsTdscdma(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellTdscdma>>;
    fn ServingCellsUmts(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellUmts>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandCellsInfo {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandCellsInfo";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandCellsInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandCellsInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandCellsInfo_Vtbl {
        unsafe extern "system" fn NeighboringCellsCdma<Impl: IMobileBroadbandCellsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NeighboringCellsCdma() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NeighboringCellsGsm<Impl: IMobileBroadbandCellsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NeighboringCellsGsm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NeighboringCellsLte<Impl: IMobileBroadbandCellsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NeighboringCellsLte() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NeighboringCellsTdscdma<Impl: IMobileBroadbandCellsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NeighboringCellsTdscdma() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NeighboringCellsUmts<Impl: IMobileBroadbandCellsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NeighboringCellsUmts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServingCellsCdma<Impl: IMobileBroadbandCellsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServingCellsCdma() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServingCellsGsm<Impl: IMobileBroadbandCellsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServingCellsGsm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServingCellsLte<Impl: IMobileBroadbandCellsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServingCellsLte() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServingCellsTdscdma<Impl: IMobileBroadbandCellsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServingCellsTdscdma() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServingCellsUmts<Impl: IMobileBroadbandCellsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServingCellsUmts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandCellsInfo, BASE_OFFSET>(),
            NeighboringCellsCdma: NeighboringCellsCdma::<Impl, IMPL_OFFSET>,
            NeighboringCellsGsm: NeighboringCellsGsm::<Impl, IMPL_OFFSET>,
            NeighboringCellsLte: NeighboringCellsLte::<Impl, IMPL_OFFSET>,
            NeighboringCellsTdscdma: NeighboringCellsTdscdma::<Impl, IMPL_OFFSET>,
            NeighboringCellsUmts: NeighboringCellsUmts::<Impl, IMPL_OFFSET>,
            ServingCellsCdma: ServingCellsCdma::<Impl, IMPL_OFFSET>,
            ServingCellsGsm: ServingCellsGsm::<Impl, IMPL_OFFSET>,
            ServingCellsLte: ServingCellsLte::<Impl, IMPL_OFFSET>,
            ServingCellsTdscdma: ServingCellsTdscdma::<Impl, IMPL_OFFSET>,
            ServingCellsUmts: ServingCellsUmts::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandCellsInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandCellsInfo2_Impl: Sized {
    fn NeighboringCellsNR(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellNR>>;
    fn ServingCellsNR(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellNR>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandCellsInfo2 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandCellsInfo2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandCellsInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandCellsInfo2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandCellsInfo2_Vtbl {
        unsafe extern "system" fn NeighboringCellsNR<Impl: IMobileBroadbandCellsInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NeighboringCellsNR() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServingCellsNR<Impl: IMobileBroadbandCellsInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServingCellsNR() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandCellsInfo2, BASE_OFFSET>(),
            NeighboringCellsNR: NeighboringCellsNR::<Impl, IMPL_OFFSET>,
            ServingCellsNR: ServingCellsNR::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandCellsInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandCurrentSlotIndexChangedEventArgs_Impl: Sized {
    fn CurrentSlotIndex(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandCurrentSlotIndexChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandCurrentSlotIndexChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandCurrentSlotIndexChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandCurrentSlotIndexChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandCurrentSlotIndexChangedEventArgs_Vtbl {
        unsafe extern "system" fn CurrentSlotIndex<Impl: IMobileBroadbandCurrentSlotIndexChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentSlotIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandCurrentSlotIndexChangedEventArgs, BASE_OFFSET>(),
            CurrentSlotIndex: CurrentSlotIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandCurrentSlotIndexChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Sms", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandDeviceInformation_Impl: Sized {
    fn NetworkDeviceStatus(&mut self) -> ::windows::core::Result<NetworkDeviceStatus>;
    fn Manufacturer(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Model(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FirmwareInformation(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CellularClass(&mut self) -> ::windows::core::Result<super::super::Devices::Sms::CellularClass>;
    fn DataClasses(&mut self) -> ::windows::core::Result<DataClasses>;
    fn CustomDataClass(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MobileEquipmentId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TelephoneNumbers(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn SubscriberId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SimIccId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceType(&mut self) -> ::windows::core::Result<MobileBroadbandDeviceType>;
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CurrentRadioState(&mut self) -> ::windows::core::Result<MobileBroadbandRadioState>;
}
#[cfg(all(feature = "Devices_Sms", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandDeviceInformation {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandDeviceInformation";
}
#[cfg(all(feature = "Devices_Sms", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandDeviceInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandDeviceInformation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandDeviceInformation_Vtbl {
        unsafe extern "system" fn NetworkDeviceStatus<Impl: IMobileBroadbandDeviceInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NetworkDeviceStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkDeviceStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Manufacturer<Impl: IMobileBroadbandDeviceInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Manufacturer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Model<Impl: IMobileBroadbandDeviceInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Model() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirmwareInformation<Impl: IMobileBroadbandDeviceInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirmwareInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CellularClass<Impl: IMobileBroadbandDeviceInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Sms::CellularClass) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CellularClass() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataClasses<Impl: IMobileBroadbandDeviceInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DataClasses) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataClasses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CustomDataClass<Impl: IMobileBroadbandDeviceInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomDataClass() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MobileEquipmentId<Impl: IMobileBroadbandDeviceInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MobileEquipmentId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TelephoneNumbers<Impl: IMobileBroadbandDeviceInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TelephoneNumbers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubscriberId<Impl: IMobileBroadbandDeviceInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubscriberId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimIccId<Impl: IMobileBroadbandDeviceInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SimIccId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceType<Impl: IMobileBroadbandDeviceInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Impl: IMobileBroadbandDeviceInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CurrentRadioState<Impl: IMobileBroadbandDeviceInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandRadioState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentRadioState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandDeviceInformation, BASE_OFFSET>(),
            NetworkDeviceStatus: NetworkDeviceStatus::<Impl, IMPL_OFFSET>,
            Manufacturer: Manufacturer::<Impl, IMPL_OFFSET>,
            Model: Model::<Impl, IMPL_OFFSET>,
            FirmwareInformation: FirmwareInformation::<Impl, IMPL_OFFSET>,
            CellularClass: CellularClass::<Impl, IMPL_OFFSET>,
            DataClasses: DataClasses::<Impl, IMPL_OFFSET>,
            CustomDataClass: CustomDataClass::<Impl, IMPL_OFFSET>,
            MobileEquipmentId: MobileEquipmentId::<Impl, IMPL_OFFSET>,
            TelephoneNumbers: TelephoneNumbers::<Impl, IMPL_OFFSET>,
            SubscriberId: SubscriberId::<Impl, IMPL_OFFSET>,
            SimIccId: SimIccId::<Impl, IMPL_OFFSET>,
            DeviceType: DeviceType::<Impl, IMPL_OFFSET>,
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            CurrentRadioState: CurrentRadioState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandDeviceInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandDeviceInformation2_Impl: Sized {
    fn PinManager(&mut self) -> ::windows::core::Result<MobileBroadbandPinManager>;
    fn Revision(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SerialNumber(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandDeviceInformation2 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandDeviceInformation2";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandDeviceInformation2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandDeviceInformation2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandDeviceInformation2_Vtbl {
        unsafe extern "system" fn PinManager<Impl: IMobileBroadbandDeviceInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PinManager() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Revision<Impl: IMobileBroadbandDeviceInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Revision() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SerialNumber<Impl: IMobileBroadbandDeviceInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SerialNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandDeviceInformation2, BASE_OFFSET>(),
            PinManager: PinManager::<Impl, IMPL_OFFSET>,
            Revision: Revision::<Impl, IMPL_OFFSET>,
            SerialNumber: SerialNumber::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandDeviceInformation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandDeviceInformation3_Impl: Sized {
    fn SimSpn(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SimPnn(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SimGid1(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandDeviceInformation3 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandDeviceInformation3";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandDeviceInformation3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandDeviceInformation3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandDeviceInformation3_Vtbl {
        unsafe extern "system" fn SimSpn<Impl: IMobileBroadbandDeviceInformation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SimSpn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimPnn<Impl: IMobileBroadbandDeviceInformation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SimPnn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SimGid1<Impl: IMobileBroadbandDeviceInformation3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SimGid1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandDeviceInformation3, BASE_OFFSET>(),
            SimSpn: SimSpn::<Impl, IMPL_OFFSET>,
            SimPnn: SimPnn::<Impl, IMPL_OFFSET>,
            SimGid1: SimGid1::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandDeviceInformation3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandDeviceInformation4_Impl: Sized {
    fn SlotManager(&mut self) -> ::windows::core::Result<MobileBroadbandSlotManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandDeviceInformation4 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandDeviceInformation4";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandDeviceInformation4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandDeviceInformation4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandDeviceInformation4_Vtbl {
        unsafe extern "system" fn SlotManager<Impl: IMobileBroadbandDeviceInformation4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SlotManager() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandDeviceInformation4, BASE_OFFSET>(),
            SlotManager: SlotManager::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandDeviceInformation4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandDeviceService_Impl: Sized {
    fn DeviceServiceId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SupportedCommands(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn OpenDataSession(&mut self) -> ::windows::core::Result<MobileBroadbandDeviceServiceDataSession>;
    fn OpenCommandSession(&mut self) -> ::windows::core::Result<MobileBroadbandDeviceServiceCommandSession>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandDeviceService {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandDeviceService";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandDeviceService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandDeviceService_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandDeviceService_Vtbl {
        unsafe extern "system" fn DeviceServiceId<Impl: IMobileBroadbandDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceServiceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedCommands<Impl: IMobileBroadbandDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedCommands() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenDataSession<Impl: IMobileBroadbandDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenDataSession() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenCommandSession<Impl: IMobileBroadbandDeviceService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenCommandSession() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandDeviceService, BASE_OFFSET>(),
            DeviceServiceId: DeviceServiceId::<Impl, IMPL_OFFSET>,
            SupportedCommands: SupportedCommands::<Impl, IMPL_OFFSET>,
            OpenDataSession: OpenDataSession::<Impl, IMPL_OFFSET>,
            OpenCommandSession: OpenCommandSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandDeviceService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMobileBroadbandDeviceServiceCommandResult_Impl: Sized {
    fn StatusCode(&mut self) -> ::windows::core::Result<u32>;
    fn ResponseData(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandDeviceServiceCommandResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandDeviceServiceCommandResult";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMobileBroadbandDeviceServiceCommandResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandDeviceServiceCommandResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandDeviceServiceCommandResult_Vtbl {
        unsafe extern "system" fn StatusCode<Impl: IMobileBroadbandDeviceServiceCommandResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseData<Impl: IMobileBroadbandDeviceServiceCommandResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandDeviceServiceCommandResult, BASE_OFFSET>(),
            StatusCode: StatusCode::<Impl, IMPL_OFFSET>,
            ResponseData: ResponseData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandDeviceServiceCommandResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMobileBroadbandDeviceServiceCommandSession_Impl: Sized {
    fn SendQueryCommandAsync(&mut self, commandid: u32, data: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandDeviceServiceCommandResult>>;
    fn SendSetCommandAsync(&mut self, commandid: u32, data: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandDeviceServiceCommandResult>>;
    fn CloseSession(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandDeviceServiceCommandSession {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandDeviceServiceCommandSession";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMobileBroadbandDeviceServiceCommandSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandDeviceServiceCommandSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandDeviceServiceCommandSession_Vtbl {
        unsafe extern "system" fn SendQueryCommandAsync<Impl: IMobileBroadbandDeviceServiceCommandSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendQueryCommandAsync(commandid, &*(&data as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendSetCommandAsync<Impl: IMobileBroadbandDeviceServiceCommandSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendSetCommandAsync(commandid, &*(&data as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseSession<Impl: IMobileBroadbandDeviceServiceCommandSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseSession().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandDeviceServiceCommandSession, BASE_OFFSET>(),
            SendQueryCommandAsync: SendQueryCommandAsync::<Impl, IMPL_OFFSET>,
            SendSetCommandAsync: SendSetCommandAsync::<Impl, IMPL_OFFSET>,
            CloseSession: CloseSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandDeviceServiceCommandSession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMobileBroadbandDeviceServiceDataReceivedEventArgs_Impl: Sized {
    fn ReceivedData(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandDeviceServiceDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandDeviceServiceDataReceivedEventArgs";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMobileBroadbandDeviceServiceDataReceivedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandDeviceServiceDataReceivedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandDeviceServiceDataReceivedEventArgs_Vtbl {
        unsafe extern "system" fn ReceivedData<Impl: IMobileBroadbandDeviceServiceDataReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceivedData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandDeviceServiceDataReceivedEventArgs, BASE_OFFSET>(),
            ReceivedData: ReceivedData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandDeviceServiceDataReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMobileBroadbandDeviceServiceDataSession_Impl: Sized {
    fn WriteDataAsync(&mut self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CloseSession(&mut self) -> ::windows::core::Result<()>;
    fn DataReceived(&mut self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandDeviceServiceDataSession, MobileBroadbandDeviceServiceDataReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDataReceived(&mut self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandDeviceServiceDataSession {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandDeviceServiceDataSession";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMobileBroadbandDeviceServiceDataSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandDeviceServiceDataSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandDeviceServiceDataSession_Vtbl {
        unsafe extern "system" fn WriteDataAsync<Impl: IMobileBroadbandDeviceServiceDataSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteDataAsync(&*(&value as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseSession<Impl: IMobileBroadbandDeviceServiceDataSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseSession().into()
        }
        unsafe extern "system" fn DataReceived<Impl: IMobileBroadbandDeviceServiceDataSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataReceived(&*(&eventhandler as *const <super::super::Foundation::TypedEventHandler<MobileBroadbandDeviceServiceDataSession, MobileBroadbandDeviceServiceDataReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MobileBroadbandDeviceServiceDataSession, MobileBroadbandDeviceServiceDataReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDataReceived<Impl: IMobileBroadbandDeviceServiceDataSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDataReceived(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandDeviceServiceDataSession, BASE_OFFSET>(),
            WriteDataAsync: WriteDataAsync::<Impl, IMPL_OFFSET>,
            CloseSession: CloseSession::<Impl, IMPL_OFFSET>,
            DataReceived: DataReceived::<Impl, IMPL_OFFSET>,
            RemoveDataReceived: RemoveDataReceived::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandDeviceServiceDataSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandDeviceServiceInformation_Impl: Sized {
    fn DeviceServiceId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IsDataReadSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsDataWriteSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandDeviceServiceInformation {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandDeviceServiceInformation";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandDeviceServiceInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandDeviceServiceInformation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandDeviceServiceInformation_Vtbl {
        unsafe extern "system" fn DeviceServiceId<Impl: IMobileBroadbandDeviceServiceInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceServiceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDataReadSupported<Impl: IMobileBroadbandDeviceServiceInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDataReadSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDataWriteSupported<Impl: IMobileBroadbandDeviceServiceInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDataWriteSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandDeviceServiceInformation, BASE_OFFSET>(),
            DeviceServiceId: DeviceServiceId::<Impl, IMPL_OFFSET>,
            IsDataReadSupported: IsDataReadSupported::<Impl, IMPL_OFFSET>,
            IsDataWriteSupported: IsDataWriteSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandDeviceServiceInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMobileBroadbandDeviceServiceTriggerDetails_Impl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceServiceId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ReceivedData(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandDeviceServiceTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandDeviceServiceTriggerDetails";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMobileBroadbandDeviceServiceTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandDeviceServiceTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandDeviceServiceTriggerDetails_Vtbl {
        unsafe extern "system" fn DeviceId<Impl: IMobileBroadbandDeviceServiceTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceServiceId<Impl: IMobileBroadbandDeviceServiceTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceServiceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceivedData<Impl: IMobileBroadbandDeviceServiceTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceivedData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandDeviceServiceTriggerDetails, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            DeviceServiceId: DeviceServiceId::<Impl, IMPL_OFFSET>,
            ReceivedData: ReceivedData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandDeviceServiceTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandModem_Impl: Sized {
    fn CurrentAccount(&mut self) -> ::windows::core::Result<MobileBroadbandAccount>;
    fn DeviceInformation(&mut self) -> ::windows::core::Result<MobileBroadbandDeviceInformation>;
    fn MaxDeviceServiceCommandSizeInBytes(&mut self) -> ::windows::core::Result<u32>;
    fn MaxDeviceServiceDataSizeInBytes(&mut self) -> ::windows::core::Result<u32>;
    fn DeviceServices(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandDeviceServiceInformation>>;
    fn GetDeviceService(&mut self, deviceserviceid: &::windows::core::GUID) -> ::windows::core::Result<MobileBroadbandDeviceService>;
    fn IsResetSupported(&mut self) -> ::windows::core::Result<bool>;
    fn ResetAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetCurrentConfigurationAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandModemConfiguration>>;
    fn CurrentNetwork(&mut self) -> ::windows::core::Result<MobileBroadbandNetwork>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandModem {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandModem";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandModem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandModem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandModem_Vtbl {
        unsafe extern "system" fn CurrentAccount<Impl: IMobileBroadbandModem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAccount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceInformation<Impl: IMobileBroadbandModem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxDeviceServiceCommandSizeInBytes<Impl: IMobileBroadbandModem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxDeviceServiceCommandSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxDeviceServiceDataSizeInBytes<Impl: IMobileBroadbandModem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxDeviceServiceDataSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceServices<Impl: IMobileBroadbandModem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceServices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceService<Impl: IMobileBroadbandModem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceserviceid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceService(&*(&deviceserviceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsResetSupported<Impl: IMobileBroadbandModem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsResetSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetAsync<Impl: IMobileBroadbandModem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResetAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentConfigurationAsync<Impl: IMobileBroadbandModem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentConfigurationAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentNetwork<Impl: IMobileBroadbandModem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentNetwork() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandModem, BASE_OFFSET>(),
            CurrentAccount: CurrentAccount::<Impl, IMPL_OFFSET>,
            DeviceInformation: DeviceInformation::<Impl, IMPL_OFFSET>,
            MaxDeviceServiceCommandSizeInBytes: MaxDeviceServiceCommandSizeInBytes::<Impl, IMPL_OFFSET>,
            MaxDeviceServiceDataSizeInBytes: MaxDeviceServiceDataSizeInBytes::<Impl, IMPL_OFFSET>,
            DeviceServices: DeviceServices::<Impl, IMPL_OFFSET>,
            GetDeviceService: GetDeviceService::<Impl, IMPL_OFFSET>,
            IsResetSupported: IsResetSupported::<Impl, IMPL_OFFSET>,
            ResetAsync: ResetAsync::<Impl, IMPL_OFFSET>,
            GetCurrentConfigurationAsync: GetCurrentConfigurationAsync::<Impl, IMPL_OFFSET>,
            CurrentNetwork: CurrentNetwork::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandModem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandModem2_Impl: Sized {
    fn GetIsPassthroughEnabledAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn SetIsPassthroughEnabledAsync(&mut self, value: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandModemStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandModem2 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandModem2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandModem2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandModem2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandModem2_Vtbl {
        unsafe extern "system" fn GetIsPassthroughEnabledAsync<Impl: IMobileBroadbandModem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsPassthroughEnabledAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsPassthroughEnabledAsync<Impl: IMobileBroadbandModem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIsPassthroughEnabledAsync(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandModem2, BASE_OFFSET>(),
            GetIsPassthroughEnabledAsync: GetIsPassthroughEnabledAsync::<Impl, IMPL_OFFSET>,
            SetIsPassthroughEnabledAsync: SetIsPassthroughEnabledAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandModem2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandModem3_Impl: Sized {
    fn TryGetPcoAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPco>>;
    fn IsInEmergencyCallMode(&mut self) -> ::windows::core::Result<bool>;
    fn IsInEmergencyCallModeChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandModem, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsInEmergencyCallModeChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandModem3 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandModem3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandModem3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandModem3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandModem3_Vtbl {
        unsafe extern "system" fn TryGetPcoAsync<Impl: IMobileBroadbandModem3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetPcoAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInEmergencyCallMode<Impl: IMobileBroadbandModem3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInEmergencyCallMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInEmergencyCallModeChanged<Impl: IMobileBroadbandModem3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInEmergencyCallModeChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MobileBroadbandModem, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MobileBroadbandModem, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveIsInEmergencyCallModeChanged<Impl: IMobileBroadbandModem3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveIsInEmergencyCallModeChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandModem3, BASE_OFFSET>(),
            TryGetPcoAsync: TryGetPcoAsync::<Impl, IMPL_OFFSET>,
            IsInEmergencyCallMode: IsInEmergencyCallMode::<Impl, IMPL_OFFSET>,
            IsInEmergencyCallModeChanged: IsInEmergencyCallModeChanged::<Impl, IMPL_OFFSET>,
            RemoveIsInEmergencyCallModeChanged: RemoveIsInEmergencyCallModeChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandModem3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandModemConfiguration_Impl: Sized {
    fn Uicc(&mut self) -> ::windows::core::Result<MobileBroadbandUicc>;
    fn HomeProviderId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HomeProviderName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandModemConfiguration {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandModemConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandModemConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandModemConfiguration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandModemConfiguration_Vtbl {
        unsafe extern "system" fn Uicc<Impl: IMobileBroadbandModemConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uicc() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HomeProviderId<Impl: IMobileBroadbandModemConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HomeProviderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HomeProviderName<Impl: IMobileBroadbandModemConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HomeProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandModemConfiguration, BASE_OFFSET>(),
            Uicc: Uicc::<Impl, IMPL_OFFSET>,
            HomeProviderId: HomeProviderId::<Impl, IMPL_OFFSET>,
            HomeProviderName: HomeProviderName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandModemConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandModemConfiguration2_Impl: Sized {
    fn SarManager(&mut self) -> ::windows::core::Result<MobileBroadbandSarManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandModemConfiguration2 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandModemConfiguration2";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandModemConfiguration2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandModemConfiguration2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandModemConfiguration2_Vtbl {
        unsafe extern "system" fn SarManager<Impl: IMobileBroadbandModemConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SarManager() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandModemConfiguration2, BASE_OFFSET>(),
            SarManager: SarManager::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandModemConfiguration2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandModemIsolation_Impl: Sized {
    fn AddAllowedHost(&mut self, host: &::core::option::Option<super::HostName>) -> ::windows::core::Result<()>;
    fn AddAllowedHostRange(&mut self, first: &::core::option::Option<super::HostName>, last: &::core::option::Option<super::HostName>) -> ::windows::core::Result<()>;
    fn ApplyConfigurationAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ClearConfigurationAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandModemIsolation {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandModemIsolation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandModemIsolation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandModemIsolation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandModemIsolation_Vtbl {
        unsafe extern "system" fn AddAllowedHost<Impl: IMobileBroadbandModemIsolation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, host: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAllowedHost(&*(&host as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddAllowedHostRange<Impl: IMobileBroadbandModemIsolation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, first: ::windows::core::RawPtr, last: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAllowedHostRange(&*(&first as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType), &*(&last as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ApplyConfigurationAsync<Impl: IMobileBroadbandModemIsolation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplyConfigurationAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearConfigurationAsync<Impl: IMobileBroadbandModemIsolation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearConfigurationAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandModemIsolation, BASE_OFFSET>(),
            AddAllowedHost: AddAllowedHost::<Impl, IMPL_OFFSET>,
            AddAllowedHostRange: AddAllowedHostRange::<Impl, IMPL_OFFSET>,
            ApplyConfigurationAsync: ApplyConfigurationAsync::<Impl, IMPL_OFFSET>,
            ClearConfigurationAsync: ClearConfigurationAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandModemIsolation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandModemIsolationFactory_Impl: Sized {
    fn Create(&mut self, modemdeviceid: &::windows::core::HSTRING, rulegroupid: &::windows::core::HSTRING) -> ::windows::core::Result<MobileBroadbandModemIsolation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandModemIsolationFactory {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandModemIsolationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandModemIsolationFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandModemIsolationFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandModemIsolationFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IMobileBroadbandModemIsolationFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modemdeviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, rulegroupid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&modemdeviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&rulegroupid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandModemIsolationFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandModemIsolationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandModemStatics_Impl: Sized {
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromId(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<MobileBroadbandModem>;
    fn GetDefault(&mut self) -> ::windows::core::Result<MobileBroadbandModem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandModemStatics {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandModemStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandModemStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandModemStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandModemStatics_Vtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IMobileBroadbandModemStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromId<Impl: IMobileBroadbandModemStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDefault<Impl: IMobileBroadbandModemStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandModemStatics, BASE_OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            FromId: FromId::<Impl, IMPL_OFFSET>,
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandModemStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Networking_Connectivity", feature = "implement_exclusive"))]
pub trait IMobileBroadbandNetwork_Impl: Sized {
    fn NetworkAdapter(&mut self) -> ::windows::core::Result<super::Connectivity::NetworkAdapter>;
    fn NetworkRegistrationState(&mut self) -> ::windows::core::Result<NetworkRegistrationState>;
    fn RegistrationNetworkError(&mut self) -> ::windows::core::Result<u32>;
    fn PacketAttachNetworkError(&mut self) -> ::windows::core::Result<u32>;
    fn ActivationNetworkError(&mut self) -> ::windows::core::Result<u32>;
    fn AccessPointName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RegisteredDataClass(&mut self) -> ::windows::core::Result<DataClasses>;
    fn RegisteredProviderId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RegisteredProviderName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ShowConnectionUI(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandNetwork {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandNetwork";
}
#[cfg(all(feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl IMobileBroadbandNetwork_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandNetwork_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandNetwork_Vtbl {
        unsafe extern "system" fn NetworkAdapter<Impl: IMobileBroadbandNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkAdapter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkRegistrationState<Impl: IMobileBroadbandNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NetworkRegistrationState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkRegistrationState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegistrationNetworkError<Impl: IMobileBroadbandNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegistrationNetworkError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PacketAttachNetworkError<Impl: IMobileBroadbandNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PacketAttachNetworkError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivationNetworkError<Impl: IMobileBroadbandNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivationNetworkError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccessPointName<Impl: IMobileBroadbandNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessPointName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisteredDataClass<Impl: IMobileBroadbandNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DataClasses) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisteredDataClass() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisteredProviderId<Impl: IMobileBroadbandNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisteredProviderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisteredProviderName<Impl: IMobileBroadbandNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisteredProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowConnectionUI<Impl: IMobileBroadbandNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowConnectionUI().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandNetwork, BASE_OFFSET>(),
            NetworkAdapter: NetworkAdapter::<Impl, IMPL_OFFSET>,
            NetworkRegistrationState: NetworkRegistrationState::<Impl, IMPL_OFFSET>,
            RegistrationNetworkError: RegistrationNetworkError::<Impl, IMPL_OFFSET>,
            PacketAttachNetworkError: PacketAttachNetworkError::<Impl, IMPL_OFFSET>,
            ActivationNetworkError: ActivationNetworkError::<Impl, IMPL_OFFSET>,
            AccessPointName: AccessPointName::<Impl, IMPL_OFFSET>,
            RegisteredDataClass: RegisteredDataClass::<Impl, IMPL_OFFSET>,
            RegisteredProviderId: RegisteredProviderId::<Impl, IMPL_OFFSET>,
            RegisteredProviderName: RegisteredProviderName::<Impl, IMPL_OFFSET>,
            ShowConnectionUI: ShowConnectionUI::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandNetwork as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandNetwork2_Impl: Sized {
    fn GetVoiceCallSupportAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RegistrationUiccApps(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandUiccApp>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandNetwork2 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandNetwork2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandNetwork2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandNetwork2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandNetwork2_Vtbl {
        unsafe extern "system" fn GetVoiceCallSupportAsync<Impl: IMobileBroadbandNetwork2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVoiceCallSupportAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegistrationUiccApps<Impl: IMobileBroadbandNetwork2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegistrationUiccApps() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandNetwork2, BASE_OFFSET>(),
            GetVoiceCallSupportAsync: GetVoiceCallSupportAsync::<Impl, IMPL_OFFSET>,
            RegistrationUiccApps: RegistrationUiccApps::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandNetwork2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandNetwork3_Impl: Sized {
    fn GetCellsInfoAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandCellsInfo>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandNetwork3 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandNetwork3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandNetwork3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandNetwork3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandNetwork3_Vtbl {
        unsafe extern "system" fn GetCellsInfoAsync<Impl: IMobileBroadbandNetwork3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCellsInfoAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandNetwork3, BASE_OFFSET>(),
            GetCellsInfoAsync: GetCellsInfoAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandNetwork3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandNetworkRegistrationStateChange_Impl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Network(&mut self) -> ::windows::core::Result<MobileBroadbandNetwork>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandNetworkRegistrationStateChange {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandNetworkRegistrationStateChange";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandNetworkRegistrationStateChange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandNetworkRegistrationStateChange_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandNetworkRegistrationStateChange_Vtbl {
        unsafe extern "system" fn DeviceId<Impl: IMobileBroadbandNetworkRegistrationStateChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Network<Impl: IMobileBroadbandNetworkRegistrationStateChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Network() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandNetworkRegistrationStateChange, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            Network: Network::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandNetworkRegistrationStateChange as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails_Impl: Sized {
    fn NetworkRegistrationStateChanges(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandNetworkRegistrationStateChange>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails_Vtbl {
        unsafe extern "system" fn NetworkRegistrationStateChanges<Impl: IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkRegistrationStateChanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails, BASE_OFFSET>(),
            NetworkRegistrationStateChanges: NetworkRegistrationStateChanges::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMobileBroadbandPco_Impl: Sized {
    fn Data(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn IsComplete(&mut self) -> ::windows::core::Result<bool>;
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandPco {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandPco";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMobileBroadbandPco_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandPco_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandPco_Vtbl {
        unsafe extern "system" fn Data<Impl: IMobileBroadbandPco_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsComplete<Impl: IMobileBroadbandPco_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Impl: IMobileBroadbandPco_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandPco, BASE_OFFSET>(),
            Data: Data::<Impl, IMPL_OFFSET>,
            IsComplete: IsComplete::<Impl, IMPL_OFFSET>,
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandPco as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandPcoDataChangeTriggerDetails_Impl: Sized {
    fn UpdatedData(&mut self) -> ::windows::core::Result<MobileBroadbandPco>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandPcoDataChangeTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandPcoDataChangeTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandPcoDataChangeTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandPcoDataChangeTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandPcoDataChangeTriggerDetails_Vtbl {
        unsafe extern "system" fn UpdatedData<Impl: IMobileBroadbandPcoDataChangeTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdatedData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandPcoDataChangeTriggerDetails, BASE_OFFSET>(),
            UpdatedData: UpdatedData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandPcoDataChangeTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandPin_Impl: Sized {
    fn Type(&mut self) -> ::windows::core::Result<MobileBroadbandPinType>;
    fn LockState(&mut self) -> ::windows::core::Result<MobileBroadbandPinLockState>;
    fn Format(&mut self) -> ::windows::core::Result<MobileBroadbandPinFormat>;
    fn Enabled(&mut self) -> ::windows::core::Result<bool>;
    fn MaxLength(&mut self) -> ::windows::core::Result<u32>;
    fn MinLength(&mut self) -> ::windows::core::Result<u32>;
    fn AttemptsRemaining(&mut self) -> ::windows::core::Result<u32>;
    fn EnableAsync(&mut self, currentpin: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>;
    fn DisableAsync(&mut self, currentpin: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>;
    fn EnterAsync(&mut self, currentpin: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>;
    fn ChangeAsync(&mut self, currentpin: &::windows::core::HSTRING, newpin: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>;
    fn UnblockAsync(&mut self, pinunblockkey: &::windows::core::HSTRING, newpin: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandPin {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandPin";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandPin_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandPin_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandPin_Vtbl {
        unsafe extern "system" fn Type<Impl: IMobileBroadbandPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandPinType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockState<Impl: IMobileBroadbandPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandPinLockState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Format<Impl: IMobileBroadbandPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandPinFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Format() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Impl: IMobileBroadbandPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxLength<Impl: IMobileBroadbandPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinLength<Impl: IMobileBroadbandPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttemptsRemaining<Impl: IMobileBroadbandPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttemptsRemaining() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableAsync<Impl: IMobileBroadbandPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentpin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableAsync(&*(&currentpin as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableAsync<Impl: IMobileBroadbandPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentpin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableAsync(&*(&currentpin as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnterAsync<Impl: IMobileBroadbandPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentpin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnterAsync(&*(&currentpin as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeAsync<Impl: IMobileBroadbandPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentpin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, newpin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangeAsync(&*(&currentpin as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&newpin as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnblockAsync<Impl: IMobileBroadbandPin_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinunblockkey: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, newpin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnblockAsync(&*(&pinunblockkey as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&newpin as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandPin, BASE_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            LockState: LockState::<Impl, IMPL_OFFSET>,
            Format: Format::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            MaxLength: MaxLength::<Impl, IMPL_OFFSET>,
            MinLength: MinLength::<Impl, IMPL_OFFSET>,
            AttemptsRemaining: AttemptsRemaining::<Impl, IMPL_OFFSET>,
            EnableAsync: EnableAsync::<Impl, IMPL_OFFSET>,
            DisableAsync: DisableAsync::<Impl, IMPL_OFFSET>,
            EnterAsync: EnterAsync::<Impl, IMPL_OFFSET>,
            ChangeAsync: ChangeAsync::<Impl, IMPL_OFFSET>,
            UnblockAsync: UnblockAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandPin as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandPinLockStateChange_Impl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PinType(&mut self) -> ::windows::core::Result<MobileBroadbandPinType>;
    fn PinLockState(&mut self) -> ::windows::core::Result<MobileBroadbandPinLockState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandPinLockStateChange {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandPinLockStateChange";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandPinLockStateChange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandPinLockStateChange_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandPinLockStateChange_Vtbl {
        unsafe extern "system" fn DeviceId<Impl: IMobileBroadbandPinLockStateChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PinType<Impl: IMobileBroadbandPinLockStateChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandPinType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PinType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PinLockState<Impl: IMobileBroadbandPinLockStateChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandPinLockState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PinLockState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandPinLockStateChange, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            PinType: PinType::<Impl, IMPL_OFFSET>,
            PinLockState: PinLockState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandPinLockStateChange as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandPinLockStateChangeTriggerDetails_Impl: Sized {
    fn PinLockStateChanges(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandPinLockStateChange>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandPinLockStateChangeTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandPinLockStateChangeTriggerDetails";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandPinLockStateChangeTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandPinLockStateChangeTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandPinLockStateChangeTriggerDetails_Vtbl {
        unsafe extern "system" fn PinLockStateChanges<Impl: IMobileBroadbandPinLockStateChangeTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PinLockStateChanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandPinLockStateChangeTriggerDetails, BASE_OFFSET>(),
            PinLockStateChanges: PinLockStateChanges::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandPinLockStateChangeTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandPinManager_Impl: Sized {
    fn SupportedPins(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandPinType>>;
    fn GetPin(&mut self, pintype: MobileBroadbandPinType) -> ::windows::core::Result<MobileBroadbandPin>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandPinManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandPinManager";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandPinManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandPinManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandPinManager_Vtbl {
        unsafe extern "system" fn SupportedPins<Impl: IMobileBroadbandPinManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedPins() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPin<Impl: IMobileBroadbandPinManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pintype: MobileBroadbandPinType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPin(pintype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandPinManager, BASE_OFFSET>(),
            SupportedPins: SupportedPins::<Impl, IMPL_OFFSET>,
            GetPin: GetPin::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandPinManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandPinOperationResult_Impl: Sized {
    fn IsSuccessful(&mut self) -> ::windows::core::Result<bool>;
    fn AttemptsRemaining(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandPinOperationResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandPinOperationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandPinOperationResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandPinOperationResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandPinOperationResult_Vtbl {
        unsafe extern "system" fn IsSuccessful<Impl: IMobileBroadbandPinOperationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSuccessful() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttemptsRemaining<Impl: IMobileBroadbandPinOperationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttemptsRemaining() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandPinOperationResult, BASE_OFFSET>(),
            IsSuccessful: IsSuccessful::<Impl, IMPL_OFFSET>,
            AttemptsRemaining: AttemptsRemaining::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandPinOperationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandRadioStateChange_Impl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RadioState(&mut self) -> ::windows::core::Result<MobileBroadbandRadioState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandRadioStateChange {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandRadioStateChange";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandRadioStateChange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandRadioStateChange_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandRadioStateChange_Vtbl {
        unsafe extern "system" fn DeviceId<Impl: IMobileBroadbandRadioStateChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RadioState<Impl: IMobileBroadbandRadioStateChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandRadioState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RadioState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandRadioStateChange, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            RadioState: RadioState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandRadioStateChange as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandRadioStateChangeTriggerDetails_Impl: Sized {
    fn RadioStateChanges(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandRadioStateChange>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandRadioStateChangeTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandRadioStateChangeTriggerDetails";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandRadioStateChangeTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandRadioStateChangeTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandRadioStateChangeTriggerDetails_Vtbl {
        unsafe extern "system" fn RadioStateChanges<Impl: IMobileBroadbandRadioStateChangeTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RadioStateChanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandRadioStateChangeTriggerDetails, BASE_OFFSET>(),
            RadioStateChanges: RadioStateChanges::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandRadioStateChangeTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandSarManager_Impl: Sized {
    fn IsBackoffEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn IsWiFiHardwareIntegrated(&mut self) -> ::windows::core::Result<bool>;
    fn IsSarControlledByHardware(&mut self) -> ::windows::core::Result<bool>;
    fn Antennas(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandAntennaSar>>;
    fn HysteresisTimerPeriod(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn TransmissionStateChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandSarManager, MobileBroadbandTransmissionStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTransmissionStateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnableBackoffAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DisableBackoffAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetConfigurationAsync(&mut self, antennas: &::core::option::Option<super::super::Foundation::Collections::IIterable<MobileBroadbandAntennaSar>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RevertSarToHardwareControlAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetTransmissionStateChangedHysteresisAsync(&mut self, timerperiod: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetIsTransmittingAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn StartTransmissionStateMonitoring(&mut self) -> ::windows::core::Result<()>;
    fn StopTransmissionStateMonitoring(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandSarManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandSarManager";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandSarManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandSarManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandSarManager_Vtbl {
        unsafe extern "system" fn IsBackoffEnabled<Impl: IMobileBroadbandSarManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBackoffEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWiFiHardwareIntegrated<Impl: IMobileBroadbandSarManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWiFiHardwareIntegrated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSarControlledByHardware<Impl: IMobileBroadbandSarManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSarControlledByHardware() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Antennas<Impl: IMobileBroadbandSarManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Antennas() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HysteresisTimerPeriod<Impl: IMobileBroadbandSarManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HysteresisTimerPeriod() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmissionStateChanged<Impl: IMobileBroadbandSarManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmissionStateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MobileBroadbandSarManager, MobileBroadbandTransmissionStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MobileBroadbandSarManager, MobileBroadbandTransmissionStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTransmissionStateChanged<Impl: IMobileBroadbandSarManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTransmissionStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnableBackoffAsync<Impl: IMobileBroadbandSarManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableBackoffAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableBackoffAsync<Impl: IMobileBroadbandSarManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableBackoffAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConfigurationAsync<Impl: IMobileBroadbandSarManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, antennas: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetConfigurationAsync(&*(&antennas as *const <super::super::Foundation::Collections::IIterable<MobileBroadbandAntennaSar> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<MobileBroadbandAntennaSar> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevertSarToHardwareControlAsync<Impl: IMobileBroadbandSarManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RevertSarToHardwareControlAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransmissionStateChangedHysteresisAsync<Impl: IMobileBroadbandSarManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timerperiod: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTransmissionStateChangedHysteresisAsync(&*(&timerperiod as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsTransmittingAsync<Impl: IMobileBroadbandSarManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsTransmittingAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTransmissionStateMonitoring<Impl: IMobileBroadbandSarManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartTransmissionStateMonitoring().into()
        }
        unsafe extern "system" fn StopTransmissionStateMonitoring<Impl: IMobileBroadbandSarManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopTransmissionStateMonitoring().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandSarManager, BASE_OFFSET>(),
            IsBackoffEnabled: IsBackoffEnabled::<Impl, IMPL_OFFSET>,
            IsWiFiHardwareIntegrated: IsWiFiHardwareIntegrated::<Impl, IMPL_OFFSET>,
            IsSarControlledByHardware: IsSarControlledByHardware::<Impl, IMPL_OFFSET>,
            Antennas: Antennas::<Impl, IMPL_OFFSET>,
            HysteresisTimerPeriod: HysteresisTimerPeriod::<Impl, IMPL_OFFSET>,
            TransmissionStateChanged: TransmissionStateChanged::<Impl, IMPL_OFFSET>,
            RemoveTransmissionStateChanged: RemoveTransmissionStateChanged::<Impl, IMPL_OFFSET>,
            EnableBackoffAsync: EnableBackoffAsync::<Impl, IMPL_OFFSET>,
            DisableBackoffAsync: DisableBackoffAsync::<Impl, IMPL_OFFSET>,
            SetConfigurationAsync: SetConfigurationAsync::<Impl, IMPL_OFFSET>,
            RevertSarToHardwareControlAsync: RevertSarToHardwareControlAsync::<Impl, IMPL_OFFSET>,
            SetTransmissionStateChangedHysteresisAsync: SetTransmissionStateChangedHysteresisAsync::<Impl, IMPL_OFFSET>,
            GetIsTransmittingAsync: GetIsTransmittingAsync::<Impl, IMPL_OFFSET>,
            StartTransmissionStateMonitoring: StartTransmissionStateMonitoring::<Impl, IMPL_OFFSET>,
            StopTransmissionStateMonitoring: StopTransmissionStateMonitoring::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandSarManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandSlotInfo_Impl: Sized {
    fn Index(&mut self) -> ::windows::core::Result<i32>;
    fn State(&mut self) -> ::windows::core::Result<MobileBroadbandSlotState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandSlotInfo {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandSlotInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandSlotInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandSlotInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandSlotInfo_Vtbl {
        unsafe extern "system" fn Index<Impl: IMobileBroadbandSlotInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Index() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IMobileBroadbandSlotInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandSlotState) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandSlotInfo, BASE_OFFSET>(),
            Index: Index::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandSlotInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandSlotInfoChangedEventArgs_Impl: Sized {
    fn SlotInfo(&mut self) -> ::windows::core::Result<MobileBroadbandSlotInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandSlotInfoChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandSlotInfoChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandSlotInfoChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandSlotInfoChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandSlotInfoChangedEventArgs_Vtbl {
        unsafe extern "system" fn SlotInfo<Impl: IMobileBroadbandSlotInfoChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SlotInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandSlotInfoChangedEventArgs, BASE_OFFSET>(),
            SlotInfo: SlotInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandSlotInfoChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandSlotManager_Impl: Sized {
    fn SlotInfos(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandSlotInfo>>;
    fn CurrentSlotIndex(&mut self) -> ::windows::core::Result<i32>;
    fn SetCurrentSlot(&mut self, slotindex: i32) -> ::windows::core::Result<MobileBroadbandModemStatus>;
    fn SetCurrentSlotAsync(&mut self, slotindex: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandModemStatus>>;
    fn SlotInfoChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandSlotManager, MobileBroadbandSlotInfoChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSlotInfoChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CurrentSlotIndexChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandSlotManager, MobileBroadbandCurrentSlotIndexChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentSlotIndexChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandSlotManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandSlotManager";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandSlotManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandSlotManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandSlotManager_Vtbl {
        unsafe extern "system" fn SlotInfos<Impl: IMobileBroadbandSlotManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SlotInfos() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentSlotIndex<Impl: IMobileBroadbandSlotManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentSlotIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentSlot<Impl: IMobileBroadbandSlotManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, slotindex: i32, result__: *mut MobileBroadbandModemStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCurrentSlot(slotindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentSlotAsync<Impl: IMobileBroadbandSlotManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, slotindex: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCurrentSlotAsync(slotindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SlotInfoChanged<Impl: IMobileBroadbandSlotManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SlotInfoChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MobileBroadbandSlotManager, MobileBroadbandSlotInfoChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MobileBroadbandSlotManager, MobileBroadbandSlotInfoChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSlotInfoChanged<Impl: IMobileBroadbandSlotManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSlotInfoChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CurrentSlotIndexChanged<Impl: IMobileBroadbandSlotManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentSlotIndexChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<MobileBroadbandSlotManager, MobileBroadbandCurrentSlotIndexChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MobileBroadbandSlotManager, MobileBroadbandCurrentSlotIndexChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCurrentSlotIndexChanged<Impl: IMobileBroadbandSlotManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCurrentSlotIndexChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandSlotManager, BASE_OFFSET>(),
            SlotInfos: SlotInfos::<Impl, IMPL_OFFSET>,
            CurrentSlotIndex: CurrentSlotIndex::<Impl, IMPL_OFFSET>,
            SetCurrentSlot: SetCurrentSlot::<Impl, IMPL_OFFSET>,
            SetCurrentSlotAsync: SetCurrentSlotAsync::<Impl, IMPL_OFFSET>,
            SlotInfoChanged: SlotInfoChanged::<Impl, IMPL_OFFSET>,
            RemoveSlotInfoChanged: RemoveSlotInfoChanged::<Impl, IMPL_OFFSET>,
            CurrentSlotIndexChanged: CurrentSlotIndexChanged::<Impl, IMPL_OFFSET>,
            RemoveCurrentSlotIndexChanged: RemoveCurrentSlotIndexChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandSlotManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandTransmissionStateChangedEventArgs_Impl: Sized {
    fn IsTransmitting(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandTransmissionStateChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandTransmissionStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandTransmissionStateChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandTransmissionStateChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandTransmissionStateChangedEventArgs_Vtbl {
        unsafe extern "system" fn IsTransmitting<Impl: IMobileBroadbandTransmissionStateChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransmitting() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandTransmissionStateChangedEventArgs, BASE_OFFSET>(),
            IsTransmitting: IsTransmitting::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandTransmissionStateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandUicc_Impl: Sized {
    fn SimIccId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetUiccAppsAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandUiccAppsResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandUicc {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandUicc";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandUicc_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandUicc_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandUicc_Vtbl {
        unsafe extern "system" fn SimIccId<Impl: IMobileBroadbandUicc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SimIccId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUiccAppsAsync<Impl: IMobileBroadbandUicc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUiccAppsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandUicc, BASE_OFFSET>(),
            SimIccId: SimIccId::<Impl, IMPL_OFFSET>,
            GetUiccAppsAsync: GetUiccAppsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandUicc as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMobileBroadbandUiccApp_Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Kind(&mut self) -> ::windows::core::Result<UiccAppKind>;
    fn GetRecordDetailsAsync(&mut self, uiccfilepath: &::core::option::Option<super::super::Foundation::Collections::IIterable<u32>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandUiccAppRecordDetailsResult>>;
    fn ReadRecordAsync(&mut self, uiccfilepath: &::core::option::Option<super::super::Foundation::Collections::IIterable<u32>>, recordindex: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandUiccAppReadRecordResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandUiccApp {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandUiccApp";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMobileBroadbandUiccApp_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandUiccApp_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandUiccApp_Vtbl {
        unsafe extern "system" fn Id<Impl: IMobileBroadbandUiccApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Kind<Impl: IMobileBroadbandUiccApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UiccAppKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecordDetailsAsync<Impl: IMobileBroadbandUiccApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiccfilepath: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecordDetailsAsync(&*(&uiccfilepath as *const <super::super::Foundation::Collections::IIterable<u32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<u32> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadRecordAsync<Impl: IMobileBroadbandUiccApp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiccfilepath: ::windows::core::RawPtr, recordindex: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadRecordAsync(&*(&uiccfilepath as *const <super::super::Foundation::Collections::IIterable<u32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<u32> as ::windows::core::DefaultType>::DefaultType), recordindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandUiccApp, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Kind: Kind::<Impl, IMPL_OFFSET>,
            GetRecordDetailsAsync: GetRecordDetailsAsync::<Impl, IMPL_OFFSET>,
            ReadRecordAsync: ReadRecordAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandUiccApp as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMobileBroadbandUiccAppReadRecordResult_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<MobileBroadbandUiccAppOperationStatus>;
    fn Data(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandUiccAppReadRecordResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandUiccAppReadRecordResult";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMobileBroadbandUiccAppReadRecordResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandUiccAppReadRecordResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandUiccAppReadRecordResult_Vtbl {
        unsafe extern "system" fn Status<Impl: IMobileBroadbandUiccAppReadRecordResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandUiccAppOperationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Data<Impl: IMobileBroadbandUiccAppReadRecordResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandUiccAppReadRecordResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Data: Data::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandUiccAppReadRecordResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandUiccAppRecordDetailsResult_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<MobileBroadbandUiccAppOperationStatus>;
    fn Kind(&mut self) -> ::windows::core::Result<UiccAppRecordKind>;
    fn RecordCount(&mut self) -> ::windows::core::Result<i32>;
    fn RecordSize(&mut self) -> ::windows::core::Result<i32>;
    fn ReadAccessCondition(&mut self) -> ::windows::core::Result<UiccAccessCondition>;
    fn WriteAccessCondition(&mut self) -> ::windows::core::Result<UiccAccessCondition>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandUiccAppRecordDetailsResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandUiccAppRecordDetailsResult";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandUiccAppRecordDetailsResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandUiccAppRecordDetailsResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandUiccAppRecordDetailsResult_Vtbl {
        unsafe extern "system" fn Status<Impl: IMobileBroadbandUiccAppRecordDetailsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandUiccAppOperationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Kind<Impl: IMobileBroadbandUiccAppRecordDetailsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UiccAppRecordKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecordCount<Impl: IMobileBroadbandUiccAppRecordDetailsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecordCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecordSize<Impl: IMobileBroadbandUiccAppRecordDetailsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecordSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadAccessCondition<Impl: IMobileBroadbandUiccAppRecordDetailsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UiccAccessCondition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadAccessCondition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteAccessCondition<Impl: IMobileBroadbandUiccAppRecordDetailsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UiccAccessCondition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteAccessCondition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandUiccAppRecordDetailsResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Kind: Kind::<Impl, IMPL_OFFSET>,
            RecordCount: RecordCount::<Impl, IMPL_OFFSET>,
            RecordSize: RecordSize::<Impl, IMPL_OFFSET>,
            ReadAccessCondition: ReadAccessCondition::<Impl, IMPL_OFFSET>,
            WriteAccessCondition: WriteAccessCondition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandUiccAppRecordDetailsResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandUiccAppsResult_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<MobileBroadbandUiccAppOperationStatus>;
    fn UiccApps(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandUiccApp>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandUiccAppsResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandUiccAppsResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandUiccAppsResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandUiccAppsResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandUiccAppsResult_Vtbl {
        unsafe extern "system" fn Status<Impl: IMobileBroadbandUiccAppsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandUiccAppOperationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UiccApps<Impl: IMobileBroadbandUiccAppsResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UiccApps() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMobileBroadbandUiccAppsResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            UiccApps: UiccApps::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandUiccAppsResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorDataUsageTriggerDetails_Impl: Sized {
    fn NotificationKind(&mut self) -> ::windows::core::Result<NetworkOperatorDataUsageNotificationKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkOperatorDataUsageTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.INetworkOperatorDataUsageTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkOperatorDataUsageTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorDataUsageTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorDataUsageTriggerDetails_Vtbl {
        unsafe extern "system" fn NotificationKind<Impl: INetworkOperatorDataUsageTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NetworkOperatorDataUsageNotificationKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotificationKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INetworkOperatorDataUsageTriggerDetails, BASE_OFFSET>(),
            NotificationKind: NotificationKind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorDataUsageTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Sms", feature = "implement_exclusive"))]
pub trait INetworkOperatorNotificationEventDetails_Impl: Sized {
    fn NotificationType(&mut self) -> ::windows::core::Result<NetworkOperatorEventMessageType>;
    fn NetworkAccountId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EncodingType(&mut self) -> ::windows::core::Result<u8>;
    fn Message(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RuleId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SmsMessage(&mut self) -> ::windows::core::Result<super::super::Devices::Sms::ISmsMessage>;
}
#[cfg(all(feature = "Devices_Sms", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INetworkOperatorNotificationEventDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.INetworkOperatorNotificationEventDetails";
}
#[cfg(all(feature = "Devices_Sms", feature = "implement_exclusive"))]
impl INetworkOperatorNotificationEventDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorNotificationEventDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorNotificationEventDetails_Vtbl {
        unsafe extern "system" fn NotificationType<Impl: INetworkOperatorNotificationEventDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NetworkOperatorEventMessageType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotificationType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkAccountId<Impl: INetworkOperatorNotificationEventDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkAccountId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncodingType<Impl: INetworkOperatorNotificationEventDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EncodingType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Impl: INetworkOperatorNotificationEventDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RuleId<Impl: INetworkOperatorNotificationEventDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RuleId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmsMessage<Impl: INetworkOperatorNotificationEventDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmsMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INetworkOperatorNotificationEventDetails, BASE_OFFSET>(),
            NotificationType: NotificationType::<Impl, IMPL_OFFSET>,
            NetworkAccountId: NetworkAccountId::<Impl, IMPL_OFFSET>,
            EncodingType: EncodingType::<Impl, IMPL_OFFSET>,
            Message: Message::<Impl, IMPL_OFFSET>,
            RuleId: RuleId::<Impl, IMPL_OFFSET>,
            SmsMessage: SmsMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorNotificationEventDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorTetheringAccessPointConfiguration_Impl: Sized {
    fn Ssid(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSsid(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Passphrase(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPassphrase(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkOperatorTetheringAccessPointConfiguration {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.INetworkOperatorTetheringAccessPointConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkOperatorTetheringAccessPointConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorTetheringAccessPointConfiguration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorTetheringAccessPointConfiguration_Vtbl {
        unsafe extern "system" fn Ssid<Impl: INetworkOperatorTetheringAccessPointConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ssid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSsid<Impl: INetworkOperatorTetheringAccessPointConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSsid(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Passphrase<Impl: INetworkOperatorTetheringAccessPointConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Passphrase() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPassphrase<Impl: INetworkOperatorTetheringAccessPointConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPassphrase(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INetworkOperatorTetheringAccessPointConfiguration, BASE_OFFSET>(),
            Ssid: Ssid::<Impl, IMPL_OFFSET>,
            SetSsid: SetSsid::<Impl, IMPL_OFFSET>,
            Passphrase: Passphrase::<Impl, IMPL_OFFSET>,
            SetPassphrase: SetPassphrase::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorTetheringAccessPointConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait INetworkOperatorTetheringAccessPointConfiguration2_Impl: Sized {
    fn IsBandSupported(&mut self, band: TetheringWiFiBand) -> ::windows::core::Result<bool>;
    fn IsBandSupportedAsync(&mut self, band: TetheringWiFiBand) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn Band(&mut self) -> ::windows::core::Result<TetheringWiFiBand>;
    fn SetBand(&mut self, value: TetheringWiFiBand) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INetworkOperatorTetheringAccessPointConfiguration2 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.INetworkOperatorTetheringAccessPointConfiguration2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl INetworkOperatorTetheringAccessPointConfiguration2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorTetheringAccessPointConfiguration2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorTetheringAccessPointConfiguration2_Vtbl {
        unsafe extern "system" fn IsBandSupported<Impl: INetworkOperatorTetheringAccessPointConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, band: TetheringWiFiBand, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBandSupported(band) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBandSupportedAsync<Impl: INetworkOperatorTetheringAccessPointConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, band: TetheringWiFiBand, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBandSupportedAsync(band) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Band<Impl: INetworkOperatorTetheringAccessPointConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TetheringWiFiBand) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Band() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBand<Impl: INetworkOperatorTetheringAccessPointConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TetheringWiFiBand) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBand(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INetworkOperatorTetheringAccessPointConfiguration2, BASE_OFFSET>(),
            IsBandSupported: IsBandSupported::<Impl, IMPL_OFFSET>,
            IsBandSupportedAsync: IsBandSupportedAsync::<Impl, IMPL_OFFSET>,
            Band: Band::<Impl, IMPL_OFFSET>,
            SetBand: SetBand::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorTetheringAccessPointConfiguration2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait INetworkOperatorTetheringClient_Impl: Sized {
    fn MacAddress(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HostNames(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::HostName>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INetworkOperatorTetheringClient {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.INetworkOperatorTetheringClient";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl INetworkOperatorTetheringClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorTetheringClient_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorTetheringClient_Vtbl {
        unsafe extern "system" fn MacAddress<Impl: INetworkOperatorTetheringClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MacAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HostNames<Impl: INetworkOperatorTetheringClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HostNames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INetworkOperatorTetheringClient, BASE_OFFSET>(),
            MacAddress: MacAddress::<Impl, IMPL_OFFSET>,
            HostNames: HostNames::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorTetheringClient as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait INetworkOperatorTetheringClientManager_Impl: Sized {
    fn GetTetheringClients(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<NetworkOperatorTetheringClient>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INetworkOperatorTetheringClientManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.INetworkOperatorTetheringClientManager";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl INetworkOperatorTetheringClientManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorTetheringClientManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorTetheringClientManager_Vtbl {
        unsafe extern "system" fn GetTetheringClients<Impl: INetworkOperatorTetheringClientManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTetheringClients() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INetworkOperatorTetheringClientManager, BASE_OFFSET>(),
            GetTetheringClients: GetTetheringClients::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorTetheringClientManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorTetheringEntitlementCheck_Impl: Sized {
    fn AuthorizeTethering(&mut self, allow: bool, entitlementfailurereason: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkOperatorTetheringEntitlementCheck {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.INetworkOperatorTetheringEntitlementCheck";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkOperatorTetheringEntitlementCheck_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorTetheringEntitlementCheck_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorTetheringEntitlementCheck_Vtbl {
        unsafe extern "system" fn AuthorizeTethering<Impl: INetworkOperatorTetheringEntitlementCheck_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: bool, entitlementfailurereason: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AuthorizeTethering(allow, &*(&entitlementfailurereason as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INetworkOperatorTetheringEntitlementCheck, BASE_OFFSET>(),
            AuthorizeTethering: AuthorizeTethering::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorTetheringEntitlementCheck as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait INetworkOperatorTetheringManager_Impl: Sized {
    fn MaxClientCount(&mut self) -> ::windows::core::Result<u32>;
    fn ClientCount(&mut self) -> ::windows::core::Result<u32>;
    fn TetheringOperationalState(&mut self) -> ::windows::core::Result<TetheringOperationalState>;
    fn GetCurrentAccessPointConfiguration(&mut self) -> ::windows::core::Result<NetworkOperatorTetheringAccessPointConfiguration>;
    fn ConfigureAccessPointAsync(&mut self, configuration: &::core::option::Option<NetworkOperatorTetheringAccessPointConfiguration>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StartTetheringAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<NetworkOperatorTetheringOperationResult>>;
    fn StopTetheringAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<NetworkOperatorTetheringOperationResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INetworkOperatorTetheringManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.INetworkOperatorTetheringManager";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl INetworkOperatorTetheringManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorTetheringManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorTetheringManager_Vtbl {
        unsafe extern "system" fn MaxClientCount<Impl: INetworkOperatorTetheringManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxClientCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientCount<Impl: INetworkOperatorTetheringManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TetheringOperationalState<Impl: INetworkOperatorTetheringManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TetheringOperationalState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TetheringOperationalState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentAccessPointConfiguration<Impl: INetworkOperatorTetheringManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentAccessPointConfiguration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConfigureAccessPointAsync<Impl: INetworkOperatorTetheringManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConfigureAccessPointAsync(&*(&configuration as *const <NetworkOperatorTetheringAccessPointConfiguration as ::windows::core::Abi>::Abi as *const <NetworkOperatorTetheringAccessPointConfiguration as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTetheringAsync<Impl: INetworkOperatorTetheringManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTetheringAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopTetheringAsync<Impl: INetworkOperatorTetheringManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopTetheringAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INetworkOperatorTetheringManager, BASE_OFFSET>(),
            MaxClientCount: MaxClientCount::<Impl, IMPL_OFFSET>,
            ClientCount: ClientCount::<Impl, IMPL_OFFSET>,
            TetheringOperationalState: TetheringOperationalState::<Impl, IMPL_OFFSET>,
            GetCurrentAccessPointConfiguration: GetCurrentAccessPointConfiguration::<Impl, IMPL_OFFSET>,
            ConfigureAccessPointAsync: ConfigureAccessPointAsync::<Impl, IMPL_OFFSET>,
            StartTetheringAsync: StartTetheringAsync::<Impl, IMPL_OFFSET>,
            StopTetheringAsync: StopTetheringAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorTetheringManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorTetheringManagerStatics_Impl: Sized {
    fn GetTetheringCapability(&mut self, networkaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<TetheringCapability>;
    fn CreateFromNetworkAccountId(&mut self, networkaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<NetworkOperatorTetheringManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkOperatorTetheringManagerStatics {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.INetworkOperatorTetheringManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkOperatorTetheringManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorTetheringManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorTetheringManagerStatics_Vtbl {
        unsafe extern "system" fn GetTetheringCapability<Impl: INetworkOperatorTetheringManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut TetheringCapability) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTetheringCapability(&*(&networkaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromNetworkAccountId<Impl: INetworkOperatorTetheringManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromNetworkAccountId(&*(&networkaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INetworkOperatorTetheringManagerStatics, BASE_OFFSET>(),
            GetTetheringCapability: GetTetheringCapability::<Impl, IMPL_OFFSET>,
            CreateFromNetworkAccountId: CreateFromNetworkAccountId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorTetheringManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Networking_Connectivity", feature = "implement_exclusive"))]
pub trait INetworkOperatorTetheringManagerStatics2_Impl: Sized {
    fn GetTetheringCapabilityFromConnectionProfile(&mut self, profile: &::core::option::Option<super::Connectivity::ConnectionProfile>) -> ::windows::core::Result<TetheringCapability>;
    fn CreateFromConnectionProfile(&mut self, profile: &::core::option::Option<super::Connectivity::ConnectionProfile>) -> ::windows::core::Result<NetworkOperatorTetheringManager>;
}
#[cfg(all(feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INetworkOperatorTetheringManagerStatics2 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.INetworkOperatorTetheringManagerStatics2";
}
#[cfg(all(feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl INetworkOperatorTetheringManagerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorTetheringManagerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorTetheringManagerStatics2_Vtbl {
        unsafe extern "system" fn GetTetheringCapabilityFromConnectionProfile<Impl: INetworkOperatorTetheringManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr, result__: *mut TetheringCapability) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTetheringCapabilityFromConnectionProfile(&*(&profile as *const <super::Connectivity::ConnectionProfile as ::windows::core::Abi>::Abi as *const <super::Connectivity::ConnectionProfile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromConnectionProfile<Impl: INetworkOperatorTetheringManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromConnectionProfile(&*(&profile as *const <super::Connectivity::ConnectionProfile as ::windows::core::Abi>::Abi as *const <super::Connectivity::ConnectionProfile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INetworkOperatorTetheringManagerStatics2, BASE_OFFSET>(),
            GetTetheringCapabilityFromConnectionProfile: GetTetheringCapabilityFromConnectionProfile::<Impl, IMPL_OFFSET>,
            CreateFromConnectionProfile: CreateFromConnectionProfile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorTetheringManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Networking_Connectivity", feature = "implement_exclusive"))]
pub trait INetworkOperatorTetheringManagerStatics3_Impl: Sized {
    fn CreateFromConnectionProfileWithTargetAdapter(&mut self, profile: &::core::option::Option<super::Connectivity::ConnectionProfile>, adapter: &::core::option::Option<super::Connectivity::NetworkAdapter>) -> ::windows::core::Result<NetworkOperatorTetheringManager>;
}
#[cfg(all(feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INetworkOperatorTetheringManagerStatics3 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.INetworkOperatorTetheringManagerStatics3";
}
#[cfg(all(feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl INetworkOperatorTetheringManagerStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorTetheringManagerStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorTetheringManagerStatics3_Vtbl {
        unsafe extern "system" fn CreateFromConnectionProfileWithTargetAdapter<Impl: INetworkOperatorTetheringManagerStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr, adapter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromConnectionProfileWithTargetAdapter(&*(&profile as *const <super::Connectivity::ConnectionProfile as ::windows::core::Abi>::Abi as *const <super::Connectivity::ConnectionProfile as ::windows::core::DefaultType>::DefaultType), &*(&adapter as *const <super::Connectivity::NetworkAdapter as ::windows::core::Abi>::Abi as *const <super::Connectivity::NetworkAdapter as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INetworkOperatorTetheringManagerStatics3, BASE_OFFSET>(),
            CreateFromConnectionProfileWithTargetAdapter: CreateFromConnectionProfileWithTargetAdapter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorTetheringManagerStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait INetworkOperatorTetheringManagerStatics4_Impl: Sized {
    fn IsNoConnectionsTimeoutEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn EnableNoConnectionsTimeout(&mut self) -> ::windows::core::Result<()>;
    fn EnableNoConnectionsTimeoutAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DisableNoConnectionsTimeout(&mut self) -> ::windows::core::Result<()>;
    fn DisableNoConnectionsTimeoutAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INetworkOperatorTetheringManagerStatics4 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.INetworkOperatorTetheringManagerStatics4";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl INetworkOperatorTetheringManagerStatics4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorTetheringManagerStatics4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorTetheringManagerStatics4_Vtbl {
        unsafe extern "system" fn IsNoConnectionsTimeoutEnabled<Impl: INetworkOperatorTetheringManagerStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsNoConnectionsTimeoutEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableNoConnectionsTimeout<Impl: INetworkOperatorTetheringManagerStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableNoConnectionsTimeout().into()
        }
        unsafe extern "system" fn EnableNoConnectionsTimeoutAsync<Impl: INetworkOperatorTetheringManagerStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableNoConnectionsTimeoutAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableNoConnectionsTimeout<Impl: INetworkOperatorTetheringManagerStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableNoConnectionsTimeout().into()
        }
        unsafe extern "system" fn DisableNoConnectionsTimeoutAsync<Impl: INetworkOperatorTetheringManagerStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableNoConnectionsTimeoutAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INetworkOperatorTetheringManagerStatics4, BASE_OFFSET>(),
            IsNoConnectionsTimeoutEnabled: IsNoConnectionsTimeoutEnabled::<Impl, IMPL_OFFSET>,
            EnableNoConnectionsTimeout: EnableNoConnectionsTimeout::<Impl, IMPL_OFFSET>,
            EnableNoConnectionsTimeoutAsync: EnableNoConnectionsTimeoutAsync::<Impl, IMPL_OFFSET>,
            DisableNoConnectionsTimeout: DisableNoConnectionsTimeout::<Impl, IMPL_OFFSET>,
            DisableNoConnectionsTimeoutAsync: DisableNoConnectionsTimeoutAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorTetheringManagerStatics4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorTetheringOperationResult_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<TetheringOperationStatus>;
    fn AdditionalErrorMessage(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkOperatorTetheringOperationResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.INetworkOperatorTetheringOperationResult";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkOperatorTetheringOperationResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorTetheringOperationResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorTetheringOperationResult_Vtbl {
        unsafe extern "system" fn Status<Impl: INetworkOperatorTetheringOperationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TetheringOperationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AdditionalErrorMessage<Impl: INetworkOperatorTetheringOperationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdditionalErrorMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INetworkOperatorTetheringOperationResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            AdditionalErrorMessage: AdditionalErrorMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorTetheringOperationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProvisionFromXmlDocumentResults_Impl: Sized {
    fn AllElementsProvisioned(&mut self) -> ::windows::core::Result<bool>;
    fn ProvisionResultsXml(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProvisionFromXmlDocumentResults {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IProvisionFromXmlDocumentResults";
}
#[cfg(feature = "implement_exclusive")]
impl IProvisionFromXmlDocumentResults_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvisionFromXmlDocumentResults_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProvisionFromXmlDocumentResults_Vtbl {
        unsafe extern "system" fn AllElementsProvisioned<Impl: IProvisionFromXmlDocumentResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllElementsProvisioned() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProvisionResultsXml<Impl: IProvisionFromXmlDocumentResults_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProvisionResultsXml() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProvisionFromXmlDocumentResults, BASE_OFFSET>(),
            AllElementsProvisioned: AllElementsProvisioned::<Impl, IMPL_OFFSET>,
            ProvisionResultsXml: ProvisionResultsXml::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvisionFromXmlDocumentResults as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
pub trait IProvisionedProfile_Impl: Sized {
    fn UpdateCost(&mut self, value: super::Connectivity::NetworkCostType) -> ::windows::core::Result<()>;
    fn UpdateUsage(&mut self, value: &ProfileUsage) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProvisionedProfile {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IProvisionedProfile";
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl IProvisionedProfile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvisionedProfile_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProvisionedProfile_Vtbl {
        unsafe extern "system" fn UpdateCost<Impl: IProvisionedProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Connectivity::NetworkCostType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateCost(value).into()
        }
        unsafe extern "system" fn UpdateUsage<Impl: IProvisionedProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ProfileUsage) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateUsage(&*(&value as *const <ProfileUsage as ::windows::core::Abi>::Abi as *const <ProfileUsage as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProvisionedProfile, BASE_OFFSET>(),
            UpdateCost: UpdateCost::<Impl, IMPL_OFFSET>,
            UpdateUsage: UpdateUsage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvisionedProfile as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IProvisioningAgent_Impl: Sized {
    fn ProvisionFromXmlDocumentAsync(&mut self, provisioningxmldocument: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProvisionFromXmlDocumentResults>>;
    fn GetProvisionedProfile(&mut self, mediatype: ProfileMediaType, profilename: &::windows::core::HSTRING) -> ::windows::core::Result<ProvisionedProfile>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProvisioningAgent {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IProvisioningAgent";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IProvisioningAgent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvisioningAgent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProvisioningAgent_Vtbl {
        unsafe extern "system" fn ProvisionFromXmlDocumentAsync<Impl: IProvisioningAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provisioningxmldocument: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProvisionFromXmlDocumentAsync(&*(&provisioningxmldocument as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProvisionedProfile<Impl: IProvisioningAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediatype: ProfileMediaType, profilename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProvisionedProfile(mediatype, &*(&profilename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProvisioningAgent, BASE_OFFSET>(),
            ProvisionFromXmlDocumentAsync: ProvisionFromXmlDocumentAsync::<Impl, IMPL_OFFSET>,
            GetProvisionedProfile: GetProvisionedProfile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvisioningAgent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProvisioningAgentStaticMethods_Impl: Sized {
    fn CreateFromNetworkAccountId(&mut self, networkaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<ProvisioningAgent>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProvisioningAgentStaticMethods {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IProvisioningAgentStaticMethods";
}
#[cfg(feature = "implement_exclusive")]
impl IProvisioningAgentStaticMethods_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvisioningAgentStaticMethods_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProvisioningAgentStaticMethods_Vtbl {
        unsafe extern "system" fn CreateFromNetworkAccountId<Impl: IProvisioningAgentStaticMethods_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromNetworkAccountId(&*(&networkaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProvisioningAgentStaticMethods, BASE_OFFSET>(),
            CreateFromNetworkAccountId: CreateFromNetworkAccountId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvisioningAgentStaticMethods as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITetheringEntitlementCheckTriggerDetails_Impl: Sized {
    fn NetworkAccountId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AllowTethering(&mut self) -> ::windows::core::Result<()>;
    fn DenyTethering(&mut self, entitlementfailurereason: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITetheringEntitlementCheckTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ITetheringEntitlementCheckTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl ITetheringEntitlementCheckTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITetheringEntitlementCheckTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITetheringEntitlementCheckTriggerDetails_Vtbl {
        unsafe extern "system" fn NetworkAccountId<Impl: ITetheringEntitlementCheckTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkAccountId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowTethering<Impl: ITetheringEntitlementCheckTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AllowTethering().into()
        }
        unsafe extern "system" fn DenyTethering<Impl: ITetheringEntitlementCheckTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entitlementfailurereason: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DenyTethering(&*(&entitlementfailurereason as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITetheringEntitlementCheckTriggerDetails, BASE_OFFSET>(),
            NetworkAccountId: NetworkAccountId::<Impl, IMPL_OFFSET>,
            AllowTethering: AllowTethering::<Impl, IMPL_OFFSET>,
            DenyTethering: DenyTethering::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITetheringEntitlementCheckTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUssdMessage_Impl: Sized {
    fn DataCodingScheme(&mut self) -> ::windows::core::Result<u8>;
    fn SetDataCodingScheme(&mut self, value: u8) -> ::windows::core::Result<()>;
    fn GetPayload(&mut self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn SetPayload(&mut self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn PayloadAsText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPayloadAsText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUssdMessage {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IUssdMessage";
}
#[cfg(feature = "implement_exclusive")]
impl IUssdMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUssdMessage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUssdMessage_Vtbl {
        unsafe extern "system" fn DataCodingScheme<Impl: IUssdMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataCodingScheme() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataCodingScheme<Impl: IUssdMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataCodingScheme(value).into()
        }
        unsafe extern "system" fn GetPayload<Impl: IUssdMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPayload() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPayload<Impl: IUssdMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPayload(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn PayloadAsText<Impl: IUssdMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PayloadAsText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPayloadAsText<Impl: IUssdMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPayloadAsText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUssdMessage, BASE_OFFSET>(),
            DataCodingScheme: DataCodingScheme::<Impl, IMPL_OFFSET>,
            SetDataCodingScheme: SetDataCodingScheme::<Impl, IMPL_OFFSET>,
            GetPayload: GetPayload::<Impl, IMPL_OFFSET>,
            SetPayload: SetPayload::<Impl, IMPL_OFFSET>,
            PayloadAsText: PayloadAsText::<Impl, IMPL_OFFSET>,
            SetPayloadAsText: SetPayloadAsText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUssdMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUssdMessageFactory_Impl: Sized {
    fn CreateMessage(&mut self, messagetext: &::windows::core::HSTRING) -> ::windows::core::Result<UssdMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUssdMessageFactory {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IUssdMessageFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IUssdMessageFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUssdMessageFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUssdMessageFactory_Vtbl {
        unsafe extern "system" fn CreateMessage<Impl: IUssdMessageFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagetext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateMessage(&*(&messagetext as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IUssdMessageFactory, BASE_OFFSET>(), CreateMessage: CreateMessage::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUssdMessageFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUssdReply_Impl: Sized {
    fn ResultCode(&mut self) -> ::windows::core::Result<UssdResultCode>;
    fn Message(&mut self) -> ::windows::core::Result<UssdMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUssdReply {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IUssdReply";
}
#[cfg(feature = "implement_exclusive")]
impl IUssdReply_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUssdReply_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUssdReply_Vtbl {
        unsafe extern "system" fn ResultCode<Impl: IUssdReply_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UssdResultCode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResultCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Impl: IUssdReply_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUssdReply, BASE_OFFSET>(),
            ResultCode: ResultCode::<Impl, IMPL_OFFSET>,
            Message: Message::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUssdReply as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUssdSession_Impl: Sized {
    fn SendMessageAndGetReplyAsync(&mut self, message: &::core::option::Option<UssdMessage>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UssdReply>>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUssdSession {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IUssdSession";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUssdSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUssdSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUssdSession_Vtbl {
        unsafe extern "system" fn SendMessageAndGetReplyAsync<Impl: IUssdSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendMessageAndGetReplyAsync(&*(&message as *const <UssdMessage as ::windows::core::Abi>::Abi as *const <UssdMessage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IUssdSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUssdSession, BASE_OFFSET>(),
            SendMessageAndGetReplyAsync: SendMessageAndGetReplyAsync::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUssdSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUssdSessionStatics_Impl: Sized {
    fn CreateFromNetworkAccountId(&mut self, networkaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<UssdSession>;
    fn CreateFromNetworkInterfaceId(&mut self, networkinterfaceid: &::windows::core::HSTRING) -> ::windows::core::Result<UssdSession>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUssdSessionStatics {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IUssdSessionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUssdSessionStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUssdSessionStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUssdSessionStatics_Vtbl {
        unsafe extern "system" fn CreateFromNetworkAccountId<Impl: IUssdSessionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromNetworkAccountId(&*(&networkaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromNetworkInterfaceId<Impl: IUssdSessionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkinterfaceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromNetworkInterfaceId(&*(&networkinterfaceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUssdSessionStatics, BASE_OFFSET>(),
            CreateFromNetworkAccountId: CreateFromNetworkAccountId::<Impl, IMPL_OFFSET>,
            CreateFromNetworkInterfaceId: CreateFromNetworkInterfaceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUssdSessionStatics as ::windows::core::Interface>::IID
    }
}
