#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IXboxLiveDeviceAddressImpl: Sized {
    fn SnapshotChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<XboxLiveDeviceAddress, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSnapshotChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetSnapshotAsBase64(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetSnapshotAsBuffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn GetSnapshotAsBytes(&self, buffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType], byteswritten: &mut u32) -> ::windows::core::Result<()>;
    fn Compare(&self, otherdeviceaddress: &::core::option::Option<XboxLiveDeviceAddress>) -> ::windows::core::Result<i32>;
    fn IsValid(&self) -> ::windows::core::Result<bool>;
    fn IsLocal(&self) -> ::windows::core::Result<bool>;
    fn NetworkAccessKind(&self) -> ::windows::core::Result<XboxLiveNetworkAccessKind>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXboxLiveDeviceAddress {
    const NAME: &'static str = "Windows.Networking.XboxLive.IXboxLiveDeviceAddress";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IXboxLiveDeviceAddressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXboxLiveDeviceAddressImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXboxLiveDeviceAddressVtbl {
        unsafe extern "system" fn SnapshotChanged<Impl: IXboxLiveDeviceAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SnapshotChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<XboxLiveDeviceAddress, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<XboxLiveDeviceAddress, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSnapshotChanged<Impl: IXboxLiveDeviceAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSnapshotChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetSnapshotAsBase64<Impl: IXboxLiveDeviceAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSnapshotAsBase64() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSnapshotAsBuffer<Impl: IXboxLiveDeviceAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSnapshotAsBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSnapshotAsBytes<Impl: IXboxLiveDeviceAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8, byteswritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSnapshotAsBytes(::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&buffer), buffer_array_size as _), ::core::mem::transmute_copy(&byteswritten)).into()
        }
        unsafe extern "system" fn Compare<Impl: IXboxLiveDeviceAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, otherdeviceaddress: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Compare(&*(&otherdeviceaddress as *const <XboxLiveDeviceAddress as ::windows::core::Abi>::Abi as *const <XboxLiveDeviceAddress as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsValid<Impl: IXboxLiveDeviceAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsValid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLocal<Impl: IXboxLiveDeviceAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkAccessKind<Impl: IXboxLiveDeviceAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveNetworkAccessKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkAccessKind() {
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
            ::windows::core::GetRuntimeClassName::<IXboxLiveDeviceAddress>,
            ::windows::core::GetTrustLevel,
            SnapshotChanged::<Impl, IMPL_OFFSET>,
            RemoveSnapshotChanged::<Impl, IMPL_OFFSET>,
            GetSnapshotAsBase64::<Impl, IMPL_OFFSET>,
            GetSnapshotAsBuffer::<Impl, IMPL_OFFSET>,
            GetSnapshotAsBytes::<Impl, IMPL_OFFSET>,
            Compare::<Impl, IMPL_OFFSET>,
            IsValid::<Impl, IMPL_OFFSET>,
            IsLocal::<Impl, IMPL_OFFSET>,
            NetworkAccessKind::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXboxLiveDeviceAddress as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IXboxLiveDeviceAddressStaticsImpl: Sized {
    fn CreateFromSnapshotBase64(&self, base64: &::windows::core::HSTRING) -> ::windows::core::Result<XboxLiveDeviceAddress>;
    fn CreateFromSnapshotBuffer(&self, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<XboxLiveDeviceAddress>;
    fn CreateFromSnapshotBytes(&self, buffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<XboxLiveDeviceAddress>;
    fn GetLocal(&self) -> ::windows::core::Result<XboxLiveDeviceAddress>;
    fn MaxSnapshotBytesSize(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXboxLiveDeviceAddressStatics {
    const NAME: &'static str = "Windows.Networking.XboxLive.IXboxLiveDeviceAddressStatics";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IXboxLiveDeviceAddressStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXboxLiveDeviceAddressStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXboxLiveDeviceAddressStaticsVtbl {
        unsafe extern "system" fn CreateFromSnapshotBase64<Impl: IXboxLiveDeviceAddressStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, base64: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromSnapshotBase64(&*(&base64 as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromSnapshotBuffer<Impl: IXboxLiveDeviceAddressStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromSnapshotBuffer(&*(&buffer as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromSnapshotBytes<Impl: IXboxLiveDeviceAddressStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromSnapshotBytes(::core::slice::from_raw_parts(::core::mem::transmute_copy(&buffer), buffer_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocal<Impl: IXboxLiveDeviceAddressStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocal() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxSnapshotBytesSize<Impl: IXboxLiveDeviceAddressStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxSnapshotBytesSize() {
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
            ::windows::core::GetRuntimeClassName::<IXboxLiveDeviceAddressStatics>,
            ::windows::core::GetTrustLevel,
            CreateFromSnapshotBase64::<Impl, IMPL_OFFSET>,
            CreateFromSnapshotBuffer::<Impl, IMPL_OFFSET>,
            CreateFromSnapshotBytes::<Impl, IMPL_OFFSET>,
            GetLocal::<Impl, IMPL_OFFSET>,
            MaxSnapshotBytesSize::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXboxLiveDeviceAddressStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IXboxLiveEndpointPairImpl: Sized {
    fn StateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<XboxLiveEndpointPair, XboxLiveEndpointPairStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetRemoteSocketAddressBytes(&self, socketaddress: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GetLocalSocketAddressBytes(&self, socketaddress: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<XboxLiveEndpointPairState>;
    fn Template(&self) -> ::windows::core::Result<XboxLiveEndpointPairTemplate>;
    fn RemoteDeviceAddress(&self) -> ::windows::core::Result<XboxLiveDeviceAddress>;
    fn RemoteHostName(&self) -> ::windows::core::Result<super::HostName>;
    fn RemotePort(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LocalHostName(&self) -> ::windows::core::Result<super::HostName>;
    fn LocalPort(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXboxLiveEndpointPair {
    const NAME: &'static str = "Windows.Networking.XboxLive.IXboxLiveEndpointPair";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IXboxLiveEndpointPairVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXboxLiveEndpointPairImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXboxLiveEndpointPairVtbl {
        unsafe extern "system" fn StateChanged<Impl: IXboxLiveEndpointPairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<XboxLiveEndpointPair, XboxLiveEndpointPairStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<XboxLiveEndpointPair, XboxLiveEndpointPairStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStateChanged<Impl: IXboxLiveEndpointPairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeleteAsync<Impl: IXboxLiveEndpointPairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteSocketAddressBytes<Impl: IXboxLiveEndpointPairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, socketAddress_array_size: u32, socketaddress: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRemoteSocketAddressBytes(::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&socketaddress), socketAddress_array_size as _)).into()
        }
        unsafe extern "system" fn GetLocalSocketAddressBytes<Impl: IXboxLiveEndpointPairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, socketAddress_array_size: u32, socketaddress: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLocalSocketAddressBytes(::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&socketaddress), socketAddress_array_size as _)).into()
        }
        unsafe extern "system" fn State<Impl: IXboxLiveEndpointPairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveEndpointPairState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Template<Impl: IXboxLiveEndpointPairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Template() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteDeviceAddress<Impl: IXboxLiveEndpointPairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteDeviceAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteHostName<Impl: IXboxLiveEndpointPairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteHostName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemotePort<Impl: IXboxLiveEndpointPairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemotePort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalHostName<Impl: IXboxLiveEndpointPairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalHostName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalPort<Impl: IXboxLiveEndpointPairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalPort() {
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
            ::windows::core::GetRuntimeClassName::<IXboxLiveEndpointPair>,
            ::windows::core::GetTrustLevel,
            StateChanged::<Impl, IMPL_OFFSET>,
            RemoveStateChanged::<Impl, IMPL_OFFSET>,
            DeleteAsync::<Impl, IMPL_OFFSET>,
            GetRemoteSocketAddressBytes::<Impl, IMPL_OFFSET>,
            GetLocalSocketAddressBytes::<Impl, IMPL_OFFSET>,
            State::<Impl, IMPL_OFFSET>,
            Template::<Impl, IMPL_OFFSET>,
            RemoteDeviceAddress::<Impl, IMPL_OFFSET>,
            RemoteHostName::<Impl, IMPL_OFFSET>,
            RemotePort::<Impl, IMPL_OFFSET>,
            LocalHostName::<Impl, IMPL_OFFSET>,
            LocalPort::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXboxLiveEndpointPair as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXboxLiveEndpointPairCreationResultImpl: Sized {
    fn DeviceAddress(&self) -> ::windows::core::Result<XboxLiveDeviceAddress>;
    fn Status(&self) -> ::windows::core::Result<XboxLiveEndpointPairCreationStatus>;
    fn IsExistingPathEvaluation(&self) -> ::windows::core::Result<bool>;
    fn EndpointPair(&self) -> ::windows::core::Result<XboxLiveEndpointPair>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXboxLiveEndpointPairCreationResult {
    const NAME: &'static str = "Windows.Networking.XboxLive.IXboxLiveEndpointPairCreationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IXboxLiveEndpointPairCreationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXboxLiveEndpointPairCreationResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXboxLiveEndpointPairCreationResultVtbl {
        unsafe extern "system" fn DeviceAddress<Impl: IXboxLiveEndpointPairCreationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IXboxLiveEndpointPairCreationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveEndpointPairCreationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsExistingPathEvaluation<Impl: IXboxLiveEndpointPairCreationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsExistingPathEvaluation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndpointPair<Impl: IXboxLiveEndpointPairCreationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndpointPair() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXboxLiveEndpointPairCreationResult>, ::windows::core::GetTrustLevel, DeviceAddress::<Impl, IMPL_OFFSET>, Status::<Impl, IMPL_OFFSET>, IsExistingPathEvaluation::<Impl, IMPL_OFFSET>, EndpointPair::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXboxLiveEndpointPairCreationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXboxLiveEndpointPairStateChangedEventArgsImpl: Sized {
    fn OldState(&self) -> ::windows::core::Result<XboxLiveEndpointPairState>;
    fn NewState(&self) -> ::windows::core::Result<XboxLiveEndpointPairState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXboxLiveEndpointPairStateChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.XboxLive.IXboxLiveEndpointPairStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IXboxLiveEndpointPairStateChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXboxLiveEndpointPairStateChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXboxLiveEndpointPairStateChangedEventArgsVtbl {
        unsafe extern "system" fn OldState<Impl: IXboxLiveEndpointPairStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveEndpointPairState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OldState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewState<Impl: IXboxLiveEndpointPairStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveEndpointPairState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXboxLiveEndpointPairStateChangedEventArgs>, ::windows::core::GetTrustLevel, OldState::<Impl, IMPL_OFFSET>, NewState::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXboxLiveEndpointPairStateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXboxLiveEndpointPairStaticsImpl: Sized {
    fn FindEndpointPairBySocketAddressBytes(&self, localsocketaddress: &[<u8 as ::windows::core::DefaultType>::DefaultType], remotesocketaddress: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<XboxLiveEndpointPair>;
    fn FindEndpointPairByHostNamesAndPorts(&self, localhostname: &::core::option::Option<super::HostName>, localport: &::windows::core::HSTRING, remotehostname: &::core::option::Option<super::HostName>, remoteport: &::windows::core::HSTRING) -> ::windows::core::Result<XboxLiveEndpointPair>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXboxLiveEndpointPairStatics {
    const NAME: &'static str = "Windows.Networking.XboxLive.IXboxLiveEndpointPairStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IXboxLiveEndpointPairStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXboxLiveEndpointPairStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXboxLiveEndpointPairStaticsVtbl {
        unsafe extern "system" fn FindEndpointPairBySocketAddressBytes<Impl: IXboxLiveEndpointPairStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localSocketAddress_array_size: u32, localsocketaddress: *const u8, remoteSocketAddress_array_size: u32, remotesocketaddress: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindEndpointPairBySocketAddressBytes(::core::slice::from_raw_parts(::core::mem::transmute_copy(&localsocketaddress), localSocketAddress_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&remotesocketaddress), remoteSocketAddress_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindEndpointPairByHostNamesAndPorts<Impl: IXboxLiveEndpointPairStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localhostname: ::windows::core::RawPtr, localport: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, remotehostname: ::windows::core::RawPtr, remoteport: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindEndpointPairByHostNamesAndPorts(
                &*(&localhostname as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType),
                &*(&localport as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&remotehostname as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType),
                &*(&remoteport as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXboxLiveEndpointPairStatics>, ::windows::core::GetTrustLevel, FindEndpointPairBySocketAddressBytes::<Impl, IMPL_OFFSET>, FindEndpointPairByHostNamesAndPorts::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXboxLiveEndpointPairStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IXboxLiveEndpointPairTemplateImpl: Sized {
    fn InboundEndpointPairCreated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<XboxLiveEndpointPairTemplate, XboxLiveInboundEndpointPairCreatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInboundEndpointPairCreated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateEndpointPairDefaultAsync(&self, deviceaddress: &::core::option::Option<XboxLiveDeviceAddress>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>;
    fn CreateEndpointPairWithBehaviorsAsync(&self, deviceaddress: &::core::option::Option<XboxLiveDeviceAddress>, behaviors: XboxLiveEndpointPairCreationBehaviors) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>;
    fn CreateEndpointPairForPortsDefaultAsync(&self, deviceaddress: &::core::option::Option<XboxLiveDeviceAddress>, initiatorport: &::windows::core::HSTRING, acceptorport: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>;
    fn CreateEndpointPairForPortsWithBehaviorsAsync(&self, deviceaddress: &::core::option::Option<XboxLiveDeviceAddress>, initiatorport: &::windows::core::HSTRING, acceptorport: &::windows::core::HSTRING, behaviors: XboxLiveEndpointPairCreationBehaviors) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SocketKind(&self) -> ::windows::core::Result<XboxLiveSocketKind>;
    fn InitiatorBoundPortRangeLower(&self) -> ::windows::core::Result<u16>;
    fn InitiatorBoundPortRangeUpper(&self) -> ::windows::core::Result<u16>;
    fn AcceptorBoundPortRangeLower(&self) -> ::windows::core::Result<u16>;
    fn AcceptorBoundPortRangeUpper(&self) -> ::windows::core::Result<u16>;
    fn EndpointPairs(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveEndpointPair>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXboxLiveEndpointPairTemplate {
    const NAME: &'static str = "Windows.Networking.XboxLive.IXboxLiveEndpointPairTemplate";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IXboxLiveEndpointPairTemplateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXboxLiveEndpointPairTemplateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXboxLiveEndpointPairTemplateVtbl {
        unsafe extern "system" fn InboundEndpointPairCreated<Impl: IXboxLiveEndpointPairTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InboundEndpointPairCreated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<XboxLiveEndpointPairTemplate, XboxLiveInboundEndpointPairCreatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<XboxLiveEndpointPairTemplate, XboxLiveInboundEndpointPairCreatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInboundEndpointPairCreated<Impl: IXboxLiveEndpointPairTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveInboundEndpointPairCreated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateEndpointPairDefaultAsync<Impl: IXboxLiveEndpointPairTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceaddress: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEndpointPairDefaultAsync(&*(&deviceaddress as *const <XboxLiveDeviceAddress as ::windows::core::Abi>::Abi as *const <XboxLiveDeviceAddress as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEndpointPairWithBehaviorsAsync<Impl: IXboxLiveEndpointPairTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceaddress: ::windows::core::RawPtr, behaviors: XboxLiveEndpointPairCreationBehaviors, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEndpointPairWithBehaviorsAsync(&*(&deviceaddress as *const <XboxLiveDeviceAddress as ::windows::core::Abi>::Abi as *const <XboxLiveDeviceAddress as ::windows::core::DefaultType>::DefaultType), behaviors) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEndpointPairForPortsDefaultAsync<Impl: IXboxLiveEndpointPairTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceaddress: ::windows::core::RawPtr, initiatorport: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, acceptorport: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEndpointPairForPortsDefaultAsync(
                &*(&deviceaddress as *const <XboxLiveDeviceAddress as ::windows::core::Abi>::Abi as *const <XboxLiveDeviceAddress as ::windows::core::DefaultType>::DefaultType),
                &*(&initiatorport as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&acceptorport as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateEndpointPairForPortsWithBehaviorsAsync<Impl: IXboxLiveEndpointPairTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceaddress: ::windows::core::RawPtr, initiatorport: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, acceptorport: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, behaviors: XboxLiveEndpointPairCreationBehaviors, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEndpointPairForPortsWithBehaviorsAsync(
                &*(&deviceaddress as *const <XboxLiveDeviceAddress as ::windows::core::Abi>::Abi as *const <XboxLiveDeviceAddress as ::windows::core::DefaultType>::DefaultType),
                &*(&initiatorport as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&acceptorport as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                behaviors,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IXboxLiveEndpointPairTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SocketKind<Impl: IXboxLiveEndpointPairTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveSocketKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SocketKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitiatorBoundPortRangeLower<Impl: IXboxLiveEndpointPairTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitiatorBoundPortRangeLower() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitiatorBoundPortRangeUpper<Impl: IXboxLiveEndpointPairTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitiatorBoundPortRangeUpper() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcceptorBoundPortRangeLower<Impl: IXboxLiveEndpointPairTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcceptorBoundPortRangeLower() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcceptorBoundPortRangeUpper<Impl: IXboxLiveEndpointPairTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcceptorBoundPortRangeUpper() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndpointPairs<Impl: IXboxLiveEndpointPairTemplateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndpointPairs() {
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
            ::windows::core::GetRuntimeClassName::<IXboxLiveEndpointPairTemplate>,
            ::windows::core::GetTrustLevel,
            InboundEndpointPairCreated::<Impl, IMPL_OFFSET>,
            RemoveInboundEndpointPairCreated::<Impl, IMPL_OFFSET>,
            CreateEndpointPairDefaultAsync::<Impl, IMPL_OFFSET>,
            CreateEndpointPairWithBehaviorsAsync::<Impl, IMPL_OFFSET>,
            CreateEndpointPairForPortsDefaultAsync::<Impl, IMPL_OFFSET>,
            CreateEndpointPairForPortsWithBehaviorsAsync::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            SocketKind::<Impl, IMPL_OFFSET>,
            InitiatorBoundPortRangeLower::<Impl, IMPL_OFFSET>,
            InitiatorBoundPortRangeUpper::<Impl, IMPL_OFFSET>,
            AcceptorBoundPortRangeLower::<Impl, IMPL_OFFSET>,
            AcceptorBoundPortRangeUpper::<Impl, IMPL_OFFSET>,
            EndpointPairs::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXboxLiveEndpointPairTemplate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IXboxLiveEndpointPairTemplateStaticsImpl: Sized {
    fn GetTemplateByName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<XboxLiveEndpointPairTemplate>;
    fn Templates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveEndpointPairTemplate>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXboxLiveEndpointPairTemplateStatics {
    const NAME: &'static str = "Windows.Networking.XboxLive.IXboxLiveEndpointPairTemplateStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IXboxLiveEndpointPairTemplateStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXboxLiveEndpointPairTemplateStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXboxLiveEndpointPairTemplateStaticsVtbl {
        unsafe extern "system" fn GetTemplateByName<Impl: IXboxLiveEndpointPairTemplateStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTemplateByName(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Templates<Impl: IXboxLiveEndpointPairTemplateStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Templates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXboxLiveEndpointPairTemplateStatics>, ::windows::core::GetTrustLevel, GetTemplateByName::<Impl, IMPL_OFFSET>, Templates::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXboxLiveEndpointPairTemplateStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXboxLiveInboundEndpointPairCreatedEventArgsImpl: Sized {
    fn EndpointPair(&self) -> ::windows::core::Result<XboxLiveEndpointPair>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXboxLiveInboundEndpointPairCreatedEventArgs {
    const NAME: &'static str = "Windows.Networking.XboxLive.IXboxLiveInboundEndpointPairCreatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IXboxLiveInboundEndpointPairCreatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXboxLiveInboundEndpointPairCreatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXboxLiveInboundEndpointPairCreatedEventArgsVtbl {
        unsafe extern "system" fn EndpointPair<Impl: IXboxLiveInboundEndpointPairCreatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndpointPair() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXboxLiveInboundEndpointPairCreatedEventArgs>, ::windows::core::GetTrustLevel, EndpointPair::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXboxLiveInboundEndpointPairCreatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IXboxLiveQualityOfServiceMeasurementImpl: Sized {
    fn MeasureAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetMetricResultsForDevice(&self, deviceaddress: &::core::option::Option<XboxLiveDeviceAddress>) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>>;
    fn GetMetricResultsForMetric(&self, metric: XboxLiveQualityOfServiceMetric) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>>;
    fn GetMetricResult(&self, deviceaddress: &::core::option::Option<XboxLiveDeviceAddress>, metric: XboxLiveQualityOfServiceMetric) -> ::windows::core::Result<XboxLiveQualityOfServiceMetricResult>;
    fn GetPrivatePayloadResult(&self, deviceaddress: &::core::option::Option<XboxLiveDeviceAddress>) -> ::windows::core::Result<XboxLiveQualityOfServicePrivatePayloadResult>;
    fn Metrics(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<XboxLiveQualityOfServiceMetric>>;
    fn DeviceAddresses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<XboxLiveDeviceAddress>>;
    fn ShouldRequestPrivatePayloads(&self) -> ::windows::core::Result<bool>;
    fn SetShouldRequestPrivatePayloads(&self, value: bool) -> ::windows::core::Result<()>;
    fn TimeoutInMilliseconds(&self) -> ::windows::core::Result<u32>;
    fn SetTimeoutInMilliseconds(&self, value: u32) -> ::windows::core::Result<()>;
    fn NumberOfProbesToAttempt(&self) -> ::windows::core::Result<u32>;
    fn SetNumberOfProbesToAttempt(&self, value: u32) -> ::windows::core::Result<()>;
    fn NumberOfResultsPending(&self) -> ::windows::core::Result<u32>;
    fn MetricResults(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>>;
    fn PrivatePayloadResults(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServicePrivatePayloadResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXboxLiveQualityOfServiceMeasurement {
    const NAME: &'static str = "Windows.Networking.XboxLive.IXboxLiveQualityOfServiceMeasurement";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IXboxLiveQualityOfServiceMeasurementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXboxLiveQualityOfServiceMeasurementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXboxLiveQualityOfServiceMeasurementVtbl {
        unsafe extern "system" fn MeasureAsync<Impl: IXboxLiveQualityOfServiceMeasurementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MeasureAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMetricResultsForDevice<Impl: IXboxLiveQualityOfServiceMeasurementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceaddress: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetricResultsForDevice(&*(&deviceaddress as *const <XboxLiveDeviceAddress as ::windows::core::Abi>::Abi as *const <XboxLiveDeviceAddress as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMetricResultsForMetric<Impl: IXboxLiveQualityOfServiceMeasurementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metric: XboxLiveQualityOfServiceMetric, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetricResultsForMetric(metric) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMetricResult<Impl: IXboxLiveQualityOfServiceMeasurementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceaddress: ::windows::core::RawPtr, metric: XboxLiveQualityOfServiceMetric, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetricResult(&*(&deviceaddress as *const <XboxLiveDeviceAddress as ::windows::core::Abi>::Abi as *const <XboxLiveDeviceAddress as ::windows::core::DefaultType>::DefaultType), metric) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrivatePayloadResult<Impl: IXboxLiveQualityOfServiceMeasurementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceaddress: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrivatePayloadResult(&*(&deviceaddress as *const <XboxLiveDeviceAddress as ::windows::core::Abi>::Abi as *const <XboxLiveDeviceAddress as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Metrics<Impl: IXboxLiveQualityOfServiceMeasurementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Metrics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceAddresses<Impl: IXboxLiveQualityOfServiceMeasurementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceAddresses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShouldRequestPrivatePayloads<Impl: IXboxLiveQualityOfServiceMeasurementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShouldRequestPrivatePayloads() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShouldRequestPrivatePayloads<Impl: IXboxLiveQualityOfServiceMeasurementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShouldRequestPrivatePayloads(value).into()
        }
        unsafe extern "system" fn TimeoutInMilliseconds<Impl: IXboxLiveQualityOfServiceMeasurementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeoutInMilliseconds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimeoutInMilliseconds<Impl: IXboxLiveQualityOfServiceMeasurementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTimeoutInMilliseconds(value).into()
        }
        unsafe extern "system" fn NumberOfProbesToAttempt<Impl: IXboxLiveQualityOfServiceMeasurementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfProbesToAttempt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumberOfProbesToAttempt<Impl: IXboxLiveQualityOfServiceMeasurementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNumberOfProbesToAttempt(value).into()
        }
        unsafe extern "system" fn NumberOfResultsPending<Impl: IXboxLiveQualityOfServiceMeasurementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfResultsPending() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MetricResults<Impl: IXboxLiveQualityOfServiceMeasurementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MetricResults() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivatePayloadResults<Impl: IXboxLiveQualityOfServiceMeasurementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivatePayloadResults() {
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
            ::windows::core::GetRuntimeClassName::<IXboxLiveQualityOfServiceMeasurement>,
            ::windows::core::GetTrustLevel,
            MeasureAsync::<Impl, IMPL_OFFSET>,
            GetMetricResultsForDevice::<Impl, IMPL_OFFSET>,
            GetMetricResultsForMetric::<Impl, IMPL_OFFSET>,
            GetMetricResult::<Impl, IMPL_OFFSET>,
            GetPrivatePayloadResult::<Impl, IMPL_OFFSET>,
            Metrics::<Impl, IMPL_OFFSET>,
            DeviceAddresses::<Impl, IMPL_OFFSET>,
            ShouldRequestPrivatePayloads::<Impl, IMPL_OFFSET>,
            SetShouldRequestPrivatePayloads::<Impl, IMPL_OFFSET>,
            TimeoutInMilliseconds::<Impl, IMPL_OFFSET>,
            SetTimeoutInMilliseconds::<Impl, IMPL_OFFSET>,
            NumberOfProbesToAttempt::<Impl, IMPL_OFFSET>,
            SetNumberOfProbesToAttempt::<Impl, IMPL_OFFSET>,
            NumberOfResultsPending::<Impl, IMPL_OFFSET>,
            MetricResults::<Impl, IMPL_OFFSET>,
            PrivatePayloadResults::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXboxLiveQualityOfServiceMeasurement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IXboxLiveQualityOfServiceMeasurementStaticsImpl: Sized {
    fn PublishPrivatePayloadBytes(&self, payload: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn ClearPrivatePayload(&self) -> ::windows::core::Result<()>;
    fn MaxSimultaneousProbeConnections(&self) -> ::windows::core::Result<u32>;
    fn SetMaxSimultaneousProbeConnections(&self, value: u32) -> ::windows::core::Result<()>;
    fn IsSystemOutboundBandwidthConstrained(&self) -> ::windows::core::Result<bool>;
    fn SetIsSystemOutboundBandwidthConstrained(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsSystemInboundBandwidthConstrained(&self) -> ::windows::core::Result<bool>;
    fn SetIsSystemInboundBandwidthConstrained(&self, value: bool) -> ::windows::core::Result<()>;
    fn PublishedPrivatePayload(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetPublishedPrivatePayload(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn MaxPrivatePayloadSize(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXboxLiveQualityOfServiceMeasurementStatics {
    const NAME: &'static str = "Windows.Networking.XboxLive.IXboxLiveQualityOfServiceMeasurementStatics";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IXboxLiveQualityOfServiceMeasurementStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXboxLiveQualityOfServiceMeasurementStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXboxLiveQualityOfServiceMeasurementStaticsVtbl {
        unsafe extern "system" fn PublishPrivatePayloadBytes<Impl: IXboxLiveQualityOfServiceMeasurementStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, payload_array_size: u32, payload: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PublishPrivatePayloadBytes(::core::slice::from_raw_parts(::core::mem::transmute_copy(&payload), payload_array_size as _)).into()
        }
        unsafe extern "system" fn ClearPrivatePayload<Impl: IXboxLiveQualityOfServiceMeasurementStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearPrivatePayload().into()
        }
        unsafe extern "system" fn MaxSimultaneousProbeConnections<Impl: IXboxLiveQualityOfServiceMeasurementStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxSimultaneousProbeConnections() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxSimultaneousProbeConnections<Impl: IXboxLiveQualityOfServiceMeasurementStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxSimultaneousProbeConnections(value).into()
        }
        unsafe extern "system" fn IsSystemOutboundBandwidthConstrained<Impl: IXboxLiveQualityOfServiceMeasurementStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSystemOutboundBandwidthConstrained() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsSystemOutboundBandwidthConstrained<Impl: IXboxLiveQualityOfServiceMeasurementStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsSystemOutboundBandwidthConstrained(value).into()
        }
        unsafe extern "system" fn IsSystemInboundBandwidthConstrained<Impl: IXboxLiveQualityOfServiceMeasurementStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSystemInboundBandwidthConstrained() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsSystemInboundBandwidthConstrained<Impl: IXboxLiveQualityOfServiceMeasurementStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsSystemInboundBandwidthConstrained(value).into()
        }
        unsafe extern "system" fn PublishedPrivatePayload<Impl: IXboxLiveQualityOfServiceMeasurementStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublishedPrivatePayload() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublishedPrivatePayload<Impl: IXboxLiveQualityOfServiceMeasurementStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPublishedPrivatePayload(&*(&value as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxPrivatePayloadSize<Impl: IXboxLiveQualityOfServiceMeasurementStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxPrivatePayloadSize() {
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
            ::windows::core::GetRuntimeClassName::<IXboxLiveQualityOfServiceMeasurementStatics>,
            ::windows::core::GetTrustLevel,
            PublishPrivatePayloadBytes::<Impl, IMPL_OFFSET>,
            ClearPrivatePayload::<Impl, IMPL_OFFSET>,
            MaxSimultaneousProbeConnections::<Impl, IMPL_OFFSET>,
            SetMaxSimultaneousProbeConnections::<Impl, IMPL_OFFSET>,
            IsSystemOutboundBandwidthConstrained::<Impl, IMPL_OFFSET>,
            SetIsSystemOutboundBandwidthConstrained::<Impl, IMPL_OFFSET>,
            IsSystemInboundBandwidthConstrained::<Impl, IMPL_OFFSET>,
            SetIsSystemInboundBandwidthConstrained::<Impl, IMPL_OFFSET>,
            PublishedPrivatePayload::<Impl, IMPL_OFFSET>,
            SetPublishedPrivatePayload::<Impl, IMPL_OFFSET>,
            MaxPrivatePayloadSize::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXboxLiveQualityOfServiceMeasurementStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXboxLiveQualityOfServiceMetricResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<XboxLiveQualityOfServiceMeasurementStatus>;
    fn DeviceAddress(&self) -> ::windows::core::Result<XboxLiveDeviceAddress>;
    fn Metric(&self) -> ::windows::core::Result<XboxLiveQualityOfServiceMetric>;
    fn Value(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXboxLiveQualityOfServiceMetricResult {
    const NAME: &'static str = "Windows.Networking.XboxLive.IXboxLiveQualityOfServiceMetricResult";
}
#[cfg(feature = "implement_exclusive")]
impl IXboxLiveQualityOfServiceMetricResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXboxLiveQualityOfServiceMetricResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXboxLiveQualityOfServiceMetricResultVtbl {
        unsafe extern "system" fn Status<Impl: IXboxLiveQualityOfServiceMetricResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveQualityOfServiceMeasurementStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceAddress<Impl: IXboxLiveQualityOfServiceMetricResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Metric<Impl: IXboxLiveQualityOfServiceMetricResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveQualityOfServiceMetric) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Metric() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IXboxLiveQualityOfServiceMetricResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXboxLiveQualityOfServiceMetricResult>, ::windows::core::GetTrustLevel, Status::<Impl, IMPL_OFFSET>, DeviceAddress::<Impl, IMPL_OFFSET>, Metric::<Impl, IMPL_OFFSET>, Value::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXboxLiveQualityOfServiceMetricResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IXboxLiveQualityOfServicePrivatePayloadResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<XboxLiveQualityOfServiceMeasurementStatus>;
    fn DeviceAddress(&self) -> ::windows::core::Result<XboxLiveDeviceAddress>;
    fn Value(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXboxLiveQualityOfServicePrivatePayloadResult {
    const NAME: &'static str = "Windows.Networking.XboxLive.IXboxLiveQualityOfServicePrivatePayloadResult";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IXboxLiveQualityOfServicePrivatePayloadResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXboxLiveQualityOfServicePrivatePayloadResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXboxLiveQualityOfServicePrivatePayloadResultVtbl {
        unsafe extern "system" fn Status<Impl: IXboxLiveQualityOfServicePrivatePayloadResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveQualityOfServiceMeasurementStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceAddress<Impl: IXboxLiveQualityOfServicePrivatePayloadResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IXboxLiveQualityOfServicePrivatePayloadResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXboxLiveQualityOfServicePrivatePayloadResult>, ::windows::core::GetTrustLevel, Status::<Impl, IMPL_OFFSET>, DeviceAddress::<Impl, IMPL_OFFSET>, Value::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXboxLiveQualityOfServicePrivatePayloadResult as ::windows::core::Interface>::IID
    }
}
