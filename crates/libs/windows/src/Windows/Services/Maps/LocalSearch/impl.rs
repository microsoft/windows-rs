#[cfg(feature = "implement_exclusive")]
pub trait ILocalCategoriesStaticsImpl: Sized {
    fn BankAndCreditUnions(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EatDrink(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Hospitals(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HotelsAndMotels(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn All(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Parking(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SeeDo(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Shop(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILocalCategoriesStatics {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.ILocalCategoriesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ILocalCategoriesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocalCategoriesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILocalCategoriesStaticsVtbl {
        unsafe extern "system" fn BankAndCreditUnions<Impl: ILocalCategoriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BankAndCreditUnions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EatDrink<Impl: ILocalCategoriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EatDrink() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Hospitals<Impl: ILocalCategoriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Hospitals() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HotelsAndMotels<Impl: ILocalCategoriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HotelsAndMotels() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn All<Impl: ILocalCategoriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).All() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parking<Impl: ILocalCategoriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parking() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeeDo<Impl: ILocalCategoriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SeeDo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shop<Impl: ILocalCategoriesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shop() {
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
            ::windows::core::GetRuntimeClassName::<ILocalCategoriesStatics>,
            ::windows::core::GetTrustLevel,
            BankAndCreditUnions::<Impl, IMPL_OFFSET>,
            EatDrink::<Impl, IMPL_OFFSET>,
            Hospitals::<Impl, IMPL_OFFSET>,
            HotelsAndMotels::<Impl, IMPL_OFFSET>,
            All::<Impl, IMPL_OFFSET>,
            Parking::<Impl, IMPL_OFFSET>,
            SeeDo::<Impl, IMPL_OFFSET>,
            Shop::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILocalCategoriesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
pub trait ILocalLocationImpl: Sized {
    fn Address(&self) -> ::windows::core::Result<super::MapAddress>;
    fn Identifier(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Point(&self) -> ::windows::core::Result<super::super::super::Devices::Geolocation::Geopoint>;
    fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DataAttribution(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILocalLocation {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.ILocalLocation";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "implement_exclusive"))]
impl ILocalLocationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocalLocationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILocalLocationVtbl {
        unsafe extern "system" fn Address<Impl: ILocalLocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Identifier<Impl: ILocalLocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Identifier() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: ILocalLocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisplayName<Impl: ILocalLocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Point<Impl: ILocalLocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PhoneNumber<Impl: ILocalLocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataAttribution<Impl: ILocalLocationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataAttribution() {
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
            ::windows::core::GetRuntimeClassName::<ILocalLocation>,
            ::windows::core::GetTrustLevel,
            Address::<Impl, IMPL_OFFSET>,
            Identifier::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            DisplayName::<Impl, IMPL_OFFSET>,
            Point::<Impl, IMPL_OFFSET>,
            PhoneNumber::<Impl, IMPL_OFFSET>,
            DataAttribution::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILocalLocation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ILocalLocation2Impl: Sized {
    fn Category(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RatingInfo(&self) -> ::windows::core::Result<LocalLocationRatingInfo>;
    fn HoursOfOperation(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<LocalLocationHoursOfOperationItem>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILocalLocation2 {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.ILocalLocation2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ILocalLocation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocalLocation2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILocalLocation2Vtbl {
        unsafe extern "system" fn Category<Impl: ILocalLocation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Category() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RatingInfo<Impl: ILocalLocation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RatingInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HoursOfOperation<Impl: ILocalLocation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HoursOfOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILocalLocation2>, ::windows::core::GetTrustLevel, Category::<Impl, IMPL_OFFSET>, RatingInfo::<Impl, IMPL_OFFSET>, HoursOfOperation::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILocalLocation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ILocalLocationFinderResultImpl: Sized {
    fn LocalLocations(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<LocalLocation>>;
    fn Status(&self) -> ::windows::core::Result<LocalLocationFinderStatus>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILocalLocationFinderResult {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.ILocalLocationFinderResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ILocalLocationFinderResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocalLocationFinderResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILocalLocationFinderResultVtbl {
        unsafe extern "system" fn LocalLocations<Impl: ILocalLocationFinderResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalLocations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: ILocalLocationFinderResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LocalLocationFinderStatus) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILocalLocationFinderResult>, ::windows::core::GetTrustLevel, LocalLocations::<Impl, IMPL_OFFSET>, Status::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILocalLocationFinderResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILocalLocationFinderStaticsImpl: Sized {
    fn FindLocalLocationsAsync(&self, searchterm: &::windows::core::HSTRING, searcharea: &::core::option::Option<super::super::super::Devices::Geolocation::Geocircle>, localcategory: &::windows::core::HSTRING, maxresults: u32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LocalLocationFinderResult>>;
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILocalLocationFinderStatics {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.ILocalLocationFinderStatics";
}
#[cfg(all(feature = "Devices_Geolocation", feature = "Foundation", feature = "implement_exclusive"))]
impl ILocalLocationFinderStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocalLocationFinderStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILocalLocationFinderStaticsVtbl {
        unsafe extern "system" fn FindLocalLocationsAsync<Impl: ILocalLocationFinderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, searchterm: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, searcharea: ::windows::core::RawPtr, localcategory: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, maxresults: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindLocalLocationsAsync(
                &*(&searchterm as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&searcharea as *const <super::super::super::Devices::Geolocation::Geocircle as ::windows::core::Abi>::Abi as *const <super::super::super::Devices::Geolocation::Geocircle as ::windows::core::DefaultType>::DefaultType),
                &*(&localcategory as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                maxresults,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILocalLocationFinderStatics>, ::windows::core::GetTrustLevel, FindLocalLocationsAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILocalLocationFinderStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Globalization", feature = "implement_exclusive"))]
pub trait ILocalLocationHoursOfOperationItemImpl: Sized {
    fn Day(&self) -> ::windows::core::Result<super::super::super::Globalization::DayOfWeek>;
    fn Start(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Span(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "Globalization", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILocalLocationHoursOfOperationItem {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.ILocalLocationHoursOfOperationItem";
}
#[cfg(all(feature = "Foundation", feature = "Globalization", feature = "implement_exclusive"))]
impl ILocalLocationHoursOfOperationItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocalLocationHoursOfOperationItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILocalLocationHoursOfOperationItemVtbl {
        unsafe extern "system" fn Day<Impl: ILocalLocationHoursOfOperationItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Globalization::DayOfWeek) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Day() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: ILocalLocationHoursOfOperationItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Start() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Span<Impl: ILocalLocationHoursOfOperationItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Span() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILocalLocationHoursOfOperationItem>, ::windows::core::GetTrustLevel, Day::<Impl, IMPL_OFFSET>, Start::<Impl, IMPL_OFFSET>, Span::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILocalLocationHoursOfOperationItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILocalLocationRatingInfoImpl: Sized {
    fn AggregateRating(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>>;
    fn RatingCount(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>>;
    fn ProviderIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILocalLocationRatingInfo {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.ILocalLocationRatingInfo";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILocalLocationRatingInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocalLocationRatingInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILocalLocationRatingInfoVtbl {
        unsafe extern "system" fn AggregateRating<Impl: ILocalLocationRatingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AggregateRating() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RatingCount<Impl: ILocalLocationRatingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RatingCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderIdentifier<Impl: ILocalLocationRatingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILocalLocationRatingInfo>, ::windows::core::GetTrustLevel, AggregateRating::<Impl, IMPL_OFFSET>, RatingCount::<Impl, IMPL_OFFSET>, ProviderIdentifier::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILocalLocationRatingInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaceInfoHelperStaticsImpl: Sized {
    fn CreateFromLocalLocation(&self, location: &::core::option::Option<LocalLocation>) -> ::windows::core::Result<super::PlaceInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPlaceInfoHelperStatics {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.IPlaceInfoHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPlaceInfoHelperStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlaceInfoHelperStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlaceInfoHelperStaticsVtbl {
        unsafe extern "system" fn CreateFromLocalLocation<Impl: IPlaceInfoHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromLocalLocation(&*(&location as *const <LocalLocation as ::windows::core::Abi>::Abi as *const <LocalLocation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlaceInfoHelperStatics>, ::windows::core::GetTrustLevel, CreateFromLocalLocation::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlaceInfoHelperStatics as ::windows::core::Interface>::IID
    }
}
