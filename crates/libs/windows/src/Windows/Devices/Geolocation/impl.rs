#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICivicAddressImpl: Sized {
    fn Country(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn State(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn City(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PostalCode(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICivicAddress {
    const NAME: &'static str = "Windows.Devices.Geolocation.ICivicAddress";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICivicAddressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICivicAddressImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICivicAddressVtbl {
        unsafe extern "system" fn Country<Impl: ICivicAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Country() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ICivicAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn City<Impl: ICivicAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).City() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostalCode<Impl: ICivicAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostalCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: ICivicAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICivicAddress, BASE_OFFSET>(),
            Country: Country::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            City: City::<Impl, IMPL_OFFSET>,
            PostalCode: PostalCode::<Impl, IMPL_OFFSET>,
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICivicAddress as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeoboundingBoxImpl: Sized + IGeoshapeImpl {
    fn NorthwestCorner(&mut self) -> ::windows::core::Result<BasicGeoposition>;
    fn SoutheastCorner(&mut self) -> ::windows::core::Result<BasicGeoposition>;
    fn Center(&mut self) -> ::windows::core::Result<BasicGeoposition>;
    fn MinAltitude(&mut self) -> ::windows::core::Result<f64>;
    fn MaxAltitude(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeoboundingBox {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeoboundingBox";
}
#[cfg(feature = "implement_exclusive")]
impl IGeoboundingBoxVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeoboundingBoxImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeoboundingBoxVtbl {
        unsafe extern "system" fn NorthwestCorner<Impl: IGeoboundingBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BasicGeoposition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NorthwestCorner() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SoutheastCorner<Impl: IGeoboundingBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BasicGeoposition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SoutheastCorner() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Center<Impl: IGeoboundingBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BasicGeoposition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Center() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinAltitude<Impl: IGeoboundingBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinAltitude() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxAltitude<Impl: IGeoboundingBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxAltitude() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeoboundingBox, BASE_OFFSET>(),
            NorthwestCorner: NorthwestCorner::<Impl, IMPL_OFFSET>,
            SoutheastCorner: SoutheastCorner::<Impl, IMPL_OFFSET>,
            Center: Center::<Impl, IMPL_OFFSET>,
            MinAltitude: MinAltitude::<Impl, IMPL_OFFSET>,
            MaxAltitude: MaxAltitude::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeoboundingBox as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeoboundingBoxFactoryImpl: Sized {
    fn Create(&mut self, northwestcorner: &BasicGeoposition, southeastcorner: &BasicGeoposition) -> ::windows::core::Result<GeoboundingBox>;
    fn CreateWithAltitudeReference(&mut self, northwestcorner: &BasicGeoposition, southeastcorner: &BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows::core::Result<GeoboundingBox>;
    fn CreateWithAltitudeReferenceAndSpatialReference(&mut self, northwestcorner: &BasicGeoposition, southeastcorner: &BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::core::Result<GeoboundingBox>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeoboundingBoxFactory {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeoboundingBoxFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGeoboundingBoxFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeoboundingBoxFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeoboundingBoxFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IGeoboundingBoxFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&northwestcorner as *const <BasicGeoposition as ::windows::core::Abi>::Abi as *const <BasicGeoposition as ::windows::core::DefaultType>::DefaultType), &*(&southeastcorner as *const <BasicGeoposition as ::windows::core::Abi>::Abi as *const <BasicGeoposition as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithAltitudeReference<Impl: IGeoboundingBoxFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithAltitudeReference(&*(&northwestcorner as *const <BasicGeoposition as ::windows::core::Abi>::Abi as *const <BasicGeoposition as ::windows::core::DefaultType>::DefaultType), &*(&southeastcorner as *const <BasicGeoposition as ::windows::core::Abi>::Abi as *const <BasicGeoposition as ::windows::core::DefaultType>::DefaultType), altitudereferencesystem) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithAltitudeReferenceAndSpatialReference<Impl: IGeoboundingBoxFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithAltitudeReferenceAndSpatialReference(&*(&northwestcorner as *const <BasicGeoposition as ::windows::core::Abi>::Abi as *const <BasicGeoposition as ::windows::core::DefaultType>::DefaultType), &*(&southeastcorner as *const <BasicGeoposition as ::windows::core::Abi>::Abi as *const <BasicGeoposition as ::windows::core::DefaultType>::DefaultType), altitudereferencesystem, spatialreferenceid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeoboundingBoxFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            CreateWithAltitudeReference: CreateWithAltitudeReference::<Impl, IMPL_OFFSET>,
            CreateWithAltitudeReferenceAndSpatialReference: CreateWithAltitudeReferenceAndSpatialReference::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeoboundingBoxFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGeoboundingBoxStaticsImpl: Sized {
    fn TryCompute(&mut self, positions: &::core::option::Option<super::super::Foundation::Collections::IIterable<BasicGeoposition>>) -> ::windows::core::Result<GeoboundingBox>;
    fn TryComputeWithAltitudeReference(&mut self, positions: &::core::option::Option<super::super::Foundation::Collections::IIterable<BasicGeoposition>>, altituderefsystem: AltitudeReferenceSystem) -> ::windows::core::Result<GeoboundingBox>;
    fn TryComputeWithAltitudeReferenceAndSpatialReference(&mut self, positions: &::core::option::Option<super::super::Foundation::Collections::IIterable<BasicGeoposition>>, altituderefsystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::core::Result<GeoboundingBox>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGeoboundingBoxStatics {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeoboundingBoxStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGeoboundingBoxStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeoboundingBoxStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeoboundingBoxStaticsVtbl {
        unsafe extern "system" fn TryCompute<Impl: IGeoboundingBoxStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, positions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCompute(&*(&positions as *const <super::super::Foundation::Collections::IIterable<BasicGeoposition> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<BasicGeoposition> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryComputeWithAltitudeReference<Impl: IGeoboundingBoxStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, positions: ::windows::core::RawPtr, altituderefsystem: AltitudeReferenceSystem, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryComputeWithAltitudeReference(&*(&positions as *const <super::super::Foundation::Collections::IIterable<BasicGeoposition> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<BasicGeoposition> as ::windows::core::DefaultType>::DefaultType), altituderefsystem) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryComputeWithAltitudeReferenceAndSpatialReference<Impl: IGeoboundingBoxStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, positions: ::windows::core::RawPtr, altituderefsystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryComputeWithAltitudeReferenceAndSpatialReference(&*(&positions as *const <super::super::Foundation::Collections::IIterable<BasicGeoposition> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<BasicGeoposition> as ::windows::core::DefaultType>::DefaultType), altituderefsystem, spatialreferenceid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeoboundingBoxStatics, BASE_OFFSET>(),
            TryCompute: TryCompute::<Impl, IMPL_OFFSET>,
            TryComputeWithAltitudeReference: TryComputeWithAltitudeReference::<Impl, IMPL_OFFSET>,
            TryComputeWithAltitudeReferenceAndSpatialReference: TryComputeWithAltitudeReferenceAndSpatialReference::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeoboundingBoxStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeocircleImpl: Sized + IGeoshapeImpl {
    fn Center(&mut self) -> ::windows::core::Result<BasicGeoposition>;
    fn Radius(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeocircle {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeocircle";
}
#[cfg(feature = "implement_exclusive")]
impl IGeocircleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeocircleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeocircleVtbl {
        unsafe extern "system" fn Center<Impl: IGeocircleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BasicGeoposition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Center() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Radius<Impl: IGeocircleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Radius() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeocircle, BASE_OFFSET>(),
            Center: Center::<Impl, IMPL_OFFSET>,
            Radius: Radius::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeocircle as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeocircleFactoryImpl: Sized {
    fn Create(&mut self, position: &BasicGeoposition, radius: f64) -> ::windows::core::Result<Geocircle>;
    fn CreateWithAltitudeReferenceSystem(&mut self, position: &BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows::core::Result<Geocircle>;
    fn CreateWithAltitudeReferenceSystemAndSpatialReferenceId(&mut self, position: &BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::core::Result<Geocircle>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeocircleFactory {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeocircleFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGeocircleFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeocircleFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeocircleFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IGeocircleFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: BasicGeoposition, radius: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&position as *const <BasicGeoposition as ::windows::core::Abi>::Abi as *const <BasicGeoposition as ::windows::core::DefaultType>::DefaultType), radius) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithAltitudeReferenceSystem<Impl: IGeocircleFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithAltitudeReferenceSystem(&*(&position as *const <BasicGeoposition as ::windows::core::Abi>::Abi as *const <BasicGeoposition as ::windows::core::DefaultType>::DefaultType), radius, altitudereferencesystem) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithAltitudeReferenceSystemAndSpatialReferenceId<Impl: IGeocircleFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithAltitudeReferenceSystemAndSpatialReferenceId(&*(&position as *const <BasicGeoposition as ::windows::core::Abi>::Abi as *const <BasicGeoposition as ::windows::core::DefaultType>::DefaultType), radius, altitudereferencesystem, spatialreferenceid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeocircleFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            CreateWithAltitudeReferenceSystem: CreateWithAltitudeReferenceSystem::<Impl, IMPL_OFFSET>,
            CreateWithAltitudeReferenceSystemAndSpatialReferenceId: CreateWithAltitudeReferenceSystemAndSpatialReferenceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeocircleFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGeocoordinateImpl: Sized {
    fn Latitude(&mut self) -> ::windows::core::Result<f64>;
    fn Longitude(&mut self) -> ::windows::core::Result<f64>;
    fn Altitude(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn Accuracy(&mut self) -> ::windows::core::Result<f64>;
    fn AltitudeAccuracy(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn Heading(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn Speed(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGeocoordinate {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeocoordinate";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGeocoordinateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeocoordinateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeocoordinateVtbl {
        unsafe extern "system" fn Latitude<Impl: IGeocoordinateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Latitude() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Longitude<Impl: IGeocoordinateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Longitude() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Altitude<Impl: IGeocoordinateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Altitude() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Accuracy<Impl: IGeocoordinateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Accuracy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AltitudeAccuracy<Impl: IGeocoordinateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AltitudeAccuracy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Heading<Impl: IGeocoordinateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Heading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Speed<Impl: IGeocoordinateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Speed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: IGeocoordinateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeocoordinate, BASE_OFFSET>(),
            Latitude: Latitude::<Impl, IMPL_OFFSET>,
            Longitude: Longitude::<Impl, IMPL_OFFSET>,
            Altitude: Altitude::<Impl, IMPL_OFFSET>,
            Accuracy: Accuracy::<Impl, IMPL_OFFSET>,
            AltitudeAccuracy: AltitudeAccuracy::<Impl, IMPL_OFFSET>,
            Heading: Heading::<Impl, IMPL_OFFSET>,
            Speed: Speed::<Impl, IMPL_OFFSET>,
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeocoordinate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGeocoordinateSatelliteDataImpl: Sized {
    fn PositionDilutionOfPrecision(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn HorizontalDilutionOfPrecision(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn VerticalDilutionOfPrecision(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGeocoordinateSatelliteData {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeocoordinateSatelliteData";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGeocoordinateSatelliteDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeocoordinateSatelliteDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeocoordinateSatelliteDataVtbl {
        unsafe extern "system" fn PositionDilutionOfPrecision<Impl: IGeocoordinateSatelliteDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PositionDilutionOfPrecision() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalDilutionOfPrecision<Impl: IGeocoordinateSatelliteDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalDilutionOfPrecision() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalDilutionOfPrecision<Impl: IGeocoordinateSatelliteDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalDilutionOfPrecision() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeocoordinateSatelliteData, BASE_OFFSET>(),
            PositionDilutionOfPrecision: PositionDilutionOfPrecision::<Impl, IMPL_OFFSET>,
            HorizontalDilutionOfPrecision: HorizontalDilutionOfPrecision::<Impl, IMPL_OFFSET>,
            VerticalDilutionOfPrecision: VerticalDilutionOfPrecision::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeocoordinateSatelliteData as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGeocoordinateSatelliteData2Impl: Sized {
    fn GeometricDilutionOfPrecision(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn TimeDilutionOfPrecision(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGeocoordinateSatelliteData2 {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeocoordinateSatelliteData2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGeocoordinateSatelliteData2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeocoordinateSatelliteData2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeocoordinateSatelliteData2Vtbl {
        unsafe extern "system" fn GeometricDilutionOfPrecision<Impl: IGeocoordinateSatelliteData2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GeometricDilutionOfPrecision() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimeDilutionOfPrecision<Impl: IGeocoordinateSatelliteData2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TimeDilutionOfPrecision() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeocoordinateSatelliteData2, BASE_OFFSET>(),
            GeometricDilutionOfPrecision: GeometricDilutionOfPrecision::<Impl, IMPL_OFFSET>,
            TimeDilutionOfPrecision: TimeDilutionOfPrecision::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeocoordinateSatelliteData2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeocoordinateWithPointImpl: Sized {
    fn Point(&mut self) -> ::windows::core::Result<Geopoint>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeocoordinateWithPoint {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeocoordinateWithPoint";
}
#[cfg(feature = "implement_exclusive")]
impl IGeocoordinateWithPointVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeocoordinateWithPointImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeocoordinateWithPointVtbl {
        unsafe extern "system" fn Point<Impl: IGeocoordinateWithPointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Point() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGeocoordinateWithPoint, BASE_OFFSET>(), Point: Point::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeocoordinateWithPoint as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGeocoordinateWithPositionDataImpl: Sized + IGeocoordinateImpl {
    fn PositionSource(&mut self) -> ::windows::core::Result<PositionSource>;
    fn SatelliteData(&mut self) -> ::windows::core::Result<GeocoordinateSatelliteData>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGeocoordinateWithPositionData {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeocoordinateWithPositionData";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGeocoordinateWithPositionDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeocoordinateWithPositionDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeocoordinateWithPositionDataVtbl {
        unsafe extern "system" fn PositionSource<Impl: IGeocoordinateWithPositionDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PositionSource) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PositionSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SatelliteData<Impl: IGeocoordinateWithPositionDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SatelliteData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeocoordinateWithPositionData, BASE_OFFSET>(),
            PositionSource: PositionSource::<Impl, IMPL_OFFSET>,
            SatelliteData: SatelliteData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeocoordinateWithPositionData as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGeocoordinateWithPositionSourceTimestampImpl: Sized {
    fn PositionSourceTimestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGeocoordinateWithPositionSourceTimestamp {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeocoordinateWithPositionSourceTimestamp";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGeocoordinateWithPositionSourceTimestampVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeocoordinateWithPositionSourceTimestampImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeocoordinateWithPositionSourceTimestampVtbl {
        unsafe extern "system" fn PositionSourceTimestamp<Impl: IGeocoordinateWithPositionSourceTimestampImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PositionSourceTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeocoordinateWithPositionSourceTimestamp, BASE_OFFSET>(),
            PositionSourceTimestamp: PositionSourceTimestamp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeocoordinateWithPositionSourceTimestamp as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeocoordinateWithRemoteSourceImpl: Sized {
    fn IsRemoteSource(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeocoordinateWithRemoteSource {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeocoordinateWithRemoteSource";
}
#[cfg(feature = "implement_exclusive")]
impl IGeocoordinateWithRemoteSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeocoordinateWithRemoteSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeocoordinateWithRemoteSourceVtbl {
        unsafe extern "system" fn IsRemoteSource<Impl: IGeocoordinateWithRemoteSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRemoteSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeocoordinateWithRemoteSource, BASE_OFFSET>(),
            IsRemoteSource: IsRemoteSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeocoordinateWithRemoteSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGeolocatorImpl: Sized {
    fn DesiredAccuracy(&mut self) -> ::windows::core::Result<PositionAccuracy>;
    fn SetDesiredAccuracy(&mut self, value: PositionAccuracy) -> ::windows::core::Result<()>;
    fn MovementThreshold(&mut self) -> ::windows::core::Result<f64>;
    fn SetMovementThreshold(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn ReportInterval(&mut self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn LocationStatus(&mut self) -> ::windows::core::Result<PositionStatus>;
    fn GetGeopositionAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Geoposition>>;
    fn GetGeopositionAsyncWithAgeAndTimeout(&mut self, maximumage: &super::super::Foundation::TimeSpan, timeout: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Geoposition>>;
    fn PositionChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Geolocator, PositionChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePositionChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StatusChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Geolocator, StatusChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGeolocator {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeolocator";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGeolocatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeolocatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeolocatorVtbl {
        unsafe extern "system" fn DesiredAccuracy<Impl: IGeolocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PositionAccuracy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredAccuracy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredAccuracy<Impl: IGeolocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PositionAccuracy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredAccuracy(value).into()
        }
        unsafe extern "system" fn MovementThreshold<Impl: IGeolocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MovementThreshold() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMovementThreshold<Impl: IGeolocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMovementThreshold(value).into()
        }
        unsafe extern "system" fn ReportInterval<Impl: IGeolocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportInterval<Impl: IGeolocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReportInterval(value).into()
        }
        unsafe extern "system" fn LocationStatus<Impl: IGeolocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PositionStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocationStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGeopositionAsync<Impl: IGeolocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGeopositionAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGeopositionAsyncWithAgeAndTimeout<Impl: IGeolocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maximumage: super::super::Foundation::TimeSpan, timeout: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGeopositionAsyncWithAgeAndTimeout(&*(&maximumage as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType), &*(&timeout as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PositionChanged<Impl: IGeolocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PositionChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<Geolocator, PositionChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<Geolocator, PositionChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePositionChanged<Impl: IGeolocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePositionChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StatusChanged<Impl: IGeolocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<Geolocator, StatusChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<Geolocator, StatusChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStatusChanged<Impl: IGeolocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStatusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeolocator, BASE_OFFSET>(),
            DesiredAccuracy: DesiredAccuracy::<Impl, IMPL_OFFSET>,
            SetDesiredAccuracy: SetDesiredAccuracy::<Impl, IMPL_OFFSET>,
            MovementThreshold: MovementThreshold::<Impl, IMPL_OFFSET>,
            SetMovementThreshold: SetMovementThreshold::<Impl, IMPL_OFFSET>,
            ReportInterval: ReportInterval::<Impl, IMPL_OFFSET>,
            SetReportInterval: SetReportInterval::<Impl, IMPL_OFFSET>,
            LocationStatus: LocationStatus::<Impl, IMPL_OFFSET>,
            GetGeopositionAsync: GetGeopositionAsync::<Impl, IMPL_OFFSET>,
            GetGeopositionAsyncWithAgeAndTimeout: GetGeopositionAsyncWithAgeAndTimeout::<Impl, IMPL_OFFSET>,
            PositionChanged: PositionChanged::<Impl, IMPL_OFFSET>,
            RemovePositionChanged: RemovePositionChanged::<Impl, IMPL_OFFSET>,
            StatusChanged: StatusChanged::<Impl, IMPL_OFFSET>,
            RemoveStatusChanged: RemoveStatusChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeolocator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeolocator2Impl: Sized {
    fn AllowFallbackToConsentlessPositions(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeolocator2 {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeolocator2";
}
#[cfg(feature = "implement_exclusive")]
impl IGeolocator2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeolocator2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeolocator2Vtbl {
        unsafe extern "system" fn AllowFallbackToConsentlessPositions<Impl: IGeolocator2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AllowFallbackToConsentlessPositions().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeolocator2, BASE_OFFSET>(),
            AllowFallbackToConsentlessPositions: AllowFallbackToConsentlessPositions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeolocator2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGeolocatorStaticsImpl: Sized {
    fn RequestAccessAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GeolocationAccessStatus>>;
    fn GetGeopositionHistoryAsync(&mut self, starttime: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Geoposition>>>;
    fn GetGeopositionHistoryWithDurationAsync(&mut self, starttime: &super::super::Foundation::DateTime, duration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Geoposition>>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGeolocatorStatics {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeolocatorStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGeolocatorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeolocatorStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeolocatorStaticsVtbl {
        unsafe extern "system" fn RequestAccessAsync<Impl: IGeolocatorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetGeopositionHistoryAsync<Impl: IGeolocatorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGeopositionHistoryAsync(&*(&starttime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGeopositionHistoryWithDurationAsync<Impl: IGeolocatorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGeopositionHistoryWithDurationAsync(&*(&starttime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType), &*(&duration as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeolocatorStatics, BASE_OFFSET>(),
            RequestAccessAsync: RequestAccessAsync::<Impl, IMPL_OFFSET>,
            GetGeopositionHistoryAsync: GetGeopositionHistoryAsync::<Impl, IMPL_OFFSET>,
            GetGeopositionHistoryWithDurationAsync: GetGeopositionHistoryWithDurationAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeolocatorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGeolocatorStatics2Impl: Sized {
    fn IsDefaultGeopositionRecommended(&mut self) -> ::windows::core::Result<bool>;
    fn SetDefaultGeoposition(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<BasicGeoposition>>) -> ::windows::core::Result<()>;
    fn DefaultGeoposition(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<BasicGeoposition>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGeolocatorStatics2 {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeolocatorStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGeolocatorStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeolocatorStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeolocatorStatics2Vtbl {
        unsafe extern "system" fn IsDefaultGeopositionRecommended<Impl: IGeolocatorStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDefaultGeopositionRecommended() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultGeoposition<Impl: IGeolocatorStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultGeoposition(&*(&value as *const <super::super::Foundation::IReference<BasicGeoposition> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<BasicGeoposition> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DefaultGeoposition<Impl: IGeolocatorStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultGeoposition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeolocatorStatics2, BASE_OFFSET>(),
            IsDefaultGeopositionRecommended: IsDefaultGeopositionRecommended::<Impl, IMPL_OFFSET>,
            SetDefaultGeoposition: SetDefaultGeoposition::<Impl, IMPL_OFFSET>,
            DefaultGeoposition: DefaultGeoposition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeolocatorStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGeolocatorWithScalarAccuracyImpl: Sized + IGeolocatorImpl {
    fn DesiredAccuracyInMeters(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn SetDesiredAccuracyInMeters(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGeolocatorWithScalarAccuracy {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeolocatorWithScalarAccuracy";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGeolocatorWithScalarAccuracyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeolocatorWithScalarAccuracyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeolocatorWithScalarAccuracyVtbl {
        unsafe extern "system" fn DesiredAccuracyInMeters<Impl: IGeolocatorWithScalarAccuracyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredAccuracyInMeters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredAccuracyInMeters<Impl: IGeolocatorWithScalarAccuracyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredAccuracyInMeters(&*(&value as *const <super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeolocatorWithScalarAccuracy, BASE_OFFSET>(),
            DesiredAccuracyInMeters: DesiredAccuracyInMeters::<Impl, IMPL_OFFSET>,
            SetDesiredAccuracyInMeters: SetDesiredAccuracyInMeters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeolocatorWithScalarAccuracy as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGeopathImpl: Sized + IGeoshapeImpl {
    fn Positions(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<BasicGeoposition>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGeopath {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeopath";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGeopathVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeopathImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeopathVtbl {
        unsafe extern "system" fn Positions<Impl: IGeopathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Positions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGeopath, BASE_OFFSET>(), Positions: Positions::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeopath as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGeopathFactoryImpl: Sized {
    fn Create(&mut self, positions: &::core::option::Option<super::super::Foundation::Collections::IIterable<BasicGeoposition>>) -> ::windows::core::Result<Geopath>;
    fn CreateWithAltitudeReference(&mut self, positions: &::core::option::Option<super::super::Foundation::Collections::IIterable<BasicGeoposition>>, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows::core::Result<Geopath>;
    fn CreateWithAltitudeReferenceAndSpatialReference(&mut self, positions: &::core::option::Option<super::super::Foundation::Collections::IIterable<BasicGeoposition>>, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::core::Result<Geopath>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGeopathFactory {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeopathFactory";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGeopathFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeopathFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeopathFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IGeopathFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, positions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&positions as *const <super::super::Foundation::Collections::IIterable<BasicGeoposition> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<BasicGeoposition> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithAltitudeReference<Impl: IGeopathFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, positions: ::windows::core::RawPtr, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithAltitudeReference(&*(&positions as *const <super::super::Foundation::Collections::IIterable<BasicGeoposition> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<BasicGeoposition> as ::windows::core::DefaultType>::DefaultType), altitudereferencesystem) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithAltitudeReferenceAndSpatialReference<Impl: IGeopathFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, positions: ::windows::core::RawPtr, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithAltitudeReferenceAndSpatialReference(&*(&positions as *const <super::super::Foundation::Collections::IIterable<BasicGeoposition> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<BasicGeoposition> as ::windows::core::DefaultType>::DefaultType), altitudereferencesystem, spatialreferenceid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeopathFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            CreateWithAltitudeReference: CreateWithAltitudeReference::<Impl, IMPL_OFFSET>,
            CreateWithAltitudeReferenceAndSpatialReference: CreateWithAltitudeReferenceAndSpatialReference::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeopathFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeopointImpl: Sized + IGeoshapeImpl {
    fn Position(&mut self) -> ::windows::core::Result<BasicGeoposition>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeopoint {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeopoint";
}
#[cfg(feature = "implement_exclusive")]
impl IGeopointVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeopointImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeopointVtbl {
        unsafe extern "system" fn Position<Impl: IGeopointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BasicGeoposition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGeopoint, BASE_OFFSET>(), Position: Position::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeopoint as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeopointFactoryImpl: Sized {
    fn Create(&mut self, position: &BasicGeoposition) -> ::windows::core::Result<Geopoint>;
    fn CreateWithAltitudeReferenceSystem(&mut self, position: &BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows::core::Result<Geopoint>;
    fn CreateWithAltitudeReferenceSystemAndSpatialReferenceId(&mut self, position: &BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::core::Result<Geopoint>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeopointFactory {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeopointFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGeopointFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeopointFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeopointFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IGeopointFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: BasicGeoposition, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&position as *const <BasicGeoposition as ::windows::core::Abi>::Abi as *const <BasicGeoposition as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithAltitudeReferenceSystem<Impl: IGeopointFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithAltitudeReferenceSystem(&*(&position as *const <BasicGeoposition as ::windows::core::Abi>::Abi as *const <BasicGeoposition as ::windows::core::DefaultType>::DefaultType), altitudereferencesystem) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithAltitudeReferenceSystemAndSpatialReferenceId<Impl: IGeopointFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithAltitudeReferenceSystemAndSpatialReferenceId(&*(&position as *const <BasicGeoposition as ::windows::core::Abi>::Abi as *const <BasicGeoposition as ::windows::core::DefaultType>::DefaultType), altitudereferencesystem, spatialreferenceid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeopointFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            CreateWithAltitudeReferenceSystem: CreateWithAltitudeReferenceSystem::<Impl, IMPL_OFFSET>,
            CreateWithAltitudeReferenceSystemAndSpatialReferenceId: CreateWithAltitudeReferenceSystemAndSpatialReferenceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeopointFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeopositionImpl: Sized {
    fn Coordinate(&mut self) -> ::windows::core::Result<Geocoordinate>;
    fn CivicAddress(&mut self) -> ::windows::core::Result<CivicAddress>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeoposition {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeoposition";
}
#[cfg(feature = "implement_exclusive")]
impl IGeopositionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeopositionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeopositionVtbl {
        unsafe extern "system" fn Coordinate<Impl: IGeopositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Coordinate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CivicAddress<Impl: IGeopositionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CivicAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeoposition, BASE_OFFSET>(),
            Coordinate: Coordinate::<Impl, IMPL_OFFSET>,
            CivicAddress: CivicAddress::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeoposition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeoposition2Impl: Sized + IGeopositionImpl {
    fn VenueData(&mut self) -> ::windows::core::Result<VenueData>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeoposition2 {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeoposition2";
}
#[cfg(feature = "implement_exclusive")]
impl IGeoposition2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeoposition2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeoposition2Vtbl {
        unsafe extern "system" fn VenueData<Impl: IGeoposition2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VenueData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGeoposition2, BASE_OFFSET>(), VenueData: VenueData::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeoposition2 as ::windows::core::Interface>::IID
    }
}
pub trait IGeoshapeImpl: Sized {
    fn GeoshapeType(&mut self) -> ::windows::core::Result<GeoshapeType>;
    fn SpatialReferenceId(&mut self) -> ::windows::core::Result<u32>;
    fn AltitudeReferenceSystem(&mut self) -> ::windows::core::Result<AltitudeReferenceSystem>;
}
impl ::windows::core::RuntimeName for IGeoshape {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeoshape";
}
impl IGeoshapeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeoshapeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeoshapeVtbl {
        unsafe extern "system" fn GeoshapeType<Impl: IGeoshapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GeoshapeType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GeoshapeType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpatialReferenceId<Impl: IGeoshapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpatialReferenceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AltitudeReferenceSystem<Impl: IGeoshapeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AltitudeReferenceSystem) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AltitudeReferenceSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeoshape, BASE_OFFSET>(),
            GeoshapeType: GeoshapeType::<Impl, IMPL_OFFSET>,
            SpatialReferenceId: SpatialReferenceId::<Impl, IMPL_OFFSET>,
            AltitudeReferenceSystem: AltitudeReferenceSystem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeoshape as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGeovisitImpl: Sized {
    fn Position(&mut self) -> ::windows::core::Result<Geoposition>;
    fn StateChange(&mut self) -> ::windows::core::Result<VisitStateChange>;
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGeovisit {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeovisit";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGeovisitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeovisitImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeovisitVtbl {
        unsafe extern "system" fn Position<Impl: IGeovisitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StateChange<Impl: IGeovisitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VisitStateChange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StateChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: IGeovisitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeovisit, BASE_OFFSET>(),
            Position: Position::<Impl, IMPL_OFFSET>,
            StateChange: StateChange::<Impl, IMPL_OFFSET>,
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeovisit as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGeovisitMonitorImpl: Sized {
    fn MonitoringScope(&mut self) -> ::windows::core::Result<VisitMonitoringScope>;
    fn Start(&mut self, value: VisitMonitoringScope) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn VisitStateChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GeovisitMonitor, GeovisitStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVisitStateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGeovisitMonitor {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeovisitMonitor";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGeovisitMonitorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeovisitMonitorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeovisitMonitorVtbl {
        unsafe extern "system" fn MonitoringScope<Impl: IGeovisitMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VisitMonitoringScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MonitoringScope() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IGeovisitMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VisitMonitoringScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start(value).into()
        }
        unsafe extern "system" fn Stop<Impl: IGeovisitMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn VisitStateChanged<Impl: IGeovisitMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VisitStateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<GeovisitMonitor, GeovisitStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GeovisitMonitor, GeovisitStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVisitStateChanged<Impl: IGeovisitMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVisitStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeovisitMonitor, BASE_OFFSET>(),
            MonitoringScope: MonitoringScope::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            VisitStateChanged: VisitStateChanged::<Impl, IMPL_OFFSET>,
            RemoveVisitStateChanged: RemoveVisitStateChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeovisitMonitor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGeovisitMonitorStaticsImpl: Sized {
    fn GetLastReportAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Geovisit>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGeovisitMonitorStatics {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeovisitMonitorStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IGeovisitMonitorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeovisitMonitorStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeovisitMonitorStaticsVtbl {
        unsafe extern "system" fn GetLastReportAsync<Impl: IGeovisitMonitorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastReportAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeovisitMonitorStatics, BASE_OFFSET>(),
            GetLastReportAsync: GetLastReportAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeovisitMonitorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeovisitStateChangedEventArgsImpl: Sized {
    fn Visit(&mut self) -> ::windows::core::Result<Geovisit>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeovisitStateChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeovisitStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IGeovisitStateChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeovisitStateChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeovisitStateChangedEventArgsVtbl {
        unsafe extern "system" fn Visit<Impl: IGeovisitStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Visit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGeovisitStateChangedEventArgs, BASE_OFFSET>(), Visit: Visit::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeovisitStateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IGeovisitTriggerDetailsImpl: Sized {
    fn ReadReports(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Geovisit>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGeovisitTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeovisitTriggerDetails";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IGeovisitTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeovisitTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeovisitTriggerDetailsVtbl {
        unsafe extern "system" fn ReadReports<Impl: IGeovisitTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadReports() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGeovisitTriggerDetails, BASE_OFFSET>(), ReadReports: ReadReports::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeovisitTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPositionChangedEventArgsImpl: Sized {
    fn Position(&mut self) -> ::windows::core::Result<Geoposition>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPositionChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Geolocation.IPositionChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPositionChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPositionChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPositionChangedEventArgsVtbl {
        unsafe extern "system" fn Position<Impl: IPositionChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPositionChangedEventArgs, BASE_OFFSET>(), Position: Position::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPositionChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStatusChangedEventArgsImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<PositionStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Geolocation.IStatusChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IStatusChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStatusChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStatusChangedEventArgsVtbl {
        unsafe extern "system" fn Status<Impl: IStatusChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PositionStatus) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IStatusChangedEventArgs, BASE_OFFSET>(), Status: Status::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStatusChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVenueDataImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Level(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVenueData {
    const NAME: &'static str = "Windows.Devices.Geolocation.IVenueData";
}
#[cfg(feature = "implement_exclusive")]
impl IVenueDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVenueDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVenueDataVtbl {
        unsafe extern "system" fn Id<Impl: IVenueDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Level<Impl: IVenueDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Level() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVenueData, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Level: Level::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVenueData as ::windows::core::Interface>::IID
    }
}
