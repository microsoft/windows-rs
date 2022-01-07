#[cfg(feature = "implement_exclusive")]
pub trait IDeviceAccessChangedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<DeviceAccessStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceAccessChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceAccessChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceAccessChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceAccessChangedEventArgsImpl, const OFFSET: isize>() -> IDeviceAccessChangedEventArgsVtbl {
        unsafe extern "system" fn Status<Impl: IDeviceAccessChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccessStatus) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeviceAccessChangedEventArgs>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceAccessChangedEventArgs2Impl: Sized + IDeviceAccessChangedEventArgsImpl {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceAccessChangedEventArgs2 {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceAccessChangedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceAccessChangedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceAccessChangedEventArgs2Impl, const OFFSET: isize>() -> IDeviceAccessChangedEventArgs2Vtbl {
        unsafe extern "system" fn Id<Impl: IDeviceAccessChangedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeviceAccessChangedEventArgs2>, ::windows::core::GetTrustLevel, Id::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceAccessInformationImpl: Sized {
    fn AccessChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DeviceAccessInformation, DeviceAccessChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccessChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CurrentStatus(&self) -> ::windows::core::Result<DeviceAccessStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceAccessInformation {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceAccessInformation";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceAccessInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceAccessInformationImpl, const OFFSET: isize>() -> IDeviceAccessInformationVtbl {
        unsafe extern "system" fn AccessChanged<Impl: IDeviceAccessInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DeviceAccessInformation, DeviceAccessChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DeviceAccessInformation, DeviceAccessChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAccessChanged<Impl: IDeviceAccessInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAccessChanged(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CurrentStatus<Impl: IDeviceAccessInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccessStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeviceAccessInformation>, ::windows::core::GetTrustLevel, AccessChanged::<Impl, OFFSET>, RemoveAccessChanged::<Impl, OFFSET>, CurrentStatus::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceAccessInformationStaticsImpl: Sized {
    fn CreateFromId(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<DeviceAccessInformation>;
    fn CreateFromDeviceClassId(&self, deviceclassid: &::windows::core::GUID) -> ::windows::core::Result<DeviceAccessInformation>;
    fn CreateFromDeviceClass(&self, deviceclass: DeviceClass) -> ::windows::core::Result<DeviceAccessInformation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceAccessInformationStatics {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceAccessInformationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceAccessInformationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceAccessInformationStaticsImpl, const OFFSET: isize>() -> IDeviceAccessInformationStaticsVtbl {
        unsafe extern "system" fn CreateFromId<Impl: IDeviceAccessInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromId(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromDeviceClassId<Impl: IDeviceAccessInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceclassid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromDeviceClassId(&*(&deviceclassid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromDeviceClass<Impl: IDeviceAccessInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceclass: DeviceClass, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromDeviceClass(deviceclass) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeviceAccessInformationStatics>, ::windows::core::GetTrustLevel, CreateFromId::<Impl, OFFSET>, CreateFromDeviceClassId::<Impl, OFFSET>, CreateFromDeviceClass::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceConnectionChangeTriggerDetailsImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceConnectionChangeTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceConnectionChangeTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceConnectionChangeTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceConnectionChangeTriggerDetailsImpl, const OFFSET: isize>() -> IDeviceConnectionChangeTriggerDetailsVtbl {
        unsafe extern "system" fn DeviceId<Impl: IDeviceConnectionChangeTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeviceConnectionChangeTriggerDetails>, ::windows::core::GetTrustLevel, DeviceId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceDisconnectButtonClickedEventArgsImpl: Sized {
    fn Device(&self) -> ::windows::core::Result<DeviceInformation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceDisconnectButtonClickedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceDisconnectButtonClickedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceDisconnectButtonClickedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceDisconnectButtonClickedEventArgsImpl, const OFFSET: isize>() -> IDeviceDisconnectButtonClickedEventArgsVtbl {
        unsafe extern "system" fn Device<Impl: IDeviceDisconnectButtonClickedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Device() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeviceDisconnectButtonClickedEventArgs>, ::windows::core::GetTrustLevel, Device::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceInformationImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsDefault(&self) -> ::windows::core::Result<bool>;
    fn EnclosureLocation(&self) -> ::windows::core::Result<EnclosureLocation>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
    fn Update(&self, updateinfo: &::core::option::Option<DeviceInformationUpdate>) -> ::windows::core::Result<()>;
    fn GetThumbnailAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceThumbnail>>;
    fn GetGlyphThumbnailAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceThumbnail>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceInformation {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceInformation";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceInformationImpl, const OFFSET: isize>() -> IDeviceInformationVtbl {
        unsafe extern "system" fn Id<Impl: IDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Name<Impl: IDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsEnabled<Impl: IDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDefault<Impl: IDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnclosureLocation<Impl: IDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnclosureLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Update<Impl: IDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updateinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update(&*(&updateinfo as *const <DeviceInformationUpdate as ::windows::core::Abi>::Abi as *const <DeviceInformationUpdate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetThumbnailAsync<Impl: IDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThumbnailAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlyphThumbnailAsync<Impl: IDeviceInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlyphThumbnailAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDeviceInformation>,
            ::windows::core::GetTrustLevel,
            Id::<Impl, OFFSET>,
            Name::<Impl, OFFSET>,
            IsEnabled::<Impl, OFFSET>,
            IsDefault::<Impl, OFFSET>,
            EnclosureLocation::<Impl, OFFSET>,
            Properties::<Impl, OFFSET>,
            Update::<Impl, OFFSET>,
            GetThumbnailAsync::<Impl, OFFSET>,
            GetGlyphThumbnailAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceInformation2Impl: Sized {
    fn Kind(&self) -> ::windows::core::Result<DeviceInformationKind>;
    fn Pairing(&self) -> ::windows::core::Result<DeviceInformationPairing>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceInformation2 {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceInformation2";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceInformation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceInformation2Impl, const OFFSET: isize>() -> IDeviceInformation2Vtbl {
        unsafe extern "system" fn Kind<Impl: IDeviceInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DeviceInformationKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Pairing<Impl: IDeviceInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pairing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeviceInformation2>, ::windows::core::GetTrustLevel, Kind::<Impl, OFFSET>, Pairing::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceInformationCustomPairingImpl: Sized {
    fn PairAsync(&self, pairingkindssupported: DevicePairingKinds) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>>;
    fn PairWithProtectionLevelAsync(&self, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>>;
    fn PairWithProtectionLevelAndSettingsAsync(&self, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: &::core::option::Option<IDevicePairingSettings>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>>;
    fn PairingRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DeviceInformationCustomPairing, DevicePairingRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePairingRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceInformationCustomPairing {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceInformationCustomPairing";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceInformationCustomPairingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceInformationCustomPairingImpl, const OFFSET: isize>() -> IDeviceInformationCustomPairingVtbl {
        unsafe extern "system" fn PairAsync<Impl: IDeviceInformationCustomPairingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pairingkindssupported: DevicePairingKinds, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PairAsync(pairingkindssupported) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PairWithProtectionLevelAsync<Impl: IDeviceInformationCustomPairingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PairWithProtectionLevelAsync(pairingkindssupported, minprotectionlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PairWithProtectionLevelAndSettingsAsync<Impl: IDeviceInformationCustomPairingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PairWithProtectionLevelAndSettingsAsync(pairingkindssupported, minprotectionlevel, &*(&devicepairingsettings as *const <IDevicePairingSettings as ::windows::core::Abi>::Abi as *const <IDevicePairingSettings as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PairingRequested<Impl: IDeviceInformationCustomPairingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PairingRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DeviceInformationCustomPairing, DevicePairingRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DeviceInformationCustomPairing, DevicePairingRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePairingRequested<Impl: IDeviceInformationCustomPairingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePairingRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeviceInformationCustomPairing>, ::windows::core::GetTrustLevel, PairAsync::<Impl, OFFSET>, PairWithProtectionLevelAsync::<Impl, OFFSET>, PairWithProtectionLevelAndSettingsAsync::<Impl, OFFSET>, PairingRequested::<Impl, OFFSET>, RemovePairingRequested::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceInformationPairingImpl: Sized {
    fn IsPaired(&self) -> ::windows::core::Result<bool>;
    fn CanPair(&self) -> ::windows::core::Result<bool>;
    fn PairAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>>;
    fn PairWithProtectionLevelAsync(&self, minprotectionlevel: DevicePairingProtectionLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceInformationPairing {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceInformationPairing";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceInformationPairingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceInformationPairingImpl, const OFFSET: isize>() -> IDeviceInformationPairingVtbl {
        unsafe extern "system" fn IsPaired<Impl: IDeviceInformationPairingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPaired() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanPair<Impl: IDeviceInformationPairingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanPair() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PairAsync<Impl: IDeviceInformationPairingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PairAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PairWithProtectionLevelAsync<Impl: IDeviceInformationPairingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minprotectionlevel: DevicePairingProtectionLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PairWithProtectionLevelAsync(minprotectionlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeviceInformationPairing>, ::windows::core::GetTrustLevel, IsPaired::<Impl, OFFSET>, CanPair::<Impl, OFFSET>, PairAsync::<Impl, OFFSET>, PairWithProtectionLevelAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceInformationPairing2Impl: Sized {
    fn ProtectionLevel(&self) -> ::windows::core::Result<DevicePairingProtectionLevel>;
    fn Custom(&self) -> ::windows::core::Result<DeviceInformationCustomPairing>;
    fn PairWithProtectionLevelAndSettingsAsync(&self, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: &::core::option::Option<IDevicePairingSettings>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>>;
    fn UnpairAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceUnpairingResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceInformationPairing2 {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceInformationPairing2";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceInformationPairing2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceInformationPairing2Impl, const OFFSET: isize>() -> IDeviceInformationPairing2Vtbl {
        unsafe extern "system" fn ProtectionLevel<Impl: IDeviceInformationPairing2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DevicePairingProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtectionLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Custom<Impl: IDeviceInformationPairing2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Custom() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PairWithProtectionLevelAndSettingsAsync<Impl: IDeviceInformationPairing2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PairWithProtectionLevelAndSettingsAsync(minprotectionlevel, &*(&devicepairingsettings as *const <IDevicePairingSettings as ::windows::core::Abi>::Abi as *const <IDevicePairingSettings as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnpairAsync<Impl: IDeviceInformationPairing2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnpairAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeviceInformationPairing2>, ::windows::core::GetTrustLevel, ProtectionLevel::<Impl, OFFSET>, Custom::<Impl, OFFSET>, PairWithProtectionLevelAndSettingsAsync::<Impl, OFFSET>, UnpairAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceInformationPairingStaticsImpl: Sized {
    fn TryRegisterForAllInboundPairingRequests(&self, pairingkindssupported: DevicePairingKinds) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceInformationPairingStatics {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceInformationPairingStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceInformationPairingStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceInformationPairingStaticsImpl, const OFFSET: isize>() -> IDeviceInformationPairingStaticsVtbl {
        unsafe extern "system" fn TryRegisterForAllInboundPairingRequests<Impl: IDeviceInformationPairingStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pairingkindssupported: DevicePairingKinds, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryRegisterForAllInboundPairingRequests(pairingkindssupported) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeviceInformationPairingStatics>, ::windows::core::GetTrustLevel, TryRegisterForAllInboundPairingRequests::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceInformationPairingStatics2Impl: Sized {
    fn TryRegisterForAllInboundPairingRequestsWithProtectionLevel(&self, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceInformationPairingStatics2 {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceInformationPairingStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceInformationPairingStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceInformationPairingStatics2Impl, const OFFSET: isize>() -> IDeviceInformationPairingStatics2Vtbl {
        unsafe extern "system" fn TryRegisterForAllInboundPairingRequestsWithProtectionLevel<Impl: IDeviceInformationPairingStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryRegisterForAllInboundPairingRequestsWithProtectionLevel(pairingkindssupported, minprotectionlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeviceInformationPairingStatics2>, ::windows::core::GetTrustLevel, TryRegisterForAllInboundPairingRequestsWithProtectionLevel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceInformationStaticsImpl: Sized {
    fn CreateFromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>>;
    fn CreateFromIdAsyncAdditionalProperties(&self, deviceid: &::windows::core::HSTRING, additionalproperties: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>>;
    fn FindAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>;
    fn FindAllAsyncDeviceClass(&self, deviceclass: DeviceClass) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>;
    fn FindAllAsyncAqsFilter(&self, aqsfilter: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>;
    fn FindAllAsyncAqsFilterAndAdditionalProperties(&self, aqsfilter: &::windows::core::HSTRING, additionalproperties: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>;
    fn CreateWatcher(&self) -> ::windows::core::Result<DeviceWatcher>;
    fn CreateWatcherDeviceClass(&self, deviceclass: DeviceClass) -> ::windows::core::Result<DeviceWatcher>;
    fn CreateWatcherAqsFilter(&self, aqsfilter: &::windows::core::HSTRING) -> ::windows::core::Result<DeviceWatcher>;
    fn CreateWatcherAqsFilterAndAdditionalProperties(&self, aqsfilter: &::windows::core::HSTRING, additionalproperties: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<DeviceWatcher>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceInformationStatics {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceInformationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceInformationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceInformationStaticsImpl, const OFFSET: isize>() -> IDeviceInformationStaticsVtbl {
        unsafe extern "system" fn CreateFromIdAsync<Impl: IDeviceInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromIdAsyncAdditionalProperties<Impl: IDeviceInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, additionalproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromIdAsyncAdditionalProperties(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&additionalproperties as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllAsync<Impl: IDeviceInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllAsyncDeviceClass<Impl: IDeviceInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceclass: DeviceClass, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllAsyncDeviceClass(deviceclass) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllAsyncAqsFilter<Impl: IDeviceInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aqsfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllAsyncAqsFilter(&*(&aqsfilter as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllAsyncAqsFilterAndAdditionalProperties<Impl: IDeviceInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aqsfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, additionalproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllAsyncAqsFilterAndAdditionalProperties(&*(&aqsfilter as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&additionalproperties as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWatcher<Impl: IDeviceInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateWatcherDeviceClass<Impl: IDeviceInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceclass: DeviceClass, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWatcherDeviceClass(deviceclass) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWatcherAqsFilter<Impl: IDeviceInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aqsfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWatcherAqsFilter(&*(&aqsfilter as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWatcherAqsFilterAndAdditionalProperties<Impl: IDeviceInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aqsfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, additionalproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWatcherAqsFilterAndAdditionalProperties(&*(&aqsfilter as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&additionalproperties as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDeviceInformationStatics>,
            ::windows::core::GetTrustLevel,
            CreateFromIdAsync::<Impl, OFFSET>,
            CreateFromIdAsyncAdditionalProperties::<Impl, OFFSET>,
            FindAllAsync::<Impl, OFFSET>,
            FindAllAsyncDeviceClass::<Impl, OFFSET>,
            FindAllAsyncAqsFilter::<Impl, OFFSET>,
            FindAllAsyncAqsFilterAndAdditionalProperties::<Impl, OFFSET>,
            CreateWatcher::<Impl, OFFSET>,
            CreateWatcherDeviceClass::<Impl, OFFSET>,
            CreateWatcherAqsFilter::<Impl, OFFSET>,
            CreateWatcherAqsFilterAndAdditionalProperties::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceInformationStatics2Impl: Sized {
    fn GetAqsFilterFromDeviceClass(&self, deviceclass: DeviceClass) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CreateFromIdAsyncWithKindAndAdditionalProperties(&self, deviceid: &::windows::core::HSTRING, additionalproperties: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, kind: DeviceInformationKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>>;
    fn FindAllAsyncWithKindAqsFilterAndAdditionalProperties(&self, aqsfilter: &::windows::core::HSTRING, additionalproperties: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, kind: DeviceInformationKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>;
    fn CreateWatcherWithKindAqsFilterAndAdditionalProperties(&self, aqsfilter: &::windows::core::HSTRING, additionalproperties: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, kind: DeviceInformationKind) -> ::windows::core::Result<DeviceWatcher>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceInformationStatics2 {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceInformationStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceInformationStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceInformationStatics2Impl, const OFFSET: isize>() -> IDeviceInformationStatics2Vtbl {
        unsafe extern "system" fn GetAqsFilterFromDeviceClass<Impl: IDeviceInformationStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceclass: DeviceClass, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAqsFilterFromDeviceClass(deviceclass) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromIdAsyncWithKindAndAdditionalProperties<Impl: IDeviceInformationStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, additionalproperties: ::windows::core::RawPtr, kind: DeviceInformationKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromIdAsyncWithKindAndAdditionalProperties(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&additionalproperties as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType), kind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllAsyncWithKindAqsFilterAndAdditionalProperties<Impl: IDeviceInformationStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aqsfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, additionalproperties: ::windows::core::RawPtr, kind: DeviceInformationKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllAsyncWithKindAqsFilterAndAdditionalProperties(&*(&aqsfilter as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&additionalproperties as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType), kind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWatcherWithKindAqsFilterAndAdditionalProperties<Impl: IDeviceInformationStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aqsfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, additionalproperties: ::windows::core::RawPtr, kind: DeviceInformationKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWatcherWithKindAqsFilterAndAdditionalProperties(&*(&aqsfilter as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&additionalproperties as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType), kind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDeviceInformationStatics2>,
            ::windows::core::GetTrustLevel,
            GetAqsFilterFromDeviceClass::<Impl, OFFSET>,
            CreateFromIdAsyncWithKindAndAdditionalProperties::<Impl, OFFSET>,
            FindAllAsyncWithKindAqsFilterAndAdditionalProperties::<Impl, OFFSET>,
            CreateWatcherWithKindAqsFilterAndAdditionalProperties::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceInformationUpdateImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceInformationUpdate {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceInformationUpdate";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceInformationUpdateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceInformationUpdateImpl, const OFFSET: isize>() -> IDeviceInformationUpdateVtbl {
        unsafe extern "system" fn Id<Impl: IDeviceInformationUpdateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: IDeviceInformationUpdateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeviceInformationUpdate>, ::windows::core::GetTrustLevel, Id::<Impl, OFFSET>, Properties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceInformationUpdate2Impl: Sized {
    fn Kind(&self) -> ::windows::core::Result<DeviceInformationKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceInformationUpdate2 {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceInformationUpdate2";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceInformationUpdate2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceInformationUpdate2Impl, const OFFSET: isize>() -> IDeviceInformationUpdate2Vtbl {
        unsafe extern "system" fn Kind<Impl: IDeviceInformationUpdate2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DeviceInformationKind) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeviceInformationUpdate2>, ::windows::core::GetTrustLevel, Kind::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDevicePairingRequestedEventArgsImpl: Sized {
    fn DeviceInformation(&self) -> ::windows::core::Result<DeviceInformation>;
    fn PairingKind(&self) -> ::windows::core::Result<DevicePairingKinds>;
    fn Pin(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Accept(&self) -> ::windows::core::Result<()>;
    fn AcceptWithPin(&self, pin: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDevicePairingRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDevicePairingRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDevicePairingRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDevicePairingRequestedEventArgsImpl, const OFFSET: isize>() -> IDevicePairingRequestedEventArgsVtbl {
        unsafe extern "system" fn DeviceInformation<Impl: IDevicePairingRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PairingKind<Impl: IDevicePairingRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DevicePairingKinds) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PairingKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pin<Impl: IDevicePairingRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Accept<Impl: IDevicePairingRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Accept().into()
        }
        unsafe extern "system" fn AcceptWithPin<Impl: IDevicePairingRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcceptWithPin(&*(&pin as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IDevicePairingRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDevicePairingRequestedEventArgs>, ::windows::core::GetTrustLevel, DeviceInformation::<Impl, OFFSET>, PairingKind::<Impl, OFFSET>, Pin::<Impl, OFFSET>, Accept::<Impl, OFFSET>, AcceptWithPin::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDevicePairingRequestedEventArgs2Impl: Sized {
    fn AcceptWithPasswordCredential(&self, passwordcredential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDevicePairingRequestedEventArgs2 {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDevicePairingRequestedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IDevicePairingRequestedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDevicePairingRequestedEventArgs2Impl, const OFFSET: isize>() -> IDevicePairingRequestedEventArgs2Vtbl {
        unsafe extern "system" fn AcceptWithPasswordCredential<Impl: IDevicePairingRequestedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, passwordcredential: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcceptWithPasswordCredential(&*(&passwordcredential as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDevicePairingRequestedEventArgs2>, ::windows::core::GetTrustLevel, AcceptWithPasswordCredential::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDevicePairingResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<DevicePairingResultStatus>;
    fn ProtectionLevelUsed(&self) -> ::windows::core::Result<DevicePairingProtectionLevel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDevicePairingResult {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDevicePairingResult";
}
#[cfg(feature = "implement_exclusive")]
impl IDevicePairingResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDevicePairingResultImpl, const OFFSET: isize>() -> IDevicePairingResultVtbl {
        unsafe extern "system" fn Status<Impl: IDevicePairingResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DevicePairingResultStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProtectionLevelUsed<Impl: IDevicePairingResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DevicePairingProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtectionLevelUsed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDevicePairingResult>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>, ProtectionLevelUsed::<Impl, OFFSET>)
    }
}
pub trait IDevicePairingSettingsImpl: Sized {}
impl ::windows::core::RuntimeName for IDevicePairingSettings {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDevicePairingSettings";
}
impl IDevicePairingSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDevicePairingSettingsImpl, const OFFSET: isize>() -> IDevicePairingSettingsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDevicePairingSettings>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDevicePickerImpl: Sized {
    fn Filter(&self) -> ::windows::core::Result<DevicePickerFilter>;
    fn Appearance(&self) -> ::windows::core::Result<DevicePickerAppearance>;
    fn RequestedProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn DeviceSelected(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DevicePicker, DeviceSelectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDeviceSelected(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DisconnectButtonClicked(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DevicePicker, DeviceDisconnectButtonClickedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisconnectButtonClicked(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DevicePickerDismissed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DevicePicker, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDevicePickerDismissed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Show(&self, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn ShowWithPlacement(&self, selection: &super::super::Foundation::Rect, placement: super::super::UI::Popups::Placement) -> ::windows::core::Result<()>;
    fn PickSingleDeviceAsync(&self, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>>;
    fn PickSingleDeviceAsyncWithPlacement(&self, selection: &super::super::Foundation::Rect, placement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>>;
    fn Hide(&self) -> ::windows::core::Result<()>;
    fn SetDisplayStatus(&self, device: &::core::option::Option<DeviceInformation>, status: &::windows::core::HSTRING, options: DevicePickerDisplayStatusOptions) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDevicePicker {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDevicePicker";
}
#[cfg(feature = "implement_exclusive")]
impl IDevicePickerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDevicePickerImpl, const OFFSET: isize>() -> IDevicePickerVtbl {
        unsafe extern "system" fn Filter<Impl: IDevicePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Filter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Appearance<Impl: IDevicePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Appearance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedProperties<Impl: IDevicePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceSelected<Impl: IDevicePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceSelected(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DevicePicker, DeviceSelectedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DevicePicker, DeviceSelectedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDeviceSelected<Impl: IDevicePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDeviceSelected(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisconnectButtonClicked<Impl: IDevicePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisconnectButtonClicked(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DevicePicker, DeviceDisconnectButtonClickedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DevicePicker, DeviceDisconnectButtonClickedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDisconnectButtonClicked<Impl: IDevicePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDisconnectButtonClicked(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DevicePickerDismissed<Impl: IDevicePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DevicePickerDismissed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DevicePicker, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DevicePicker, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDevicePickerDismissed<Impl: IDevicePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDevicePickerDismissed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Show<Impl: IDevicePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Show(&*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShowWithPlacement<Impl: IDevicePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, placement: super::super::UI::Popups::Placement) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowWithPlacement(&*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), placement).into()
        }
        unsafe extern "system" fn PickSingleDeviceAsync<Impl: IDevicePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PickSingleDeviceAsync(&*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PickSingleDeviceAsyncWithPlacement<Impl: IDevicePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, placement: super::super::UI::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PickSingleDeviceAsyncWithPlacement(&*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), placement) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Hide<Impl: IDevicePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Hide().into()
        }
        unsafe extern "system" fn SetDisplayStatus<Impl: IDevicePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, device: ::windows::core::RawPtr, status: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: DevicePickerDisplayStatusOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayStatus(&*(&device as *const <DeviceInformation as ::windows::core::Abi>::Abi as *const <DeviceInformation as ::windows::core::DefaultType>::DefaultType), &*(&status as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), options).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDevicePicker>,
            ::windows::core::GetTrustLevel,
            Filter::<Impl, OFFSET>,
            Appearance::<Impl, OFFSET>,
            RequestedProperties::<Impl, OFFSET>,
            DeviceSelected::<Impl, OFFSET>,
            RemoveDeviceSelected::<Impl, OFFSET>,
            DisconnectButtonClicked::<Impl, OFFSET>,
            RemoveDisconnectButtonClicked::<Impl, OFFSET>,
            DevicePickerDismissed::<Impl, OFFSET>,
            RemoveDevicePickerDismissed::<Impl, OFFSET>,
            Show::<Impl, OFFSET>,
            ShowWithPlacement::<Impl, OFFSET>,
            PickSingleDeviceAsync::<Impl, OFFSET>,
            PickSingleDeviceAsyncWithPlacement::<Impl, OFFSET>,
            Hide::<Impl, OFFSET>,
            SetDisplayStatus::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDevicePickerAppearanceImpl: Sized {
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ForegroundColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetForegroundColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn BackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetBackgroundColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn AccentColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetAccentColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn SelectedForegroundColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetSelectedForegroundColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn SelectedBackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetSelectedBackgroundColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn SelectedAccentColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetSelectedAccentColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDevicePickerAppearance {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDevicePickerAppearance";
}
#[cfg(feature = "implement_exclusive")]
impl IDevicePickerAppearanceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDevicePickerAppearanceImpl, const OFFSET: isize>() -> IDevicePickerAppearanceVtbl {
        unsafe extern "system" fn Title<Impl: IDevicePickerAppearanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Impl: IDevicePickerAppearanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ForegroundColor<Impl: IDevicePickerAppearanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForegroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForegroundColor<Impl: IDevicePickerAppearanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForegroundColor(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BackgroundColor<Impl: IDevicePickerAppearanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackgroundColor<Impl: IDevicePickerAppearanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackgroundColor(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AccentColor<Impl: IDevicePickerAppearanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccentColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccentColor<Impl: IDevicePickerAppearanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccentColor(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedForegroundColor<Impl: IDevicePickerAppearanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedForegroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedForegroundColor<Impl: IDevicePickerAppearanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedForegroundColor(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedBackgroundColor<Impl: IDevicePickerAppearanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedBackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedBackgroundColor<Impl: IDevicePickerAppearanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedBackgroundColor(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedAccentColor<Impl: IDevicePickerAppearanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedAccentColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedAccentColor<Impl: IDevicePickerAppearanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedAccentColor(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDevicePickerAppearance>,
            ::windows::core::GetTrustLevel,
            Title::<Impl, OFFSET>,
            SetTitle::<Impl, OFFSET>,
            ForegroundColor::<Impl, OFFSET>,
            SetForegroundColor::<Impl, OFFSET>,
            BackgroundColor::<Impl, OFFSET>,
            SetBackgroundColor::<Impl, OFFSET>,
            AccentColor::<Impl, OFFSET>,
            SetAccentColor::<Impl, OFFSET>,
            SelectedForegroundColor::<Impl, OFFSET>,
            SetSelectedForegroundColor::<Impl, OFFSET>,
            SelectedBackgroundColor::<Impl, OFFSET>,
            SetSelectedBackgroundColor::<Impl, OFFSET>,
            SelectedAccentColor::<Impl, OFFSET>,
            SetSelectedAccentColor::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDevicePickerFilterImpl: Sized {
    fn SupportedDeviceClasses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<DeviceClass>>;
    fn SupportedDeviceSelectors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDevicePickerFilter {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDevicePickerFilter";
}
#[cfg(feature = "implement_exclusive")]
impl IDevicePickerFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDevicePickerFilterImpl, const OFFSET: isize>() -> IDevicePickerFilterVtbl {
        unsafe extern "system" fn SupportedDeviceClasses<Impl: IDevicePickerFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedDeviceClasses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedDeviceSelectors<Impl: IDevicePickerFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedDeviceSelectors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDevicePickerFilter>, ::windows::core::GetTrustLevel, SupportedDeviceClasses::<Impl, OFFSET>, SupportedDeviceSelectors::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceSelectedEventArgsImpl: Sized {
    fn SelectedDevice(&self) -> ::windows::core::Result<DeviceInformation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceSelectedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceSelectedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceSelectedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceSelectedEventArgsImpl, const OFFSET: isize>() -> IDeviceSelectedEventArgsVtbl {
        unsafe extern "system" fn SelectedDevice<Impl: IDeviceSelectedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeviceSelectedEventArgs>, ::windows::core::GetTrustLevel, SelectedDevice::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceUnpairingResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<DeviceUnpairingResultStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceUnpairingResult {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceUnpairingResult";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceUnpairingResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceUnpairingResultImpl, const OFFSET: isize>() -> IDeviceUnpairingResultVtbl {
        unsafe extern "system" fn Status<Impl: IDeviceUnpairingResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DeviceUnpairingResultStatus) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeviceUnpairingResult>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceWatcherImpl: Sized {
    fn Added(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DeviceWatcher, DeviceInformation>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Updated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DeviceWatcher, DeviceInformationUpdate>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DeviceWatcher, DeviceInformationUpdate>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DeviceWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DeviceWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<DeviceWatcherStatus>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceWatcher {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceWatcher";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceWatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceWatcherImpl, const OFFSET: isize>() -> IDeviceWatcherVtbl {
        unsafe extern "system" fn Added<Impl: IDeviceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Added(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DeviceWatcher, DeviceInformation> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DeviceWatcher, DeviceInformation> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAdded<Impl: IDeviceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Updated<Impl: IDeviceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Updated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DeviceWatcher, DeviceInformationUpdate> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DeviceWatcher, DeviceInformationUpdate> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUpdated<Impl: IDeviceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUpdated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Removed<Impl: IDeviceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Removed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DeviceWatcher, DeviceInformationUpdate> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DeviceWatcher, DeviceInformationUpdate> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRemoved<Impl: IDeviceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnumerationCompleted<Impl: IDeviceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerationCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DeviceWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DeviceWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnumerationCompleted<Impl: IDeviceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnumerationCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stopped<Impl: IDeviceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stopped(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DeviceWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DeviceWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStopped<Impl: IDeviceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStopped(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Status<Impl: IDeviceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DeviceWatcherStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Start<Impl: IDeviceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IDeviceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDeviceWatcher>,
            ::windows::core::GetTrustLevel,
            Added::<Impl, OFFSET>,
            RemoveAdded::<Impl, OFFSET>,
            Updated::<Impl, OFFSET>,
            RemoveUpdated::<Impl, OFFSET>,
            Removed::<Impl, OFFSET>,
            RemoveRemoved::<Impl, OFFSET>,
            EnumerationCompleted::<Impl, OFFSET>,
            RemoveEnumerationCompleted::<Impl, OFFSET>,
            Stopped::<Impl, OFFSET>,
            RemoveStopped::<Impl, OFFSET>,
            Status::<Impl, OFFSET>,
            Start::<Impl, OFFSET>,
            Stop::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceWatcher2Impl: Sized {
    fn GetBackgroundTrigger(&self, requestedeventkinds: &::core::option::Option<super::super::Foundation::Collections::IIterable<DeviceWatcherEventKind>>) -> ::windows::core::Result<super::super::ApplicationModel::Background::DeviceWatcherTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceWatcher2 {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceWatcher2";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceWatcher2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceWatcher2Impl, const OFFSET: isize>() -> IDeviceWatcher2Vtbl {
        unsafe extern "system" fn GetBackgroundTrigger<Impl: IDeviceWatcher2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedeventkinds: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBackgroundTrigger(&*(&requestedeventkinds as *const <super::super::Foundation::Collections::IIterable<DeviceWatcherEventKind> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<DeviceWatcherEventKind> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeviceWatcher2>, ::windows::core::GetTrustLevel, GetBackgroundTrigger::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceWatcherEventImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<DeviceWatcherEventKind>;
    fn DeviceInformation(&self) -> ::windows::core::Result<DeviceInformation>;
    fn DeviceInformationUpdate(&self) -> ::windows::core::Result<DeviceInformationUpdate>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceWatcherEvent {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceWatcherEvent";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceWatcherEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceWatcherEventImpl, const OFFSET: isize>() -> IDeviceWatcherEventVtbl {
        unsafe extern "system" fn Kind<Impl: IDeviceWatcherEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DeviceWatcherEventKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceInformation<Impl: IDeviceWatcherEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceInformationUpdate<Impl: IDeviceWatcherEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceInformationUpdate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeviceWatcherEvent>, ::windows::core::GetTrustLevel, Kind::<Impl, OFFSET>, DeviceInformation::<Impl, OFFSET>, DeviceInformationUpdate::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceWatcherTriggerDetailsImpl: Sized {
    fn DeviceWatcherEvents(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<DeviceWatcherEvent>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceWatcherTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceWatcherTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceWatcherTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceWatcherTriggerDetailsImpl, const OFFSET: isize>() -> IDeviceWatcherTriggerDetailsVtbl {
        unsafe extern "system" fn DeviceWatcherEvents<Impl: IDeviceWatcherTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceWatcherEvents() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeviceWatcherTriggerDetails>, ::windows::core::GetTrustLevel, DeviceWatcherEvents::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEnclosureLocationImpl: Sized {
    fn InDock(&self) -> ::windows::core::Result<bool>;
    fn InLid(&self) -> ::windows::core::Result<bool>;
    fn Panel(&self) -> ::windows::core::Result<Panel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEnclosureLocation {
    const NAME: &'static str = "Windows.Devices.Enumeration.IEnclosureLocation";
}
#[cfg(feature = "implement_exclusive")]
impl IEnclosureLocationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnclosureLocationImpl, const OFFSET: isize>() -> IEnclosureLocationVtbl {
        unsafe extern "system" fn InDock<Impl: IEnclosureLocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InDock() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InLid<Impl: IEnclosureLocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InLid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Panel<Impl: IEnclosureLocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Panel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Panel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnclosureLocation>, ::windows::core::GetTrustLevel, InDock::<Impl, OFFSET>, InLid::<Impl, OFFSET>, Panel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEnclosureLocation2Impl: Sized + IEnclosureLocationImpl {
    fn RotationAngleInDegreesClockwise(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEnclosureLocation2 {
    const NAME: &'static str = "Windows.Devices.Enumeration.IEnclosureLocation2";
}
#[cfg(feature = "implement_exclusive")]
impl IEnclosureLocation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnclosureLocation2Impl, const OFFSET: isize>() -> IEnclosureLocation2Vtbl {
        unsafe extern "system" fn RotationAngleInDegreesClockwise<Impl: IEnclosureLocation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationAngleInDegreesClockwise() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnclosureLocation2>, ::windows::core::GetTrustLevel, RotationAngleInDegreesClockwise::<Impl, OFFSET>)
    }
}
