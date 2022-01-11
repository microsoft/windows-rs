#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IESimImpl: Sized {
    fn AvailableMemoryInBytes(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn Eid(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FirmwareVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MobileBroadbandModemDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Policy(&self) -> ::windows::core::Result<ESimPolicy>;
    fn State(&self) -> ::windows::core::Result<ESimState>;
    fn GetProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ESimProfile>>;
    fn DeleteProfileAsync(&self, profileid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>>;
    fn DownloadProfileMetadataAsync(&self, activationcode: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimDownloadProfileMetadataResult>>;
    fn ResetAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>>;
    fn ProfileChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ESim, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProfileChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IESim {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESim";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IESimVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimVtbl {
        unsafe extern "system" fn AvailableMemoryInBytes<Impl: IESimImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Eid<Impl: IESimImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FirmwareVersion<Impl: IESimImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MobileBroadbandModemDeviceId<Impl: IESimImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Policy<Impl: IESimImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn State<Impl: IESimImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ESimState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetProfiles<Impl: IESimImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeleteProfileAsync<Impl: IESimImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profileid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DownloadProfileMetadataAsync<Impl: IESimImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activationcode: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResetAsync<Impl: IESimImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProfileChanged<Impl: IESimImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveProfileChanged<Impl: IESimImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveProfileChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IESim>,
            ::windows::core::GetTrustLevel,
            AvailableMemoryInBytes::<Impl, IMPL_OFFSET>,
            Eid::<Impl, IMPL_OFFSET>,
            FirmwareVersion::<Impl, IMPL_OFFSET>,
            MobileBroadbandModemDeviceId::<Impl, IMPL_OFFSET>,
            Policy::<Impl, IMPL_OFFSET>,
            State::<Impl, IMPL_OFFSET>,
            GetProfiles::<Impl, IMPL_OFFSET>,
            DeleteProfileAsync::<Impl, IMPL_OFFSET>,
            DownloadProfileMetadataAsync::<Impl, IMPL_OFFSET>,
            ResetAsync::<Impl, IMPL_OFFSET>,
            ProfileChanged::<Impl, IMPL_OFFSET>,
            RemoveProfileChanged::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESim as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IESim2Impl: Sized {
    fn Discover(&self) -> ::windows::core::Result<ESimDiscoverResult>;
    fn DiscoverWithServerAddressAndMatchingId(&self, serveraddress: &::windows::core::HSTRING, matchingid: &::windows::core::HSTRING) -> ::windows::core::Result<ESimDiscoverResult>;
    fn DiscoverAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimDiscoverResult>>;
    fn DiscoverWithServerAddressAndMatchingIdAsync(&self, serveraddress: &::windows::core::HSTRING, matchingid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimDiscoverResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IESim2 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESim2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IESim2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESim2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESim2Vtbl {
        unsafe extern "system" fn Discover<Impl: IESim2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DiscoverWithServerAddressAndMatchingId<Impl: IESim2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serveraddress: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, matchingid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DiscoverAsync<Impl: IESim2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DiscoverWithServerAddressAndMatchingIdAsync<Impl: IESim2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serveraddress: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, matchingid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IESim2>, ::windows::core::GetTrustLevel, Discover::<Impl, IMPL_OFFSET>, DiscoverWithServerAddressAndMatchingId::<Impl, IMPL_OFFSET>, DiscoverAsync::<Impl, IMPL_OFFSET>, DiscoverWithServerAddressAndMatchingIdAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESim2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimAddedEventArgsImpl: Sized {
    fn ESim(&self) -> ::windows::core::Result<ESim>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IESimAddedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimAddedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IESimAddedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimAddedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimAddedEventArgsVtbl {
        unsafe extern "system" fn ESim<Impl: IESimAddedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IESimAddedEventArgs>, ::windows::core::GetTrustLevel, ESim::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimAddedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimDiscoverEventImpl: Sized {
    fn MatchingId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RspServerAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IESimDiscoverEvent {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimDiscoverEvent";
}
#[cfg(feature = "implement_exclusive")]
impl IESimDiscoverEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimDiscoverEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimDiscoverEventVtbl {
        unsafe extern "system" fn MatchingId<Impl: IESimDiscoverEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RspServerAddress<Impl: IESimDiscoverEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IESimDiscoverEvent>, ::windows::core::GetTrustLevel, MatchingId::<Impl, IMPL_OFFSET>, RspServerAddress::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimDiscoverEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IESimDiscoverResultImpl: Sized {
    fn Events(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ESimDiscoverEvent>>;
    fn Kind(&self) -> ::windows::core::Result<ESimDiscoverResultKind>;
    fn ProfileMetadata(&self) -> ::windows::core::Result<ESimProfileMetadata>;
    fn Result(&self) -> ::windows::core::Result<ESimOperationResult>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IESimDiscoverResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimDiscoverResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IESimDiscoverResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimDiscoverResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimDiscoverResultVtbl {
        unsafe extern "system" fn Events<Impl: IESimDiscoverResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Kind<Impl: IESimDiscoverResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ESimDiscoverResultKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProfileMetadata<Impl: IESimDiscoverResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Result<Impl: IESimDiscoverResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IESimDiscoverResult>, ::windows::core::GetTrustLevel, Events::<Impl, IMPL_OFFSET>, Kind::<Impl, IMPL_OFFSET>, ProfileMetadata::<Impl, IMPL_OFFSET>, Result::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimDiscoverResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimDownloadProfileMetadataResultImpl: Sized {
    fn Result(&self) -> ::windows::core::Result<ESimOperationResult>;
    fn ProfileMetadata(&self) -> ::windows::core::Result<ESimProfileMetadata>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IESimDownloadProfileMetadataResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimDownloadProfileMetadataResult";
}
#[cfg(feature = "implement_exclusive")]
impl IESimDownloadProfileMetadataResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimDownloadProfileMetadataResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimDownloadProfileMetadataResultVtbl {
        unsafe extern "system" fn Result<Impl: IESimDownloadProfileMetadataResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProfileMetadata<Impl: IESimDownloadProfileMetadataResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IESimDownloadProfileMetadataResult>, ::windows::core::GetTrustLevel, Result::<Impl, IMPL_OFFSET>, ProfileMetadata::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimDownloadProfileMetadataResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IESimManagerStaticsImpl: Sized {
    fn ServiceInfo(&self) -> ::windows::core::Result<ESimServiceInfo>;
    fn TryCreateESimWatcher(&self) -> ::windows::core::Result<ESimWatcher>;
    fn ServiceInfoChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveServiceInfoChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IESimManagerStatics {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IESimManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimManagerStaticsVtbl {
        unsafe extern "system" fn ServiceInfo<Impl: IESimManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryCreateESimWatcher<Impl: IESimManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ServiceInfoChanged<Impl: IESimManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveServiceInfoChanged<Impl: IESimManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveServiceInfoChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IESimManagerStatics>, ::windows::core::GetTrustLevel, ServiceInfo::<Impl, IMPL_OFFSET>, TryCreateESimWatcher::<Impl, IMPL_OFFSET>, ServiceInfoChanged::<Impl, IMPL_OFFSET>, RemoveServiceInfoChanged::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimOperationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<ESimOperationStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IESimOperationResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimOperationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IESimOperationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimOperationResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimOperationResultVtbl {
        unsafe extern "system" fn Status<Impl: IESimOperationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ESimOperationStatus) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IESimOperationResult>, ::windows::core::GetTrustLevel, Status::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimOperationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimPolicyImpl: Sized {
    fn ShouldEnableManagingUi(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IESimPolicy {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimPolicy";
}
#[cfg(feature = "implement_exclusive")]
impl IESimPolicyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimPolicyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimPolicyVtbl {
        unsafe extern "system" fn ShouldEnableManagingUi<Impl: IESimPolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IESimPolicy>, ::windows::core::GetTrustLevel, ShouldEnableManagingUi::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimPolicy as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IESimProfileImpl: Sized {
    fn Class(&self) -> ::windows::core::Result<ESimProfileClass>;
    fn Nickname(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Policy(&self) -> ::windows::core::Result<ESimProfilePolicy>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProviderIcon(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn State(&self) -> ::windows::core::Result<ESimProfileState>;
    fn DisableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>>;
    fn EnableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>>;
    fn SetNicknameAsync(&self, newnickname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IESimProfile {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimProfile";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IESimProfileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimProfileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimProfileVtbl {
        unsafe extern "system" fn Class<Impl: IESimProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ESimProfileClass) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Nickname<Impl: IESimProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Policy<Impl: IESimProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Id<Impl: IESimProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProviderIcon<Impl: IESimProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProviderId<Impl: IESimProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProviderName<Impl: IESimProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn State<Impl: IESimProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ESimProfileState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisableAsync<Impl: IESimProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EnableAsync<Impl: IESimProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetNicknameAsync<Impl: IESimProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newnickname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IESimProfile>,
            ::windows::core::GetTrustLevel,
            Class::<Impl, IMPL_OFFSET>,
            Nickname::<Impl, IMPL_OFFSET>,
            Policy::<Impl, IMPL_OFFSET>,
            Id::<Impl, IMPL_OFFSET>,
            ProviderIcon::<Impl, IMPL_OFFSET>,
            ProviderId::<Impl, IMPL_OFFSET>,
            ProviderName::<Impl, IMPL_OFFSET>,
            State::<Impl, IMPL_OFFSET>,
            DisableAsync::<Impl, IMPL_OFFSET>,
            EnableAsync::<Impl, IMPL_OFFSET>,
            SetNicknameAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimProfile as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IESimProfileMetadataImpl: Sized {
    fn IsConfirmationCodeRequired(&self) -> ::windows::core::Result<bool>;
    fn Policy(&self) -> ::windows::core::Result<ESimProfilePolicy>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProviderIcon(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn State(&self) -> ::windows::core::Result<ESimProfileMetadataState>;
    fn DenyInstallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>>;
    fn ConfirmInstallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<ESimOperationResult, ESimProfileInstallProgress>>;
    fn ConfirmInstallWithConfirmationCodeAsync(&self, confirmationcode: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<ESimOperationResult, ESimProfileInstallProgress>>;
    fn PostponeInstallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ESimOperationResult>>;
    fn StateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ESimProfileMetadata, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IESimProfileMetadata {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimProfileMetadata";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IESimProfileMetadataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimProfileMetadataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimProfileMetadataVtbl {
        unsafe extern "system" fn IsConfirmationCodeRequired<Impl: IESimProfileMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Policy<Impl: IESimProfileMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Id<Impl: IESimProfileMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProviderIcon<Impl: IESimProfileMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProviderId<Impl: IESimProfileMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProviderName<Impl: IESimProfileMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn State<Impl: IESimProfileMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ESimProfileMetadataState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DenyInstallAsync<Impl: IESimProfileMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConfirmInstallAsync<Impl: IESimProfileMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConfirmInstallWithConfirmationCodeAsync<Impl: IESimProfileMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confirmationcode: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PostponeInstallAsync<Impl: IESimProfileMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StateChanged<Impl: IESimProfileMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveStateChanged<Impl: IESimProfileMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IESimProfileMetadata>,
            ::windows::core::GetTrustLevel,
            IsConfirmationCodeRequired::<Impl, IMPL_OFFSET>,
            Policy::<Impl, IMPL_OFFSET>,
            Id::<Impl, IMPL_OFFSET>,
            ProviderIcon::<Impl, IMPL_OFFSET>,
            ProviderId::<Impl, IMPL_OFFSET>,
            ProviderName::<Impl, IMPL_OFFSET>,
            State::<Impl, IMPL_OFFSET>,
            DenyInstallAsync::<Impl, IMPL_OFFSET>,
            ConfirmInstallAsync::<Impl, IMPL_OFFSET>,
            ConfirmInstallWithConfirmationCodeAsync::<Impl, IMPL_OFFSET>,
            PostponeInstallAsync::<Impl, IMPL_OFFSET>,
            StateChanged::<Impl, IMPL_OFFSET>,
            RemoveStateChanged::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimProfileMetadata as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimProfilePolicyImpl: Sized {
    fn CanDelete(&self) -> ::windows::core::Result<bool>;
    fn CanDisable(&self) -> ::windows::core::Result<bool>;
    fn IsManagedByEnterprise(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IESimProfilePolicy {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimProfilePolicy";
}
#[cfg(feature = "implement_exclusive")]
impl IESimProfilePolicyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimProfilePolicyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimProfilePolicyVtbl {
        unsafe extern "system" fn CanDelete<Impl: IESimProfilePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CanDisable<Impl: IESimProfilePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsManagedByEnterprise<Impl: IESimProfilePolicyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IESimProfilePolicy>, ::windows::core::GetTrustLevel, CanDelete::<Impl, IMPL_OFFSET>, CanDisable::<Impl, IMPL_OFFSET>, IsManagedByEnterprise::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimProfilePolicy as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimRemovedEventArgsImpl: Sized {
    fn ESim(&self) -> ::windows::core::Result<ESim>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IESimRemovedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimRemovedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IESimRemovedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimRemovedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimRemovedEventArgsVtbl {
        unsafe extern "system" fn ESim<Impl: IESimRemovedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IESimRemovedEventArgs>, ::windows::core::GetTrustLevel, ESim::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimRemovedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimServiceInfoImpl: Sized {
    fn AuthenticationPreference(&self) -> ::windows::core::Result<ESimAuthenticationPreference>;
    fn IsESimUiEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IESimServiceInfo {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimServiceInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IESimServiceInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimServiceInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimServiceInfoVtbl {
        unsafe extern "system" fn AuthenticationPreference<Impl: IESimServiceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ESimAuthenticationPreference) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsESimUiEnabled<Impl: IESimServiceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IESimServiceInfo>, ::windows::core::GetTrustLevel, AuthenticationPreference::<Impl, IMPL_OFFSET>, IsESimUiEnabled::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimServiceInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IESimUpdatedEventArgsImpl: Sized {
    fn ESim(&self) -> ::windows::core::Result<ESim>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IESimUpdatedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimUpdatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IESimUpdatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimUpdatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimUpdatedEventArgsVtbl {
        unsafe extern "system" fn ESim<Impl: IESimUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IESimUpdatedEventArgs>, ::windows::core::GetTrustLevel, ESim::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimUpdatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IESimWatcherImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<ESimWatcherStatus>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Added(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ESimWatcher, ESimAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ESimWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ESimWatcher, ESimRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ESimWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Updated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ESimWatcher, ESimUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IESimWatcher {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IESimWatcher";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IESimWatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IESimWatcherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IESimWatcherVtbl {
        unsafe extern "system" fn Status<Impl: IESimWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ESimWatcherStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Start<Impl: IESimWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IESimWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Added<Impl: IESimWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveAdded<Impl: IESimWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnumerationCompleted<Impl: IESimWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveEnumerationCompleted<Impl: IESimWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnumerationCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Removed<Impl: IESimWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveRemoved<Impl: IESimWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stopped<Impl: IESimWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveStopped<Impl: IESimWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStopped(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Updated<Impl: IESimWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveUpdated<Impl: IESimWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUpdated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IESimWatcher>,
            ::windows::core::GetTrustLevel,
            Status::<Impl, IMPL_OFFSET>,
            Start::<Impl, IMPL_OFFSET>,
            Stop::<Impl, IMPL_OFFSET>,
            Added::<Impl, IMPL_OFFSET>,
            RemoveAdded::<Impl, IMPL_OFFSET>,
            EnumerationCompleted::<Impl, IMPL_OFFSET>,
            RemoveEnumerationCompleted::<Impl, IMPL_OFFSET>,
            Removed::<Impl, IMPL_OFFSET>,
            RemoveRemoved::<Impl, IMPL_OFFSET>,
            Stopped::<Impl, IMPL_OFFSET>,
            RemoveStopped::<Impl, IMPL_OFFSET>,
            Updated::<Impl, IMPL_OFFSET>,
            RemoveUpdated::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IESimWatcher as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IFdnAccessManagerStaticsImpl: Sized {
    fn RequestUnlockAsync(&self, contactlistid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFdnAccessManagerStatics {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IFdnAccessManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IFdnAccessManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFdnAccessManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFdnAccessManagerStaticsVtbl {
        unsafe extern "system" fn RequestUnlockAsync<Impl: IFdnAccessManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contactlistid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFdnAccessManagerStatics>, ::windows::core::GetTrustLevel, RequestUnlockAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFdnAccessManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
pub trait IHotspotAuthenticationContextImpl: Sized {
    fn WirelessNetworkId(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn NetworkAdapter(&self) -> ::windows::core::Result<super::Connectivity::NetworkAdapter>;
    fn RedirectMessageUrl(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn RedirectMessageXml(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
    fn AuthenticationUrl(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn IssueCredentials(&self, username: &::windows::core::HSTRING, password: &::windows::core::HSTRING, extraparameters: &::windows::core::HSTRING, markasmanualconnectonfailure: bool) -> ::windows::core::Result<()>;
    fn AbortAuthentication(&self, markasmanual: bool) -> ::windows::core::Result<()>;
    fn SkipAuthentication(&self) -> ::windows::core::Result<()>;
    fn TriggerAttentionRequired(&self, packagerelativeapplicationid: &::windows::core::HSTRING, applicationparameters: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHotspotAuthenticationContext {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IHotspotAuthenticationContext";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl IHotspotAuthenticationContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHotspotAuthenticationContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHotspotAuthenticationContextVtbl {
        unsafe extern "system" fn WirelessNetworkId<Impl: IHotspotAuthenticationContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NetworkAdapter<Impl: IHotspotAuthenticationContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RedirectMessageUrl<Impl: IHotspotAuthenticationContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RedirectMessageXml<Impl: IHotspotAuthenticationContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AuthenticationUrl<Impl: IHotspotAuthenticationContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IssueCredentials<Impl: IHotspotAuthenticationContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, extraparameters: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, markasmanualconnectonfailure: bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AbortAuthentication<Impl: IHotspotAuthenticationContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, markasmanual: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AbortAuthentication(markasmanual).into()
        }
        unsafe extern "system" fn SkipAuthentication<Impl: IHotspotAuthenticationContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SkipAuthentication().into()
        }
        unsafe extern "system" fn TriggerAttentionRequired<Impl: IHotspotAuthenticationContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagerelativeapplicationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, applicationparameters: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TriggerAttentionRequired(&*(&packagerelativeapplicationid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&applicationparameters as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHotspotAuthenticationContext>,
            ::windows::core::GetTrustLevel,
            WirelessNetworkId::<Impl, IMPL_OFFSET>,
            NetworkAdapter::<Impl, IMPL_OFFSET>,
            RedirectMessageUrl::<Impl, IMPL_OFFSET>,
            RedirectMessageXml::<Impl, IMPL_OFFSET>,
            AuthenticationUrl::<Impl, IMPL_OFFSET>,
            IssueCredentials::<Impl, IMPL_OFFSET>,
            AbortAuthentication::<Impl, IMPL_OFFSET>,
            SkipAuthentication::<Impl, IMPL_OFFSET>,
            TriggerAttentionRequired::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHotspotAuthenticationContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHotspotAuthenticationContext2Impl: Sized {
    fn IssueCredentialsAsync(&self, username: &::windows::core::HSTRING, password: &::windows::core::HSTRING, extraparameters: &::windows::core::HSTRING, markasmanualconnectonfailure: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HotspotCredentialsAuthenticationResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHotspotAuthenticationContext2 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IHotspotAuthenticationContext2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHotspotAuthenticationContext2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHotspotAuthenticationContext2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHotspotAuthenticationContext2Vtbl {
        unsafe extern "system" fn IssueCredentialsAsync<Impl: IHotspotAuthenticationContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, extraparameters: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, markasmanualconnectonfailure: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHotspotAuthenticationContext2>, ::windows::core::GetTrustLevel, IssueCredentialsAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHotspotAuthenticationContext2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHotspotAuthenticationContextStaticsImpl: Sized {
    fn TryGetAuthenticationContext(&self, eventoken: &::windows::core::HSTRING, context: &mut ::core::option::Option<HotspotAuthenticationContext>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHotspotAuthenticationContextStatics {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IHotspotAuthenticationContextStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHotspotAuthenticationContextStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHotspotAuthenticationContextStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHotspotAuthenticationContextStaticsVtbl {
        unsafe extern "system" fn TryGetAuthenticationContext<Impl: IHotspotAuthenticationContextStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventoken: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, context: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHotspotAuthenticationContextStatics>, ::windows::core::GetTrustLevel, TryGetAuthenticationContext::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHotspotAuthenticationContextStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHotspotAuthenticationEventDetailsImpl: Sized {
    fn EventToken(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHotspotAuthenticationEventDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IHotspotAuthenticationEventDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IHotspotAuthenticationEventDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHotspotAuthenticationEventDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHotspotAuthenticationEventDetailsVtbl {
        unsafe extern "system" fn EventToken<Impl: IHotspotAuthenticationEventDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHotspotAuthenticationEventDetails>, ::windows::core::GetTrustLevel, EventToken::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHotspotAuthenticationEventDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHotspotCredentialsAuthenticationResultImpl: Sized {
    fn HasNetworkErrorOccurred(&self) -> ::windows::core::Result<bool>;
    fn ResponseCode(&self) -> ::windows::core::Result<HotspotAuthenticationResponseCode>;
    fn LogoffUrl(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn AuthenticationReplyXml(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHotspotCredentialsAuthenticationResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IHotspotCredentialsAuthenticationResult";
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation", feature = "implement_exclusive"))]
impl IHotspotCredentialsAuthenticationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHotspotCredentialsAuthenticationResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHotspotCredentialsAuthenticationResultVtbl {
        unsafe extern "system" fn HasNetworkErrorOccurred<Impl: IHotspotCredentialsAuthenticationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResponseCode<Impl: IHotspotCredentialsAuthenticationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HotspotAuthenticationResponseCode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LogoffUrl<Impl: IHotspotCredentialsAuthenticationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AuthenticationReplyXml<Impl: IHotspotCredentialsAuthenticationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHotspotCredentialsAuthenticationResult>, ::windows::core::GetTrustLevel, HasNetworkErrorOccurred::<Impl, IMPL_OFFSET>, ResponseCode::<Impl, IMPL_OFFSET>, LogoffUrl::<Impl, IMPL_OFFSET>, AuthenticationReplyXml::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHotspotCredentialsAuthenticationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IKnownCSimFilePathsStaticsImpl: Sized {
    fn EFSpn(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn Gid1(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn Gid2(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKnownCSimFilePathsStatics {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IKnownCSimFilePathsStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IKnownCSimFilePathsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownCSimFilePathsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownCSimFilePathsStaticsVtbl {
        unsafe extern "system" fn EFSpn<Impl: IKnownCSimFilePathsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Gid1<Impl: IKnownCSimFilePathsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Gid2<Impl: IKnownCSimFilePathsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IKnownCSimFilePathsStatics>, ::windows::core::GetTrustLevel, EFSpn::<Impl, IMPL_OFFSET>, Gid1::<Impl, IMPL_OFFSET>, Gid2::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownCSimFilePathsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IKnownRuimFilePathsStaticsImpl: Sized {
    fn EFSpn(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn Gid1(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn Gid2(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKnownRuimFilePathsStatics {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IKnownRuimFilePathsStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IKnownRuimFilePathsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownRuimFilePathsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownRuimFilePathsStaticsVtbl {
        unsafe extern "system" fn EFSpn<Impl: IKnownRuimFilePathsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Gid1<Impl: IKnownRuimFilePathsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Gid2<Impl: IKnownRuimFilePathsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IKnownRuimFilePathsStatics>, ::windows::core::GetTrustLevel, EFSpn::<Impl, IMPL_OFFSET>, Gid1::<Impl, IMPL_OFFSET>, Gid2::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownRuimFilePathsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IKnownSimFilePathsStaticsImpl: Sized {
    fn EFOns(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn EFSpn(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn Gid1(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn Gid2(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKnownSimFilePathsStatics {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IKnownSimFilePathsStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IKnownSimFilePathsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownSimFilePathsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownSimFilePathsStaticsVtbl {
        unsafe extern "system" fn EFOns<Impl: IKnownSimFilePathsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EFSpn<Impl: IKnownSimFilePathsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Gid1<Impl: IKnownSimFilePathsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Gid2<Impl: IKnownSimFilePathsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IKnownSimFilePathsStatics>, ::windows::core::GetTrustLevel, EFOns::<Impl, IMPL_OFFSET>, EFSpn::<Impl, IMPL_OFFSET>, Gid1::<Impl, IMPL_OFFSET>, Gid2::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownSimFilePathsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IKnownUSimFilePathsStaticsImpl: Sized {
    fn EFSpn(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn EFOpl(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn EFPnn(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn Gid1(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn Gid2(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKnownUSimFilePathsStatics {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IKnownUSimFilePathsStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IKnownUSimFilePathsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownUSimFilePathsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownUSimFilePathsStaticsVtbl {
        unsafe extern "system" fn EFSpn<Impl: IKnownUSimFilePathsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EFOpl<Impl: IKnownUSimFilePathsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EFPnn<Impl: IKnownUSimFilePathsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Gid1<Impl: IKnownUSimFilePathsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Gid2<Impl: IKnownUSimFilePathsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IKnownUSimFilePathsStatics>, ::windows::core::GetTrustLevel, EFSpn::<Impl, IMPL_OFFSET>, EFOpl::<Impl, IMPL_OFFSET>, EFPnn::<Impl, IMPL_OFFSET>, Gid1::<Impl, IMPL_OFFSET>, Gid2::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownUSimFilePathsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandAccountImpl: Sized {
    fn NetworkAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceProviderGuid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ServiceProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CurrentNetwork(&self) -> ::windows::core::Result<MobileBroadbandNetwork>;
    fn CurrentDeviceInformation(&self) -> ::windows::core::Result<MobileBroadbandDeviceInformation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandAccount {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandAccount";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandAccountVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandAccountImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandAccountVtbl {
        unsafe extern "system" fn NetworkAccountId<Impl: IMobileBroadbandAccountImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ServiceProviderGuid<Impl: IMobileBroadbandAccountImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ServiceProviderName<Impl: IMobileBroadbandAccountImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CurrentNetwork<Impl: IMobileBroadbandAccountImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CurrentDeviceInformation<Impl: IMobileBroadbandAccountImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMobileBroadbandAccount>,
            ::windows::core::GetTrustLevel,
            NetworkAccountId::<Impl, IMPL_OFFSET>,
            ServiceProviderGuid::<Impl, IMPL_OFFSET>,
            ServiceProviderName::<Impl, IMPL_OFFSET>,
            CurrentNetwork::<Impl, IMPL_OFFSET>,
            CurrentDeviceInformation::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandAccount as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
pub trait IMobileBroadbandAccount2Impl: Sized {
    fn GetConnectionProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::Connectivity::ConnectionProfile>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandAccount2 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandAccount2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl IMobileBroadbandAccount2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandAccount2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandAccount2Vtbl {
        unsafe extern "system" fn GetConnectionProfiles<Impl: IMobileBroadbandAccount2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandAccount2>, ::windows::core::GetTrustLevel, GetConnectionProfiles::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandAccount2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandAccount3Impl: Sized {
    fn AccountExperienceUrl(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandAccount3 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandAccount3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandAccount3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandAccount3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandAccount3Vtbl {
        unsafe extern "system" fn AccountExperienceUrl<Impl: IMobileBroadbandAccount3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandAccount3>, ::windows::core::GetTrustLevel, AccountExperienceUrl::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandAccount3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandAccountEventArgsImpl: Sized {
    fn NetworkAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandAccountEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandAccountEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandAccountEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandAccountEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandAccountEventArgsVtbl {
        unsafe extern "system" fn NetworkAccountId<Impl: IMobileBroadbandAccountEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandAccountEventArgs>, ::windows::core::GetTrustLevel, NetworkAccountId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandAccountEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandAccountStaticsImpl: Sized {
    fn AvailableNetworkAccountIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn CreateFromNetworkAccountId(&self, networkaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<MobileBroadbandAccount>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandAccountStatics {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandAccountStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandAccountStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandAccountStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandAccountStaticsVtbl {
        unsafe extern "system" fn AvailableNetworkAccountIds<Impl: IMobileBroadbandAccountStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromNetworkAccountId<Impl: IMobileBroadbandAccountStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandAccountStatics>, ::windows::core::GetTrustLevel, AvailableNetworkAccountIds::<Impl, IMPL_OFFSET>, CreateFromNetworkAccountId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandAccountStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandAccountUpdatedEventArgsImpl: Sized {
    fn NetworkAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HasDeviceInformationChanged(&self) -> ::windows::core::Result<bool>;
    fn HasNetworkChanged(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandAccountUpdatedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandAccountUpdatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandAccountUpdatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandAccountUpdatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandAccountUpdatedEventArgsVtbl {
        unsafe extern "system" fn NetworkAccountId<Impl: IMobileBroadbandAccountUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HasDeviceInformationChanged<Impl: IMobileBroadbandAccountUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HasNetworkChanged<Impl: IMobileBroadbandAccountUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandAccountUpdatedEventArgs>, ::windows::core::GetTrustLevel, NetworkAccountId::<Impl, IMPL_OFFSET>, HasDeviceInformationChanged::<Impl, IMPL_OFFSET>, HasNetworkChanged::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandAccountUpdatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandAccountWatcherImpl: Sized {
    fn AccountAdded(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccountAdded(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AccountUpdated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccountUpdated(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AccountRemoved(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccountRemoved(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandAccountWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<MobileBroadbandAccountWatcherStatus>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandAccountWatcher {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandAccountWatcher";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandAccountWatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandAccountWatcherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandAccountWatcherVtbl {
        unsafe extern "system" fn AccountAdded<Impl: IMobileBroadbandAccountWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveAccountAdded<Impl: IMobileBroadbandAccountWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAccountAdded(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AccountUpdated<Impl: IMobileBroadbandAccountWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveAccountUpdated<Impl: IMobileBroadbandAccountWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAccountUpdated(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AccountRemoved<Impl: IMobileBroadbandAccountWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveAccountRemoved<Impl: IMobileBroadbandAccountWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAccountRemoved(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnumerationCompleted<Impl: IMobileBroadbandAccountWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveEnumerationCompleted<Impl: IMobileBroadbandAccountWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnumerationCompleted(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stopped<Impl: IMobileBroadbandAccountWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveStopped<Impl: IMobileBroadbandAccountWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStopped(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Status<Impl: IMobileBroadbandAccountWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandAccountWatcherStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Start<Impl: IMobileBroadbandAccountWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IMobileBroadbandAccountWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMobileBroadbandAccountWatcher>,
            ::windows::core::GetTrustLevel,
            AccountAdded::<Impl, IMPL_OFFSET>,
            RemoveAccountAdded::<Impl, IMPL_OFFSET>,
            AccountUpdated::<Impl, IMPL_OFFSET>,
            RemoveAccountUpdated::<Impl, IMPL_OFFSET>,
            AccountRemoved::<Impl, IMPL_OFFSET>,
            RemoveAccountRemoved::<Impl, IMPL_OFFSET>,
            EnumerationCompleted::<Impl, IMPL_OFFSET>,
            RemoveEnumerationCompleted::<Impl, IMPL_OFFSET>,
            Stopped::<Impl, IMPL_OFFSET>,
            RemoveStopped::<Impl, IMPL_OFFSET>,
            Status::<Impl, IMPL_OFFSET>,
            Start::<Impl, IMPL_OFFSET>,
            Stop::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandAccountWatcher as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandAntennaSarImpl: Sized {
    fn AntennaIndex(&self) -> ::windows::core::Result<i32>;
    fn SarBackoffIndex(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandAntennaSar {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandAntennaSar";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandAntennaSarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandAntennaSarImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandAntennaSarVtbl {
        unsafe extern "system" fn AntennaIndex<Impl: IMobileBroadbandAntennaSarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SarBackoffIndex<Impl: IMobileBroadbandAntennaSarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandAntennaSar>, ::windows::core::GetTrustLevel, AntennaIndex::<Impl, IMPL_OFFSET>, SarBackoffIndex::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandAntennaSar as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandAntennaSarFactoryImpl: Sized {
    fn CreateWithIndex(&self, antennaindex: i32, sarbackoffindex: i32) -> ::windows::core::Result<MobileBroadbandAntennaSar>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandAntennaSarFactory {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandAntennaSarFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandAntennaSarFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandAntennaSarFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandAntennaSarFactoryVtbl {
        unsafe extern "system" fn CreateWithIndex<Impl: IMobileBroadbandAntennaSarFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, antennaindex: i32, sarbackoffindex: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandAntennaSarFactory>, ::windows::core::GetTrustLevel, CreateWithIndex::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandAntennaSarFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandCellCdmaImpl: Sized {
    fn BaseStationId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn BaseStationPNCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn BaseStationLatitude(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn BaseStationLongitude(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn BaseStationLastBroadcastGpsTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>;
    fn NetworkId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn PilotSignalStrengthInDB(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn SystemId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandCellCdma {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandCellCdma";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandCellCdmaVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandCellCdmaImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandCellCdmaVtbl {
        unsafe extern "system" fn BaseStationId<Impl: IMobileBroadbandCellCdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BaseStationPNCode<Impl: IMobileBroadbandCellCdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BaseStationLatitude<Impl: IMobileBroadbandCellCdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BaseStationLongitude<Impl: IMobileBroadbandCellCdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BaseStationLastBroadcastGpsTime<Impl: IMobileBroadbandCellCdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NetworkId<Impl: IMobileBroadbandCellCdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PilotSignalStrengthInDB<Impl: IMobileBroadbandCellCdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SystemId<Impl: IMobileBroadbandCellCdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMobileBroadbandCellCdma>,
            ::windows::core::GetTrustLevel,
            BaseStationId::<Impl, IMPL_OFFSET>,
            BaseStationPNCode::<Impl, IMPL_OFFSET>,
            BaseStationLatitude::<Impl, IMPL_OFFSET>,
            BaseStationLongitude::<Impl, IMPL_OFFSET>,
            BaseStationLastBroadcastGpsTime::<Impl, IMPL_OFFSET>,
            NetworkId::<Impl, IMPL_OFFSET>,
            PilotSignalStrengthInDB::<Impl, IMPL_OFFSET>,
            SystemId::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandCellCdma as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandCellGsmImpl: Sized {
    fn BaseStationId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn CellId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn ChannelNumber(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn LocationAreaCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReceivedSignalStrengthInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn TimingAdvanceInBitPeriods(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandCellGsm {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandCellGsm";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandCellGsmVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandCellGsmImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandCellGsmVtbl {
        unsafe extern "system" fn BaseStationId<Impl: IMobileBroadbandCellGsmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CellId<Impl: IMobileBroadbandCellGsmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ChannelNumber<Impl: IMobileBroadbandCellGsmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LocationAreaCode<Impl: IMobileBroadbandCellGsmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProviderId<Impl: IMobileBroadbandCellGsmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReceivedSignalStrengthInDBm<Impl: IMobileBroadbandCellGsmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TimingAdvanceInBitPeriods<Impl: IMobileBroadbandCellGsmImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMobileBroadbandCellGsm>,
            ::windows::core::GetTrustLevel,
            BaseStationId::<Impl, IMPL_OFFSET>,
            CellId::<Impl, IMPL_OFFSET>,
            ChannelNumber::<Impl, IMPL_OFFSET>,
            LocationAreaCode::<Impl, IMPL_OFFSET>,
            ProviderId::<Impl, IMPL_OFFSET>,
            ReceivedSignalStrengthInDBm::<Impl, IMPL_OFFSET>,
            TimingAdvanceInBitPeriods::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandCellGsm as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandCellLteImpl: Sized {
    fn CellId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn ChannelNumber(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn PhysicalCellId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReferenceSignalReceivedPowerInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn ReferenceSignalReceivedQualityInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn TimingAdvanceInBitPeriods(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn TrackingAreaCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandCellLte {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandCellLte";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandCellLteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandCellLteImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandCellLteVtbl {
        unsafe extern "system" fn CellId<Impl: IMobileBroadbandCellLteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ChannelNumber<Impl: IMobileBroadbandCellLteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PhysicalCellId<Impl: IMobileBroadbandCellLteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProviderId<Impl: IMobileBroadbandCellLteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReferenceSignalReceivedPowerInDBm<Impl: IMobileBroadbandCellLteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReferenceSignalReceivedQualityInDBm<Impl: IMobileBroadbandCellLteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TimingAdvanceInBitPeriods<Impl: IMobileBroadbandCellLteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TrackingAreaCode<Impl: IMobileBroadbandCellLteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMobileBroadbandCellLte>,
            ::windows::core::GetTrustLevel,
            CellId::<Impl, IMPL_OFFSET>,
            ChannelNumber::<Impl, IMPL_OFFSET>,
            PhysicalCellId::<Impl, IMPL_OFFSET>,
            ProviderId::<Impl, IMPL_OFFSET>,
            ReferenceSignalReceivedPowerInDBm::<Impl, IMPL_OFFSET>,
            ReferenceSignalReceivedQualityInDBm::<Impl, IMPL_OFFSET>,
            TimingAdvanceInBitPeriods::<Impl, IMPL_OFFSET>,
            TrackingAreaCode::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandCellLte as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandCellNRImpl: Sized {
    fn CellId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i64>>;
    fn ChannelNumber(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn PhysicalCellId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReferenceSignalReceivedPowerInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn ReferenceSignalReceivedQualityInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn TimingAdvanceInNanoseconds(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn TrackingAreaCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SignalToNoiseRatioInDB(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandCellNR {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandCellNR";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandCellNRVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandCellNRImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandCellNRVtbl {
        unsafe extern "system" fn CellId<Impl: IMobileBroadbandCellNRImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ChannelNumber<Impl: IMobileBroadbandCellNRImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PhysicalCellId<Impl: IMobileBroadbandCellNRImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProviderId<Impl: IMobileBroadbandCellNRImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReferenceSignalReceivedPowerInDBm<Impl: IMobileBroadbandCellNRImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReferenceSignalReceivedQualityInDBm<Impl: IMobileBroadbandCellNRImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TimingAdvanceInNanoseconds<Impl: IMobileBroadbandCellNRImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TrackingAreaCode<Impl: IMobileBroadbandCellNRImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SignalToNoiseRatioInDB<Impl: IMobileBroadbandCellNRImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMobileBroadbandCellNR>,
            ::windows::core::GetTrustLevel,
            CellId::<Impl, IMPL_OFFSET>,
            ChannelNumber::<Impl, IMPL_OFFSET>,
            PhysicalCellId::<Impl, IMPL_OFFSET>,
            ProviderId::<Impl, IMPL_OFFSET>,
            ReferenceSignalReceivedPowerInDBm::<Impl, IMPL_OFFSET>,
            ReferenceSignalReceivedQualityInDBm::<Impl, IMPL_OFFSET>,
            TimingAdvanceInNanoseconds::<Impl, IMPL_OFFSET>,
            TrackingAreaCode::<Impl, IMPL_OFFSET>,
            SignalToNoiseRatioInDB::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandCellNR as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandCellTdscdmaImpl: Sized {
    fn CellId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn CellParameterId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn ChannelNumber(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn LocationAreaCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn PathLossInDB(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReceivedSignalCodePowerInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn TimingAdvanceInBitPeriods(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandCellTdscdma {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandCellTdscdma";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandCellTdscdmaVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandCellTdscdmaImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandCellTdscdmaVtbl {
        unsafe extern "system" fn CellId<Impl: IMobileBroadbandCellTdscdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CellParameterId<Impl: IMobileBroadbandCellTdscdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ChannelNumber<Impl: IMobileBroadbandCellTdscdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LocationAreaCode<Impl: IMobileBroadbandCellTdscdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PathLossInDB<Impl: IMobileBroadbandCellTdscdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProviderId<Impl: IMobileBroadbandCellTdscdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReceivedSignalCodePowerInDBm<Impl: IMobileBroadbandCellTdscdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TimingAdvanceInBitPeriods<Impl: IMobileBroadbandCellTdscdmaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMobileBroadbandCellTdscdma>,
            ::windows::core::GetTrustLevel,
            CellId::<Impl, IMPL_OFFSET>,
            CellParameterId::<Impl, IMPL_OFFSET>,
            ChannelNumber::<Impl, IMPL_OFFSET>,
            LocationAreaCode::<Impl, IMPL_OFFSET>,
            PathLossInDB::<Impl, IMPL_OFFSET>,
            ProviderId::<Impl, IMPL_OFFSET>,
            ReceivedSignalCodePowerInDBm::<Impl, IMPL_OFFSET>,
            TimingAdvanceInBitPeriods::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandCellTdscdma as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandCellUmtsImpl: Sized {
    fn CellId(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn ChannelNumber(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn LocationAreaCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn PathLossInDB(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn PrimaryScramblingCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReceivedSignalCodePowerInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn SignalToNoiseRatioInDB(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandCellUmts {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandCellUmts";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandCellUmtsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandCellUmtsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandCellUmtsVtbl {
        unsafe extern "system" fn CellId<Impl: IMobileBroadbandCellUmtsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ChannelNumber<Impl: IMobileBroadbandCellUmtsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LocationAreaCode<Impl: IMobileBroadbandCellUmtsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PathLossInDB<Impl: IMobileBroadbandCellUmtsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PrimaryScramblingCode<Impl: IMobileBroadbandCellUmtsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProviderId<Impl: IMobileBroadbandCellUmtsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReceivedSignalCodePowerInDBm<Impl: IMobileBroadbandCellUmtsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SignalToNoiseRatioInDB<Impl: IMobileBroadbandCellUmtsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMobileBroadbandCellUmts>,
            ::windows::core::GetTrustLevel,
            CellId::<Impl, IMPL_OFFSET>,
            ChannelNumber::<Impl, IMPL_OFFSET>,
            LocationAreaCode::<Impl, IMPL_OFFSET>,
            PathLossInDB::<Impl, IMPL_OFFSET>,
            PrimaryScramblingCode::<Impl, IMPL_OFFSET>,
            ProviderId::<Impl, IMPL_OFFSET>,
            ReceivedSignalCodePowerInDBm::<Impl, IMPL_OFFSET>,
            SignalToNoiseRatioInDB::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandCellUmts as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandCellsInfoImpl: Sized {
    fn NeighboringCellsCdma(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellCdma>>;
    fn NeighboringCellsGsm(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellGsm>>;
    fn NeighboringCellsLte(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellLte>>;
    fn NeighboringCellsTdscdma(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellTdscdma>>;
    fn NeighboringCellsUmts(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellUmts>>;
    fn ServingCellsCdma(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellCdma>>;
    fn ServingCellsGsm(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellGsm>>;
    fn ServingCellsLte(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellLte>>;
    fn ServingCellsTdscdma(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellTdscdma>>;
    fn ServingCellsUmts(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellUmts>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandCellsInfo {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandCellsInfo";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandCellsInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandCellsInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandCellsInfoVtbl {
        unsafe extern "system" fn NeighboringCellsCdma<Impl: IMobileBroadbandCellsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NeighboringCellsGsm<Impl: IMobileBroadbandCellsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NeighboringCellsLte<Impl: IMobileBroadbandCellsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NeighboringCellsTdscdma<Impl: IMobileBroadbandCellsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NeighboringCellsUmts<Impl: IMobileBroadbandCellsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ServingCellsCdma<Impl: IMobileBroadbandCellsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ServingCellsGsm<Impl: IMobileBroadbandCellsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ServingCellsLte<Impl: IMobileBroadbandCellsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ServingCellsTdscdma<Impl: IMobileBroadbandCellsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ServingCellsUmts<Impl: IMobileBroadbandCellsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMobileBroadbandCellsInfo>,
            ::windows::core::GetTrustLevel,
            NeighboringCellsCdma::<Impl, IMPL_OFFSET>,
            NeighboringCellsGsm::<Impl, IMPL_OFFSET>,
            NeighboringCellsLte::<Impl, IMPL_OFFSET>,
            NeighboringCellsTdscdma::<Impl, IMPL_OFFSET>,
            NeighboringCellsUmts::<Impl, IMPL_OFFSET>,
            ServingCellsCdma::<Impl, IMPL_OFFSET>,
            ServingCellsGsm::<Impl, IMPL_OFFSET>,
            ServingCellsLte::<Impl, IMPL_OFFSET>,
            ServingCellsTdscdma::<Impl, IMPL_OFFSET>,
            ServingCellsUmts::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandCellsInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandCellsInfo2Impl: Sized {
    fn NeighboringCellsNR(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellNR>>;
    fn ServingCellsNR(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandCellNR>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandCellsInfo2 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandCellsInfo2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandCellsInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandCellsInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandCellsInfo2Vtbl {
        unsafe extern "system" fn NeighboringCellsNR<Impl: IMobileBroadbandCellsInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ServingCellsNR<Impl: IMobileBroadbandCellsInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandCellsInfo2>, ::windows::core::GetTrustLevel, NeighboringCellsNR::<Impl, IMPL_OFFSET>, ServingCellsNR::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandCellsInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandCurrentSlotIndexChangedEventArgsImpl: Sized {
    fn CurrentSlotIndex(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandCurrentSlotIndexChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandCurrentSlotIndexChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandCurrentSlotIndexChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandCurrentSlotIndexChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandCurrentSlotIndexChangedEventArgsVtbl {
        unsafe extern "system" fn CurrentSlotIndex<Impl: IMobileBroadbandCurrentSlotIndexChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandCurrentSlotIndexChangedEventArgs>, ::windows::core::GetTrustLevel, CurrentSlotIndex::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandCurrentSlotIndexChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Sms", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandDeviceInformationImpl: Sized {
    fn NetworkDeviceStatus(&self) -> ::windows::core::Result<NetworkDeviceStatus>;
    fn Manufacturer(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Model(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FirmwareInformation(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CellularClass(&self) -> ::windows::core::Result<super::super::Devices::Sms::CellularClass>;
    fn DataClasses(&self) -> ::windows::core::Result<DataClasses>;
    fn CustomDataClass(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MobileEquipmentId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TelephoneNumbers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn SubscriberId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SimIccId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceType(&self) -> ::windows::core::Result<MobileBroadbandDeviceType>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CurrentRadioState(&self) -> ::windows::core::Result<MobileBroadbandRadioState>;
}
#[cfg(all(feature = "Devices_Sms", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandDeviceInformation {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandDeviceInformation";
}
#[cfg(all(feature = "Devices_Sms", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandDeviceInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandDeviceInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandDeviceInformationVtbl {
        unsafe extern "system" fn NetworkDeviceStatus<Impl: IMobileBroadbandDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NetworkDeviceStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Manufacturer<Impl: IMobileBroadbandDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Model<Impl: IMobileBroadbandDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FirmwareInformation<Impl: IMobileBroadbandDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CellularClass<Impl: IMobileBroadbandDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Sms::CellularClass) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DataClasses<Impl: IMobileBroadbandDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DataClasses) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CustomDataClass<Impl: IMobileBroadbandDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MobileEquipmentId<Impl: IMobileBroadbandDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TelephoneNumbers<Impl: IMobileBroadbandDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SubscriberId<Impl: IMobileBroadbandDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SimIccId<Impl: IMobileBroadbandDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceType<Impl: IMobileBroadbandDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandDeviceType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceId<Impl: IMobileBroadbandDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CurrentRadioState<Impl: IMobileBroadbandDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandRadioState) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMobileBroadbandDeviceInformation>,
            ::windows::core::GetTrustLevel,
            NetworkDeviceStatus::<Impl, IMPL_OFFSET>,
            Manufacturer::<Impl, IMPL_OFFSET>,
            Model::<Impl, IMPL_OFFSET>,
            FirmwareInformation::<Impl, IMPL_OFFSET>,
            CellularClass::<Impl, IMPL_OFFSET>,
            DataClasses::<Impl, IMPL_OFFSET>,
            CustomDataClass::<Impl, IMPL_OFFSET>,
            MobileEquipmentId::<Impl, IMPL_OFFSET>,
            TelephoneNumbers::<Impl, IMPL_OFFSET>,
            SubscriberId::<Impl, IMPL_OFFSET>,
            SimIccId::<Impl, IMPL_OFFSET>,
            DeviceType::<Impl, IMPL_OFFSET>,
            DeviceId::<Impl, IMPL_OFFSET>,
            CurrentRadioState::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandDeviceInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandDeviceInformation2Impl: Sized {
    fn PinManager(&self) -> ::windows::core::Result<MobileBroadbandPinManager>;
    fn Revision(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SerialNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandDeviceInformation2 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandDeviceInformation2";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandDeviceInformation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandDeviceInformation2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandDeviceInformation2Vtbl {
        unsafe extern "system" fn PinManager<Impl: IMobileBroadbandDeviceInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Revision<Impl: IMobileBroadbandDeviceInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SerialNumber<Impl: IMobileBroadbandDeviceInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandDeviceInformation2>, ::windows::core::GetTrustLevel, PinManager::<Impl, IMPL_OFFSET>, Revision::<Impl, IMPL_OFFSET>, SerialNumber::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandDeviceInformation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandDeviceInformation3Impl: Sized {
    fn SimSpn(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SimPnn(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SimGid1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandDeviceInformation3 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandDeviceInformation3";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandDeviceInformation3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandDeviceInformation3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandDeviceInformation3Vtbl {
        unsafe extern "system" fn SimSpn<Impl: IMobileBroadbandDeviceInformation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SimPnn<Impl: IMobileBroadbandDeviceInformation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SimGid1<Impl: IMobileBroadbandDeviceInformation3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandDeviceInformation3>, ::windows::core::GetTrustLevel, SimSpn::<Impl, IMPL_OFFSET>, SimPnn::<Impl, IMPL_OFFSET>, SimGid1::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandDeviceInformation3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandDeviceInformation4Impl: Sized {
    fn SlotManager(&self) -> ::windows::core::Result<MobileBroadbandSlotManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandDeviceInformation4 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandDeviceInformation4";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandDeviceInformation4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandDeviceInformation4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandDeviceInformation4Vtbl {
        unsafe extern "system" fn SlotManager<Impl: IMobileBroadbandDeviceInformation4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandDeviceInformation4>, ::windows::core::GetTrustLevel, SlotManager::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandDeviceInformation4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandDeviceServiceImpl: Sized {
    fn DeviceServiceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SupportedCommands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn OpenDataSession(&self) -> ::windows::core::Result<MobileBroadbandDeviceServiceDataSession>;
    fn OpenCommandSession(&self) -> ::windows::core::Result<MobileBroadbandDeviceServiceCommandSession>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandDeviceService {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandDeviceService";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandDeviceServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandDeviceServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandDeviceServiceVtbl {
        unsafe extern "system" fn DeviceServiceId<Impl: IMobileBroadbandDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SupportedCommands<Impl: IMobileBroadbandDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OpenDataSession<Impl: IMobileBroadbandDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OpenCommandSession<Impl: IMobileBroadbandDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandDeviceService>, ::windows::core::GetTrustLevel, DeviceServiceId::<Impl, IMPL_OFFSET>, SupportedCommands::<Impl, IMPL_OFFSET>, OpenDataSession::<Impl, IMPL_OFFSET>, OpenCommandSession::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandDeviceService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMobileBroadbandDeviceServiceCommandResultImpl: Sized {
    fn StatusCode(&self) -> ::windows::core::Result<u32>;
    fn ResponseData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandDeviceServiceCommandResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandDeviceServiceCommandResult";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMobileBroadbandDeviceServiceCommandResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandDeviceServiceCommandResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandDeviceServiceCommandResultVtbl {
        unsafe extern "system" fn StatusCode<Impl: IMobileBroadbandDeviceServiceCommandResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResponseData<Impl: IMobileBroadbandDeviceServiceCommandResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandDeviceServiceCommandResult>, ::windows::core::GetTrustLevel, StatusCode::<Impl, IMPL_OFFSET>, ResponseData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandDeviceServiceCommandResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMobileBroadbandDeviceServiceCommandSessionImpl: Sized {
    fn SendQueryCommandAsync(&self, commandid: u32, data: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandDeviceServiceCommandResult>>;
    fn SendSetCommandAsync(&self, commandid: u32, data: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandDeviceServiceCommandResult>>;
    fn CloseSession(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandDeviceServiceCommandSession {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandDeviceServiceCommandSession";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMobileBroadbandDeviceServiceCommandSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandDeviceServiceCommandSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandDeviceServiceCommandSessionVtbl {
        unsafe extern "system" fn SendQueryCommandAsync<Impl: IMobileBroadbandDeviceServiceCommandSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SendSetCommandAsync<Impl: IMobileBroadbandDeviceServiceCommandSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u32, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CloseSession<Impl: IMobileBroadbandDeviceServiceCommandSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseSession().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandDeviceServiceCommandSession>, ::windows::core::GetTrustLevel, SendQueryCommandAsync::<Impl, IMPL_OFFSET>, SendSetCommandAsync::<Impl, IMPL_OFFSET>, CloseSession::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandDeviceServiceCommandSession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMobileBroadbandDeviceServiceDataReceivedEventArgsImpl: Sized {
    fn ReceivedData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandDeviceServiceDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandDeviceServiceDataReceivedEventArgs";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMobileBroadbandDeviceServiceDataReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandDeviceServiceDataReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandDeviceServiceDataReceivedEventArgsVtbl {
        unsafe extern "system" fn ReceivedData<Impl: IMobileBroadbandDeviceServiceDataReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandDeviceServiceDataReceivedEventArgs>, ::windows::core::GetTrustLevel, ReceivedData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandDeviceServiceDataReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMobileBroadbandDeviceServiceDataSessionImpl: Sized {
    fn WriteDataAsync(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CloseSession(&self) -> ::windows::core::Result<()>;
    fn DataReceived(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandDeviceServiceDataSession, MobileBroadbandDeviceServiceDataReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDataReceived(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandDeviceServiceDataSession {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandDeviceServiceDataSession";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMobileBroadbandDeviceServiceDataSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandDeviceServiceDataSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandDeviceServiceDataSessionVtbl {
        unsafe extern "system" fn WriteDataAsync<Impl: IMobileBroadbandDeviceServiceDataSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CloseSession<Impl: IMobileBroadbandDeviceServiceDataSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseSession().into()
        }
        unsafe extern "system" fn DataReceived<Impl: IMobileBroadbandDeviceServiceDataSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveDataReceived<Impl: IMobileBroadbandDeviceServiceDataSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDataReceived(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandDeviceServiceDataSession>, ::windows::core::GetTrustLevel, WriteDataAsync::<Impl, IMPL_OFFSET>, CloseSession::<Impl, IMPL_OFFSET>, DataReceived::<Impl, IMPL_OFFSET>, RemoveDataReceived::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandDeviceServiceDataSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandDeviceServiceInformationImpl: Sized {
    fn DeviceServiceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IsDataReadSupported(&self) -> ::windows::core::Result<bool>;
    fn IsDataWriteSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandDeviceServiceInformation {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandDeviceServiceInformation";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandDeviceServiceInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandDeviceServiceInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandDeviceServiceInformationVtbl {
        unsafe extern "system" fn DeviceServiceId<Impl: IMobileBroadbandDeviceServiceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsDataReadSupported<Impl: IMobileBroadbandDeviceServiceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsDataWriteSupported<Impl: IMobileBroadbandDeviceServiceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandDeviceServiceInformation>, ::windows::core::GetTrustLevel, DeviceServiceId::<Impl, IMPL_OFFSET>, IsDataReadSupported::<Impl, IMPL_OFFSET>, IsDataWriteSupported::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandDeviceServiceInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMobileBroadbandDeviceServiceTriggerDetailsImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceServiceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ReceivedData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandDeviceServiceTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandDeviceServiceTriggerDetails";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMobileBroadbandDeviceServiceTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandDeviceServiceTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandDeviceServiceTriggerDetailsVtbl {
        unsafe extern "system" fn DeviceId<Impl: IMobileBroadbandDeviceServiceTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceServiceId<Impl: IMobileBroadbandDeviceServiceTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReceivedData<Impl: IMobileBroadbandDeviceServiceTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandDeviceServiceTriggerDetails>, ::windows::core::GetTrustLevel, DeviceId::<Impl, IMPL_OFFSET>, DeviceServiceId::<Impl, IMPL_OFFSET>, ReceivedData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandDeviceServiceTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandModemImpl: Sized {
    fn CurrentAccount(&self) -> ::windows::core::Result<MobileBroadbandAccount>;
    fn DeviceInformation(&self) -> ::windows::core::Result<MobileBroadbandDeviceInformation>;
    fn MaxDeviceServiceCommandSizeInBytes(&self) -> ::windows::core::Result<u32>;
    fn MaxDeviceServiceDataSizeInBytes(&self) -> ::windows::core::Result<u32>;
    fn DeviceServices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandDeviceServiceInformation>>;
    fn GetDeviceService(&self, deviceserviceid: &::windows::core::GUID) -> ::windows::core::Result<MobileBroadbandDeviceService>;
    fn IsResetSupported(&self) -> ::windows::core::Result<bool>;
    fn ResetAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetCurrentConfigurationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandModemConfiguration>>;
    fn CurrentNetwork(&self) -> ::windows::core::Result<MobileBroadbandNetwork>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandModem {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandModem";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandModemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandModemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandModemVtbl {
        unsafe extern "system" fn CurrentAccount<Impl: IMobileBroadbandModemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceInformation<Impl: IMobileBroadbandModemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxDeviceServiceCommandSizeInBytes<Impl: IMobileBroadbandModemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxDeviceServiceDataSizeInBytes<Impl: IMobileBroadbandModemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceServices<Impl: IMobileBroadbandModemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceService<Impl: IMobileBroadbandModemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceserviceid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsResetSupported<Impl: IMobileBroadbandModemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResetAsync<Impl: IMobileBroadbandModemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetCurrentConfigurationAsync<Impl: IMobileBroadbandModemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CurrentNetwork<Impl: IMobileBroadbandModemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMobileBroadbandModem>,
            ::windows::core::GetTrustLevel,
            CurrentAccount::<Impl, IMPL_OFFSET>,
            DeviceInformation::<Impl, IMPL_OFFSET>,
            MaxDeviceServiceCommandSizeInBytes::<Impl, IMPL_OFFSET>,
            MaxDeviceServiceDataSizeInBytes::<Impl, IMPL_OFFSET>,
            DeviceServices::<Impl, IMPL_OFFSET>,
            GetDeviceService::<Impl, IMPL_OFFSET>,
            IsResetSupported::<Impl, IMPL_OFFSET>,
            ResetAsync::<Impl, IMPL_OFFSET>,
            GetCurrentConfigurationAsync::<Impl, IMPL_OFFSET>,
            CurrentNetwork::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandModem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandModem2Impl: Sized {
    fn GetIsPassthroughEnabledAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn SetIsPassthroughEnabledAsync(&self, value: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandModemStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandModem2 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandModem2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandModem2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandModem2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandModem2Vtbl {
        unsafe extern "system" fn GetIsPassthroughEnabledAsync<Impl: IMobileBroadbandModem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsPassthroughEnabledAsync<Impl: IMobileBroadbandModem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandModem2>, ::windows::core::GetTrustLevel, GetIsPassthroughEnabledAsync::<Impl, IMPL_OFFSET>, SetIsPassthroughEnabledAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandModem2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandModem3Impl: Sized {
    fn TryGetPcoAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPco>>;
    fn IsInEmergencyCallMode(&self) -> ::windows::core::Result<bool>;
    fn IsInEmergencyCallModeChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandModem, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsInEmergencyCallModeChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandModem3 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandModem3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandModem3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandModem3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandModem3Vtbl {
        unsafe extern "system" fn TryGetPcoAsync<Impl: IMobileBroadbandModem3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsInEmergencyCallMode<Impl: IMobileBroadbandModem3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsInEmergencyCallModeChanged<Impl: IMobileBroadbandModem3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveIsInEmergencyCallModeChanged<Impl: IMobileBroadbandModem3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveIsInEmergencyCallModeChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandModem3>, ::windows::core::GetTrustLevel, TryGetPcoAsync::<Impl, IMPL_OFFSET>, IsInEmergencyCallMode::<Impl, IMPL_OFFSET>, IsInEmergencyCallModeChanged::<Impl, IMPL_OFFSET>, RemoveIsInEmergencyCallModeChanged::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandModem3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandModemConfigurationImpl: Sized {
    fn Uicc(&self) -> ::windows::core::Result<MobileBroadbandUicc>;
    fn HomeProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HomeProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandModemConfiguration {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandModemConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandModemConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandModemConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandModemConfigurationVtbl {
        unsafe extern "system" fn Uicc<Impl: IMobileBroadbandModemConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HomeProviderId<Impl: IMobileBroadbandModemConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HomeProviderName<Impl: IMobileBroadbandModemConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandModemConfiguration>, ::windows::core::GetTrustLevel, Uicc::<Impl, IMPL_OFFSET>, HomeProviderId::<Impl, IMPL_OFFSET>, HomeProviderName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandModemConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandModemConfiguration2Impl: Sized {
    fn SarManager(&self) -> ::windows::core::Result<MobileBroadbandSarManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandModemConfiguration2 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandModemConfiguration2";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandModemConfiguration2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandModemConfiguration2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandModemConfiguration2Vtbl {
        unsafe extern "system" fn SarManager<Impl: IMobileBroadbandModemConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandModemConfiguration2>, ::windows::core::GetTrustLevel, SarManager::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandModemConfiguration2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandModemIsolationImpl: Sized {
    fn AddAllowedHost(&self, host: &::core::option::Option<super::HostName>) -> ::windows::core::Result<()>;
    fn AddAllowedHostRange(&self, first: &::core::option::Option<super::HostName>, last: &::core::option::Option<super::HostName>) -> ::windows::core::Result<()>;
    fn ApplyConfigurationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ClearConfigurationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandModemIsolation {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandModemIsolation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandModemIsolationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandModemIsolationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandModemIsolationVtbl {
        unsafe extern "system" fn AddAllowedHost<Impl: IMobileBroadbandModemIsolationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, host: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAllowedHost(&*(&host as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddAllowedHostRange<Impl: IMobileBroadbandModemIsolationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, first: ::windows::core::RawPtr, last: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAllowedHostRange(&*(&first as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType), &*(&last as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ApplyConfigurationAsync<Impl: IMobileBroadbandModemIsolationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClearConfigurationAsync<Impl: IMobileBroadbandModemIsolationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandModemIsolation>, ::windows::core::GetTrustLevel, AddAllowedHost::<Impl, IMPL_OFFSET>, AddAllowedHostRange::<Impl, IMPL_OFFSET>, ApplyConfigurationAsync::<Impl, IMPL_OFFSET>, ClearConfigurationAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandModemIsolation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandModemIsolationFactoryImpl: Sized {
    fn Create(&self, modemdeviceid: &::windows::core::HSTRING, rulegroupid: &::windows::core::HSTRING) -> ::windows::core::Result<MobileBroadbandModemIsolation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandModemIsolationFactory {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandModemIsolationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandModemIsolationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandModemIsolationFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandModemIsolationFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IMobileBroadbandModemIsolationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, modemdeviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, rulegroupid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandModemIsolationFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandModemIsolationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandModemStaticsImpl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromId(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<MobileBroadbandModem>;
    fn GetDefault(&self) -> ::windows::core::Result<MobileBroadbandModem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandModemStatics {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandModemStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandModemStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandModemStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandModemStaticsVtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IMobileBroadbandModemStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromId<Impl: IMobileBroadbandModemStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDefault<Impl: IMobileBroadbandModemStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandModemStatics>, ::windows::core::GetTrustLevel, GetDeviceSelector::<Impl, IMPL_OFFSET>, FromId::<Impl, IMPL_OFFSET>, GetDefault::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandModemStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Networking_Connectivity", feature = "implement_exclusive"))]
pub trait IMobileBroadbandNetworkImpl: Sized {
    fn NetworkAdapter(&self) -> ::windows::core::Result<super::Connectivity::NetworkAdapter>;
    fn NetworkRegistrationState(&self) -> ::windows::core::Result<NetworkRegistrationState>;
    fn RegistrationNetworkError(&self) -> ::windows::core::Result<u32>;
    fn PacketAttachNetworkError(&self) -> ::windows::core::Result<u32>;
    fn ActivationNetworkError(&self) -> ::windows::core::Result<u32>;
    fn AccessPointName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RegisteredDataClass(&self) -> ::windows::core::Result<DataClasses>;
    fn RegisteredProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RegisteredProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ShowConnectionUI(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandNetwork {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandNetwork";
}
#[cfg(all(feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl IMobileBroadbandNetworkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandNetworkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandNetworkVtbl {
        unsafe extern "system" fn NetworkAdapter<Impl: IMobileBroadbandNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NetworkRegistrationState<Impl: IMobileBroadbandNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NetworkRegistrationState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RegistrationNetworkError<Impl: IMobileBroadbandNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PacketAttachNetworkError<Impl: IMobileBroadbandNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ActivationNetworkError<Impl: IMobileBroadbandNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AccessPointName<Impl: IMobileBroadbandNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RegisteredDataClass<Impl: IMobileBroadbandNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DataClasses) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RegisteredProviderId<Impl: IMobileBroadbandNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RegisteredProviderName<Impl: IMobileBroadbandNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShowConnectionUI<Impl: IMobileBroadbandNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowConnectionUI().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMobileBroadbandNetwork>,
            ::windows::core::GetTrustLevel,
            NetworkAdapter::<Impl, IMPL_OFFSET>,
            NetworkRegistrationState::<Impl, IMPL_OFFSET>,
            RegistrationNetworkError::<Impl, IMPL_OFFSET>,
            PacketAttachNetworkError::<Impl, IMPL_OFFSET>,
            ActivationNetworkError::<Impl, IMPL_OFFSET>,
            AccessPointName::<Impl, IMPL_OFFSET>,
            RegisteredDataClass::<Impl, IMPL_OFFSET>,
            RegisteredProviderId::<Impl, IMPL_OFFSET>,
            RegisteredProviderName::<Impl, IMPL_OFFSET>,
            ShowConnectionUI::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandNetwork as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandNetwork2Impl: Sized {
    fn GetVoiceCallSupportAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RegistrationUiccApps(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandUiccApp>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandNetwork2 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandNetwork2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandNetwork2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandNetwork2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandNetwork2Vtbl {
        unsafe extern "system" fn GetVoiceCallSupportAsync<Impl: IMobileBroadbandNetwork2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RegistrationUiccApps<Impl: IMobileBroadbandNetwork2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandNetwork2>, ::windows::core::GetTrustLevel, GetVoiceCallSupportAsync::<Impl, IMPL_OFFSET>, RegistrationUiccApps::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandNetwork2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandNetwork3Impl: Sized {
    fn GetCellsInfoAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandCellsInfo>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandNetwork3 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandNetwork3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandNetwork3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandNetwork3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandNetwork3Vtbl {
        unsafe extern "system" fn GetCellsInfoAsync<Impl: IMobileBroadbandNetwork3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandNetwork3>, ::windows::core::GetTrustLevel, GetCellsInfoAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandNetwork3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandNetworkRegistrationStateChangeImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Network(&self) -> ::windows::core::Result<MobileBroadbandNetwork>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandNetworkRegistrationStateChange {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandNetworkRegistrationStateChange";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandNetworkRegistrationStateChangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandNetworkRegistrationStateChangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandNetworkRegistrationStateChangeVtbl {
        unsafe extern "system" fn DeviceId<Impl: IMobileBroadbandNetworkRegistrationStateChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Network<Impl: IMobileBroadbandNetworkRegistrationStateChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandNetworkRegistrationStateChange>, ::windows::core::GetTrustLevel, DeviceId::<Impl, IMPL_OFFSET>, Network::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandNetworkRegistrationStateChange as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandNetworkRegistrationStateChangeTriggerDetailsImpl: Sized {
    fn NetworkRegistrationStateChanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandNetworkRegistrationStateChange>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandNetworkRegistrationStateChangeTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandNetworkRegistrationStateChangeTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandNetworkRegistrationStateChangeTriggerDetailsVtbl {
        unsafe extern "system" fn NetworkRegistrationStateChanges<Impl: IMobileBroadbandNetworkRegistrationStateChangeTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails>, ::windows::core::GetTrustLevel, NetworkRegistrationStateChanges::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMobileBroadbandPcoImpl: Sized {
    fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn IsComplete(&self) -> ::windows::core::Result<bool>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandPco {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandPco";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMobileBroadbandPcoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandPcoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandPcoVtbl {
        unsafe extern "system" fn Data<Impl: IMobileBroadbandPcoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsComplete<Impl: IMobileBroadbandPcoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceId<Impl: IMobileBroadbandPcoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandPco>, ::windows::core::GetTrustLevel, Data::<Impl, IMPL_OFFSET>, IsComplete::<Impl, IMPL_OFFSET>, DeviceId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandPco as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandPcoDataChangeTriggerDetailsImpl: Sized {
    fn UpdatedData(&self) -> ::windows::core::Result<MobileBroadbandPco>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandPcoDataChangeTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandPcoDataChangeTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandPcoDataChangeTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandPcoDataChangeTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandPcoDataChangeTriggerDetailsVtbl {
        unsafe extern "system" fn UpdatedData<Impl: IMobileBroadbandPcoDataChangeTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandPcoDataChangeTriggerDetails>, ::windows::core::GetTrustLevel, UpdatedData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandPcoDataChangeTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandPinImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<MobileBroadbandPinType>;
    fn LockState(&self) -> ::windows::core::Result<MobileBroadbandPinLockState>;
    fn Format(&self) -> ::windows::core::Result<MobileBroadbandPinFormat>;
    fn Enabled(&self) -> ::windows::core::Result<bool>;
    fn MaxLength(&self) -> ::windows::core::Result<u32>;
    fn MinLength(&self) -> ::windows::core::Result<u32>;
    fn AttemptsRemaining(&self) -> ::windows::core::Result<u32>;
    fn EnableAsync(&self, currentpin: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>;
    fn DisableAsync(&self, currentpin: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>;
    fn EnterAsync(&self, currentpin: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>;
    fn ChangeAsync(&self, currentpin: &::windows::core::HSTRING, newpin: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>;
    fn UnblockAsync(&self, pinunblockkey: &::windows::core::HSTRING, newpin: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandPin {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandPin";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandPinVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandPinImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandPinVtbl {
        unsafe extern "system" fn Type<Impl: IMobileBroadbandPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandPinType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LockState<Impl: IMobileBroadbandPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandPinLockState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Format<Impl: IMobileBroadbandPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandPinFormat) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Enabled<Impl: IMobileBroadbandPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxLength<Impl: IMobileBroadbandPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MinLength<Impl: IMobileBroadbandPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AttemptsRemaining<Impl: IMobileBroadbandPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EnableAsync<Impl: IMobileBroadbandPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentpin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisableAsync<Impl: IMobileBroadbandPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentpin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EnterAsync<Impl: IMobileBroadbandPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentpin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ChangeAsync<Impl: IMobileBroadbandPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentpin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, newpin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UnblockAsync<Impl: IMobileBroadbandPinImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinunblockkey: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, newpin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMobileBroadbandPin>,
            ::windows::core::GetTrustLevel,
            Type::<Impl, IMPL_OFFSET>,
            LockState::<Impl, IMPL_OFFSET>,
            Format::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            MaxLength::<Impl, IMPL_OFFSET>,
            MinLength::<Impl, IMPL_OFFSET>,
            AttemptsRemaining::<Impl, IMPL_OFFSET>,
            EnableAsync::<Impl, IMPL_OFFSET>,
            DisableAsync::<Impl, IMPL_OFFSET>,
            EnterAsync::<Impl, IMPL_OFFSET>,
            ChangeAsync::<Impl, IMPL_OFFSET>,
            UnblockAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandPin as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandPinLockStateChangeImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PinType(&self) -> ::windows::core::Result<MobileBroadbandPinType>;
    fn PinLockState(&self) -> ::windows::core::Result<MobileBroadbandPinLockState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandPinLockStateChange {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandPinLockStateChange";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandPinLockStateChangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandPinLockStateChangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandPinLockStateChangeVtbl {
        unsafe extern "system" fn DeviceId<Impl: IMobileBroadbandPinLockStateChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PinType<Impl: IMobileBroadbandPinLockStateChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandPinType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PinLockState<Impl: IMobileBroadbandPinLockStateChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandPinLockState) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandPinLockStateChange>, ::windows::core::GetTrustLevel, DeviceId::<Impl, IMPL_OFFSET>, PinType::<Impl, IMPL_OFFSET>, PinLockState::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandPinLockStateChange as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandPinLockStateChangeTriggerDetailsImpl: Sized {
    fn PinLockStateChanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandPinLockStateChange>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandPinLockStateChangeTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandPinLockStateChangeTriggerDetails";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandPinLockStateChangeTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandPinLockStateChangeTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandPinLockStateChangeTriggerDetailsVtbl {
        unsafe extern "system" fn PinLockStateChanges<Impl: IMobileBroadbandPinLockStateChangeTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandPinLockStateChangeTriggerDetails>, ::windows::core::GetTrustLevel, PinLockStateChanges::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandPinLockStateChangeTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandPinManagerImpl: Sized {
    fn SupportedPins(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandPinType>>;
    fn GetPin(&self, pintype: MobileBroadbandPinType) -> ::windows::core::Result<MobileBroadbandPin>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandPinManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandPinManager";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandPinManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandPinManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandPinManagerVtbl {
        unsafe extern "system" fn SupportedPins<Impl: IMobileBroadbandPinManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetPin<Impl: IMobileBroadbandPinManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pintype: MobileBroadbandPinType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandPinManager>, ::windows::core::GetTrustLevel, SupportedPins::<Impl, IMPL_OFFSET>, GetPin::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandPinManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandPinOperationResultImpl: Sized {
    fn IsSuccessful(&self) -> ::windows::core::Result<bool>;
    fn AttemptsRemaining(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandPinOperationResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandPinOperationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandPinOperationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandPinOperationResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandPinOperationResultVtbl {
        unsafe extern "system" fn IsSuccessful<Impl: IMobileBroadbandPinOperationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AttemptsRemaining<Impl: IMobileBroadbandPinOperationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandPinOperationResult>, ::windows::core::GetTrustLevel, IsSuccessful::<Impl, IMPL_OFFSET>, AttemptsRemaining::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandPinOperationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandRadioStateChangeImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RadioState(&self) -> ::windows::core::Result<MobileBroadbandRadioState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandRadioStateChange {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandRadioStateChange";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandRadioStateChangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandRadioStateChangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandRadioStateChangeVtbl {
        unsafe extern "system" fn DeviceId<Impl: IMobileBroadbandRadioStateChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RadioState<Impl: IMobileBroadbandRadioStateChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandRadioState) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandRadioStateChange>, ::windows::core::GetTrustLevel, DeviceId::<Impl, IMPL_OFFSET>, RadioState::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandRadioStateChange as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandRadioStateChangeTriggerDetailsImpl: Sized {
    fn RadioStateChanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandRadioStateChange>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandRadioStateChangeTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandRadioStateChangeTriggerDetails";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandRadioStateChangeTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandRadioStateChangeTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandRadioStateChangeTriggerDetailsVtbl {
        unsafe extern "system" fn RadioStateChanges<Impl: IMobileBroadbandRadioStateChangeTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandRadioStateChangeTriggerDetails>, ::windows::core::GetTrustLevel, RadioStateChanges::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandRadioStateChangeTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandSarManagerImpl: Sized {
    fn IsBackoffEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsWiFiHardwareIntegrated(&self) -> ::windows::core::Result<bool>;
    fn IsSarControlledByHardware(&self) -> ::windows::core::Result<bool>;
    fn Antennas(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandAntennaSar>>;
    fn HysteresisTimerPeriod(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn TransmissionStateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandSarManager, MobileBroadbandTransmissionStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTransmissionStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnableBackoffAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DisableBackoffAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetConfigurationAsync(&self, antennas: &::core::option::Option<super::super::Foundation::Collections::IIterable<MobileBroadbandAntennaSar>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RevertSarToHardwareControlAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetTransmissionStateChangedHysteresisAsync(&self, timerperiod: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetIsTransmittingAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn StartTransmissionStateMonitoring(&self) -> ::windows::core::Result<()>;
    fn StopTransmissionStateMonitoring(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandSarManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandSarManager";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandSarManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandSarManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandSarManagerVtbl {
        unsafe extern "system" fn IsBackoffEnabled<Impl: IMobileBroadbandSarManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsWiFiHardwareIntegrated<Impl: IMobileBroadbandSarManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsSarControlledByHardware<Impl: IMobileBroadbandSarManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Antennas<Impl: IMobileBroadbandSarManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HysteresisTimerPeriod<Impl: IMobileBroadbandSarManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TransmissionStateChanged<Impl: IMobileBroadbandSarManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveTransmissionStateChanged<Impl: IMobileBroadbandSarManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTransmissionStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnableBackoffAsync<Impl: IMobileBroadbandSarManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisableBackoffAsync<Impl: IMobileBroadbandSarManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetConfigurationAsync<Impl: IMobileBroadbandSarManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, antennas: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RevertSarToHardwareControlAsync<Impl: IMobileBroadbandSarManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTransmissionStateChangedHysteresisAsync<Impl: IMobileBroadbandSarManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timerperiod: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetIsTransmittingAsync<Impl: IMobileBroadbandSarManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StartTransmissionStateMonitoring<Impl: IMobileBroadbandSarManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartTransmissionStateMonitoring().into()
        }
        unsafe extern "system" fn StopTransmissionStateMonitoring<Impl: IMobileBroadbandSarManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopTransmissionStateMonitoring().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMobileBroadbandSarManager>,
            ::windows::core::GetTrustLevel,
            IsBackoffEnabled::<Impl, IMPL_OFFSET>,
            IsWiFiHardwareIntegrated::<Impl, IMPL_OFFSET>,
            IsSarControlledByHardware::<Impl, IMPL_OFFSET>,
            Antennas::<Impl, IMPL_OFFSET>,
            HysteresisTimerPeriod::<Impl, IMPL_OFFSET>,
            TransmissionStateChanged::<Impl, IMPL_OFFSET>,
            RemoveTransmissionStateChanged::<Impl, IMPL_OFFSET>,
            EnableBackoffAsync::<Impl, IMPL_OFFSET>,
            DisableBackoffAsync::<Impl, IMPL_OFFSET>,
            SetConfigurationAsync::<Impl, IMPL_OFFSET>,
            RevertSarToHardwareControlAsync::<Impl, IMPL_OFFSET>,
            SetTransmissionStateChangedHysteresisAsync::<Impl, IMPL_OFFSET>,
            GetIsTransmittingAsync::<Impl, IMPL_OFFSET>,
            StartTransmissionStateMonitoring::<Impl, IMPL_OFFSET>,
            StopTransmissionStateMonitoring::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandSarManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandSlotInfoImpl: Sized {
    fn Index(&self) -> ::windows::core::Result<i32>;
    fn State(&self) -> ::windows::core::Result<MobileBroadbandSlotState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandSlotInfo {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandSlotInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandSlotInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandSlotInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandSlotInfoVtbl {
        unsafe extern "system" fn Index<Impl: IMobileBroadbandSlotInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn State<Impl: IMobileBroadbandSlotInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandSlotState) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandSlotInfo>, ::windows::core::GetTrustLevel, Index::<Impl, IMPL_OFFSET>, State::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandSlotInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandSlotInfoChangedEventArgsImpl: Sized {
    fn SlotInfo(&self) -> ::windows::core::Result<MobileBroadbandSlotInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandSlotInfoChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandSlotInfoChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandSlotInfoChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandSlotInfoChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandSlotInfoChangedEventArgsVtbl {
        unsafe extern "system" fn SlotInfo<Impl: IMobileBroadbandSlotInfoChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandSlotInfoChangedEventArgs>, ::windows::core::GetTrustLevel, SlotInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandSlotInfoChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandSlotManagerImpl: Sized {
    fn SlotInfos(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandSlotInfo>>;
    fn CurrentSlotIndex(&self) -> ::windows::core::Result<i32>;
    fn SetCurrentSlot(&self, slotindex: i32) -> ::windows::core::Result<MobileBroadbandModemStatus>;
    fn SetCurrentSlotAsync(&self, slotindex: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandModemStatus>>;
    fn SlotInfoChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandSlotManager, MobileBroadbandSlotInfoChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSlotInfoChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CurrentSlotIndexChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MobileBroadbandSlotManager, MobileBroadbandCurrentSlotIndexChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentSlotIndexChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandSlotManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandSlotManager";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandSlotManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandSlotManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandSlotManagerVtbl {
        unsafe extern "system" fn SlotInfos<Impl: IMobileBroadbandSlotManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CurrentSlotIndex<Impl: IMobileBroadbandSlotManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCurrentSlot<Impl: IMobileBroadbandSlotManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, slotindex: i32, result__: *mut MobileBroadbandModemStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCurrentSlotAsync<Impl: IMobileBroadbandSlotManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, slotindex: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SlotInfoChanged<Impl: IMobileBroadbandSlotManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveSlotInfoChanged<Impl: IMobileBroadbandSlotManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSlotInfoChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CurrentSlotIndexChanged<Impl: IMobileBroadbandSlotManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveCurrentSlotIndexChanged<Impl: IMobileBroadbandSlotManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCurrentSlotIndexChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMobileBroadbandSlotManager>,
            ::windows::core::GetTrustLevel,
            SlotInfos::<Impl, IMPL_OFFSET>,
            CurrentSlotIndex::<Impl, IMPL_OFFSET>,
            SetCurrentSlot::<Impl, IMPL_OFFSET>,
            SetCurrentSlotAsync::<Impl, IMPL_OFFSET>,
            SlotInfoChanged::<Impl, IMPL_OFFSET>,
            RemoveSlotInfoChanged::<Impl, IMPL_OFFSET>,
            CurrentSlotIndexChanged::<Impl, IMPL_OFFSET>,
            RemoveCurrentSlotIndexChanged::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandSlotManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandTransmissionStateChangedEventArgsImpl: Sized {
    fn IsTransmitting(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandTransmissionStateChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandTransmissionStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandTransmissionStateChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandTransmissionStateChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandTransmissionStateChangedEventArgsVtbl {
        unsafe extern "system" fn IsTransmitting<Impl: IMobileBroadbandTransmissionStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandTransmissionStateChangedEventArgs>, ::windows::core::GetTrustLevel, IsTransmitting::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandTransmissionStateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMobileBroadbandUiccImpl: Sized {
    fn SimIccId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetUiccAppsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandUiccAppsResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandUicc {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandUicc";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMobileBroadbandUiccVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandUiccImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandUiccVtbl {
        unsafe extern "system" fn SimIccId<Impl: IMobileBroadbandUiccImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetUiccAppsAsync<Impl: IMobileBroadbandUiccImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandUicc>, ::windows::core::GetTrustLevel, SimIccId::<Impl, IMPL_OFFSET>, GetUiccAppsAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandUicc as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMobileBroadbandUiccAppImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn Kind(&self) -> ::windows::core::Result<UiccAppKind>;
    fn GetRecordDetailsAsync(&self, uiccfilepath: &::core::option::Option<super::super::Foundation::Collections::IIterable<u32>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandUiccAppRecordDetailsResult>>;
    fn ReadRecordAsync(&self, uiccfilepath: &::core::option::Option<super::super::Foundation::Collections::IIterable<u32>>, recordindex: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MobileBroadbandUiccAppReadRecordResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandUiccApp {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandUiccApp";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMobileBroadbandUiccAppVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandUiccAppImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandUiccAppVtbl {
        unsafe extern "system" fn Id<Impl: IMobileBroadbandUiccAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Kind<Impl: IMobileBroadbandUiccAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UiccAppKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetRecordDetailsAsync<Impl: IMobileBroadbandUiccAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiccfilepath: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReadRecordAsync<Impl: IMobileBroadbandUiccAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uiccfilepath: ::windows::core::RawPtr, recordindex: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandUiccApp>, ::windows::core::GetTrustLevel, Id::<Impl, IMPL_OFFSET>, Kind::<Impl, IMPL_OFFSET>, GetRecordDetailsAsync::<Impl, IMPL_OFFSET>, ReadRecordAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandUiccApp as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMobileBroadbandUiccAppReadRecordResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<MobileBroadbandUiccAppOperationStatus>;
    fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandUiccAppReadRecordResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandUiccAppReadRecordResult";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMobileBroadbandUiccAppReadRecordResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandUiccAppReadRecordResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandUiccAppReadRecordResultVtbl {
        unsafe extern "system" fn Status<Impl: IMobileBroadbandUiccAppReadRecordResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandUiccAppOperationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Data<Impl: IMobileBroadbandUiccAppReadRecordResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandUiccAppReadRecordResult>, ::windows::core::GetTrustLevel, Status::<Impl, IMPL_OFFSET>, Data::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandUiccAppReadRecordResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMobileBroadbandUiccAppRecordDetailsResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<MobileBroadbandUiccAppOperationStatus>;
    fn Kind(&self) -> ::windows::core::Result<UiccAppRecordKind>;
    fn RecordCount(&self) -> ::windows::core::Result<i32>;
    fn RecordSize(&self) -> ::windows::core::Result<i32>;
    fn ReadAccessCondition(&self) -> ::windows::core::Result<UiccAccessCondition>;
    fn WriteAccessCondition(&self) -> ::windows::core::Result<UiccAccessCondition>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMobileBroadbandUiccAppRecordDetailsResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandUiccAppRecordDetailsResult";
}
#[cfg(feature = "implement_exclusive")]
impl IMobileBroadbandUiccAppRecordDetailsResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandUiccAppRecordDetailsResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandUiccAppRecordDetailsResultVtbl {
        unsafe extern "system" fn Status<Impl: IMobileBroadbandUiccAppRecordDetailsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandUiccAppOperationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Kind<Impl: IMobileBroadbandUiccAppRecordDetailsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UiccAppRecordKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RecordCount<Impl: IMobileBroadbandUiccAppRecordDetailsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RecordSize<Impl: IMobileBroadbandUiccAppRecordDetailsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReadAccessCondition<Impl: IMobileBroadbandUiccAppRecordDetailsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UiccAccessCondition) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WriteAccessCondition<Impl: IMobileBroadbandUiccAppRecordDetailsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UiccAccessCondition) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMobileBroadbandUiccAppRecordDetailsResult>,
            ::windows::core::GetTrustLevel,
            Status::<Impl, IMPL_OFFSET>,
            Kind::<Impl, IMPL_OFFSET>,
            RecordCount::<Impl, IMPL_OFFSET>,
            RecordSize::<Impl, IMPL_OFFSET>,
            ReadAccessCondition::<Impl, IMPL_OFFSET>,
            WriteAccessCondition::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandUiccAppRecordDetailsResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMobileBroadbandUiccAppsResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<MobileBroadbandUiccAppOperationStatus>;
    fn UiccApps(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MobileBroadbandUiccApp>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMobileBroadbandUiccAppsResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IMobileBroadbandUiccAppsResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMobileBroadbandUiccAppsResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMobileBroadbandUiccAppsResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMobileBroadbandUiccAppsResultVtbl {
        unsafe extern "system" fn Status<Impl: IMobileBroadbandUiccAppsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MobileBroadbandUiccAppOperationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UiccApps<Impl: IMobileBroadbandUiccAppsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMobileBroadbandUiccAppsResult>, ::windows::core::GetTrustLevel, Status::<Impl, IMPL_OFFSET>, UiccApps::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMobileBroadbandUiccAppsResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorDataUsageTriggerDetailsImpl: Sized {
    fn NotificationKind(&self) -> ::windows::core::Result<NetworkOperatorDataUsageNotificationKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkOperatorDataUsageTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.INetworkOperatorDataUsageTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkOperatorDataUsageTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorDataUsageTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorDataUsageTriggerDetailsVtbl {
        unsafe extern "system" fn NotificationKind<Impl: INetworkOperatorDataUsageTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NetworkOperatorDataUsageNotificationKind) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetworkOperatorDataUsageTriggerDetails>, ::windows::core::GetTrustLevel, NotificationKind::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorDataUsageTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Sms", feature = "implement_exclusive"))]
pub trait INetworkOperatorNotificationEventDetailsImpl: Sized {
    fn NotificationType(&self) -> ::windows::core::Result<NetworkOperatorEventMessageType>;
    fn NetworkAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EncodingType(&self) -> ::windows::core::Result<u8>;
    fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RuleId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SmsMessage(&self) -> ::windows::core::Result<super::super::Devices::Sms::ISmsMessage>;
}
#[cfg(all(feature = "Devices_Sms", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INetworkOperatorNotificationEventDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.INetworkOperatorNotificationEventDetails";
}
#[cfg(all(feature = "Devices_Sms", feature = "implement_exclusive"))]
impl INetworkOperatorNotificationEventDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorNotificationEventDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorNotificationEventDetailsVtbl {
        unsafe extern "system" fn NotificationType<Impl: INetworkOperatorNotificationEventDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NetworkOperatorEventMessageType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NetworkAccountId<Impl: INetworkOperatorNotificationEventDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EncodingType<Impl: INetworkOperatorNotificationEventDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Message<Impl: INetworkOperatorNotificationEventDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RuleId<Impl: INetworkOperatorNotificationEventDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SmsMessage<Impl: INetworkOperatorNotificationEventDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<INetworkOperatorNotificationEventDetails>,
            ::windows::core::GetTrustLevel,
            NotificationType::<Impl, IMPL_OFFSET>,
            NetworkAccountId::<Impl, IMPL_OFFSET>,
            EncodingType::<Impl, IMPL_OFFSET>,
            Message::<Impl, IMPL_OFFSET>,
            RuleId::<Impl, IMPL_OFFSET>,
            SmsMessage::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorNotificationEventDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorTetheringAccessPointConfigurationImpl: Sized {
    fn Ssid(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSsid(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Passphrase(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPassphrase(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkOperatorTetheringAccessPointConfiguration {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.INetworkOperatorTetheringAccessPointConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkOperatorTetheringAccessPointConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorTetheringAccessPointConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorTetheringAccessPointConfigurationVtbl {
        unsafe extern "system" fn Ssid<Impl: INetworkOperatorTetheringAccessPointConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSsid<Impl: INetworkOperatorTetheringAccessPointConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSsid(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Passphrase<Impl: INetworkOperatorTetheringAccessPointConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPassphrase<Impl: INetworkOperatorTetheringAccessPointConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPassphrase(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetworkOperatorTetheringAccessPointConfiguration>, ::windows::core::GetTrustLevel, Ssid::<Impl, IMPL_OFFSET>, SetSsid::<Impl, IMPL_OFFSET>, Passphrase::<Impl, IMPL_OFFSET>, SetPassphrase::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorTetheringAccessPointConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait INetworkOperatorTetheringAccessPointConfiguration2Impl: Sized {
    fn IsBandSupported(&self, band: TetheringWiFiBand) -> ::windows::core::Result<bool>;
    fn IsBandSupportedAsync(&self, band: TetheringWiFiBand) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn Band(&self) -> ::windows::core::Result<TetheringWiFiBand>;
    fn SetBand(&self, value: TetheringWiFiBand) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INetworkOperatorTetheringAccessPointConfiguration2 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.INetworkOperatorTetheringAccessPointConfiguration2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl INetworkOperatorTetheringAccessPointConfiguration2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorTetheringAccessPointConfiguration2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorTetheringAccessPointConfiguration2Vtbl {
        unsafe extern "system" fn IsBandSupported<Impl: INetworkOperatorTetheringAccessPointConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, band: TetheringWiFiBand, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsBandSupportedAsync<Impl: INetworkOperatorTetheringAccessPointConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, band: TetheringWiFiBand, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Band<Impl: INetworkOperatorTetheringAccessPointConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TetheringWiFiBand) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBand<Impl: INetworkOperatorTetheringAccessPointConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TetheringWiFiBand) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBand(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetworkOperatorTetheringAccessPointConfiguration2>, ::windows::core::GetTrustLevel, IsBandSupported::<Impl, IMPL_OFFSET>, IsBandSupportedAsync::<Impl, IMPL_OFFSET>, Band::<Impl, IMPL_OFFSET>, SetBand::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorTetheringAccessPointConfiguration2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait INetworkOperatorTetheringClientImpl: Sized {
    fn MacAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HostNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::HostName>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INetworkOperatorTetheringClient {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.INetworkOperatorTetheringClient";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl INetworkOperatorTetheringClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorTetheringClientImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorTetheringClientVtbl {
        unsafe extern "system" fn MacAddress<Impl: INetworkOperatorTetheringClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HostNames<Impl: INetworkOperatorTetheringClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetworkOperatorTetheringClient>, ::windows::core::GetTrustLevel, MacAddress::<Impl, IMPL_OFFSET>, HostNames::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorTetheringClient as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait INetworkOperatorTetheringClientManagerImpl: Sized {
    fn GetTetheringClients(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<NetworkOperatorTetheringClient>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INetworkOperatorTetheringClientManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.INetworkOperatorTetheringClientManager";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl INetworkOperatorTetheringClientManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorTetheringClientManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorTetheringClientManagerVtbl {
        unsafe extern "system" fn GetTetheringClients<Impl: INetworkOperatorTetheringClientManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetworkOperatorTetheringClientManager>, ::windows::core::GetTrustLevel, GetTetheringClients::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorTetheringClientManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorTetheringEntitlementCheckImpl: Sized {
    fn AuthorizeTethering(&self, allow: bool, entitlementfailurereason: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkOperatorTetheringEntitlementCheck {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.INetworkOperatorTetheringEntitlementCheck";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkOperatorTetheringEntitlementCheckVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorTetheringEntitlementCheckImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorTetheringEntitlementCheckVtbl {
        unsafe extern "system" fn AuthorizeTethering<Impl: INetworkOperatorTetheringEntitlementCheckImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: bool, entitlementfailurereason: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AuthorizeTethering(allow, &*(&entitlementfailurereason as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetworkOperatorTetheringEntitlementCheck>, ::windows::core::GetTrustLevel, AuthorizeTethering::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorTetheringEntitlementCheck as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait INetworkOperatorTetheringManagerImpl: Sized {
    fn MaxClientCount(&self) -> ::windows::core::Result<u32>;
    fn ClientCount(&self) -> ::windows::core::Result<u32>;
    fn TetheringOperationalState(&self) -> ::windows::core::Result<TetheringOperationalState>;
    fn GetCurrentAccessPointConfiguration(&self) -> ::windows::core::Result<NetworkOperatorTetheringAccessPointConfiguration>;
    fn ConfigureAccessPointAsync(&self, configuration: &::core::option::Option<NetworkOperatorTetheringAccessPointConfiguration>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StartTetheringAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<NetworkOperatorTetheringOperationResult>>;
    fn StopTetheringAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<NetworkOperatorTetheringOperationResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INetworkOperatorTetheringManager {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.INetworkOperatorTetheringManager";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl INetworkOperatorTetheringManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorTetheringManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorTetheringManagerVtbl {
        unsafe extern "system" fn MaxClientCount<Impl: INetworkOperatorTetheringManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClientCount<Impl: INetworkOperatorTetheringManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TetheringOperationalState<Impl: INetworkOperatorTetheringManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TetheringOperationalState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetCurrentAccessPointConfiguration<Impl: INetworkOperatorTetheringManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConfigureAccessPointAsync<Impl: INetworkOperatorTetheringManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StartTetheringAsync<Impl: INetworkOperatorTetheringManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StopTetheringAsync<Impl: INetworkOperatorTetheringManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<INetworkOperatorTetheringManager>,
            ::windows::core::GetTrustLevel,
            MaxClientCount::<Impl, IMPL_OFFSET>,
            ClientCount::<Impl, IMPL_OFFSET>,
            TetheringOperationalState::<Impl, IMPL_OFFSET>,
            GetCurrentAccessPointConfiguration::<Impl, IMPL_OFFSET>,
            ConfigureAccessPointAsync::<Impl, IMPL_OFFSET>,
            StartTetheringAsync::<Impl, IMPL_OFFSET>,
            StopTetheringAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorTetheringManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorTetheringManagerStaticsImpl: Sized {
    fn GetTetheringCapability(&self, networkaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<TetheringCapability>;
    fn CreateFromNetworkAccountId(&self, networkaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<NetworkOperatorTetheringManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkOperatorTetheringManagerStatics {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.INetworkOperatorTetheringManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkOperatorTetheringManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorTetheringManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorTetheringManagerStaticsVtbl {
        unsafe extern "system" fn GetTetheringCapability<Impl: INetworkOperatorTetheringManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut TetheringCapability) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromNetworkAccountId<Impl: INetworkOperatorTetheringManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetworkOperatorTetheringManagerStatics>, ::windows::core::GetTrustLevel, GetTetheringCapability::<Impl, IMPL_OFFSET>, CreateFromNetworkAccountId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorTetheringManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Networking_Connectivity", feature = "implement_exclusive"))]
pub trait INetworkOperatorTetheringManagerStatics2Impl: Sized {
    fn GetTetheringCapabilityFromConnectionProfile(&self, profile: &::core::option::Option<super::Connectivity::ConnectionProfile>) -> ::windows::core::Result<TetheringCapability>;
    fn CreateFromConnectionProfile(&self, profile: &::core::option::Option<super::Connectivity::ConnectionProfile>) -> ::windows::core::Result<NetworkOperatorTetheringManager>;
}
#[cfg(all(feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INetworkOperatorTetheringManagerStatics2 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.INetworkOperatorTetheringManagerStatics2";
}
#[cfg(all(feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl INetworkOperatorTetheringManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorTetheringManagerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorTetheringManagerStatics2Vtbl {
        unsafe extern "system" fn GetTetheringCapabilityFromConnectionProfile<Impl: INetworkOperatorTetheringManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr, result__: *mut TetheringCapability) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromConnectionProfile<Impl: INetworkOperatorTetheringManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetworkOperatorTetheringManagerStatics2>, ::windows::core::GetTrustLevel, GetTetheringCapabilityFromConnectionProfile::<Impl, IMPL_OFFSET>, CreateFromConnectionProfile::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorTetheringManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Networking_Connectivity", feature = "implement_exclusive"))]
pub trait INetworkOperatorTetheringManagerStatics3Impl: Sized {
    fn CreateFromConnectionProfileWithTargetAdapter(&self, profile: &::core::option::Option<super::Connectivity::ConnectionProfile>, adapter: &::core::option::Option<super::Connectivity::NetworkAdapter>) -> ::windows::core::Result<NetworkOperatorTetheringManager>;
}
#[cfg(all(feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INetworkOperatorTetheringManagerStatics3 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.INetworkOperatorTetheringManagerStatics3";
}
#[cfg(all(feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl INetworkOperatorTetheringManagerStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorTetheringManagerStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorTetheringManagerStatics3Vtbl {
        unsafe extern "system" fn CreateFromConnectionProfileWithTargetAdapter<Impl: INetworkOperatorTetheringManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr, adapter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetworkOperatorTetheringManagerStatics3>, ::windows::core::GetTrustLevel, CreateFromConnectionProfileWithTargetAdapter::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorTetheringManagerStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait INetworkOperatorTetheringManagerStatics4Impl: Sized {
    fn IsNoConnectionsTimeoutEnabled(&self) -> ::windows::core::Result<bool>;
    fn EnableNoConnectionsTimeout(&self) -> ::windows::core::Result<()>;
    fn EnableNoConnectionsTimeoutAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DisableNoConnectionsTimeout(&self) -> ::windows::core::Result<()>;
    fn DisableNoConnectionsTimeoutAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INetworkOperatorTetheringManagerStatics4 {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.INetworkOperatorTetheringManagerStatics4";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl INetworkOperatorTetheringManagerStatics4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorTetheringManagerStatics4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorTetheringManagerStatics4Vtbl {
        unsafe extern "system" fn IsNoConnectionsTimeoutEnabled<Impl: INetworkOperatorTetheringManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EnableNoConnectionsTimeout<Impl: INetworkOperatorTetheringManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableNoConnectionsTimeout().into()
        }
        unsafe extern "system" fn EnableNoConnectionsTimeoutAsync<Impl: INetworkOperatorTetheringManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisableNoConnectionsTimeout<Impl: INetworkOperatorTetheringManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableNoConnectionsTimeout().into()
        }
        unsafe extern "system" fn DisableNoConnectionsTimeoutAsync<Impl: INetworkOperatorTetheringManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<INetworkOperatorTetheringManagerStatics4>,
            ::windows::core::GetTrustLevel,
            IsNoConnectionsTimeoutEnabled::<Impl, IMPL_OFFSET>,
            EnableNoConnectionsTimeout::<Impl, IMPL_OFFSET>,
            EnableNoConnectionsTimeoutAsync::<Impl, IMPL_OFFSET>,
            DisableNoConnectionsTimeout::<Impl, IMPL_OFFSET>,
            DisableNoConnectionsTimeoutAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorTetheringManagerStatics4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorTetheringOperationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<TetheringOperationStatus>;
    fn AdditionalErrorMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INetworkOperatorTetheringOperationResult {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.INetworkOperatorTetheringOperationResult";
}
#[cfg(feature = "implement_exclusive")]
impl INetworkOperatorTetheringOperationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetworkOperatorTetheringOperationResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetworkOperatorTetheringOperationResultVtbl {
        unsafe extern "system" fn Status<Impl: INetworkOperatorTetheringOperationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TetheringOperationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AdditionalErrorMessage<Impl: INetworkOperatorTetheringOperationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetworkOperatorTetheringOperationResult>, ::windows::core::GetTrustLevel, Status::<Impl, IMPL_OFFSET>, AdditionalErrorMessage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetworkOperatorTetheringOperationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProvisionFromXmlDocumentResultsImpl: Sized {
    fn AllElementsProvisioned(&self) -> ::windows::core::Result<bool>;
    fn ProvisionResultsXml(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProvisionFromXmlDocumentResults {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IProvisionFromXmlDocumentResults";
}
#[cfg(feature = "implement_exclusive")]
impl IProvisionFromXmlDocumentResultsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvisionFromXmlDocumentResultsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProvisionFromXmlDocumentResultsVtbl {
        unsafe extern "system" fn AllElementsProvisioned<Impl: IProvisionFromXmlDocumentResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProvisionResultsXml<Impl: IProvisionFromXmlDocumentResultsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProvisionFromXmlDocumentResults>, ::windows::core::GetTrustLevel, AllElementsProvisioned::<Impl, IMPL_OFFSET>, ProvisionResultsXml::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvisionFromXmlDocumentResults as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
pub trait IProvisionedProfileImpl: Sized {
    fn UpdateCost(&self, value: super::Connectivity::NetworkCostType) -> ::windows::core::Result<()>;
    fn UpdateUsage(&self, value: &ProfileUsage) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProvisionedProfile {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IProvisionedProfile";
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl IProvisionedProfileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvisionedProfileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProvisionedProfileVtbl {
        unsafe extern "system" fn UpdateCost<Impl: IProvisionedProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Connectivity::NetworkCostType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateCost(value).into()
        }
        unsafe extern "system" fn UpdateUsage<Impl: IProvisionedProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ProfileUsage) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateUsage(&*(&value as *const <ProfileUsage as ::windows::core::Abi>::Abi as *const <ProfileUsage as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProvisionedProfile>, ::windows::core::GetTrustLevel, UpdateCost::<Impl, IMPL_OFFSET>, UpdateUsage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvisionedProfile as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IProvisioningAgentImpl: Sized {
    fn ProvisionFromXmlDocumentAsync(&self, provisioningxmldocument: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ProvisionFromXmlDocumentResults>>;
    fn GetProvisionedProfile(&self, mediatype: ProfileMediaType, profilename: &::windows::core::HSTRING) -> ::windows::core::Result<ProvisionedProfile>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProvisioningAgent {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IProvisioningAgent";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IProvisioningAgentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvisioningAgentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProvisioningAgentVtbl {
        unsafe extern "system" fn ProvisionFromXmlDocumentAsync<Impl: IProvisioningAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provisioningxmldocument: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetProvisionedProfile<Impl: IProvisioningAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediatype: ProfileMediaType, profilename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProvisioningAgent>, ::windows::core::GetTrustLevel, ProvisionFromXmlDocumentAsync::<Impl, IMPL_OFFSET>, GetProvisionedProfile::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvisioningAgent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProvisioningAgentStaticMethodsImpl: Sized {
    fn CreateFromNetworkAccountId(&self, networkaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<ProvisioningAgent>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProvisioningAgentStaticMethods {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IProvisioningAgentStaticMethods";
}
#[cfg(feature = "implement_exclusive")]
impl IProvisioningAgentStaticMethodsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvisioningAgentStaticMethodsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProvisioningAgentStaticMethodsVtbl {
        unsafe extern "system" fn CreateFromNetworkAccountId<Impl: IProvisioningAgentStaticMethodsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProvisioningAgentStaticMethods>, ::windows::core::GetTrustLevel, CreateFromNetworkAccountId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvisioningAgentStaticMethods as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITetheringEntitlementCheckTriggerDetailsImpl: Sized {
    fn NetworkAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AllowTethering(&self) -> ::windows::core::Result<()>;
    fn DenyTethering(&self, entitlementfailurereason: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITetheringEntitlementCheckTriggerDetails {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.ITetheringEntitlementCheckTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl ITetheringEntitlementCheckTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITetheringEntitlementCheckTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITetheringEntitlementCheckTriggerDetailsVtbl {
        unsafe extern "system" fn NetworkAccountId<Impl: ITetheringEntitlementCheckTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AllowTethering<Impl: ITetheringEntitlementCheckTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AllowTethering().into()
        }
        unsafe extern "system" fn DenyTethering<Impl: ITetheringEntitlementCheckTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entitlementfailurereason: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DenyTethering(&*(&entitlementfailurereason as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITetheringEntitlementCheckTriggerDetails>, ::windows::core::GetTrustLevel, NetworkAccountId::<Impl, IMPL_OFFSET>, AllowTethering::<Impl, IMPL_OFFSET>, DenyTethering::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITetheringEntitlementCheckTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUssdMessageImpl: Sized {
    fn DataCodingScheme(&self) -> ::windows::core::Result<u8>;
    fn SetDataCodingScheme(&self, value: u8) -> ::windows::core::Result<()>;
    fn GetPayload(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn SetPayload(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn PayloadAsText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPayloadAsText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUssdMessage {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IUssdMessage";
}
#[cfg(feature = "implement_exclusive")]
impl IUssdMessageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUssdMessageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUssdMessageVtbl {
        unsafe extern "system" fn DataCodingScheme<Impl: IUssdMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDataCodingScheme<Impl: IUssdMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataCodingScheme(value).into()
        }
        unsafe extern "system" fn GetPayload<Impl: IUssdMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPayload<Impl: IUssdMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPayload(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)).into()
        }
        unsafe extern "system" fn PayloadAsText<Impl: IUssdMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPayloadAsText<Impl: IUssdMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPayloadAsText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUssdMessage>,
            ::windows::core::GetTrustLevel,
            DataCodingScheme::<Impl, IMPL_OFFSET>,
            SetDataCodingScheme::<Impl, IMPL_OFFSET>,
            GetPayload::<Impl, IMPL_OFFSET>,
            SetPayload::<Impl, IMPL_OFFSET>,
            PayloadAsText::<Impl, IMPL_OFFSET>,
            SetPayloadAsText::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUssdMessage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUssdMessageFactoryImpl: Sized {
    fn CreateMessage(&self, messagetext: &::windows::core::HSTRING) -> ::windows::core::Result<UssdMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUssdMessageFactory {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IUssdMessageFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IUssdMessageFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUssdMessageFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUssdMessageFactoryVtbl {
        unsafe extern "system" fn CreateMessage<Impl: IUssdMessageFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagetext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUssdMessageFactory>, ::windows::core::GetTrustLevel, CreateMessage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUssdMessageFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUssdReplyImpl: Sized {
    fn ResultCode(&self) -> ::windows::core::Result<UssdResultCode>;
    fn Message(&self) -> ::windows::core::Result<UssdMessage>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUssdReply {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IUssdReply";
}
#[cfg(feature = "implement_exclusive")]
impl IUssdReplyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUssdReplyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUssdReplyVtbl {
        unsafe extern "system" fn ResultCode<Impl: IUssdReplyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UssdResultCode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Message<Impl: IUssdReplyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUssdReply>, ::windows::core::GetTrustLevel, ResultCode::<Impl, IMPL_OFFSET>, Message::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUssdReply as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUssdSessionImpl: Sized {
    fn SendMessageAndGetReplyAsync(&self, message: &::core::option::Option<UssdMessage>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UssdReply>>;
    fn Close(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUssdSession {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IUssdSession";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUssdSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUssdSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUssdSessionVtbl {
        unsafe extern "system" fn SendMessageAndGetReplyAsync<Impl: IUssdSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Close<Impl: IUssdSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUssdSession>, ::windows::core::GetTrustLevel, SendMessageAndGetReplyAsync::<Impl, IMPL_OFFSET>, Close::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUssdSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUssdSessionStaticsImpl: Sized {
    fn CreateFromNetworkAccountId(&self, networkaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<UssdSession>;
    fn CreateFromNetworkInterfaceId(&self, networkinterfaceid: &::windows::core::HSTRING) -> ::windows::core::Result<UssdSession>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUssdSessionStatics {
    const NAME: &'static str = "Windows.Networking.NetworkOperators.IUssdSessionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUssdSessionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUssdSessionStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUssdSessionStaticsVtbl {
        unsafe extern "system" fn CreateFromNetworkAccountId<Impl: IUssdSessionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromNetworkInterfaceId<Impl: IUssdSessionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkinterfaceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUssdSessionStatics>, ::windows::core::GetTrustLevel, CreateFromNetworkAccountId::<Impl, IMPL_OFFSET>, CreateFromNetworkInterfaceId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUssdSessionStatics as ::windows::core::Interface>::IID
    }
}
