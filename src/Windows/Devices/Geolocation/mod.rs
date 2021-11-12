#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Devices_Geolocation_Geofencing")]
pub mod Geofencing;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AltitudeReferenceSystem(pub i32);
impl AltitudeReferenceSystem {
    pub const Unspecified: AltitudeReferenceSystem = AltitudeReferenceSystem(0i32);
    pub const Terrain: AltitudeReferenceSystem = AltitudeReferenceSystem(1i32);
    pub const Ellipsoid: AltitudeReferenceSystem = AltitudeReferenceSystem(2i32);
    pub const Geoid: AltitudeReferenceSystem = AltitudeReferenceSystem(3i32);
    pub const Surface: AltitudeReferenceSystem = AltitudeReferenceSystem(4i32);
}
impl ::core::convert::From<i32> for AltitudeReferenceSystem {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AltitudeReferenceSystem {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AltitudeReferenceSystem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.AltitudeReferenceSystem;i4)");
}
impl ::windows::core::DefaultType for AltitudeReferenceSystem {
    type DefaultType = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct BasicGeoposition {
    pub Latitude: f64,
    pub Longitude: f64,
    pub Altitude: f64,
}
impl BasicGeoposition {}
impl ::core::default::Default for BasicGeoposition {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BasicGeoposition {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BasicGeoposition").field("Latitude", &self.Latitude).field("Longitude", &self.Longitude).field("Altitude", &self.Altitude).finish()
    }
}
impl ::core::cmp::PartialEq for BasicGeoposition {
    fn eq(&self, other: &Self) -> bool {
        self.Latitude == other.Latitude && self.Longitude == other.Longitude && self.Altitude == other.Altitude
    }
}
impl ::core::cmp::Eq for BasicGeoposition {}
unsafe impl ::windows::core::Abi for BasicGeoposition {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BasicGeoposition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Devices.Geolocation.BasicGeoposition;f8;f8;f8)");
}
impl ::windows::core::DefaultType for BasicGeoposition {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CivicAddress(pub ::windows::core::IInspectable);
impl CivicAddress {
    pub fn Country(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn State(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn City(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn PostalCode(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CivicAddress {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.CivicAddress;{a8567a1a-64f4-4d48-bcea-f6b008eca34c})");
}
unsafe impl ::windows::core::Interface for CivicAddress {
    type Vtable = ICivicAddress_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8567a1a_64f4_4d48_bcea_f6b008eca34c);
}
impl ::windows::core::RuntimeName for CivicAddress {
    const NAME: &'static str = "Windows.Devices.Geolocation.CivicAddress";
}
impl ::core::convert::From<CivicAddress> for ::windows::core::IUnknown {
    fn from(value: CivicAddress) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CivicAddress> for ::windows::core::IUnknown {
    fn from(value: &CivicAddress) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CivicAddress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CivicAddress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CivicAddress> for ::windows::core::IInspectable {
    fn from(value: CivicAddress) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CivicAddress> for ::windows::core::IInspectable {
    fn from(value: &CivicAddress) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CivicAddress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CivicAddress {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CivicAddress {}
unsafe impl ::core::marker::Sync for CivicAddress {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GeoboundingBox(pub ::windows::core::IInspectable);
impl GeoboundingBox {
    pub fn NorthwestCorner(&self) -> ::windows::core::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__: BasicGeoposition = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BasicGeoposition>(result__)
        }
    }
    pub fn SoutheastCorner(&self) -> ::windows::core::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__: BasicGeoposition = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BasicGeoposition>(result__)
        }
    }
    pub fn Center(&self) -> ::windows::core::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__: BasicGeoposition = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BasicGeoposition>(result__)
        }
    }
    pub fn MinAltitude(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn MaxAltitude(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn GeoshapeType(&self) -> ::windows::core::Result<GeoshapeType> {
        let this = &::windows::core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__: GeoshapeType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GeoshapeType>(result__)
        }
    }
    pub fn SpatialReferenceId(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn AltitudeReferenceSystem(&self) -> ::windows::core::Result<AltitudeReferenceSystem> {
        let this = &::windows::core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__: AltitudeReferenceSystem = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AltitudeReferenceSystem>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, BasicGeoposition>, Param1: ::windows::core::IntoParam<'a, BasicGeoposition>>(northwestcorner: Param0, southeastcorner: Param1) -> ::windows::core::Result<GeoboundingBox> {
        Self::IGeoboundingBoxFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), northwestcorner.into_param().abi(), southeastcorner.into_param().abi(), &mut result__).from_abi::<GeoboundingBox>(result__)
        })
    }
    pub fn CreateWithAltitudeReference<'a, Param0: ::windows::core::IntoParam<'a, BasicGeoposition>, Param1: ::windows::core::IntoParam<'a, BasicGeoposition>>(northwestcorner: Param0, southeastcorner: Param1, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows::core::Result<GeoboundingBox> {
        Self::IGeoboundingBoxFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), northwestcorner.into_param().abi(), southeastcorner.into_param().abi(), altitudereferencesystem, &mut result__).from_abi::<GeoboundingBox>(result__)
        })
    }
    pub fn CreateWithAltitudeReferenceAndSpatialReference<'a, Param0: ::windows::core::IntoParam<'a, BasicGeoposition>, Param1: ::windows::core::IntoParam<'a, BasicGeoposition>>(northwestcorner: Param0, southeastcorner: Param1, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::core::Result<GeoboundingBox> {
        Self::IGeoboundingBoxFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), northwestcorner.into_param().abi(), southeastcorner.into_param().abi(), altitudereferencesystem, spatialreferenceid, &mut result__).from_abi::<GeoboundingBox>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryCompute<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<BasicGeoposition>>>(positions: Param0) -> ::windows::core::Result<GeoboundingBox> {
        Self::IGeoboundingBoxStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), positions.into_param().abi(), &mut result__).from_abi::<GeoboundingBox>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryComputeWithAltitudeReference<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<BasicGeoposition>>>(positions: Param0, altituderefsystem: AltitudeReferenceSystem) -> ::windows::core::Result<GeoboundingBox> {
        Self::IGeoboundingBoxStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), positions.into_param().abi(), altituderefsystem, &mut result__).from_abi::<GeoboundingBox>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryComputeWithAltitudeReferenceAndSpatialReference<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<BasicGeoposition>>>(positions: Param0, altituderefsystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::core::Result<GeoboundingBox> {
        Self::IGeoboundingBoxStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), positions.into_param().abi(), altituderefsystem, spatialreferenceid, &mut result__).from_abi::<GeoboundingBox>(result__)
        })
    }
    pub fn IGeoboundingBoxFactory<R, F: FnOnce(&IGeoboundingBoxFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GeoboundingBox, IGeoboundingBoxFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGeoboundingBoxStatics<R, F: FnOnce(&IGeoboundingBoxStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GeoboundingBox, IGeoboundingBoxStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for GeoboundingBox {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.GeoboundingBox;{0896c80b-274f-43da-9a06-cbfcdaeb4ec2})");
}
unsafe impl ::windows::core::Interface for GeoboundingBox {
    type Vtable = IGeoboundingBox_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0896c80b_274f_43da_9a06_cbfcdaeb4ec2);
}
impl ::windows::core::RuntimeName for GeoboundingBox {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeoboundingBox";
}
impl ::core::convert::From<GeoboundingBox> for ::windows::core::IUnknown {
    fn from(value: GeoboundingBox) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GeoboundingBox> for ::windows::core::IUnknown {
    fn from(value: &GeoboundingBox) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GeoboundingBox {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GeoboundingBox {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GeoboundingBox> for ::windows::core::IInspectable {
    fn from(value: GeoboundingBox) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GeoboundingBox> for ::windows::core::IInspectable {
    fn from(value: &GeoboundingBox) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GeoboundingBox {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GeoboundingBox {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<GeoboundingBox> for IGeoshape {
    type Error = ::windows::core::Error;
    fn try_from(value: GeoboundingBox) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&GeoboundingBox> for IGeoshape {
    type Error = ::windows::core::Error;
    fn try_from(value: &GeoboundingBox) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGeoshape> for GeoboundingBox {
    fn into_param(self) -> ::windows::core::Param<'a, IGeoshape> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGeoshape> for &GeoboundingBox {
    fn into_param(self) -> ::windows::core::Param<'a, IGeoshape> {
        ::core::convert::TryInto::<IGeoshape>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for GeoboundingBox {}
unsafe impl ::core::marker::Sync for GeoboundingBox {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Geocircle(pub ::windows::core::IInspectable);
impl Geocircle {
    pub fn Center(&self) -> ::windows::core::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__: BasicGeoposition = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BasicGeoposition>(result__)
        }
    }
    pub fn Radius(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn GeoshapeType(&self) -> ::windows::core::Result<GeoshapeType> {
        let this = &::windows::core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__: GeoshapeType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GeoshapeType>(result__)
        }
    }
    pub fn SpatialReferenceId(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn AltitudeReferenceSystem(&self) -> ::windows::core::Result<AltitudeReferenceSystem> {
        let this = &::windows::core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__: AltitudeReferenceSystem = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AltitudeReferenceSystem>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, BasicGeoposition>>(position: Param0, radius: f64) -> ::windows::core::Result<Geocircle> {
        Self::IGeocircleFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), position.into_param().abi(), radius, &mut result__).from_abi::<Geocircle>(result__)
        })
    }
    pub fn CreateWithAltitudeReferenceSystem<'a, Param0: ::windows::core::IntoParam<'a, BasicGeoposition>>(position: Param0, radius: f64, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows::core::Result<Geocircle> {
        Self::IGeocircleFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), position.into_param().abi(), radius, altitudereferencesystem, &mut result__).from_abi::<Geocircle>(result__)
        })
    }
    pub fn CreateWithAltitudeReferenceSystemAndSpatialReferenceId<'a, Param0: ::windows::core::IntoParam<'a, BasicGeoposition>>(position: Param0, radius: f64, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::core::Result<Geocircle> {
        Self::IGeocircleFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), position.into_param().abi(), radius, altitudereferencesystem, spatialreferenceid, &mut result__).from_abi::<Geocircle>(result__)
        })
    }
    pub fn IGeocircleFactory<R, F: FnOnce(&IGeocircleFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Geocircle, IGeocircleFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Geocircle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geocircle;{39e45843-a7f9-4e63-92a7-ba0c28d124b1})");
}
unsafe impl ::windows::core::Interface for Geocircle {
    type Vtable = IGeocircle_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39e45843_a7f9_4e63_92a7_ba0c28d124b1);
}
impl ::windows::core::RuntimeName for Geocircle {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geocircle";
}
impl ::core::convert::From<Geocircle> for ::windows::core::IUnknown {
    fn from(value: Geocircle) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Geocircle> for ::windows::core::IUnknown {
    fn from(value: &Geocircle) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Geocircle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Geocircle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Geocircle> for ::windows::core::IInspectable {
    fn from(value: Geocircle) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Geocircle> for ::windows::core::IInspectable {
    fn from(value: &Geocircle) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Geocircle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Geocircle {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<Geocircle> for IGeoshape {
    type Error = ::windows::core::Error;
    fn try_from(value: Geocircle) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Geocircle> for IGeoshape {
    type Error = ::windows::core::Error;
    fn try_from(value: &Geocircle) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGeoshape> for Geocircle {
    fn into_param(self) -> ::windows::core::Param<'a, IGeoshape> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGeoshape> for &Geocircle {
    fn into_param(self) -> ::windows::core::Param<'a, IGeoshape> {
        ::core::convert::TryInto::<IGeoshape>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for Geocircle {}
unsafe impl ::core::marker::Sync for Geocircle {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Geocoordinate(pub ::windows::core::IInspectable);
impl Geocoordinate {
    #[cfg(feature = "deprecated")]
    pub fn Latitude(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Longitude(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn Altitude(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    pub fn Accuracy(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn AltitudeAccuracy(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Heading(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Speed(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn Point(&self) -> ::windows::core::Result<Geopoint> {
        let this = &::windows::core::Interface::cast::<IGeocoordinateWithPoint>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Geopoint>(result__)
        }
    }
    pub fn PositionSource(&self) -> ::windows::core::Result<PositionSource> {
        let this = &::windows::core::Interface::cast::<IGeocoordinateWithPositionData>(self)?;
        unsafe {
            let mut result__: PositionSource = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PositionSource>(result__)
        }
    }
    pub fn SatelliteData(&self) -> ::windows::core::Result<GeocoordinateSatelliteData> {
        let this = &::windows::core::Interface::cast::<IGeocoordinateWithPositionData>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GeocoordinateSatelliteData>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PositionSourceTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = &::windows::core::Interface::cast::<IGeocoordinateWithPositionSourceTimestamp>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    pub fn IsRemoteSource(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IGeocoordinateWithRemoteSource>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for Geocoordinate {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geocoordinate;{ee21a3aa-976a-4c70-803d-083ea55bcbc4})");
}
unsafe impl ::windows::core::Interface for Geocoordinate {
    type Vtable = IGeocoordinate_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee21a3aa_976a_4c70_803d_083ea55bcbc4);
}
impl ::windows::core::RuntimeName for Geocoordinate {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geocoordinate";
}
impl ::core::convert::From<Geocoordinate> for ::windows::core::IUnknown {
    fn from(value: Geocoordinate) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Geocoordinate> for ::windows::core::IUnknown {
    fn from(value: &Geocoordinate) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Geocoordinate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Geocoordinate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Geocoordinate> for ::windows::core::IInspectable {
    fn from(value: Geocoordinate) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Geocoordinate> for ::windows::core::IInspectable {
    fn from(value: &Geocoordinate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Geocoordinate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Geocoordinate {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for Geocoordinate {}
unsafe impl ::core::marker::Sync for Geocoordinate {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GeocoordinateSatelliteData(pub ::windows::core::IInspectable);
impl GeocoordinateSatelliteData {
    #[cfg(feature = "Foundation")]
    pub fn PositionDilutionOfPrecision(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn HorizontalDilutionOfPrecision(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn VerticalDilutionOfPrecision(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GeometricDilutionOfPrecision(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows::core::Interface::cast::<IGeocoordinateSatelliteData2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TimeDilutionOfPrecision(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows::core::Interface::cast::<IGeocoordinateSatelliteData2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GeocoordinateSatelliteData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.GeocoordinateSatelliteData;{c32a74d9-2608-474c-912c-06dd490f4af7})");
}
unsafe impl ::windows::core::Interface for GeocoordinateSatelliteData {
    type Vtable = IGeocoordinateSatelliteData_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc32a74d9_2608_474c_912c_06dd490f4af7);
}
impl ::windows::core::RuntimeName for GeocoordinateSatelliteData {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeocoordinateSatelliteData";
}
impl ::core::convert::From<GeocoordinateSatelliteData> for ::windows::core::IUnknown {
    fn from(value: GeocoordinateSatelliteData) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GeocoordinateSatelliteData> for ::windows::core::IUnknown {
    fn from(value: &GeocoordinateSatelliteData) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GeocoordinateSatelliteData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GeocoordinateSatelliteData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GeocoordinateSatelliteData> for ::windows::core::IInspectable {
    fn from(value: GeocoordinateSatelliteData) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GeocoordinateSatelliteData> for ::windows::core::IInspectable {
    fn from(value: &GeocoordinateSatelliteData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GeocoordinateSatelliteData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GeocoordinateSatelliteData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GeocoordinateSatelliteData {}
unsafe impl ::core::marker::Sync for GeocoordinateSatelliteData {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GeolocationAccessStatus(pub i32);
impl GeolocationAccessStatus {
    pub const Unspecified: GeolocationAccessStatus = GeolocationAccessStatus(0i32);
    pub const Allowed: GeolocationAccessStatus = GeolocationAccessStatus(1i32);
    pub const Denied: GeolocationAccessStatus = GeolocationAccessStatus(2i32);
}
impl ::core::convert::From<i32> for GeolocationAccessStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GeolocationAccessStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GeolocationAccessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.GeolocationAccessStatus;i4)");
}
impl ::windows::core::DefaultType for GeolocationAccessStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Geolocator(pub ::windows::core::IInspectable);
impl Geolocator {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Geolocator, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn DesiredAccuracy(&self) -> ::windows::core::Result<PositionAccuracy> {
        let this = self;
        unsafe {
            let mut result__: PositionAccuracy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PositionAccuracy>(result__)
        }
    }
    pub fn SetDesiredAccuracy(&self, value: PositionAccuracy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn MovementThreshold(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetMovementThreshold(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn LocationStatus(&self) -> ::windows::core::Result<PositionStatus> {
        let this = self;
        unsafe {
            let mut result__: PositionStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PositionStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetGeopositionAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Geoposition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Geoposition>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetGeopositionAsyncWithAgeAndTimeout<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, maximumage: Param0, timeout: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Geoposition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), maximumage.into_param().abi(), timeout.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Geoposition>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PositionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<Geolocator, PositionChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePositionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<Geolocator, StatusChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn DesiredAccuracyInMeters(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = &::windows::core::Interface::cast::<IGeolocatorWithScalarAccuracy>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetDesiredAccuracyInMeters<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<u32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGeolocatorWithScalarAccuracy>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GeolocationAccessStatus>> {
        Self::IGeolocatorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<GeolocationAccessStatus>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetGeopositionHistoryAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(starttime: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Geoposition>>> {
        Self::IGeolocatorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), starttime.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Geoposition>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetGeopositionHistoryWithDurationAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(starttime: Param0, duration: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Geoposition>>> {
        Self::IGeolocatorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), starttime.into_param().abi(), duration.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Geoposition>>>(result__)
        })
    }
    pub fn AllowFallbackToConsentlessPositions(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IGeolocator2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn IsDefaultGeopositionRecommended() -> ::windows::core::Result<bool> {
        Self::IGeolocatorStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn SetDefaultGeoposition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<BasicGeoposition>>>(value: Param0) -> ::windows::core::Result<()> {
        Self::IGeolocatorStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn DefaultGeoposition() -> ::windows::core::Result<super::super::Foundation::IReference<BasicGeoposition>> {
        Self::IGeolocatorStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<BasicGeoposition>>(result__)
        })
    }
    pub fn IGeolocatorStatics<R, F: FnOnce(&IGeolocatorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Geolocator, IGeolocatorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IGeolocatorStatics2<R, F: FnOnce(&IGeolocatorStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Geolocator, IGeolocatorStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Geolocator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geolocator;{a9c3bf62-4524-4989-8aa9-de019d2e551f})");
}
unsafe impl ::windows::core::Interface for Geolocator {
    type Vtable = IGeolocator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9c3bf62_4524_4989_8aa9_de019d2e551f);
}
impl ::windows::core::RuntimeName for Geolocator {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geolocator";
}
impl ::core::convert::From<Geolocator> for ::windows::core::IUnknown {
    fn from(value: Geolocator) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Geolocator> for ::windows::core::IUnknown {
    fn from(value: &Geolocator) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Geolocator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Geolocator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Geolocator> for ::windows::core::IInspectable {
    fn from(value: Geolocator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Geolocator> for ::windows::core::IInspectable {
    fn from(value: &Geolocator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Geolocator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Geolocator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for Geolocator {}
unsafe impl ::core::marker::Sync for Geolocator {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Geopath(pub ::windows::core::IInspectable);
impl Geopath {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Positions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<BasicGeoposition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<BasicGeoposition>>(result__)
        }
    }
    pub fn GeoshapeType(&self) -> ::windows::core::Result<GeoshapeType> {
        let this = &::windows::core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__: GeoshapeType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GeoshapeType>(result__)
        }
    }
    pub fn SpatialReferenceId(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn AltitudeReferenceSystem(&self) -> ::windows::core::Result<AltitudeReferenceSystem> {
        let this = &::windows::core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__: AltitudeReferenceSystem = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AltitudeReferenceSystem>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<BasicGeoposition>>>(positions: Param0) -> ::windows::core::Result<Geopath> {
        Self::IGeopathFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), positions.into_param().abi(), &mut result__).from_abi::<Geopath>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithAltitudeReference<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<BasicGeoposition>>>(positions: Param0, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows::core::Result<Geopath> {
        Self::IGeopathFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), positions.into_param().abi(), altitudereferencesystem, &mut result__).from_abi::<Geopath>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWithAltitudeReferenceAndSpatialReference<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<BasicGeoposition>>>(positions: Param0, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::core::Result<Geopath> {
        Self::IGeopathFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), positions.into_param().abi(), altitudereferencesystem, spatialreferenceid, &mut result__).from_abi::<Geopath>(result__)
        })
    }
    pub fn IGeopathFactory<R, F: FnOnce(&IGeopathFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Geopath, IGeopathFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Geopath {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geopath;{e53fd7b9-2da4-4714-a652-de8593289898})");
}
unsafe impl ::windows::core::Interface for Geopath {
    type Vtable = IGeopath_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe53fd7b9_2da4_4714_a652_de8593289898);
}
impl ::windows::core::RuntimeName for Geopath {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geopath";
}
impl ::core::convert::From<Geopath> for ::windows::core::IUnknown {
    fn from(value: Geopath) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Geopath> for ::windows::core::IUnknown {
    fn from(value: &Geopath) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Geopath {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Geopath {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Geopath> for ::windows::core::IInspectable {
    fn from(value: Geopath) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Geopath> for ::windows::core::IInspectable {
    fn from(value: &Geopath) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Geopath {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Geopath {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<Geopath> for IGeoshape {
    type Error = ::windows::core::Error;
    fn try_from(value: Geopath) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Geopath> for IGeoshape {
    type Error = ::windows::core::Error;
    fn try_from(value: &Geopath) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGeoshape> for Geopath {
    fn into_param(self) -> ::windows::core::Param<'a, IGeoshape> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGeoshape> for &Geopath {
    fn into_param(self) -> ::windows::core::Param<'a, IGeoshape> {
        ::core::convert::TryInto::<IGeoshape>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for Geopath {}
unsafe impl ::core::marker::Sync for Geopath {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Geopoint(pub ::windows::core::IInspectable);
impl Geopoint {
    pub fn Position(&self) -> ::windows::core::Result<BasicGeoposition> {
        let this = self;
        unsafe {
            let mut result__: BasicGeoposition = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BasicGeoposition>(result__)
        }
    }
    pub fn GeoshapeType(&self) -> ::windows::core::Result<GeoshapeType> {
        let this = &::windows::core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__: GeoshapeType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GeoshapeType>(result__)
        }
    }
    pub fn SpatialReferenceId(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn AltitudeReferenceSystem(&self) -> ::windows::core::Result<AltitudeReferenceSystem> {
        let this = &::windows::core::Interface::cast::<IGeoshape>(self)?;
        unsafe {
            let mut result__: AltitudeReferenceSystem = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AltitudeReferenceSystem>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, BasicGeoposition>>(position: Param0) -> ::windows::core::Result<Geopoint> {
        Self::IGeopointFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), position.into_param().abi(), &mut result__).from_abi::<Geopoint>(result__)
        })
    }
    pub fn CreateWithAltitudeReferenceSystem<'a, Param0: ::windows::core::IntoParam<'a, BasicGeoposition>>(position: Param0, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows::core::Result<Geopoint> {
        Self::IGeopointFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), position.into_param().abi(), altitudereferencesystem, &mut result__).from_abi::<Geopoint>(result__)
        })
    }
    pub fn CreateWithAltitudeReferenceSystemAndSpatialReferenceId<'a, Param0: ::windows::core::IntoParam<'a, BasicGeoposition>>(position: Param0, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::core::Result<Geopoint> {
        Self::IGeopointFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), position.into_param().abi(), altitudereferencesystem, spatialreferenceid, &mut result__).from_abi::<Geopoint>(result__)
        })
    }
    pub fn IGeopointFactory<R, F: FnOnce(&IGeopointFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Geopoint, IGeopointFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Geopoint {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geopoint;{6bfa00eb-e56e-49bb-9caf-cbaa78a8bcef})");
}
unsafe impl ::windows::core::Interface for Geopoint {
    type Vtable = IGeopoint_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bfa00eb_e56e_49bb_9caf_cbaa78a8bcef);
}
impl ::windows::core::RuntimeName for Geopoint {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geopoint";
}
impl ::core::convert::From<Geopoint> for ::windows::core::IUnknown {
    fn from(value: Geopoint) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Geopoint> for ::windows::core::IUnknown {
    fn from(value: &Geopoint) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Geopoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Geopoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Geopoint> for ::windows::core::IInspectable {
    fn from(value: Geopoint) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Geopoint> for ::windows::core::IInspectable {
    fn from(value: &Geopoint) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Geopoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Geopoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<Geopoint> for IGeoshape {
    type Error = ::windows::core::Error;
    fn try_from(value: Geopoint) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Geopoint> for IGeoshape {
    type Error = ::windows::core::Error;
    fn try_from(value: &Geopoint) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGeoshape> for Geopoint {
    fn into_param(self) -> ::windows::core::Param<'a, IGeoshape> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGeoshape> for &Geopoint {
    fn into_param(self) -> ::windows::core::Param<'a, IGeoshape> {
        ::core::convert::TryInto::<IGeoshape>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for Geopoint {}
unsafe impl ::core::marker::Sync for Geopoint {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Geoposition(pub ::windows::core::IInspectable);
impl Geoposition {
    pub fn Coordinate(&self) -> ::windows::core::Result<Geocoordinate> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Geocoordinate>(result__)
        }
    }
    pub fn CivicAddress(&self) -> ::windows::core::Result<CivicAddress> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CivicAddress>(result__)
        }
    }
    pub fn VenueData(&self) -> ::windows::core::Result<VenueData> {
        let this = &::windows::core::Interface::cast::<IGeoposition2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VenueData>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for Geoposition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geoposition;{c18d0454-7d41-4ff7-a957-9dffb4ef7f5b})");
}
unsafe impl ::windows::core::Interface for Geoposition {
    type Vtable = IGeoposition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc18d0454_7d41_4ff7_a957_9dffb4ef7f5b);
}
impl ::windows::core::RuntimeName for Geoposition {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geoposition";
}
impl ::core::convert::From<Geoposition> for ::windows::core::IUnknown {
    fn from(value: Geoposition) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Geoposition> for ::windows::core::IUnknown {
    fn from(value: &Geoposition) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Geoposition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Geoposition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Geoposition> for ::windows::core::IInspectable {
    fn from(value: Geoposition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Geoposition> for ::windows::core::IInspectable {
    fn from(value: &Geoposition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Geoposition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Geoposition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for Geoposition {}
unsafe impl ::core::marker::Sync for Geoposition {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GeoshapeType(pub i32);
impl GeoshapeType {
    pub const Geopoint: GeoshapeType = GeoshapeType(0i32);
    pub const Geocircle: GeoshapeType = GeoshapeType(1i32);
    pub const Geopath: GeoshapeType = GeoshapeType(2i32);
    pub const GeoboundingBox: GeoshapeType = GeoshapeType(3i32);
}
impl ::core::convert::From<i32> for GeoshapeType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GeoshapeType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for GeoshapeType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.GeoshapeType;i4)");
}
impl ::windows::core::DefaultType for GeoshapeType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Geovisit(pub ::windows::core::IInspectable);
impl Geovisit {
    pub fn Position(&self) -> ::windows::core::Result<Geoposition> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Geoposition>(result__)
        }
    }
    pub fn StateChange(&self) -> ::windows::core::Result<VisitStateChange> {
        let this = self;
        unsafe {
            let mut result__: VisitStateChange = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VisitStateChange>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for Geovisit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.Geovisit;{b1877a76-9ef6-41ab-a0dd-793ece76e2de})");
}
unsafe impl ::windows::core::Interface for Geovisit {
    type Vtable = IGeovisit_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1877a76_9ef6_41ab_a0dd_793ece76e2de);
}
impl ::windows::core::RuntimeName for Geovisit {
    const NAME: &'static str = "Windows.Devices.Geolocation.Geovisit";
}
impl ::core::convert::From<Geovisit> for ::windows::core::IUnknown {
    fn from(value: Geovisit) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Geovisit> for ::windows::core::IUnknown {
    fn from(value: &Geovisit) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Geovisit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Geovisit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Geovisit> for ::windows::core::IInspectable {
    fn from(value: Geovisit) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Geovisit> for ::windows::core::IInspectable {
    fn from(value: &Geovisit) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Geovisit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Geovisit {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for Geovisit {}
unsafe impl ::core::marker::Sync for Geovisit {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GeovisitMonitor(pub ::windows::core::IInspectable);
impl GeovisitMonitor {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GeovisitMonitor, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn MonitoringScope(&self) -> ::windows::core::Result<VisitMonitoringScope> {
        let this = self;
        unsafe {
            let mut result__: VisitMonitoringScope = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VisitMonitoringScope>(result__)
        }
    }
    pub fn Start(&self, value: VisitMonitoringScope) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn VisitStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<GeovisitMonitor, GeovisitStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveVisitStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetLastReportAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Geovisit>> {
        Self::IGeovisitMonitorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Geovisit>>(result__)
        })
    }
    pub fn IGeovisitMonitorStatics<R, F: FnOnce(&IGeovisitMonitorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GeovisitMonitor, IGeovisitMonitorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for GeovisitMonitor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.GeovisitMonitor;{80118aaf-5944-4591-83c1-396647f54f2c})");
}
unsafe impl ::windows::core::Interface for GeovisitMonitor {
    type Vtable = IGeovisitMonitor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80118aaf_5944_4591_83c1_396647f54f2c);
}
impl ::windows::core::RuntimeName for GeovisitMonitor {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeovisitMonitor";
}
impl ::core::convert::From<GeovisitMonitor> for ::windows::core::IUnknown {
    fn from(value: GeovisitMonitor) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GeovisitMonitor> for ::windows::core::IUnknown {
    fn from(value: &GeovisitMonitor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GeovisitMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GeovisitMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GeovisitMonitor> for ::windows::core::IInspectable {
    fn from(value: GeovisitMonitor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GeovisitMonitor> for ::windows::core::IInspectable {
    fn from(value: &GeovisitMonitor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GeovisitMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GeovisitMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GeovisitMonitor {}
unsafe impl ::core::marker::Sync for GeovisitMonitor {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GeovisitStateChangedEventArgs(pub ::windows::core::IInspectable);
impl GeovisitStateChangedEventArgs {
    pub fn Visit(&self) -> ::windows::core::Result<Geovisit> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Geovisit>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GeovisitStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.GeovisitStateChangedEventArgs;{ceb4d1ff-8b53-4968-beed-4cecd029ce15})");
}
unsafe impl ::windows::core::Interface for GeovisitStateChangedEventArgs {
    type Vtable = IGeovisitStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xceb4d1ff_8b53_4968_beed_4cecd029ce15);
}
impl ::windows::core::RuntimeName for GeovisitStateChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeovisitStateChangedEventArgs";
}
impl ::core::convert::From<GeovisitStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: GeovisitStateChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GeovisitStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GeovisitStateChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GeovisitStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GeovisitStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GeovisitStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: GeovisitStateChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GeovisitStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GeovisitStateChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GeovisitStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GeovisitStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GeovisitStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for GeovisitStateChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct GeovisitTriggerDetails(pub ::windows::core::IInspectable);
impl GeovisitTriggerDetails {
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadReports(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Geovisit>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<Geovisit>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for GeovisitTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.GeovisitTriggerDetails;{ea770d9e-d1c9-454b-99b7-b2f8cdd2482f})");
}
unsafe impl ::windows::core::Interface for GeovisitTriggerDetails {
    type Vtable = IGeovisitTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea770d9e_d1c9_454b_99b7_b2f8cdd2482f);
}
impl ::windows::core::RuntimeName for GeovisitTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Geolocation.GeovisitTriggerDetails";
}
impl ::core::convert::From<GeovisitTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: GeovisitTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&GeovisitTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &GeovisitTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GeovisitTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GeovisitTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<GeovisitTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: GeovisitTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&GeovisitTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &GeovisitTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GeovisitTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GeovisitTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for GeovisitTriggerDetails {}
unsafe impl ::core::marker::Sync for GeovisitTriggerDetails {}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICivicAddress(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICivicAddress {
    type Vtable = ICivicAddress_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8567a1a_64f4_4d48_bcea_f6b008eca34c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICivicAddress_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeoboundingBox(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeoboundingBox {
    type Vtable = IGeoboundingBox_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0896c80b_274f_43da_9a06_cbfcdaeb4ec2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoboundingBox_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BasicGeoposition) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BasicGeoposition) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BasicGeoposition) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeoboundingBoxFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeoboundingBoxFactory {
    type Vtable = IGeoboundingBoxFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4dfba589_0411_4abc_b3b5_5bbccb57d98c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoboundingBoxFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, northwestcorner: BasicGeoposition, southeastcorner: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeoboundingBoxStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeoboundingBoxStatics {
    type Vtable = IGeoboundingBoxStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67b80708_e61a_4cd0_841b_93233792b5ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoboundingBoxStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, positions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, positions: ::windows::core::RawPtr, altituderefsystem: AltitudeReferenceSystem, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, positions: ::windows::core::RawPtr, altituderefsystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeocircle(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeocircle {
    type Vtable = IGeocircle_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39e45843_a7f9_4e63_92a7_ba0c28d124b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocircle_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BasicGeoposition) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeocircleFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeocircleFactory {
    type Vtable = IGeocircleFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xafd6531f_72b1_4f7d_87cc_4ed4c9849c05);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocircleFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, position: BasicGeoposition, radius: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, position: BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, position: BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeocoordinate(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeocoordinate {
    type Vtable = IGeocoordinate_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee21a3aa_976a_4c70_803d_083ea55bcbc4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinate_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeocoordinateSatelliteData(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeocoordinateSatelliteData {
    type Vtable = IGeocoordinateSatelliteData_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc32a74d9_2608_474c_912c_06dd490f4af7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateSatelliteData_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeocoordinateSatelliteData2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeocoordinateSatelliteData2 {
    type Vtable = IGeocoordinateSatelliteData2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x761c8cfd_a19d_5a51_80f5_71676115483e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateSatelliteData2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeocoordinateWithPoint(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeocoordinateWithPoint {
    type Vtable = IGeocoordinateWithPoint_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfeea0525_d22c_4d46_b527_0b96066fc7db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateWithPoint_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeocoordinateWithPositionData(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeocoordinateWithPositionData {
    type Vtable = IGeocoordinateWithPositionData_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95e634be_dbd6_40ac_b8f2_a65c0340d9a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateWithPositionData_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PositionSource) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeocoordinateWithPositionSourceTimestamp(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeocoordinateWithPositionSourceTimestamp {
    type Vtable = IGeocoordinateWithPositionSourceTimestamp_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8543fc02_c9f1_4610_afe0_8bc3a6a87036);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateWithPositionSourceTimestamp_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeocoordinateWithRemoteSource(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeocoordinateWithRemoteSource {
    type Vtable = IGeocoordinateWithRemoteSource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x397cebd7_ee38_5f3b_8900_c4a7bc9cf953);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeocoordinateWithRemoteSource_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeolocator(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeolocator {
    type Vtable = IGeolocator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9c3bf62_4524_4989_8aa9_de019d2e551f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeolocator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PositionAccuracy) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PositionAccuracy) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PositionStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, maximumage: super::super::Foundation::TimeSpan, timeout: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeolocator2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeolocator2 {
    type Vtable = IGeolocator2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1b42e6d_8891_43b4_ad36_27c6fe9a97b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeolocator2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeolocatorStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeolocatorStatics {
    type Vtable = IGeolocatorStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a8e7571_2df5_4591_9f87_eb5fd894e9b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeolocatorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, starttime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, starttime: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeolocatorStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeolocatorStatics2 {
    type Vtable = IGeolocatorStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x993011a2_fa1c_4631_a71d_0dbeb1250d9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeolocatorStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeolocatorWithScalarAccuracy(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeolocatorWithScalarAccuracy {
    type Vtable = IGeolocatorWithScalarAccuracy_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96f5d3c1_b80f_460a_994d_a96c47a51aa4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeolocatorWithScalarAccuracy_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeopath(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeopath {
    type Vtable = IGeopath_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe53fd7b9_2da4_4714_a652_de8593289898);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeopath_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeopathFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeopathFactory {
    type Vtable = IGeopathFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27bea9c8_c7e7_4359_9b9b_fca3e05ef593);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeopathFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, positions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, positions: ::windows::core::RawPtr, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, positions: ::windows::core::RawPtr, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeopoint(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeopoint {
    type Vtable = IGeopoint_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bfa00eb_e56e_49bb_9caf_cbaa78a8bcef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeopoint_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BasicGeoposition) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeopointFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeopointFactory {
    type Vtable = IGeopointFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb6b8d33_76bd_4e30_8af7_a844dc37b7a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeopointFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, position: BasicGeoposition, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, position: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, position: BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeoposition(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeoposition {
    type Vtable = IGeoposition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc18d0454_7d41_4ff7_a957_9dffb4ef7f5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoposition_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeoposition2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeoposition2 {
    type Vtable = IGeoposition2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f62f697_8671_4b0d_86f8_474a8496187c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoposition2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGeoshape(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeoshape {
    type Vtable = IGeoshape_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc99ca2af_c729_43c1_8fab_d6dec914df7e);
}
impl IGeoshape {
    pub fn GeoshapeType(&self) -> ::windows::core::Result<GeoshapeType> {
        let this = self;
        unsafe {
            let mut result__: GeoshapeType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<GeoshapeType>(result__)
        }
    }
    pub fn SpatialReferenceId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn AltitudeReferenceSystem(&self) -> ::windows::core::Result<AltitudeReferenceSystem> {
        let this = self;
        unsafe {
            let mut result__: AltitudeReferenceSystem = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AltitudeReferenceSystem>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IGeoshape {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{c99ca2af-c729-43c1-8fab-d6dec914df7e}");
}
impl ::core::convert::From<IGeoshape> for ::windows::core::IUnknown {
    fn from(value: IGeoshape) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IGeoshape> for ::windows::core::IUnknown {
    fn from(value: &IGeoshape) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGeoshape {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGeoshape {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IGeoshape> for ::windows::core::IInspectable {
    fn from(value: IGeoshape) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGeoshape> for ::windows::core::IInspectable {
    fn from(value: &IGeoshape) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IGeoshape {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IGeoshape {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeoshape_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut GeoshapeType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AltitudeReferenceSystem) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeovisit(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeovisit {
    type Vtable = IGeovisit_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1877a76_9ef6_41ab_a0dd_793ece76e2de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisit_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VisitStateChange) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeovisitMonitor(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeovisitMonitor {
    type Vtable = IGeovisitMonitor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80118aaf_5944_4591_83c1_396647f54f2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisitMonitor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VisitMonitoringScope) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: VisitMonitoringScope) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeovisitMonitorStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeovisitMonitorStatics {
    type Vtable = IGeovisitMonitorStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbcf976a7_bbf2_4cdd_95cf_554c82edfb87);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisitMonitorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeovisitStateChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeovisitStateChangedEventArgs {
    type Vtable = IGeovisitStateChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xceb4d1ff_8b53_4968_beed_4cecd029ce15);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisitStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGeovisitTriggerDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGeovisitTriggerDetails {
    type Vtable = IGeovisitTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea770d9e_d1c9_454b_99b7_b2f8cdd2482f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisitTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPositionChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPositionChangedEventArgs {
    type Vtable = IPositionChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x37859ce5_9d1e_46c5_bf3b_6ad8cac1a093);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPositionChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IStatusChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStatusChangedEventArgs {
    type Vtable = IStatusChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3453d2da_8c93_4111_a205_9aecfc9be5c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStatusChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PositionStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVenueData(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVenueData {
    type Vtable = IVenueData_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66f39187_60e3_4b2f_b527_4f53f1c3c677);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVenueData_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PositionAccuracy(pub i32);
impl PositionAccuracy {
    pub const Default: PositionAccuracy = PositionAccuracy(0i32);
    pub const High: PositionAccuracy = PositionAccuracy(1i32);
}
impl ::core::convert::From<i32> for PositionAccuracy {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PositionAccuracy {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PositionAccuracy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.PositionAccuracy;i4)");
}
impl ::windows::core::DefaultType for PositionAccuracy {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PositionChangedEventArgs(pub ::windows::core::IInspectable);
impl PositionChangedEventArgs {
    pub fn Position(&self) -> ::windows::core::Result<Geoposition> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Geoposition>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PositionChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.PositionChangedEventArgs;{37859ce5-9d1e-46c5-bf3b-6ad8cac1a093})");
}
unsafe impl ::windows::core::Interface for PositionChangedEventArgs {
    type Vtable = IPositionChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x37859ce5_9d1e_46c5_bf3b_6ad8cac1a093);
}
impl ::windows::core::RuntimeName for PositionChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Geolocation.PositionChangedEventArgs";
}
impl ::core::convert::From<PositionChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PositionChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PositionChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PositionChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PositionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PositionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PositionChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PositionChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PositionChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PositionChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PositionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PositionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PositionChangedEventArgs {}
unsafe impl ::core::marker::Sync for PositionChangedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PositionSource(pub i32);
impl PositionSource {
    pub const Cellular: PositionSource = PositionSource(0i32);
    pub const Satellite: PositionSource = PositionSource(1i32);
    pub const WiFi: PositionSource = PositionSource(2i32);
    pub const IPAddress: PositionSource = PositionSource(3i32);
    pub const Unknown: PositionSource = PositionSource(4i32);
    pub const Default: PositionSource = PositionSource(5i32);
    pub const Obfuscated: PositionSource = PositionSource(6i32);
}
impl ::core::convert::From<i32> for PositionSource {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PositionSource {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PositionSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.PositionSource;i4)");
}
impl ::windows::core::DefaultType for PositionSource {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PositionStatus(pub i32);
impl PositionStatus {
    pub const Ready: PositionStatus = PositionStatus(0i32);
    pub const Initializing: PositionStatus = PositionStatus(1i32);
    pub const NoData: PositionStatus = PositionStatus(2i32);
    pub const Disabled: PositionStatus = PositionStatus(3i32);
    pub const NotInitialized: PositionStatus = PositionStatus(4i32);
    pub const NotAvailable: PositionStatus = PositionStatus(5i32);
}
impl ::core::convert::From<i32> for PositionStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PositionStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PositionStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.PositionStatus;i4)");
}
impl ::windows::core::DefaultType for PositionStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StatusChangedEventArgs(pub ::windows::core::IInspectable);
impl StatusChangedEventArgs {
    pub fn Status(&self) -> ::windows::core::Result<PositionStatus> {
        let this = self;
        unsafe {
            let mut result__: PositionStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PositionStatus>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for StatusChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.StatusChangedEventArgs;{3453d2da-8c93-4111-a205-9aecfc9be5c0})");
}
unsafe impl ::windows::core::Interface for StatusChangedEventArgs {
    type Vtable = IStatusChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3453d2da_8c93_4111_a205_9aecfc9be5c0);
}
impl ::windows::core::RuntimeName for StatusChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Geolocation.StatusChangedEventArgs";
}
impl ::core::convert::From<StatusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: StatusChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StatusChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &StatusChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StatusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: StatusChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StatusChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &StatusChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StatusChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for StatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for StatusChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VenueData(pub ::windows::core::IInspectable);
impl VenueData {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Level(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VenueData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Geolocation.VenueData;{66f39187-60e3-4b2f-b527-4f53f1c3c677})");
}
unsafe impl ::windows::core::Interface for VenueData {
    type Vtable = IVenueData_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66f39187_60e3_4b2f_b527_4f53f1c3c677);
}
impl ::windows::core::RuntimeName for VenueData {
    const NAME: &'static str = "Windows.Devices.Geolocation.VenueData";
}
impl ::core::convert::From<VenueData> for ::windows::core::IUnknown {
    fn from(value: VenueData) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VenueData> for ::windows::core::IUnknown {
    fn from(value: &VenueData) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VenueData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VenueData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VenueData> for ::windows::core::IInspectable {
    fn from(value: VenueData) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VenueData> for ::windows::core::IInspectable {
    fn from(value: &VenueData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VenueData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VenueData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VenueData {}
unsafe impl ::core::marker::Sync for VenueData {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VisitMonitoringScope(pub i32);
impl VisitMonitoringScope {
    pub const Venue: VisitMonitoringScope = VisitMonitoringScope(0i32);
    pub const City: VisitMonitoringScope = VisitMonitoringScope(1i32);
}
impl ::core::convert::From<i32> for VisitMonitoringScope {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VisitMonitoringScope {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VisitMonitoringScope {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.VisitMonitoringScope;i4)");
}
impl ::windows::core::DefaultType for VisitMonitoringScope {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VisitStateChange(pub i32);
impl VisitStateChange {
    pub const TrackingLost: VisitStateChange = VisitStateChange(0i32);
    pub const Arrived: VisitStateChange = VisitStateChange(1i32);
    pub const Departed: VisitStateChange = VisitStateChange(2i32);
    pub const OtherMovement: VisitStateChange = VisitStateChange(3i32);
}
impl ::core::convert::From<i32> for VisitStateChange {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VisitStateChange {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VisitStateChange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Geolocation.VisitStateChange;i4)");
}
impl ::windows::core::DefaultType for VisitStateChange {
    type DefaultType = Self;
}
