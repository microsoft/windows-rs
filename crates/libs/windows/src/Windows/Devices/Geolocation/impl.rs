#[cfg(feature = "implement_exclusive")]
pub trait ICivicAddressImpl: Sized {
    fn Country(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn State(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn City(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PostalCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICivicAddress {
    const NAME: &'static str = "Windows.Devices.Geolocation.ICivicAddress";
}
#[cfg(feature = "implement_exclusive")]
impl ICivicAddressVtbl {
    pub const fn new<Impl: ICivicAddressImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICivicAddressVtbl {
        unsafe extern "system" fn Country<Impl: ICivicAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Country() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ICivicAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn City<Impl: ICivicAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).City() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostalCode<Impl: ICivicAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PostalCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: ICivicAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICivicAddress>, base.5, Country::<Impl, OFFSET>, State::<Impl, OFFSET>, City::<Impl, OFFSET>, PostalCode::<Impl, OFFSET>, Timestamp::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeoboundingBoxImpl: Sized + IGeoshapeImpl {
    fn NorthwestCorner(&self) -> ::windows::core::Result<BasicGeoposition>;
    fn SoutheastCorner(&self) -> ::windows::core::Result<BasicGeoposition>;
    fn Center(&self) -> ::windows::core::Result<BasicGeoposition>;
    fn MinAltitude(&self) -> ::windows::core::Result<f64>;
    fn MaxAltitude(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeoboundingBox {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeoboundingBox";
}
#[cfg(feature = "implement_exclusive")]
impl IGeoboundingBoxVtbl {
    pub const fn new<Impl: IGeoboundingBoxImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeoboundingBoxVtbl {
        unsafe extern "system" fn NorthwestCorner<Impl: IGeoboundingBoxImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BasicGeoposition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NorthwestCorner() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SoutheastCorner<Impl: IGeoboundingBoxImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BasicGeoposition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SoutheastCorner() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Center<Impl: IGeoboundingBoxImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BasicGeoposition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Center() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinAltitude<Impl: IGeoboundingBoxImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinAltitude() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxAltitude<Impl: IGeoboundingBoxImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxAltitude() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeoboundingBox>, base.5, NorthwestCorner::<Impl, OFFSET>, SoutheastCorner::<Impl, OFFSET>, Center::<Impl, OFFSET>, MinAltitude::<Impl, OFFSET>, MaxAltitude::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeoboundingBoxFactoryImpl: Sized {
    fn Create(&self, northwestcorner: &BasicGeoposition, southeastcorner: &BasicGeoposition) -> ::windows::core::Result<GeoboundingBox>;
    fn CreateWithAltitudeReference(&self, northwestcorner: &BasicGeoposition, southeastcorner: &BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows::core::Result<GeoboundingBox>;
    fn CreateWithAltitudeReferenceAndSpatialReference(&self, northwestcorner: &BasicGeoposition, southeastcorner: &BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::core::Result<GeoboundingBox>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeoboundingBoxFactory {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeoboundingBoxFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGeoboundingBoxFactoryVtbl {
    pub const fn new<Impl: IGeoboundingBoxFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeoboundingBoxFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IGeoboundingBoxFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&northwestcorner as *const <BasicGeoposition as ::windows::core::Abi>::Abi as *const <BasicGeoposition as ::windows::core::DefaultType>::DefaultType), &*(&southeastcorner as *const <BasicGeoposition as ::windows::core::Abi>::Abi as *const <BasicGeoposition as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithAltitudeReference<Impl: IGeoboundingBoxFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithAltitudeReference(&*(&northwestcorner as *const <BasicGeoposition as ::windows::core::Abi>::Abi as *const <BasicGeoposition as ::windows::core::DefaultType>::DefaultType), &*(&southeastcorner as *const <BasicGeoposition as ::windows::core::Abi>::Abi as *const <BasicGeoposition as ::windows::core::DefaultType>::DefaultType), altitudereferencesystem) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithAltitudeReferenceAndSpatialReference<Impl: IGeoboundingBoxFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithAltitudeReferenceAndSpatialReference(&*(&northwestcorner as *const <BasicGeoposition as ::windows::core::Abi>::Abi as *const <BasicGeoposition as ::windows::core::DefaultType>::DefaultType), &*(&southeastcorner as *const <BasicGeoposition as ::windows::core::Abi>::Abi as *const <BasicGeoposition as ::windows::core::DefaultType>::DefaultType), altitudereferencesystem, spatialreferenceid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeoboundingBoxFactory>, base.5, Create::<Impl, OFFSET>, CreateWithAltitudeReference::<Impl, OFFSET>, CreateWithAltitudeReferenceAndSpatialReference::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeoboundingBoxStaticsImpl: Sized {
    fn TryCompute(&self, positions: &::core::option::Option<super::super::Foundation::Collections::IIterable<BasicGeoposition>>) -> ::windows::core::Result<GeoboundingBox>;
    fn TryComputeWithAltitudeReference(&self, positions: &::core::option::Option<super::super::Foundation::Collections::IIterable<BasicGeoposition>>, altituderefsystem: AltitudeReferenceSystem) -> ::windows::core::Result<GeoboundingBox>;
    fn TryComputeWithAltitudeReferenceAndSpatialReference(&self, positions: &::core::option::Option<super::super::Foundation::Collections::IIterable<BasicGeoposition>>, altituderefsystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::core::Result<GeoboundingBox>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeoboundingBoxStatics {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeoboundingBoxStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGeoboundingBoxStaticsVtbl {
    pub const fn new<Impl: IGeoboundingBoxStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeoboundingBoxStaticsVtbl {
        unsafe extern "system" fn TryCompute<Impl: IGeoboundingBoxStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, positions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryCompute(&*(&positions as *const <super::super::Foundation::Collections::IIterable<BasicGeoposition> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<BasicGeoposition> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryComputeWithAltitudeReference<Impl: IGeoboundingBoxStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, positions: ::windows::core::RawPtr, altituderefsystem: AltitudeReferenceSystem, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryComputeWithAltitudeReference(&*(&positions as *const <super::super::Foundation::Collections::IIterable<BasicGeoposition> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<BasicGeoposition> as ::windows::core::DefaultType>::DefaultType), altituderefsystem) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryComputeWithAltitudeReferenceAndSpatialReference<Impl: IGeoboundingBoxStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, positions: ::windows::core::RawPtr, altituderefsystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryComputeWithAltitudeReferenceAndSpatialReference(&*(&positions as *const <super::super::Foundation::Collections::IIterable<BasicGeoposition> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<BasicGeoposition> as ::windows::core::DefaultType>::DefaultType), altituderefsystem, spatialreferenceid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeoboundingBoxStatics>, base.5, TryCompute::<Impl, OFFSET>, TryComputeWithAltitudeReference::<Impl, OFFSET>, TryComputeWithAltitudeReferenceAndSpatialReference::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeocircleImpl: Sized + IGeoshapeImpl {
    fn Center(&self) -> ::windows::core::Result<BasicGeoposition>;
    fn Radius(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeocircle {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeocircle";
}
#[cfg(feature = "implement_exclusive")]
impl IGeocircleVtbl {
    pub const fn new<Impl: IGeocircleImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeocircleVtbl {
        unsafe extern "system" fn Center<Impl: IGeocircleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BasicGeoposition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Center() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Radius<Impl: IGeocircleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Radius() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeocircle>, base.5, Center::<Impl, OFFSET>, Radius::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeocircleFactoryImpl: Sized {
    fn Create(&self, position: &BasicGeoposition, radius: f64) -> ::windows::core::Result<Geocircle>;
    fn CreateWithAltitudeReferenceSystem(&self, position: &BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows::core::Result<Geocircle>;
    fn CreateWithAltitudeReferenceSystemAndSpatialReferenceId(&self, position: &BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::core::Result<Geocircle>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeocircleFactory {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeocircleFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGeocircleFactoryVtbl {
    pub const fn new<Impl: IGeocircleFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeocircleFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IGeocircleFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, position: BasicGeoposition, radius: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&position as *const <BasicGeoposition as ::windows::core::Abi>::Abi as *const <BasicGeoposition as ::windows::core::DefaultType>::DefaultType), radius) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithAltitudeReferenceSystem<Impl: IGeocircleFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, position: BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithAltitudeReferenceSystem(&*(&position as *const <BasicGeoposition as ::windows::core::Abi>::Abi as *const <BasicGeoposition as ::windows::core::DefaultType>::DefaultType), radius, altitudereferencesystem) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithAltitudeReferenceSystemAndSpatialReferenceId<Impl: IGeocircleFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, position: BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithAltitudeReferenceSystemAndSpatialReferenceId(&*(&position as *const <BasicGeoposition as ::windows::core::Abi>::Abi as *const <BasicGeoposition as ::windows::core::DefaultType>::DefaultType), radius, altitudereferencesystem, spatialreferenceid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeocircleFactory>, base.5, Create::<Impl, OFFSET>, CreateWithAltitudeReferenceSystem::<Impl, OFFSET>, CreateWithAltitudeReferenceSystemAndSpatialReferenceId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeocoordinateImpl: Sized {
    fn Latitude(&self) -> ::windows::core::Result<f64>;
    fn Longitude(&self) -> ::windows::core::Result<f64>;
    fn Altitude(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn Accuracy(&self) -> ::windows::core::Result<f64>;
    fn AltitudeAccuracy(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn Heading(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn Speed(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeocoordinate {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeocoordinate";
}
#[cfg(feature = "implement_exclusive")]
impl IGeocoordinateVtbl {
    pub const fn new<Impl: IGeocoordinateImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeocoordinateVtbl {
        unsafe extern "system" fn Latitude<Impl: IGeocoordinateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Latitude() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Longitude<Impl: IGeocoordinateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Longitude() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Altitude<Impl: IGeocoordinateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Altitude() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Accuracy<Impl: IGeocoordinateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Accuracy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AltitudeAccuracy<Impl: IGeocoordinateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AltitudeAccuracy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Heading<Impl: IGeocoordinateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Heading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Speed<Impl: IGeocoordinateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Speed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: IGeocoordinateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeocoordinate>, base.5, Latitude::<Impl, OFFSET>, Longitude::<Impl, OFFSET>, Altitude::<Impl, OFFSET>, Accuracy::<Impl, OFFSET>, AltitudeAccuracy::<Impl, OFFSET>, Heading::<Impl, OFFSET>, Speed::<Impl, OFFSET>, Timestamp::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeocoordinateSatelliteDataImpl: Sized {
    fn PositionDilutionOfPrecision(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn HorizontalDilutionOfPrecision(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn VerticalDilutionOfPrecision(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeocoordinateSatelliteData {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeocoordinateSatelliteData";
}
#[cfg(feature = "implement_exclusive")]
impl IGeocoordinateSatelliteDataVtbl {
    pub const fn new<Impl: IGeocoordinateSatelliteDataImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeocoordinateSatelliteDataVtbl {
        unsafe extern "system" fn PositionDilutionOfPrecision<Impl: IGeocoordinateSatelliteDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PositionDilutionOfPrecision() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalDilutionOfPrecision<Impl: IGeocoordinateSatelliteDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HorizontalDilutionOfPrecision() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalDilutionOfPrecision<Impl: IGeocoordinateSatelliteDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VerticalDilutionOfPrecision() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeocoordinateSatelliteData>, base.5, PositionDilutionOfPrecision::<Impl, OFFSET>, HorizontalDilutionOfPrecision::<Impl, OFFSET>, VerticalDilutionOfPrecision::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeocoordinateSatelliteData2Impl: Sized {
    fn GeometricDilutionOfPrecision(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn TimeDilutionOfPrecision(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeocoordinateSatelliteData2 {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeocoordinateSatelliteData2";
}
#[cfg(feature = "implement_exclusive")]
impl IGeocoordinateSatelliteData2Vtbl {
    pub const fn new<Impl: IGeocoordinateSatelliteData2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeocoordinateSatelliteData2Vtbl {
        unsafe extern "system" fn GeometricDilutionOfPrecision<Impl: IGeocoordinateSatelliteData2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GeometricDilutionOfPrecision() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TimeDilutionOfPrecision<Impl: IGeocoordinateSatelliteData2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TimeDilutionOfPrecision() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeocoordinateSatelliteData2>, base.5, GeometricDilutionOfPrecision::<Impl, OFFSET>, TimeDilutionOfPrecision::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeocoordinateWithPointImpl: Sized {
    fn Point(&self) -> ::windows::core::Result<Geopoint>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeocoordinateWithPoint {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeocoordinateWithPoint";
}
#[cfg(feature = "implement_exclusive")]
impl IGeocoordinateWithPointVtbl {
    pub const fn new<Impl: IGeocoordinateWithPointImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeocoordinateWithPointVtbl {
        unsafe extern "system" fn Point<Impl: IGeocoordinateWithPointImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Point() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeocoordinateWithPoint>, base.5, Point::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeocoordinateWithPositionDataImpl: Sized + IGeocoordinateImpl {
    fn PositionSource(&self) -> ::windows::core::Result<PositionSource>;
    fn SatelliteData(&self) -> ::windows::core::Result<GeocoordinateSatelliteData>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeocoordinateWithPositionData {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeocoordinateWithPositionData";
}
#[cfg(feature = "implement_exclusive")]
impl IGeocoordinateWithPositionDataVtbl {
    pub const fn new<Impl: IGeocoordinateWithPositionDataImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeocoordinateWithPositionDataVtbl {
        unsafe extern "system" fn PositionSource<Impl: IGeocoordinateWithPositionDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PositionSource) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PositionSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SatelliteData<Impl: IGeocoordinateWithPositionDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SatelliteData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeocoordinateWithPositionData>, base.5, PositionSource::<Impl, OFFSET>, SatelliteData::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeocoordinateWithPositionSourceTimestampImpl: Sized {
    fn PositionSourceTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeocoordinateWithPositionSourceTimestamp {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeocoordinateWithPositionSourceTimestamp";
}
#[cfg(feature = "implement_exclusive")]
impl IGeocoordinateWithPositionSourceTimestampVtbl {
    pub const fn new<Impl: IGeocoordinateWithPositionSourceTimestampImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeocoordinateWithPositionSourceTimestampVtbl {
        unsafe extern "system" fn PositionSourceTimestamp<Impl: IGeocoordinateWithPositionSourceTimestampImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PositionSourceTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeocoordinateWithPositionSourceTimestamp>, base.5, PositionSourceTimestamp::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeocoordinateWithRemoteSourceImpl: Sized {
    fn IsRemoteSource(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeocoordinateWithRemoteSource {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeocoordinateWithRemoteSource";
}
#[cfg(feature = "implement_exclusive")]
impl IGeocoordinateWithRemoteSourceVtbl {
    pub const fn new<Impl: IGeocoordinateWithRemoteSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeocoordinateWithRemoteSourceVtbl {
        unsafe extern "system" fn IsRemoteSource<Impl: IGeocoordinateWithRemoteSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRemoteSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeocoordinateWithRemoteSource>, base.5, IsRemoteSource::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeolocatorImpl: Sized {
    fn DesiredAccuracy(&self) -> ::windows::core::Result<PositionAccuracy>;
    fn SetDesiredAccuracy(&self, value: PositionAccuracy) -> ::windows::core::Result<()>;
    fn MovementThreshold(&self) -> ::windows::core::Result<f64>;
    fn SetMovementThreshold(&self, value: f64) -> ::windows::core::Result<()>;
    fn ReportInterval(&self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()>;
    fn LocationStatus(&self) -> ::windows::core::Result<PositionStatus>;
    fn GetGeopositionAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Geoposition>>;
    fn GetGeopositionAsyncWithAgeAndTimeout(&self, maximumage: &super::super::Foundation::TimeSpan, timeout: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Geoposition>>;
    fn PositionChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Geolocator, PositionChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePositionChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Geolocator, StatusChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeolocator {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeolocator";
}
#[cfg(feature = "implement_exclusive")]
impl IGeolocatorVtbl {
    pub const fn new<Impl: IGeolocatorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeolocatorVtbl {
        unsafe extern "system" fn DesiredAccuracy<Impl: IGeolocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PositionAccuracy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredAccuracy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredAccuracy<Impl: IGeolocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PositionAccuracy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDesiredAccuracy(value).into()
        }
        unsafe extern "system" fn MovementThreshold<Impl: IGeolocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MovementThreshold() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMovementThreshold<Impl: IGeolocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMovementThreshold(value).into()
        }
        unsafe extern "system" fn ReportInterval<Impl: IGeolocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReportInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReportInterval<Impl: IGeolocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetReportInterval(value).into()
        }
        unsafe extern "system" fn LocationStatus<Impl: IGeolocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PositionStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocationStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGeopositionAsync<Impl: IGeolocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGeopositionAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGeopositionAsyncWithAgeAndTimeout<Impl: IGeolocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maximumage: super::super::Foundation::TimeSpan, timeout: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGeopositionAsyncWithAgeAndTimeout(&*(&maximumage as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType), &*(&timeout as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PositionChanged<Impl: IGeolocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PositionChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<Geolocator, PositionChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<Geolocator, PositionChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePositionChanged<Impl: IGeolocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePositionChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StatusChanged<Impl: IGeolocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StatusChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<Geolocator, StatusChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<Geolocator, StatusChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStatusChanged<Impl: IGeolocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveStatusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IGeolocator>,
            base.5,
            DesiredAccuracy::<Impl, OFFSET>,
            SetDesiredAccuracy::<Impl, OFFSET>,
            MovementThreshold::<Impl, OFFSET>,
            SetMovementThreshold::<Impl, OFFSET>,
            ReportInterval::<Impl, OFFSET>,
            SetReportInterval::<Impl, OFFSET>,
            LocationStatus::<Impl, OFFSET>,
            GetGeopositionAsync::<Impl, OFFSET>,
            GetGeopositionAsyncWithAgeAndTimeout::<Impl, OFFSET>,
            PositionChanged::<Impl, OFFSET>,
            RemovePositionChanged::<Impl, OFFSET>,
            StatusChanged::<Impl, OFFSET>,
            RemoveStatusChanged::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeolocator2Impl: Sized {
    fn AllowFallbackToConsentlessPositions(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeolocator2 {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeolocator2";
}
#[cfg(feature = "implement_exclusive")]
impl IGeolocator2Vtbl {
    pub const fn new<Impl: IGeolocator2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeolocator2Vtbl {
        unsafe extern "system" fn AllowFallbackToConsentlessPositions<Impl: IGeolocator2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AllowFallbackToConsentlessPositions().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeolocator2>, base.5, AllowFallbackToConsentlessPositions::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeolocatorStaticsImpl: Sized {
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GeolocationAccessStatus>>;
    fn GetGeopositionHistoryAsync(&self, starttime: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Geoposition>>>;
    fn GetGeopositionHistoryWithDurationAsync(&self, starttime: &super::super::Foundation::DateTime, duration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Geoposition>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeolocatorStatics {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeolocatorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGeolocatorStaticsVtbl {
    pub const fn new<Impl: IGeolocatorStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeolocatorStaticsVtbl {
        unsafe extern "system" fn RequestAccessAsync<Impl: IGeolocatorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestAccessAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGeopositionHistoryAsync<Impl: IGeolocatorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGeopositionHistoryAsync(&*(&starttime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGeopositionHistoryWithDurationAsync<Impl: IGeolocatorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGeopositionHistoryWithDurationAsync(&*(&starttime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType), &*(&duration as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeolocatorStatics>, base.5, RequestAccessAsync::<Impl, OFFSET>, GetGeopositionHistoryAsync::<Impl, OFFSET>, GetGeopositionHistoryWithDurationAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeolocatorStatics2Impl: Sized {
    fn IsDefaultGeopositionRecommended(&self) -> ::windows::core::Result<bool>;
    fn SetDefaultGeoposition(&self, value: &::core::option::Option<super::super::Foundation::IReference<BasicGeoposition>>) -> ::windows::core::Result<()>;
    fn DefaultGeoposition(&self) -> ::windows::core::Result<super::super::Foundation::IReference<BasicGeoposition>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeolocatorStatics2 {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeolocatorStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IGeolocatorStatics2Vtbl {
    pub const fn new<Impl: IGeolocatorStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeolocatorStatics2Vtbl {
        unsafe extern "system" fn IsDefaultGeopositionRecommended<Impl: IGeolocatorStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDefaultGeopositionRecommended() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultGeoposition<Impl: IGeolocatorStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDefaultGeoposition(&*(&value as *const <super::super::Foundation::IReference<BasicGeoposition> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<BasicGeoposition> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DefaultGeoposition<Impl: IGeolocatorStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultGeoposition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeolocatorStatics2>, base.5, IsDefaultGeopositionRecommended::<Impl, OFFSET>, SetDefaultGeoposition::<Impl, OFFSET>, DefaultGeoposition::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeolocatorWithScalarAccuracyImpl: Sized + IGeolocatorImpl {
    fn DesiredAccuracyInMeters(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn SetDesiredAccuracyInMeters(&self, value: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeolocatorWithScalarAccuracy {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeolocatorWithScalarAccuracy";
}
#[cfg(feature = "implement_exclusive")]
impl IGeolocatorWithScalarAccuracyVtbl {
    pub const fn new<Impl: IGeolocatorWithScalarAccuracyImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeolocatorWithScalarAccuracyVtbl {
        unsafe extern "system" fn DesiredAccuracyInMeters<Impl: IGeolocatorWithScalarAccuracyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredAccuracyInMeters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredAccuracyInMeters<Impl: IGeolocatorWithScalarAccuracyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDesiredAccuracyInMeters(&*(&value as *const <super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeolocatorWithScalarAccuracy>, base.5, DesiredAccuracyInMeters::<Impl, OFFSET>, SetDesiredAccuracyInMeters::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeopathImpl: Sized + IGeoshapeImpl {
    fn Positions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<BasicGeoposition>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeopath {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeopath";
}
#[cfg(feature = "implement_exclusive")]
impl IGeopathVtbl {
    pub const fn new<Impl: IGeopathImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeopathVtbl {
        unsafe extern "system" fn Positions<Impl: IGeopathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Positions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeopath>, base.5, Positions::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeopathFactoryImpl: Sized {
    fn Create(&self, positions: &::core::option::Option<super::super::Foundation::Collections::IIterable<BasicGeoposition>>) -> ::windows::core::Result<Geopath>;
    fn CreateWithAltitudeReference(&self, positions: &::core::option::Option<super::super::Foundation::Collections::IIterable<BasicGeoposition>>, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows::core::Result<Geopath>;
    fn CreateWithAltitudeReferenceAndSpatialReference(&self, positions: &::core::option::Option<super::super::Foundation::Collections::IIterable<BasicGeoposition>>, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::core::Result<Geopath>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeopathFactory {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeopathFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGeopathFactoryVtbl {
    pub const fn new<Impl: IGeopathFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeopathFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IGeopathFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, positions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&positions as *const <super::super::Foundation::Collections::IIterable<BasicGeoposition> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<BasicGeoposition> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithAltitudeReference<Impl: IGeopathFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, positions: ::windows::core::RawPtr, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithAltitudeReference(&*(&positions as *const <super::super::Foundation::Collections::IIterable<BasicGeoposition> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<BasicGeoposition> as ::windows::core::DefaultType>::DefaultType), altitudereferencesystem) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithAltitudeReferenceAndSpatialReference<Impl: IGeopathFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, positions: ::windows::core::RawPtr, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithAltitudeReferenceAndSpatialReference(&*(&positions as *const <super::super::Foundation::Collections::IIterable<BasicGeoposition> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<BasicGeoposition> as ::windows::core::DefaultType>::DefaultType), altitudereferencesystem, spatialreferenceid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeopathFactory>, base.5, Create::<Impl, OFFSET>, CreateWithAltitudeReference::<Impl, OFFSET>, CreateWithAltitudeReferenceAndSpatialReference::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeopointImpl: Sized + IGeoshapeImpl {
    fn Position(&self) -> ::windows::core::Result<BasicGeoposition>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeopoint {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeopoint";
}
#[cfg(feature = "implement_exclusive")]
impl IGeopointVtbl {
    pub const fn new<Impl: IGeopointImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeopointVtbl {
        unsafe extern "system" fn Position<Impl: IGeopointImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut BasicGeoposition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeopoint>, base.5, Position::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeopointFactoryImpl: Sized {
    fn Create(&self, position: &BasicGeoposition) -> ::windows::core::Result<Geopoint>;
    fn CreateWithAltitudeReferenceSystem(&self, position: &BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows::core::Result<Geopoint>;
    fn CreateWithAltitudeReferenceSystemAndSpatialReferenceId(&self, position: &BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::core::Result<Geopoint>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeopointFactory {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeopointFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGeopointFactoryVtbl {
    pub const fn new<Impl: IGeopointFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeopointFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IGeopointFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, position: BasicGeoposition, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&position as *const <BasicGeoposition as ::windows::core::Abi>::Abi as *const <BasicGeoposition as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithAltitudeReferenceSystem<Impl: IGeopointFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, position: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithAltitudeReferenceSystem(&*(&position as *const <BasicGeoposition as ::windows::core::Abi>::Abi as *const <BasicGeoposition as ::windows::core::DefaultType>::DefaultType), altitudereferencesystem) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithAltitudeReferenceSystemAndSpatialReferenceId<Impl: IGeopointFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, position: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithAltitudeReferenceSystemAndSpatialReferenceId(&*(&position as *const <BasicGeoposition as ::windows::core::Abi>::Abi as *const <BasicGeoposition as ::windows::core::DefaultType>::DefaultType), altitudereferencesystem, spatialreferenceid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeopointFactory>, base.5, Create::<Impl, OFFSET>, CreateWithAltitudeReferenceSystem::<Impl, OFFSET>, CreateWithAltitudeReferenceSystemAndSpatialReferenceId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeopositionImpl: Sized {
    fn Coordinate(&self) -> ::windows::core::Result<Geocoordinate>;
    fn CivicAddress(&self) -> ::windows::core::Result<CivicAddress>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeoposition {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeoposition";
}
#[cfg(feature = "implement_exclusive")]
impl IGeopositionVtbl {
    pub const fn new<Impl: IGeopositionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeopositionVtbl {
        unsafe extern "system" fn Coordinate<Impl: IGeopositionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Coordinate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CivicAddress<Impl: IGeopositionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CivicAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeoposition>, base.5, Coordinate::<Impl, OFFSET>, CivicAddress::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeoposition2Impl: Sized + IGeopositionImpl {
    fn VenueData(&self) -> ::windows::core::Result<VenueData>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeoposition2 {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeoposition2";
}
#[cfg(feature = "implement_exclusive")]
impl IGeoposition2Vtbl {
    pub const fn new<Impl: IGeoposition2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeoposition2Vtbl {
        unsafe extern "system" fn VenueData<Impl: IGeoposition2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VenueData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeoposition2>, base.5, VenueData::<Impl, OFFSET>)
    }
}
pub trait IGeoshapeImpl: Sized {
    fn GeoshapeType(&self) -> ::windows::core::Result<GeoshapeType>;
    fn SpatialReferenceId(&self) -> ::windows::core::Result<u32>;
    fn AltitudeReferenceSystem(&self) -> ::windows::core::Result<AltitudeReferenceSystem>;
}
impl ::windows::core::RuntimeName for IGeoshape {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeoshape";
}
impl IGeoshapeVtbl {
    pub const fn new<Impl: IGeoshapeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeoshapeVtbl {
        unsafe extern "system" fn GeoshapeType<Impl: IGeoshapeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut GeoshapeType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GeoshapeType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpatialReferenceId<Impl: IGeoshapeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SpatialReferenceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AltitudeReferenceSystem<Impl: IGeoshapeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AltitudeReferenceSystem) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AltitudeReferenceSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeoshape>, base.5, GeoshapeType::<Impl, OFFSET>, SpatialReferenceId::<Impl, OFFSET>, AltitudeReferenceSystem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeovisitImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<Geoposition>;
    fn StateChange(&self) -> ::windows::core::Result<VisitStateChange>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeovisit {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeovisit";
}
#[cfg(feature = "implement_exclusive")]
impl IGeovisitVtbl {
    pub const fn new<Impl: IGeovisitImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeovisitVtbl {
        unsafe extern "system" fn Position<Impl: IGeovisitImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StateChange<Impl: IGeovisitImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut VisitStateChange) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StateChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: IGeovisitImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeovisit>, base.5, Position::<Impl, OFFSET>, StateChange::<Impl, OFFSET>, Timestamp::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeovisitMonitorImpl: Sized {
    fn MonitoringScope(&self) -> ::windows::core::Result<VisitMonitoringScope>;
    fn Start(&self, value: VisitMonitoringScope) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn VisitStateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GeovisitMonitor, GeovisitStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVisitStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeovisitMonitor {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeovisitMonitor";
}
#[cfg(feature = "implement_exclusive")]
impl IGeovisitMonitorVtbl {
    pub const fn new<Impl: IGeovisitMonitorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeovisitMonitorVtbl {
        unsafe extern "system" fn MonitoringScope<Impl: IGeovisitMonitorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut VisitMonitoringScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MonitoringScope() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IGeovisitMonitorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: VisitMonitoringScope) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Start(value).into()
        }
        unsafe extern "system" fn Stop<Impl: IGeovisitMonitorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn VisitStateChanged<Impl: IGeovisitMonitorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VisitStateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<GeovisitMonitor, GeovisitStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<GeovisitMonitor, GeovisitStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVisitStateChanged<Impl: IGeovisitMonitorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveVisitStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeovisitMonitor>, base.5, MonitoringScope::<Impl, OFFSET>, Start::<Impl, OFFSET>, Stop::<Impl, OFFSET>, VisitStateChanged::<Impl, OFFSET>, RemoveVisitStateChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeovisitMonitorStaticsImpl: Sized {
    fn GetLastReportAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Geovisit>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeovisitMonitorStatics {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeovisitMonitorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGeovisitMonitorStaticsVtbl {
    pub const fn new<Impl: IGeovisitMonitorStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeovisitMonitorStaticsVtbl {
        unsafe extern "system" fn GetLastReportAsync<Impl: IGeovisitMonitorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLastReportAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeovisitMonitorStatics>, base.5, GetLastReportAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeovisitStateChangedEventArgsImpl: Sized {
    fn Visit(&self) -> ::windows::core::Result<Geovisit>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeovisitStateChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeovisitStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IGeovisitStateChangedEventArgsVtbl {
    pub const fn new<Impl: IGeovisitStateChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeovisitStateChangedEventArgsVtbl {
        unsafe extern "system" fn Visit<Impl: IGeovisitStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Visit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeovisitStateChangedEventArgs>, base.5, Visit::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeovisitTriggerDetailsImpl: Sized {
    fn ReadReports(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Geovisit>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeovisitTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Geolocation.IGeovisitTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IGeovisitTriggerDetailsVtbl {
    pub const fn new<Impl: IGeovisitTriggerDetailsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeovisitTriggerDetailsVtbl {
        unsafe extern "system" fn ReadReports<Impl: IGeovisitTriggerDetailsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadReports() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeovisitTriggerDetails>, base.5, ReadReports::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPositionChangedEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<Geoposition>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPositionChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Geolocation.IPositionChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPositionChangedEventArgsVtbl {
    pub const fn new<Impl: IPositionChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPositionChangedEventArgsVtbl {
        unsafe extern "system" fn Position<Impl: IPositionChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPositionChangedEventArgs>, base.5, Position::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStatusChangedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<PositionStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Geolocation.IStatusChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IStatusChangedEventArgsVtbl {
    pub const fn new<Impl: IStatusChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStatusChangedEventArgsVtbl {
        unsafe extern "system" fn Status<Impl: IStatusChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PositionStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IStatusChangedEventArgs>, base.5, Status::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVenueDataImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Level(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVenueData {
    const NAME: &'static str = "Windows.Devices.Geolocation.IVenueData";
}
#[cfg(feature = "implement_exclusive")]
impl IVenueDataVtbl {
    pub const fn new<Impl: IVenueDataImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVenueDataVtbl {
        unsafe extern "system" fn Id<Impl: IVenueDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Level<Impl: IVenueDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Level() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVenueData>, base.5, Id::<Impl, OFFSET>, Level::<Impl, OFFSET>)
    }
}
