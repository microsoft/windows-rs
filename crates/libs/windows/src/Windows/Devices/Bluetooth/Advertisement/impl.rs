#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IBluetoothLEAdvertisementImpl: Sized {
    fn Flags(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<BluetoothLEAdvertisementFlags>>;
    fn SetFlags(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<BluetoothLEAdvertisementFlags>>) -> ::windows::core::Result<()>;
    fn LocalName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLocalName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ServiceUuids(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::GUID>>;
    fn ManufacturerData(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<BluetoothLEManufacturerData>>;
    fn DataSections(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<BluetoothLEAdvertisementDataSection>>;
    fn GetManufacturerDataByCompanyId(&self, companyid: u16) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<BluetoothLEManufacturerData>>;
    fn GetSectionsByType(&self, r#type: u8) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<BluetoothLEAdvertisementDataSection>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisement {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisement";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBluetoothLEAdvertisementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementVtbl {
        unsafe extern "system" fn Flags<Impl: IBluetoothLEAdvertisementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Flags() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Impl: IBluetoothLEAdvertisementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFlags(&*(&value as *const <super::super::super::Foundation::IReference<BluetoothLEAdvertisementFlags> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<BluetoothLEAdvertisementFlags> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LocalName<Impl: IBluetoothLEAdvertisementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalName<Impl: IBluetoothLEAdvertisementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ServiceUuids<Impl: IBluetoothLEAdvertisementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceUuids() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManufacturerData<Impl: IBluetoothLEAdvertisementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManufacturerData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataSections<Impl: IBluetoothLEAdvertisementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataSections() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetManufacturerDataByCompanyId<Impl: IBluetoothLEAdvertisementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, companyid: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetManufacturerDataByCompanyId(companyid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSectionsByType<Impl: IBluetoothLEAdvertisementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSectionsByType(r#type) {
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
            ::windows::core::GetRuntimeClassName::<IBluetoothLEAdvertisement>,
            ::windows::core::GetTrustLevel,
            Flags::<Impl, IMPL_OFFSET>,
            SetFlags::<Impl, IMPL_OFFSET>,
            LocalName::<Impl, IMPL_OFFSET>,
            SetLocalName::<Impl, IMPL_OFFSET>,
            ServiceUuids::<Impl, IMPL_OFFSET>,
            ManufacturerData::<Impl, IMPL_OFFSET>,
            DataSections::<Impl, IMPL_OFFSET>,
            GetManufacturerDataByCompanyId::<Impl, IMPL_OFFSET>,
            GetSectionsByType::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAdvertisement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IBluetoothLEAdvertisementBytePatternImpl: Sized {
    fn DataType(&self) -> ::windows::core::Result<u8>;
    fn SetDataType(&self, value: u8) -> ::windows::core::Result<()>;
    fn Offset(&self) -> ::windows::core::Result<i16>;
    fn SetOffset(&self, value: i16) -> ::windows::core::Result<()>;
    fn Data(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetData(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementBytePattern {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementBytePattern";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IBluetoothLEAdvertisementBytePatternVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementBytePatternImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementBytePatternVtbl {
        unsafe extern "system" fn DataType<Impl: IBluetoothLEAdvertisementBytePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataType<Impl: IBluetoothLEAdvertisementBytePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataType(value).into()
        }
        unsafe extern "system" fn Offset<Impl: IBluetoothLEAdvertisementBytePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOffset<Impl: IBluetoothLEAdvertisementBytePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOffset(value).into()
        }
        unsafe extern "system" fn Data<Impl: IBluetoothLEAdvertisementBytePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetData<Impl: IBluetoothLEAdvertisementBytePatternImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(&*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBluetoothLEAdvertisementBytePattern>, ::windows::core::GetTrustLevel, DataType::<Impl, IMPL_OFFSET>, SetDataType::<Impl, IMPL_OFFSET>, Offset::<Impl, IMPL_OFFSET>, SetOffset::<Impl, IMPL_OFFSET>, Data::<Impl, IMPL_OFFSET>, SetData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAdvertisementBytePattern as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IBluetoothLEAdvertisementBytePatternFactoryImpl: Sized {
    fn Create(&self, datatype: u8, offset: i16, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<BluetoothLEAdvertisementBytePattern>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementBytePatternFactory {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementBytePatternFactory";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IBluetoothLEAdvertisementBytePatternFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementBytePatternFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementBytePatternFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IBluetoothLEAdvertisementBytePatternFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatype: u8, offset: i16, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(datatype, offset, &*(&data as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBluetoothLEAdvertisementBytePatternFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAdvertisementBytePatternFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IBluetoothLEAdvertisementDataSectionImpl: Sized {
    fn DataType(&self) -> ::windows::core::Result<u8>;
    fn SetDataType(&self, value: u8) -> ::windows::core::Result<()>;
    fn Data(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetData(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementDataSection {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementDataSection";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IBluetoothLEAdvertisementDataSectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementDataSectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementDataSectionVtbl {
        unsafe extern "system" fn DataType<Impl: IBluetoothLEAdvertisementDataSectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataType<Impl: IBluetoothLEAdvertisementDataSectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataType(value).into()
        }
        unsafe extern "system" fn Data<Impl: IBluetoothLEAdvertisementDataSectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetData<Impl: IBluetoothLEAdvertisementDataSectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(&*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBluetoothLEAdvertisementDataSection>, ::windows::core::GetTrustLevel, DataType::<Impl, IMPL_OFFSET>, SetDataType::<Impl, IMPL_OFFSET>, Data::<Impl, IMPL_OFFSET>, SetData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAdvertisementDataSection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IBluetoothLEAdvertisementDataSectionFactoryImpl: Sized {
    fn Create(&self, datatype: u8, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<BluetoothLEAdvertisementDataSection>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementDataSectionFactory {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementDataSectionFactory";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IBluetoothLEAdvertisementDataSectionFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementDataSectionFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementDataSectionFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IBluetoothLEAdvertisementDataSectionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatype: u8, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(datatype, &*(&data as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBluetoothLEAdvertisementDataSectionFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAdvertisementDataSectionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementDataTypesStaticsImpl: Sized {
    fn Flags(&self) -> ::windows::core::Result<u8>;
    fn IncompleteService16BitUuids(&self) -> ::windows::core::Result<u8>;
    fn CompleteService16BitUuids(&self) -> ::windows::core::Result<u8>;
    fn IncompleteService32BitUuids(&self) -> ::windows::core::Result<u8>;
    fn CompleteService32BitUuids(&self) -> ::windows::core::Result<u8>;
    fn IncompleteService128BitUuids(&self) -> ::windows::core::Result<u8>;
    fn CompleteService128BitUuids(&self) -> ::windows::core::Result<u8>;
    fn ShortenedLocalName(&self) -> ::windows::core::Result<u8>;
    fn CompleteLocalName(&self) -> ::windows::core::Result<u8>;
    fn TxPowerLevel(&self) -> ::windows::core::Result<u8>;
    fn SlaveConnectionIntervalRange(&self) -> ::windows::core::Result<u8>;
    fn ServiceSolicitation16BitUuids(&self) -> ::windows::core::Result<u8>;
    fn ServiceSolicitation32BitUuids(&self) -> ::windows::core::Result<u8>;
    fn ServiceSolicitation128BitUuids(&self) -> ::windows::core::Result<u8>;
    fn ServiceData16BitUuids(&self) -> ::windows::core::Result<u8>;
    fn ServiceData32BitUuids(&self) -> ::windows::core::Result<u8>;
    fn ServiceData128BitUuids(&self) -> ::windows::core::Result<u8>;
    fn PublicTargetAddress(&self) -> ::windows::core::Result<u8>;
    fn RandomTargetAddress(&self) -> ::windows::core::Result<u8>;
    fn Appearance(&self) -> ::windows::core::Result<u8>;
    fn AdvertisingInterval(&self) -> ::windows::core::Result<u8>;
    fn ManufacturerSpecificData(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementDataTypesStatics {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementDataTypesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEAdvertisementDataTypesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementDataTypesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementDataTypesStaticsVtbl {
        unsafe extern "system" fn Flags<Impl: IBluetoothLEAdvertisementDataTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Flags() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncompleteService16BitUuids<Impl: IBluetoothLEAdvertisementDataTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncompleteService16BitUuids() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompleteService16BitUuids<Impl: IBluetoothLEAdvertisementDataTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompleteService16BitUuids() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncompleteService32BitUuids<Impl: IBluetoothLEAdvertisementDataTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncompleteService32BitUuids() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompleteService32BitUuids<Impl: IBluetoothLEAdvertisementDataTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompleteService32BitUuids() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncompleteService128BitUuids<Impl: IBluetoothLEAdvertisementDataTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncompleteService128BitUuids() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompleteService128BitUuids<Impl: IBluetoothLEAdvertisementDataTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompleteService128BitUuids() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShortenedLocalName<Impl: IBluetoothLEAdvertisementDataTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShortenedLocalName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompleteLocalName<Impl: IBluetoothLEAdvertisementDataTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompleteLocalName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TxPowerLevel<Impl: IBluetoothLEAdvertisementDataTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SlaveConnectionIntervalRange<Impl: IBluetoothLEAdvertisementDataTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SlaveConnectionIntervalRange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceSolicitation16BitUuids<Impl: IBluetoothLEAdvertisementDataTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceSolicitation16BitUuids() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceSolicitation32BitUuids<Impl: IBluetoothLEAdvertisementDataTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceSolicitation32BitUuids() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceSolicitation128BitUuids<Impl: IBluetoothLEAdvertisementDataTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceSolicitation128BitUuids() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceData16BitUuids<Impl: IBluetoothLEAdvertisementDataTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceData16BitUuids() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceData32BitUuids<Impl: IBluetoothLEAdvertisementDataTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceData32BitUuids() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceData128BitUuids<Impl: IBluetoothLEAdvertisementDataTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceData128BitUuids() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublicTargetAddress<Impl: IBluetoothLEAdvertisementDataTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublicTargetAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RandomTargetAddress<Impl: IBluetoothLEAdvertisementDataTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RandomTargetAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Appearance<Impl: IBluetoothLEAdvertisementDataTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AdvertisingInterval<Impl: IBluetoothLEAdvertisementDataTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdvertisingInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManufacturerSpecificData<Impl: IBluetoothLEAdvertisementDataTypesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManufacturerSpecificData() {
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
            ::windows::core::GetRuntimeClassName::<IBluetoothLEAdvertisementDataTypesStatics>,
            ::windows::core::GetTrustLevel,
            Flags::<Impl, IMPL_OFFSET>,
            IncompleteService16BitUuids::<Impl, IMPL_OFFSET>,
            CompleteService16BitUuids::<Impl, IMPL_OFFSET>,
            IncompleteService32BitUuids::<Impl, IMPL_OFFSET>,
            CompleteService32BitUuids::<Impl, IMPL_OFFSET>,
            IncompleteService128BitUuids::<Impl, IMPL_OFFSET>,
            CompleteService128BitUuids::<Impl, IMPL_OFFSET>,
            ShortenedLocalName::<Impl, IMPL_OFFSET>,
            CompleteLocalName::<Impl, IMPL_OFFSET>,
            TxPowerLevel::<Impl, IMPL_OFFSET>,
            SlaveConnectionIntervalRange::<Impl, IMPL_OFFSET>,
            ServiceSolicitation16BitUuids::<Impl, IMPL_OFFSET>,
            ServiceSolicitation32BitUuids::<Impl, IMPL_OFFSET>,
            ServiceSolicitation128BitUuids::<Impl, IMPL_OFFSET>,
            ServiceData16BitUuids::<Impl, IMPL_OFFSET>,
            ServiceData32BitUuids::<Impl, IMPL_OFFSET>,
            ServiceData128BitUuids::<Impl, IMPL_OFFSET>,
            PublicTargetAddress::<Impl, IMPL_OFFSET>,
            RandomTargetAddress::<Impl, IMPL_OFFSET>,
            Appearance::<Impl, IMPL_OFFSET>,
            AdvertisingInterval::<Impl, IMPL_OFFSET>,
            ManufacturerSpecificData::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAdvertisementDataTypesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IBluetoothLEAdvertisementFilterImpl: Sized {
    fn Advertisement(&self) -> ::windows::core::Result<BluetoothLEAdvertisement>;
    fn SetAdvertisement(&self, value: &::core::option::Option<BluetoothLEAdvertisement>) -> ::windows::core::Result<()>;
    fn BytePatterns(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<BluetoothLEAdvertisementBytePattern>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementFilter {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementFilter";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IBluetoothLEAdvertisementFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementFilterVtbl {
        unsafe extern "system" fn Advertisement<Impl: IBluetoothLEAdvertisementFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Advertisement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAdvertisement<Impl: IBluetoothLEAdvertisementFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAdvertisement(&*(&value as *const <BluetoothLEAdvertisement as ::windows::core::Abi>::Abi as *const <BluetoothLEAdvertisement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BytePatterns<Impl: IBluetoothLEAdvertisementFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BytePatterns() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBluetoothLEAdvertisementFilter>, ::windows::core::GetTrustLevel, Advertisement::<Impl, IMPL_OFFSET>, SetAdvertisement::<Impl, IMPL_OFFSET>, BytePatterns::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAdvertisementFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBluetoothLEAdvertisementPublisherImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<BluetoothLEAdvertisementPublisherStatus>;
    fn Advertisement(&self) -> ::windows::core::Result<BluetoothLEAdvertisement>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn StatusChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<BluetoothLEAdvertisementPublisher, BluetoothLEAdvertisementPublisherStatusChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementPublisher {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisher";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBluetoothLEAdvertisementPublisherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementPublisherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementPublisherVtbl {
        unsafe extern "system" fn Status<Impl: IBluetoothLEAdvertisementPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BluetoothLEAdvertisementPublisherStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Advertisement<Impl: IBluetoothLEAdvertisementPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Advertisement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IBluetoothLEAdvertisementPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IBluetoothLEAdvertisementPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn StatusChanged<Impl: IBluetoothLEAdvertisementPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<BluetoothLEAdvertisementPublisher, BluetoothLEAdvertisementPublisherStatusChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<BluetoothLEAdvertisementPublisher, BluetoothLEAdvertisementPublisherStatusChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStatusChanged<Impl: IBluetoothLEAdvertisementPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStatusChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IBluetoothLEAdvertisementPublisher>,
            ::windows::core::GetTrustLevel,
            Status::<Impl, IMPL_OFFSET>,
            Advertisement::<Impl, IMPL_OFFSET>,
            Start::<Impl, IMPL_OFFSET>,
            Stop::<Impl, IMPL_OFFSET>,
            StatusChanged::<Impl, IMPL_OFFSET>,
            RemoveStatusChanged::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAdvertisementPublisher as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBluetoothLEAdvertisementPublisher2Impl: Sized {
    fn PreferredTransmitPowerLevelInDBm(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i16>>;
    fn SetPreferredTransmitPowerLevelInDBm(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<i16>>) -> ::windows::core::Result<()>;
    fn UseExtendedAdvertisement(&self) -> ::windows::core::Result<bool>;
    fn SetUseExtendedAdvertisement(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsAnonymous(&self) -> ::windows::core::Result<bool>;
    fn SetIsAnonymous(&self, value: bool) -> ::windows::core::Result<()>;
    fn IncludeTransmitPowerLevel(&self) -> ::windows::core::Result<bool>;
    fn SetIncludeTransmitPowerLevel(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementPublisher2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisher2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBluetoothLEAdvertisementPublisher2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementPublisher2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementPublisher2Vtbl {
        unsafe extern "system" fn PreferredTransmitPowerLevelInDBm<Impl: IBluetoothLEAdvertisementPublisher2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredTransmitPowerLevelInDBm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredTransmitPowerLevelInDBm<Impl: IBluetoothLEAdvertisementPublisher2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredTransmitPowerLevelInDBm(&*(&value as *const <super::super::super::Foundation::IReference<i16> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<i16> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UseExtendedAdvertisement<Impl: IBluetoothLEAdvertisementPublisher2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseExtendedAdvertisement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseExtendedAdvertisement<Impl: IBluetoothLEAdvertisementPublisher2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUseExtendedAdvertisement(value).into()
        }
        unsafe extern "system" fn IsAnonymous<Impl: IBluetoothLEAdvertisementPublisher2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAnonymous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAnonymous<Impl: IBluetoothLEAdvertisementPublisher2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsAnonymous(value).into()
        }
        unsafe extern "system" fn IncludeTransmitPowerLevel<Impl: IBluetoothLEAdvertisementPublisher2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncludeTransmitPowerLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncludeTransmitPowerLevel<Impl: IBluetoothLEAdvertisementPublisher2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncludeTransmitPowerLevel(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IBluetoothLEAdvertisementPublisher2>,
            ::windows::core::GetTrustLevel,
            PreferredTransmitPowerLevelInDBm::<Impl, IMPL_OFFSET>,
            SetPreferredTransmitPowerLevelInDBm::<Impl, IMPL_OFFSET>,
            UseExtendedAdvertisement::<Impl, IMPL_OFFSET>,
            SetUseExtendedAdvertisement::<Impl, IMPL_OFFSET>,
            IsAnonymous::<Impl, IMPL_OFFSET>,
            SetIsAnonymous::<Impl, IMPL_OFFSET>,
            IncludeTransmitPowerLevel::<Impl, IMPL_OFFSET>,
            SetIncludeTransmitPowerLevel::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAdvertisementPublisher2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementPublisherFactoryImpl: Sized {
    fn Create(&self, advertisement: &::core::option::Option<BluetoothLEAdvertisement>) -> ::windows::core::Result<BluetoothLEAdvertisementPublisher>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementPublisherFactory {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisherFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEAdvertisementPublisherFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementPublisherFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementPublisherFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IBluetoothLEAdvertisementPublisherFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, advertisement: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&advertisement as *const <BluetoothLEAdvertisement as ::windows::core::Abi>::Abi as *const <BluetoothLEAdvertisement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBluetoothLEAdvertisementPublisherFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAdvertisementPublisherFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementPublisherStatusChangedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<BluetoothLEAdvertisementPublisherStatus>;
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementPublisherStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisherStatusChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEAdvertisementPublisherStatusChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementPublisherStatusChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementPublisherStatusChangedEventArgsVtbl {
        unsafe extern "system" fn Status<Impl: IBluetoothLEAdvertisementPublisherStatusChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BluetoothLEAdvertisementPublisherStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Error<Impl: IBluetoothLEAdvertisementPublisherStatusChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBluetoothLEAdvertisementPublisherStatusChangedEventArgs>, ::windows::core::GetTrustLevel, Status::<Impl, IMPL_OFFSET>, Error::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAdvertisementPublisherStatusChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2Impl: Sized {
    fn SelectedTransmitPowerLevelInDBm(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i16>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2Vtbl {
        unsafe extern "system" fn SelectedTransmitPowerLevelInDBm<Impl: IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedTransmitPowerLevelInDBm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2>, ::windows::core::GetTrustLevel, SelectedTransmitPowerLevelInDBm::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAdvertisementPublisherStatusChangedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBluetoothLEAdvertisementReceivedEventArgsImpl: Sized {
    fn RawSignalStrengthInDBm(&self) -> ::windows::core::Result<i16>;
    fn BluetoothAddress(&self) -> ::windows::core::Result<u64>;
    fn AdvertisementType(&self) -> ::windows::core::Result<BluetoothLEAdvertisementType>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn Advertisement(&self) -> ::windows::core::Result<BluetoothLEAdvertisement>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementReceivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBluetoothLEAdvertisementReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementReceivedEventArgsVtbl {
        unsafe extern "system" fn RawSignalStrengthInDBm<Impl: IBluetoothLEAdvertisementReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawSignalStrengthInDBm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BluetoothAddress<Impl: IBluetoothLEAdvertisementReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BluetoothAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdvertisementType<Impl: IBluetoothLEAdvertisementReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BluetoothLEAdvertisementType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdvertisementType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: IBluetoothLEAdvertisementReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Advertisement<Impl: IBluetoothLEAdvertisementReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Advertisement() {
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
            ::windows::core::GetRuntimeClassName::<IBluetoothLEAdvertisementReceivedEventArgs>,
            ::windows::core::GetTrustLevel,
            RawSignalStrengthInDBm::<Impl, IMPL_OFFSET>,
            BluetoothAddress::<Impl, IMPL_OFFSET>,
            AdvertisementType::<Impl, IMPL_OFFSET>,
            Timestamp::<Impl, IMPL_OFFSET>,
            Advertisement::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAdvertisementReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBluetoothLEAdvertisementReceivedEventArgs2Impl: Sized {
    fn BluetoothAddressType(&self) -> ::windows::core::Result<super::BluetoothAddressType>;
    fn TransmitPowerLevelInDBm(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i16>>;
    fn IsAnonymous(&self) -> ::windows::core::Result<bool>;
    fn IsConnectable(&self) -> ::windows::core::Result<bool>;
    fn IsScannable(&self) -> ::windows::core::Result<bool>;
    fn IsDirected(&self) -> ::windows::core::Result<bool>;
    fn IsScanResponse(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementReceivedEventArgs2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementReceivedEventArgs2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBluetoothLEAdvertisementReceivedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementReceivedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementReceivedEventArgs2Vtbl {
        unsafe extern "system" fn BluetoothAddressType<Impl: IBluetoothLEAdvertisementReceivedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothAddressType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BluetoothAddressType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransmitPowerLevelInDBm<Impl: IBluetoothLEAdvertisementReceivedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransmitPowerLevelInDBm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAnonymous<Impl: IBluetoothLEAdvertisementReceivedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAnonymous() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnectable<Impl: IBluetoothLEAdvertisementReceivedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsScannable<Impl: IBluetoothLEAdvertisementReceivedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsScannable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDirected<Impl: IBluetoothLEAdvertisementReceivedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDirected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsScanResponse<Impl: IBluetoothLEAdvertisementReceivedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsScanResponse() {
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
            ::windows::core::GetRuntimeClassName::<IBluetoothLEAdvertisementReceivedEventArgs2>,
            ::windows::core::GetTrustLevel,
            BluetoothAddressType::<Impl, IMPL_OFFSET>,
            TransmitPowerLevelInDBm::<Impl, IMPL_OFFSET>,
            IsAnonymous::<Impl, IMPL_OFFSET>,
            IsConnectable::<Impl, IMPL_OFFSET>,
            IsScannable::<Impl, IMPL_OFFSET>,
            IsDirected::<Impl, IMPL_OFFSET>,
            IsScanResponse::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAdvertisementReceivedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBluetoothLEAdvertisementWatcherImpl: Sized {
    fn MinSamplingInterval(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn MaxSamplingInterval(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn MinOutOfRangeTimeout(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn MaxOutOfRangeTimeout(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Status(&self) -> ::windows::core::Result<BluetoothLEAdvertisementWatcherStatus>;
    fn ScanningMode(&self) -> ::windows::core::Result<BluetoothLEScanningMode>;
    fn SetScanningMode(&self, value: BluetoothLEScanningMode) -> ::windows::core::Result<()>;
    fn SignalStrengthFilter(&self) -> ::windows::core::Result<super::BluetoothSignalStrengthFilter>;
    fn SetSignalStrengthFilter(&self, value: &::core::option::Option<super::BluetoothSignalStrengthFilter>) -> ::windows::core::Result<()>;
    fn AdvertisementFilter(&self) -> ::windows::core::Result<BluetoothLEAdvertisementFilter>;
    fn SetAdvertisementFilter(&self, value: &::core::option::Option<BluetoothLEAdvertisementFilter>) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Received(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<BluetoothLEAdvertisementWatcher, BluetoothLEAdvertisementReceivedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveReceived(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<BluetoothLEAdvertisementWatcher, BluetoothLEAdvertisementWatcherStoppedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementWatcher {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcher";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBluetoothLEAdvertisementWatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementWatcherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementWatcherVtbl {
        unsafe extern "system" fn MinSamplingInterval<Impl: IBluetoothLEAdvertisementWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinSamplingInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxSamplingInterval<Impl: IBluetoothLEAdvertisementWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxSamplingInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinOutOfRangeTimeout<Impl: IBluetoothLEAdvertisementWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinOutOfRangeTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxOutOfRangeTimeout<Impl: IBluetoothLEAdvertisementWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxOutOfRangeTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IBluetoothLEAdvertisementWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BluetoothLEAdvertisementWatcherStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ScanningMode<Impl: IBluetoothLEAdvertisementWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BluetoothLEScanningMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScanningMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScanningMode<Impl: IBluetoothLEAdvertisementWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: BluetoothLEScanningMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScanningMode(value).into()
        }
        unsafe extern "system" fn SignalStrengthFilter<Impl: IBluetoothLEAdvertisementWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignalStrengthFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignalStrengthFilter<Impl: IBluetoothLEAdvertisementWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignalStrengthFilter(&*(&value as *const <super::BluetoothSignalStrengthFilter as ::windows::core::Abi>::Abi as *const <super::BluetoothSignalStrengthFilter as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AdvertisementFilter<Impl: IBluetoothLEAdvertisementWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdvertisementFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAdvertisementFilter<Impl: IBluetoothLEAdvertisementWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAdvertisementFilter(&*(&value as *const <BluetoothLEAdvertisementFilter as ::windows::core::Abi>::Abi as *const <BluetoothLEAdvertisementFilter as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Start<Impl: IBluetoothLEAdvertisementWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IBluetoothLEAdvertisementWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Received<Impl: IBluetoothLEAdvertisementWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Received(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<BluetoothLEAdvertisementWatcher, BluetoothLEAdvertisementReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<BluetoothLEAdvertisementWatcher, BluetoothLEAdvertisementReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveReceived<Impl: IBluetoothLEAdvertisementWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveReceived(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stopped<Impl: IBluetoothLEAdvertisementWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stopped(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<BluetoothLEAdvertisementWatcher, BluetoothLEAdvertisementWatcherStoppedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<BluetoothLEAdvertisementWatcher, BluetoothLEAdvertisementWatcherStoppedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStopped<Impl: IBluetoothLEAdvertisementWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStopped(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IBluetoothLEAdvertisementWatcher>,
            ::windows::core::GetTrustLevel,
            MinSamplingInterval::<Impl, IMPL_OFFSET>,
            MaxSamplingInterval::<Impl, IMPL_OFFSET>,
            MinOutOfRangeTimeout::<Impl, IMPL_OFFSET>,
            MaxOutOfRangeTimeout::<Impl, IMPL_OFFSET>,
            Status::<Impl, IMPL_OFFSET>,
            ScanningMode::<Impl, IMPL_OFFSET>,
            SetScanningMode::<Impl, IMPL_OFFSET>,
            SignalStrengthFilter::<Impl, IMPL_OFFSET>,
            SetSignalStrengthFilter::<Impl, IMPL_OFFSET>,
            AdvertisementFilter::<Impl, IMPL_OFFSET>,
            SetAdvertisementFilter::<Impl, IMPL_OFFSET>,
            Start::<Impl, IMPL_OFFSET>,
            Stop::<Impl, IMPL_OFFSET>,
            Received::<Impl, IMPL_OFFSET>,
            RemoveReceived::<Impl, IMPL_OFFSET>,
            Stopped::<Impl, IMPL_OFFSET>,
            RemoveStopped::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAdvertisementWatcher as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementWatcher2Impl: Sized {
    fn AllowExtendedAdvertisements(&self) -> ::windows::core::Result<bool>;
    fn SetAllowExtendedAdvertisements(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementWatcher2 {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcher2";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEAdvertisementWatcher2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementWatcher2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementWatcher2Vtbl {
        unsafe extern "system" fn AllowExtendedAdvertisements<Impl: IBluetoothLEAdvertisementWatcher2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowExtendedAdvertisements() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowExtendedAdvertisements<Impl: IBluetoothLEAdvertisementWatcher2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowExtendedAdvertisements(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBluetoothLEAdvertisementWatcher2>, ::windows::core::GetTrustLevel, AllowExtendedAdvertisements::<Impl, IMPL_OFFSET>, SetAllowExtendedAdvertisements::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAdvertisementWatcher2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementWatcherFactoryImpl: Sized {
    fn Create(&self, advertisementfilter: &::core::option::Option<BluetoothLEAdvertisementFilter>) -> ::windows::core::Result<BluetoothLEAdvertisementWatcher>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementWatcherFactory {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcherFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEAdvertisementWatcherFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementWatcherFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementWatcherFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IBluetoothLEAdvertisementWatcherFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, advertisementfilter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&advertisementfilter as *const <BluetoothLEAdvertisementFilter as ::windows::core::Abi>::Abi as *const <BluetoothLEAdvertisementFilter as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBluetoothLEAdvertisementWatcherFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAdvertisementWatcherFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementWatcherStoppedEventArgsImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<super::BluetoothError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBluetoothLEAdvertisementWatcherStoppedEventArgs {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcherStoppedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IBluetoothLEAdvertisementWatcherStoppedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEAdvertisementWatcherStoppedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEAdvertisementWatcherStoppedEventArgsVtbl {
        unsafe extern "system" fn Error<Impl: IBluetoothLEAdvertisementWatcherStoppedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::BluetoothError) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBluetoothLEAdvertisementWatcherStoppedEventArgs>, ::windows::core::GetTrustLevel, Error::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEAdvertisementWatcherStoppedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IBluetoothLEManufacturerDataImpl: Sized {
    fn CompanyId(&self) -> ::windows::core::Result<u16>;
    fn SetCompanyId(&self, value: u16) -> ::windows::core::Result<()>;
    fn Data(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetData(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEManufacturerData {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEManufacturerData";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IBluetoothLEManufacturerDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEManufacturerDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEManufacturerDataVtbl {
        unsafe extern "system" fn CompanyId<Impl: IBluetoothLEManufacturerDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompanyId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompanyId<Impl: IBluetoothLEManufacturerDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompanyId(value).into()
        }
        unsafe extern "system" fn Data<Impl: IBluetoothLEManufacturerDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetData<Impl: IBluetoothLEManufacturerDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(&*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBluetoothLEManufacturerData>, ::windows::core::GetTrustLevel, CompanyId::<Impl, IMPL_OFFSET>, SetCompanyId::<Impl, IMPL_OFFSET>, Data::<Impl, IMPL_OFFSET>, SetData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEManufacturerData as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IBluetoothLEManufacturerDataFactoryImpl: Sized {
    fn Create(&self, companyid: u16, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<BluetoothLEManufacturerData>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBluetoothLEManufacturerDataFactory {
    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEManufacturerDataFactory";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IBluetoothLEManufacturerDataFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBluetoothLEManufacturerDataFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBluetoothLEManufacturerDataFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IBluetoothLEManufacturerDataFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, companyid: u16, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(companyid, &*(&data as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBluetoothLEManufacturerDataFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBluetoothLEManufacturerDataFactory as ::windows::core::Interface>::IID
    }
}
