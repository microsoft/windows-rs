#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IXboxLiveDeviceAddress_Impl: Sized {
    fn SnapshotChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<XboxLiveDeviceAddress, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSnapshotChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetSnapshotAsBase64(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetSnapshotAsBuffer(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn GetSnapshotAsBytes(&mut self, buffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType], byteswritten: &mut u32) -> ::windows::core::Result<()>;
    fn Compare(&mut self, otherdeviceaddress: &::core::option::Option<XboxLiveDeviceAddress>) -> ::windows::core::Result<i32>;
    fn IsValid(&mut self) -> ::windows::core::Result<bool>;
    fn IsLocal(&mut self) -> ::windows::core::Result<bool>;
    fn NetworkAccessKind(&mut self) -> ::windows::core::Result<XboxLiveNetworkAccessKind>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXboxLiveDeviceAddress {
    const NAME: &'static str = "Windows.Networking.XboxLive.IXboxLiveDeviceAddress";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IXboxLiveDeviceAddress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXboxLiveDeviceAddress_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXboxLiveDeviceAddress_Vtbl {
        unsafe extern "system" fn SnapshotChanged<Impl: IXboxLiveDeviceAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveSnapshotChanged<Impl: IXboxLiveDeviceAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSnapshotChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetSnapshotAsBase64<Impl: IXboxLiveDeviceAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSnapshotAsBuffer<Impl: IXboxLiveDeviceAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSnapshotAsBytes<Impl: IXboxLiveDeviceAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *mut u8, byteswritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSnapshotAsBytes(::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&buffer), buffer_array_size as _), ::core::mem::transmute_copy(&byteswritten)).into()
        }
        unsafe extern "system" fn Compare<Impl: IXboxLiveDeviceAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, otherdeviceaddress: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsValid<Impl: IXboxLiveDeviceAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsLocal<Impl: IXboxLiveDeviceAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NetworkAccessKind<Impl: IXboxLiveDeviceAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveNetworkAccessKind) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXboxLiveDeviceAddress, BASE_OFFSET>(),
            SnapshotChanged: SnapshotChanged::<Impl, IMPL_OFFSET>,
            RemoveSnapshotChanged: RemoveSnapshotChanged::<Impl, IMPL_OFFSET>,
            GetSnapshotAsBase64: GetSnapshotAsBase64::<Impl, IMPL_OFFSET>,
            GetSnapshotAsBuffer: GetSnapshotAsBuffer::<Impl, IMPL_OFFSET>,
            GetSnapshotAsBytes: GetSnapshotAsBytes::<Impl, IMPL_OFFSET>,
            Compare: Compare::<Impl, IMPL_OFFSET>,
            IsValid: IsValid::<Impl, IMPL_OFFSET>,
            IsLocal: IsLocal::<Impl, IMPL_OFFSET>,
            NetworkAccessKind: NetworkAccessKind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXboxLiveDeviceAddress as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IXboxLiveDeviceAddressStatics_Impl: Sized {
    fn CreateFromSnapshotBase64(&mut self, base64: &::windows::core::HSTRING) -> ::windows::core::Result<XboxLiveDeviceAddress>;
    fn CreateFromSnapshotBuffer(&mut self, buffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<XboxLiveDeviceAddress>;
    fn CreateFromSnapshotBytes(&mut self, buffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<XboxLiveDeviceAddress>;
    fn GetLocal(&mut self) -> ::windows::core::Result<XboxLiveDeviceAddress>;
    fn MaxSnapshotBytesSize(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXboxLiveDeviceAddressStatics {
    const NAME: &'static str = "Windows.Networking.XboxLive.IXboxLiveDeviceAddressStatics";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IXboxLiveDeviceAddressStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXboxLiveDeviceAddressStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXboxLiveDeviceAddressStatics_Vtbl {
        unsafe extern "system" fn CreateFromSnapshotBase64<Impl: IXboxLiveDeviceAddressStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, base64: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromSnapshotBuffer<Impl: IXboxLiveDeviceAddressStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromSnapshotBytes<Impl: IXboxLiveDeviceAddressStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer_array_size: u32, buffer: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetLocal<Impl: IXboxLiveDeviceAddressStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxSnapshotBytesSize<Impl: IXboxLiveDeviceAddressStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXboxLiveDeviceAddressStatics, BASE_OFFSET>(),
            CreateFromSnapshotBase64: CreateFromSnapshotBase64::<Impl, IMPL_OFFSET>,
            CreateFromSnapshotBuffer: CreateFromSnapshotBuffer::<Impl, IMPL_OFFSET>,
            CreateFromSnapshotBytes: CreateFromSnapshotBytes::<Impl, IMPL_OFFSET>,
            GetLocal: GetLocal::<Impl, IMPL_OFFSET>,
            MaxSnapshotBytesSize: MaxSnapshotBytesSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXboxLiveDeviceAddressStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IXboxLiveEndpointPair_Impl: Sized {
    fn StateChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<XboxLiveEndpointPair, XboxLiveEndpointPairStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DeleteAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetRemoteSocketAddressBytes(&mut self, socketaddress: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn GetLocalSocketAddressBytes(&mut self, socketaddress: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn State(&mut self) -> ::windows::core::Result<XboxLiveEndpointPairState>;
    fn Template(&mut self) -> ::windows::core::Result<XboxLiveEndpointPairTemplate>;
    fn RemoteDeviceAddress(&mut self) -> ::windows::core::Result<XboxLiveDeviceAddress>;
    fn RemoteHostName(&mut self) -> ::windows::core::Result<super::HostName>;
    fn RemotePort(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LocalHostName(&mut self) -> ::windows::core::Result<super::HostName>;
    fn LocalPort(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXboxLiveEndpointPair {
    const NAME: &'static str = "Windows.Networking.XboxLive.IXboxLiveEndpointPair";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IXboxLiveEndpointPair_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXboxLiveEndpointPair_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXboxLiveEndpointPair_Vtbl {
        unsafe extern "system" fn StateChanged<Impl: IXboxLiveEndpointPair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveStateChanged<Impl: IXboxLiveEndpointPair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeleteAsync<Impl: IXboxLiveEndpointPair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetRemoteSocketAddressBytes<Impl: IXboxLiveEndpointPair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, socketAddress_array_size: u32, socketaddress: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRemoteSocketAddressBytes(::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&socketaddress), socketAddress_array_size as _)).into()
        }
        unsafe extern "system" fn GetLocalSocketAddressBytes<Impl: IXboxLiveEndpointPair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, socketAddress_array_size: u32, socketaddress: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLocalSocketAddressBytes(::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&socketaddress), socketAddress_array_size as _)).into()
        }
        unsafe extern "system" fn State<Impl: IXboxLiveEndpointPair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveEndpointPairState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Template<Impl: IXboxLiveEndpointPair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoteDeviceAddress<Impl: IXboxLiveEndpointPair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoteHostName<Impl: IXboxLiveEndpointPair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemotePort<Impl: IXboxLiveEndpointPair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LocalHostName<Impl: IXboxLiveEndpointPair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LocalPort<Impl: IXboxLiveEndpointPair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXboxLiveEndpointPair, BASE_OFFSET>(),
            StateChanged: StateChanged::<Impl, IMPL_OFFSET>,
            RemoveStateChanged: RemoveStateChanged::<Impl, IMPL_OFFSET>,
            DeleteAsync: DeleteAsync::<Impl, IMPL_OFFSET>,
            GetRemoteSocketAddressBytes: GetRemoteSocketAddressBytes::<Impl, IMPL_OFFSET>,
            GetLocalSocketAddressBytes: GetLocalSocketAddressBytes::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            Template: Template::<Impl, IMPL_OFFSET>,
            RemoteDeviceAddress: RemoteDeviceAddress::<Impl, IMPL_OFFSET>,
            RemoteHostName: RemoteHostName::<Impl, IMPL_OFFSET>,
            RemotePort: RemotePort::<Impl, IMPL_OFFSET>,
            LocalHostName: LocalHostName::<Impl, IMPL_OFFSET>,
            LocalPort: LocalPort::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXboxLiveEndpointPair as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXboxLiveEndpointPairCreationResult_Impl: Sized {
    fn DeviceAddress(&mut self) -> ::windows::core::Result<XboxLiveDeviceAddress>;
    fn Status(&mut self) -> ::windows::core::Result<XboxLiveEndpointPairCreationStatus>;
    fn IsExistingPathEvaluation(&mut self) -> ::windows::core::Result<bool>;
    fn EndpointPair(&mut self) -> ::windows::core::Result<XboxLiveEndpointPair>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXboxLiveEndpointPairCreationResult {
    const NAME: &'static str = "Windows.Networking.XboxLive.IXboxLiveEndpointPairCreationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IXboxLiveEndpointPairCreationResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXboxLiveEndpointPairCreationResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXboxLiveEndpointPairCreationResult_Vtbl {
        unsafe extern "system" fn DeviceAddress<Impl: IXboxLiveEndpointPairCreationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Status<Impl: IXboxLiveEndpointPairCreationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveEndpointPairCreationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsExistingPathEvaluation<Impl: IXboxLiveEndpointPairCreationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EndpointPair<Impl: IXboxLiveEndpointPairCreationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXboxLiveEndpointPairCreationResult, BASE_OFFSET>(),
            DeviceAddress: DeviceAddress::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            IsExistingPathEvaluation: IsExistingPathEvaluation::<Impl, IMPL_OFFSET>,
            EndpointPair: EndpointPair::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXboxLiveEndpointPairCreationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXboxLiveEndpointPairStateChangedEventArgs_Impl: Sized {
    fn OldState(&mut self) -> ::windows::core::Result<XboxLiveEndpointPairState>;
    fn NewState(&mut self) -> ::windows::core::Result<XboxLiveEndpointPairState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXboxLiveEndpointPairStateChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.XboxLive.IXboxLiveEndpointPairStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IXboxLiveEndpointPairStateChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXboxLiveEndpointPairStateChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXboxLiveEndpointPairStateChangedEventArgs_Vtbl {
        unsafe extern "system" fn OldState<Impl: IXboxLiveEndpointPairStateChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveEndpointPairState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NewState<Impl: IXboxLiveEndpointPairStateChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveEndpointPairState) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXboxLiveEndpointPairStateChangedEventArgs, BASE_OFFSET>(),
            OldState: OldState::<Impl, IMPL_OFFSET>,
            NewState: NewState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXboxLiveEndpointPairStateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXboxLiveEndpointPairStatics_Impl: Sized {
    fn FindEndpointPairBySocketAddressBytes(&mut self, localsocketaddress: &[<u8 as ::windows::core::DefaultType>::DefaultType], remotesocketaddress: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<XboxLiveEndpointPair>;
    fn FindEndpointPairByHostNamesAndPorts(&mut self, localhostname: &::core::option::Option<super::HostName>, localport: &::windows::core::HSTRING, remotehostname: &::core::option::Option<super::HostName>, remoteport: &::windows::core::HSTRING) -> ::windows::core::Result<XboxLiveEndpointPair>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXboxLiveEndpointPairStatics {
    const NAME: &'static str = "Windows.Networking.XboxLive.IXboxLiveEndpointPairStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IXboxLiveEndpointPairStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXboxLiveEndpointPairStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXboxLiveEndpointPairStatics_Vtbl {
        unsafe extern "system" fn FindEndpointPairBySocketAddressBytes<Impl: IXboxLiveEndpointPairStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localSocketAddress_array_size: u32, localsocketaddress: *const u8, remoteSocketAddress_array_size: u32, remotesocketaddress: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindEndpointPairByHostNamesAndPorts<Impl: IXboxLiveEndpointPairStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localhostname: ::windows::core::RawPtr, localport: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, remotehostname: ::windows::core::RawPtr, remoteport: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXboxLiveEndpointPairStatics, BASE_OFFSET>(),
            FindEndpointPairBySocketAddressBytes: FindEndpointPairBySocketAddressBytes::<Impl, IMPL_OFFSET>,
            FindEndpointPairByHostNamesAndPorts: FindEndpointPairByHostNamesAndPorts::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXboxLiveEndpointPairStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IXboxLiveEndpointPairTemplate_Impl: Sized {
    fn InboundEndpointPairCreated(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<XboxLiveEndpointPairTemplate, XboxLiveInboundEndpointPairCreatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInboundEndpointPairCreated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateEndpointPairDefaultAsync(&mut self, deviceaddress: &::core::option::Option<XboxLiveDeviceAddress>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>;
    fn CreateEndpointPairWithBehaviorsAsync(&mut self, deviceaddress: &::core::option::Option<XboxLiveDeviceAddress>, behaviors: XboxLiveEndpointPairCreationBehaviors) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>;
    fn CreateEndpointPairForPortsDefaultAsync(&mut self, deviceaddress: &::core::option::Option<XboxLiveDeviceAddress>, initiatorport: &::windows::core::HSTRING, acceptorport: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>;
    fn CreateEndpointPairForPortsWithBehaviorsAsync(&mut self, deviceaddress: &::core::option::Option<XboxLiveDeviceAddress>, initiatorport: &::windows::core::HSTRING, acceptorport: &::windows::core::HSTRING, behaviors: XboxLiveEndpointPairCreationBehaviors) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SocketKind(&mut self) -> ::windows::core::Result<XboxLiveSocketKind>;
    fn InitiatorBoundPortRangeLower(&mut self) -> ::windows::core::Result<u16>;
    fn InitiatorBoundPortRangeUpper(&mut self) -> ::windows::core::Result<u16>;
    fn AcceptorBoundPortRangeLower(&mut self) -> ::windows::core::Result<u16>;
    fn AcceptorBoundPortRangeUpper(&mut self) -> ::windows::core::Result<u16>;
    fn EndpointPairs(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveEndpointPair>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXboxLiveEndpointPairTemplate {
    const NAME: &'static str = "Windows.Networking.XboxLive.IXboxLiveEndpointPairTemplate";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IXboxLiveEndpointPairTemplate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXboxLiveEndpointPairTemplate_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXboxLiveEndpointPairTemplate_Vtbl {
        unsafe extern "system" fn InboundEndpointPairCreated<Impl: IXboxLiveEndpointPairTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveInboundEndpointPairCreated<Impl: IXboxLiveEndpointPairTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveInboundEndpointPairCreated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateEndpointPairDefaultAsync<Impl: IXboxLiveEndpointPairTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceaddress: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateEndpointPairWithBehaviorsAsync<Impl: IXboxLiveEndpointPairTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceaddress: ::windows::core::RawPtr, behaviors: XboxLiveEndpointPairCreationBehaviors, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateEndpointPairForPortsDefaultAsync<Impl: IXboxLiveEndpointPairTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceaddress: ::windows::core::RawPtr, initiatorport: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, acceptorport: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateEndpointPairForPortsWithBehaviorsAsync<Impl: IXboxLiveEndpointPairTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceaddress: ::windows::core::RawPtr, initiatorport: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, acceptorport: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, behaviors: XboxLiveEndpointPairCreationBehaviors, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Name<Impl: IXboxLiveEndpointPairTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SocketKind<Impl: IXboxLiveEndpointPairTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveSocketKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InitiatorBoundPortRangeLower<Impl: IXboxLiveEndpointPairTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InitiatorBoundPortRangeUpper<Impl: IXboxLiveEndpointPairTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AcceptorBoundPortRangeLower<Impl: IXboxLiveEndpointPairTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AcceptorBoundPortRangeUpper<Impl: IXboxLiveEndpointPairTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EndpointPairs<Impl: IXboxLiveEndpointPairTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXboxLiveEndpointPairTemplate, BASE_OFFSET>(),
            InboundEndpointPairCreated: InboundEndpointPairCreated::<Impl, IMPL_OFFSET>,
            RemoveInboundEndpointPairCreated: RemoveInboundEndpointPairCreated::<Impl, IMPL_OFFSET>,
            CreateEndpointPairDefaultAsync: CreateEndpointPairDefaultAsync::<Impl, IMPL_OFFSET>,
            CreateEndpointPairWithBehaviorsAsync: CreateEndpointPairWithBehaviorsAsync::<Impl, IMPL_OFFSET>,
            CreateEndpointPairForPortsDefaultAsync: CreateEndpointPairForPortsDefaultAsync::<Impl, IMPL_OFFSET>,
            CreateEndpointPairForPortsWithBehaviorsAsync: CreateEndpointPairForPortsWithBehaviorsAsync::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            SocketKind: SocketKind::<Impl, IMPL_OFFSET>,
            InitiatorBoundPortRangeLower: InitiatorBoundPortRangeLower::<Impl, IMPL_OFFSET>,
            InitiatorBoundPortRangeUpper: InitiatorBoundPortRangeUpper::<Impl, IMPL_OFFSET>,
            AcceptorBoundPortRangeLower: AcceptorBoundPortRangeLower::<Impl, IMPL_OFFSET>,
            AcceptorBoundPortRangeUpper: AcceptorBoundPortRangeUpper::<Impl, IMPL_OFFSET>,
            EndpointPairs: EndpointPairs::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXboxLiveEndpointPairTemplate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IXboxLiveEndpointPairTemplateStatics_Impl: Sized {
    fn GetTemplateByName(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<XboxLiveEndpointPairTemplate>;
    fn Templates(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveEndpointPairTemplate>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXboxLiveEndpointPairTemplateStatics {
    const NAME: &'static str = "Windows.Networking.XboxLive.IXboxLiveEndpointPairTemplateStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IXboxLiveEndpointPairTemplateStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXboxLiveEndpointPairTemplateStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXboxLiveEndpointPairTemplateStatics_Vtbl {
        unsafe extern "system" fn GetTemplateByName<Impl: IXboxLiveEndpointPairTemplateStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Templates<Impl: IXboxLiveEndpointPairTemplateStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXboxLiveEndpointPairTemplateStatics, BASE_OFFSET>(),
            GetTemplateByName: GetTemplateByName::<Impl, IMPL_OFFSET>,
            Templates: Templates::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXboxLiveEndpointPairTemplateStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXboxLiveInboundEndpointPairCreatedEventArgs_Impl: Sized {
    fn EndpointPair(&mut self) -> ::windows::core::Result<XboxLiveEndpointPair>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXboxLiveInboundEndpointPairCreatedEventArgs {
    const NAME: &'static str = "Windows.Networking.XboxLive.IXboxLiveInboundEndpointPairCreatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IXboxLiveInboundEndpointPairCreatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXboxLiveInboundEndpointPairCreatedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXboxLiveInboundEndpointPairCreatedEventArgs_Vtbl {
        unsafe extern "system" fn EndpointPair<Impl: IXboxLiveInboundEndpointPairCreatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXboxLiveInboundEndpointPairCreatedEventArgs, BASE_OFFSET>(),
            EndpointPair: EndpointPair::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXboxLiveInboundEndpointPairCreatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IXboxLiveQualityOfServiceMeasurement_Impl: Sized {
    fn MeasureAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetMetricResultsForDevice(&mut self, deviceaddress: &::core::option::Option<XboxLiveDeviceAddress>) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>>;
    fn GetMetricResultsForMetric(&mut self, metric: XboxLiveQualityOfServiceMetric) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>>;
    fn GetMetricResult(&mut self, deviceaddress: &::core::option::Option<XboxLiveDeviceAddress>, metric: XboxLiveQualityOfServiceMetric) -> ::windows::core::Result<XboxLiveQualityOfServiceMetricResult>;
    fn GetPrivatePayloadResult(&mut self, deviceaddress: &::core::option::Option<XboxLiveDeviceAddress>) -> ::windows::core::Result<XboxLiveQualityOfServicePrivatePayloadResult>;
    fn Metrics(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<XboxLiveQualityOfServiceMetric>>;
    fn DeviceAddresses(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<XboxLiveDeviceAddress>>;
    fn ShouldRequestPrivatePayloads(&mut self) -> ::windows::core::Result<bool>;
    fn SetShouldRequestPrivatePayloads(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn TimeoutInMilliseconds(&mut self) -> ::windows::core::Result<u32>;
    fn SetTimeoutInMilliseconds(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn NumberOfProbesToAttempt(&mut self) -> ::windows::core::Result<u32>;
    fn SetNumberOfProbesToAttempt(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn NumberOfResultsPending(&mut self) -> ::windows::core::Result<u32>;
    fn MetricResults(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServiceMetricResult>>;
    fn PrivatePayloadResults(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<XboxLiveQualityOfServicePrivatePayloadResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXboxLiveQualityOfServiceMeasurement {
    const NAME: &'static str = "Windows.Networking.XboxLive.IXboxLiveQualityOfServiceMeasurement";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IXboxLiveQualityOfServiceMeasurement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXboxLiveQualityOfServiceMeasurement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXboxLiveQualityOfServiceMeasurement_Vtbl {
        unsafe extern "system" fn MeasureAsync<Impl: IXboxLiveQualityOfServiceMeasurement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetMetricResultsForDevice<Impl: IXboxLiveQualityOfServiceMeasurement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceaddress: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetMetricResultsForMetric<Impl: IXboxLiveQualityOfServiceMeasurement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metric: XboxLiveQualityOfServiceMetric, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetMetricResult<Impl: IXboxLiveQualityOfServiceMeasurement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceaddress: ::windows::core::RawPtr, metric: XboxLiveQualityOfServiceMetric, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetPrivatePayloadResult<Impl: IXboxLiveQualityOfServiceMeasurement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceaddress: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Metrics<Impl: IXboxLiveQualityOfServiceMeasurement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceAddresses<Impl: IXboxLiveQualityOfServiceMeasurement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShouldRequestPrivatePayloads<Impl: IXboxLiveQualityOfServiceMeasurement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetShouldRequestPrivatePayloads<Impl: IXboxLiveQualityOfServiceMeasurement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShouldRequestPrivatePayloads(value).into()
        }
        unsafe extern "system" fn TimeoutInMilliseconds<Impl: IXboxLiveQualityOfServiceMeasurement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTimeoutInMilliseconds<Impl: IXboxLiveQualityOfServiceMeasurement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTimeoutInMilliseconds(value).into()
        }
        unsafe extern "system" fn NumberOfProbesToAttempt<Impl: IXboxLiveQualityOfServiceMeasurement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetNumberOfProbesToAttempt<Impl: IXboxLiveQualityOfServiceMeasurement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNumberOfProbesToAttempt(value).into()
        }
        unsafe extern "system" fn NumberOfResultsPending<Impl: IXboxLiveQualityOfServiceMeasurement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MetricResults<Impl: IXboxLiveQualityOfServiceMeasurement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PrivatePayloadResults<Impl: IXboxLiveQualityOfServiceMeasurement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXboxLiveQualityOfServiceMeasurement, BASE_OFFSET>(),
            MeasureAsync: MeasureAsync::<Impl, IMPL_OFFSET>,
            GetMetricResultsForDevice: GetMetricResultsForDevice::<Impl, IMPL_OFFSET>,
            GetMetricResultsForMetric: GetMetricResultsForMetric::<Impl, IMPL_OFFSET>,
            GetMetricResult: GetMetricResult::<Impl, IMPL_OFFSET>,
            GetPrivatePayloadResult: GetPrivatePayloadResult::<Impl, IMPL_OFFSET>,
            Metrics: Metrics::<Impl, IMPL_OFFSET>,
            DeviceAddresses: DeviceAddresses::<Impl, IMPL_OFFSET>,
            ShouldRequestPrivatePayloads: ShouldRequestPrivatePayloads::<Impl, IMPL_OFFSET>,
            SetShouldRequestPrivatePayloads: SetShouldRequestPrivatePayloads::<Impl, IMPL_OFFSET>,
            TimeoutInMilliseconds: TimeoutInMilliseconds::<Impl, IMPL_OFFSET>,
            SetTimeoutInMilliseconds: SetTimeoutInMilliseconds::<Impl, IMPL_OFFSET>,
            NumberOfProbesToAttempt: NumberOfProbesToAttempt::<Impl, IMPL_OFFSET>,
            SetNumberOfProbesToAttempt: SetNumberOfProbesToAttempt::<Impl, IMPL_OFFSET>,
            NumberOfResultsPending: NumberOfResultsPending::<Impl, IMPL_OFFSET>,
            MetricResults: MetricResults::<Impl, IMPL_OFFSET>,
            PrivatePayloadResults: PrivatePayloadResults::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXboxLiveQualityOfServiceMeasurement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IXboxLiveQualityOfServiceMeasurementStatics_Impl: Sized {
    fn PublishPrivatePayloadBytes(&mut self, payload: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn ClearPrivatePayload(&mut self) -> ::windows::core::Result<()>;
    fn MaxSimultaneousProbeConnections(&mut self) -> ::windows::core::Result<u32>;
    fn SetMaxSimultaneousProbeConnections(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn IsSystemOutboundBandwidthConstrained(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsSystemOutboundBandwidthConstrained(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsSystemInboundBandwidthConstrained(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsSystemInboundBandwidthConstrained(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn PublishedPrivatePayload(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn SetPublishedPrivatePayload(&mut self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn MaxPrivatePayloadSize(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXboxLiveQualityOfServiceMeasurementStatics {
    const NAME: &'static str = "Windows.Networking.XboxLive.IXboxLiveQualityOfServiceMeasurementStatics";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IXboxLiveQualityOfServiceMeasurementStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXboxLiveQualityOfServiceMeasurementStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXboxLiveQualityOfServiceMeasurementStatics_Vtbl {
        unsafe extern "system" fn PublishPrivatePayloadBytes<Impl: IXboxLiveQualityOfServiceMeasurementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, payload_array_size: u32, payload: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PublishPrivatePayloadBytes(::core::slice::from_raw_parts(::core::mem::transmute_copy(&payload), payload_array_size as _)).into()
        }
        unsafe extern "system" fn ClearPrivatePayload<Impl: IXboxLiveQualityOfServiceMeasurementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearPrivatePayload().into()
        }
        unsafe extern "system" fn MaxSimultaneousProbeConnections<Impl: IXboxLiveQualityOfServiceMeasurementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMaxSimultaneousProbeConnections<Impl: IXboxLiveQualityOfServiceMeasurementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxSimultaneousProbeConnections(value).into()
        }
        unsafe extern "system" fn IsSystemOutboundBandwidthConstrained<Impl: IXboxLiveQualityOfServiceMeasurementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsSystemOutboundBandwidthConstrained<Impl: IXboxLiveQualityOfServiceMeasurementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsSystemOutboundBandwidthConstrained(value).into()
        }
        unsafe extern "system" fn IsSystemInboundBandwidthConstrained<Impl: IXboxLiveQualityOfServiceMeasurementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsSystemInboundBandwidthConstrained<Impl: IXboxLiveQualityOfServiceMeasurementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsSystemInboundBandwidthConstrained(value).into()
        }
        unsafe extern "system" fn PublishedPrivatePayload<Impl: IXboxLiveQualityOfServiceMeasurementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPublishedPrivatePayload<Impl: IXboxLiveQualityOfServiceMeasurementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPublishedPrivatePayload(&*(&value as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxPrivatePayloadSize<Impl: IXboxLiveQualityOfServiceMeasurementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXboxLiveQualityOfServiceMeasurementStatics, BASE_OFFSET>(),
            PublishPrivatePayloadBytes: PublishPrivatePayloadBytes::<Impl, IMPL_OFFSET>,
            ClearPrivatePayload: ClearPrivatePayload::<Impl, IMPL_OFFSET>,
            MaxSimultaneousProbeConnections: MaxSimultaneousProbeConnections::<Impl, IMPL_OFFSET>,
            SetMaxSimultaneousProbeConnections: SetMaxSimultaneousProbeConnections::<Impl, IMPL_OFFSET>,
            IsSystemOutboundBandwidthConstrained: IsSystemOutboundBandwidthConstrained::<Impl, IMPL_OFFSET>,
            SetIsSystemOutboundBandwidthConstrained: SetIsSystemOutboundBandwidthConstrained::<Impl, IMPL_OFFSET>,
            IsSystemInboundBandwidthConstrained: IsSystemInboundBandwidthConstrained::<Impl, IMPL_OFFSET>,
            SetIsSystemInboundBandwidthConstrained: SetIsSystemInboundBandwidthConstrained::<Impl, IMPL_OFFSET>,
            PublishedPrivatePayload: PublishedPrivatePayload::<Impl, IMPL_OFFSET>,
            SetPublishedPrivatePayload: SetPublishedPrivatePayload::<Impl, IMPL_OFFSET>,
            MaxPrivatePayloadSize: MaxPrivatePayloadSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXboxLiveQualityOfServiceMeasurementStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXboxLiveQualityOfServiceMetricResult_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<XboxLiveQualityOfServiceMeasurementStatus>;
    fn DeviceAddress(&mut self) -> ::windows::core::Result<XboxLiveDeviceAddress>;
    fn Metric(&mut self) -> ::windows::core::Result<XboxLiveQualityOfServiceMetric>;
    fn Value(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXboxLiveQualityOfServiceMetricResult {
    const NAME: &'static str = "Windows.Networking.XboxLive.IXboxLiveQualityOfServiceMetricResult";
}
#[cfg(feature = "implement_exclusive")]
impl IXboxLiveQualityOfServiceMetricResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXboxLiveQualityOfServiceMetricResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXboxLiveQualityOfServiceMetricResult_Vtbl {
        unsafe extern "system" fn Status<Impl: IXboxLiveQualityOfServiceMetricResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveQualityOfServiceMeasurementStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceAddress<Impl: IXboxLiveQualityOfServiceMetricResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Metric<Impl: IXboxLiveQualityOfServiceMetricResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveQualityOfServiceMetric) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Value<Impl: IXboxLiveQualityOfServiceMetricResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXboxLiveQualityOfServiceMetricResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            DeviceAddress: DeviceAddress::<Impl, IMPL_OFFSET>,
            Metric: Metric::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXboxLiveQualityOfServiceMetricResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IXboxLiveQualityOfServicePrivatePayloadResult_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<XboxLiveQualityOfServiceMeasurementStatus>;
    fn DeviceAddress(&mut self) -> ::windows::core::Result<XboxLiveDeviceAddress>;
    fn Value(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXboxLiveQualityOfServicePrivatePayloadResult {
    const NAME: &'static str = "Windows.Networking.XboxLive.IXboxLiveQualityOfServicePrivatePayloadResult";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IXboxLiveQualityOfServicePrivatePayloadResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXboxLiveQualityOfServicePrivatePayloadResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXboxLiveQualityOfServicePrivatePayloadResult_Vtbl {
        unsafe extern "system" fn Status<Impl: IXboxLiveQualityOfServicePrivatePayloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut XboxLiveQualityOfServiceMeasurementStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceAddress<Impl: IXboxLiveQualityOfServicePrivatePayloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Value<Impl: IXboxLiveQualityOfServicePrivatePayloadResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXboxLiveQualityOfServicePrivatePayloadResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            DeviceAddress: DeviceAddress::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXboxLiveQualityOfServicePrivatePayloadResult as ::windows::core::Interface>::IID
    }
}
