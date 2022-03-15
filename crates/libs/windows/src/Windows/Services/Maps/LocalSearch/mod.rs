#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocalCategoriesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILocalCategoriesStatics {
    type Vtable = ILocalCategoriesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf49399f5_8261_4321_9974_ef92d49a8dca);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalCategoriesStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub BankAndCreditUnions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub EatDrink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Hospitals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HotelsAndMotels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub All: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Parking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SeeDo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Shop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocalLocation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILocalLocation {
    type Vtable = ILocalLocation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb0fe9ab_4502_4f2c_94a9_0d60de0e2163);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalLocation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Identifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Geolocation")]
    pub Point: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    Point: usize,
    pub PhoneNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DataAttribution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocalLocation2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILocalLocation2 {
    type Vtable = ILocalLocation2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e9e307c_ecb5_4ffc_bb8c_ba50ba8c2dc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalLocation2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Category: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RatingInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub HoursOfOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    HoursOfOperation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocalLocationFinderResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILocalLocationFinderResult {
    type Vtable = ILocalLocationFinderResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd09b6cc6_f338_4191_9fd8_5440b9a68f52);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalLocationFinderResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub LocalLocations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LocalLocations: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LocalLocationFinderStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocalLocationFinderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILocalLocationFinderStatics {
    type Vtable = ILocalLocationFinderStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2ef7344_a0de_48ca_81a8_07c7dcfd37ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalLocationFinderStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub FindLocalLocationsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, searchterm: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, searcharea: ::windows::core::RawPtr, localcategory: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, maxresults: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Geolocation", feature = "Foundation")))]
    FindLocalLocationsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocalLocationHoursOfOperationItem(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILocalLocationHoursOfOperationItem {
    type Vtable = ILocalLocationHoursOfOperationItem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23548c72_a1c7_43f1_a4f0_1091c39ec640);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalLocationHoursOfOperationItem_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Globalization")]
    pub Day: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Globalization::DayOfWeek) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    Day: usize,
    #[cfg(feature = "Foundation")]
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Start: usize,
    #[cfg(feature = "Foundation")]
    pub Span: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Span: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocalLocationRatingInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILocalLocationRatingInfo {
    type Vtable = ILocalLocationRatingInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb1dab56_3354_4311_8bc0_a2d4d5eb806e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocalLocationRatingInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub AggregateRating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AggregateRating: usize,
    #[cfg(feature = "Foundation")]
    pub RatingCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RatingCount: usize,
    pub ProviderIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaceInfoHelperStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPlaceInfoHelperStatics {
    type Vtable = IPlaceInfoHelperStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd1ca9a7_a9c6_491b_bc09_e80fcea48ee6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaceInfoHelperStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateFromLocalLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, location: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
pub struct LocalCategories {}
impl LocalCategories {
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
    pub fn BankAndCreditUnions() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BankAndCreditUnions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
    pub fn EatDrink() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EatDrink)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
    pub fn Hospitals() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Hospitals)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
    pub fn HotelsAndMotels() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HotelsAndMotels)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
    pub fn All() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).All)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
    pub fn Parking() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Parking)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
    pub fn SeeDo() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SeeDo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
    pub fn Shop() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILocalCategoriesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Shop)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILocalCategoriesStatics<R, F: FnOnce(&ILocalCategoriesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LocalCategories, ILocalCategoriesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for LocalCategories {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalCategories";
}
#[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
#[repr(transparent)]
pub struct LocalLocation(::windows::core::IUnknown);
impl LocalLocation {
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
    pub fn Address(&self) -> ::windows::core::Result<super::MapAddress> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Address)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::MapAddress>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
    pub fn Identifier(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Identifier)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`, `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn Point(&self) -> ::windows::core::Result<super::super::super::Devices::Geolocation::Geopoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Point)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Geolocation::Geopoint>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
    pub fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PhoneNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
    pub fn DataAttribution(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DataAttribution)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
    pub fn Category(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILocalLocation2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Category)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
    pub fn RatingInfo(&self) -> ::windows::core::Result<LocalLocationRatingInfo> {
        let this = &::windows::core::Interface::cast::<ILocalLocation2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RatingInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LocalLocationRatingInfo>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HoursOfOperation(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<LocalLocationHoursOfOperationItem>> {
        let this = &::windows::core::Interface::cast::<ILocalLocation2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HoursOfOperation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<LocalLocationHoursOfOperationItem>>(result__)
        }
    }
}
impl ::core::clone::Clone for LocalLocation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LocalLocation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LocalLocation {}
impl ::core::fmt::Debug for LocalLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LocalLocation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LocalLocation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.LocalSearch.LocalLocation;{bb0fe9ab-4502-4f2c-94a9-0d60de0e2163})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LocalLocation {
    type Vtable = ILocalLocation_Vtbl;
    const IID: ::windows::core::GUID = <ILocalLocation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LocalLocation {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalLocation";
}
impl ::core::convert::From<LocalLocation> for ::windows::core::IUnknown {
    fn from(value: LocalLocation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LocalLocation> for ::windows::core::IUnknown {
    fn from(value: &LocalLocation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LocalLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LocalLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LocalLocation> for ::windows::core::IInspectable {
    fn from(value: LocalLocation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LocalLocation> for ::windows::core::IInspectable {
    fn from(value: &LocalLocation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LocalLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LocalLocation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LocalLocation {}
unsafe impl ::core::marker::Sync for LocalLocation {}
#[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
pub struct LocalLocationFinder {}
impl LocalLocationFinder {
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`, `\"Devices_Geolocation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Geolocation", feature = "Foundation"))]
    pub fn FindLocalLocationsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::super::Devices::Geolocation::Geocircle>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(searchterm: Param0, searcharea: Param1, localcategory: Param2, maxresults: u32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LocalLocationFinderResult>> {
        Self::ILocalLocationFinderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindLocalLocationsAsync)(::core::mem::transmute_copy(this), searchterm.into_param().abi(), searcharea.into_param().abi(), localcategory.into_param().abi(), maxresults, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<LocalLocationFinderResult>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILocalLocationFinderStatics<R, F: FnOnce(&ILocalLocationFinderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LocalLocationFinder, ILocalLocationFinderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for LocalLocationFinder {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalLocationFinder";
}
#[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
#[repr(transparent)]
pub struct LocalLocationFinderResult(::windows::core::IUnknown);
impl LocalLocationFinderResult {
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LocalLocations(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<LocalLocation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocalLocations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<LocalLocation>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<LocalLocationFinderStatus> {
        let this = self;
        unsafe {
            let mut result__: LocalLocationFinderStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LocalLocationFinderStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for LocalLocationFinderResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LocalLocationFinderResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LocalLocationFinderResult {}
impl ::core::fmt::Debug for LocalLocationFinderResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LocalLocationFinderResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LocalLocationFinderResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.LocalSearch.LocalLocationFinderResult;{d09b6cc6-f338-4191-9fd8-5440b9a68f52})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LocalLocationFinderResult {
    type Vtable = ILocalLocationFinderResult_Vtbl;
    const IID: ::windows::core::GUID = <ILocalLocationFinderResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LocalLocationFinderResult {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalLocationFinderResult";
}
impl ::core::convert::From<LocalLocationFinderResult> for ::windows::core::IUnknown {
    fn from(value: LocalLocationFinderResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LocalLocationFinderResult> for ::windows::core::IUnknown {
    fn from(value: &LocalLocationFinderResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LocalLocationFinderResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LocalLocationFinderResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LocalLocationFinderResult> for ::windows::core::IInspectable {
    fn from(value: LocalLocationFinderResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LocalLocationFinderResult> for ::windows::core::IInspectable {
    fn from(value: &LocalLocationFinderResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LocalLocationFinderResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LocalLocationFinderResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LocalLocationFinderResult {}
unsafe impl ::core::marker::Sync for LocalLocationFinderResult {}
#[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LocalLocationFinderStatus(pub i32);
impl LocalLocationFinderStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const InvalidCredentials: Self = Self(2i32);
    pub const InvalidCategory: Self = Self(3i32);
    pub const InvalidSearchTerm: Self = Self(4i32);
    pub const InvalidSearchArea: Self = Self(5i32);
    pub const NetworkFailure: Self = Self(6i32);
    pub const NotSupported: Self = Self(7i32);
}
impl ::core::marker::Copy for LocalLocationFinderStatus {}
impl ::core::clone::Clone for LocalLocationFinderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LocalLocationFinderStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LocalLocationFinderStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for LocalLocationFinderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LocalLocationFinderStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LocalLocationFinderStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Services.Maps.LocalSearch.LocalLocationFinderStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
#[repr(transparent)]
pub struct LocalLocationHoursOfOperationItem(::windows::core::IUnknown);
impl LocalLocationHoursOfOperationItem {
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`, `\"Globalization\"`*"]
    #[cfg(feature = "Globalization")]
    pub fn Day(&self) -> ::windows::core::Result<super::super::super::Globalization::DayOfWeek> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Globalization::DayOfWeek = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Day)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Globalization::DayOfWeek>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Start(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Span(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Span)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for LocalLocationHoursOfOperationItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LocalLocationHoursOfOperationItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LocalLocationHoursOfOperationItem {}
impl ::core::fmt::Debug for LocalLocationHoursOfOperationItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LocalLocationHoursOfOperationItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LocalLocationHoursOfOperationItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.LocalSearch.LocalLocationHoursOfOperationItem;{23548c72-a1c7-43f1-a4f0-1091c39ec640})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LocalLocationHoursOfOperationItem {
    type Vtable = ILocalLocationHoursOfOperationItem_Vtbl;
    const IID: ::windows::core::GUID = <ILocalLocationHoursOfOperationItem as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LocalLocationHoursOfOperationItem {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalLocationHoursOfOperationItem";
}
impl ::core::convert::From<LocalLocationHoursOfOperationItem> for ::windows::core::IUnknown {
    fn from(value: LocalLocationHoursOfOperationItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LocalLocationHoursOfOperationItem> for ::windows::core::IUnknown {
    fn from(value: &LocalLocationHoursOfOperationItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LocalLocationHoursOfOperationItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LocalLocationHoursOfOperationItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LocalLocationHoursOfOperationItem> for ::windows::core::IInspectable {
    fn from(value: LocalLocationHoursOfOperationItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LocalLocationHoursOfOperationItem> for ::windows::core::IInspectable {
    fn from(value: &LocalLocationHoursOfOperationItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LocalLocationHoursOfOperationItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LocalLocationHoursOfOperationItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LocalLocationHoursOfOperationItem {}
unsafe impl ::core::marker::Sync for LocalLocationHoursOfOperationItem {}
#[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
#[repr(transparent)]
pub struct LocalLocationRatingInfo(::windows::core::IUnknown);
impl LocalLocationRatingInfo {
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AggregateRating(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AggregateRating)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RatingCount(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RatingCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
    pub fn ProviderIdentifier(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProviderIdentifier)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for LocalLocationRatingInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LocalLocationRatingInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LocalLocationRatingInfo {}
impl ::core::fmt::Debug for LocalLocationRatingInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LocalLocationRatingInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LocalLocationRatingInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Services.Maps.LocalSearch.LocalLocationRatingInfo;{cb1dab56-3354-4311-8bc0-a2d4d5eb806e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LocalLocationRatingInfo {
    type Vtable = ILocalLocationRatingInfo_Vtbl;
    const IID: ::windows::core::GUID = <ILocalLocationRatingInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LocalLocationRatingInfo {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.LocalLocationRatingInfo";
}
impl ::core::convert::From<LocalLocationRatingInfo> for ::windows::core::IUnknown {
    fn from(value: LocalLocationRatingInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LocalLocationRatingInfo> for ::windows::core::IUnknown {
    fn from(value: &LocalLocationRatingInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LocalLocationRatingInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LocalLocationRatingInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LocalLocationRatingInfo> for ::windows::core::IInspectable {
    fn from(value: LocalLocationRatingInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LocalLocationRatingInfo> for ::windows::core::IInspectable {
    fn from(value: &LocalLocationRatingInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LocalLocationRatingInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LocalLocationRatingInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LocalLocationRatingInfo {}
unsafe impl ::core::marker::Sync for LocalLocationRatingInfo {}
#[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
pub struct PlaceInfoHelper {}
impl PlaceInfoHelper {
    #[doc = "*Required features: `\"Services_Maps_LocalSearch\"`*"]
    pub fn CreateFromLocalLocation<'a, Param0: ::windows::core::IntoParam<'a, LocalLocation>>(location: Param0) -> ::windows::core::Result<super::PlaceInfo> {
        Self::IPlaceInfoHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFromLocalLocation)(::core::mem::transmute_copy(this), location.into_param().abi(), &mut result__).from_abi::<super::PlaceInfo>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPlaceInfoHelperStatics<R, F: FnOnce(&IPlaceInfoHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PlaceInfoHelper, IPlaceInfoHelperStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PlaceInfoHelper {
    const NAME: &'static str = "Windows.Services.Maps.LocalSearch.PlaceInfoHelper";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
