#[cfg(feature = "implement_exclusive")]
pub trait IDeviceAccessChangedEventArgsImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<DeviceAccessStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceAccessChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceAccessChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceAccessChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceAccessChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceAccessChangedEventArgsVtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceAccessChangedEventArgs, BASE_OFFSET>(), Status: Status::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceAccessChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceAccessChangedEventArgs2Impl: Sized + IDeviceAccessChangedEventArgsImpl {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceAccessChangedEventArgs2 {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceAccessChangedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceAccessChangedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceAccessChangedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceAccessChangedEventArgs2Vtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceAccessChangedEventArgs2, BASE_OFFSET>(), Id: Id::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceAccessChangedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDeviceAccessInformationImpl: Sized {
    fn AccessChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DeviceAccessInformation, DeviceAccessChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccessChanged(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CurrentStatus(&mut self) -> ::windows::core::Result<DeviceAccessStatus>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDeviceAccessInformation {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceAccessInformation";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDeviceAccessInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceAccessInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceAccessInformationVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceAccessInformation, BASE_OFFSET>(),
            AccessChanged: AccessChanged::<Impl, IMPL_OFFSET>,
            RemoveAccessChanged: RemoveAccessChanged::<Impl, IMPL_OFFSET>,
            CurrentStatus: CurrentStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceAccessInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceAccessInformationStaticsImpl: Sized {
    fn CreateFromId(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<DeviceAccessInformation>;
    fn CreateFromDeviceClassId(&mut self, deviceclassid: &::windows::core::GUID) -> ::windows::core::Result<DeviceAccessInformation>;
    fn CreateFromDeviceClass(&mut self, deviceclass: DeviceClass) -> ::windows::core::Result<DeviceAccessInformation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceAccessInformationStatics {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceAccessInformationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceAccessInformationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceAccessInformationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceAccessInformationStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceAccessInformationStatics, BASE_OFFSET>(),
            CreateFromId: CreateFromId::<Impl, IMPL_OFFSET>,
            CreateFromDeviceClassId: CreateFromDeviceClassId::<Impl, IMPL_OFFSET>,
            CreateFromDeviceClass: CreateFromDeviceClass::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceAccessInformationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceConnectionChangeTriggerDetailsImpl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceConnectionChangeTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceConnectionChangeTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceConnectionChangeTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceConnectionChangeTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceConnectionChangeTriggerDetailsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceConnectionChangeTriggerDetails, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceConnectionChangeTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceDisconnectButtonClickedEventArgsImpl: Sized {
    fn Device(&mut self) -> ::windows::core::Result<DeviceInformation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceDisconnectButtonClickedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceDisconnectButtonClickedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceDisconnectButtonClickedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceDisconnectButtonClickedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceDisconnectButtonClickedEventArgsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceDisconnectButtonClickedEventArgs, BASE_OFFSET>(),
            Device: Device::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceDisconnectButtonClickedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IDeviceInformationImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn IsDefault(&mut self) -> ::windows::core::Result<bool>;
    fn EnclosureLocation(&mut self) -> ::windows::core::Result<EnclosureLocation>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
    fn Update(&mut self, updateinfo: &::core::option::Option<DeviceInformationUpdate>) -> ::windows::core::Result<()>;
    fn GetThumbnailAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceThumbnail>>;
    fn GetGlyphThumbnailAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceThumbnail>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDeviceInformation {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceInformation";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IDeviceInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceInformationVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceInformation, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            IsDefault: IsDefault::<Impl, IMPL_OFFSET>,
            EnclosureLocation: EnclosureLocation::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            Update: Update::<Impl, IMPL_OFFSET>,
            GetThumbnailAsync: GetThumbnailAsync::<Impl, IMPL_OFFSET>,
            GetGlyphThumbnailAsync: GetGlyphThumbnailAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceInformation2Impl: Sized {
    fn Kind(&mut self) -> ::windows::core::Result<DeviceInformationKind>;
    fn Pairing(&mut self) -> ::windows::core::Result<DeviceInformationPairing>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceInformation2 {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceInformation2";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceInformation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceInformation2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceInformation2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceInformation2, BASE_OFFSET>(),
            Kind: Kind::<Impl, IMPL_OFFSET>,
            Pairing: Pairing::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceInformation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDeviceInformationCustomPairingImpl: Sized {
    fn PairAsync(&mut self, pairingkindssupported: DevicePairingKinds) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>>;
    fn PairWithProtectionLevelAsync(&mut self, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>>;
    fn PairWithProtectionLevelAndSettingsAsync(&mut self, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: &::core::option::Option<IDevicePairingSettings>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>>;
    fn PairingRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DeviceInformationCustomPairing, DevicePairingRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePairingRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDeviceInformationCustomPairing {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceInformationCustomPairing";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDeviceInformationCustomPairingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceInformationCustomPairingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceInformationCustomPairingVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceInformationCustomPairing, BASE_OFFSET>(),
            PairAsync: PairAsync::<Impl, IMPL_OFFSET>,
            PairWithProtectionLevelAsync: PairWithProtectionLevelAsync::<Impl, IMPL_OFFSET>,
            PairWithProtectionLevelAndSettingsAsync: PairWithProtectionLevelAndSettingsAsync::<Impl, IMPL_OFFSET>,
            PairingRequested: PairingRequested::<Impl, IMPL_OFFSET>,
            RemovePairingRequested: RemovePairingRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceInformationCustomPairing as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDeviceInformationPairingImpl: Sized {
    fn IsPaired(&mut self) -> ::windows::core::Result<bool>;
    fn CanPair(&mut self) -> ::windows::core::Result<bool>;
    fn PairAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>>;
    fn PairWithProtectionLevelAsync(&mut self, minprotectionlevel: DevicePairingProtectionLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDeviceInformationPairing {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceInformationPairing";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDeviceInformationPairingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceInformationPairingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceInformationPairingVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceInformationPairing, BASE_OFFSET>(),
            IsPaired: IsPaired::<Impl, IMPL_OFFSET>,
            CanPair: CanPair::<Impl, IMPL_OFFSET>,
            PairAsync: PairAsync::<Impl, IMPL_OFFSET>,
            PairWithProtectionLevelAsync: PairWithProtectionLevelAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceInformationPairing as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDeviceInformationPairing2Impl: Sized {
    fn ProtectionLevel(&mut self) -> ::windows::core::Result<DevicePairingProtectionLevel>;
    fn Custom(&mut self) -> ::windows::core::Result<DeviceInformationCustomPairing>;
    fn PairWithProtectionLevelAndSettingsAsync(&mut self, minprotectionlevel: DevicePairingProtectionLevel, devicepairingsettings: &::core::option::Option<IDevicePairingSettings>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DevicePairingResult>>;
    fn UnpairAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceUnpairingResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDeviceInformationPairing2 {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceInformationPairing2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDeviceInformationPairing2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceInformationPairing2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceInformationPairing2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceInformationPairing2, BASE_OFFSET>(),
            ProtectionLevel: ProtectionLevel::<Impl, IMPL_OFFSET>,
            Custom: Custom::<Impl, IMPL_OFFSET>,
            PairWithProtectionLevelAndSettingsAsync: PairWithProtectionLevelAndSettingsAsync::<Impl, IMPL_OFFSET>,
            UnpairAsync: UnpairAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceInformationPairing2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceInformationPairingStaticsImpl: Sized {
    fn TryRegisterForAllInboundPairingRequests(&mut self, pairingkindssupported: DevicePairingKinds) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceInformationPairingStatics {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceInformationPairingStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceInformationPairingStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceInformationPairingStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceInformationPairingStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceInformationPairingStatics, BASE_OFFSET>(),
            TryRegisterForAllInboundPairingRequests: TryRegisterForAllInboundPairingRequests::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceInformationPairingStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceInformationPairingStatics2Impl: Sized {
    fn TryRegisterForAllInboundPairingRequestsWithProtectionLevel(&mut self, pairingkindssupported: DevicePairingKinds, minprotectionlevel: DevicePairingProtectionLevel) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceInformationPairingStatics2 {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceInformationPairingStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceInformationPairingStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceInformationPairingStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceInformationPairingStatics2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceInformationPairingStatics2, BASE_OFFSET>(),
            TryRegisterForAllInboundPairingRequestsWithProtectionLevel: TryRegisterForAllInboundPairingRequestsWithProtectionLevel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceInformationPairingStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDeviceInformationStaticsImpl: Sized {
    fn CreateFromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>>;
    fn CreateFromIdAsyncAdditionalProperties(&mut self, deviceid: &::windows::core::HSTRING, additionalproperties: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>>;
    fn FindAllAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>;
    fn FindAllAsyncDeviceClass(&mut self, deviceclass: DeviceClass) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>;
    fn FindAllAsyncAqsFilter(&mut self, aqsfilter: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>;
    fn FindAllAsyncAqsFilterAndAdditionalProperties(&mut self, aqsfilter: &::windows::core::HSTRING, additionalproperties: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>;
    fn CreateWatcher(&mut self) -> ::windows::core::Result<DeviceWatcher>;
    fn CreateWatcherDeviceClass(&mut self, deviceclass: DeviceClass) -> ::windows::core::Result<DeviceWatcher>;
    fn CreateWatcherAqsFilter(&mut self, aqsfilter: &::windows::core::HSTRING) -> ::windows::core::Result<DeviceWatcher>;
    fn CreateWatcherAqsFilterAndAdditionalProperties(&mut self, aqsfilter: &::windows::core::HSTRING, additionalproperties: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<DeviceWatcher>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDeviceInformationStatics {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceInformationStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDeviceInformationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceInformationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceInformationStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceInformationStatics, BASE_OFFSET>(),
            CreateFromIdAsync: CreateFromIdAsync::<Impl, IMPL_OFFSET>,
            CreateFromIdAsyncAdditionalProperties: CreateFromIdAsyncAdditionalProperties::<Impl, IMPL_OFFSET>,
            FindAllAsync: FindAllAsync::<Impl, IMPL_OFFSET>,
            FindAllAsyncDeviceClass: FindAllAsyncDeviceClass::<Impl, IMPL_OFFSET>,
            FindAllAsyncAqsFilter: FindAllAsyncAqsFilter::<Impl, IMPL_OFFSET>,
            FindAllAsyncAqsFilterAndAdditionalProperties: FindAllAsyncAqsFilterAndAdditionalProperties::<Impl, IMPL_OFFSET>,
            CreateWatcher: CreateWatcher::<Impl, IMPL_OFFSET>,
            CreateWatcherDeviceClass: CreateWatcherDeviceClass::<Impl, IMPL_OFFSET>,
            CreateWatcherAqsFilter: CreateWatcherAqsFilter::<Impl, IMPL_OFFSET>,
            CreateWatcherAqsFilterAndAdditionalProperties: CreateWatcherAqsFilterAndAdditionalProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceInformationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDeviceInformationStatics2Impl: Sized {
    fn GetAqsFilterFromDeviceClass(&mut self, deviceclass: DeviceClass) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CreateFromIdAsyncWithKindAndAdditionalProperties(&mut self, deviceid: &::windows::core::HSTRING, additionalproperties: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, kind: DeviceInformationKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>>;
    fn FindAllAsyncWithKindAqsFilterAndAdditionalProperties(&mut self, aqsfilter: &::windows::core::HSTRING, additionalproperties: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, kind: DeviceInformationKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformationCollection>>;
    fn CreateWatcherWithKindAqsFilterAndAdditionalProperties(&mut self, aqsfilter: &::windows::core::HSTRING, additionalproperties: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, kind: DeviceInformationKind) -> ::windows::core::Result<DeviceWatcher>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDeviceInformationStatics2 {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceInformationStatics2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDeviceInformationStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceInformationStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceInformationStatics2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceInformationStatics2, BASE_OFFSET>(),
            GetAqsFilterFromDeviceClass: GetAqsFilterFromDeviceClass::<Impl, IMPL_OFFSET>,
            CreateFromIdAsyncWithKindAndAdditionalProperties: CreateFromIdAsyncWithKindAndAdditionalProperties::<Impl, IMPL_OFFSET>,
            FindAllAsyncWithKindAqsFilterAndAdditionalProperties: FindAllAsyncWithKindAqsFilterAndAdditionalProperties::<Impl, IMPL_OFFSET>,
            CreateWatcherWithKindAqsFilterAndAdditionalProperties: CreateWatcherWithKindAqsFilterAndAdditionalProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceInformationStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDeviceInformationUpdateImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDeviceInformationUpdate {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceInformationUpdate";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDeviceInformationUpdateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceInformationUpdateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceInformationUpdateVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceInformationUpdate, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceInformationUpdate as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceInformationUpdate2Impl: Sized {
    fn Kind(&mut self) -> ::windows::core::Result<DeviceInformationKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceInformationUpdate2 {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceInformationUpdate2";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceInformationUpdate2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceInformationUpdate2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceInformationUpdate2Vtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceInformationUpdate2, BASE_OFFSET>(), Kind: Kind::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceInformationUpdate2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDevicePairingRequestedEventArgsImpl: Sized {
    fn DeviceInformation(&mut self) -> ::windows::core::Result<DeviceInformation>;
    fn PairingKind(&mut self) -> ::windows::core::Result<DevicePairingKinds>;
    fn Pin(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Accept(&mut self) -> ::windows::core::Result<()>;
    fn AcceptWithPin(&mut self, pin: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDevicePairingRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDevicePairingRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDevicePairingRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDevicePairingRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDevicePairingRequestedEventArgsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDevicePairingRequestedEventArgs, BASE_OFFSET>(),
            DeviceInformation: DeviceInformation::<Impl, IMPL_OFFSET>,
            PairingKind: PairingKind::<Impl, IMPL_OFFSET>,
            Pin: Pin::<Impl, IMPL_OFFSET>,
            Accept: Accept::<Impl, IMPL_OFFSET>,
            AcceptWithPin: AcceptWithPin::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDevicePairingRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IDevicePairingRequestedEventArgs2Impl: Sized {
    fn AcceptWithPasswordCredential(&mut self, passwordcredential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDevicePairingRequestedEventArgs2 {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDevicePairingRequestedEventArgs2";
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IDevicePairingRequestedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDevicePairingRequestedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDevicePairingRequestedEventArgs2Vtbl {
        unsafe extern "system" fn AcceptWithPasswordCredential<Impl: IDevicePairingRequestedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, passwordcredential: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcceptWithPasswordCredential(&*(&passwordcredential as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDevicePairingRequestedEventArgs2, BASE_OFFSET>(),
            AcceptWithPasswordCredential: AcceptWithPasswordCredential::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDevicePairingRequestedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDevicePairingResultImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<DevicePairingResultStatus>;
    fn ProtectionLevelUsed(&mut self) -> ::windows::core::Result<DevicePairingProtectionLevel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDevicePairingResult {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDevicePairingResult";
}
#[cfg(feature = "implement_exclusive")]
impl IDevicePairingResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDevicePairingResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDevicePairingResultVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDevicePairingResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            ProtectionLevelUsed: ProtectionLevelUsed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDevicePairingResult as ::windows::core::Interface>::IID
    }
}
pub trait IDevicePairingSettingsImpl: Sized {}
impl ::windows::core::RuntimeName for IDevicePairingSettings {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDevicePairingSettings";
}
impl IDevicePairingSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDevicePairingSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDevicePairingSettingsVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDevicePairingSettings, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDevicePairingSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Popups", feature = "implement_exclusive"))]
pub trait IDevicePickerImpl: Sized {
    fn Filter(&mut self) -> ::windows::core::Result<DevicePickerFilter>;
    fn Appearance(&mut self) -> ::windows::core::Result<DevicePickerAppearance>;
    fn RequestedProperties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn DeviceSelected(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DevicePicker, DeviceSelectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDeviceSelected(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DisconnectButtonClicked(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DevicePicker, DeviceDisconnectButtonClickedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisconnectButtonClicked(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DevicePickerDismissed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DevicePicker, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDevicePickerDismissed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Show(&mut self, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn ShowWithPlacement(&mut self, selection: &super::super::Foundation::Rect, placement: super::super::UI::Popups::Placement) -> ::windows::core::Result<()>;
    fn PickSingleDeviceAsync(&mut self, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>>;
    fn PickSingleDeviceAsyncWithPlacement(&mut self, selection: &super::super::Foundation::Rect, placement: super::super::UI::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceInformation>>;
    fn Hide(&mut self) -> ::windows::core::Result<()>;
    fn SetDisplayStatus(&mut self, device: &::core::option::Option<DeviceInformation>, status: &::windows::core::HSTRING, options: DevicePickerDisplayStatusOptions) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Popups", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDevicePicker {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDevicePicker";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Popups", feature = "implement_exclusive"))]
impl IDevicePickerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDevicePickerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDevicePickerVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDevicePicker, BASE_OFFSET>(),
            Filter: Filter::<Impl, IMPL_OFFSET>,
            Appearance: Appearance::<Impl, IMPL_OFFSET>,
            RequestedProperties: RequestedProperties::<Impl, IMPL_OFFSET>,
            DeviceSelected: DeviceSelected::<Impl, IMPL_OFFSET>,
            RemoveDeviceSelected: RemoveDeviceSelected::<Impl, IMPL_OFFSET>,
            DisconnectButtonClicked: DisconnectButtonClicked::<Impl, IMPL_OFFSET>,
            RemoveDisconnectButtonClicked: RemoveDisconnectButtonClicked::<Impl, IMPL_OFFSET>,
            DevicePickerDismissed: DevicePickerDismissed::<Impl, IMPL_OFFSET>,
            RemoveDevicePickerDismissed: RemoveDevicePickerDismissed::<Impl, IMPL_OFFSET>,
            Show: Show::<Impl, IMPL_OFFSET>,
            ShowWithPlacement: ShowWithPlacement::<Impl, IMPL_OFFSET>,
            PickSingleDeviceAsync: PickSingleDeviceAsync::<Impl, IMPL_OFFSET>,
            PickSingleDeviceAsyncWithPlacement: PickSingleDeviceAsyncWithPlacement::<Impl, IMPL_OFFSET>,
            Hide: Hide::<Impl, IMPL_OFFSET>,
            SetDisplayStatus: SetDisplayStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDevicePicker as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
pub trait IDevicePickerAppearanceImpl: Sized {
    fn Title(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ForegroundColor(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetForegroundColor(&mut self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn BackgroundColor(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetBackgroundColor(&mut self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn AccentColor(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetAccentColor(&mut self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn SelectedForegroundColor(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetSelectedForegroundColor(&mut self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn SelectedBackgroundColor(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetSelectedBackgroundColor(&mut self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn SelectedAccentColor(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetSelectedAccentColor(&mut self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDevicePickerAppearance {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDevicePickerAppearance";
}
#[cfg(all(feature = "UI", feature = "implement_exclusive"))]
impl IDevicePickerAppearanceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDevicePickerAppearanceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDevicePickerAppearanceVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDevicePickerAppearance, BASE_OFFSET>(),
            Title: Title::<Impl, IMPL_OFFSET>,
            SetTitle: SetTitle::<Impl, IMPL_OFFSET>,
            ForegroundColor: ForegroundColor::<Impl, IMPL_OFFSET>,
            SetForegroundColor: SetForegroundColor::<Impl, IMPL_OFFSET>,
            BackgroundColor: BackgroundColor::<Impl, IMPL_OFFSET>,
            SetBackgroundColor: SetBackgroundColor::<Impl, IMPL_OFFSET>,
            AccentColor: AccentColor::<Impl, IMPL_OFFSET>,
            SetAccentColor: SetAccentColor::<Impl, IMPL_OFFSET>,
            SelectedForegroundColor: SelectedForegroundColor::<Impl, IMPL_OFFSET>,
            SetSelectedForegroundColor: SetSelectedForegroundColor::<Impl, IMPL_OFFSET>,
            SelectedBackgroundColor: SelectedBackgroundColor::<Impl, IMPL_OFFSET>,
            SetSelectedBackgroundColor: SetSelectedBackgroundColor::<Impl, IMPL_OFFSET>,
            SelectedAccentColor: SelectedAccentColor::<Impl, IMPL_OFFSET>,
            SetSelectedAccentColor: SetSelectedAccentColor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDevicePickerAppearance as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDevicePickerFilterImpl: Sized {
    fn SupportedDeviceClasses(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<DeviceClass>>;
    fn SupportedDeviceSelectors(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDevicePickerFilter {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDevicePickerFilter";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDevicePickerFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDevicePickerFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDevicePickerFilterVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDevicePickerFilter, BASE_OFFSET>(),
            SupportedDeviceClasses: SupportedDeviceClasses::<Impl, IMPL_OFFSET>,
            SupportedDeviceSelectors: SupportedDeviceSelectors::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDevicePickerFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceSelectedEventArgsImpl: Sized {
    fn SelectedDevice(&mut self) -> ::windows::core::Result<DeviceInformation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceSelectedEventArgs {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceSelectedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceSelectedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceSelectedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceSelectedEventArgsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceSelectedEventArgs, BASE_OFFSET>(),
            SelectedDevice: SelectedDevice::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceSelectedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceUnpairingResultImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<DeviceUnpairingResultStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceUnpairingResult {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceUnpairingResult";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceUnpairingResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceUnpairingResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceUnpairingResultVtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceUnpairingResult, BASE_OFFSET>(), Status: Status::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceUnpairingResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDeviceWatcherImpl: Sized {
    fn Added(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DeviceWatcher, DeviceInformation>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Updated(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DeviceWatcher, DeviceInformationUpdate>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DeviceWatcher, DeviceInformationUpdate>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DeviceWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DeviceWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&mut self) -> ::windows::core::Result<DeviceWatcherStatus>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDeviceWatcher {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceWatcher";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDeviceWatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceWatcherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceWatcherVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceWatcher, BASE_OFFSET>(),
            Added: Added::<Impl, IMPL_OFFSET>,
            RemoveAdded: RemoveAdded::<Impl, IMPL_OFFSET>,
            Updated: Updated::<Impl, IMPL_OFFSET>,
            RemoveUpdated: RemoveUpdated::<Impl, IMPL_OFFSET>,
            Removed: Removed::<Impl, IMPL_OFFSET>,
            RemoveRemoved: RemoveRemoved::<Impl, IMPL_OFFSET>,
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
        iid == &<IDeviceWatcher as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Background", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDeviceWatcher2Impl: Sized {
    fn GetBackgroundTrigger(&mut self, requestedeventkinds: &::core::option::Option<super::super::Foundation::Collections::IIterable<DeviceWatcherEventKind>>) -> ::windows::core::Result<super::super::ApplicationModel::Background::DeviceWatcherTrigger>;
}
#[cfg(all(feature = "ApplicationModel_Background", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDeviceWatcher2 {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceWatcher2";
}
#[cfg(all(feature = "ApplicationModel_Background", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDeviceWatcher2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceWatcher2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceWatcher2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceWatcher2, BASE_OFFSET>(),
            GetBackgroundTrigger: GetBackgroundTrigger::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceWatcher2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceWatcherEventImpl: Sized {
    fn Kind(&mut self) -> ::windows::core::Result<DeviceWatcherEventKind>;
    fn DeviceInformation(&mut self) -> ::windows::core::Result<DeviceInformation>;
    fn DeviceInformationUpdate(&mut self) -> ::windows::core::Result<DeviceInformationUpdate>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceWatcherEvent {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceWatcherEvent";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceWatcherEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceWatcherEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceWatcherEventVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceWatcherEvent, BASE_OFFSET>(),
            Kind: Kind::<Impl, IMPL_OFFSET>,
            DeviceInformation: DeviceInformation::<Impl, IMPL_OFFSET>,
            DeviceInformationUpdate: DeviceInformationUpdate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceWatcherEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDeviceWatcherTriggerDetailsImpl: Sized {
    fn DeviceWatcherEvents(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<DeviceWatcherEvent>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDeviceWatcherTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceWatcherTriggerDetails";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDeviceWatcherTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceWatcherTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceWatcherTriggerDetailsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceWatcherTriggerDetails, BASE_OFFSET>(),
            DeviceWatcherEvents: DeviceWatcherEvents::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceWatcherTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEnclosureLocationImpl: Sized {
    fn InDock(&mut self) -> ::windows::core::Result<bool>;
    fn InLid(&mut self) -> ::windows::core::Result<bool>;
    fn Panel(&mut self) -> ::windows::core::Result<Panel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEnclosureLocation {
    const NAME: &'static str = "Windows.Devices.Enumeration.IEnclosureLocation";
}
#[cfg(feature = "implement_exclusive")]
impl IEnclosureLocationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnclosureLocationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnclosureLocationVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEnclosureLocation, BASE_OFFSET>(),
            InDock: InDock::<Impl, IMPL_OFFSET>,
            InLid: InLid::<Impl, IMPL_OFFSET>,
            Panel: Panel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnclosureLocation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEnclosureLocation2Impl: Sized + IEnclosureLocationImpl {
    fn RotationAngleInDegreesClockwise(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEnclosureLocation2 {
    const NAME: &'static str = "Windows.Devices.Enumeration.IEnclosureLocation2";
}
#[cfg(feature = "implement_exclusive")]
impl IEnclosureLocation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnclosureLocation2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnclosureLocation2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEnclosureLocation2, BASE_OFFSET>(),
            RotationAngleInDegreesClockwise: RotationAngleInDegreesClockwise::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnclosureLocation2 as ::windows::core::Interface>::IID
    }
}
