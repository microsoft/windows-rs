#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristicImpl: Sized {
    fn GetDescriptors(&self, descriptoruuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDescriptor>>;
    fn CharacteristicProperties(&self) -> ::windows::core::Result<GattCharacteristicProperties>;
    fn ProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel>;
    fn SetProtectionLevel(&self, value: GattProtectionLevel) -> ::windows::core::Result<()>;
    fn UserDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Uuid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn AttributeHandle(&self) -> ::windows::core::Result<u16>;
    fn PresentationFormats(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattPresentationFormat>>;
    fn ReadValueAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadResult>>;
    fn ReadValueWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadResult>>;
    fn WriteValueAsync(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>;
    fn WriteValueWithOptionAsync(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, writeoption: GattWriteOption) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>;
    fn ReadClientCharacteristicConfigurationDescriptorAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadClientCharacteristicConfigurationDescriptorResult>>;
    fn WriteClientCharacteristicConfigurationDescriptorAsync(&self, clientcharacteristicconfigurationdescriptorvalue: GattClientCharacteristicConfigurationDescriptorValue) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>;
    fn ValueChanged(&self, valuechangedhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattCharacteristic, GattValueChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveValueChanged(&self, valuechangedeventcookie: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattCharacteristic {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattCharacteristic";
}
#[cfg(feature = "implement_exclusive")]
impl IGattCharacteristicVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattCharacteristicImpl, const OFFSET: isize>() -> IGattCharacteristicVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGattCharacteristic>,
            ::windows::core::GetTrustLevel,
            GetDescriptors::<Impl, OFFSET>,
            CharacteristicProperties::<Impl, OFFSET>,
            ProtectionLevel::<Impl, OFFSET>,
            SetProtectionLevel::<Impl, OFFSET>,
            UserDescription::<Impl, OFFSET>,
            Uuid::<Impl, OFFSET>,
            AttributeHandle::<Impl, OFFSET>,
            PresentationFormats::<Impl, OFFSET>,
            ReadValueAsync::<Impl, OFFSET>,
            ReadValueWithCacheModeAsync::<Impl, OFFSET>,
            WriteValueAsync::<Impl, OFFSET>,
            WriteValueWithOptionAsync::<Impl, OFFSET>,
            ReadClientCharacteristicConfigurationDescriptorAsync::<Impl, OFFSET>,
            WriteClientCharacteristicConfigurationDescriptorAsync::<Impl, OFFSET>,
            ValueChanged::<Impl, OFFSET>,
            RemoveValueChanged::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristic2Impl: Sized + IGattCharacteristicImpl {
    fn Service(&self) -> ::windows::core::Result<GattDeviceService>;
    fn GetAllDescriptors(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDescriptor>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattCharacteristic2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattCharacteristic2";
}
#[cfg(feature = "implement_exclusive")]
impl IGattCharacteristic2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattCharacteristic2Impl, const OFFSET: isize>() -> IGattCharacteristic2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattCharacteristic2>, ::windows::core::GetTrustLevel, Service::<Impl, OFFSET>, GetAllDescriptors::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristic3Impl: Sized {
    fn GetDescriptorsAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>>;
    fn GetDescriptorsWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>>;
    fn GetDescriptorsForUuidAsync(&self, descriptoruuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>>;
    fn GetDescriptorsForUuidWithCacheModeAsync(&self, descriptoruuid: &::windows::core::GUID, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDescriptorsResult>>;
    fn WriteValueWithResultAsync(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>;
    fn WriteValueWithResultAndOptionAsync(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, writeoption: GattWriteOption) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>;
    fn WriteClientCharacteristicConfigurationDescriptorWithResultAsync(&self, clientcharacteristicconfigurationdescriptorvalue: GattClientCharacteristicConfigurationDescriptorValue) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattCharacteristic3 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattCharacteristic3";
}
#[cfg(feature = "implement_exclusive")]
impl IGattCharacteristic3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattCharacteristic3Impl, const OFFSET: isize>() -> IGattCharacteristic3Vtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGattCharacteristic3>,
            ::windows::core::GetTrustLevel,
            GetDescriptorsAsync::<Impl, OFFSET>,
            GetDescriptorsWithCacheModeAsync::<Impl, OFFSET>,
            GetDescriptorsForUuidAsync::<Impl, OFFSET>,
            GetDescriptorsForUuidWithCacheModeAsync::<Impl, OFFSET>,
            WriteValueWithResultAsync::<Impl, OFFSET>,
            WriteValueWithResultAndOptionAsync::<Impl, OFFSET>,
            WriteClientCharacteristicConfigurationDescriptorWithResultAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristicStaticsImpl: Sized {
    fn ConvertShortIdToUuid(&self, shortid: u16) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattCharacteristicStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattCharacteristicStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGattCharacteristicStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattCharacteristicStaticsImpl, const OFFSET: isize>() -> IGattCharacteristicStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattCharacteristicStatics>, ::windows::core::GetTrustLevel, ConvertShortIdToUuid::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristicUuidsStaticsImpl: Sized {
    fn BatteryLevel(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn BloodPressureFeature(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn BloodPressureMeasurement(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn BodySensorLocation(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CscFeature(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CscMeasurement(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GlucoseFeature(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GlucoseMeasurement(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GlucoseMeasurementContext(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn HeartRateControlPoint(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn HeartRateMeasurement(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IntermediateCuffPressure(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IntermediateTemperature(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn MeasurementInterval(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RecordAccessControlPoint(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RscFeature(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RscMeasurement(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SCControlPoint(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SensorLocation(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TemperatureMeasurement(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TemperatureType(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattCharacteristicUuidsStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattCharacteristicUuidsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGattCharacteristicUuidsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattCharacteristicUuidsStaticsImpl, const OFFSET: isize>() -> IGattCharacteristicUuidsStaticsVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGattCharacteristicUuidsStatics>,
            ::windows::core::GetTrustLevel,
            BatteryLevel::<Impl, OFFSET>,
            BloodPressureFeature::<Impl, OFFSET>,
            BloodPressureMeasurement::<Impl, OFFSET>,
            BodySensorLocation::<Impl, OFFSET>,
            CscFeature::<Impl, OFFSET>,
            CscMeasurement::<Impl, OFFSET>,
            GlucoseFeature::<Impl, OFFSET>,
            GlucoseMeasurement::<Impl, OFFSET>,
            GlucoseMeasurementContext::<Impl, OFFSET>,
            HeartRateControlPoint::<Impl, OFFSET>,
            HeartRateMeasurement::<Impl, OFFSET>,
            IntermediateCuffPressure::<Impl, OFFSET>,
            IntermediateTemperature::<Impl, OFFSET>,
            MeasurementInterval::<Impl, OFFSET>,
            RecordAccessControlPoint::<Impl, OFFSET>,
            RscFeature::<Impl, OFFSET>,
            RscMeasurement::<Impl, OFFSET>,
            SCControlPoint::<Impl, OFFSET>,
            SensorLocation::<Impl, OFFSET>,
            TemperatureMeasurement::<Impl, OFFSET>,
            TemperatureType::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristicUuidsStatics2Impl: Sized {
    fn AlertCategoryId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn AlertCategoryIdBitMask(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn AlertLevel(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn AlertNotificationControlPoint(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn AlertStatus(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GapAppearance(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn BootKeyboardInputReport(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn BootKeyboardOutputReport(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn BootMouseInputReport(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CurrentTime(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CyclingPowerControlPoint(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CyclingPowerFeature(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CyclingPowerMeasurement(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CyclingPowerVector(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn DateTime(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn DayDateTime(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn DayOfWeek(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GapDeviceName(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn DstOffset(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ExactTime256(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn FirmwareRevisionString(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn HardwareRevisionString(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn HidControlPoint(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn HidInformation(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Ieee1107320601RegulatoryCertificationDataList(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn LnControlPoint(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn LnFeature(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn LocalTimeInformation(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn LocationAndSpeed(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ManufacturerNameString(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ModelNumberString(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Navigation(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn NewAlert(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GapPeripheralPreferredConnectionParameters(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GapPeripheralPrivacyFlag(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn PnpId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn PositionQuality(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ProtocolMode(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GapReconnectionAddress(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ReferenceTimeInformation(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Report(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ReportMap(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RingerControlPoint(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RingerSetting(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ScanIntervalWindow(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ScanRefresh(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SerialNumberString(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GattServiceChanged(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SoftwareRevisionString(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SupportedNewAlertCategory(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SupportUnreadAlertCategory(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SystemId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TimeAccuracy(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TimeSource(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TimeUpdateControlPoint(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TimeUpdateState(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TimeWithDst(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TimeZone(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TxPowerLevel(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn UnreadAlertStatus(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattCharacteristicUuidsStatics2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattCharacteristicUuidsStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IGattCharacteristicUuidsStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattCharacteristicUuidsStatics2Impl, const OFFSET: isize>() -> IGattCharacteristicUuidsStatics2Vtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGattCharacteristicUuidsStatics2>,
            ::windows::core::GetTrustLevel,
            AlertCategoryId::<Impl, OFFSET>,
            AlertCategoryIdBitMask::<Impl, OFFSET>,
            AlertLevel::<Impl, OFFSET>,
            AlertNotificationControlPoint::<Impl, OFFSET>,
            AlertStatus::<Impl, OFFSET>,
            GapAppearance::<Impl, OFFSET>,
            BootKeyboardInputReport::<Impl, OFFSET>,
            BootKeyboardOutputReport::<Impl, OFFSET>,
            BootMouseInputReport::<Impl, OFFSET>,
            CurrentTime::<Impl, OFFSET>,
            CyclingPowerControlPoint::<Impl, OFFSET>,
            CyclingPowerFeature::<Impl, OFFSET>,
            CyclingPowerMeasurement::<Impl, OFFSET>,
            CyclingPowerVector::<Impl, OFFSET>,
            DateTime::<Impl, OFFSET>,
            DayDateTime::<Impl, OFFSET>,
            DayOfWeek::<Impl, OFFSET>,
            GapDeviceName::<Impl, OFFSET>,
            DstOffset::<Impl, OFFSET>,
            ExactTime256::<Impl, OFFSET>,
            FirmwareRevisionString::<Impl, OFFSET>,
            HardwareRevisionString::<Impl, OFFSET>,
            HidControlPoint::<Impl, OFFSET>,
            HidInformation::<Impl, OFFSET>,
            Ieee1107320601RegulatoryCertificationDataList::<Impl, OFFSET>,
            LnControlPoint::<Impl, OFFSET>,
            LnFeature::<Impl, OFFSET>,
            LocalTimeInformation::<Impl, OFFSET>,
            LocationAndSpeed::<Impl, OFFSET>,
            ManufacturerNameString::<Impl, OFFSET>,
            ModelNumberString::<Impl, OFFSET>,
            Navigation::<Impl, OFFSET>,
            NewAlert::<Impl, OFFSET>,
            GapPeripheralPreferredConnectionParameters::<Impl, OFFSET>,
            GapPeripheralPrivacyFlag::<Impl, OFFSET>,
            PnpId::<Impl, OFFSET>,
            PositionQuality::<Impl, OFFSET>,
            ProtocolMode::<Impl, OFFSET>,
            GapReconnectionAddress::<Impl, OFFSET>,
            ReferenceTimeInformation::<Impl, OFFSET>,
            Report::<Impl, OFFSET>,
            ReportMap::<Impl, OFFSET>,
            RingerControlPoint::<Impl, OFFSET>,
            RingerSetting::<Impl, OFFSET>,
            ScanIntervalWindow::<Impl, OFFSET>,
            ScanRefresh::<Impl, OFFSET>,
            SerialNumberString::<Impl, OFFSET>,
            GattServiceChanged::<Impl, OFFSET>,
            SoftwareRevisionString::<Impl, OFFSET>,
            SupportedNewAlertCategory::<Impl, OFFSET>,
            SupportUnreadAlertCategory::<Impl, OFFSET>,
            SystemId::<Impl, OFFSET>,
            TimeAccuracy::<Impl, OFFSET>,
            TimeSource::<Impl, OFFSET>,
            TimeUpdateControlPoint::<Impl, OFFSET>,
            TimeUpdateState::<Impl, OFFSET>,
            TimeWithDst::<Impl, OFFSET>,
            TimeZone::<Impl, OFFSET>,
            TxPowerLevel::<Impl, OFFSET>,
            UnreadAlertStatus::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristicsResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus>;
    fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>>;
    fn Characteristics(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattCharacteristic>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattCharacteristicsResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattCharacteristicsResult";
}
#[cfg(feature = "implement_exclusive")]
impl IGattCharacteristicsResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattCharacteristicsResultImpl, const OFFSET: isize>() -> IGattCharacteristicsResultVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattCharacteristicsResult>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>, ProtocolError::<Impl, OFFSET>, Characteristics::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattClientNotificationResultImpl: Sized {
    fn SubscribedClient(&self) -> ::windows::core::Result<GattSubscribedClient>;
    fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus>;
    fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattClientNotificationResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattClientNotificationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IGattClientNotificationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattClientNotificationResultImpl, const OFFSET: isize>() -> IGattClientNotificationResultVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattClientNotificationResult>, ::windows::core::GetTrustLevel, SubscribedClient::<Impl, OFFSET>, Status::<Impl, OFFSET>, ProtocolError::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattClientNotificationResult2Impl: Sized {
    fn BytesSent(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattClientNotificationResult2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattClientNotificationResult2";
}
#[cfg(feature = "implement_exclusive")]
impl IGattClientNotificationResult2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattClientNotificationResult2Impl, const OFFSET: isize>() -> IGattClientNotificationResult2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattClientNotificationResult2>, ::windows::core::GetTrustLevel, BytesSent::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattDescriptorImpl: Sized {
    fn ProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel>;
    fn SetProtectionLevel(&self, value: GattProtectionLevel) -> ::windows::core::Result<()>;
    fn Uuid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn AttributeHandle(&self) -> ::windows::core::Result<u16>;
    fn ReadValueAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadResult>>;
    fn ReadValueWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadResult>>;
    fn WriteValueAsync(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattDescriptor {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattDescriptor";
}
#[cfg(feature = "implement_exclusive")]
impl IGattDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattDescriptorImpl, const OFFSET: isize>() -> IGattDescriptorVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGattDescriptor>,
            ::windows::core::GetTrustLevel,
            ProtectionLevel::<Impl, OFFSET>,
            SetProtectionLevel::<Impl, OFFSET>,
            Uuid::<Impl, OFFSET>,
            AttributeHandle::<Impl, OFFSET>,
            ReadValueAsync::<Impl, OFFSET>,
            ReadValueWithCacheModeAsync::<Impl, OFFSET>,
            WriteValueAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattDescriptor2Impl: Sized {
    fn WriteValueWithResultAsync(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattDescriptor2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattDescriptor2";
}
#[cfg(feature = "implement_exclusive")]
impl IGattDescriptor2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattDescriptor2Impl, const OFFSET: isize>() -> IGattDescriptor2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattDescriptor2>, ::windows::core::GetTrustLevel, WriteValueWithResultAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattDescriptorStaticsImpl: Sized {
    fn ConvertShortIdToUuid(&self, shortid: u16) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattDescriptorStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattDescriptorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGattDescriptorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattDescriptorStaticsImpl, const OFFSET: isize>() -> IGattDescriptorStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattDescriptorStatics>, ::windows::core::GetTrustLevel, ConvertShortIdToUuid::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattDescriptorUuidsStaticsImpl: Sized {
    fn CharacteristicAggregateFormat(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CharacteristicExtendedProperties(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CharacteristicPresentationFormat(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CharacteristicUserDescription(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ClientCharacteristicConfiguration(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ServerCharacteristicConfiguration(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattDescriptorUuidsStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattDescriptorUuidsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGattDescriptorUuidsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattDescriptorUuidsStaticsImpl, const OFFSET: isize>() -> IGattDescriptorUuidsStaticsVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGattDescriptorUuidsStatics>,
            ::windows::core::GetTrustLevel,
            CharacteristicAggregateFormat::<Impl, OFFSET>,
            CharacteristicExtendedProperties::<Impl, OFFSET>,
            CharacteristicPresentationFormat::<Impl, OFFSET>,
            CharacteristicUserDescription::<Impl, OFFSET>,
            ClientCharacteristicConfiguration::<Impl, OFFSET>,
            ServerCharacteristicConfiguration::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattDescriptorsResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus>;
    fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>>;
    fn Descriptors(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDescriptor>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattDescriptorsResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattDescriptorsResult";
}
#[cfg(feature = "implement_exclusive")]
impl IGattDescriptorsResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattDescriptorsResultImpl, const OFFSET: isize>() -> IGattDescriptorsResultVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattDescriptorsResult>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>, ProtocolError::<Impl, OFFSET>, Descriptors::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGattDeviceServiceImpl: Sized + IClosableImpl {
    fn GetCharacteristics(&self, characteristicuuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattCharacteristic>>;
    fn GetIncludedServices(&self, serviceuuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Uuid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn AttributeHandle(&self) -> ::windows::core::Result<u16>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattDeviceService {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattDeviceService";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGattDeviceServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattDeviceServiceImpl, const OFFSET: isize>() -> IGattDeviceServiceVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattDeviceService>, ::windows::core::GetTrustLevel, GetCharacteristics::<Impl, OFFSET>, GetIncludedServices::<Impl, OFFSET>, DeviceId::<Impl, OFFSET>, Uuid::<Impl, OFFSET>, AttributeHandle::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGattDeviceService2Impl: Sized + IClosableImpl + IGattDeviceServiceImpl {
    fn Device(&self) -> ::windows::core::Result<super::BluetoothLEDevice>;
    fn ParentServices(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>>;
    fn GetAllCharacteristics(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattCharacteristic>>;
    fn GetAllIncludedServices(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGattDeviceService2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattDeviceService2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGattDeviceService2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattDeviceService2Impl, const OFFSET: isize>() -> IGattDeviceService2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattDeviceService2>, ::windows::core::GetTrustLevel, Device::<Impl, OFFSET>, ParentServices::<Impl, OFFSET>, GetAllCharacteristics::<Impl, OFFSET>, GetAllIncludedServices::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattDeviceService3Impl: Sized {
    fn DeviceAccessInformation(&self) -> ::windows::core::Result<super::super::Enumeration::DeviceAccessInformation>;
    fn Session(&self) -> ::windows::core::Result<GattSession>;
    fn SharingMode(&self) -> ::windows::core::Result<GattSharingMode>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::Enumeration::DeviceAccessStatus>>;
    fn OpenAsync(&self, sharingmode: GattSharingMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattOpenStatus>>;
    fn GetCharacteristicsAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>>;
    fn GetCharacteristicsWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>>;
    fn GetCharacteristicsForUuidAsync(&self, characteristicuuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>>;
    fn GetCharacteristicsForUuidWithCacheModeAsync(&self, characteristicuuid: &::windows::core::GUID, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCharacteristicsResult>>;
    fn GetIncludedServicesAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>>;
    fn GetIncludedServicesWithCacheModeAsync(&self, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>>;
    fn GetIncludedServicesForUuidAsync(&self, serviceuuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>>;
    fn GetIncludedServicesForUuidWithCacheModeAsync(&self, serviceuuid: &::windows::core::GUID, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceServicesResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattDeviceService3 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattDeviceService3";
}
#[cfg(feature = "implement_exclusive")]
impl IGattDeviceService3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattDeviceService3Impl, const OFFSET: isize>() -> IGattDeviceService3Vtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGattDeviceService3>,
            ::windows::core::GetTrustLevel,
            DeviceAccessInformation::<Impl, OFFSET>,
            Session::<Impl, OFFSET>,
            SharingMode::<Impl, OFFSET>,
            RequestAccessAsync::<Impl, OFFSET>,
            OpenAsync::<Impl, OFFSET>,
            GetCharacteristicsAsync::<Impl, OFFSET>,
            GetCharacteristicsWithCacheModeAsync::<Impl, OFFSET>,
            GetCharacteristicsForUuidAsync::<Impl, OFFSET>,
            GetCharacteristicsForUuidWithCacheModeAsync::<Impl, OFFSET>,
            GetIncludedServicesAsync::<Impl, OFFSET>,
            GetIncludedServicesWithCacheModeAsync::<Impl, OFFSET>,
            GetIncludedServicesForUuidAsync::<Impl, OFFSET>,
            GetIncludedServicesForUuidWithCacheModeAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattDeviceServiceStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceService>>;
    fn GetDeviceSelectorFromUuid(&self, serviceuuid: &::windows::core::GUID) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromShortId(&self, serviceshortid: u16) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ConvertShortIdToUuid(&self, shortid: u16) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattDeviceServiceStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattDeviceServiceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGattDeviceServiceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattDeviceServiceStaticsImpl, const OFFSET: isize>() -> IGattDeviceServiceStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattDeviceServiceStatics>, ::windows::core::GetTrustLevel, FromIdAsync::<Impl, OFFSET>, GetDeviceSelectorFromUuid::<Impl, OFFSET>, GetDeviceSelectorFromShortId::<Impl, OFFSET>, ConvertShortIdToUuid::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattDeviceServiceStatics2Impl: Sized {
    fn FromIdWithSharingModeAsync(&self, deviceid: &::windows::core::HSTRING, sharingmode: GattSharingMode) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattDeviceService>>;
    fn GetDeviceSelectorForBluetoothDeviceId(&self, bluetoothdeviceid: &::core::option::Option<super::BluetoothDeviceId>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorForBluetoothDeviceIdWithCacheMode(&self, bluetoothdeviceid: &::core::option::Option<super::BluetoothDeviceId>, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorForBluetoothDeviceIdAndUuid(&self, bluetoothdeviceid: &::core::option::Option<super::BluetoothDeviceId>, serviceuuid: &::windows::core::GUID) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorForBluetoothDeviceIdAndUuidWithCacheMode(&self, bluetoothdeviceid: &::core::option::Option<super::BluetoothDeviceId>, serviceuuid: &::windows::core::GUID, cachemode: super::BluetoothCacheMode) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattDeviceServiceStatics2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattDeviceServiceStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IGattDeviceServiceStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattDeviceServiceStatics2Impl, const OFFSET: isize>() -> IGattDeviceServiceStatics2Vtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGattDeviceServiceStatics2>,
            ::windows::core::GetTrustLevel,
            FromIdWithSharingModeAsync::<Impl, OFFSET>,
            GetDeviceSelectorForBluetoothDeviceId::<Impl, OFFSET>,
            GetDeviceSelectorForBluetoothDeviceIdWithCacheMode::<Impl, OFFSET>,
            GetDeviceSelectorForBluetoothDeviceIdAndUuid::<Impl, OFFSET>,
            GetDeviceSelectorForBluetoothDeviceIdAndUuidWithCacheMode::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattDeviceServicesResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus>;
    fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>>;
    fn Services(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattDeviceService>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattDeviceServicesResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattDeviceServicesResult";
}
#[cfg(feature = "implement_exclusive")]
impl IGattDeviceServicesResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattDeviceServicesResultImpl, const OFFSET: isize>() -> IGattDeviceServicesResultVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattDeviceServicesResult>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>, ProtocolError::<Impl, OFFSET>, Services::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattLocalCharacteristicImpl: Sized {
    fn Uuid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn StaticValue(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn CharacteristicProperties(&self) -> ::windows::core::Result<GattCharacteristicProperties>;
    fn ReadProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel>;
    fn WriteProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel>;
    fn CreateDescriptorAsync(&self, descriptoruuid: &::windows::core::GUID, parameters: &::core::option::Option<GattLocalDescriptorParameters>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattLocalDescriptorResult>>;
    fn Descriptors(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattLocalDescriptor>>;
    fn UserDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PresentationFormats(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattPresentationFormat>>;
    fn SubscribedClients(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattSubscribedClient>>;
    fn SubscribedClientsChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattLocalCharacteristic, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSubscribedClientsChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ReadRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattLocalCharacteristic, GattReadRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn WriteRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattLocalCharacteristic, GattWriteRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveWriteRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NotifyValueAsync(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<GattClientNotificationResult>>>;
    fn NotifyValueForSubscribedClientAsync(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, subscribedclient: &::core::option::Option<GattSubscribedClient>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattClientNotificationResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattLocalCharacteristic {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattLocalCharacteristic";
}
#[cfg(feature = "implement_exclusive")]
impl IGattLocalCharacteristicVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattLocalCharacteristicImpl, const OFFSET: isize>() -> IGattLocalCharacteristicVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGattLocalCharacteristic>,
            ::windows::core::GetTrustLevel,
            Uuid::<Impl, OFFSET>,
            StaticValue::<Impl, OFFSET>,
            CharacteristicProperties::<Impl, OFFSET>,
            ReadProtectionLevel::<Impl, OFFSET>,
            WriteProtectionLevel::<Impl, OFFSET>,
            CreateDescriptorAsync::<Impl, OFFSET>,
            Descriptors::<Impl, OFFSET>,
            UserDescription::<Impl, OFFSET>,
            PresentationFormats::<Impl, OFFSET>,
            SubscribedClients::<Impl, OFFSET>,
            SubscribedClientsChanged::<Impl, OFFSET>,
            RemoveSubscribedClientsChanged::<Impl, OFFSET>,
            ReadRequested::<Impl, OFFSET>,
            RemoveReadRequested::<Impl, OFFSET>,
            WriteRequested::<Impl, OFFSET>,
            RemoveWriteRequested::<Impl, OFFSET>,
            NotifyValueAsync::<Impl, OFFSET>,
            NotifyValueForSubscribedClientAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattLocalCharacteristicParametersImpl: Sized {
    fn SetStaticValue(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn StaticValue(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetCharacteristicProperties(&self, value: GattCharacteristicProperties) -> ::windows::core::Result<()>;
    fn CharacteristicProperties(&self) -> ::windows::core::Result<GattCharacteristicProperties>;
    fn SetReadProtectionLevel(&self, value: GattProtectionLevel) -> ::windows::core::Result<()>;
    fn ReadProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel>;
    fn SetWriteProtectionLevel(&self, value: GattProtectionLevel) -> ::windows::core::Result<()>;
    fn WriteProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel>;
    fn SetUserDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UserDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PresentationFormats(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<GattPresentationFormat>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattLocalCharacteristicParameters {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattLocalCharacteristicParameters";
}
#[cfg(feature = "implement_exclusive")]
impl IGattLocalCharacteristicParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattLocalCharacteristicParametersImpl, const OFFSET: isize>() -> IGattLocalCharacteristicParametersVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGattLocalCharacteristicParameters>,
            ::windows::core::GetTrustLevel,
            SetStaticValue::<Impl, OFFSET>,
            StaticValue::<Impl, OFFSET>,
            SetCharacteristicProperties::<Impl, OFFSET>,
            CharacteristicProperties::<Impl, OFFSET>,
            SetReadProtectionLevel::<Impl, OFFSET>,
            ReadProtectionLevel::<Impl, OFFSET>,
            SetWriteProtectionLevel::<Impl, OFFSET>,
            WriteProtectionLevel::<Impl, OFFSET>,
            SetUserDescription::<Impl, OFFSET>,
            UserDescription::<Impl, OFFSET>,
            PresentationFormats::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattLocalCharacteristicResultImpl: Sized {
    fn Characteristic(&self) -> ::windows::core::Result<GattLocalCharacteristic>;
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattLocalCharacteristicResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattLocalCharacteristicResult";
}
#[cfg(feature = "implement_exclusive")]
impl IGattLocalCharacteristicResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattLocalCharacteristicResultImpl, const OFFSET: isize>() -> IGattLocalCharacteristicResultVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattLocalCharacteristicResult>, ::windows::core::GetTrustLevel, Characteristic::<Impl, OFFSET>, Error::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattLocalDescriptorImpl: Sized {
    fn Uuid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn StaticValue(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn ReadProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel>;
    fn WriteProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel>;
    fn ReadRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattLocalDescriptor, GattReadRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveReadRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn WriteRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattLocalDescriptor, GattWriteRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveWriteRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattLocalDescriptor {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattLocalDescriptor";
}
#[cfg(feature = "implement_exclusive")]
impl IGattLocalDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattLocalDescriptorImpl, const OFFSET: isize>() -> IGattLocalDescriptorVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGattLocalDescriptor>,
            ::windows::core::GetTrustLevel,
            Uuid::<Impl, OFFSET>,
            StaticValue::<Impl, OFFSET>,
            ReadProtectionLevel::<Impl, OFFSET>,
            WriteProtectionLevel::<Impl, OFFSET>,
            ReadRequested::<Impl, OFFSET>,
            RemoveReadRequested::<Impl, OFFSET>,
            WriteRequested::<Impl, OFFSET>,
            RemoveWriteRequested::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattLocalDescriptorParametersImpl: Sized {
    fn SetStaticValue(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn StaticValue(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetReadProtectionLevel(&self, value: GattProtectionLevel) -> ::windows::core::Result<()>;
    fn ReadProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel>;
    fn SetWriteProtectionLevel(&self, value: GattProtectionLevel) -> ::windows::core::Result<()>;
    fn WriteProtectionLevel(&self) -> ::windows::core::Result<GattProtectionLevel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattLocalDescriptorParameters {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattLocalDescriptorParameters";
}
#[cfg(feature = "implement_exclusive")]
impl IGattLocalDescriptorParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattLocalDescriptorParametersImpl, const OFFSET: isize>() -> IGattLocalDescriptorParametersVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGattLocalDescriptorParameters>,
            ::windows::core::GetTrustLevel,
            SetStaticValue::<Impl, OFFSET>,
            StaticValue::<Impl, OFFSET>,
            SetReadProtectionLevel::<Impl, OFFSET>,
            ReadProtectionLevel::<Impl, OFFSET>,
            SetWriteProtectionLevel::<Impl, OFFSET>,
            WriteProtectionLevel::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattLocalDescriptorResultImpl: Sized {
    fn Descriptor(&self) -> ::windows::core::Result<GattLocalDescriptor>;
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattLocalDescriptorResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattLocalDescriptorResult";
}
#[cfg(feature = "implement_exclusive")]
impl IGattLocalDescriptorResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattLocalDescriptorResultImpl, const OFFSET: isize>() -> IGattLocalDescriptorResultVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattLocalDescriptorResult>, ::windows::core::GetTrustLevel, Descriptor::<Impl, OFFSET>, Error::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattLocalServiceImpl: Sized {
    fn Uuid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CreateCharacteristicAsync(&self, characteristicuuid: &::windows::core::GUID, parameters: &::core::option::Option<GattLocalCharacteristicParameters>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattLocalCharacteristicResult>>;
    fn Characteristics(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GattLocalCharacteristic>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattLocalService {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattLocalService";
}
#[cfg(feature = "implement_exclusive")]
impl IGattLocalServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattLocalServiceImpl, const OFFSET: isize>() -> IGattLocalServiceVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattLocalService>, ::windows::core::GetTrustLevel, Uuid::<Impl, OFFSET>, CreateCharacteristicAsync::<Impl, OFFSET>, Characteristics::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattPresentationFormatImpl: Sized {
    fn FormatType(&self) -> ::windows::core::Result<u8>;
    fn Exponent(&self) -> ::windows::core::Result<i32>;
    fn Unit(&self) -> ::windows::core::Result<u16>;
    fn Namespace(&self) -> ::windows::core::Result<u8>;
    fn Description(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattPresentationFormat {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattPresentationFormat";
}
#[cfg(feature = "implement_exclusive")]
impl IGattPresentationFormatVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattPresentationFormatImpl, const OFFSET: isize>() -> IGattPresentationFormatVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattPresentationFormat>, ::windows::core::GetTrustLevel, FormatType::<Impl, OFFSET>, Exponent::<Impl, OFFSET>, Unit::<Impl, OFFSET>, Namespace::<Impl, OFFSET>, Description::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattPresentationFormatStaticsImpl: Sized {
    fn BluetoothSigAssignedNumbers(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattPresentationFormatStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattPresentationFormatStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGattPresentationFormatStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattPresentationFormatStaticsImpl, const OFFSET: isize>() -> IGattPresentationFormatStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattPresentationFormatStatics>, ::windows::core::GetTrustLevel, BluetoothSigAssignedNumbers::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattPresentationFormatStatics2Impl: Sized + IGattPresentationFormatStaticsImpl {
    fn FromParts(&self, formattype: u8, exponent: i32, unit: u16, namespaceid: u8, description: u16) -> ::windows::core::Result<GattPresentationFormat>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattPresentationFormatStatics2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattPresentationFormatStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IGattPresentationFormatStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattPresentationFormatStatics2Impl, const OFFSET: isize>() -> IGattPresentationFormatStatics2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattPresentationFormatStatics2>, ::windows::core::GetTrustLevel, FromParts::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattPresentationFormatTypesStaticsImpl: Sized {
    fn Boolean(&self) -> ::windows::core::Result<u8>;
    fn Bit2(&self) -> ::windows::core::Result<u8>;
    fn Nibble(&self) -> ::windows::core::Result<u8>;
    fn UInt8(&self) -> ::windows::core::Result<u8>;
    fn UInt12(&self) -> ::windows::core::Result<u8>;
    fn UInt16(&self) -> ::windows::core::Result<u8>;
    fn UInt24(&self) -> ::windows::core::Result<u8>;
    fn UInt32(&self) -> ::windows::core::Result<u8>;
    fn UInt48(&self) -> ::windows::core::Result<u8>;
    fn UInt64(&self) -> ::windows::core::Result<u8>;
    fn UInt128(&self) -> ::windows::core::Result<u8>;
    fn SInt8(&self) -> ::windows::core::Result<u8>;
    fn SInt12(&self) -> ::windows::core::Result<u8>;
    fn SInt16(&self) -> ::windows::core::Result<u8>;
    fn SInt24(&self) -> ::windows::core::Result<u8>;
    fn SInt32(&self) -> ::windows::core::Result<u8>;
    fn SInt48(&self) -> ::windows::core::Result<u8>;
    fn SInt64(&self) -> ::windows::core::Result<u8>;
    fn SInt128(&self) -> ::windows::core::Result<u8>;
    fn Float32(&self) -> ::windows::core::Result<u8>;
    fn Float64(&self) -> ::windows::core::Result<u8>;
    fn SFloat(&self) -> ::windows::core::Result<u8>;
    fn Float(&self) -> ::windows::core::Result<u8>;
    fn DUInt16(&self) -> ::windows::core::Result<u8>;
    fn Utf8(&self) -> ::windows::core::Result<u8>;
    fn Utf16(&self) -> ::windows::core::Result<u8>;
    fn Struct(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattPresentationFormatTypesStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattPresentationFormatTypesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGattPresentationFormatTypesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattPresentationFormatTypesStaticsImpl, const OFFSET: isize>() -> IGattPresentationFormatTypesStaticsVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGattPresentationFormatTypesStatics>,
            ::windows::core::GetTrustLevel,
            Boolean::<Impl, OFFSET>,
            Bit2::<Impl, OFFSET>,
            Nibble::<Impl, OFFSET>,
            UInt8::<Impl, OFFSET>,
            UInt12::<Impl, OFFSET>,
            UInt16::<Impl, OFFSET>,
            UInt24::<Impl, OFFSET>,
            UInt32::<Impl, OFFSET>,
            UInt48::<Impl, OFFSET>,
            UInt64::<Impl, OFFSET>,
            UInt128::<Impl, OFFSET>,
            SInt8::<Impl, OFFSET>,
            SInt12::<Impl, OFFSET>,
            SInt16::<Impl, OFFSET>,
            SInt24::<Impl, OFFSET>,
            SInt32::<Impl, OFFSET>,
            SInt48::<Impl, OFFSET>,
            SInt64::<Impl, OFFSET>,
            SInt128::<Impl, OFFSET>,
            Float32::<Impl, OFFSET>,
            Float64::<Impl, OFFSET>,
            SFloat::<Impl, OFFSET>,
            Float::<Impl, OFFSET>,
            DUInt16::<Impl, OFFSET>,
            Utf8::<Impl, OFFSET>,
            Utf16::<Impl, OFFSET>,
            Struct::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattProtocolErrorStaticsImpl: Sized {
    fn InvalidHandle(&self) -> ::windows::core::Result<u8>;
    fn ReadNotPermitted(&self) -> ::windows::core::Result<u8>;
    fn WriteNotPermitted(&self) -> ::windows::core::Result<u8>;
    fn InvalidPdu(&self) -> ::windows::core::Result<u8>;
    fn InsufficientAuthentication(&self) -> ::windows::core::Result<u8>;
    fn RequestNotSupported(&self) -> ::windows::core::Result<u8>;
    fn InvalidOffset(&self) -> ::windows::core::Result<u8>;
    fn InsufficientAuthorization(&self) -> ::windows::core::Result<u8>;
    fn PrepareQueueFull(&self) -> ::windows::core::Result<u8>;
    fn AttributeNotFound(&self) -> ::windows::core::Result<u8>;
    fn AttributeNotLong(&self) -> ::windows::core::Result<u8>;
    fn InsufficientEncryptionKeySize(&self) -> ::windows::core::Result<u8>;
    fn InvalidAttributeValueLength(&self) -> ::windows::core::Result<u8>;
    fn UnlikelyError(&self) -> ::windows::core::Result<u8>;
    fn InsufficientEncryption(&self) -> ::windows::core::Result<u8>;
    fn UnsupportedGroupType(&self) -> ::windows::core::Result<u8>;
    fn InsufficientResources(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattProtocolErrorStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattProtocolErrorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGattProtocolErrorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattProtocolErrorStaticsImpl, const OFFSET: isize>() -> IGattProtocolErrorStaticsVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGattProtocolErrorStatics>,
            ::windows::core::GetTrustLevel,
            InvalidHandle::<Impl, OFFSET>,
            ReadNotPermitted::<Impl, OFFSET>,
            WriteNotPermitted::<Impl, OFFSET>,
            InvalidPdu::<Impl, OFFSET>,
            InsufficientAuthentication::<Impl, OFFSET>,
            RequestNotSupported::<Impl, OFFSET>,
            InvalidOffset::<Impl, OFFSET>,
            InsufficientAuthorization::<Impl, OFFSET>,
            PrepareQueueFull::<Impl, OFFSET>,
            AttributeNotFound::<Impl, OFFSET>,
            AttributeNotLong::<Impl, OFFSET>,
            InsufficientEncryptionKeySize::<Impl, OFFSET>,
            InvalidAttributeValueLength::<Impl, OFFSET>,
            UnlikelyError::<Impl, OFFSET>,
            InsufficientEncryption::<Impl, OFFSET>,
            UnsupportedGroupType::<Impl, OFFSET>,
            InsufficientResources::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattReadClientCharacteristicConfigurationDescriptorResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus>;
    fn ClientCharacteristicConfigurationDescriptor(&self) -> ::windows::core::Result<GattClientCharacteristicConfigurationDescriptorValue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattReadClientCharacteristicConfigurationDescriptorResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattReadClientCharacteristicConfigurationDescriptorResult";
}
#[cfg(feature = "implement_exclusive")]
impl IGattReadClientCharacteristicConfigurationDescriptorResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattReadClientCharacteristicConfigurationDescriptorResultImpl, const OFFSET: isize>() -> IGattReadClientCharacteristicConfigurationDescriptorResultVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattReadClientCharacteristicConfigurationDescriptorResult>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>, ClientCharacteristicConfigurationDescriptor::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattReadClientCharacteristicConfigurationDescriptorResult2Impl: Sized {
    fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattReadClientCharacteristicConfigurationDescriptorResult2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattReadClientCharacteristicConfigurationDescriptorResult2";
}
#[cfg(feature = "implement_exclusive")]
impl IGattReadClientCharacteristicConfigurationDescriptorResult2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattReadClientCharacteristicConfigurationDescriptorResult2Impl, const OFFSET: isize>() -> IGattReadClientCharacteristicConfigurationDescriptorResult2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattReadClientCharacteristicConfigurationDescriptorResult2>, ::windows::core::GetTrustLevel, ProtocolError::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattReadRequestImpl: Sized {
    fn Offset(&self) -> ::windows::core::Result<u32>;
    fn Length(&self) -> ::windows::core::Result<u32>;
    fn State(&self) -> ::windows::core::Result<GattRequestState>;
    fn StateChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattReadRequest, GattRequestStateChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RespondWithValue(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn RespondWithProtocolError(&self, protocolerror: u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattReadRequest {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattReadRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IGattReadRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattReadRequestImpl, const OFFSET: isize>() -> IGattReadRequestVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattReadRequest>, ::windows::core::GetTrustLevel, Offset::<Impl, OFFSET>, Length::<Impl, OFFSET>, State::<Impl, OFFSET>, StateChanged::<Impl, OFFSET>, RemoveStateChanged::<Impl, OFFSET>, RespondWithValue::<Impl, OFFSET>, RespondWithProtocolError::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattReadRequestedEventArgsImpl: Sized {
    fn Session(&self) -> ::windows::core::Result<GattSession>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
    fn GetRequestAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattReadRequest>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattReadRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattReadRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IGattReadRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattReadRequestedEventArgsImpl, const OFFSET: isize>() -> IGattReadRequestedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattReadRequestedEventArgs>, ::windows::core::GetTrustLevel, Session::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>, GetRequestAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattReadResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus>;
    fn Value(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattReadResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattReadResult";
}
#[cfg(feature = "implement_exclusive")]
impl IGattReadResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattReadResultImpl, const OFFSET: isize>() -> IGattReadResultVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattReadResult>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>, Value::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattReadResult2Impl: Sized {
    fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattReadResult2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattReadResult2";
}
#[cfg(feature = "implement_exclusive")]
impl IGattReadResult2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattReadResult2Impl, const OFFSET: isize>() -> IGattReadResult2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattReadResult2>, ::windows::core::GetTrustLevel, ProtocolError::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattReliableWriteTransactionImpl: Sized {
    fn WriteValue(&self, characteristic: &::core::option::Option<GattCharacteristic>, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn CommitAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattCommunicationStatus>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattReliableWriteTransaction {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattReliableWriteTransaction";
}
#[cfg(feature = "implement_exclusive")]
impl IGattReliableWriteTransactionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattReliableWriteTransactionImpl, const OFFSET: isize>() -> IGattReliableWriteTransactionVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattReliableWriteTransaction>, ::windows::core::GetTrustLevel, WriteValue::<Impl, OFFSET>, CommitAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattReliableWriteTransaction2Impl: Sized {
    fn CommitWithResultAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattReliableWriteTransaction2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattReliableWriteTransaction2";
}
#[cfg(feature = "implement_exclusive")]
impl IGattReliableWriteTransaction2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattReliableWriteTransaction2Impl, const OFFSET: isize>() -> IGattReliableWriteTransaction2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattReliableWriteTransaction2>, ::windows::core::GetTrustLevel, CommitWithResultAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattRequestStateChangedEventArgsImpl: Sized {
    fn State(&self) -> ::windows::core::Result<GattRequestState>;
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattRequestStateChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattRequestStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IGattRequestStateChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattRequestStateChangedEventArgsImpl, const OFFSET: isize>() -> IGattRequestStateChangedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattRequestStateChangedEventArgs>, ::windows::core::GetTrustLevel, State::<Impl, OFFSET>, Error::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceProviderImpl: Sized {
    fn Service(&self) -> ::windows::core::Result<GattLocalService>;
    fn AdvertisementStatus(&self) -> ::windows::core::Result<GattServiceProviderAdvertisementStatus>;
    fn AdvertisementStatusChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattServiceProvider, GattServiceProviderAdvertisementStatusChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdvertisementStatusChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StartAdvertising(&self) -> ::windows::core::Result<()>;
    fn StartAdvertisingWithParameters(&self, parameters: &::core::option::Option<GattServiceProviderAdvertisingParameters>) -> ::windows::core::Result<()>;
    fn StopAdvertising(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattServiceProvider {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattServiceProvider";
}
#[cfg(feature = "implement_exclusive")]
impl IGattServiceProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceProviderImpl, const OFFSET: isize>() -> IGattServiceProviderVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGattServiceProvider>,
            ::windows::core::GetTrustLevel,
            Service::<Impl, OFFSET>,
            AdvertisementStatus::<Impl, OFFSET>,
            AdvertisementStatusChanged::<Impl, OFFSET>,
            RemoveAdvertisementStatusChanged::<Impl, OFFSET>,
            StartAdvertising::<Impl, OFFSET>,
            StartAdvertisingWithParameters::<Impl, OFFSET>,
            StopAdvertising::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceProviderAdvertisementStatusChangedEventArgsImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
    fn Status(&self) -> ::windows::core::Result<GattServiceProviderAdvertisementStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattServiceProviderAdvertisementStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattServiceProviderAdvertisementStatusChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IGattServiceProviderAdvertisementStatusChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceProviderAdvertisementStatusChangedEventArgsImpl, const OFFSET: isize>() -> IGattServiceProviderAdvertisementStatusChangedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattServiceProviderAdvertisementStatusChangedEventArgs>, ::windows::core::GetTrustLevel, Error::<Impl, OFFSET>, Status::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceProviderAdvertisingParametersImpl: Sized {
    fn SetIsConnectable(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsConnectable(&self) -> ::windows::core::Result<bool>;
    fn SetIsDiscoverable(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsDiscoverable(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattServiceProviderAdvertisingParameters {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattServiceProviderAdvertisingParameters";
}
#[cfg(feature = "implement_exclusive")]
impl IGattServiceProviderAdvertisingParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceProviderAdvertisingParametersImpl, const OFFSET: isize>() -> IGattServiceProviderAdvertisingParametersVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattServiceProviderAdvertisingParameters>, ::windows::core::GetTrustLevel, SetIsConnectable::<Impl, OFFSET>, IsConnectable::<Impl, OFFSET>, SetIsDiscoverable::<Impl, OFFSET>, IsDiscoverable::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceProviderAdvertisingParameters2Impl: Sized {
    fn SetServiceData(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn ServiceData(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattServiceProviderAdvertisingParameters2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattServiceProviderAdvertisingParameters2";
}
#[cfg(feature = "implement_exclusive")]
impl IGattServiceProviderAdvertisingParameters2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceProviderAdvertisingParameters2Impl, const OFFSET: isize>() -> IGattServiceProviderAdvertisingParameters2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattServiceProviderAdvertisingParameters2>, ::windows::core::GetTrustLevel, SetServiceData::<Impl, OFFSET>, ServiceData::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceProviderResultImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
    fn ServiceProvider(&self) -> ::windows::core::Result<GattServiceProvider>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattServiceProviderResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattServiceProviderResult";
}
#[cfg(feature = "implement_exclusive")]
impl IGattServiceProviderResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceProviderResultImpl, const OFFSET: isize>() -> IGattServiceProviderResultVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattServiceProviderResult>, ::windows::core::GetTrustLevel, Error::<Impl, OFFSET>, ServiceProvider::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceProviderStaticsImpl: Sized {
    fn CreateAsync(&self, serviceuuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattServiceProviderResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattServiceProviderStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattServiceProviderStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGattServiceProviderStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceProviderStaticsImpl, const OFFSET: isize>() -> IGattServiceProviderStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattServiceProviderStatics>, ::windows::core::GetTrustLevel, CreateAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceUuidsStaticsImpl: Sized {
    fn Battery(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn BloodPressure(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CyclingSpeedAndCadence(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GenericAccess(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GenericAttribute(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Glucose(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn HealthThermometer(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn HeartRate(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RunningSpeedAndCadence(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattServiceUuidsStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattServiceUuidsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGattServiceUuidsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceUuidsStaticsImpl, const OFFSET: isize>() -> IGattServiceUuidsStaticsVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGattServiceUuidsStatics>,
            ::windows::core::GetTrustLevel,
            Battery::<Impl, OFFSET>,
            BloodPressure::<Impl, OFFSET>,
            CyclingSpeedAndCadence::<Impl, OFFSET>,
            GenericAccess::<Impl, OFFSET>,
            GenericAttribute::<Impl, OFFSET>,
            Glucose::<Impl, OFFSET>,
            HealthThermometer::<Impl, OFFSET>,
            HeartRate::<Impl, OFFSET>,
            RunningSpeedAndCadence::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceUuidsStatics2Impl: Sized {
    fn AlertNotification(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CurrentTime(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CyclingPower(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn DeviceInformation(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn HumanInterfaceDevice(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ImmediateAlert(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn LinkLoss(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn LocationAndNavigation(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn NextDstChange(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn PhoneAlertStatus(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ReferenceTimeUpdate(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ScanParameters(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn TxPower(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattServiceUuidsStatics2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattServiceUuidsStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IGattServiceUuidsStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattServiceUuidsStatics2Impl, const OFFSET: isize>() -> IGattServiceUuidsStatics2Vtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGattServiceUuidsStatics2>,
            ::windows::core::GetTrustLevel,
            AlertNotification::<Impl, OFFSET>,
            CurrentTime::<Impl, OFFSET>,
            CyclingPower::<Impl, OFFSET>,
            DeviceInformation::<Impl, OFFSET>,
            HumanInterfaceDevice::<Impl, OFFSET>,
            ImmediateAlert::<Impl, OFFSET>,
            LinkLoss::<Impl, OFFSET>,
            LocationAndNavigation::<Impl, OFFSET>,
            NextDstChange::<Impl, OFFSET>,
            PhoneAlertStatus::<Impl, OFFSET>,
            ReferenceTimeUpdate::<Impl, OFFSET>,
            ScanParameters::<Impl, OFFSET>,
            TxPower::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattSessionImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<super::BluetoothDeviceId>;
    fn CanMaintainConnection(&self) -> ::windows::core::Result<bool>;
    fn SetMaintainConnection(&self, value: bool) -> ::windows::core::Result<()>;
    fn MaintainConnection(&self) -> ::windows::core::Result<bool>;
    fn MaxPduSize(&self) -> ::windows::core::Result<u16>;
    fn SessionStatus(&self) -> ::windows::core::Result<GattSessionStatus>;
    fn MaxPduSizeChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMaxPduSizeChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SessionStatusChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattSession, GattSessionStatusChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSessionStatusChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattSession {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattSession";
}
#[cfg(feature = "implement_exclusive")]
impl IGattSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattSessionImpl, const OFFSET: isize>() -> IGattSessionVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGattSession>,
            ::windows::core::GetTrustLevel,
            DeviceId::<Impl, OFFSET>,
            CanMaintainConnection::<Impl, OFFSET>,
            SetMaintainConnection::<Impl, OFFSET>,
            MaintainConnection::<Impl, OFFSET>,
            MaxPduSize::<Impl, OFFSET>,
            SessionStatus::<Impl, OFFSET>,
            MaxPduSizeChanged::<Impl, OFFSET>,
            RemoveMaxPduSizeChanged::<Impl, OFFSET>,
            SessionStatusChanged::<Impl, OFFSET>,
            RemoveSessionStatusChanged::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattSessionStaticsImpl: Sized {
    fn FromDeviceIdAsync(&self, deviceid: &::core::option::Option<super::BluetoothDeviceId>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattSession>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattSessionStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattSessionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGattSessionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattSessionStaticsImpl, const OFFSET: isize>() -> IGattSessionStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattSessionStatics>, ::windows::core::GetTrustLevel, FromDeviceIdAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattSessionStatusChangedEventArgsImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
    fn Status(&self) -> ::windows::core::Result<GattSessionStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattSessionStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattSessionStatusChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IGattSessionStatusChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattSessionStatusChangedEventArgsImpl, const OFFSET: isize>() -> IGattSessionStatusChangedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattSessionStatusChangedEventArgs>, ::windows::core::GetTrustLevel, Error::<Impl, OFFSET>, Status::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattSubscribedClientImpl: Sized {
    fn Session(&self) -> ::windows::core::Result<GattSession>;
    fn MaxNotificationSize(&self) -> ::windows::core::Result<u16>;
    fn MaxNotificationSizeChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattSubscribedClient, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMaxNotificationSizeChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattSubscribedClient {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattSubscribedClient";
}
#[cfg(feature = "implement_exclusive")]
impl IGattSubscribedClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattSubscribedClientImpl, const OFFSET: isize>() -> IGattSubscribedClientVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattSubscribedClient>, ::windows::core::GetTrustLevel, Session::<Impl, OFFSET>, MaxNotificationSize::<Impl, OFFSET>, MaxNotificationSizeChanged::<Impl, OFFSET>, RemoveMaxNotificationSizeChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattValueChangedEventArgsImpl: Sized {
    fn CharacteristicValue(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattValueChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattValueChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IGattValueChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattValueChangedEventArgsImpl, const OFFSET: isize>() -> IGattValueChangedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattValueChangedEventArgs>, ::windows::core::GetTrustLevel, CharacteristicValue::<Impl, OFFSET>, Timestamp::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattWriteRequestImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn Offset(&self) -> ::windows::core::Result<u32>;
    fn Option(&self) -> ::windows::core::Result<GattWriteOption>;
    fn State(&self) -> ::windows::core::Result<GattRequestState>;
    fn StateChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GattWriteRequest, GattRequestStateChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Respond(&self) -> ::windows::core::Result<()>;
    fn RespondWithProtocolError(&self, protocolerror: u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattWriteRequest {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattWriteRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IGattWriteRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattWriteRequestImpl, const OFFSET: isize>() -> IGattWriteRequestVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IGattWriteRequest>,
            ::windows::core::GetTrustLevel,
            Value::<Impl, OFFSET>,
            Offset::<Impl, OFFSET>,
            Option::<Impl, OFFSET>,
            State::<Impl, OFFSET>,
            StateChanged::<Impl, OFFSET>,
            RemoveStateChanged::<Impl, OFFSET>,
            Respond::<Impl, OFFSET>,
            RespondWithProtocolError::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattWriteRequestedEventArgsImpl: Sized {
    fn Session(&self) -> ::windows::core::Result<GattSession>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
    fn GetRequestAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<GattWriteRequest>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattWriteRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattWriteRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IGattWriteRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattWriteRequestedEventArgsImpl, const OFFSET: isize>() -> IGattWriteRequestedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattWriteRequestedEventArgs>, ::windows::core::GetTrustLevel, Session::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>, GetRequestAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattWriteResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<GattCommunicationStatus>;
    fn ProtocolError(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u8>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGattWriteResult {
    const NAME: &'static str = "Windows.Devices.Bluetooth.GenericAttributeProfile.IGattWriteResult";
}
#[cfg(feature = "implement_exclusive")]
impl IGattWriteResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGattWriteResultImpl, const OFFSET: isize>() -> IGattWriteResultVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGattWriteResult>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>, ProtocolError::<Impl, OFFSET>)
    }
}
