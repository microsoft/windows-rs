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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocalCategoriesStaticsImpl, const OFFSET: isize>() -> ILocalCategoriesStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILocalCategoriesStatics>, ::windows::core::GetTrustLevel, BankAndCreditUnions::<Impl, OFFSET>, EatDrink::<Impl, OFFSET>, Hospitals::<Impl, OFFSET>, HotelsAndMotels::<Impl, OFFSET>, All::<Impl, OFFSET>, Parking::<Impl, OFFSET>, SeeDo::<Impl, OFFSET>, Shop::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILocalLocationImpl: Sized {
    fn Address(&self) -> ::windows::core::Result<super::MapAddress>;
    fn Identifier(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Point(&self) -> ::windows::core::Result<super::super::super::Devices::Geolocation::Geopoint>;
    fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DataAttribution(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILocalLocation {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.ILocalLocation";
}
#[cfg(feature = "implement_exclusive")]
impl ILocalLocationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocalLocationImpl, const OFFSET: isize>() -> ILocalLocationVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILocalLocation>, ::windows::core::GetTrustLevel, Address::<Impl, OFFSET>, Identifier::<Impl, OFFSET>, Description::<Impl, OFFSET>, DisplayName::<Impl, OFFSET>, Point::<Impl, OFFSET>, PhoneNumber::<Impl, OFFSET>, DataAttribution::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILocalLocation2Impl: Sized {
    fn Category(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RatingInfo(&self) -> ::windows::core::Result<LocalLocationRatingInfo>;
    fn HoursOfOperation(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<LocalLocationHoursOfOperationItem>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILocalLocation2 {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.ILocalLocation2";
}
#[cfg(feature = "implement_exclusive")]
impl ILocalLocation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocalLocation2Impl, const OFFSET: isize>() -> ILocalLocation2Vtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILocalLocation2>, ::windows::core::GetTrustLevel, Category::<Impl, OFFSET>, RatingInfo::<Impl, OFFSET>, HoursOfOperation::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILocalLocationFinderResultImpl: Sized {
    fn LocalLocations(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<LocalLocation>>;
    fn Status(&self) -> ::windows::core::Result<LocalLocationFinderStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILocalLocationFinderResult {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.ILocalLocationFinderResult";
}
#[cfg(feature = "implement_exclusive")]
impl ILocalLocationFinderResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocalLocationFinderResultImpl, const OFFSET: isize>() -> ILocalLocationFinderResultVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILocalLocationFinderResult>, ::windows::core::GetTrustLevel, LocalLocations::<Impl, OFFSET>, Status::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILocalLocationFinderStaticsImpl: Sized {
    fn FindLocalLocationsAsync(&self, searchterm: &::windows::core::HSTRING, searcharea: &::core::option::Option<super::super::super::Devices::Geolocation::Geocircle>, localcategory: &::windows::core::HSTRING, maxresults: u32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LocalLocationFinderResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILocalLocationFinderStatics {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.ILocalLocationFinderStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ILocalLocationFinderStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocalLocationFinderStaticsImpl, const OFFSET: isize>() -> ILocalLocationFinderStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILocalLocationFinderStatics>, ::windows::core::GetTrustLevel, FindLocalLocationsAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILocalLocationHoursOfOperationItemImpl: Sized {
    fn Day(&self) -> ::windows::core::Result<super::super::super::Globalization::DayOfWeek>;
    fn Start(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Span(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILocalLocationHoursOfOperationItem {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.ILocalLocationHoursOfOperationItem";
}
#[cfg(feature = "implement_exclusive")]
impl ILocalLocationHoursOfOperationItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocalLocationHoursOfOperationItemImpl, const OFFSET: isize>() -> ILocalLocationHoursOfOperationItemVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILocalLocationHoursOfOperationItem>, ::windows::core::GetTrustLevel, Day::<Impl, OFFSET>, Start::<Impl, OFFSET>, Span::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILocalLocationRatingInfoImpl: Sized {
    fn AggregateRating(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>>;
    fn RatingCount(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>>;
    fn ProviderIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILocalLocationRatingInfo {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.ILocalLocationRatingInfo";
}
#[cfg(feature = "implement_exclusive")]
impl ILocalLocationRatingInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocalLocationRatingInfoImpl, const OFFSET: isize>() -> ILocalLocationRatingInfoVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILocalLocationRatingInfo>, ::windows::core::GetTrustLevel, AggregateRating::<Impl, OFFSET>, RatingCount::<Impl, OFFSET>, ProviderIdentifier::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlaceInfoHelperStaticsImpl, const OFFSET: isize>() -> IPlaceInfoHelperStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlaceInfoHelperStatics>, ::windows::core::GetTrustLevel, CreateFromLocalLocation::<Impl, OFFSET>)
    }
}
