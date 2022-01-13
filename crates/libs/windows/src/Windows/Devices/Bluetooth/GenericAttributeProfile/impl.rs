#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IGattCharacteristicImpl: Sized {
    fn GetDescriptors(&mut self, descriptoruuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDescriptor>>;
    fn CharacteristicProperties(&mut self) -> ::windows::core::Result<GattCharacteristicProperties>;
    fn ProtectionLevel(&mut self) -> ::windows::core::Result<GattProtectionLevel>;
    fn SetProtectionLevel(&mut self, value: GattProtectionLevel) -> ::windows::core::Result<()>;
    fn UserDescription(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Uuid(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn AttributeHandle(&mut self) -> ::windows::core::Result<u16>;
    fn PresentationFormats(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattPresentationFormat>>;
    fn ReadValueAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadResult>>;
    fn ReadValueWithCacheModeAsync(&mut self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadResult>>;
    fn WriteValueAsync(&mut self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>;
    fn WriteValueWithOptionAsync(&mut self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, writeoption: GattWriteOption) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>;
    fn ReadClientCharacteristicConfigurationDescriptorAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadClientCharacteristicConfigurationDescriptorResult>>;
    fn WriteClientCharacteristicConfigurationDescriptorAsync(&mut self, clientcharacteristicconfigurationdescriptorvalue: GattClientCharacteristicConfigurationDescriptorValue) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>;
    fn ValueChanged(&mut self, valuechangedhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattCharacteristic, GattValueChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveValueChanged(&mut self, valuechangedeventcookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattCharacteristic {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattCharacteristic";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IGattCharacteristicVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattCharacteristicImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattCharacteristicVtbl {
        unsafe extern "system" fn GetDescriptors<Impl: IGattCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptoruuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescriptors(&*(&descriptoruuid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CharacteristicProperties<Impl: IGattCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattCharacteristicProperties) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CharacteristicProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProtectionLevel<Impl: IGattCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetProtectionLevel<Impl: IGattCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: GattProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProtectionLevel(value).into()
        }
        unsafe extern "system" fn UserDescription<Impl: IGattCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uuid<Impl: IGattCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uuid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttributeHandle<Impl: IGattCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttributeHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresentationFormats<Impl: IGattCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresentationFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadValueAsync<Impl: IGattCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadValueAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadValueWithCacheModeAsync<Impl: IGattCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cachemode: super::BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadValueWithCacheModeAsync(cachemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteValueAsync<Impl: IGattCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteValueAsync(&*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteValueWithOptionAsync<Impl: IGattCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, writeoption: GattWriteOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteValueWithOptionAsync(&*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), writeoption) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadClientCharacteristicConfigurationDescriptorAsync<Impl: IGattCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadClientCharacteristicConfigurationDescriptorAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteClientCharacteristicConfigurationDescriptorAsync<Impl: IGattCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientcharacteristicconfigurationdescriptorvalue: GattClientCharacteristicConfigurationDescriptorValue, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteClientCharacteristicConfigurationDescriptorAsync(clientcharacteristicconfigurationdescriptorvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValueChanged<Impl: IGattCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuechangedhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValueChanged(&*(&valuechangedhandler as *const <super::super::super::Foundation::TypedEventHandler<GattCharacteristic, GattValueChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<GattCharacteristic, GattValueChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveValueChanged<Impl: IGattCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuechangedeventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveValueChanged(&*(&valuechangedeventcookie as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattCharacteristic, BASE_OFFSET>(),
            GetDescriptors: GetDescriptors::<Impl, IMPL_OFFSET>,
            CharacteristicProperties: CharacteristicProperties::<Impl, IMPL_OFFSET>,
            ProtectionLevel: ProtectionLevel::<Impl, IMPL_OFFSET>,
            SetProtectionLevel: SetProtectionLevel::<Impl, IMPL_OFFSET>,
            UserDescription: UserDescription::<Impl, IMPL_OFFSET>,
            Uuid: Uuid::<Impl, IMPL_OFFSET>,
            AttributeHandle: AttributeHandle::<Impl, IMPL_OFFSET>,
            PresentationFormats: PresentationFormats::<Impl, IMPL_OFFSET>,
            ReadValueAsync: ReadValueAsync::<Impl, IMPL_OFFSET>,
            ReadValueWithCacheModeAsync: ReadValueWithCacheModeAsync::<Impl, IMPL_OFFSET>,
            WriteValueAsync: WriteValueAsync::<Impl, IMPL_OFFSET>,
            WriteValueWithOptionAsync: WriteValueWithOptionAsync::<Impl, IMPL_OFFSET>,
            ReadClientCharacteristicConfigurationDescriptorAsync: ReadClientCharacteristicConfigurationDescriptorAsync::<Impl, IMPL_OFFSET>,
            WriteClientCharacteristicConfigurationDescriptorAsync: WriteClientCharacteristicConfigurationDescriptorAsync::<Impl, IMPL_OFFSET>,
            ValueChanged: ValueChanged::<Impl, IMPL_OFFSET>,
            RemoveValueChanged: RemoveValueChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattCharacteristic as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IGattCharacteristic2Impl: Sized + IGattCharacteristicImpl {
    fn Service(&mut self) -> ::windows::core::Result<GattDeviceService>;
    fn GetAllDescriptors(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDescriptor>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattCharacteristic2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattCharacteristic2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IGattCharacteristic2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattCharacteristic2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattCharacteristic2Vtbl {
        unsafe extern "system" fn Service<Impl: IGattCharacteristic2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Service() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllDescriptors<Impl: IGattCharacteristic2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllDescriptors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattCharacteristic2, BASE_OFFSET>(),
            Service: Service::<Impl, IMPL_OFFSET>,
            GetAllDescriptors: GetAllDescriptors::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattCharacteristic2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IGattCharacteristic3Impl: Sized {
    fn GetDescriptorsAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>>;
    fn GetDescriptorsWithCacheModeAsync(&mut self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>>;
    fn GetDescriptorsForUuidAsync(&mut self, descriptoruuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>>;
    fn GetDescriptorsForUuidWithCacheModeAsync(&mut self, descriptoruuid: &::windows::core::GUID, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>>;
    fn WriteValueWithResultAsync(&mut self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>;
    fn WriteValueWithResultAndOptionAsync(&mut self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, writeoption: GattWriteOption) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>;
    fn WriteClientCharacteristicConfigurationDescriptorWithResultAsync(&mut self, clientcharacteristicconfigurationdescriptorvalue: GattClientCharacteristicConfigurationDescriptorValue) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattCharacteristic3 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattCharacteristic3";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IGattCharacteristic3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattCharacteristic3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattCharacteristic3Vtbl {
        unsafe extern "system" fn GetDescriptorsAsync<Impl: IGattCharacteristic3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescriptorsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescriptorsWithCacheModeAsync<Impl: IGattCharacteristic3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cachemode: super::BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescriptorsWithCacheModeAsync(cachemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescriptorsForUuidAsync<Impl: IGattCharacteristic3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptoruuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescriptorsForUuidAsync(&*(&descriptoruuid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescriptorsForUuidWithCacheModeAsync<Impl: IGattCharacteristic3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptoruuid: ::windows::core::GUID, cachemode: super::BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescriptorsForUuidWithCacheModeAsync(&*(&descriptoruuid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), cachemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteValueWithResultAsync<Impl: IGattCharacteristic3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteValueWithResultAsync(&*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteValueWithResultAndOptionAsync<Impl: IGattCharacteristic3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, writeoption: GattWriteOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteValueWithResultAndOptionAsync(&*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), writeoption) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteClientCharacteristicConfigurationDescriptorWithResultAsync<Impl: IGattCharacteristic3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientcharacteristicconfigurationdescriptorvalue: GattClientCharacteristicConfigurationDescriptorValue, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteClientCharacteristicConfigurationDescriptorWithResultAsync(clientcharacteristicconfigurationdescriptorvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattCharacteristic3, BASE_OFFSET>(),
            GetDescriptorsAsync: GetDescriptorsAsync::<Impl, IMPL_OFFSET>,
            GetDescriptorsWithCacheModeAsync: GetDescriptorsWithCacheModeAsync::<Impl, IMPL_OFFSET>,
            GetDescriptorsForUuidAsync: GetDescriptorsForUuidAsync::<Impl, IMPL_OFFSET>,
            GetDescriptorsForUuidWithCacheModeAsync: GetDescriptorsForUuidWithCacheModeAsync::<Impl, IMPL_OFFSET>,
            WriteValueWithResultAsync: WriteValueWithResultAsync::<Impl, IMPL_OFFSET>,
            WriteValueWithResultAndOptionAsync: WriteValueWithResultAndOptionAsync::<Impl, IMPL_OFFSET>,
            WriteClientCharacteristicConfigurationDescriptorWithResultAsync: WriteClientCharacteristicConfigurationDescriptorWithResultAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattCharacteristic3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristicStaticsImpl: Sized {
    fn ConvertShortIdToUuid(&mut self, shortid: u16) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattCharacteristicStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattCharacteristicStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGattCharacteristicStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattCharacteristicStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattCharacteristicStaticsVtbl {
        unsafe extern "system" fn ConvertShortIdToUuid<Impl: IGattCharacteristicStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortid: u16, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertShortIdToUuid(shortid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattCharacteristicStatics, BASE_OFFSET>(),
            ConvertShortIdToUuid: ConvertShortIdToUuid::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattCharacteristicStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristicUuidsStaticsImpl: Sized {
    fn BatteryLevel(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn BloodPressureFeature(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn BloodPressureMeasurement(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn BodySensorLocation(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CscFeature(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CscMeasurement(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GlucoseFeature(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GlucoseMeasurement(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GlucoseMeasurementContext(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn HeartRateControlPoint(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn HeartRateMeasurement(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IntermediateCuffPressure(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IntermediateTemperature(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn MeasurementInterval(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RecordAccessControlPoint(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RscFeature(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RscMeasurement(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SCControlPoint(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SensorLocation(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TemperatureMeasurement(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TemperatureType(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattCharacteristicUuidsStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattCharacteristicUuidsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGattCharacteristicUuidsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattCharacteristicUuidsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattCharacteristicUuidsStaticsVtbl {
        unsafe extern "system" fn BatteryLevel<Impl: IGattCharacteristicUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BatteryLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BloodPressureFeature<Impl: IGattCharacteristicUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BloodPressureFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BloodPressureMeasurement<Impl: IGattCharacteristicUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BloodPressureMeasurement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BodySensorLocation<Impl: IGattCharacteristicUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BodySensorLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CscFeature<Impl: IGattCharacteristicUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CscFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CscMeasurement<Impl: IGattCharacteristicUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CscMeasurement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GlucoseFeature<Impl: IGattCharacteristicUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GlucoseFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GlucoseMeasurement<Impl: IGattCharacteristicUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GlucoseMeasurement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GlucoseMeasurementContext<Impl: IGattCharacteristicUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GlucoseMeasurementContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeartRateControlPoint<Impl: IGattCharacteristicUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeartRateControlPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeartRateMeasurement<Impl: IGattCharacteristicUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeartRateMeasurement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IntermediateCuffPressure<Impl: IGattCharacteristicUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IntermediateCuffPressure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IntermediateTemperature<Impl: IGattCharacteristicUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IntermediateTemperature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MeasurementInterval<Impl: IGattCharacteristicUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MeasurementInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecordAccessControlPoint<Impl: IGattCharacteristicUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecordAccessControlPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RscFeature<Impl: IGattCharacteristicUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RscFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RscMeasurement<Impl: IGattCharacteristicUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RscMeasurement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SCControlPoint<Impl: IGattCharacteristicUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SCControlPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SensorLocation<Impl: IGattCharacteristicUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SensorLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TemperatureMeasurement<Impl: IGattCharacteristicUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TemperatureMeasurement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TemperatureType<Impl: IGattCharacteristicUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TemperatureType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattCharacteristicUuidsStatics, BASE_OFFSET>(),
            BatteryLevel: BatteryLevel::<Impl, IMPL_OFFSET>,
            BloodPressureFeature: BloodPressureFeature::<Impl, IMPL_OFFSET>,
            BloodPressureMeasurement: BloodPressureMeasurement::<Impl, IMPL_OFFSET>,
            BodySensorLocation: BodySensorLocation::<Impl, IMPL_OFFSET>,
            CscFeature: CscFeature::<Impl, IMPL_OFFSET>,
            CscMeasurement: CscMeasurement::<Impl, IMPL_OFFSET>,
            GlucoseFeature: GlucoseFeature::<Impl, IMPL_OFFSET>,
            GlucoseMeasurement: GlucoseMeasurement::<Impl, IMPL_OFFSET>,
            GlucoseMeasurementContext: GlucoseMeasurementContext::<Impl, IMPL_OFFSET>,
            HeartRateControlPoint: HeartRateControlPoint::<Impl, IMPL_OFFSET>,
            HeartRateMeasurement: HeartRateMeasurement::<Impl, IMPL_OFFSET>,
            IntermediateCuffPressure: IntermediateCuffPressure::<Impl, IMPL_OFFSET>,
            IntermediateTemperature: IntermediateTemperature::<Impl, IMPL_OFFSET>,
            MeasurementInterval: MeasurementInterval::<Impl, IMPL_OFFSET>,
            RecordAccessControlPoint: RecordAccessControlPoint::<Impl, IMPL_OFFSET>,
            RscFeature: RscFeature::<Impl, IMPL_OFFSET>,
            RscMeasurement: RscMeasurement::<Impl, IMPL_OFFSET>,
            SCControlPoint: SCControlPoint::<Impl, IMPL_OFFSET>,
            SensorLocation: SensorLocation::<Impl, IMPL_OFFSET>,
            TemperatureMeasurement: TemperatureMeasurement::<Impl, IMPL_OFFSET>,
            TemperatureType: TemperatureType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattCharacteristicUuidsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristicUuidsStatics2Impl: Sized {
    fn AlertCategoryId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn AlertCategoryIdBitMask(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn AlertLevel(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn AlertNotificationControlPoint(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn AlertStatus(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GapAppearance(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn BootKeyboardInputReport(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn BootKeyboardOutputReport(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn BootMouseInputReport(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CurrentTime(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CyclingPowerControlPoint(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CyclingPowerFeature(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CyclingPowerMeasurement(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CyclingPowerVector(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn DateTime(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn DayDateTime(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn DayOfWeek(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GapDeviceName(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn DstOffset(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ExactTime256(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn FirmwareRevisionString(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn HardwareRevisionString(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn HidControlPoint(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn HidInformation(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Ieee1107320601RegulatoryCertificationDataList(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn LnControlPoint(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn LnFeature(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn LocalTimeInformation(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn LocationAndSpeed(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ManufacturerNameString(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ModelNumberString(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Navigation(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn NewAlert(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GapPeripheralPreferredConnectionParameters(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GapPeripheralPrivacyFlag(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn PnpId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn PositionQuality(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ProtocolMode(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GapReconnectionAddress(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ReferenceTimeInformation(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Report(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ReportMap(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RingerControlPoint(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RingerSetting(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ScanIntervalWindow(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ScanRefresh(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SerialNumberString(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GattServiceChanged(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SoftwareRevisionString(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SupportedNewAlertCategory(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SupportUnreadAlertCategory(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SystemId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TimeAccuracy(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TimeSource(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TimeUpdateControlPoint(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TimeUpdateState(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TimeWithDst(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TimeZone(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TxPowerLevel(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn UnreadAlertStatus(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattCharacteristicUuidsStatics2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattCharacteristicUuidsStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IGattCharacteristicUuidsStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattCharacteristicUuidsStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattCharacteristicUuidsStatics2Vtbl {
        unsafe extern "system" fn AlertCategoryId<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlertCategoryId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlertCategoryIdBitMask<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlertCategoryIdBitMask() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlertLevel<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlertLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlertNotificationControlPoint<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlertNotificationControlPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AlertStatus<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlertStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GapAppearance<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GapAppearance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BootKeyboardInputReport<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BootKeyboardInputReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BootKeyboardOutputReport<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BootKeyboardOutputReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BootMouseInputReport<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BootMouseInputReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentTime<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CyclingPowerControlPoint<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CyclingPowerControlPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CyclingPowerFeature<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CyclingPowerFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CyclingPowerMeasurement<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CyclingPowerMeasurement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CyclingPowerVector<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CyclingPowerVector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DateTime<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DayDateTime<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DayDateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DayOfWeek<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DayOfWeek() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GapDeviceName<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GapDeviceName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DstOffset<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DstOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExactTime256<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExactTime256() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirmwareRevisionString<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirmwareRevisionString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareRevisionString<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HardwareRevisionString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HidControlPoint<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HidControlPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HidInformation<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HidInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ieee1107320601RegulatoryCertificationDataList<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ieee1107320601RegulatoryCertificationDataList() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LnControlPoint<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LnControlPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LnFeature<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LnFeature() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalTimeInformation<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalTimeInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocationAndSpeed<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocationAndSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManufacturerNameString<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManufacturerNameString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelNumberString<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModelNumberString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Navigation<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Navigation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewAlert<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewAlert() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GapPeripheralPreferredConnectionParameters<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GapPeripheralPreferredConnectionParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GapPeripheralPrivacyFlag<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GapPeripheralPrivacyFlag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PnpId<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PnpId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PositionQuality<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PositionQuality() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProtocolMode<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtocolMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GapReconnectionAddress<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GapReconnectionAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferenceTimeInformation<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReferenceTimeInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Report<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Report() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportMap<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportMap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RingerControlPoint<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RingerControlPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RingerSetting<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RingerSetting() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScanIntervalWindow<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScanIntervalWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScanRefresh<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScanRefresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SerialNumberString<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SerialNumberString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GattServiceChanged<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GattServiceChanged() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SoftwareRevisionString<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SoftwareRevisionString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedNewAlertCategory<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedNewAlertCategory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportUnreadAlertCategory<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportUnreadAlertCategory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemId<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TimeAccuracy<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeAccuracy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimeSource<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimeUpdateControlPoint<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeUpdateControlPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimeUpdateState<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeUpdateState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimeWithDst<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeWithDst() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimeZone<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeZone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxPowerLevel<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxPowerLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnreadAlertStatus<Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnreadAlertStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattCharacteristicUuidsStatics2, BASE_OFFSET>(),
            AlertCategoryId: AlertCategoryId::<Impl, IMPL_OFFSET>,
            AlertCategoryIdBitMask: AlertCategoryIdBitMask::<Impl, IMPL_OFFSET>,
            AlertLevel: AlertLevel::<Impl, IMPL_OFFSET>,
            AlertNotificationControlPoint: AlertNotificationControlPoint::<Impl, IMPL_OFFSET>,
            AlertStatus: AlertStatus::<Impl, IMPL_OFFSET>,
            GapAppearance: GapAppearance::<Impl, IMPL_OFFSET>,
            BootKeyboardInputReport: BootKeyboardInputReport::<Impl, IMPL_OFFSET>,
            BootKeyboardOutputReport: BootKeyboardOutputReport::<Impl, IMPL_OFFSET>,
            BootMouseInputReport: BootMouseInputReport::<Impl, IMPL_OFFSET>,
            CurrentTime: CurrentTime::<Impl, IMPL_OFFSET>,
            CyclingPowerControlPoint: CyclingPowerControlPoint::<Impl, IMPL_OFFSET>,
            CyclingPowerFeature: CyclingPowerFeature::<Impl, IMPL_OFFSET>,
            CyclingPowerMeasurement: CyclingPowerMeasurement::<Impl, IMPL_OFFSET>,
            CyclingPowerVector: CyclingPowerVector::<Impl, IMPL_OFFSET>,
            DateTime: DateTime::<Impl, IMPL_OFFSET>,
            DayDateTime: DayDateTime::<Impl, IMPL_OFFSET>,
            DayOfWeek: DayOfWeek::<Impl, IMPL_OFFSET>,
            GapDeviceName: GapDeviceName::<Impl, IMPL_OFFSET>,
            DstOffset: DstOffset::<Impl, IMPL_OFFSET>,
            ExactTime256: ExactTime256::<Impl, IMPL_OFFSET>,
            FirmwareRevisionString: FirmwareRevisionString::<Impl, IMPL_OFFSET>,
            HardwareRevisionString: HardwareRevisionString::<Impl, IMPL_OFFSET>,
            HidControlPoint: HidControlPoint::<Impl, IMPL_OFFSET>,
            HidInformation: HidInformation::<Impl, IMPL_OFFSET>,
            Ieee1107320601RegulatoryCertificationDataList: Ieee1107320601RegulatoryCertificationDataList::<Impl, IMPL_OFFSET>,
            LnControlPoint: LnControlPoint::<Impl, IMPL_OFFSET>,
            LnFeature: LnFeature::<Impl, IMPL_OFFSET>,
            LocalTimeInformation: LocalTimeInformation::<Impl, IMPL_OFFSET>,
            LocationAndSpeed: LocationAndSpeed::<Impl, IMPL_OFFSET>,
            ManufacturerNameString: ManufacturerNameString::<Impl, IMPL_OFFSET>,
            ModelNumberString: ModelNumberString::<Impl, IMPL_OFFSET>,
            Navigation: Navigation::<Impl, IMPL_OFFSET>,
            NewAlert: NewAlert::<Impl, IMPL_OFFSET>,
            GapPeripheralPreferredConnectionParameters: GapPeripheralPreferredConnectionParameters::<Impl, IMPL_OFFSET>,
            GapPeripheralPrivacyFlag: GapPeripheralPrivacyFlag::<Impl, IMPL_OFFSET>,
            PnpId: PnpId::<Impl, IMPL_OFFSET>,
            PositionQuality: PositionQuality::<Impl, IMPL_OFFSET>,
            ProtocolMode: ProtocolMode::<Impl, IMPL_OFFSET>,
            GapReconnectionAddress: GapReconnectionAddress::<Impl, IMPL_OFFSET>,
            ReferenceTimeInformation: ReferenceTimeInformation::<Impl, IMPL_OFFSET>,
            Report: Report::<Impl, IMPL_OFFSET>,
            ReportMap: ReportMap::<Impl, IMPL_OFFSET>,
            RingerControlPoint: RingerControlPoint::<Impl, IMPL_OFFSET>,
            RingerSetting: RingerSetting::<Impl, IMPL_OFFSET>,
            ScanIntervalWindow: ScanIntervalWindow::<Impl, IMPL_OFFSET>,
            ScanRefresh: ScanRefresh::<Impl, IMPL_OFFSET>,
            SerialNumberString: SerialNumberString::<Impl, IMPL_OFFSET>,
            GattServiceChanged: GattServiceChanged::<Impl, IMPL_OFFSET>,
            SoftwareRevisionString: SoftwareRevisionString::<Impl, IMPL_OFFSET>,
            SupportedNewAlertCategory: SupportedNewAlertCategory::<Impl, IMPL_OFFSET>,
            SupportUnreadAlertCategory: SupportUnreadAlertCategory::<Impl, IMPL_OFFSET>,
            SystemId: SystemId::<Impl, IMPL_OFFSET>,
            TimeAccuracy: TimeAccuracy::<Impl, IMPL_OFFSET>,
            TimeSource: TimeSource::<Impl, IMPL_OFFSET>,
            TimeUpdateControlPoint: TimeUpdateControlPoint::<Impl, IMPL_OFFSET>,
            TimeUpdateState: TimeUpdateState::<Impl, IMPL_OFFSET>,
            TimeWithDst: TimeWithDst::<Impl, IMPL_OFFSET>,
            TimeZone: TimeZone::<Impl, IMPL_OFFSET>,
            TxPowerLevel: TxPowerLevel::<Impl, IMPL_OFFSET>,
            UnreadAlertStatus: UnreadAlertStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattCharacteristicUuidsStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGattCharacteristicsResultImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<GattCommunicationStatus>;
    fn ProtocolError(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>>;
    fn Characteristics(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattCharacteristic>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattCharacteristicsResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattCharacteristicsResult";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGattCharacteristicsResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattCharacteristicsResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattCharacteristicsResultVtbl {
        unsafe extern "system" fn Status<Impl: IGattCharacteristicsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattCommunicationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProtocolError<Impl: IGattCharacteristicsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtocolError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Characteristics<Impl: IGattCharacteristicsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Characteristics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattCharacteristicsResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            ProtocolError: ProtocolError::<Impl, IMPL_OFFSET>,
            Characteristics: Characteristics::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattCharacteristicsResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGattClientNotificationResultImpl: Sized {
    fn SubscribedClient(&mut self) -> ::windows::core::Result<GattSubscribedClient>;
    fn Status(&mut self) -> ::windows::core::Result<GattCommunicationStatus>;
    fn ProtocolError(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattClientNotificationResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattClientNotificationResult";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGattClientNotificationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattClientNotificationResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattClientNotificationResultVtbl {
        unsafe extern "system" fn SubscribedClient<Impl: IGattClientNotificationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubscribedClient() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IGattClientNotificationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattCommunicationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProtocolError<Impl: IGattClientNotificationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtocolError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattClientNotificationResult, BASE_OFFSET>(),
            SubscribedClient: SubscribedClient::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            ProtocolError: ProtocolError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattClientNotificationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattClientNotificationResult2Impl: Sized {
    fn BytesSent(&mut self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattClientNotificationResult2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattClientNotificationResult2";
}
#[cfg(feature = "implement_exclusive")]
impl IGattClientNotificationResult2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattClientNotificationResult2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattClientNotificationResult2Vtbl {
        unsafe extern "system" fn BytesSent<Impl: IGattClientNotificationResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BytesSent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattClientNotificationResult2, BASE_OFFSET>(),
            BytesSent: BytesSent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattClientNotificationResult2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IGattDescriptorImpl: Sized {
    fn ProtectionLevel(&mut self) -> ::windows::core::Result<GattProtectionLevel>;
    fn SetProtectionLevel(&mut self, value: GattProtectionLevel) -> ::windows::core::Result<()>;
    fn Uuid(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn AttributeHandle(&mut self) -> ::windows::core::Result<u16>;
    fn ReadValueAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadResult>>;
    fn ReadValueWithCacheModeAsync(&mut self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadResult>>;
    fn WriteValueAsync(&mut self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattDescriptor {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattDescriptor";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IGattDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattDescriptorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattDescriptorVtbl {
        unsafe extern "system" fn ProtectionLevel<Impl: IGattDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetProtectionLevel<Impl: IGattDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: GattProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProtectionLevel(value).into()
        }
        unsafe extern "system" fn Uuid<Impl: IGattDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uuid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttributeHandle<Impl: IGattDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttributeHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadValueAsync<Impl: IGattDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadValueAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadValueWithCacheModeAsync<Impl: IGattDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cachemode: super::BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadValueWithCacheModeAsync(cachemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteValueAsync<Impl: IGattDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteValueAsync(&*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattDescriptor, BASE_OFFSET>(),
            ProtectionLevel: ProtectionLevel::<Impl, IMPL_OFFSET>,
            SetProtectionLevel: SetProtectionLevel::<Impl, IMPL_OFFSET>,
            Uuid: Uuid::<Impl, IMPL_OFFSET>,
            AttributeHandle: AttributeHandle::<Impl, IMPL_OFFSET>,
            ReadValueAsync: ReadValueAsync::<Impl, IMPL_OFFSET>,
            ReadValueWithCacheModeAsync: ReadValueWithCacheModeAsync::<Impl, IMPL_OFFSET>,
            WriteValueAsync: WriteValueAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattDescriptor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IGattDescriptor2Impl: Sized {
    fn WriteValueWithResultAsync(&mut self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattDescriptor2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattDescriptor2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IGattDescriptor2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattDescriptor2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattDescriptor2Vtbl {
        unsafe extern "system" fn WriteValueWithResultAsync<Impl: IGattDescriptor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteValueWithResultAsync(&*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattDescriptor2, BASE_OFFSET>(),
            WriteValueWithResultAsync: WriteValueWithResultAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattDescriptor2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattDescriptorStaticsImpl: Sized {
    fn ConvertShortIdToUuid(&mut self, shortid: u16) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattDescriptorStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattDescriptorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGattDescriptorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattDescriptorStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattDescriptorStaticsVtbl {
        unsafe extern "system" fn ConvertShortIdToUuid<Impl: IGattDescriptorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortid: u16, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertShortIdToUuid(shortid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattDescriptorStatics, BASE_OFFSET>(),
            ConvertShortIdToUuid: ConvertShortIdToUuid::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattDescriptorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattDescriptorUuidsStaticsImpl: Sized {
    fn CharacteristicAggregateFormat(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CharacteristicExtendedProperties(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CharacteristicPresentationFormat(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CharacteristicUserDescription(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ClientCharacteristicConfiguration(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ServerCharacteristicConfiguration(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattDescriptorUuidsStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattDescriptorUuidsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGattDescriptorUuidsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattDescriptorUuidsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattDescriptorUuidsStaticsVtbl {
        unsafe extern "system" fn CharacteristicAggregateFormat<Impl: IGattDescriptorUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CharacteristicAggregateFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CharacteristicExtendedProperties<Impl: IGattDescriptorUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CharacteristicExtendedProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CharacteristicPresentationFormat<Impl: IGattDescriptorUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CharacteristicPresentationFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CharacteristicUserDescription<Impl: IGattDescriptorUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CharacteristicUserDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientCharacteristicConfiguration<Impl: IGattDescriptorUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientCharacteristicConfiguration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerCharacteristicConfiguration<Impl: IGattDescriptorUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerCharacteristicConfiguration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattDescriptorUuidsStatics, BASE_OFFSET>(),
            CharacteristicAggregateFormat: CharacteristicAggregateFormat::<Impl, IMPL_OFFSET>,
            CharacteristicExtendedProperties: CharacteristicExtendedProperties::<Impl, IMPL_OFFSET>,
            CharacteristicPresentationFormat: CharacteristicPresentationFormat::<Impl, IMPL_OFFSET>,
            CharacteristicUserDescription: CharacteristicUserDescription::<Impl, IMPL_OFFSET>,
            ClientCharacteristicConfiguration: ClientCharacteristicConfiguration::<Impl, IMPL_OFFSET>,
            ServerCharacteristicConfiguration: ServerCharacteristicConfiguration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattDescriptorUuidsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGattDescriptorsResultImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<GattCommunicationStatus>;
    fn ProtocolError(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>>;
    fn Descriptors(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDescriptor>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattDescriptorsResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattDescriptorsResult";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGattDescriptorsResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattDescriptorsResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattDescriptorsResultVtbl {
        unsafe extern "system" fn Status<Impl: IGattDescriptorsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattCommunicationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProtocolError<Impl: IGattDescriptorsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtocolError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Descriptors<Impl: IGattDescriptorsResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Descriptors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattDescriptorsResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            ProtocolError: ProtocolError::<Impl, IMPL_OFFSET>,
            Descriptors: Descriptors::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattDescriptorsResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGattDeviceServiceImpl: Sized + IClosableImpl {
    fn GetCharacteristics(&mut self, characteristicuuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattCharacteristic>>;
    fn GetIncludedServices(&mut self, serviceuuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>>;
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Uuid(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn AttributeHandle(&mut self) -> ::windows::core::Result<u16>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattDeviceService {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattDeviceService";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGattDeviceServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattDeviceServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattDeviceServiceVtbl {
        unsafe extern "system" fn GetCharacteristics<Impl: IGattDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, characteristicuuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCharacteristics(&*(&characteristicuuid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIncludedServices<Impl: IGattDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceuuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIncludedServices(&*(&serviceuuid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Impl: IGattDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Uuid<Impl: IGattDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uuid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttributeHandle<Impl: IGattDeviceServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttributeHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattDeviceService, BASE_OFFSET>(),
            GetCharacteristics: GetCharacteristics::<Impl, IMPL_OFFSET>,
            GetIncludedServices: GetIncludedServices::<Impl, IMPL_OFFSET>,
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            Uuid: Uuid::<Impl, IMPL_OFFSET>,
            AttributeHandle: AttributeHandle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattDeviceService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGattDeviceService2Impl: Sized + IClosableImpl + IGattDeviceServiceImpl {
    fn Device(&mut self) -> ::windows::core::Result<super::BluetoothLEDevice>;
    fn ParentServices(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>>;
    fn GetAllCharacteristics(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattCharacteristic>>;
    fn GetAllIncludedServices(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattDeviceService2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattDeviceService2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGattDeviceService2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattDeviceService2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattDeviceService2Vtbl {
        unsafe extern "system" fn Device<Impl: IGattDeviceService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ParentServices<Impl: IGattDeviceService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParentServices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllCharacteristics<Impl: IGattDeviceService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllCharacteristics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllIncludedServices<Impl: IGattDeviceService2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllIncludedServices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattDeviceService2, BASE_OFFSET>(),
            Device: Device::<Impl, IMPL_OFFSET>,
            ParentServices: ParentServices::<Impl, IMPL_OFFSET>,
            GetAllCharacteristics: GetAllCharacteristics::<Impl, IMPL_OFFSET>,
            GetAllIncludedServices: GetAllIncludedServices::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattDeviceService2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGattDeviceService3Impl: Sized {
    fn DeviceAccessInformation(&mut self) -> ::windows::core::Result<super::super::Enumeration::DeviceAccessInformation>;
    fn Session(&mut self) -> ::windows::core::Result<GattSession>;
    fn SharingMode(&mut self) -> ::windows::core::Result<GattSharingMode>;
    fn RequestAccessAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::Enumeration::DeviceAccessStatus>>;
    fn OpenAsync(&mut self, sharingmode: GattSharingMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattOpenStatus>>;
    fn GetCharacteristicsAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>>;
    fn GetCharacteristicsWithCacheModeAsync(&mut self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>>;
    fn GetCharacteristicsForUuidAsync(&mut self, characteristicuuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>>;
    fn GetCharacteristicsForUuidWithCacheModeAsync(&mut self, characteristicuuid: &::windows::core::GUID, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>>;
    fn GetIncludedServicesAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>>;
    fn GetIncludedServicesWithCacheModeAsync(&mut self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>>;
    fn GetIncludedServicesForUuidAsync(&mut self, serviceuuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>>;
    fn GetIncludedServicesForUuidWithCacheModeAsync(&mut self, serviceuuid: &::windows::core::GUID, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattDeviceService3 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattDeviceService3";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "implement_exclusive"))]
impl IGattDeviceService3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattDeviceService3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattDeviceService3Vtbl {
        unsafe extern "system" fn DeviceAccessInformation<Impl: IGattDeviceService3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceAccessInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Session<Impl: IGattDeviceService3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SharingMode<Impl: IGattDeviceService3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattSharingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SharingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessAsync<Impl: IGattDeviceService3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenAsync<Impl: IGattDeviceService3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharingmode: GattSharingMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenAsync(sharingmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCharacteristicsAsync<Impl: IGattDeviceService3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCharacteristicsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCharacteristicsWithCacheModeAsync<Impl: IGattDeviceService3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cachemode: super::BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCharacteristicsWithCacheModeAsync(cachemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCharacteristicsForUuidAsync<Impl: IGattDeviceService3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, characteristicuuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCharacteristicsForUuidAsync(&*(&characteristicuuid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCharacteristicsForUuidWithCacheModeAsync<Impl: IGattDeviceService3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, characteristicuuid: ::windows::core::GUID, cachemode: super::BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCharacteristicsForUuidWithCacheModeAsync(&*(&characteristicuuid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), cachemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIncludedServicesAsync<Impl: IGattDeviceService3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIncludedServicesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIncludedServicesWithCacheModeAsync<Impl: IGattDeviceService3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cachemode: super::BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIncludedServicesWithCacheModeAsync(cachemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIncludedServicesForUuidAsync<Impl: IGattDeviceService3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceuuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIncludedServicesForUuidAsync(&*(&serviceuuid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIncludedServicesForUuidWithCacheModeAsync<Impl: IGattDeviceService3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceuuid: ::windows::core::GUID, cachemode: super::BluetoothCacheMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIncludedServicesForUuidWithCacheModeAsync(&*(&serviceuuid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), cachemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattDeviceService3, BASE_OFFSET>(),
            DeviceAccessInformation: DeviceAccessInformation::<Impl, IMPL_OFFSET>,
            Session: Session::<Impl, IMPL_OFFSET>,
            SharingMode: SharingMode::<Impl, IMPL_OFFSET>,
            RequestAccessAsync: RequestAccessAsync::<Impl, IMPL_OFFSET>,
            OpenAsync: OpenAsync::<Impl, IMPL_OFFSET>,
            GetCharacteristicsAsync: GetCharacteristicsAsync::<Impl, IMPL_OFFSET>,
            GetCharacteristicsWithCacheModeAsync: GetCharacteristicsWithCacheModeAsync::<Impl, IMPL_OFFSET>,
            GetCharacteristicsForUuidAsync: GetCharacteristicsForUuidAsync::<Impl, IMPL_OFFSET>,
            GetCharacteristicsForUuidWithCacheModeAsync: GetCharacteristicsForUuidWithCacheModeAsync::<Impl, IMPL_OFFSET>,
            GetIncludedServicesAsync: GetIncludedServicesAsync::<Impl, IMPL_OFFSET>,
            GetIncludedServicesWithCacheModeAsync: GetIncludedServicesWithCacheModeAsync::<Impl, IMPL_OFFSET>,
            GetIncludedServicesForUuidAsync: GetIncludedServicesForUuidAsync::<Impl, IMPL_OFFSET>,
            GetIncludedServicesForUuidWithCacheModeAsync: GetIncludedServicesForUuidWithCacheModeAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattDeviceService3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGattDeviceServiceStaticsImpl: Sized {
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceService>>;
    fn GetDeviceSelectorFromUuid(&mut self, serviceuuid: &::windows::core::GUID) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromShortId(&mut self, serviceshortid: u16) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ConvertShortIdToUuid(&mut self, shortid: u16) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattDeviceServiceStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattDeviceServiceStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGattDeviceServiceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattDeviceServiceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattDeviceServiceStaticsVtbl {
        unsafe extern "system" fn FromIdAsync<Impl: IGattDeviceServiceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromUuid<Impl: IGattDeviceServiceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceuuid: ::windows::core::GUID, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromUuid(&*(&serviceuuid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromShortId<Impl: IGattDeviceServiceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceshortid: u16, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromShortId(serviceshortid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertShortIdToUuid<Impl: IGattDeviceServiceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shortid: u16, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertShortIdToUuid(shortid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattDeviceServiceStatics, BASE_OFFSET>(),
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorFromUuid: GetDeviceSelectorFromUuid::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorFromShortId: GetDeviceSelectorFromShortId::<Impl, IMPL_OFFSET>,
            ConvertShortIdToUuid: ConvertShortIdToUuid::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattDeviceServiceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGattDeviceServiceStatics2Impl: Sized {
    fn FromIdWithSharingModeAsync(&mut self, deviceid: &::windows::core::HSTRING, sharingmode: GattSharingMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceService>>;
    fn GetDeviceSelectorForBluetoothDeviceId(&mut self, bluetoothdeviceid: &::core::option::Option<super::BluetoothDeviceId>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorForBluetoothDeviceIdWithCacheMode(&mut self, bluetoothdeviceid: &::core::option::Option<super::BluetoothDeviceId>, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorForBluetoothDeviceIdAndUuid(&mut self, bluetoothdeviceid: &::core::option::Option<super::BluetoothDeviceId>, serviceuuid: &::windows::core::GUID) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorForBluetoothDeviceIdAndUuidWithCacheMode(&mut self, bluetoothdeviceid: &::core::option::Option<super::BluetoothDeviceId>, serviceuuid: &::windows::core::GUID, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattDeviceServiceStatics2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattDeviceServiceStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGattDeviceServiceStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattDeviceServiceStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattDeviceServiceStatics2Vtbl {
        unsafe extern "system" fn FromIdWithSharingModeAsync<Impl: IGattDeviceServiceStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sharingmode: GattSharingMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdWithSharingModeAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), sharingmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorForBluetoothDeviceId<Impl: IGattDeviceServiceStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluetoothdeviceid: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorForBluetoothDeviceId(&*(&bluetoothdeviceid as *const <super::BluetoothDeviceId as ::windows::core::Abi>::Abi as *const <super::BluetoothDeviceId as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorForBluetoothDeviceIdWithCacheMode<Impl: IGattDeviceServiceStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluetoothdeviceid: ::windows::core::RawPtr, cachemode: super::BluetoothCacheMode, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorForBluetoothDeviceIdWithCacheMode(&*(&bluetoothdeviceid as *const <super::BluetoothDeviceId as ::windows::core::Abi>::Abi as *const <super::BluetoothDeviceId as ::windows::core::DefaultType>::DefaultType), cachemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorForBluetoothDeviceIdAndUuid<Impl: IGattDeviceServiceStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluetoothdeviceid: ::windows::core::RawPtr, serviceuuid: ::windows::core::GUID, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorForBluetoothDeviceIdAndUuid(&*(&bluetoothdeviceid as *const <super::BluetoothDeviceId as ::windows::core::Abi>::Abi as *const <super::BluetoothDeviceId as ::windows::core::DefaultType>::DefaultType), &*(&serviceuuid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorForBluetoothDeviceIdAndUuidWithCacheMode<Impl: IGattDeviceServiceStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bluetoothdeviceid: ::windows::core::RawPtr, serviceuuid: ::windows::core::GUID, cachemode: super::BluetoothCacheMode, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorForBluetoothDeviceIdAndUuidWithCacheMode(&*(&bluetoothdeviceid as *const <super::BluetoothDeviceId as ::windows::core::Abi>::Abi as *const <super::BluetoothDeviceId as ::windows::core::DefaultType>::DefaultType), &*(&serviceuuid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), cachemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattDeviceServiceStatics2, BASE_OFFSET>(),
            FromIdWithSharingModeAsync: FromIdWithSharingModeAsync::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorForBluetoothDeviceId: GetDeviceSelectorForBluetoothDeviceId::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorForBluetoothDeviceIdWithCacheMode: GetDeviceSelectorForBluetoothDeviceIdWithCacheMode::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorForBluetoothDeviceIdAndUuid: GetDeviceSelectorForBluetoothDeviceIdAndUuid::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorForBluetoothDeviceIdAndUuidWithCacheMode: GetDeviceSelectorForBluetoothDeviceIdAndUuidWithCacheMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattDeviceServiceStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGattDeviceServicesResultImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<GattCommunicationStatus>;
    fn ProtocolError(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>>;
    fn Services(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattDeviceServicesResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattDeviceServicesResult";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGattDeviceServicesResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattDeviceServicesResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattDeviceServicesResultVtbl {
        unsafe extern "system" fn Status<Impl: IGattDeviceServicesResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattCommunicationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProtocolError<Impl: IGattDeviceServicesResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtocolError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Services<Impl: IGattDeviceServicesResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Services() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattDeviceServicesResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            ProtocolError: ProtocolError::<Impl, IMPL_OFFSET>,
            Services: Services::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattDeviceServicesResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IGattLocalCharacteristicImpl: Sized {
    fn Uuid(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn StaticValue(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn CharacteristicProperties(&mut self) -> ::windows::core::Result<GattCharacteristicProperties>;
    fn ReadProtectionLevel(&mut self) -> ::windows::core::Result<GattProtectionLevel>;
    fn WriteProtectionLevel(&mut self) -> ::windows::core::Result<GattProtectionLevel>;
    fn CreateDescriptorAsync(&mut self, descriptoruuid: &::windows::core::GUID, parameters: &::core::option::Option<GattLocalDescriptorParameters>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattLocalDescriptorResult>>;
    fn Descriptors(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattLocalDescriptor>>;
    fn UserDescription(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PresentationFormats(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattPresentationFormat>>;
    fn SubscribedClients(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattSubscribedClient>>;
    fn SubscribedClientsChanged(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattLocalCharacteristic, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSubscribedClientsChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ReadRequested(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattLocalCharacteristic, GattReadRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadRequested(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn WriteRequested(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattLocalCharacteristic, GattWriteRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveWriteRequested(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NotifyValueAsync(&mut self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<GattClientNotificationResult>>>;
    fn NotifyValueForSubscribedClientAsync(&mut self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, subscribedclient: &::core::option::Option<GattSubscribedClient>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattClientNotificationResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattLocalCharacteristic {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattLocalCharacteristic";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IGattLocalCharacteristicVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattLocalCharacteristicImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattLocalCharacteristicVtbl {
        unsafe extern "system" fn Uuid<Impl: IGattLocalCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uuid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StaticValue<Impl: IGattLocalCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StaticValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CharacteristicProperties<Impl: IGattLocalCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattCharacteristicProperties) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CharacteristicProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadProtectionLevel<Impl: IGattLocalCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadProtectionLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteProtectionLevel<Impl: IGattLocalCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteProtectionLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDescriptorAsync<Impl: IGattLocalCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptoruuid: ::windows::core::GUID, parameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDescriptorAsync(&*(&descriptoruuid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&parameters as *const <GattLocalDescriptorParameters as ::windows::core::Abi>::Abi as *const <GattLocalDescriptorParameters as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Descriptors<Impl: IGattLocalCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Descriptors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserDescription<Impl: IGattLocalCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresentationFormats<Impl: IGattLocalCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresentationFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubscribedClients<Impl: IGattLocalCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubscribedClients() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubscribedClientsChanged<Impl: IGattLocalCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubscribedClientsChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<GattLocalCharacteristic, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<GattLocalCharacteristic, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSubscribedClientsChanged<Impl: IGattLocalCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSubscribedClientsChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReadRequested<Impl: IGattLocalCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<GattLocalCharacteristic, GattReadRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<GattLocalCharacteristic, GattReadRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReadRequested<Impl: IGattLocalCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WriteRequested<Impl: IGattLocalCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<GattLocalCharacteristic, GattWriteRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<GattLocalCharacteristic, GattWriteRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveWriteRequested<Impl: IGattLocalCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveWriteRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NotifyValueAsync<Impl: IGattLocalCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyValueAsync(&*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyValueForSubscribedClientAsync<Impl: IGattLocalCharacteristicImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, subscribedclient: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyValueForSubscribedClientAsync(&*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), &*(&subscribedclient as *const <GattSubscribedClient as ::windows::core::Abi>::Abi as *const <GattSubscribedClient as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattLocalCharacteristic, BASE_OFFSET>(),
            Uuid: Uuid::<Impl, IMPL_OFFSET>,
            StaticValue: StaticValue::<Impl, IMPL_OFFSET>,
            CharacteristicProperties: CharacteristicProperties::<Impl, IMPL_OFFSET>,
            ReadProtectionLevel: ReadProtectionLevel::<Impl, IMPL_OFFSET>,
            WriteProtectionLevel: WriteProtectionLevel::<Impl, IMPL_OFFSET>,
            CreateDescriptorAsync: CreateDescriptorAsync::<Impl, IMPL_OFFSET>,
            Descriptors: Descriptors::<Impl, IMPL_OFFSET>,
            UserDescription: UserDescription::<Impl, IMPL_OFFSET>,
            PresentationFormats: PresentationFormats::<Impl, IMPL_OFFSET>,
            SubscribedClients: SubscribedClients::<Impl, IMPL_OFFSET>,
            SubscribedClientsChanged: SubscribedClientsChanged::<Impl, IMPL_OFFSET>,
            RemoveSubscribedClientsChanged: RemoveSubscribedClientsChanged::<Impl, IMPL_OFFSET>,
            ReadRequested: ReadRequested::<Impl, IMPL_OFFSET>,
            RemoveReadRequested: RemoveReadRequested::<Impl, IMPL_OFFSET>,
            WriteRequested: WriteRequested::<Impl, IMPL_OFFSET>,
            RemoveWriteRequested: RemoveWriteRequested::<Impl, IMPL_OFFSET>,
            NotifyValueAsync: NotifyValueAsync::<Impl, IMPL_OFFSET>,
            NotifyValueForSubscribedClientAsync: NotifyValueForSubscribedClientAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattLocalCharacteristic as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IGattLocalCharacteristicParametersImpl: Sized {
    fn SetStaticValue(&mut self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn StaticValue(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetCharacteristicProperties(&mut self, value: GattCharacteristicProperties) -> ::windows::core::Result<()>;
    fn CharacteristicProperties(&mut self) -> ::windows::core::Result<GattCharacteristicProperties>;
    fn SetReadProtectionLevel(&mut self, value: GattProtectionLevel) -> ::windows::core::Result<()>;
    fn ReadProtectionLevel(&mut self) -> ::windows::core::Result<GattProtectionLevel>;
    fn SetWriteProtectionLevel(&mut self, value: GattProtectionLevel) -> ::windows::core::Result<()>;
    fn WriteProtectionLevel(&mut self) -> ::windows::core::Result<GattProtectionLevel>;
    fn SetUserDescription(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UserDescription(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PresentationFormats(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<GattPresentationFormat>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattLocalCharacteristicParameters {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattLocalCharacteristicParameters";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IGattLocalCharacteristicParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattLocalCharacteristicParametersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattLocalCharacteristicParametersVtbl {
        unsafe extern "system" fn SetStaticValue<Impl: IGattLocalCharacteristicParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStaticValue(&*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StaticValue<Impl: IGattLocalCharacteristicParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StaticValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharacteristicProperties<Impl: IGattLocalCharacteristicParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: GattCharacteristicProperties) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCharacteristicProperties(value).into()
        }
        unsafe extern "system" fn CharacteristicProperties<Impl: IGattLocalCharacteristicParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattCharacteristicProperties) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CharacteristicProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReadProtectionLevel<Impl: IGattLocalCharacteristicParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: GattProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReadProtectionLevel(value).into()
        }
        unsafe extern "system" fn ReadProtectionLevel<Impl: IGattLocalCharacteristicParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadProtectionLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWriteProtectionLevel<Impl: IGattLocalCharacteristicParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: GattProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWriteProtectionLevel(value).into()
        }
        unsafe extern "system" fn WriteProtectionLevel<Impl: IGattLocalCharacteristicParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteProtectionLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserDescription<Impl: IGattLocalCharacteristicParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UserDescription<Impl: IGattLocalCharacteristicParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresentationFormats<Impl: IGattLocalCharacteristicParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresentationFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattLocalCharacteristicParameters, BASE_OFFSET>(),
            SetStaticValue: SetStaticValue::<Impl, IMPL_OFFSET>,
            StaticValue: StaticValue::<Impl, IMPL_OFFSET>,
            SetCharacteristicProperties: SetCharacteristicProperties::<Impl, IMPL_OFFSET>,
            CharacteristicProperties: CharacteristicProperties::<Impl, IMPL_OFFSET>,
            SetReadProtectionLevel: SetReadProtectionLevel::<Impl, IMPL_OFFSET>,
            ReadProtectionLevel: ReadProtectionLevel::<Impl, IMPL_OFFSET>,
            SetWriteProtectionLevel: SetWriteProtectionLevel::<Impl, IMPL_OFFSET>,
            WriteProtectionLevel: WriteProtectionLevel::<Impl, IMPL_OFFSET>,
            SetUserDescription: SetUserDescription::<Impl, IMPL_OFFSET>,
            UserDescription: UserDescription::<Impl, IMPL_OFFSET>,
            PresentationFormats: PresentationFormats::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattLocalCharacteristicParameters as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattLocalCharacteristicResultImpl: Sized {
    fn Characteristic(&mut self) -> ::windows::core::Result<GattLocalCharacteristic>;
    fn Error(&mut self) -> ::windows::core::Result<super::BluetoothError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattLocalCharacteristicResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattLocalCharacteristicResult";
}
#[cfg(feature = "implement_exclusive")]
impl IGattLocalCharacteristicResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattLocalCharacteristicResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattLocalCharacteristicResultVtbl {
        unsafe extern "system" fn Characteristic<Impl: IGattLocalCharacteristicResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Characteristic() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: IGattLocalCharacteristicResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattLocalCharacteristicResult, BASE_OFFSET>(),
            Characteristic: Characteristic::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattLocalCharacteristicResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IGattLocalDescriptorImpl: Sized {
    fn Uuid(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn StaticValue(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn ReadProtectionLevel(&mut self) -> ::windows::core::Result<GattProtectionLevel>;
    fn WriteProtectionLevel(&mut self) -> ::windows::core::Result<GattProtectionLevel>;
    fn ReadRequested(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattLocalDescriptor, GattReadRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadRequested(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn WriteRequested(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattLocalDescriptor, GattWriteRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveWriteRequested(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattLocalDescriptor {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattLocalDescriptor";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IGattLocalDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattLocalDescriptorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattLocalDescriptorVtbl {
        unsafe extern "system" fn Uuid<Impl: IGattLocalDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uuid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StaticValue<Impl: IGattLocalDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StaticValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadProtectionLevel<Impl: IGattLocalDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadProtectionLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteProtectionLevel<Impl: IGattLocalDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteProtectionLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadRequested<Impl: IGattLocalDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<GattLocalDescriptor, GattReadRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<GattLocalDescriptor, GattReadRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReadRequested<Impl: IGattLocalDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReadRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WriteRequested<Impl: IGattLocalDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<GattLocalDescriptor, GattWriteRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<GattLocalDescriptor, GattWriteRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveWriteRequested<Impl: IGattLocalDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveWriteRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattLocalDescriptor, BASE_OFFSET>(),
            Uuid: Uuid::<Impl, IMPL_OFFSET>,
            StaticValue: StaticValue::<Impl, IMPL_OFFSET>,
            ReadProtectionLevel: ReadProtectionLevel::<Impl, IMPL_OFFSET>,
            WriteProtectionLevel: WriteProtectionLevel::<Impl, IMPL_OFFSET>,
            ReadRequested: ReadRequested::<Impl, IMPL_OFFSET>,
            RemoveReadRequested: RemoveReadRequested::<Impl, IMPL_OFFSET>,
            WriteRequested: WriteRequested::<Impl, IMPL_OFFSET>,
            RemoveWriteRequested: RemoveWriteRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattLocalDescriptor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IGattLocalDescriptorParametersImpl: Sized {
    fn SetStaticValue(&mut self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn StaticValue(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetReadProtectionLevel(&mut self, value: GattProtectionLevel) -> ::windows::core::Result<()>;
    fn ReadProtectionLevel(&mut self) -> ::windows::core::Result<GattProtectionLevel>;
    fn SetWriteProtectionLevel(&mut self, value: GattProtectionLevel) -> ::windows::core::Result<()>;
    fn WriteProtectionLevel(&mut self) -> ::windows::core::Result<GattProtectionLevel>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattLocalDescriptorParameters {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattLocalDescriptorParameters";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IGattLocalDescriptorParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattLocalDescriptorParametersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattLocalDescriptorParametersVtbl {
        unsafe extern "system" fn SetStaticValue<Impl: IGattLocalDescriptorParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStaticValue(&*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StaticValue<Impl: IGattLocalDescriptorParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StaticValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReadProtectionLevel<Impl: IGattLocalDescriptorParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: GattProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReadProtectionLevel(value).into()
        }
        unsafe extern "system" fn ReadProtectionLevel<Impl: IGattLocalDescriptorParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadProtectionLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWriteProtectionLevel<Impl: IGattLocalDescriptorParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: GattProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWriteProtectionLevel(value).into()
        }
        unsafe extern "system" fn WriteProtectionLevel<Impl: IGattLocalDescriptorParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteProtectionLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattLocalDescriptorParameters, BASE_OFFSET>(),
            SetStaticValue: SetStaticValue::<Impl, IMPL_OFFSET>,
            StaticValue: StaticValue::<Impl, IMPL_OFFSET>,
            SetReadProtectionLevel: SetReadProtectionLevel::<Impl, IMPL_OFFSET>,
            ReadProtectionLevel: ReadProtectionLevel::<Impl, IMPL_OFFSET>,
            SetWriteProtectionLevel: SetWriteProtectionLevel::<Impl, IMPL_OFFSET>,
            WriteProtectionLevel: WriteProtectionLevel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattLocalDescriptorParameters as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattLocalDescriptorResultImpl: Sized {
    fn Descriptor(&mut self) -> ::windows::core::Result<GattLocalDescriptor>;
    fn Error(&mut self) -> ::windows::core::Result<super::BluetoothError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattLocalDescriptorResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattLocalDescriptorResult";
}
#[cfg(feature = "implement_exclusive")]
impl IGattLocalDescriptorResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattLocalDescriptorResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattLocalDescriptorResultVtbl {
        unsafe extern "system" fn Descriptor<Impl: IGattLocalDescriptorResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Descriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: IGattLocalDescriptorResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattLocalDescriptorResult, BASE_OFFSET>(),
            Descriptor: Descriptor::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattLocalDescriptorResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGattLocalServiceImpl: Sized {
    fn Uuid(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CreateCharacteristicAsync(&mut self, characteristicuuid: &::windows::core::GUID, parameters: &::core::option::Option<GattLocalCharacteristicParameters>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattLocalCharacteristicResult>>;
    fn Characteristics(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattLocalCharacteristic>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattLocalService {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattLocalService";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGattLocalServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattLocalServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattLocalServiceVtbl {
        unsafe extern "system" fn Uuid<Impl: IGattLocalServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uuid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCharacteristicAsync<Impl: IGattLocalServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, characteristicuuid: ::windows::core::GUID, parameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCharacteristicAsync(&*(&characteristicuuid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&parameters as *const <GattLocalCharacteristicParameters as ::windows::core::Abi>::Abi as *const <GattLocalCharacteristicParameters as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Characteristics<Impl: IGattLocalServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Characteristics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattLocalService, BASE_OFFSET>(),
            Uuid: Uuid::<Impl, IMPL_OFFSET>,
            CreateCharacteristicAsync: CreateCharacteristicAsync::<Impl, IMPL_OFFSET>,
            Characteristics: Characteristics::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattLocalService as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattPresentationFormatImpl: Sized {
    fn FormatType(&mut self) -> ::windows::core::Result<u8>;
    fn Exponent(&mut self) -> ::windows::core::Result<i32>;
    fn Unit(&mut self) -> ::windows::core::Result<u16>;
    fn Namespace(&mut self) -> ::windows::core::Result<u8>;
    fn Description(&mut self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattPresentationFormat {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattPresentationFormat";
}
#[cfg(feature = "implement_exclusive")]
impl IGattPresentationFormatVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattPresentationFormatImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattPresentationFormatVtbl {
        unsafe extern "system" fn FormatType<Impl: IGattPresentationFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Exponent<Impl: IGattPresentationFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Exponent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unit<Impl: IGattPresentationFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Namespace<Impl: IGattPresentationFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Namespace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IGattPresentationFormatImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattPresentationFormat, BASE_OFFSET>(),
            FormatType: FormatType::<Impl, IMPL_OFFSET>,
            Exponent: Exponent::<Impl, IMPL_OFFSET>,
            Unit: Unit::<Impl, IMPL_OFFSET>,
            Namespace: Namespace::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattPresentationFormat as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattPresentationFormatStaticsImpl: Sized {
    fn BluetoothSigAssignedNumbers(&mut self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattPresentationFormatStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattPresentationFormatStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGattPresentationFormatStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattPresentationFormatStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattPresentationFormatStaticsVtbl {
        unsafe extern "system" fn BluetoothSigAssignedNumbers<Impl: IGattPresentationFormatStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BluetoothSigAssignedNumbers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattPresentationFormatStatics, BASE_OFFSET>(),
            BluetoothSigAssignedNumbers: BluetoothSigAssignedNumbers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattPresentationFormatStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattPresentationFormatStatics2Impl: Sized + IGattPresentationFormatStaticsImpl {
    fn FromParts(&mut self, formattype: u8, exponent: i32, unit: u16, namespaceid: u8, description: u16) -> ::windows::core::Result<GattPresentationFormat>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattPresentationFormatStatics2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattPresentationFormatStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IGattPresentationFormatStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattPresentationFormatStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattPresentationFormatStatics2Vtbl {
        unsafe extern "system" fn FromParts<Impl: IGattPresentationFormatStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formattype: u8, exponent: i32, unit: u16, namespaceid: u8, description: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromParts(formattype, exponent, unit, namespaceid, description) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattPresentationFormatStatics2, BASE_OFFSET>(),
            FromParts: FromParts::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattPresentationFormatStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattPresentationFormatTypesStaticsImpl: Sized {
    fn Boolean(&mut self) -> ::windows::core::Result<u8>;
    fn Bit2(&mut self) -> ::windows::core::Result<u8>;
    fn Nibble(&mut self) -> ::windows::core::Result<u8>;
    fn UInt8(&mut self) -> ::windows::core::Result<u8>;
    fn UInt12(&mut self) -> ::windows::core::Result<u8>;
    fn UInt16(&mut self) -> ::windows::core::Result<u8>;
    fn UInt24(&mut self) -> ::windows::core::Result<u8>;
    fn UInt32(&mut self) -> ::windows::core::Result<u8>;
    fn UInt48(&mut self) -> ::windows::core::Result<u8>;
    fn UInt64(&mut self) -> ::windows::core::Result<u8>;
    fn UInt128(&mut self) -> ::windows::core::Result<u8>;
    fn SInt8(&mut self) -> ::windows::core::Result<u8>;
    fn SInt12(&mut self) -> ::windows::core::Result<u8>;
    fn SInt16(&mut self) -> ::windows::core::Result<u8>;
    fn SInt24(&mut self) -> ::windows::core::Result<u8>;
    fn SInt32(&mut self) -> ::windows::core::Result<u8>;
    fn SInt48(&mut self) -> ::windows::core::Result<u8>;
    fn SInt64(&mut self) -> ::windows::core::Result<u8>;
    fn SInt128(&mut self) -> ::windows::core::Result<u8>;
    fn Float32(&mut self) -> ::windows::core::Result<u8>;
    fn Float64(&mut self) -> ::windows::core::Result<u8>;
    fn SFloat(&mut self) -> ::windows::core::Result<u8>;
    fn Float(&mut self) -> ::windows::core::Result<u8>;
    fn DUInt16(&mut self) -> ::windows::core::Result<u8>;
    fn Utf8(&mut self) -> ::windows::core::Result<u8>;
    fn Utf16(&mut self) -> ::windows::core::Result<u8>;
    fn Struct(&mut self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattPresentationFormatTypesStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattPresentationFormatTypesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGattPresentationFormatTypesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattPresentationFormatTypesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattPresentationFormatTypesStaticsVtbl {
        unsafe extern "system" fn Boolean<Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Boolean() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bit2<Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bit2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Nibble<Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Nibble() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UInt8<Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UInt8() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UInt12<Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UInt12() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UInt16<Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UInt16() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UInt24<Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UInt24() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UInt32<Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UInt32() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UInt48<Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UInt48() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UInt64<Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UInt64() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UInt128<Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UInt128() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SInt8<Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SInt8() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SInt12<Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SInt12() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SInt16<Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SInt16() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SInt24<Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SInt24() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SInt32<Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SInt32() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SInt48<Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SInt48() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SInt64<Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SInt64() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SInt128<Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SInt128() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Float32<Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Float32() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Float64<Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Float64() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SFloat<Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SFloat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Float<Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Float() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DUInt16<Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DUInt16() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Utf8<Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Utf8() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Utf16<Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Utf16() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Struct<Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Struct() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattPresentationFormatTypesStatics, BASE_OFFSET>(),
            Boolean: Boolean::<Impl, IMPL_OFFSET>,
            Bit2: Bit2::<Impl, IMPL_OFFSET>,
            Nibble: Nibble::<Impl, IMPL_OFFSET>,
            UInt8: UInt8::<Impl, IMPL_OFFSET>,
            UInt12: UInt12::<Impl, IMPL_OFFSET>,
            UInt16: UInt16::<Impl, IMPL_OFFSET>,
            UInt24: UInt24::<Impl, IMPL_OFFSET>,
            UInt32: UInt32::<Impl, IMPL_OFFSET>,
            UInt48: UInt48::<Impl, IMPL_OFFSET>,
            UInt64: UInt64::<Impl, IMPL_OFFSET>,
            UInt128: UInt128::<Impl, IMPL_OFFSET>,
            SInt8: SInt8::<Impl, IMPL_OFFSET>,
            SInt12: SInt12::<Impl, IMPL_OFFSET>,
            SInt16: SInt16::<Impl, IMPL_OFFSET>,
            SInt24: SInt24::<Impl, IMPL_OFFSET>,
            SInt32: SInt32::<Impl, IMPL_OFFSET>,
            SInt48: SInt48::<Impl, IMPL_OFFSET>,
            SInt64: SInt64::<Impl, IMPL_OFFSET>,
            SInt128: SInt128::<Impl, IMPL_OFFSET>,
            Float32: Float32::<Impl, IMPL_OFFSET>,
            Float64: Float64::<Impl, IMPL_OFFSET>,
            SFloat: SFloat::<Impl, IMPL_OFFSET>,
            Float: Float::<Impl, IMPL_OFFSET>,
            DUInt16: DUInt16::<Impl, IMPL_OFFSET>,
            Utf8: Utf8::<Impl, IMPL_OFFSET>,
            Utf16: Utf16::<Impl, IMPL_OFFSET>,
            Struct: Struct::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattPresentationFormatTypesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattProtocolErrorStaticsImpl: Sized {
    fn InvalidHandle(&mut self) -> ::windows::core::Result<u8>;
    fn ReadNotPermitted(&mut self) -> ::windows::core::Result<u8>;
    fn WriteNotPermitted(&mut self) -> ::windows::core::Result<u8>;
    fn InvalidPdu(&mut self) -> ::windows::core::Result<u8>;
    fn InsufficientAuthentication(&mut self) -> ::windows::core::Result<u8>;
    fn RequestNotSupported(&mut self) -> ::windows::core::Result<u8>;
    fn InvalidOffset(&mut self) -> ::windows::core::Result<u8>;
    fn InsufficientAuthorization(&mut self) -> ::windows::core::Result<u8>;
    fn PrepareQueueFull(&mut self) -> ::windows::core::Result<u8>;
    fn AttributeNotFound(&mut self) -> ::windows::core::Result<u8>;
    fn AttributeNotLong(&mut self) -> ::windows::core::Result<u8>;
    fn InsufficientEncryptionKeySize(&mut self) -> ::windows::core::Result<u8>;
    fn InvalidAttributeValueLength(&mut self) -> ::windows::core::Result<u8>;
    fn UnlikelyError(&mut self) -> ::windows::core::Result<u8>;
    fn InsufficientEncryption(&mut self) -> ::windows::core::Result<u8>;
    fn UnsupportedGroupType(&mut self) -> ::windows::core::Result<u8>;
    fn InsufficientResources(&mut self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattProtocolErrorStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattProtocolErrorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGattProtocolErrorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattProtocolErrorStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattProtocolErrorStaticsVtbl {
        unsafe extern "system" fn InvalidHandle<Impl: IGattProtocolErrorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvalidHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadNotPermitted<Impl: IGattProtocolErrorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadNotPermitted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteNotPermitted<Impl: IGattProtocolErrorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteNotPermitted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvalidPdu<Impl: IGattProtocolErrorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvalidPdu() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsufficientAuthentication<Impl: IGattProtocolErrorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsufficientAuthentication() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestNotSupported<Impl: IGattProtocolErrorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestNotSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvalidOffset<Impl: IGattProtocolErrorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvalidOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsufficientAuthorization<Impl: IGattProtocolErrorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsufficientAuthorization() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrepareQueueFull<Impl: IGattProtocolErrorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrepareQueueFull() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttributeNotFound<Impl: IGattProtocolErrorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttributeNotFound() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttributeNotLong<Impl: IGattProtocolErrorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttributeNotLong() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsufficientEncryptionKeySize<Impl: IGattProtocolErrorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsufficientEncryptionKeySize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvalidAttributeValueLength<Impl: IGattProtocolErrorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvalidAttributeValueLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnlikelyError<Impl: IGattProtocolErrorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnlikelyError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsufficientEncryption<Impl: IGattProtocolErrorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsufficientEncryption() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnsupportedGroupType<Impl: IGattProtocolErrorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnsupportedGroupType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsufficientResources<Impl: IGattProtocolErrorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InsufficientResources() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattProtocolErrorStatics, BASE_OFFSET>(),
            InvalidHandle: InvalidHandle::<Impl, IMPL_OFFSET>,
            ReadNotPermitted: ReadNotPermitted::<Impl, IMPL_OFFSET>,
            WriteNotPermitted: WriteNotPermitted::<Impl, IMPL_OFFSET>,
            InvalidPdu: InvalidPdu::<Impl, IMPL_OFFSET>,
            InsufficientAuthentication: InsufficientAuthentication::<Impl, IMPL_OFFSET>,
            RequestNotSupported: RequestNotSupported::<Impl, IMPL_OFFSET>,
            InvalidOffset: InvalidOffset::<Impl, IMPL_OFFSET>,
            InsufficientAuthorization: InsufficientAuthorization::<Impl, IMPL_OFFSET>,
            PrepareQueueFull: PrepareQueueFull::<Impl, IMPL_OFFSET>,
            AttributeNotFound: AttributeNotFound::<Impl, IMPL_OFFSET>,
            AttributeNotLong: AttributeNotLong::<Impl, IMPL_OFFSET>,
            InsufficientEncryptionKeySize: InsufficientEncryptionKeySize::<Impl, IMPL_OFFSET>,
            InvalidAttributeValueLength: InvalidAttributeValueLength::<Impl, IMPL_OFFSET>,
            UnlikelyError: UnlikelyError::<Impl, IMPL_OFFSET>,
            InsufficientEncryption: InsufficientEncryption::<Impl, IMPL_OFFSET>,
            UnsupportedGroupType: UnsupportedGroupType::<Impl, IMPL_OFFSET>,
            InsufficientResources: InsufficientResources::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattProtocolErrorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattReadClientCharacteristicConfigurationDescriptorResultImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<GattCommunicationStatus>;
    fn ClientCharacteristicConfigurationDescriptor(&mut self) -> ::windows::core::Result<GattClientCharacteristicConfigurationDescriptorValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattReadClientCharacteristicConfigurationDescriptorResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattReadClientCharacteristicConfigurationDescriptorResult";
}
#[cfg(feature = "implement_exclusive")]
impl IGattReadClientCharacteristicConfigurationDescriptorResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattReadClientCharacteristicConfigurationDescriptorResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattReadClientCharacteristicConfigurationDescriptorResultVtbl {
        unsafe extern "system" fn Status<Impl: IGattReadClientCharacteristicConfigurationDescriptorResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattCommunicationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClientCharacteristicConfigurationDescriptor<Impl: IGattReadClientCharacteristicConfigurationDescriptorResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattClientCharacteristicConfigurationDescriptorValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientCharacteristicConfigurationDescriptor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattReadClientCharacteristicConfigurationDescriptorResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            ClientCharacteristicConfigurationDescriptor: ClientCharacteristicConfigurationDescriptor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattReadClientCharacteristicConfigurationDescriptorResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGattReadClientCharacteristicConfigurationDescriptorResult2Impl: Sized {
    fn ProtocolError(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattReadClientCharacteristicConfigurationDescriptorResult2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattReadClientCharacteristicConfigurationDescriptorResult2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGattReadClientCharacteristicConfigurationDescriptorResult2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattReadClientCharacteristicConfigurationDescriptorResult2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattReadClientCharacteristicConfigurationDescriptorResult2Vtbl {
        unsafe extern "system" fn ProtocolError<Impl: IGattReadClientCharacteristicConfigurationDescriptorResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtocolError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattReadClientCharacteristicConfigurationDescriptorResult2, BASE_OFFSET>(),
            ProtocolError: ProtocolError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattReadClientCharacteristicConfigurationDescriptorResult2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IGattReadRequestImpl: Sized {
    fn Offset(&mut self) -> ::windows::core::Result<u32>;
    fn Length(&mut self) -> ::windows::core::Result<u32>;
    fn State(&mut self) -> ::windows::core::Result<GattRequestState>;
    fn StateChanged(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattReadRequest, GattRequestStateChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RespondWithValue(&mut self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn RespondWithProtocolError(&mut self, protocolerror: u8) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattReadRequest {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattReadRequest";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IGattReadRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattReadRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattReadRequestVtbl {
        unsafe extern "system" fn Offset<Impl: IGattReadRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Offset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Impl: IGattReadRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Length() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IGattReadRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattRequestState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StateChanged<Impl: IGattReadRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StateChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<GattReadRequest, GattRequestStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<GattReadRequest, GattRequestStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStateChanged<Impl: IGattReadRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStateChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RespondWithValue<Impl: IGattReadRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RespondWithValue(&*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RespondWithProtocolError<Impl: IGattReadRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protocolerror: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RespondWithProtocolError(protocolerror).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattReadRequest, BASE_OFFSET>(),
            Offset: Offset::<Impl, IMPL_OFFSET>,
            Length: Length::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            StateChanged: StateChanged::<Impl, IMPL_OFFSET>,
            RemoveStateChanged: RemoveStateChanged::<Impl, IMPL_OFFSET>,
            RespondWithValue: RespondWithValue::<Impl, IMPL_OFFSET>,
            RespondWithProtocolError: RespondWithProtocolError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattReadRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGattReadRequestedEventArgsImpl: Sized {
    fn Session(&mut self) -> ::windows::core::Result<GattSession>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
    fn GetRequestAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadRequest>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattReadRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattReadRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGattReadRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattReadRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattReadRequestedEventArgsVtbl {
        unsafe extern "system" fn Session<Impl: IGattReadRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IGattReadRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetRequestAsync<Impl: IGattReadRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRequestAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattReadRequestedEventArgs, BASE_OFFSET>(),
            Session: Session::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
            GetRequestAsync: GetRequestAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattReadRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IGattReadResultImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<GattCommunicationStatus>;
    fn Value(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattReadResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattReadResult";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IGattReadResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattReadResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattReadResultVtbl {
        unsafe extern "system" fn Status<Impl: IGattReadResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattCommunicationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Value<Impl: IGattReadResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattReadResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattReadResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGattReadResult2Impl: Sized {
    fn ProtocolError(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattReadResult2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattReadResult2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGattReadResult2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattReadResult2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattReadResult2Vtbl {
        unsafe extern "system" fn ProtocolError<Impl: IGattReadResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtocolError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGattReadResult2, BASE_OFFSET>(), ProtocolError: ProtocolError::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattReadResult2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IGattReliableWriteTransactionImpl: Sized {
    fn WriteValue(&mut self, characteristic: &::core::option::Option<GattCharacteristic>, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn CommitAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattReliableWriteTransaction {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattReliableWriteTransaction";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IGattReliableWriteTransactionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattReliableWriteTransactionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattReliableWriteTransactionVtbl {
        unsafe extern "system" fn WriteValue<Impl: IGattReliableWriteTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, characteristic: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteValue(&*(&characteristic as *const <GattCharacteristic as ::windows::core::Abi>::Abi as *const <GattCharacteristic as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CommitAsync<Impl: IGattReliableWriteTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattReliableWriteTransaction, BASE_OFFSET>(),
            WriteValue: WriteValue::<Impl, IMPL_OFFSET>,
            CommitAsync: CommitAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattReliableWriteTransaction as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGattReliableWriteTransaction2Impl: Sized {
    fn CommitWithResultAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattReliableWriteTransaction2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattReliableWriteTransaction2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGattReliableWriteTransaction2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattReliableWriteTransaction2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattReliableWriteTransaction2Vtbl {
        unsafe extern "system" fn CommitWithResultAsync<Impl: IGattReliableWriteTransaction2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitWithResultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattReliableWriteTransaction2, BASE_OFFSET>(),
            CommitWithResultAsync: CommitWithResultAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattReliableWriteTransaction2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattRequestStateChangedEventArgsImpl: Sized {
    fn State(&mut self) -> ::windows::core::Result<GattRequestState>;
    fn Error(&mut self) -> ::windows::core::Result<super::BluetoothError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattRequestStateChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattRequestStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IGattRequestStateChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattRequestStateChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattRequestStateChangedEventArgsVtbl {
        unsafe extern "system" fn State<Impl: IGattRequestStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattRequestState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Error<Impl: IGattRequestStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattRequestStateChangedEventArgs, BASE_OFFSET>(),
            State: State::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattRequestStateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGattServiceProviderImpl: Sized {
    fn Service(&mut self) -> ::windows::core::Result<GattLocalService>;
    fn AdvertisementStatus(&mut self) -> ::windows::core::Result<GattServiceProviderAdvertisementStatus>;
    fn AdvertisementStatusChanged(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattServiceProvider, GattServiceProviderAdvertisementStatusChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdvertisementStatusChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StartAdvertising(&mut self) -> ::windows::core::Result<()>;
    fn StartAdvertisingWithParameters(&mut self, parameters: &::core::option::Option<GattServiceProviderAdvertisingParameters>) -> ::windows::core::Result<()>;
    fn StopAdvertising(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattServiceProvider {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattServiceProvider";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGattServiceProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattServiceProviderVtbl {
        unsafe extern "system" fn Service<Impl: IGattServiceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Service() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdvertisementStatus<Impl: IGattServiceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattServiceProviderAdvertisementStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdvertisementStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdvertisementStatusChanged<Impl: IGattServiceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdvertisementStatusChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<GattServiceProvider, GattServiceProviderAdvertisementStatusChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<GattServiceProvider, GattServiceProviderAdvertisementStatusChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAdvertisementStatusChanged<Impl: IGattServiceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAdvertisementStatusChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartAdvertising<Impl: IGattServiceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartAdvertising().into()
        }
        unsafe extern "system" fn StartAdvertisingWithParameters<Impl: IGattServiceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameters: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartAdvertisingWithParameters(&*(&parameters as *const <GattServiceProviderAdvertisingParameters as ::windows::core::Abi>::Abi as *const <GattServiceProviderAdvertisingParameters as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StopAdvertising<Impl: IGattServiceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopAdvertising().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattServiceProvider, BASE_OFFSET>(),
            Service: Service::<Impl, IMPL_OFFSET>,
            AdvertisementStatus: AdvertisementStatus::<Impl, IMPL_OFFSET>,
            AdvertisementStatusChanged: AdvertisementStatusChanged::<Impl, IMPL_OFFSET>,
            RemoveAdvertisementStatusChanged: RemoveAdvertisementStatusChanged::<Impl, IMPL_OFFSET>,
            StartAdvertising: StartAdvertising::<Impl, IMPL_OFFSET>,
            StartAdvertisingWithParameters: StartAdvertisingWithParameters::<Impl, IMPL_OFFSET>,
            StopAdvertising: StopAdvertising::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattServiceProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceProviderAdvertisementStatusChangedEventArgsImpl: Sized {
    fn Error(&mut self) -> ::windows::core::Result<super::BluetoothError>;
    fn Status(&mut self) -> ::windows::core::Result<GattServiceProviderAdvertisementStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattServiceProviderAdvertisementStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattServiceProviderAdvertisementStatusChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IGattServiceProviderAdvertisementStatusChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceProviderAdvertisementStatusChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattServiceProviderAdvertisementStatusChangedEventArgsVtbl {
        unsafe extern "system" fn Error<Impl: IGattServiceProviderAdvertisementStatusChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IGattServiceProviderAdvertisementStatusChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattServiceProviderAdvertisementStatus) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattServiceProviderAdvertisementStatusChangedEventArgs, BASE_OFFSET>(),
            Error: Error::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattServiceProviderAdvertisementStatusChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceProviderAdvertisingParametersImpl: Sized {
    fn SetIsConnectable(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsConnectable(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsDiscoverable(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsDiscoverable(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattServiceProviderAdvertisingParameters {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattServiceProviderAdvertisingParameters";
}
#[cfg(feature = "implement_exclusive")]
impl IGattServiceProviderAdvertisingParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceProviderAdvertisingParametersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattServiceProviderAdvertisingParametersVtbl {
        unsafe extern "system" fn SetIsConnectable<Impl: IGattServiceProviderAdvertisingParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsConnectable(value).into()
        }
        unsafe extern "system" fn IsConnectable<Impl: IGattServiceProviderAdvertisingParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConnectable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsDiscoverable<Impl: IGattServiceProviderAdvertisingParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsDiscoverable(value).into()
        }
        unsafe extern "system" fn IsDiscoverable<Impl: IGattServiceProviderAdvertisingParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDiscoverable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattServiceProviderAdvertisingParameters, BASE_OFFSET>(),
            SetIsConnectable: SetIsConnectable::<Impl, IMPL_OFFSET>,
            IsConnectable: IsConnectable::<Impl, IMPL_OFFSET>,
            SetIsDiscoverable: SetIsDiscoverable::<Impl, IMPL_OFFSET>,
            IsDiscoverable: IsDiscoverable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattServiceProviderAdvertisingParameters as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IGattServiceProviderAdvertisingParameters2Impl: Sized {
    fn SetServiceData(&mut self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn ServiceData(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattServiceProviderAdvertisingParameters2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattServiceProviderAdvertisingParameters2";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IGattServiceProviderAdvertisingParameters2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceProviderAdvertisingParameters2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattServiceProviderAdvertisingParameters2Vtbl {
        unsafe extern "system" fn SetServiceData<Impl: IGattServiceProviderAdvertisingParameters2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServiceData(&*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ServiceData<Impl: IGattServiceProviderAdvertisingParameters2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattServiceProviderAdvertisingParameters2, BASE_OFFSET>(),
            SetServiceData: SetServiceData::<Impl, IMPL_OFFSET>,
            ServiceData: ServiceData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattServiceProviderAdvertisingParameters2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceProviderResultImpl: Sized {
    fn Error(&mut self) -> ::windows::core::Result<super::BluetoothError>;
    fn ServiceProvider(&mut self) -> ::windows::core::Result<GattServiceProvider>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattServiceProviderResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattServiceProviderResult";
}
#[cfg(feature = "implement_exclusive")]
impl IGattServiceProviderResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceProviderResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattServiceProviderResultVtbl {
        unsafe extern "system" fn Error<Impl: IGattServiceProviderResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceProvider<Impl: IGattServiceProviderResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattServiceProviderResult, BASE_OFFSET>(),
            Error: Error::<Impl, IMPL_OFFSET>,
            ServiceProvider: ServiceProvider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattServiceProviderResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGattServiceProviderStaticsImpl: Sized {
    fn CreateAsync(&mut self, serviceuuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattServiceProviderResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattServiceProviderStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattServiceProviderStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGattServiceProviderStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceProviderStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattServiceProviderStaticsVtbl {
        unsafe extern "system" fn CreateAsync<Impl: IGattServiceProviderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, serviceuuid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAsync(&*(&serviceuuid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattServiceProviderStatics, BASE_OFFSET>(),
            CreateAsync: CreateAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattServiceProviderStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceUuidsStaticsImpl: Sized {
    fn Battery(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn BloodPressure(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CyclingSpeedAndCadence(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GenericAccess(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GenericAttribute(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Glucose(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn HealthThermometer(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn HeartRate(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RunningSpeedAndCadence(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattServiceUuidsStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattServiceUuidsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGattServiceUuidsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceUuidsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattServiceUuidsStaticsVtbl {
        unsafe extern "system" fn Battery<Impl: IGattServiceUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Battery() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BloodPressure<Impl: IGattServiceUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BloodPressure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CyclingSpeedAndCadence<Impl: IGattServiceUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CyclingSpeedAndCadence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenericAccess<Impl: IGattServiceUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenericAccess() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenericAttribute<Impl: IGattServiceUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenericAttribute() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Glucose<Impl: IGattServiceUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Glucose() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HealthThermometer<Impl: IGattServiceUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HealthThermometer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeartRate<Impl: IGattServiceUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeartRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunningSpeedAndCadence<Impl: IGattServiceUuidsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunningSpeedAndCadence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattServiceUuidsStatics, BASE_OFFSET>(),
            Battery: Battery::<Impl, IMPL_OFFSET>,
            BloodPressure: BloodPressure::<Impl, IMPL_OFFSET>,
            CyclingSpeedAndCadence: CyclingSpeedAndCadence::<Impl, IMPL_OFFSET>,
            GenericAccess: GenericAccess::<Impl, IMPL_OFFSET>,
            GenericAttribute: GenericAttribute::<Impl, IMPL_OFFSET>,
            Glucose: Glucose::<Impl, IMPL_OFFSET>,
            HealthThermometer: HealthThermometer::<Impl, IMPL_OFFSET>,
            HeartRate: HeartRate::<Impl, IMPL_OFFSET>,
            RunningSpeedAndCadence: RunningSpeedAndCadence::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattServiceUuidsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceUuidsStatics2Impl: Sized {
    fn AlertNotification(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CurrentTime(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CyclingPower(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn DeviceInformation(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn HumanInterfaceDevice(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ImmediateAlert(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn LinkLoss(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn LocationAndNavigation(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn NextDstChange(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn PhoneAlertStatus(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ReferenceTimeUpdate(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ScanParameters(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TxPower(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattServiceUuidsStatics2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattServiceUuidsStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IGattServiceUuidsStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceUuidsStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattServiceUuidsStatics2Vtbl {
        unsafe extern "system" fn AlertNotification<Impl: IGattServiceUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlertNotification() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentTime<Impl: IGattServiceUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CyclingPower<Impl: IGattServiceUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CyclingPower() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceInformation<Impl: IGattServiceUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HumanInterfaceDevice<Impl: IGattServiceUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HumanInterfaceDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImmediateAlert<Impl: IGattServiceUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImmediateAlert() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LinkLoss<Impl: IGattServiceUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LinkLoss() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocationAndNavigation<Impl: IGattServiceUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocationAndNavigation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextDstChange<Impl: IGattServiceUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NextDstChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhoneAlertStatus<Impl: IGattServiceUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneAlertStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferenceTimeUpdate<Impl: IGattServiceUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReferenceTimeUpdate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScanParameters<Impl: IGattServiceUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScanParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxPower<Impl: IGattServiceUuidsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TxPower() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattServiceUuidsStatics2, BASE_OFFSET>(),
            AlertNotification: AlertNotification::<Impl, IMPL_OFFSET>,
            CurrentTime: CurrentTime::<Impl, IMPL_OFFSET>,
            CyclingPower: CyclingPower::<Impl, IMPL_OFFSET>,
            DeviceInformation: DeviceInformation::<Impl, IMPL_OFFSET>,
            HumanInterfaceDevice: HumanInterfaceDevice::<Impl, IMPL_OFFSET>,
            ImmediateAlert: ImmediateAlert::<Impl, IMPL_OFFSET>,
            LinkLoss: LinkLoss::<Impl, IMPL_OFFSET>,
            LocationAndNavigation: LocationAndNavigation::<Impl, IMPL_OFFSET>,
            NextDstChange: NextDstChange::<Impl, IMPL_OFFSET>,
            PhoneAlertStatus: PhoneAlertStatus::<Impl, IMPL_OFFSET>,
            ReferenceTimeUpdate: ReferenceTimeUpdate::<Impl, IMPL_OFFSET>,
            ScanParameters: ScanParameters::<Impl, IMPL_OFFSET>,
            TxPower: TxPower::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattServiceUuidsStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGattSessionImpl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<super::BluetoothDeviceId>;
    fn CanMaintainConnection(&mut self) -> ::windows::core::Result<bool>;
    fn SetMaintainConnection(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn MaintainConnection(&mut self) -> ::windows::core::Result<bool>;
    fn MaxPduSize(&mut self) -> ::windows::core::Result<u16>;
    fn SessionStatus(&mut self) -> ::windows::core::Result<GattSessionStatus>;
    fn MaxPduSizeChanged(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMaxPduSizeChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SessionStatusChanged(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattSession, GattSessionStatusChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSessionStatusChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattSession {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattSession";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGattSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattSessionVtbl {
        unsafe extern "system" fn DeviceId<Impl: IGattSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CanMaintainConnection<Impl: IGattSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanMaintainConnection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaintainConnection<Impl: IGattSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaintainConnection(value).into()
        }
        unsafe extern "system" fn MaintainConnection<Impl: IGattSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaintainConnection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxPduSize<Impl: IGattSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxPduSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionStatus<Impl: IGattSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattSessionStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxPduSizeChanged<Impl: IGattSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxPduSizeChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<GattSession, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<GattSession, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMaxPduSizeChanged<Impl: IGattSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMaxPduSizeChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SessionStatusChanged<Impl: IGattSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionStatusChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<GattSession, GattSessionStatusChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<GattSession, GattSessionStatusChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSessionStatusChanged<Impl: IGattSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSessionStatusChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattSession, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            CanMaintainConnection: CanMaintainConnection::<Impl, IMPL_OFFSET>,
            SetMaintainConnection: SetMaintainConnection::<Impl, IMPL_OFFSET>,
            MaintainConnection: MaintainConnection::<Impl, IMPL_OFFSET>,
            MaxPduSize: MaxPduSize::<Impl, IMPL_OFFSET>,
            SessionStatus: SessionStatus::<Impl, IMPL_OFFSET>,
            MaxPduSizeChanged: MaxPduSizeChanged::<Impl, IMPL_OFFSET>,
            RemoveMaxPduSizeChanged: RemoveMaxPduSizeChanged::<Impl, IMPL_OFFSET>,
            SessionStatusChanged: SessionStatusChanged::<Impl, IMPL_OFFSET>,
            RemoveSessionStatusChanged: RemoveSessionStatusChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattSession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGattSessionStaticsImpl: Sized {
    fn FromDeviceIdAsync(&mut self, deviceid: &::core::option::Option<super::BluetoothDeviceId>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattSession>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattSessionStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattSessionStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGattSessionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattSessionStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattSessionStaticsVtbl {
        unsafe extern "system" fn FromDeviceIdAsync<Impl: IGattSessionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromDeviceIdAsync(&*(&deviceid as *const <super::BluetoothDeviceId as ::windows::core::Abi>::Abi as *const <super::BluetoothDeviceId as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattSessionStatics, BASE_OFFSET>(),
            FromDeviceIdAsync: FromDeviceIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattSessionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattSessionStatusChangedEventArgsImpl: Sized {
    fn Error(&mut self) -> ::windows::core::Result<super::BluetoothError>;
    fn Status(&mut self) -> ::windows::core::Result<GattSessionStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattSessionStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattSessionStatusChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IGattSessionStatusChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattSessionStatusChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattSessionStatusChangedEventArgsVtbl {
        unsafe extern "system" fn Error<Impl: IGattSessionStatusChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IGattSessionStatusChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattSessionStatus) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattSessionStatusChangedEventArgs, BASE_OFFSET>(),
            Error: Error::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattSessionStatusChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGattSubscribedClientImpl: Sized {
    fn Session(&mut self) -> ::windows::core::Result<GattSession>;
    fn MaxNotificationSize(&mut self) -> ::windows::core::Result<u16>;
    fn MaxNotificationSizeChanged(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattSubscribedClient, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMaxNotificationSizeChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattSubscribedClient {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattSubscribedClient";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGattSubscribedClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattSubscribedClientImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattSubscribedClientVtbl {
        unsafe extern "system" fn Session<Impl: IGattSubscribedClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxNotificationSize<Impl: IGattSubscribedClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxNotificationSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxNotificationSizeChanged<Impl: IGattSubscribedClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxNotificationSizeChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<GattSubscribedClient, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<GattSubscribedClient, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMaxNotificationSizeChanged<Impl: IGattSubscribedClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMaxNotificationSizeChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattSubscribedClient, BASE_OFFSET>(),
            Session: Session::<Impl, IMPL_OFFSET>,
            MaxNotificationSize: MaxNotificationSize::<Impl, IMPL_OFFSET>,
            MaxNotificationSizeChanged: MaxNotificationSizeChanged::<Impl, IMPL_OFFSET>,
            RemoveMaxNotificationSizeChanged: RemoveMaxNotificationSizeChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattSubscribedClient as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IGattValueChangedEventArgsImpl: Sized {
    fn CharacteristicValue(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattValueChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattValueChangedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IGattValueChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattValueChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattValueChangedEventArgsVtbl {
        unsafe extern "system" fn CharacteristicValue<Impl: IGattValueChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CharacteristicValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: IGattValueChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattValueChangedEventArgs, BASE_OFFSET>(),
            CharacteristicValue: CharacteristicValue::<Impl, IMPL_OFFSET>,
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattValueChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IGattWriteRequestImpl: Sized {
    fn Value(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn Offset(&mut self) -> ::windows::core::Result<u32>;
    fn Option(&mut self) -> ::windows::core::Result<GattWriteOption>;
    fn State(&mut self) -> ::windows::core::Result<GattRequestState>;
    fn StateChanged(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattWriteRequest, GattRequestStateChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Respond(&mut self) -> ::windows::core::Result<()>;
    fn RespondWithProtocolError(&mut self, protocolerror: u8) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattWriteRequest {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattWriteRequest";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IGattWriteRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattWriteRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattWriteRequestVtbl {
        unsafe extern "system" fn Value<Impl: IGattWriteRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Offset<Impl: IGattWriteRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Offset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Option<Impl: IGattWriteRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattWriteOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Option() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: IGattWriteRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattRequestState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StateChanged<Impl: IGattWriteRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StateChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<GattWriteRequest, GattRequestStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<GattWriteRequest, GattRequestStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStateChanged<Impl: IGattWriteRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStateChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Respond<Impl: IGattWriteRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Respond().into()
        }
        unsafe extern "system" fn RespondWithProtocolError<Impl: IGattWriteRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protocolerror: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RespondWithProtocolError(protocolerror).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattWriteRequest, BASE_OFFSET>(),
            Value: Value::<Impl, IMPL_OFFSET>,
            Offset: Offset::<Impl, IMPL_OFFSET>,
            Option: Option::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            StateChanged: StateChanged::<Impl, IMPL_OFFSET>,
            RemoveStateChanged: RemoveStateChanged::<Impl, IMPL_OFFSET>,
            Respond: Respond::<Impl, IMPL_OFFSET>,
            RespondWithProtocolError: RespondWithProtocolError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattWriteRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGattWriteRequestedEventArgsImpl: Sized {
    fn Session(&mut self) -> ::windows::core::Result<GattSession>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
    fn GetRequestAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteRequest>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattWriteRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattWriteRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGattWriteRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattWriteRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattWriteRequestedEventArgsVtbl {
        unsafe extern "system" fn Session<Impl: IGattWriteRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IGattWriteRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetRequestAsync<Impl: IGattWriteRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRequestAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattWriteRequestedEventArgs, BASE_OFFSET>(),
            Session: Session::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
            GetRequestAsync: GetRequestAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattWriteRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGattWriteResultImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<GattCommunicationStatus>;
    fn ProtocolError(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattWriteResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattWriteResult";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGattWriteResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattWriteResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGattWriteResultVtbl {
        unsafe extern "system" fn Status<Impl: IGattWriteResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GattCommunicationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProtocolError<Impl: IGattWriteResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtocolError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGattWriteResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            ProtocolError: ProtocolError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGattWriteResult as ::windows::core::Interface>::IID
    }
}
