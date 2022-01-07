#[cfg(feature = "implement_exclusive")]
pub trait IEnhancedWaypointImpl: Sized {
    fn Point(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::Geopoint>;
    fn Kind(&self) -> ::windows::core::Result<WaypointKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEnhancedWaypoint {
    const NAME: &'static str = "Windows.Services.Maps.IEnhancedWaypoint";
}
#[cfg(feature = "implement_exclusive")]
impl IEnhancedWaypointVtbl {
    pub const fn new<Impl: IEnhancedWaypointImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnhancedWaypointVtbl {
        unsafe extern "system" fn Point<Impl: IEnhancedWaypointImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Kind<Impl: IEnhancedWaypointImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut WaypointKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnhancedWaypoint>, base.5, Point::<Impl, OFFSET>, Kind::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEnhancedWaypointFactoryImpl: Sized {
    fn Create(&self, point: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, kind: WaypointKind) -> ::windows::core::Result<EnhancedWaypoint>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEnhancedWaypointFactory {
    const NAME: &'static str = "Windows.Services.Maps.IEnhancedWaypointFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IEnhancedWaypointFactoryVtbl {
    pub const fn new<Impl: IEnhancedWaypointFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEnhancedWaypointFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IEnhancedWaypointFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, point: ::windows::core::RawPtr, kind: WaypointKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&point as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), kind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEnhancedWaypointFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IManeuverWarningImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<ManeuverWarningKind>;
    fn Severity(&self) -> ::windows::core::Result<ManeuverWarningSeverity>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IManeuverWarning {
    const NAME: &'static str = "Windows.Services.Maps.IManeuverWarning";
}
#[cfg(feature = "implement_exclusive")]
impl IManeuverWarningVtbl {
    pub const fn new<Impl: IManeuverWarningImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IManeuverWarningVtbl {
        unsafe extern "system" fn Kind<Impl: IManeuverWarningImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ManeuverWarningKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Severity<Impl: IManeuverWarningImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ManeuverWarningSeverity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Severity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IManeuverWarning>, base.5, Kind::<Impl, OFFSET>, Severity::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapAddressImpl: Sized {
    fn BuildingName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BuildingFloor(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BuildingRoom(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BuildingWing(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StreetNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Street(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Neighborhood(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn District(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Town(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Region(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RegionCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Country(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CountryCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PostCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Continent(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapAddress {
    const NAME: &'static str = "Windows.Services.Maps.IMapAddress";
}
#[cfg(feature = "implement_exclusive")]
impl IMapAddressVtbl {
    pub const fn new<Impl: IMapAddressImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapAddressVtbl {
        unsafe extern "system" fn BuildingName<Impl: IMapAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BuildingName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BuildingFloor<Impl: IMapAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BuildingFloor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BuildingRoom<Impl: IMapAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BuildingRoom() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BuildingWing<Impl: IMapAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BuildingWing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StreetNumber<Impl: IMapAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StreetNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Street<Impl: IMapAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Street() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Neighborhood<Impl: IMapAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Neighborhood() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn District<Impl: IMapAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).District() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Town<Impl: IMapAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Town() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Region<Impl: IMapAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Region() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegionCode<Impl: IMapAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegionCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Country<Impl: IMapAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CountryCode<Impl: IMapAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CountryCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostCode<Impl: IMapAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PostCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Continent<Impl: IMapAddressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Continent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IMapAddress>,
            base.5,
            BuildingName::<Impl, OFFSET>,
            BuildingFloor::<Impl, OFFSET>,
            BuildingRoom::<Impl, OFFSET>,
            BuildingWing::<Impl, OFFSET>,
            StreetNumber::<Impl, OFFSET>,
            Street::<Impl, OFFSET>,
            Neighborhood::<Impl, OFFSET>,
            District::<Impl, OFFSET>,
            Town::<Impl, OFFSET>,
            Region::<Impl, OFFSET>,
            RegionCode::<Impl, OFFSET>,
            Country::<Impl, OFFSET>,
            CountryCode::<Impl, OFFSET>,
            PostCode::<Impl, OFFSET>,
            Continent::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapAddress2Impl: Sized {
    fn FormattedAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapAddress2 {
    const NAME: &'static str = "Windows.Services.Maps.IMapAddress2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapAddress2Vtbl {
    pub const fn new<Impl: IMapAddress2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapAddress2Vtbl {
        unsafe extern "system" fn FormattedAddress<Impl: IMapAddress2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FormattedAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapAddress2>, base.5, FormattedAddress::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapLocationImpl: Sized {
    fn Point(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::Geopoint>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Address(&self) -> ::windows::core::Result<MapAddress>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapLocation {
    const NAME: &'static str = "Windows.Services.Maps.IMapLocation";
}
#[cfg(feature = "implement_exclusive")]
impl IMapLocationVtbl {
    pub const fn new<Impl: IMapLocationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapLocationVtbl {
        unsafe extern "system" fn Point<Impl: IMapLocationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisplayName<Impl: IMapLocationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IMapLocationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Address<Impl: IMapLocationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapLocation>, base.5, Point::<Impl, OFFSET>, DisplayName::<Impl, OFFSET>, Description::<Impl, OFFSET>, Address::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapLocationFinderResultImpl: Sized {
    fn Locations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MapLocation>>;
    fn Status(&self) -> ::windows::core::Result<MapLocationFinderStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapLocationFinderResult {
    const NAME: &'static str = "Windows.Services.Maps.IMapLocationFinderResult";
}
#[cfg(feature = "implement_exclusive")]
impl IMapLocationFinderResultVtbl {
    pub const fn new<Impl: IMapLocationFinderResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapLocationFinderResultVtbl {
        unsafe extern "system" fn Locations<Impl: IMapLocationFinderResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Locations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IMapLocationFinderResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MapLocationFinderStatus) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapLocationFinderResult>, base.5, Locations::<Impl, OFFSET>, Status::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapLocationFinderStaticsImpl: Sized {
    fn FindLocationsAtAsync(&self, querypoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>>;
    fn FindLocationsAsync(&self, searchtext: &::windows::core::HSTRING, referencepoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>>;
    fn FindLocationsWithMaxCountAsync(&self, searchtext: &::windows::core::HSTRING, referencepoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, maxcount: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapLocationFinderStatics {
    const NAME: &'static str = "Windows.Services.Maps.IMapLocationFinderStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapLocationFinderStaticsVtbl {
    pub const fn new<Impl: IMapLocationFinderStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapLocationFinderStaticsVtbl {
        unsafe extern "system" fn FindLocationsAtAsync<Impl: IMapLocationFinderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, querypoint: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindLocationsAtAsync(&*(&querypoint as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindLocationsAsync<Impl: IMapLocationFinderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, searchtext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, referencepoint: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindLocationsAsync(&*(&searchtext as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&referencepoint as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindLocationsWithMaxCountAsync<Impl: IMapLocationFinderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, searchtext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, referencepoint: ::windows::core::RawPtr, maxcount: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindLocationsWithMaxCountAsync(&*(&searchtext as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&referencepoint as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), maxcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapLocationFinderStatics>, base.5, FindLocationsAtAsync::<Impl, OFFSET>, FindLocationsAsync::<Impl, OFFSET>, FindLocationsWithMaxCountAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapLocationFinderStatics2Impl: Sized {
    fn FindLocationsAtWithAccuracyAsync(&self, querypoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, accuracy: MapLocationDesiredAccuracy) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapLocationFinderStatics2 {
    const NAME: &'static str = "Windows.Services.Maps.IMapLocationFinderStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapLocationFinderStatics2Vtbl {
    pub const fn new<Impl: IMapLocationFinderStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapLocationFinderStatics2Vtbl {
        unsafe extern "system" fn FindLocationsAtWithAccuracyAsync<Impl: IMapLocationFinderStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, querypoint: ::windows::core::RawPtr, accuracy: MapLocationDesiredAccuracy, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindLocationsAtWithAccuracyAsync(&*(&querypoint as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), accuracy) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapLocationFinderStatics2>, base.5, FindLocationsAtWithAccuracyAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapManagerStaticsImpl: Sized {
    fn ShowDownloadedMapsUI(&self) -> ::windows::core::Result<()>;
    fn ShowMapsUpdateUI(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapManagerStatics {
    const NAME: &'static str = "Windows.Services.Maps.IMapManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapManagerStaticsVtbl {
    pub const fn new<Impl: IMapManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapManagerStaticsVtbl {
        unsafe extern "system" fn ShowDownloadedMapsUI<Impl: IMapManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ShowDownloadedMapsUI().into()
        }
        unsafe extern "system" fn ShowMapsUpdateUI<Impl: IMapManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ShowMapsUpdateUI().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapManagerStatics>, base.5, ShowDownloadedMapsUI::<Impl, OFFSET>, ShowMapsUpdateUI::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteImpl: Sized {
    fn BoundingBox(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::GeoboundingBox>;
    fn LengthInMeters(&self) -> ::windows::core::Result<f64>;
    fn EstimatedDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Path(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::Geopath>;
    fn Legs(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MapRouteLeg>>;
    fn IsTrafficBased(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapRoute {
    const NAME: &'static str = "Windows.Services.Maps.IMapRoute";
}
#[cfg(feature = "implement_exclusive")]
impl IMapRouteVtbl {
    pub const fn new<Impl: IMapRouteImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapRouteVtbl {
        unsafe extern "system" fn BoundingBox<Impl: IMapRouteImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BoundingBox() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LengthInMeters<Impl: IMapRouteImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LengthInMeters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EstimatedDuration<Impl: IMapRouteImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EstimatedDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Impl: IMapRouteImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Legs<Impl: IMapRouteImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Legs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTrafficBased<Impl: IMapRouteImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsTrafficBased() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapRoute>, base.5, BoundingBox::<Impl, OFFSET>, LengthInMeters::<Impl, OFFSET>, EstimatedDuration::<Impl, OFFSET>, Path::<Impl, OFFSET>, Legs::<Impl, OFFSET>, IsTrafficBased::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRoute2Impl: Sized {
    fn ViolatedRestrictions(&self) -> ::windows::core::Result<MapRouteRestrictions>;
    fn HasBlockedRoads(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapRoute2 {
    const NAME: &'static str = "Windows.Services.Maps.IMapRoute2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapRoute2Vtbl {
    pub const fn new<Impl: IMapRoute2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapRoute2Vtbl {
        unsafe extern "system" fn ViolatedRestrictions<Impl: IMapRoute2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MapRouteRestrictions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ViolatedRestrictions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasBlockedRoads<Impl: IMapRoute2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasBlockedRoads() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapRoute2>, base.5, ViolatedRestrictions::<Impl, OFFSET>, HasBlockedRoads::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRoute3Impl: Sized {
    fn DurationWithoutTraffic(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn TrafficCongestion(&self) -> ::windows::core::Result<TrafficCongestion>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapRoute3 {
    const NAME: &'static str = "Windows.Services.Maps.IMapRoute3";
}
#[cfg(feature = "implement_exclusive")]
impl IMapRoute3Vtbl {
    pub const fn new<Impl: IMapRoute3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapRoute3Vtbl {
        unsafe extern "system" fn DurationWithoutTraffic<Impl: IMapRoute3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DurationWithoutTraffic() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrafficCongestion<Impl: IMapRoute3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TrafficCongestion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrafficCongestion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapRoute3>, base.5, DurationWithoutTraffic::<Impl, OFFSET>, TrafficCongestion::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRoute4Impl: Sized {
    fn IsScenic(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapRoute4 {
    const NAME: &'static str = "Windows.Services.Maps.IMapRoute4";
}
#[cfg(feature = "implement_exclusive")]
impl IMapRoute4Vtbl {
    pub const fn new<Impl: IMapRoute4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapRoute4Vtbl {
        unsafe extern "system" fn IsScenic<Impl: IMapRoute4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsScenic() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapRoute4>, base.5, IsScenic::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteDrivingOptionsImpl: Sized {
    fn MaxAlternateRouteCount(&self) -> ::windows::core::Result<u32>;
    fn SetMaxAlternateRouteCount(&self, value: u32) -> ::windows::core::Result<()>;
    fn InitialHeading(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn SetInitialHeading(&self, value: &::core::option::Option<super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
    fn RouteOptimization(&self) -> ::windows::core::Result<MapRouteOptimization>;
    fn SetRouteOptimization(&self, value: MapRouteOptimization) -> ::windows::core::Result<()>;
    fn RouteRestrictions(&self) -> ::windows::core::Result<MapRouteRestrictions>;
    fn SetRouteRestrictions(&self, value: MapRouteRestrictions) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapRouteDrivingOptions {
    const NAME: &'static str = "Windows.Services.Maps.IMapRouteDrivingOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IMapRouteDrivingOptionsVtbl {
    pub const fn new<Impl: IMapRouteDrivingOptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapRouteDrivingOptionsVtbl {
        unsafe extern "system" fn MaxAlternateRouteCount<Impl: IMapRouteDrivingOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxAlternateRouteCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxAlternateRouteCount<Impl: IMapRouteDrivingOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaxAlternateRouteCount(value).into()
        }
        unsafe extern "system" fn InitialHeading<Impl: IMapRouteDrivingOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InitialHeading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialHeading<Impl: IMapRouteDrivingOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInitialHeading(&*(&value as *const <super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RouteOptimization<Impl: IMapRouteDrivingOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MapRouteOptimization) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RouteOptimization() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRouteOptimization<Impl: IMapRouteDrivingOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MapRouteOptimization) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRouteOptimization(value).into()
        }
        unsafe extern "system" fn RouteRestrictions<Impl: IMapRouteDrivingOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MapRouteRestrictions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RouteRestrictions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRouteRestrictions<Impl: IMapRouteDrivingOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MapRouteRestrictions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRouteRestrictions(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapRouteDrivingOptions>, base.5, MaxAlternateRouteCount::<Impl, OFFSET>, SetMaxAlternateRouteCount::<Impl, OFFSET>, InitialHeading::<Impl, OFFSET>, SetInitialHeading::<Impl, OFFSET>, RouteOptimization::<Impl, OFFSET>, SetRouteOptimization::<Impl, OFFSET>, RouteRestrictions::<Impl, OFFSET>, SetRouteRestrictions::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteDrivingOptions2Impl: Sized {
    fn DepartureTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetDepartureTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapRouteDrivingOptions2 {
    const NAME: &'static str = "Windows.Services.Maps.IMapRouteDrivingOptions2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapRouteDrivingOptions2Vtbl {
    pub const fn new<Impl: IMapRouteDrivingOptions2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapRouteDrivingOptions2Vtbl {
        unsafe extern "system" fn DepartureTime<Impl: IMapRouteDrivingOptions2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DepartureTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDepartureTime<Impl: IMapRouteDrivingOptions2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDepartureTime(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapRouteDrivingOptions2>, base.5, DepartureTime::<Impl, OFFSET>, SetDepartureTime::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteFinderResultImpl: Sized {
    fn Route(&self) -> ::windows::core::Result<MapRoute>;
    fn Status(&self) -> ::windows::core::Result<MapRouteFinderStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapRouteFinderResult {
    const NAME: &'static str = "Windows.Services.Maps.IMapRouteFinderResult";
}
#[cfg(feature = "implement_exclusive")]
impl IMapRouteFinderResultVtbl {
    pub const fn new<Impl: IMapRouteFinderResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapRouteFinderResultVtbl {
        unsafe extern "system" fn Route<Impl: IMapRouteFinderResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Route() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IMapRouteFinderResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MapRouteFinderStatus) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapRouteFinderResult>, base.5, Route::<Impl, OFFSET>, Status::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteFinderResult2Impl: Sized {
    fn AlternateRoutes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MapRoute>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapRouteFinderResult2 {
    const NAME: &'static str = "Windows.Services.Maps.IMapRouteFinderResult2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapRouteFinderResult2Vtbl {
    pub const fn new<Impl: IMapRouteFinderResult2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapRouteFinderResult2Vtbl {
        unsafe extern "system" fn AlternateRoutes<Impl: IMapRouteFinderResult2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AlternateRoutes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapRouteFinderResult2>, base.5, AlternateRoutes::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteFinderStaticsImpl: Sized {
    fn GetDrivingRouteAsync(&self, startpoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, endpoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>;
    fn GetDrivingRouteWithOptimizationAsync(&self, startpoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, endpoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, optimization: MapRouteOptimization) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>;
    fn GetDrivingRouteWithOptimizationAndRestrictionsAsync(&self, startpoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, endpoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>;
    fn GetDrivingRouteWithOptimizationRestrictionsAndHeadingAsync(&self, startpoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, endpoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingindegrees: f64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>;
    fn GetDrivingRouteFromWaypointsAsync(&self, waypoints: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>;
    fn GetDrivingRouteFromWaypointsAndOptimizationAsync(&self, waypoints: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint>>, optimization: MapRouteOptimization) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>;
    fn GetDrivingRouteFromWaypointsOptimizationAndRestrictionsAsync(&self, waypoints: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint>>, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>;
    fn GetDrivingRouteFromWaypointsOptimizationRestrictionsAndHeadingAsync(&self, waypoints: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint>>, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingindegrees: f64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>;
    fn GetWalkingRouteAsync(&self, startpoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, endpoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>;
    fn GetWalkingRouteFromWaypointsAsync(&self, waypoints: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapRouteFinderStatics {
    const NAME: &'static str = "Windows.Services.Maps.IMapRouteFinderStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapRouteFinderStaticsVtbl {
    pub const fn new<Impl: IMapRouteFinderStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapRouteFinderStaticsVtbl {
        unsafe extern "system" fn GetDrivingRouteAsync<Impl: IMapRouteFinderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startpoint: ::windows::core::RawPtr, endpoint: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDrivingRouteAsync(&*(&startpoint as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), &*(&endpoint as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDrivingRouteWithOptimizationAsync<Impl: IMapRouteFinderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startpoint: ::windows::core::RawPtr, endpoint: ::windows::core::RawPtr, optimization: MapRouteOptimization, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDrivingRouteWithOptimizationAsync(&*(&startpoint as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), &*(&endpoint as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), optimization) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDrivingRouteWithOptimizationAndRestrictionsAsync<Impl: IMapRouteFinderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startpoint: ::windows::core::RawPtr, endpoint: ::windows::core::RawPtr, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDrivingRouteWithOptimizationAndRestrictionsAsync(&*(&startpoint as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), &*(&endpoint as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), optimization, restrictions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDrivingRouteWithOptimizationRestrictionsAndHeadingAsync<Impl: IMapRouteFinderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startpoint: ::windows::core::RawPtr, endpoint: ::windows::core::RawPtr, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDrivingRouteWithOptimizationRestrictionsAndHeadingAsync(&*(&startpoint as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), &*(&endpoint as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), optimization, restrictions, headingindegrees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDrivingRouteFromWaypointsAsync<Impl: IMapRouteFinderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, waypoints: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDrivingRouteFromWaypointsAsync(&*(&waypoints as *const <super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDrivingRouteFromWaypointsAndOptimizationAsync<Impl: IMapRouteFinderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, waypoints: ::windows::core::RawPtr, optimization: MapRouteOptimization, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDrivingRouteFromWaypointsAndOptimizationAsync(&*(&waypoints as *const <super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint> as ::windows::core::DefaultType>::DefaultType), optimization) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDrivingRouteFromWaypointsOptimizationAndRestrictionsAsync<Impl: IMapRouteFinderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, waypoints: ::windows::core::RawPtr, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDrivingRouteFromWaypointsOptimizationAndRestrictionsAsync(&*(&waypoints as *const <super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint> as ::windows::core::DefaultType>::DefaultType), optimization, restrictions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDrivingRouteFromWaypointsOptimizationRestrictionsAndHeadingAsync<Impl: IMapRouteFinderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, waypoints: ::windows::core::RawPtr, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingindegrees: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDrivingRouteFromWaypointsOptimizationRestrictionsAndHeadingAsync(&*(&waypoints as *const <super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint> as ::windows::core::DefaultType>::DefaultType), optimization, restrictions, headingindegrees) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWalkingRouteAsync<Impl: IMapRouteFinderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startpoint: ::windows::core::RawPtr, endpoint: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetWalkingRouteAsync(&*(&startpoint as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), &*(&endpoint as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWalkingRouteFromWaypointsAsync<Impl: IMapRouteFinderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, waypoints: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetWalkingRouteFromWaypointsAsync(&*(&waypoints as *const <super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IMapRouteFinderStatics>,
            base.5,
            GetDrivingRouteAsync::<Impl, OFFSET>,
            GetDrivingRouteWithOptimizationAsync::<Impl, OFFSET>,
            GetDrivingRouteWithOptimizationAndRestrictionsAsync::<Impl, OFFSET>,
            GetDrivingRouteWithOptimizationRestrictionsAndHeadingAsync::<Impl, OFFSET>,
            GetDrivingRouteFromWaypointsAsync::<Impl, OFFSET>,
            GetDrivingRouteFromWaypointsAndOptimizationAsync::<Impl, OFFSET>,
            GetDrivingRouteFromWaypointsOptimizationAndRestrictionsAsync::<Impl, OFFSET>,
            GetDrivingRouteFromWaypointsOptimizationRestrictionsAndHeadingAsync::<Impl, OFFSET>,
            GetWalkingRouteAsync::<Impl, OFFSET>,
            GetWalkingRouteFromWaypointsAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteFinderStatics2Impl: Sized {
    fn GetDrivingRouteWithOptionsAsync(&self, startpoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, endpoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, options: &::core::option::Option<MapRouteDrivingOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapRouteFinderStatics2 {
    const NAME: &'static str = "Windows.Services.Maps.IMapRouteFinderStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapRouteFinderStatics2Vtbl {
    pub const fn new<Impl: IMapRouteFinderStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapRouteFinderStatics2Vtbl {
        unsafe extern "system" fn GetDrivingRouteWithOptionsAsync<Impl: IMapRouteFinderStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startpoint: ::windows::core::RawPtr, endpoint: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDrivingRouteWithOptionsAsync(
                &*(&startpoint as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType),
                &*(&endpoint as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType),
                &*(&options as *const <MapRouteDrivingOptions as ::windows::core::Abi>::Abi as *const <MapRouteDrivingOptions as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapRouteFinderStatics2>, base.5, GetDrivingRouteWithOptionsAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteFinderStatics3Impl: Sized {
    fn GetDrivingRouteFromEnhancedWaypointsAsync(&self, waypoints: &::core::option::Option<super::super::Foundation::Collections::IIterable<EnhancedWaypoint>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>;
    fn GetDrivingRouteFromEnhancedWaypointsWithOptionsAsync(&self, waypoints: &::core::option::Option<super::super::Foundation::Collections::IIterable<EnhancedWaypoint>>, options: &::core::option::Option<MapRouteDrivingOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapRouteFinderStatics3 {
    const NAME: &'static str = "Windows.Services.Maps.IMapRouteFinderStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IMapRouteFinderStatics3Vtbl {
    pub const fn new<Impl: IMapRouteFinderStatics3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapRouteFinderStatics3Vtbl {
        unsafe extern "system" fn GetDrivingRouteFromEnhancedWaypointsAsync<Impl: IMapRouteFinderStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, waypoints: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDrivingRouteFromEnhancedWaypointsAsync(&*(&waypoints as *const <super::super::Foundation::Collections::IIterable<EnhancedWaypoint> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<EnhancedWaypoint> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDrivingRouteFromEnhancedWaypointsWithOptionsAsync<Impl: IMapRouteFinderStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, waypoints: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDrivingRouteFromEnhancedWaypointsWithOptionsAsync(&*(&waypoints as *const <super::super::Foundation::Collections::IIterable<EnhancedWaypoint> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<EnhancedWaypoint> as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <MapRouteDrivingOptions as ::windows::core::Abi>::Abi as *const <MapRouteDrivingOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapRouteFinderStatics3>, base.5, GetDrivingRouteFromEnhancedWaypointsAsync::<Impl, OFFSET>, GetDrivingRouteFromEnhancedWaypointsWithOptionsAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteLegImpl: Sized {
    fn BoundingBox(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::GeoboundingBox>;
    fn Path(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::Geopath>;
    fn LengthInMeters(&self) -> ::windows::core::Result<f64>;
    fn EstimatedDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Maneuvers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MapRouteManeuver>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapRouteLeg {
    const NAME: &'static str = "Windows.Services.Maps.IMapRouteLeg";
}
#[cfg(feature = "implement_exclusive")]
impl IMapRouteLegVtbl {
    pub const fn new<Impl: IMapRouteLegImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapRouteLegVtbl {
        unsafe extern "system" fn BoundingBox<Impl: IMapRouteLegImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BoundingBox() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Impl: IMapRouteLegImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LengthInMeters<Impl: IMapRouteLegImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LengthInMeters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EstimatedDuration<Impl: IMapRouteLegImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EstimatedDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Maneuvers<Impl: IMapRouteLegImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Maneuvers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapRouteLeg>, base.5, BoundingBox::<Impl, OFFSET>, Path::<Impl, OFFSET>, LengthInMeters::<Impl, OFFSET>, EstimatedDuration::<Impl, OFFSET>, Maneuvers::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteLeg2Impl: Sized {
    fn DurationWithoutTraffic(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn TrafficCongestion(&self) -> ::windows::core::Result<TrafficCongestion>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapRouteLeg2 {
    const NAME: &'static str = "Windows.Services.Maps.IMapRouteLeg2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapRouteLeg2Vtbl {
    pub const fn new<Impl: IMapRouteLeg2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapRouteLeg2Vtbl {
        unsafe extern "system" fn DurationWithoutTraffic<Impl: IMapRouteLeg2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DurationWithoutTraffic() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrafficCongestion<Impl: IMapRouteLeg2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TrafficCongestion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrafficCongestion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapRouteLeg2>, base.5, DurationWithoutTraffic::<Impl, OFFSET>, TrafficCongestion::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteManeuverImpl: Sized {
    fn StartingPoint(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::Geopoint>;
    fn LengthInMeters(&self) -> ::windows::core::Result<f64>;
    fn InstructionText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Kind(&self) -> ::windows::core::Result<MapRouteManeuverKind>;
    fn ExitNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ManeuverNotices(&self) -> ::windows::core::Result<MapManeuverNotices>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapRouteManeuver {
    const NAME: &'static str = "Windows.Services.Maps.IMapRouteManeuver";
}
#[cfg(feature = "implement_exclusive")]
impl IMapRouteManeuverVtbl {
    pub const fn new<Impl: IMapRouteManeuverImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapRouteManeuverVtbl {
        unsafe extern "system" fn StartingPoint<Impl: IMapRouteManeuverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartingPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LengthInMeters<Impl: IMapRouteManeuverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LengthInMeters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstructionText<Impl: IMapRouteManeuverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InstructionText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Kind<Impl: IMapRouteManeuverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MapRouteManeuverKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExitNumber<Impl: IMapRouteManeuverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExitNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManeuverNotices<Impl: IMapRouteManeuverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MapManeuverNotices) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ManeuverNotices() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapRouteManeuver>, base.5, StartingPoint::<Impl, OFFSET>, LengthInMeters::<Impl, OFFSET>, InstructionText::<Impl, OFFSET>, Kind::<Impl, OFFSET>, ExitNumber::<Impl, OFFSET>, ManeuverNotices::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteManeuver2Impl: Sized {
    fn StartHeading(&self) -> ::windows::core::Result<f64>;
    fn EndHeading(&self) -> ::windows::core::Result<f64>;
    fn StreetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapRouteManeuver2 {
    const NAME: &'static str = "Windows.Services.Maps.IMapRouteManeuver2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapRouteManeuver2Vtbl {
    pub const fn new<Impl: IMapRouteManeuver2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapRouteManeuver2Vtbl {
        unsafe extern "system" fn StartHeading<Impl: IMapRouteManeuver2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartHeading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndHeading<Impl: IMapRouteManeuver2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndHeading() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StreetName<Impl: IMapRouteManeuver2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StreetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapRouteManeuver2>, base.5, StartHeading::<Impl, OFFSET>, EndHeading::<Impl, OFFSET>, StreetName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteManeuver3Impl: Sized {
    fn Warnings(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ManeuverWarning>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapRouteManeuver3 {
    const NAME: &'static str = "Windows.Services.Maps.IMapRouteManeuver3";
}
#[cfg(feature = "implement_exclusive")]
impl IMapRouteManeuver3Vtbl {
    pub const fn new<Impl: IMapRouteManeuver3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapRouteManeuver3Vtbl {
        unsafe extern "system" fn Warnings<Impl: IMapRouteManeuver3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Warnings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapRouteManeuver3>, base.5, Warnings::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapServiceStaticsImpl: Sized {
    fn SetServiceToken(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ServiceToken(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapServiceStatics {
    const NAME: &'static str = "Windows.Services.Maps.IMapServiceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMapServiceStaticsVtbl {
    pub const fn new<Impl: IMapServiceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapServiceStaticsVtbl {
        unsafe extern "system" fn SetServiceToken<Impl: IMapServiceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetServiceToken(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ServiceToken<Impl: IMapServiceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServiceToken() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapServiceStatics>, base.5, SetServiceToken::<Impl, OFFSET>, ServiceToken::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapServiceStatics2Impl: Sized {
    fn WorldViewRegionCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapServiceStatics2 {
    const NAME: &'static str = "Windows.Services.Maps.IMapServiceStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IMapServiceStatics2Vtbl {
    pub const fn new<Impl: IMapServiceStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapServiceStatics2Vtbl {
        unsafe extern "system" fn WorldViewRegionCode<Impl: IMapServiceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WorldViewRegionCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapServiceStatics2>, base.5, WorldViewRegionCode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapServiceStatics3Impl: Sized {
    fn DataAttributions(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapServiceStatics3 {
    const NAME: &'static str = "Windows.Services.Maps.IMapServiceStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IMapServiceStatics3Vtbl {
    pub const fn new<Impl: IMapServiceStatics3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapServiceStatics3Vtbl {
        unsafe extern "system" fn DataAttributions<Impl: IMapServiceStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DataAttributions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapServiceStatics3>, base.5, DataAttributions::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapServiceStatics4Impl: Sized {
    fn SetDataUsagePreference(&self, value: MapServiceDataUsagePreference) -> ::windows::core::Result<()>;
    fn DataUsagePreference(&self) -> ::windows::core::Result<MapServiceDataUsagePreference>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapServiceStatics4 {
    const NAME: &'static str = "Windows.Services.Maps.IMapServiceStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IMapServiceStatics4Vtbl {
    pub const fn new<Impl: IMapServiceStatics4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapServiceStatics4Vtbl {
        unsafe extern "system" fn SetDataUsagePreference<Impl: IMapServiceStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MapServiceDataUsagePreference) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDataUsagePreference(value).into()
        }
        unsafe extern "system" fn DataUsagePreference<Impl: IMapServiceStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MapServiceDataUsagePreference) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DataUsagePreference() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapServiceStatics4>, base.5, SetDataUsagePreference::<Impl, OFFSET>, DataUsagePreference::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaceInfoImpl: Sized {
    fn Show(&self, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn ShowWithPreferredPlacement(&self, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<()>;
    fn Identifier(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Geoshape(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::IGeoshape>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlaceInfo {
    const NAME: &'static str = "Windows.Services.Maps.IPlaceInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IPlaceInfoVtbl {
    pub const fn new<Impl: IPlaceInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPlaceInfoVtbl {
        unsafe extern "system" fn Show<Impl: IPlaceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Show(&*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShowWithPreferredPlacement<Impl: IPlaceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ShowWithPreferredPlacement(&*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), preferredplacement).into()
        }
        unsafe extern "system" fn Identifier<Impl: IPlaceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Identifier() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IPlaceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayAddress<Impl: IPlaceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Geoshape<Impl: IPlaceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Geoshape() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPlaceInfo>, base.5, Show::<Impl, OFFSET>, ShowWithPreferredPlacement::<Impl, OFFSET>, Identifier::<Impl, OFFSET>, DisplayName::<Impl, OFFSET>, DisplayAddress::<Impl, OFFSET>, Geoshape::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaceInfoCreateOptionsImpl: Sized {
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlaceInfoCreateOptions {
    const NAME: &'static str = "Windows.Services.Maps.IPlaceInfoCreateOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IPlaceInfoCreateOptionsVtbl {
    pub const fn new<Impl: IPlaceInfoCreateOptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPlaceInfoCreateOptionsVtbl {
        unsafe extern "system" fn SetDisplayName<Impl: IPlaceInfoCreateOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayName<Impl: IPlaceInfoCreateOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayAddress<Impl: IPlaceInfoCreateOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDisplayAddress(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayAddress<Impl: IPlaceInfoCreateOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPlaceInfoCreateOptions>, base.5, SetDisplayName::<Impl, OFFSET>, DisplayName::<Impl, OFFSET>, SetDisplayAddress::<Impl, OFFSET>, DisplayAddress::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaceInfoStaticsImpl: Sized {
    fn Create(&self, referencepoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<PlaceInfo>;
    fn CreateWithGeopointAndOptions(&self, referencepoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, options: &::core::option::Option<PlaceInfoCreateOptions>) -> ::windows::core::Result<PlaceInfo>;
    fn CreateFromIdentifier(&self, identifier: &::windows::core::HSTRING) -> ::windows::core::Result<PlaceInfo>;
    fn CreateFromIdentifierWithOptions(&self, identifier: &::windows::core::HSTRING, defaultpoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, options: &::core::option::Option<PlaceInfoCreateOptions>) -> ::windows::core::Result<PlaceInfo>;
    fn CreateFromMapLocation(&self, location: &::core::option::Option<MapLocation>) -> ::windows::core::Result<PlaceInfo>;
    fn IsShowSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlaceInfoStatics {
    const NAME: &'static str = "Windows.Services.Maps.IPlaceInfoStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPlaceInfoStaticsVtbl {
    pub const fn new<Impl: IPlaceInfoStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPlaceInfoStaticsVtbl {
        unsafe extern "system" fn Create<Impl: IPlaceInfoStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, referencepoint: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&referencepoint as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithGeopointAndOptions<Impl: IPlaceInfoStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, referencepoint: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithGeopointAndOptions(&*(&referencepoint as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <PlaceInfoCreateOptions as ::windows::core::Abi>::Abi as *const <PlaceInfoCreateOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromIdentifier<Impl: IPlaceInfoStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromIdentifier(&*(&identifier as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromIdentifierWithOptions<Impl: IPlaceInfoStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, identifier: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, defaultpoint: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromIdentifierWithOptions(
                &*(&identifier as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&defaultpoint as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::Abi>::Abi as *const <super::super::Devices::Geolocation::Geopoint as ::windows::core::DefaultType>::DefaultType),
                &*(&options as *const <PlaceInfoCreateOptions as ::windows::core::Abi>::Abi as *const <PlaceInfoCreateOptions as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromMapLocation<Impl: IPlaceInfoStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromMapLocation(&*(&location as *const <MapLocation as ::windows::core::Abi>::Abi as *const <MapLocation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsShowSupported<Impl: IPlaceInfoStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsShowSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPlaceInfoStatics>, base.5, Create::<Impl, OFFSET>, CreateWithGeopointAndOptions::<Impl, OFFSET>, CreateFromIdentifier::<Impl, OFFSET>, CreateFromIdentifierWithOptions::<Impl, OFFSET>, CreateFromMapLocation::<Impl, OFFSET>, IsShowSupported::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaceInfoStatics2Impl: Sized {
    fn CreateFromAddress(&self, displayaddress: &::windows::core::HSTRING) -> ::windows::core::Result<PlaceInfo>;
    fn CreateFromAddressWithName(&self, displayaddress: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<PlaceInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlaceInfoStatics2 {
    const NAME: &'static str = "Windows.Services.Maps.IPlaceInfoStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IPlaceInfoStatics2Vtbl {
    pub const fn new<Impl: IPlaceInfoStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPlaceInfoStatics2Vtbl {
        unsafe extern "system" fn CreateFromAddress<Impl: IPlaceInfoStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, displayaddress: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromAddress(&*(&displayaddress as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromAddressWithName<Impl: IPlaceInfoStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, displayaddress: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromAddressWithName(&*(&displayaddress as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPlaceInfoStatics2>, base.5, CreateFromAddress::<Impl, OFFSET>, CreateFromAddressWithName::<Impl, OFFSET>)
    }
}
